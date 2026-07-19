---
okf_document_id: "backend-request-handling"
title: "Backend Request Handling"
project: "ZinharCMS"
category: "backend"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "eed1e0dbdf6d873457d1165158b3c8fbfd6647e1"
last_verified_date: "2026-07-17"
primary_sources:
  - "backend/src/main.rs"
  - "backend/src/lib.rs"
  - "backend/src/routes/mod.rs"
  - "backend/src/middleware/auth.rs"
  - "backend/src/middleware/tenant.rs"
  - "backend/src/error.rs"
related_documents:
  - "backend/overview.md"
  - "backend/module-boundaries.md"
  - "backend/services-and-domain.md"
  - "backend/error-handling.md"
  - "architecture/runtime-flows.md"
related_diagrams:
  - "backend/diagrams/backend-request-lifecycle.mmd"
  - "architecture/diagrams/backend-request-flow.mmd"
uncertainty_markers:
  - "INFERRED_FROM_CODE"
  - "MIDDLEWARE_ORDER_DETAIL_UNCONFIRMED MDO-01"
  - "MODULE_BOUNDARY_UNCLEAR MBU-01"
---

# Backend Request Handling

## Phase 6 Route Contract Cross-Reference

The exhaustive method/path inventory now lives in the [API Endpoint Catalog](../api/endpoint-catalog.md), with router composition in [API Route Architecture](../api/route-architecture.md). Registration establishes 168 handler-method endpoints: 17 public, 12 bearer-authenticated, and 139 bearer-and-tenant.

Handler extractors define path, query, JSON, raw-byte, multipart, header, connection, and WebSocket-upgrade inputs. Router placement—not the presence of a `Claims` or `TenantContext` parameter alone—defines the access zone. Review [Request Contracts](../api/request-contracts.md), [Authentication](../api/authentication.md), and [Tenant Context](../api/tenant-context.md) before changing middleware or extractors.

## Request Entry

`backend/src/main.rs` binds a Tokio TCP listener, builds the Axum application through `cms_backend::app`, applies global Tower HTTP layers, and serves requests with peer connection information. `backend/src/lib.rs` delegates application construction to `routes::router`.

## Global HTTP Concerns

The application registers a 30-second timeout, security headers, response compression, configured CORS, request ID propagation/generation, and HTTP tracing. These layers are visibly composed in `backend/src/main.rs`. Tower layer execution order can be non-intuitive; Phase 3 does not restate an exact inbound ordering beyond the verified source composition (`MDO-01`).

## Router Groups

`backend/src/routes/mod.rs` composes three logical surfaces:

| Surface | Context | Representative ownership |
|---|---|---|
| Public | No protected router middleware | Root, health, readiness, OpenAPI, public auth, public billing, public delivery, static uploads |
| Authenticated | `auth_middleware` route layer | Current-user/auth lifecycle, protected organizations, beta operations, built-in plugins |
| Tenant-protected | `tenant_middleware` route layer and upload body limit | Content, billing, media, Marketplace, organizations, pages, comments, CMS webhooks |

Tenant middleware is responsible for resolving authentication and tenant context for the tenant group. Detailed authorization correctness and attack analysis are deferred to the security phase.

## Context Extraction

Handlers commonly extract `State<AppState>`, path/query/header data, JSON or multipart bodies, `Claims`, and `TenantContext`. WebSocket handlers also upgrade the connection and use the process-local preview broadcast state. Request DTO validation may be performed in handlers or delegated to services; there is no single verified validation pipeline shared by all modules.

## Handler Processing

Observed handler work includes:

1. parse Axum extractors and request DTOs;
2. verify role, membership, ownership, quota, rate, status, or feature preconditions;
3. call a service and/or execute SQL directly;
4. coordinate transactions for selected multi-write operations;
5. trigger cache, audit, email, webhook, file, Stripe, or preview side effects;
6. construct JSON, status-only, redirect/provider, file, OpenAPI, or WebSocket responses.

Handlers are therefore not uniform thin controllers. The exact sequence varies by module and must be verified at the owning route.

## Services and Domain Calls

Service modules cover reusable validation, policy, token/password behavior, workflow transitions, quota/rate checks, provider integrations, media processing, and the larger Marketplace domain. Some are pure or mostly pure helpers; others accept a `PgPool`, Redis client, configuration values, or request context and perform I/O. See [Services and Domain](services-and-domain.md).

## Persistence

Handlers and services issue SQLx queries against the shared PostgreSQL pool. Selected tenant operations invoke RLS helpers or run within explicit transactions. Redis access is used for caching and counters; media uses the configured upload directory. There is no verified repository layer that all requests traverse.

## Response and Error Mapping

Most fallible application handlers return `Result<_, AppError>`. `AppError::into_response` selects an HTTP status and serializes an `ErrorBody` with a stable error category plus the enum display string. Some framework rejections, timeout responses, static file results, and WebSocket failures follow Axum/Tower behavior instead. See [Error Handling](error-handling.md).

## Representative Lifecycle

A tenant mutation normally follows this documented shape, with module-specific steps omitted when absent:

1. Global HTTP layers establish request metadata and transport policy.
2. Router matching selects the tenant-protected group.
3. Tenant middleware verifies bearer context, resolves organization context, and performs cross-cutting policy checks.
4. The route handler validates resource-specific input and authorization.
5. SQLx and/or services perform the primary operation, sometimes inside a transaction.
6. Optional cache invalidation or outbound/process-local side effects run.
7. The handler returns a response or `AppError` maps the failure.
8. Tracing and request-ID layers finish request observability.

## Variants

- Public delivery emphasizes cache lookup, database fallback, and cache population.
- Media combines multipart extraction, filesystem I/O, processing, and metadata persistence.
- Page preview upgrades to WebSocket and subscribes to an in-memory broadcast channel.
- Stripe and webhook handlers verify provider-specific input and coordinate external state.
- Health/readiness handlers query dependencies directly from the shared state.

## Uncertainties and Deferred Detail

Runtime proxy behavior, deployed timeout interactions, full extractor rejection shape, provider latency, and request ordering across multiple replicas remain `UNKNOWN` without deployment evidence. Endpoint-by-endpoint API contracts are Phase 4 work; detailed security assessment belongs to the dedicated security phase.

## Related Documentation

Use the [request lifecycle diagram](diagrams/backend-request-lifecycle.mmd), [module catalog](module-catalog.md), [Persistence Access](persistence-access.md), [Phase 2 Runtime Flows](../architecture/runtime-flows.md), [Authentication Architecture](../security/authentication-architecture.md), and [Authorization Decision Flow](../security/diagrams/authorization-decision-flow.mmd).

Phase 7 confirms that route placement establishes public, bearer, or tenant access; handler RBAC/ownership/lifecycle checks and RLS then complete authorization. Framework and middleware rejections do not all share the same error body.
