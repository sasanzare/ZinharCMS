---
okf_document_id: "backend-overview"
title: "Backend Overview"
project: "ZinharCMS"
category: "backend"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
primary_sources:
  - "backend/Cargo.toml"
  - "backend/src/main.rs"
  - "backend/src/lib.rs"
  - "backend/src/routes/mod.rs"
  - "backend/src/state.rs"
  - "backend/src/config.rs"
  - "backend/src/error.rs"
related_documents:
  - "backend/README.md"
  - "backend/module-catalog.md"
  - "backend/request-handling.md"
  - "backend/configuration-and-state.md"
  - "backend/shared-infrastructure.md"
  - "backend/backend-risks.md"
  - "architecture/overview.md"
related_diagrams:
  - "backend/diagrams/backend-module-map.mmd"
  - "backend/diagrams/backend-request-lifecycle.mmd"
  - "backend/diagrams/application-state-composition.mmd"
uncertainty_markers:
  - "UNKNOWN"
  - "NEEDS_OWNER_CONFIRMATION"
  - "INFERRED_FROM_CODE"
  - "IMPLEMENTATION_STATUS_UNCLEAR"
---

# Backend Overview

## Backend Identity

| Field | Verified value |
|---|---|
| Language and edition | Rust 2024 |
| Main framework/runtime | Axum 0.8 on Tokio |
| Package structure | One Cargo package, `cms-backend`; no Cargo workspace at repository root |
| Executable entry | `backend/src/main.rs` |
| Library composition | `backend/src/lib.rs` and `routes::router` |
| Runtime purpose | Multi-tenant CMS/SaaS/Marketplace HTTP and WebSocket backend |
| Status | `VERIFIED` implemented repository backend; actual production deployment remains `UNKNOWN` |

## Backend Responsibilities

Verified implemented responsibilities include authentication, organization tenancy, CMS content/pages/comments/media, delivery/cache, billing/quota, CMS webhooks, built-in plugins, beta operations, and Marketplace creator/review/catalog/install/runtime/finance/feedback/analytics capabilities.

Partially implemented or operationally incomplete responsibilities include public domain routing, durable side-effect delivery, production storage/observability/recovery, automated Marketplace transfers, partial refunds/disputes, and external notification/telemetry outcomes. These are not described as implemented.

Readiness scripts/tests and production-like configuration do not prove deployment, beta, or GA state; those remain `IMPLEMENTATION_STATUS_UNCLEAR`.

## Backend Structure

The backend is a hybrid technical/feature modular monolith. One crate/process contains route modules organized primarily by capability, middleware, reusable services, selected models, built-in plugins, shared state/configuration, and direct SQLx persistence. Module boundaries are not independent packages and are only partly enforced by Rust visibility, traits, and router registration.

## Main Execution Path

1. `main` loads optional dotenv state and initializes tracing.
2. `Config::from_env` validates required configuration.
3. A lazy PostgreSQL pool is created; embedded migrations run.
4. Initial administrator/organization membership bootstrap runs when required.
5. A Redis client and `AppState` are constructed.
6. `cms_backend::app` composes the root router and public/authenticated/tenant groups.
7. Global Tower layers add timeout, security headers, compression, CORS, request IDs, and tracing.
8. Tokio binds the listener and Axum serves requests.
9. Ctrl+C or Unix termination triggers graceful server shutdown.

## Main Backend Layers

| Observed layer | Paths | Actual responsibility |
|---|---|---|
| Bootstrap/composition | `main.rs`, `lib.rs`, `config.rs`, `state.rs` | Construct process and shared dependencies |
| HTTP boundary | `routes/*.rs` | Routers, DTOs, handlers, validation, SQL, orchestration |
| Middleware/context | `middleware/*.rs` | Security headers, claims, tenant/rate/quota context |
| Services/policy | `services/*.rs` | Reusable rules, integrations, processing and tests |
| Models | `models/*.rs` | Selected SQLx/domain rows; not a complete domain layer |
| Plugins | `plugins/*.rs` | Trusted in-process hook interface and implementation |
| Persistence | SQLx call sites, `db/mod.rs`, migrations | Direct queries, transactions, RLS and schema evolution |

## Shared Backend Concerns

`AppError` provides common HTTP conversion; tracing/Tower layers cover request observability; serde/utoipa support serialization/schema metadata; auth/tenant middleware provides context; SQLx and RLS helpers provide persistence; `AppState` shares config/pool/Redis/preview channels; Tokio tasks and broadcast channels support limited transient work; built-in plugins expose in-process hooks.

No durable worker, message queue, search service, object-store adapter, external identity provider, metrics exporter, or uploaded-code executor is verified.

## Known Backend Unknowns

- Production topology, ownership, recovery and SLOs are `UNKNOWN`.
- Intended module ownership and public interfaces are not governed beyond current structure.
- Query-level tenant/RLS coverage requires later database/security verification.
- Business rationale behind thresholds and transitions is `BUSINESS_RULE_UNVERIFIED` when code establishes behavior but not intent.
- Test coverage percentages are `UNKNOWN`; no coverage report was used.
- See [Backend Risks](backend-risks.md) and [Module Boundaries](module-boundaries.md).
