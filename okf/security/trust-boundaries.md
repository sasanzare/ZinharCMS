---
okf_document_id: "security-trust-boundaries"
title: "Security Trust Boundaries"
project: "ZinharCMS"
category: "security"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "partially_verified"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/main.rs"
  - "backend/src/routes/mod.rs"
  - "backend/src/middleware/auth.rs"
  - "backend/src/middleware/tenant.rs"
  - "backend/src/services/rls.rs"
  - "frontend/src/services/api.ts"
related_documents:
  - "overview.md"
  - "browser-and-http-security.md"
  - "tenant-access-control.md"
related_diagrams:
  - "diagrams/trust-boundaries.mmd"
---

# Security Trust Boundaries

## Boundary Catalog

| Boundary | Untrusted or less-trusted side | Trusted side after checks | Evidence |
| --- | --- | --- | --- |
| Browser to HTTP API | Browser state, request body, bearer token, tenant header | Axum request pipeline | `main.rs`, `routes/mod.rs` |
| Public to protected router | Unauthenticated request | Verified access-token claims | `middleware/auth.rs` |
| Protected to tenant router | Claimed organization ID | Active organization membership and `TenantContext` | `middleware/tenant.rs` |
| Application to PostgreSQL | Handler/service parameters | Tenant-aware SQL session or explicit bypass transaction | `services/rls.rs`, migrations |
| Application to Redis | Organization/user identifiers and counters | Rate-limit/cache operations | `services/rate_limit.rs`, `services/cache.rs` |
| API to filesystem | Multipart/package bytes and paths | Validated upload/package paths | media and Marketplace services |
| Application to providers | Billing, email, or webhook requests | Provider-specific validation/signature rules | Stripe, email, webhook services |
| Marketplace package to host API | Declared operation and payload | Approved permission, safe entry point, runtime state, kill switches | `services/marketplace_runtime.rs` |

## Public Surfaces

System probes, OpenAPI, public authentication, delivery endpoints, Stripe webhook, and upload serving are reachable without bearer middleware. Their handlers or delegated services must provide their own validation. Public does not mean unrestricted: delivery reads published data, Stripe requires signature verification, and static upload behavior is delegated to `ServeDir`.

## Identity and Tenant Boundaries

The access-token claim carries a global role. Organization identity is separately supplied by `X-Organization-Id`; preview clients may use query parameters for both token and organization. Tenant middleware ignores a client-supplied organization role and loads the active membership role from PostgreSQL.

## Persistence Boundary

Tenant-aware SQL helpers set `zinhar.organization_id`, `zinhar.user_id`, and `zinhar.rls_bypass`. Forced RLS then filters or checks tenant rows. Explicit bypass transactions set `zinhar.rls_bypass=true` for selected platform-wide operations. `ADMINISTRATIVE_BYPASS_UNCLEAR ABY-01` records that bypass authorization is distributed among callers rather than enforced inside the helper.

## Unverified Deployment Edges

TLS, proxy header normalization, network segmentation, database roles, Redis authentication, upload serving at an external proxy, and provider egress policy are outside this repository-only review. These relationships are dashed in the diagram and carry `SECURITY_HEADER_STATUS_UNCLEAR SHSU-01`, `COOKIE_SECURITY_UNVERIFIED CSU-01`, and `SECRET_HANDLING_UNVERIFIED SHU-01`.
