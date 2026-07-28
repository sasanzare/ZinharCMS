use axum::body::Body;
use axum::http::header::{
    CONTENT_SECURITY_POLICY, HeaderName, HeaderValue, REFERRER_POLICY, X_CONTENT_TYPE_OPTIONS,
};
use axum::http::{HeaderMap, Request, Response};
use axum::middleware::Next;

static X_FRAME_OPTIONS: HeaderName = HeaderName::from_static("x-frame-options");
static PERMISSIONS_POLICY: HeaderName = HeaderName::from_static("permissions-policy");
static CROSS_ORIGIN_OPENER_POLICY: HeaderName =
    HeaderName::from_static("cross-origin-opener-policy");
static CROSS_ORIGIN_RESOURCE_POLICY: HeaderName =
    HeaderName::from_static("cross-origin-resource-policy");

const API_CONTENT_SECURITY_POLICY: &str = concat!(
    "default-src 'none'; ",
    "base-uri 'none'; ",
    "object-src 'none'; ",
    "frame-ancestors 'none'; ",
    "form-action 'none'; ",
    "script-src 'none'; ",
    "script-src-attr 'none'; ",
    "style-src 'none'; ",
    "img-src 'none'; ",
    "font-src 'none'; ",
    "connect-src 'none'; ",
    "worker-src 'none'; ",
    "manifest-src 'none'; ",
    "media-src 'none'; ",
    "frame-src 'none'; ",
    "trusted-types 'none'; ",
    "require-trusted-types-for 'script'"
);

pub async fn security_headers(request: Request<Body>, next: Next) -> Response<Body> {
    let mut response = next.run(request).await;
    apply_security_headers(response.headers_mut());
    response
}

pub fn apply_security_headers(headers: &mut HeaderMap) {
    headers.insert(
        CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(API_CONTENT_SECURITY_POLICY),
    );
    headers.insert(X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));
    headers.insert(
        REFERRER_POLICY,
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    headers.insert(X_FRAME_OPTIONS.clone(), HeaderValue::from_static("DENY"));
    headers.insert(
        CROSS_ORIGIN_OPENER_POLICY.clone(),
        HeaderValue::from_static("same-origin"),
    );
    headers.insert(
        CROSS_ORIGIN_RESOURCE_POLICY.clone(),
        HeaderValue::from_static("same-site"),
    );
    headers.insert(
        PERMISSIONS_POLICY.clone(),
        HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
    );
}

#[cfg(test)]
mod tests {
    use axum::http::header::{CONTENT_SECURITY_POLICY, REFERRER_POLICY, X_CONTENT_TYPE_OPTIONS};
    use axum::http::{HeaderMap, HeaderName};

    use super::apply_security_headers;

    #[test]
    fn applies_production_security_headers() {
        let mut headers = HeaderMap::new();
        apply_security_headers(&mut headers);

        let policy = headers[CONTENT_SECURITY_POLICY].to_str().unwrap();
        assert!(policy.contains("default-src 'none'"));
        assert!(policy.contains("base-uri 'none'"));
        assert!(policy.contains("object-src 'none'"));
        assert!(policy.contains("frame-ancestors 'none'"));
        assert!(policy.contains("form-action 'none'"));
        assert!(!policy.contains("data:"));
        assert!(!policy.contains("blob:"));
        assert!(!policy.contains("'unsafe-eval'"));
        assert_eq!(headers[X_CONTENT_TYPE_OPTIONS], "nosniff");
        assert_eq!(headers[REFERRER_POLICY], "strict-origin-when-cross-origin");
        assert_eq!(headers[HeaderName::from_static("x-frame-options")], "DENY");
        assert_eq!(
            headers[HeaderName::from_static("cross-origin-opener-policy")],
            "same-origin"
        );
        assert_eq!(
            headers[HeaderName::from_static("cross-origin-resource-policy")],
            "same-site"
        );
        assert!(policy.contains("script-src 'none'"));
        assert!(policy.contains("script-src-attr 'none'"));
        assert!(policy.contains("trusted-types 'none'"));
        assert!(policy.contains("require-trusted-types-for 'script'"));
        assert!(
            headers[HeaderName::from_static("permissions-policy")]
                .to_str()
                .unwrap()
                .contains("geolocation=()")
        );
    }
}
