---
okf_document_id: "architecture-boundaries"
title: "System Boundaries"
project: "ZinharCMS"
category: "architecture"
phase: 2
status: "current"
review_status: "verified"
source_of_truth: false
architecture_status: "mixed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
primary_sources:
  - "backend/src/routes/mod.rs"
  - "backend/src/middleware/auth.rs"
  - "backend/src/middleware/tenant.rs"
  - "backend/src/services/rls.rs"
  - "backend/src/state.rs"
  - "backend/src/plugins/mod.rs"
  - "backend/src/services/marketplace_runtime.rs"
  - "frontend/src/services/api.ts"
  - "docker-compose.prod.yml"
related_documents:
  - "architecture/README.md"
  - "architecture/overview.md"
  - "architecture/components.md"
  - "architecture/dependency-model.md"
  - "architecture/runtime-flows.md"
  - "architecture/integration-points.md"
  - "architecture/architecture-risks.md"
  - "architecture/decisions/decision-register.md"
  - "backend/module-catalog.md"
  - "backend/module-boundaries.md"
  - "backend/backend-risks.md"
related_diagrams:
  - "architecture/diagrams/system-context.mmd"
  - "architecture/diagrams/container-view.mmd"
  - "architecture/diagrams/backend-request-flow.mmd"
  - "backend/diagrams/backend-module-map.mmd"
uncertainty_markers:
  - "UNKNOWN U-01"
  - "UNKNOWN U-08"
  - "NEEDS_OWNER_CONFIRMATION NOC-01"
  - "NEEDS_OWNER_CONFIRMATION NOC-09"
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR ABU-01"
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR ABU-02"
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR ABU-03"
  - "IMPLEMENTATION_STATUS_UNCLEAR ISU-01"
---

# System Boundaries

## System Boundary

The verified ZinharCMS system boundary contains the React SPA, the Rust/Axum backend, PostgreSQL, Redis, and backend-managed local files. Stripe and configured webhook receivers are external systems. Browser users and public-content consumers are external actors.

The boundary is repository-defined, not deployment-certified. Whether PostgreSQL, Redis, files, or Nginx are managed services in an actual environment is unknown.

## Trust and Request Boundaries

| Boundary | Entry condition | Enforcement location | Downstream scope |
|---|---|---|---|
| Public HTTP | No application identity required | Public router group and handler validation | Health, readiness, login/refresh, Stripe webhook, public delivery, uploaded files |
| Authenticated HTTP | Valid access token | Authentication middleware | Protected auth, beta, organization, and plugin endpoints |
| Tenant HTTP | Valid identity plus organization membership | Tenant middleware, route checks, services, and RLS helpers | CMS, pages, media, billing, webhooks, and Marketplace operations |
| WebSocket preview | Token and organization supplied for preview request | Preview route and tenant resolution | Process-local page broadcast channel |
| External provider callback | Provider-specific validation | Stripe signature verification or webhook receiver contract | Billing and integration state changes |
| Public file boundary | URL under `/uploads` | Static directory service | Files present below configured upload directory |

Authentication and tenant selection are separate. A valid user token does not itself establish organization membership. Client-side route guards improve navigation but are not an authorization boundary.

## Tenant Boundary

Organizations are the primary tenant. Tenant middleware reads `X-Organization-Id`, verifies membership, invokes rate and quota checks, and exposes `TenantContext` to downstream code. Preview handling also permits organization selection through its request query. RLS helpers set PostgreSQL session variables for organization and user context and deliberately close tenant-scoped connections when released.

This is a mixed enforcement model. Some modules use RLS-aware helpers, while route and service code also performs direct SQL and explicit ownership checks. Phase 2 verifies the mechanisms but does not certify complete coverage for every query. Comprehensive isolation verification remains a database and security phase responsibility.

## Backend Internal Boundaries

The source tree suggests technical layers—routes, middleware, services, plugins, models, database helpers, and shared state—but these are not hard package or process boundaries. Route handlers commonly perform SQL directly and coordinate cache, files, plugins, email, and webhooks. Services import middleware context types, and one health service imports a route-owned response type.

