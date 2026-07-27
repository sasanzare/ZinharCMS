use std::net::{IpAddr, SocketAddr};

use axum::http::HeaderMap;
use chrono::{Duration, Utc};
use ipnet::IpNet;
use serde_json::Value;
use sqlx::PgPool;

use crate::error::AppError;
use crate::services::entry_validation::FieldSchemaDocument;

pub const LOGIN_RATE_LIMIT_MAX_FAILURES: i64 = 5;
pub const LOGIN_RATE_LIMIT_WINDOW_SECONDS: i64 = 15 * 60;

pub async fn require_login_allowed(
    db: &PgPool,
    ip_address: &str,
    max_failures: i64,
    window_seconds: i64,
) -> Result<(), AppError> {
    let since = Utc::now() - Duration::seconds(window_seconds);
    let failed_count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM login_attempts
        WHERE ip_address = $1
          AND succeeded = FALSE
          AND attempted_at >= $2
        "#,
    )
    .bind(ip_address)
    .bind(since)
    .fetch_one(db)
    .await?;

    if failed_count >= max_failures {
        Err(AppError::TooManyRequests(
            "too many failed login attempts; try again later".to_owned(),
        ))
    } else {
        Ok(())
    }
}
pub async fn record_login_attempt(
    db: &PgPool,
    email: &str,
    ip_address: &str,
    succeeded: bool,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO login_attempts (email, ip_address, succeeded)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(email)
    .bind(ip_address)
    .bind(succeeded)
    .execute(db)
    .await?;

    Ok(())
}

pub fn client_ip(headers: &HeaderMap, socket_peer: IpAddr, trusted_proxies: &[IpNet]) -> String {
    if !is_trusted_proxy(socket_peer, trusted_proxies) {
        return socket_peer.to_string();
    }

    let forwarded_chain = if headers.contains_key("forwarded") {
        parse_forwarded_chain(headers)
    } else if headers.contains_key("x-forwarded-for") {
        parse_x_forwarded_for_chain(headers)
    } else if headers.contains_key("x-real-ip") {
        parse_x_real_ip(headers).map(|address| vec![address])
    } else {
        return socket_peer.to_string();
    };

    let Ok(chain) = forwarded_chain else {
        return socket_peer.to_string();
    };

    chain
        .into_iter()
        .rev()
        .find(|address| !is_trusted_proxy(*address, trusted_proxies))
        .unwrap_or(socket_peer)
        .to_string()
}

fn is_trusted_proxy(address: IpAddr, trusted_proxies: &[IpNet]) -> bool {
    trusted_proxies
        .iter()
        .any(|network| network.contains(&address))
}

fn parse_forwarded_chain(headers: &HeaderMap) -> Result<Vec<IpAddr>, ()> {
    let mut addresses = Vec::new();
    for value in headers.get_all("forwarded") {
        let value = value.to_str().map_err(|_| ())?;
        for element in split_quoted(value, ',')? {
            let mut forwarded_for = None;
            for parameter in split_quoted(element, ';')? {
                let Some((name, value)) = parameter.trim().split_once('=') else {
                    return Err(());
                };
                if name.trim().eq_ignore_ascii_case("for") {
                    if forwarded_for.is_some() {
                        return Err(());
                    }
                    forwarded_for = Some(parse_forwarded_address(value.trim())?);
                }
            }
            addresses.push(forwarded_for.ok_or(())?);
        }
    }
    (!addresses.is_empty()).then_some(addresses).ok_or(())
}

fn split_quoted(value: &str, delimiter: char) -> Result<Vec<&str>, ()> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut quoted = false;
    for (index, character) in value.char_indices() {
        match character {
            '"' => quoted = !quoted,
            '\\' if quoted => return Err(()),
            _ if character == delimiter && !quoted => {
                let part = value[start..index].trim();
                if part.is_empty() {
                    return Err(());
                }
                parts.push(part);
                start = index + character.len_utf8();
            }
            _ => {}
        }
    }
    if quoted {
        return Err(());
    }
    let part = value[start..].trim();
    if part.is_empty() {
        return Err(());
    }
    parts.push(part);
    Ok(parts)
}

