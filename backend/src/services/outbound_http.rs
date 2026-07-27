use std::fmt;
use std::future::Future;
use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use axum::http::HeaderMap;
use reqwest::dns::{Addrs, Name, Resolve, Resolving};
use reqwest::redirect::Policy;
use reqwest::{Client, StatusCode, Url};

const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(3);
const DEFAULT_TOTAL_TIMEOUT: Duration = Duration::from_secs(10);
pub const MAX_OUTBOUND_RESPONSE_BYTES: usize = 64 * 1024;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum OutboundRequestError {
    Blocked,
    Failed,
    ResponseTooLarge,
}

impl fmt::Display for OutboundRequestError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Blocked => "outbound destination blocked",
            Self::Failed => "outbound request failed",
            Self::ResponseTooLarge => "outbound response exceeded the allowed size",
        })
    }
}

impl std::error::Error for OutboundRequestError {}

type AddressResolution = Pin<Box<dyn Future<Output = io::Result<Vec<IpAddr>>> + Send + 'static>>;

trait AddressResolver: Send + Sync {
    fn resolve(&self, hostname: String) -> AddressResolution;
}

#[derive(Clone)]
struct SystemAddressResolver;

impl AddressResolver for SystemAddressResolver {
    fn resolve(&self, hostname: String) -> AddressResolution {
        Box::pin(async move {
            let addresses = tokio::net::lookup_host((hostname.as_str(), 0))
                .await?
                .map(|address| address.ip())
                .collect::<Vec<_>>();
            Ok(addresses)
        })
    }
}

#[derive(Clone)]
struct SafeDnsResolver {
    resolver: Arc<dyn AddressResolver>,
    allow_forbidden_for_tests: bool,
}

impl SafeDnsResolver {
    fn new(resolver: Arc<dyn AddressResolver>, allow_forbidden_for_tests: bool) -> Self {
        Self {
            resolver,
            allow_forbidden_for_tests,
        }
    }

    async fn resolve_addresses(&self, hostname: &str) -> io::Result<Vec<IpAddr>> {
        let hostname = hostname.trim_end_matches('.').to_ascii_lowercase();
        if hostname.is_empty() {
            return Err(io::Error::other("destination resolution failed"));
        }
        let addresses = self
            .resolver
            .resolve(hostname)
            .await
            .map_err(|_| io::Error::other("destination resolution failed"))?;
        if addresses.is_empty()
            || (!self.allow_forbidden_for_tests && validate_resolved_addresses(&addresses).is_err())
        {
            return Err(io::Error::other("destination resolution failed"));
        }
        Ok(addresses)
    }
}

impl Resolve for SafeDnsResolver {
    fn resolve(&self, name: Name) -> Resolving {
        let resolver = self.clone();
        let hostname = name.as_str().to_owned();
        Box::pin(async move {
            let addresses = resolver.resolve_addresses(&hostname).await.map_err(|_| {
                Box::new(io::Error::other("destination resolution failed"))
                    as Box<dyn std::error::Error + Send + Sync>
            })?;
            let addresses: Addrs = Box::new(
                addresses
                    .into_iter()
                    .map(|address| SocketAddr::new(address, 0)),
            );
            Ok(addresses)
        })
    }
}

#[derive(Clone)]
pub struct OutboundHttpClient {
    client: Client,
    response_limit: usize,
}

#[derive(Debug)]
pub struct OutboundResponse {
    pub status: StatusCode,
    pub body: Vec<u8>,
}

impl OutboundHttpClient {
    pub fn new() -> Result<Self, OutboundRequestError> {
        Self::build(
            Arc::new(SystemAddressResolver),
            false,
            DEFAULT_CONNECT_TIMEOUT,
            DEFAULT_TOTAL_TIMEOUT,
            MAX_OUTBOUND_RESPONSE_BYTES,
        )
    }

    fn build(
        resolver: Arc<dyn AddressResolver>,
        allow_forbidden_for_tests: bool,
        connect_timeout: Duration,
        total_timeout: Duration,
        response_limit: usize,
    ) -> Result<Self, OutboundRequestError> {
        let resolver = Arc::new(SafeDnsResolver::new(resolver, allow_forbidden_for_tests));
        let client = Client::builder()
            .dns_resolver(resolver)
            .redirect(Policy::none())
            .no_proxy()
            .connect_timeout(connect_timeout)
            .timeout(total_timeout)
            .http1_only()
            .pool_max_idle_per_host(0)
            .build()
            .map_err(|_| OutboundRequestError::Failed)?;
        Ok(Self {
            client,
            response_limit,
        })
    }

    #[cfg(test)]
    fn for_test(
        resolver: Arc<dyn AddressResolver>,
        total_timeout: Duration,
        response_limit: usize,
    ) -> Result<Self, OutboundRequestError> {
        Self::build(resolver, true, total_timeout, total_timeout, response_limit)
    }