These observations are registered as:

- `ABU-01`: route, service, and persistence responsibilities are not consistently separated;
- `ABU-02`: data ownership is spread across route modules, services, migrations, and shared infrastructure;
- `ABU-03`: preview and background side effects remain inside the web-server process without a durable worker boundary.

## Frontend Boundary

The SPA owns browser presentation, route selection, local state, and request construction. The backend owns authorization and durable state. `frontend/src/services/api.ts` is the observed HTTP boundary, but its TypeScript models are maintained separately from Rust request and response types. `localStorage` holds access and refresh tokens and current organization state, while requests also opt into browser credentials.

No shared code package, generated API client, or generated schema contract joins the frontend and backend.

## Plugin and Marketplace Boundaries

Built-in plugins implement the Rust `CmsPlugin` abstraction and run inside the backend process. They share its trust and resource boundary.

Marketplace packages cross a different boundary. The backend stores package artifacts, validates metadata and policy, records installation and entitlement state, and exposes allowlisted host capabilities through runtime adapters. The verified Marketplace runtime reports execution as not executed; it does not load and run arbitrary uploaded package code. Documentation that implies unrestricted server execution is `PLANNED_NOT_IMPLEMENTED PNI-01`, not current architecture.

## Storage Boundary

PostgreSQL stores relational state. Redis stores cache and rate-limit data. The local upload directory stores media and Marketplace artifacts and is also exposed through the `/uploads` static route. File writes and database writes do not share one atomic transaction. The durability, shared-volume behavior, backup, private-object policy, CDN behavior, and multi-instance consistency of production storage require owner confirmation.

## Public Delivery Boundary

The public delivery handler currently resolves the active organization with the slug `default`, then reads through Redis and PostgreSQL. No verified public-domain-to-organization routing mechanism was found. This is `UNKNOWN U-08`, `NOC-01`, and `ISU-01`; a multi-tenant public delivery topology must not be inferred.

## External Boundaries

- Stripe is called through its HTTPS API and calls the public Stripe webhook endpoint.
- Email can be logged, disabled, or sent as JSON to a configured webhook target.
- CMS webhooks are sent to customer-configured HTTP or HTTPS endpoints with a CMS signature.
- PostgreSQL, Redis, filesystem, and provider credentials are selected through environment configuration.
- No external identity provider, message broker, search engine, metrics collector, or object-storage service is verified.

## Backend Boundary Catalog

| Boundary and status | Responsibility | Owned paths and high-level data | Public and internal interfaces | Dependencies and consumers |
|---|---|---|---|---|
| Process composition — `EXPLICIT` | Construct state, dependencies, router, layers, listener, and shutdown | `backend/src/main.rs`; runtime configuration and clients | Server listener and `AppState` construction | Depends on config, PostgreSQL, Redis, router; consumed by the entire backend process |
| Public HTTP — `EXPLICIT` | Expose unauthenticated or provider-validated routes | Public group in `backend/src/routes/mod.rs`; health, auth, delivery, callback, and file behavior | HTTP routes and static directory service | Depends on handlers and integrations; consumed by browsers, public consumers, and Stripe |
| Authenticated HTTP — `EXPLICIT` | Enforce token identity before protected non-tenant operations | Authenticated group and `backend/src/middleware/auth.rs`; identity and organization-management concepts | Bearer-token HTTP contract and attached `Claims` | Depends on JWT service and state; consumed by authenticated clients |
| Tenant HTTP — `EXPLICIT` | Establish membership, organization, rate, and quota context | Tenant group and `backend/src/middleware/tenant.rs`; organization-owned feature data | `X-Organization-Id`, `Claims`, and `TenantContext` | Depends on PostgreSQL, Redis, quota/rate services; consumed by CMS, page, billing, media, webhook, and Marketplace handlers |
| Feature routes — `OBSERVED` | Implement CMS, pages, billing, media, webhooks, plugins, and Marketplace HTTP behavior | Matching files in `backend/src/routes`; ownership is distributed across feature tables/files | Axum handlers, request/response types, direct service and SQL calls | Depends on middleware, services, plugins, state, SQLx, files, providers; consumed by clients |
| Reusable services and policy — `VIOLATED` | Provide JWT, RLS, RBAC, cache, quota, integrations, processing, and Marketplace policy | `backend/src/services`; service-owned data is not consistently isolated | Rust functions and types | Depends on state/infrastructure; imports middleware context types, and health imports a route type under DDU-01/DDU-02 |
| Built-in plugin host — `EXPLICIT` | Register and run trusted Rust plugin hooks | `backend/src/plugins`; plugin configuration and hook results | `CmsPlugin` interface and registry | Depends on backend types; consumed by content routes |
| Persistence ownership — `UNCLEAR` | Read and mutate relational state under ownership and RLS rules | SQL across routes/services plus `backend/migrations`; data ownership under ABU-02 | SQLx pool, connections, transactions, and RLS helpers | Consumed broadly by routes/services; no mandatory repository boundary |