fn parse_forwarded_address(value: &str) -> Result<IpAddr, ()> {
    let value = value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .unwrap_or(value);
    if value.eq_ignore_ascii_case("unknown") || value.starts_with('_') {
        return Err(());
    }
    if let Some(bracketed) = value.strip_prefix('[') {
        let (address, suffix) = bracketed.split_once(']').ok_or(())?;
        if !suffix.is_empty() {
            let port = suffix.strip_prefix(':').ok_or(())?;
            port.parse::<u16>().map_err(|_| ())?;
        }
        return address.parse::<IpAddr>().map_err(|_| ());
    }
    if let Ok(address) = value.parse::<IpAddr>() {
        return Ok(address);
    }
    value
        .parse::<SocketAddr>()
        .map(|address| address.ip())
        .map_err(|_| ())
}

fn parse_x_forwarded_for_chain(headers: &HeaderMap) -> Result<Vec<IpAddr>, ()> {
    let mut addresses = Vec::new();
    for value in headers.get_all("x-forwarded-for") {
        let value = value.to_str().map_err(|_| ())?;
        for candidate in value.split(',') {
            let candidate = candidate.trim();
            if candidate.is_empty() {
                return Err(());
            }
            addresses.push(candidate.parse::<IpAddr>().map_err(|_| ())?);
        }
    }
    (!addresses.is_empty()).then_some(addresses).ok_or(())
}

fn parse_x_real_ip(headers: &HeaderMap) -> Result<IpAddr, ()> {
    let mut values = headers.get_all("x-real-ip").iter();
    let value = values.next().ok_or(())?.to_str().map_err(|_| ())?;
    if values.next().is_some() {
        return Err(());
    }
    value.trim().parse::<IpAddr>().map_err(|_| ())
}

pub fn sanitize_entry_data(fields: &FieldSchemaDocument, data: Value) -> Value {
    let Some(mut object) = data.as_object().cloned() else {
        return data;
    };

    for field in fields
        .fields
        .iter()
        .filter(|field| field.field_type == "richtext")
    {
        if let Some(value) = object.get(&field.name).and_then(Value::as_str) {
            object.insert(field.name.clone(), Value::String(sanitize_richtext(value)));
        }
    }

    Value::Object(object)
}

pub fn sanitize_richtext(value: &str) -> String {
    let without_blocks = ["script", "style", "iframe", "object", "embed"]
        .into_iter()
        .fold(value.to_owned(), remove_tag_block);
    sanitize_tags(&without_blocks)
}

fn remove_tag_block(input: String, tag: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let lower = input.to_ascii_lowercase();
    let mut cursor = 0;
    let start_pattern = format!("<{tag}");
    let end_pattern = format!("</{tag}>");

    while let Some(relative_start) = lower[cursor..].find(&start_pattern) {
        let start = cursor + relative_start;
        output.push_str(&input[cursor..start]);
        let search_from = start + start_pattern.len();
        if let Some(relative_end) = lower[search_from..].find(&end_pattern) {
            cursor = search_from + relative_end + end_pattern.len();
        } else {
            cursor = input.len();
            break;
        }
    }

    output.push_str(&input[cursor..]);
    output
}

fn sanitize_tags(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut cursor = 0;
    while let Some(relative_start) = input[cursor..].find('<') {
        let start = cursor + relative_start;
        output.push_str(&input[cursor..start]);
        let Some(relative_end) = input[start..].find('>') else {
            output.push_str("&lt;");
            cursor = start + 1;
            continue;
        };
        let end = start + relative_end;
        if let Some(tag) = sanitize_html_tag(&input[start + 1..end]) {
            output.push_str(&tag);
        }
        cursor = end + 1;
    }
    output.push_str(&input[cursor..]);
    output
}

