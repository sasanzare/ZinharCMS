# Phase Seven Implementation

Phase seven goal: security hardening for authentication, API responses, webhooks, rich text, and media uploads.

## Delivered

- Failed-login rate limiting by client IP: 5 failed attempts per 15 minutes by default.
- Refresh tokens are issued as `HttpOnly` cookies on register, login, and refresh.
- Logout clears the refresh cookie and revokes the token when one is present.
- CORS now supports credentialed requests from the configured `CORS_ORIGIN`.
- Security headers are added to API responses:
  - `Content-Security-Policy`
  - `X-Content-Type-Options`
  - `Referrer-Policy`
  - `X-Frame-Options`
  - `Permissions-Policy`
- Rich text entry fields are sanitized before save.
- Webhook URLs reject credentials, localhost, private IPs, loopback, link-local, multicast, unspecified, and common metadata hosts.
- Media uploads validate file type by content signatures instead of trusting filename or declared MIME type.
- SVG uploads are no longer accepted because serving user-controlled SVG can execute script in browsers.

## Configuration

| Variable | Default | Purpose |
| --- | --- | --- |
| `COOKIE_SECURE` | `false` | Set to `true` in HTTPS environments so refresh cookies include `Secure`. |
| `LOGIN_RATE_LIMIT_MAX_FAILURES` | `5` | Failed login attempts allowed within the window. |
| `LOGIN_RATE_LIMIT_WINDOW_SECONDS` | `900` | Rate-limit window in seconds. |

## Notes

The auth response still returns the access token in JSON. The refresh token body field is now `null`; browsers receive the real refresh token through the `zinhar_refresh_token` cookie.