## Frontend Boundary Catalog

| Boundary | Responsibility | Paths | Interfaces and dependencies | Status |
|---|---|---|---|---|
| Application shell | Providers, shared layout, global presentation | `frontend/src/main.tsx`; `frontend/src/components/AppShell.tsx` | Router, store, health hook, i18n, shared components | `OBSERVED` |
| Routing and guards | Public, authenticated, organization-scoped navigation | `frontend/src/router.tsx`; `frontend/src/components/RequireAuth.tsx`; `frontend/src/pages/WorkspaceRedirectPage.tsx` | Route definitions, store state, navigation | `EXPLICIT` |
| Feature pages | Admin, CMS, page-builder, SaaS, and Marketplace UI orchestration | `frontend/src/pages` | Components, hooks, i18n, store, API client, API types | `OBSERVED` |
| Shared UI | Reusable layout and presentation components | `frontend/src/components` | Props, hooks, styles, i18n | `OBSERVED` |
| State and authentication persistence | User, token, organization, and shared application state | `frontend/src/stores/useAppStore.ts`; API storage setters | Zustand API and `localStorage` | `UNCLEAR` ownership because persistence crosses store and API module |
| Backend integration | Central request construction and response parsing | `frontend/src/services/api.ts`; `frontend/src/types/api.ts` | HTTP/JSON/multipart/WebSocket URLs and manual types | `OBSERVED`; contract authority remains DDU-03 |

## Shared-Code Boundaries

| Sharing scope | Verified mechanism | Ownership status |
|---|---|---|
| Across backend modules | Public Rust module functions/types, `AppState`, middleware contexts, models, services, and plugin traits | Mixed; cross-layer context ownership is DDU-01/DDU-02 |
| Across frontend features | Shared components, hooks, Zustand store, i18n, API client, and API types | Observed; session persistence ownership is distributed |
| Between server and client | Runtime HTTP/JSON/multipart/WebSocket contract only | No shared source or generated schema; DDU-03 |
| Tests | Rust tests colocated with backend modules; frontend test setup and page tests share testing utilities | Clear at tooling level; coverage is selective |
| Repository tooling | Root scripts, Marketplace CLI, Compose, Dockerfiles, and CI workflows | Repository-level ownership is not identified under U-12/NOC-15 |
| Plugins | Built-in Rust trait and registry; Marketplace allowlisted host adapters | Two distinct trust models; arbitrary server package execution remains PNI-01 |

## Data Ownership Boundaries

