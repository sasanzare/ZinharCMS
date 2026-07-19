---
okf_document_id: "architecture-overview"
title: "Architecture Overview"
project: "ZinharCMS"
category: "architecture"
phase: 2
status: "current"
review_status: "verified"
source_of_truth: false
architecture_status: "mixed"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-17"
primary_sources:
  - "backend/Cargo.toml"
  - "backend/src/main.rs"
  - "backend/src/lib.rs"
  - "backend/src/state.rs"
  - "backend/src/routes/mod.rs"
  - "frontend/package.json"
  - "frontend/src/main.tsx"
  - "frontend/src/router.tsx"
  - "docker-compose.yml"
  - "docker-compose.prod.yml"
related_documents:
  - "architecture/README.md"
  - "architecture/boundaries.md"
  - "architecture/components.md"
  - "architecture/dependency-model.md"
  - "architecture/runtime-flows.md"
  - "architecture/integration-points.md"
  - "architecture/architecture-risks.md"
  - "architecture/decisions/decision-register.md"
related_diagrams:
  - "architecture/diagrams/system-context.mmd"
  - "architecture/diagrams/container-view.mmd"
  - "architecture/diagrams/dependency-direction.mmd"
uncertainty_markers:
  - "INFERRED_FROM_CODE"
  - "INFERRED_FROM_STRUCTURE"
  - "INFERRED_FROM_CONFIGURATION"
  - "UNKNOWN U-01"
  - "NEEDS_OWNER_CONFIRMATION NOC-06"
  - "IMPLEMENTATION_STATUS_UNCLEAR ISU-03"
---

# Architecture Overview

## Architecture Summary

ZinharCMS is an observed client-server system. The browser runs a React single-page administration application. That client communicates through HTTP, JSON, multipart requests, and one WebSocket preview flow with a Rust/Axum backend. The backend is best classified as a modular monolith: one Cargo crate builds one server process containing authentication, tenancy, CMS, page builder, billing, media, webhook, plugin, and Marketplace capabilities.

The repository does not contain independently deployable domain services, a durable job worker, a message broker, a separate search service, an object-storage adapter, an API gateway, or an external identity provider. Calling the backend a microservice architecture would therefore contradict current evidence.

## Major Runtime Components

| Name | Responsibility and runtime role | Implementation and technology | Inputs and outputs | Dependencies | Deployment status | Evidence |
|---|---|---|---|---|---|---|
| React SPA | Browser presentation, navigation, state, and backend calls | `frontend/src`; React, TypeScript, Vite | User actions and backend responses; DOM and backend requests | Browser, central API client, backend contract | Local process and production-like Nginx artifact verified; actual production unknown | `frontend/package.json`; `frontend/src/main.tsx`; `frontend/src/router.tsx` |
| Axum backend | HTTP/WebSocket server, identity, tenant enforcement, orchestration, persistence, and integrations | `backend/src`; Rust, Axum, Tokio | HTTP/WebSocket requests; JSON, files, status, and WebSocket messages | PostgreSQL, Redis, files, configured HTTPS targets | Local process and production-like image verified; actual production unknown | `backend/Cargo.toml`; `backend/src/main.rs`; `backend/src/routes/mod.rs` |
| PostgreSQL | Durable relational system of record and RLS substrate | `backend/migrations`; PostgreSQL 16 in Compose | SQL and session context; rows and transaction results | Backend and persistent volume in tracked Compose | Local and production-like configuration verified; deployed service unknown | `backend/migrations`; `docker-compose.yml`; `docker-compose.prod.yml` |
| Redis | Delivery cache, rate-limit state, and readiness dependency | Redis 7 in Compose; Redis client in backend | Keys and values; cache/rate results and dependency status | Backend and persistent volume in tracked Compose | Local and production-like configuration verified; deployed service unknown | `backend/src/services/cache.rs`; `backend/src/services/rate_limit.rs`; Compose files |
| Upload filesystem | Media originals/variants and Marketplace package artifacts | Files below configured `UPLOAD_DIR` | Multipart/package bytes; stored and statically served files | Backend host or volume | Local and production-like volume behavior verified; production durability unknown | `backend/src/config.rs`; `backend/src/routes/media.rs`; `backend/src/routes/marketplace.rs` |
| Nginx SPA server | Serve built frontend assets and client-route fallback | `frontend/nginx.conf`; Nginx | Browser HTTP requests; static assets and `index.html` fallback | Built frontend artifact | Production-like image only; actual ingress and production status unknown | `frontend/Dockerfile.prod`; `frontend/nginx.conf` |
| Built-in plugin host | Resolve and run trusted Rust plugin hooks in the backend | `backend/src/plugins`; in-process Rust traits | Content hook context; validation or transformed hook results | Route handlers and backend process | Implemented inside backend; not independently deployed | `backend/src/plugins/mod.rs`; `backend/src/plugins/seo.rs` |
| Configured HTTPS integrations | Stripe, email webhook, and CMS webhook communication | Backend HTTP clients and public/provider routes | Provider requests/callbacks and signed webhook payloads | Network, credentials, provider availability | Code paths verified when configured; live production wiring unknown | `backend/src/services/stripe_billing.rs`; `backend/src/services/email.rs`; `backend/src/services/webhooks.rs` |
| In-process transient work | Page-preview broadcasts and spawned webhook delivery tasks | Tokio broadcast channels and spawned futures | Preview messages and webhook payloads; transient delivery results | Backend process memory and network | Implemented; not durable or independently deployed | `backend/src/state.rs`; `backend/src/routes/pages.rs`; `backend/src/services/webhooks.rs` |

