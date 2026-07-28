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
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
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

- API Content Security Policy: deny all sources, base changes, objects, frames,
  forms, scripts, styles, images, media, connections, and unapproved Trusted
  Types policies.
- `X-Content-Type-Options: nosniff`.
- `Referrer-Policy: strict-origin-when-cross-origin`.
- `X-Frame-Options: DENY`.
- `Cross-Origin-Opener-Policy: same-origin`.
- `Cross-Origin-Resource-Policy: same-site`.
- permissions policy disabling camera, microphone, and geolocation.

The frontend Nginx template adds the enforced production CSP, HSTS, COOP,
same-origin CORP, frame denial, `nosniff`, Referrer Policy, and Permissions
Policy to static and SPA responses. Vite development and preview provide
development and production policy validation respectively. TLS termination is
still external to the repository.

## CORS

The server permits one configured origin, credentialed requests, methods GET/POST/PUT/PATCH/DELETE, and headers Authorization, Content-Type, and X-Organization-Id. No wildcard origin is configured in code. The effective deployed origin was not inspected.

## Cookies and CSRF

The refresh cookie is `HttpOnly`, `SameSite=Lax`, path-scoped, and conditionally `Secure`. State-changing tenant requests use bearer tokens in an Authorization header, reducing ambient-cookie exposure. Refresh can be invoked with the ambient cookie and has no explicit CSRF token; SameSite and CORS are the visible controls. Logout also requires a bearer token.

## Request Controls

All application routes receive a 30-second timeout, request IDs, tracing, compression, CORS, and security headers through the main binary composition. Tenant routes have a configured body limit. Public static uploads are delegated to `ServeDir`; range, caching, sniffing, and method behavior should be verified at the deployed boundary.

## Preview Query Credentials

Preview WebSocket URLs contain no credentials or organization values. The
browser obtains a short-lived one-time ticket and offers it as a credential
subprotocol; the server validates exact Origin, consumes the ticket atomically,
and selects only the stable application protocol.

## Status Markers

- `SECURITY_HEADER_STATUS_UNCLEAR SHSU-01`: API middleware, Vite preview, and
  Nginx template behavior are tested, but a real deployed proxy can still
  override headers; target-environment TLS and HSTS remain unverified.
- `COOKIE_SECURITY_UNVERIFIED CSU-01`: production Secure/TLS behavior is configuration-dependent.
- `RATE_LIMITING_STATUS_UNCLEAR RLSU-01`: proxy trust for client IP and live Redis behavior are unverified.

## Deployment Boundary

The frontend Nginx image serves static SPA files, long-lived asset caching, and
security headers; it does not terminate configured TLS or proxy API traffic.
Operators must set exact `CSP_API_ORIGIN` and `CSP_WEBSOCKET_ORIGIN` values and
align them with CORS, preview Origin validation, and the built
`VITE_API_URL`. Production ingress, TLS, trusted proxy behavior, traffic
switching, and application health probes remain outside tracked deployment
configuration. See [Container Builds](../delivery/container-builds.md),
[Deployment Workflow](../delivery/deployment-workflow.md), and
[Health and Readiness](../operations/health-and-readiness.md).