| Data concept | Apparent owner | Access pattern | Boundary status |
|---|---|---|---|
| Users, organizations, memberships, roles | Authentication, organization, tenant, and RBAC modules | Direct SQL plus middleware/service checks | `OBSERVED`, with shared cross-cutting ownership |
| Content types, entries, comments | Content/comment routes and plugin hooks | Direct SQL, ownership checks, RLS-aware operations | `UNCLEAR` at route/service/persistence split under ABU-01/ABU-02 |
| Pages, versions, preview | Page routes and preview channels | PostgreSQL plus process-local broadcast state | `OBSERVED`; durability boundary is ABU-03 |
| Media metadata and files | Media routes and processing service | PostgreSQL plus organization-scoped filesystem | `UNCLEAR` atomic ownership across database and files |
| Billing and Stripe state | Billing routes, Stripe service, migrations | PostgreSQL and Stripe HTTPS/webhook | `OBSERVED`; provider operations cross transaction boundaries |
| Marketplace catalog, packages, installations, finance, feedback | Marketplace routes, services, and migrations | Direct SQL, local package files, Stripe, and adapters | `UNCLEAR` because a broad domain spans multiple route/service groups |
| Delivery cache | Delivery route and cache service | Redis with PostgreSQL fallback | `OBSERVED`; invalidation remains distributed |

## Cross-Boundary Communication

| Mechanism | Verified use | Absent or constrained alternative |
|---|---|---|
| Direct Rust calls | Routes, middleware, services, plugins, and helpers inside one process | No network boundary between backend capabilities |
| HTTP/JSON and multipart | SPA and external callers to backend; backend to providers | No shared/generated server-client package |
| WebSocket | Page preview | Process-local fan-out only |
| SQL | Routes/services to PostgreSQL through SQLx and RLS helpers | No enforced repository layer |
| Redis protocol | Cache, rate limiting, and readiness | Not used as verified page-preview pub/sub or durable queue |
| Shared process state | `AppState` dependencies and preview channel map | Not shared across replicas |
| Plugin interface | In-process `CmsPlugin` hooks and Marketplace host adapters | No arbitrary uploaded server-code execution |
| Durable events or queues | None verified | Spawned tasks are non-durable and remain ABU-03 |

## Boundary Violations and Risks

| Finding | Evidence | Marker or risk |
|---|---|---|
| HTTP handlers directly access SQL and infrastructure | `backend/src/routes`; SQLx, state, Redis, file, and provider calls | ABU-01; AR-001 |
| Multiple route/service modules access related data directly | `backend/src/routes`; `backend/src/services`; migrations | ABU-02; AR-007 |
| Services import middleware-owned context types | JWT, quota, rate-limit, and RLS service imports | DDU-01; AR-002 |
| Health service imports a route-owned response type | `backend/src/services/health.rs` | DDU-02; AR-002 |
| Browser contracts duplicate backend behavior manually | `frontend/src/types/api.ts` and route types | DDU-03; AR-003 |
| No frontend feature bypass of central `fetch` client was found | Verified `fetch` search under `frontend/src` | Current control; recheck when adding integration code |
| Infrastructure concerns leak into feature orchestration | File, cache, SQL, email, Stripe, and webhook calls in handlers | ABU-01; AR-001/AR-006 |
| Preview and webhooks depend on process lifetime | `AppState` broadcast map and spawned tasks | ABU-03; AR-004/AR-005 |
| Pages and Marketplace adapters import across apparent domain/layer ownership | `backend/src/routes/pages.rs`; `backend/src/services/marketplace_adapters.rs` | DDU-04; BE-RISK-001/003 |
| Content and Pages invoke invalidation behavior owned by Delivery routes | affected route imports and calls | DDU-05; BE-RISK-001/007 |
| CMS Billing and Marketplace Finance separately own Stripe-facing business behavior | billing and Marketplace finance route/service sources | RO-03; BE-RISK-012 |

## Related Architecture Views

- [Components and Responsibilities](components.md)
- [Dependency Model](dependency-model.md)
- [Runtime Flows](runtime-flows.md)
- [Integration Points](integration-points.md)
- [Backend Request Flow Diagram](diagrams/backend-request-flow.mmd)
- [System Context Diagram](diagrams/system-context.mmd)
- [Backend Module Catalog](../backend/module-catalog.md)
- [Backend Module Boundaries](../backend/module-boundaries.md)
- [Backend Module Map](../backend/diagrams/backend-module-map.mmd)
- [Backend Risk Register](../backend/backend-risks.md)