No verified queue, durable worker, object store, search engine, external identity provider, API gateway, metrics exporter, or APM collector is included as a current component.

## Backend Composition

`backend/src/main.rs` loads environment configuration, initializes tracing, constructs a SQLx PostgreSQL pool and Redis client, runs embedded migrations, seeds an administrator when the users table is empty, constructs shared `AppState`, assembles the router, adds global Tower layers, binds the listener, and handles graceful shutdown.

The router separates three request groups:

1. public endpoints, including health, readiness, public authentication, Stripe webhook reception, public delivery, and uploaded-file serving;
2. authentication-protected endpoints, including protected authentication, beta operations, organization operations, and built-in plugin routes;
3. tenant-protected endpoints, including content, pages, comments, billing, media, webhooks, and Marketplace route groups.

Tenant middleware resolves identity when necessary, requires an organization identifier, loads membership, applies rate and quota checks, and inserts tenant context for downstream handlers. PostgreSQL RLS helpers add organization and user context for queries that use those helpers. Detailed enforcement coverage belongs to the later security and database phases.

## Frontend Composition

`frontend/src/main.tsx` initializes the React application and routing. `frontend/src/router.tsx` defines public, authenticated, and organization-scoped UI routes. `RequireAuth` and workspace redirection implement client navigation guards, while `useAppStore` holds shared authentication and organization state. All observed browser `fetch` use is centralized in `frontend/src/services/api.ts`; frontend request and response types are maintained manually in `frontend/src/types/api.ts`.

## Data and State

PostgreSQL is the durable system of record. Redis has different failure behavior by use case: delivery cache operations can fall back to PostgreSQL, while tenant rate limiting and readiness can reject requests when Redis is unavailable. Media and Marketplace package files are stored on the local filesystem selected by `UPLOAD_DIR`. Page-preview channels are held in a process-local `HashMap` of broadcast senders and therefore do not form shared multi-instance state.

## Architectural Layers

| Actual layer or area | Purpose and source paths | Observed dependency direction | Violation or unclear boundary |
|---|---|---|---|
| Browser presentation | Pages and shared components under `frontend/src/pages` and `frontend/src/components` | Pages use components, hooks, state, i18n, types, and API client | Large pages combine presentation and feature orchestration |
| Frontend routing and state | `frontend/src/router.tsx`; `frontend/src/stores/useAppStore.ts` | Entry point to router; routes to pages; pages and API client to shared state | Session persistence is shared between store logic, API setters, and `localStorage` |
| Frontend integration | `frontend/src/services/api.ts`; `frontend/src/types/api.ts` | UI to central client to backend HTTP contract | TypeScript contracts manually duplicate Rust behavior under DDU-03 |
| HTTP routing and transport | `backend/src/routes/mod.rs`; `backend/src/routes` | Router to middleware and handlers; handlers to services and infrastructure | Handlers also own SQL, files, policy, and integration orchestration under ABU-01 |
| Middleware and request context | `backend/src/middleware` | Routes to middleware; middleware to selected services and state | Services import middleware-owned context types under DDU-01 |
| Service and policy modules | `backend/src/services`; `backend/src/plugins` | Routes to services/plugins; services to infrastructure and other services | One service imports a route response type under DDU-02; domain and infrastructure roles are mixed |
| Persistence and infrastructure | SQLx calls, `backend/src/services/rls.rs`, cache/files/provider adapters | Routes and services to PostgreSQL, Redis, files, and HTTP providers | No enforced repository layer or single data owner under ABU-02 |

Allowed dependency direction is an inferred convention, not an enforced architecture rule. Current imports and runtime behavior remain authoritative when the table and code differ.

## Architectural Style

| Pattern | Classification | Evidence and limit |
|---|---|---|
| Client-server | Observed | Separate React browser application and Rust HTTP/WebSocket backend |
| Modular monolith | Inferred from code and structure | One crate and process contains technical and feature modules; there are no independent domain services |
| Shared-state dependency injection | Observed | `AppState` is constructed in `main.rs` and injected into Axum handlers |
| Middleware pipeline | Observed | Global Tower layers plus authentication and tenant router middleware |
| Handler-service-direct-persistence mix | Observed | Services exist, but handlers also issue SQL and call infrastructure directly; no repository pattern is enforced |
| Feature-page frontend with central API client | Observed | Pages are feature-oriented and all verified `fetch` use is in `services/api.ts` |
| Plugin-based extension | Observed | Trusted built-in `CmsPlugin` hooks execute in process; Marketplace packages follow a non-executing policy/adapter path |
| Event-driven or queue architecture | Not observed | Spawned tasks and broadcast channels are transient in-process mechanisms, not a durable event system |