fn sanitize_html_tag(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed.starts_with('!') || trimmed.starts_with('?') {
        return None;
    }

    let closing = trimmed.starts_with('/');
    let tag_body = trimmed.trim_start_matches('/').trim_start();
    let tag_name = tag_body
        .split(|ch: char| ch.is_ascii_whitespace() || ch == '/')
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase();
    let allowed = matches!(
        tag_name.as_str(),
        "a" | "b"
            | "blockquote"
            | "br"
            | "code"
            | "em"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "i"
            | "img"
            | "li"
            | "ol"
            | "p"
            | "pre"
            | "span"
            | "strong"
            | "u"
            | "ul"
    );
    if !allowed {
        return None;
    }

    if closing {
        Some(format!("</{tag_name}>"))
    } else {
        Some(format!("<{tag_name}>"))
    }
}

#[cfg(test)]
mod tests {
    use axum::http::{HeaderMap, HeaderValue};
    use ipnet::IpNet;

    use super::*;

    #[test]
    fn richtext_removes_script_blocks() {
        assert_eq!(
            sanitize_richtext("<p>safe</p><script>alert(1)</script>"),
            "<p>safe</p>"
        );
    }

    #[test]
    fn richtext_strips_attributes() {
        assert_eq!(
            sanitize_richtext("<img src=x onerror=alert(1)><a href=\"javascript:alert(1)\">x</a>"),
            "<img><a>x</a>"
        );
    }

    fn trusted(values: &[&str]) -> Vec<IpNet> {
        values.iter().map(|value| value.parse().unwrap()).collect()
    }

    #[test]
    fn direct_clients_cannot_spoof_forwarding_headers() {
        let peer = "203.0.113.10".parse().unwrap();
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", HeaderValue::from_static("198.51.100.20"));
        headers.insert("forwarded", HeaderValue::from_static("for=198.51.100.21"));

        assert_eq!(client_ip(&headers, peer, &[]), "203.0.113.10");
    }

    #[test]
    fn trusted_proxy_chain_selects_nearest_untrusted_hop() {
        let peer = "10.0.0.2".parse().unwrap();
        let proxies = trusted(&["10.0.0.0/8", "192.168.0.0/16"]);
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-forwarded-for",
            HeaderValue::from_static("198.51.100.20, 203.0.113.50, 192.168.1.2"),
        );

        assert_eq!(client_ip(&headers, peer, &proxies), "203.0.113.50");
    }

    #[test]
    fn multiple_trusted_proxies_are_removed_from_the_nearest_side() {
        let peer = "10.0.0.2".parse().unwrap();
        let proxies = trusted(&["10.0.0.0/8", "192.168.0.0/16"]);
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-forwarded-for",
            HeaderValue::from_static("198.51.100.20, 192.168.1.2, 10.0.0.3"),
        );

        assert_eq!(client_ip(&headers, peer, &proxies), "198.51.100.20");
    }

    #[test]
    fn forwarded_has_precedence_and_supports_ipv6() {
        let peer = "2001:db8:ffff::2".parse().unwrap();
        let proxies = trusted(&["2001:db8:ffff::/48"]);
        let mut headers = HeaderMap::new();
        headers.insert(
            "forwarded",
            HeaderValue::from_static("for=\"[2001:db8::20]\";proto=https"),
        );
        headers.insert("x-forwarded-for", HeaderValue::from_static("198.51.100.20"));

        assert_eq!(client_ip(&headers, peer, &proxies), "2001:db8::20");
    }

    #[test]
    fn malformed_forwarded_header_falls_back_to_socket_peer() {
        let peer = "10.0.0.2".parse().unwrap();
        let proxies = trusted(&["10.0.0.0/8"]);
        let mut headers = HeaderMap::new();
        headers.insert("forwarded", HeaderValue::from_static("for=unknown"));
        headers.insert("x-forwarded-for", HeaderValue::from_static("198.51.100.20"));

        assert_eq!(client_ip(&headers, peer, &proxies), "10.0.0.2");
    }

    #[test]
    fn x_real_ip_is_used_only_for_a_trusted_peer() {
        let peer = "10.0.0.2".parse().unwrap();
        let proxies = trusted(&["10.0.0.0/8"]);
        let mut headers = HeaderMap::new();
        headers.insert("x-real-ip", HeaderValue::from_static("2001:db8::20"));

        assert_eq!(client_ip(&headers, peer, &proxies), "2001:db8::20");
    }
}
