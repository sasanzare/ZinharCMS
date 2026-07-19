---
okf_document_id: "api-route-architecture"
title: "API Route Architecture"
project: "ZinharCMS"
category: "api"
phase: 6
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "eed1e0dbdf6d873457d1165158b3c8fbfd6647e1"
last_verified_date: "2026-07-18"
primary_sources:
  - "backend/src/routes/mod.rs"
  - "backend/src/middleware/auth.rs"
  - "backend/src/middleware/tenant.rs"
  - "backend/src/middleware/security.rs"
related_documents:
  - "api/overview.md"
  - "api/authentication.md"
  - "api/authorization.md"
  - "api/tenant-context.md"
  - "backend/request-handling.md"
related_diagrams:
  - "api/diagrams/api-route-map.mmd"
  - "api/diagrams/api-request-lifecycle.mmd"
uncertainty_markers:
  - "ROUTE_REGISTRATION_UNCLEAR RRU-01"
---

# API Route Architecture

## Authoritative Composition

`routes::router(state)` constructs three subtrees and mounts a static service:

```text
root Router
|- public handlers
|  |- /, /health, /ready, /openapi.json
|  |- auth::public_router
|  |- billing::public_router
|  `- delivery::router
|- authenticated Router -> auth_middleware
|  |- auth::protected_router
|  |- beta::protected_router
|  |- organizations::protected_router
|  `- plugins::router
|- tenant-protected Router -> tenant_middleware -> body limit
|  |- content, beta, billing, media
|  |- Marketplace core, adapters, analytics, finance, runtime
|  |- organizations::tenant_router
|  `- pages, comments, webhooks
`- /uploads -> ServeDir
```

The route tree is the authority for reachability and access zone. Handler extractors alone are insufficient: a handler may not explicitly extract `Claims` even though its router is protected.

## Global Transport Layers

The application applies a 30-second timeout, security headers, response compression, configured CORS, request-ID creation and propagation, and HTTP tracing. The timeout fallback uses HTTP 408. CORS origin is configuration-driven. The global layer order should be read directly from `backend/src/lib.rs` before changing behavior because Tower layer order affects which responses receive which middleware.

## Authenticated Subtree

`auth_middleware` expects an `Authorization: Bearer <JWT>` header, validates the access token, and inserts `Claims`. The page-preview path is a special case that may accept the access token from `access_token` or `token` query parameters to support browser WebSocket clients.

## Tenant-Protected Subtree

`tenant_middleware` validates the access token, requires an active organization membership, applies organization rate limiting, applies quota checks except to paths under `/api/billing`, and inserts both `Claims` and `TenantContext`. Standard HTTP requests select the organization with `X-Organization-Id`; preview may use `organization_id` in the query string.

The tenant subtree has `DefaultBodyLimit` set to configured `MAX_UPLOAD_SIZE + 1 MiB`. The extra allowance covers multipart framing; the media handler separately enforces the configured file-byte maximum.

## Route-Specific Policy

Handlers perform role checks through `services::rbac`, ownership checks, global-admin checks, workflow checks, signature verification, validation, and persistence operations. These checks are not centralized in one authorization middleware, so the owning group and endpoint-family documents must be reviewed for sensitive changes.

## Non-Handler Surface

`ServeDir` below `/uploads` is public and bypasses JSON handlers, `AppError`, tenant selection, and endpoint OpenAPI annotations. Static service method and conditional-request behavior is delegated to Tower HTTP. Because it is not represented by a `.route(...)` handler registration, it is documented separately and excluded from the 168 endpoint count.

## Registration Caveat

`ROUTE_REGISTRATION_UNCLEAR RRU-01` is reserved for future cases where nested router composition prevents a method/path from being established. No such ambiguity was found in this snapshot: all 168 handler-method pairs were resolved from route registration and no duplicates were found.
