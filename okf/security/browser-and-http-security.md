---
okf_document_id: "security-browser-http"
title: "Browser and HTTP Security"
project: "ZinharCMS"
category: "security-http"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "partially_verified"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/main.rs"
  - "backend/src/middleware/security.rs"
  - "backend/src/routes/auth.rs"
  - "backend/src/routes/mod.rs"
  - "frontend/src/services/api.ts"
related_documents:
  - "trust-boundaries.md"
  - "frontend-security-boundaries.md"
  - "secrets-and-configuration.md"
related_diagrams:
  - "diagrams/trust-boundaries.mmd"
---

# Browser and HTTP Security

## Response Headers

Application middleware adds:

- Content Security Policy: default self; images self/data/blob; media self; objects none; frame ancestors none; base URI self; form action self.
- `X-Content-Type-Options: nosniff`.
- `Referrer-Policy: same-origin`.
- `X-Frame-Options: DENY`.
- permissions policy disabling camera, microphone, and geolocation.

No application HSTS, COOP, COEP, CORP, cache-control policy for authentication responses, or content-disposition policy was found. HSTS may be owned by an external TLS proxy, but that was not verified.

## CORS

The server permits one configured origin, credentialed requests, methods GET/POST/PUT/PATCH/DELETE, and headers Authorization, Content-Type, and X-Organization-Id. No wildcard origin is configured in code. The effective deployed origin was not inspected.

## Cookies and CSRF

The refresh cookie is `HttpOnly`, `SameSite=Lax`, path-scoped, and conditionally `Secure`. State-changing tenant requests use bearer tokens in an Authorization header, reducing ambient-cookie exposure. Refresh can be invoked with the ambient cookie and has no explicit CSRF token; SameSite and CORS are the visible controls. Logout also requires a bearer token.

## Request Controls

All application routes receive a 30-second timeout, request IDs, tracing, compression, CORS, and security headers through the main binary composition. Tenant routes have a configured body limit. Public static uploads are delegated to `ServeDir`; range, caching, sniffing, and method behavior should be verified at the deployed boundary.

## Preview Query Credentials

Preview WebSocket compatibility allows access-token and organization values in the URL query. URLs can be logged or retained more widely than headers. Treat this as a constrained exception and redact it at proxies/logging systems.

## Status Markers

- `SECURITY_HEADER_STATUS_UNCLEAR SHSU-01`: direct middleware behavior is tested, but deployed proxy overrides, error/static responses, TLS, and HSTS are unverified.
- `COOKIE_SECURITY_UNVERIFIED CSU-01`: production Secure/TLS behavior is configuration-dependent.
- `RATE_LIMITING_STATUS_UNCLEAR RLSU-01`: proxy trust for client IP and live Redis behavior are unverified.
