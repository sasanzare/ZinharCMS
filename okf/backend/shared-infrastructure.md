---
okf_document_id: "backend-shared-infrastructure"
title: "Backend Shared Infrastructure"
project: "ZinharCMS"
category: "backend"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "mixed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
primary_sources:
  - "backend/Cargo.toml"
  - "backend/src/main.rs"
  - "backend/src/error.rs"
  - "backend/src/middleware"
  - "backend/src/services"
related_documents:
  - "backend/module-catalog.md"
  - "backend/dependency-map.md"
  - "backend/configuration-and-state.md"
  - "backend/error-handling.md"
  - "backend/testing-map.md"
related_diagrams:
  - "backend/diagrams/backend-dependency-flow.mmd"
  - "backend/diagrams/application-state-composition.mmd"
uncertainty_markers:
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR"
  - "MODULE_BOUNDARY_UNCLEAR MBU-01"
  - "IMPLEMENTATION_STATUS_UNCLEAR"
---

# Backend Shared Infrastructure

## Shared Area Catalog

| Shared area | Purpose | Source path | Main consumers | Domain logic | Ownership | Stability | Main risks |
|---|---|---|---|---|---|---|---|
| HTTP runtime | Axum/Tower routing, timeouts, compression, CORS, request IDs, tracing | `backend/src/main.rs`, `backend/src/routes/mod.rs` | All HTTP modules | Limited; composition selects policy surfaces | Bootstrap and Runtime | Stable core | Layer ordering and central composition change impact |
| Logging and tracing | Structured application and request logs | `backend/src/main.rs`, call-site `tracing` use | Startup and selected modules | No intended domain ownership | Bootstrap and Runtime | Established | Coverage and sensitive-field policy are not uniformly verified |
| Serialization and OpenAPI | JSON shapes and generated API description | route/model/service derives; `backend/src/routes/mod.rs` | All HTTP modules and clients | DTOs often encode domain vocabulary | Owning route/module | Established but distributed | Transport and domain shapes can drift |
| Validation helpers | Dynamic entry, Marketplace artifact, password/security and provider validation | `backend/src/services/entry_validation.rs`, `marketplace_validation.rs`, related services | Content, Marketplace, identity/provider paths | Yes | Owning domain service | Varies | No single validation pipeline or error vocabulary |
| Time and identifiers | UUID v7, timestamps, expiry/window calculations | shared crates and module call sites | Nearly all modules | Sometimes | Owning module | Established | Clock semantics and identifier policy are distributed |
| Error model | Map application failures to HTTP payloads | `backend/src/error.rs` | Routes, middleware, services | Category mapping only | Shared backend | High-coupling core | Internal/provider/database detail can reach messages |
| Authentication context | Bearer claims extraction and token helpers | `backend/src/middleware/auth.rs`, `backend/src/services/jwt.rs` | Protected and tenant modules | Yes, identity policy | Authentication | High-coupling | Broad type ownership and duplicated resource checks |
| Tenant policy context | Organization context, RBAC, RLS, quota, rate limit | `backend/src/middleware/tenant.rs`, related service modules | All tenant-protected domains | Yes | Tenant Authorization and RLS | High-coupling | Cross-cutting ownership and per-query enforcement ambiguity |
| Database helpers | Pool creation and migration execution | `backend/src/db/mod.rs` | Startup and all persistent modules | No | Bootstrap/Persistence | Small stable core | Fixed pool cap; direct SQL bypasses one access boundary |
| Application state | Share config, PostgreSQL, Redis, preview channels | `backend/src/state.rs` | Routers, handlers, middleware | No intended domain logic | Bootstrap and Runtime | Stable shape | Broad access and process-local preview state |
| Cache | Redis-backed delivery/cache helpers and invalidation | `backend/src/services/cache.rs`, delivery/content/pages call sites | Delivery, Content, Pages, rate/security helpers | Cache keys reflect domain concepts | Public Delivery and Cache; ownership overlaps | Cross-cutting | Invalidation ownership and outage behavior |
| Audit | Persist audit events | `backend/src/services/audit.rs` | Selected domain mutations | Event vocabulary is domain-aware | Shared service; domain producers | Cross-cutting | Inconsistent call-site coverage is unverified |
| File/media infrastructure | Upload storage, static serving, media inspection | `backend/src/routes/media.rs`, `backend/src/services/media_processing.rs`, upload service registration | Media and public upload consumers | Media policy included | Media | Feature-specific | Filesystem/database atomicity and replica storage |
| Email | Log or webhook email delivery behavior | `backend/src/services/email.rs` | Auth/org/billing/operational flows where invoked | Template/event selection may be domain-aware | Shared integration | Config-dependent | Delivery durability and failure-mode differences |
| Stripe | CMS subscription integration and Marketplace financial use | `backend/src/services/stripe_billing.rs`, Marketplace finance sources | Billing and Marketplace Finance | Provider adapter plus domain orchestration | Overlapping | Config/provider-dependent | Duplicate provider ownership and external failure handling |
| CMS webhooks | Registration/delivery behavior for CMS events | `backend/src/routes/webhooks.rs`, `backend/src/services/webhooks.rs` | CMS domains and outbound consumers | Yes, event types and policy | CMS Webhooks | Feature-specific | Retry, ordering, and durable dispatch are unclear |
| Built-in plugin contract | Register and execute built-in SEO behavior | `backend/src/plugins/mod.rs`, `backend/src/plugins/seo.rs`, `backend/src/routes/plugins.rs` | Plugin endpoints and page/content flows | Yes | Built-In Plugins | Narrow | Plugin/host coupling and future trust boundaries |
| Test helpers | Module-local constructors, sample data, and focused helper tests | `#[cfg(test)]` modules in backend source | Individual source modules | Test-only | Owning module | Distributed | Reuse and isolation are inconsistent |

## High-Coupling Hubs

`AppState`, `AppError`, `Claims`, `TenantContext`, `PgPool`, and `routes::router` are the main high-coupling structures. They are expected shared dependencies, but changes to their types or behavior can affect most modules. Marketplace domain/policy/package types form an additional dense feature-local hub.

## Absent or Unverified Shared Facilities

No general durable queue/worker subsystem, search engine client, application metrics exporter, dependency-injection container, repository framework, distributed event bus, or shared object-storage adapter was found. Standard HTTP tracing is present, but an operational metrics backend is `IMPLEMENTATION_STATUS_UNCLEAR`. Absence from the inspected repository does not prove an external deployment does not add such facilities.

## Ownership Guidance

Small helpers should remain documented here unless they acquire a distinct domain responsibility, public interface, independent lifecycle, or substantial test/dependency surface. Cross-cutting code that contains business decisions must retain an explicit domain owner rather than being labeled neutral infrastructure.

## Related Documentation

See the [Dependency Map](dependency-map.md), [Configuration and State](configuration-and-state.md), [Error Handling](error-handling.md), and each owning entry in the [Module Catalog](module-catalog.md).