## Runtime Topology

### Verified Local-Development Topology

The default Compose file runs PostgreSQL, Redis, and pgAdmin. Root development commands start backend and frontend application processes separately from that infrastructure.

### Verified Container Topology

The production-like Compose file defines PostgreSQL, Redis, backend, and frontend services. Its frontend image uses Nginx for SPA files and route fallback; that Nginx configuration does not proxy backend API traffic.

### Verified Deployment Topology

No deployed environment, hosting provider, ingress, TLS, replica, DNS, secrets, or release evidence is present. Therefore no production deployment topology is verified.

### Unknown Production Topology

These files establish a reference topology only. The hosting provider, load balancer, TLS termination, DNS, secrets delivery, replica count, promotion process, and actually deployed topology are `UNKNOWN U-01` and require `NOC-06`. Beta or GA state remains `ISU-03`.

## Architectural Characteristics

- A single backend deployment unit simplifies local composition and transactions but concentrates responsibilities.
- Route handlers frequently combine HTTP handling, SQL, domain checks, and integration orchestration; layer boundaries are not consistently enforced.
- Organization tenancy is a cross-cutting boundary implemented through middleware, direct ownership checks, and PostgreSQL RLS helpers.
- Frontend-to-backend contracts are manually mirrored rather than generated from one schema authority.
- External side effects are generally synchronous or spawned in the server process; no durable delivery queue is present.
- Marketplace packages are stored and inspected, but arbitrary uploaded package code is not executed by the official backend runtime.

## Architectural Constraints

| Constraint | Verified effect | Evidence or marker |
|---|---|---|
| Rust/Axum backend package | Backend modules compile and deploy together as one crate/process | `backend/Cargo.toml`; `backend/src/lib.rs` |
| React/Vite frontend package | UI builds separately and communicates through runtime contracts | `frontend/package.json`; `frontend/src/services/api.ts` |
| No shared server/client package | Rust and TypeScript contract changes require coordinated manual validation | DDU-03 |
| PostgreSQL schema and RLS | Migrations are schema authority; tenant-aware data paths require organization context | `backend/migrations`; `backend/src/services/rls.rs` |
| Organization tenant boundary | Protected feature requests require identity, organization, membership, rate, and quota context | `backend/src/middleware/tenant.rs` |
| Central API client | Verified browser `fetch` calls pass through one module | `frontend/src/services/api.ts` |
| Built-in versus Marketplace plugin trust | Built-in Rust plugins run in process; arbitrary uploaded server code is not executed | `backend/src/plugins`; `backend/src/services/marketplace_runtime.rs`; PNI-01 |
| Local artifact storage | Media and package durability depend on configured filesystem topology | `UPLOAD_DIR`; NOC-02 |
| Reference deployment only | Tracked Compose and Dockerfiles cannot establish actual production architecture | U-01; NOC-06; ISU-03 |

## Known Architecture Unknowns

- Production hosting, ingress, TLS, replicas, storage, backups, observability, promotion, and ownership remain `UNKNOWN` or `NEEDS_OWNER_CONFIRMATION`.
- Public domain-to-organization delivery is unresolved under U-08, NOC-01, and ISU-01.
- Route/service/persistence ownership remains unclear under ABU-01 and ABU-02; see [System Boundaries](boundaries.md) and [Components](components.md).
- Middleware/service and server/client contract direction remains unclear under DDU-01 through DDU-03; see [Dependency Model](dependency-model.md).
- Durable post-mutation delivery remains unresolved under ABU-03 and NOC-09; see [Runtime Flows](runtime-flows.md).
- Consolidated severity and follow-up are in [Architecture Risks](architecture-risks.md).

## Confidence Statement

Classification, process boundaries, runtime dependencies, and repository-defined request composition have high confidence because they are directly visible in code and executable configuration. Actual production deployment, operational ownership, scaling behavior, and release state have low confidence because repository evidence does not establish them.

## Related Architecture Views

- [System Boundaries](boundaries.md)
- [Components and Responsibilities](components.md)
- [Dependency Model](dependency-model.md)
- [Runtime Flows](runtime-flows.md)
- [Integration Points](integration-points.md)
- [Architecture Risks](architecture-risks.md)
- [Architecture Decision Register](decisions/decision-register.md)
- [System Context Diagram](diagrams/system-context.mmd)
- [Container View Diagram](diagrams/container-view.mmd)
- [Dependency Direction Diagram](diagrams/dependency-direction.mmd)

## Operational Architecture Status

The verified deployment reference is a modular-monolith backend, static React/Nginx frontend, PostgreSQL, Redis, and local upload storage assembled by production-like Compose. Backend startup owns migrations, bootstrap seed, request tracing, and graceful shutdown. No production provider, ingress/TLS, replica model, durable worker, metrics/alerts, backup automation, or disaster-recovery topology is established. See [Runtime Topology](../operations/runtime-topology.md), [Deployment Workflow](../delivery/deployment-workflow.md), and [Operational Risks](../operations/operational-risks.md).