    pub async fn post_json(
        &self,
        destination: &str,
        headers: HeaderMap,
        body: &[u8],
    ) -> Result<OutboundResponse, OutboundRequestError> {
        let destination = normalize_and_validate_url(destination)?;
        let response = self
            .client
            .post(destination)
            .headers(headers)
            .header("content-type", "application/json")
            .body(body.to_vec())
            .send()
            .await
            .map_err(|_| OutboundRequestError::Failed)?;
        let status = response.status();
        let body = read_bounded_response(response, self.response_limit).await?;
        Ok(OutboundResponse { status, body })
    }
}

async fn read_bounded_response(
    mut response: reqwest::Response,
    limit: usize,
) -> Result<Vec<u8>, OutboundRequestError> {
    if response
        .content_length()
        .is_some_and(|length| length > limit as u64)
    {
        return Err(OutboundRequestError::ResponseTooLarge);
    }

    let mut body = Vec::with_capacity(
        response
            .content_length()
            .unwrap_or_default()
            .min(limit as u64) as usize,
    );
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| OutboundRequestError::Failed)?
    {
        if body.len().saturating_add(chunk.len()) > limit {
            return Err(OutboundRequestError::ResponseTooLarge);
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}

pub fn normalize_and_validate_url(destination: &str) -> Result<Url, OutboundRequestError> {
    let mut url = Url::parse(destination).map_err(|_| OutboundRequestError::Blocked)?;
    if !matches!(url.scheme(), "http" | "https")
        || url.cannot_be_a_base()
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return Err(OutboundRequestError::Blocked);
    }

    let host = url.host_str().ok_or(OutboundRequestError::Blocked)?;
    if host.is_empty() {
        return Err(OutboundRequestError::Blocked);
    }
    let ip_candidate = host
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .unwrap_or(host);
    if let Ok(address) = ip_candidate.parse::<IpAddr>() {
        if is_forbidden_ip(address) {
            return Err(OutboundRequestError::Blocked);
        }
        return Ok(url);
    }

    let normalized_host = host.trim_end_matches('.').to_ascii_lowercase();
    if normalized_host.is_empty()
        || matches!(
            normalized_host.as_str(),
            "localhost" | "ip6-localhost" | "metadata.google.internal"
        )
        || normalized_host.ends_with(".localhost")
    {
        return Err(OutboundRequestError::Blocked);
    }
    url.set_host(Some(&normalized_host))
        .map_err(|_| OutboundRequestError::Blocked)?;
    Ok(url)
}

pub fn validate_resolved_addresses(addresses: &[IpAddr]) -> Result<(), OutboundRequestError> {
    if addresses.is_empty() || addresses.iter().copied().any(is_forbidden_ip) {
        Err(OutboundRequestError::Blocked)
    } else {
        Ok(())
    }
}

pub fn is_forbidden_ip(address: IpAddr) -> bool {
    match address {
        IpAddr::V4(address) => is_forbidden_ipv4(address),
        IpAddr::V6(address) => is_forbidden_ipv6(address),
    }
}

fn is_forbidden_ipv4(address: Ipv4Addr) -> bool {
    let [a, b, c, _] = address.octets();
    a == 0
        || a == 10
        || a == 127
        || (a == 100 && (64..=127).contains(&b))
        || (a == 169 && b == 254)
        || (a == 172 && (16..=31).contains(&b))
        || (a == 192 && b == 0 && c == 0)
        || (a == 192 && b == 0 && c == 2)
        || (a == 192 && b == 88 && c == 99)
        || (a == 192 && b == 168)
        || (a == 198 && (b == 18 || b == 19))
        || (a == 198 && b == 51 && c == 100)
        || (a == 203 && b == 0 && c == 113)
        || a >= 224
}

fn is_forbidden_ipv6(address: Ipv6Addr) -> bool {
    if address.is_unspecified() || address.is_loopback() || address.to_ipv4().is_some() {
        return true;
    }

    let segments = address.segments();
    let first = segments[0];
    let second = segments[1];
    (first & 0xfe00) == 0xfc00
        || (first & 0xffc0) == 0xfe80
        || (first & 0xff00) == 0xff00
        || (first & 0xe000) != 0x2000
        || (first == 0x2001 && second == 0x0db8)
        || (first == 0x2001 && second == 0)
        || (first == 0x2001 && second == 2)
        || first == 0x2002
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::io;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    use axum::http::HeaderMap;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    use super::{
        AddressResolution, AddressResolver, OutboundHttpClient, OutboundRequestError,
        SafeDnsResolver, is_forbidden_ip, normalize_and_validate_url, validate_resolved_addresses,
    };

    #[derive(Clone, Default)]
    struct MockResolver {
        addresses: Arc<Mutex<HashMap<String, Vec<IpAddr>>>>,
        failures: Arc<Mutex<Vec<String>>>,
        calls: Arc<AtomicUsize>,
    }

    impl MockResolver {
        fn with_address(self, hostname: &str, address: IpAddr) -> Self {
            self.addresses
                .lock()
                .unwrap()
                .entry(hostname.to_owned())
                .or_default()
                .push(address);
            self
        }

        fn fail_for(self, hostname: &str) -> Self {
            self.failures.lock().unwrap().push(hostname.to_owned());
            self
        }
    }

    impl AddressResolver for MockResolver {
        fn resolve(&self, hostname: String) -> AddressResolution {
            self.calls.fetch_add(1, Ordering::SeqCst);
            let addresses = self.addresses.lock().unwrap().get(&hostname).cloned();
            let should_fail = self.failures.lock().unwrap().contains(&hostname);
            Box::pin(async move {
                if should_fail {
                    Err(io::Error::other("resolver detail must remain private"))
                } else {
                    Ok(addresses.unwrap_or_default())
                }
            })
        }
    }

    async fn start_server(
        responses: Vec<(Duration, String)>,
    ) -> (std::net::SocketAddr, tokio::task::JoinHandle<Vec<String>>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let handle = tokio::spawn(async move {
            let mut requests = Vec::new();
            for (delay, response) in responses {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut buffer = vec![0_u8; 4096];
                let read = stream.read(&mut buffer).await.unwrap();
                requests.push(String::from_utf8_lossy(&buffer[..read]).into_owned());
                tokio::time::sleep(delay).await;
                let _ = stream.write_all(response.as_bytes()).await;
                let _ = stream.shutdown().await;
            }
            requests
        });
        (address, handle)
    }

    fn test_client(
        resolver: MockResolver,
        timeout: Duration,
        response_limit: usize,
    ) -> OutboundHttpClient {
        OutboundHttpClient::for_test(Arc::new(resolver), timeout, response_limit).unwrap()
    }

    #[test]
    fn accepts_public_https_hostname_and_normalizes_trailing_dot() {
        let url = normalize_and_validate_url("https://Example.COM./hooks").unwrap();
        assert_eq!(url.as_str(), "https://example.com/hooks");
        assert!(validate_resolved_addresses(&["93.184.216.34".parse().unwrap()]).is_ok());
    }

    #[test]
    fn rejects_forbidden_literal_destinations() {
        for destination in [
            "http://127.0.0.1/hook",
            "http://10.1.2.3/hook",
            "http://169.254.1.1/hook",
            "http://169.254.169.254/latest/meta-data",
            "http://[::1]/hook",
            "http://[fc00::1]/hook",
            "http://[fe80::1]/hook",
            "http://[::ffff:127.0.0.1]/hook",
        ] {
            assert!(
                normalize_and_validate_url(destination).is_err(),
                "{destination} must be rejected"
            );
        }
    }

    #[test]
    fn rejects_reserved_and_non_routable_addresses() {
        for address in [
            IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            "100.64.0.1".parse().unwrap(),
            "192.0.2.1".parse().unwrap(),
            "198.18.0.1".parse().unwrap(),
            "203.0.113.1".parse().unwrap(),
            "224.0.0.1".parse().unwrap(),
            "240.0.0.1".parse().unwrap(),
            IpAddr::V4(Ipv4Addr::BROADCAST),
            IpAddr::V6(Ipv6Addr::UNSPECIFIED),
            "2001:db8::1".parse().unwrap(),
            "ff02::1".parse().unwrap(),
        ] {
            assert!(is_forbidden_ip(address), "{address} must be rejected");
        }
    }

    #[test]
    fn rejects_private_or_ambiguous_dns_results() {
        assert!(validate_resolved_addresses(&[]).is_err());
        assert!(validate_resolved_addresses(&["10.0.0.1".parse().unwrap()]).is_err());
        assert!(
            validate_resolved_addresses(&[
                "93.184.216.34".parse().unwrap(),
                "10.0.0.1".parse().unwrap(),
            ])
            .is_err()
        );
    }

    #[test]
    fn rejects_credentials_schemes_and_malformed_urls() {
        for destination in [
            "https://user:secret@example.com/hook",
            "ftp://example.com/hook",
            "not a URL",
            "https://[::1",
        ] {
            assert!(
                normalize_and_validate_url(destination).is_err(),
                "{destination} must be rejected"
            );
        }
    }

    #[tokio::test]
    async fn resolver_rejects_private_mixed_and_empty_results() {
        let resolver = MockResolver::default()
            .with_address("private.example", "10.0.0.1".parse().unwrap())
            .with_address("mixed.example", "93.184.216.34".parse().unwrap())
            .with_address("mixed.example", "10.0.0.1".parse().unwrap());
        let resolver = SafeDnsResolver::new(Arc::new(resolver), false);

        assert!(resolver.resolve_addresses("private.example").await.is_err());
        assert!(resolver.resolve_addresses("mixed.example").await.is_err());
        assert!(resolver.resolve_addresses("empty.example").await.is_err());
    }

    #[tokio::test]
    async fn approved_dns_result_is_used_for_the_actual_connection() {
        let response = "HTTP/1.1 204 No Content\r\nConnection: close\r\n\r\n".to_owned();
        let (address, server) = start_server(vec![(Duration::ZERO, response)]).await;
        let resolver = MockResolver::default().with_address("public.example", address.ip());
        let client = test_client(resolver, Duration::from_secs(1), 1024);

        let response = client
            .post_json(
                &format!("http://public.example:{}/hook", address.port()),
                HeaderMap::new(),
                b"{}",
            )
            .await
            .unwrap();
        assert_eq!(response.status.as_u16(), 204);

        let requests = server.await.unwrap();
        assert!(
            requests[0]
                .to_ascii_lowercase()
                .contains(&format!("host: public.example:{}", address.port()))
        );
    }

    #[tokio::test]
    async fn redirect_responses_are_not_followed() {
        for location in [
            "http://127.0.0.1/private",
            "http://forbidden.internal/private",
        ] {
            let response = format!(
                "HTTP/1.1 302 Found\r\nLocation: {location}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            );
            let (address, server) = start_server(vec![(Duration::ZERO, response)]).await;
            let resolver = MockResolver::default().with_address("public.example", address.ip());
            let client = test_client(resolver, Duration::from_secs(1), 1024);

            let response = client
                .post_json(
                    &format!("http://public.example:{}/hook", address.port()),
                    HeaderMap::new(),
                    b"{}",
                )
                .await
                .unwrap();
            assert_eq!(response.status.as_u16(), 302);
            assert_eq!(server.await.unwrap().len(), 1);
        }
    }

    #[tokio::test]
    async fn response_reading_is_bounded() {
        let body = "x".repeat(128);
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        let (address, server) = start_server(vec![(Duration::ZERO, response)]).await;
        let resolver = MockResolver::default().with_address("public.example", address.ip());
        let client = test_client(resolver, Duration::from_secs(1), 32);

        let error = client
            .post_json(
                &format!("http://public.example:{}/hook", address.port()),
                HeaderMap::new(),
                b"{}",
            )
            .await
            .unwrap_err();
        assert_eq!(error, OutboundRequestError::ResponseTooLarge);
        server.await.unwrap();
    }

    #[tokio::test]
    async fn total_request_timeout_is_enforced() {
        let response = "HTTP/1.1 204 No Content\r\nConnection: close\r\n\r\n".to_owned();
        let (address, server) = start_server(vec![(Duration::from_millis(200), response)]).await;
        let resolver = MockResolver::default().with_address("public.example", address.ip());
        let client = test_client(resolver, Duration::from_millis(50), 1024);

        let error = client
            .post_json(
                &format!("http://public.example:{}/hook", address.port()),
                HeaderMap::new(),
                b"{}",
            )
            .await
            .unwrap_err();
        assert_eq!(error, OutboundRequestError::Failed);
        server.await.unwrap();
    }

    #[tokio::test]
    async fn destination_is_revalidated_for_every_dispatch() {
        let response = "HTTP/1.1 204 No Content\r\nConnection: close\r\n\r\n".to_owned();
        let (address, server) = start_server(vec![
            (Duration::ZERO, response.clone()),
            (Duration::ZERO, response),
        ])
        .await;
        let resolver = MockResolver::default().with_address("public.example", address.ip());
        let calls = Arc::clone(&resolver.calls);
        let client = test_client(resolver, Duration::from_secs(1), 1024);
        let destination = format!("http://public.example:{}/hook", address.port());

        client
            .post_json(&destination, HeaderMap::new(), b"{}")
            .await
            .unwrap();
        client
            .post_json(&destination, HeaderMap::new(), b"{}")
            .await
            .unwrap();

        assert_eq!(calls.load(Ordering::SeqCst), 2);
        assert_eq!(server.await.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn resolver_and_destination_details_are_redacted() {
        let resolver = MockResolver::default().fail_for("sensitive.example");
        let client = test_client(resolver, Duration::from_secs(1), 1024);

        let error = client
            .post_json(
                "http://sensitive.example/hook?credential=value",
                HeaderMap::new(),
                b"{}",
            )
            .await
            .unwrap_err();
        let message = error.to_string();
        assert_eq!(message, "outbound request failed");
        assert!(!message.contains("sensitive"));
        assert!(!message.contains("credential"));
        assert!(!message.contains("resolver detail"));
    }
}
