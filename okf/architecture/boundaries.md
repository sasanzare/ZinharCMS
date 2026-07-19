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
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/routes/mod.rs"
  - "backend/src/middleware/auth.rs"
  - "backend/src/middleware/tenant.rs"
  - "backend/src/services/rls.rs"
  - "backend/src/state.rs"
  - "backend/src/plugins/mod.rs"
  - "backend/src/services/marketplace_runtime.rs"
  - "frontend/src/services/api.ts"
  - "frontend/src/router.tsx"
  - "frontend/src/components"
  - "frontend/src/stores/useAppStore.ts"
  - "frontend/src/pages"
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
  - "frontend/feature-boundaries.md"
  - "frontend/authentication-and-access.md"
  - "frontend/api-client.md"
  - "frontend/frontend-risks.md"
related_diagrams:
  - "architecture/diagrams/system-context.mmd"
  - "architecture/diagrams/container-view.mmd"
  - "architecture/diagrams/backend-request-flow.mmd"
  - "backend/diagrams/backend-module-map.mmd"
  - "frontend/diagrams/frontend-application-map.mmd"
  - "frontend/diagrams/frontend-api-flow.mmd"
uncertainty_markers:
  - "UNKNOWN U-01"
  - "UNKNOWN U-08"
  - "NEEDS_OWNER_CONFIRMATION NOC-01"
  - "NEEDS_OWNER_CONFIRMATION NOC-09"
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR ABU-01"
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR ABU-02"
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR ABU-03"
  - "IMPLEMENTATION_STATUS_UNCLEAR ISU-01"
  - "FEATURE_BOUNDARY_UNCLEAR FBU-01"
  - "STATE_OWNERSHIP_UNCLEAR SOU-01"
  - "API_CONTRACT_UNCLEAR ACU-01"
  - "AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01"
  - "COMPONENT_OWNERSHIP_UNCLEAR COU-01"
  - "UI_BEHAVIOR_UNVERIFIED UBU-01"
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

The frontend route guard checks only for a stored access token. Static navigation is visible to all client-authenticated users, while selected pages hide or disable actions using membership/global roles. These are experience boundaries, not security boundaries, and remain `AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01` until the Phase 7 route/action matrix is verified against backend enforcement.

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
| Application boundary | One React/Vite management SPA and production-like Nginx image | `frontend/`; `frontend/src/main.tsx`; frontend Dockerfiles/Nginx | Browser entry, one build artifact, backend base URL | `EXPLICIT`; deployment state unclear |
| Application shell | Providers, shared layout, global presentation | `frontend/src/main.tsx`; `frontend/src/components/AppShell.tsx` | Router, store, health hook, i18n, shared components | `OVERLAPPING` under FBU-02/FRO-02 |
| Routing and guards | Public login, token-admitted protected routes, workspace selection | `frontend/src/router.tsx`; `frontend/src/components/RequireAuth.tsx`; `frontend/src/pages/WorkspaceRedirectPage.tsx` | Eager route definitions, token state, navigation | `EXPLICIT`; authorization remains ABV-01 |
| Feature pages | Admin, CMS, page-builder, SaaS, and Marketplace UI orchestration | `frontend/src/pages` | Components, hooks, i18n, store, API client, API types | `OBSERVED` to `OVERLAPPING`; see Phase 4 catalog |
| Shared UI | Reusable layout and presentation components | `frontend/src/components` | Props, hooks, styles, i18n | `OBSERVED` |
| State and authentication persistence | User, token, organization, and shared application state | `frontend/src/stores/useAppStore.ts`; API storage setters | Zustand API and `localStorage` | `UNCLEAR` ownership because persistence crosses store and API module |
| Backend integration | Central request construction and response parsing | `frontend/src/services/api.ts`; `frontend/src/types/api.ts` | HTTP/JSON/multipart/WebSocket URLs and manual types | `OBSERVED`; contract authority remains DDU-03 |
| Localization and direction | Locale selection, fallback, translation, document direction | `frontend/src/i18n`; direction-aware CSS | React context, browser storage, global styles | `EXPLICIT`; browser behavior UBU-01 |
| Styling and design vocabulary | Global semantic classes, Tailwind tooling, responsive and RTL rules | `frontend/src/styles/index.css`; Vite config | Global class names and feature markup | `UNCLEAR` formal ownership under COU-01 |
| Page Builder | Page composition, local preview, persistence, workflow, versions, templates, preview handoff | `frontend/src/pages/PagesPage.tsx` | dnd-kit, page/component/Marketplace APIs, store session context | `OVERLAPPING` under FRO-01 |

The detailed route/page/component/state/API/schema/test/permission/asset ownership matrix is maintained in [Frontend Feature Boundaries](../frontend/feature-boundaries.md). It selects 13 significant frontend features without treating each source file as an independent boundary.

## Shared-Code Boundaries

| Sharing scope | Verified mechanism | Ownership status |
|---|---|---|
| Across backend modules | Public Rust module functions/types, `AppState`, middleware contexts, models, services, and plugin traits | Mixed; cross-layer context ownership is DDU-01/DDU-02 |
| Across frontend features | Shared components, hooks, Zustand store, i18n, API client, and API types | Observed; session persistence ownership is distributed |
| Between server and client | Runtime HTTP/JSON/multipart/WebSocket contract only | No shared source or generated schema; DDU-03 |
| Tests | Rust tests colocated with backend modules; frontend test setup and page tests share testing utilities | Clear at tooling level; coverage is selective |
| Repository tooling | Root scripts, Marketplace CLI, Compose, Dockerfiles, and CI workflows | Repository-level ownership is not identified under U-12/NOC-15 |
| Plugins | Built-in Rust trait and registry; Marketplace allowlisted host adapters | Two distinct trust models; arbitrary server package execution remains PNI-01 |

Frontend source sharing is not package-enforced. Route pages can import the central store, API, types, i18n, shared components, and global class names directly. Marketplace, Pages, and Organization are high-overlap boundaries, while `i18n/index.ts` is the clearest explicit frontend module interface.

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

## Phase 5 Persistence Boundary Findings

- The database boundary is shared across modules; no mandatory repository/DAO layer owns SQL access (`PBU-01`).
- Thirty-two tables have forced RLS in migration intent, but the hardening verifier lists only 24 (`DCC-12`).
- Global and tenant tables coexist; privileged bypass transactions are a security-sensitive boundary.
- Parent and organization FKs do not universally prove same-tenant coherence (`TIV-01`, `CCU-01`).
- Media files, Stripe operations, post-commit audit, and spawned webhooks cross database atomicity (`TBU-01` through `TBU-04`).

Use [Database Module Data Ownership](../database/module-data-ownership.md), [Multi-Tenancy](../database/multi-tenancy.md), and [Transactions and Consistency](../database/transactions-and-consistency.md) before changing these boundaries.

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

## Phase 7 Security Boundary Refinement

The browser/API, public router, bearer middleware, tenant membership, handler RBAC/ownership, tenant SQL/RLS, filesystem, Redis, and external-provider boundaries are mapped in [Security Trust Boundaries](../security/trust-boundaries.md). A global role is not a tenant membership, and explicit RLS bypass is a backend transaction mode rather than an automatic administrator privilege.

## Phase 8 Domain Boundary Refinement

The [Domain Catalog](../domain/domain-catalog.md) identifies ten behavior boundaries without asserting separate deployable services. Cross-domain workflows commonly commit PostgreSQL state before cache invalidation, filesystem work, email, webhooks, or provider calls. Marketplace packages cross a host-adapter permission boundary; CMS built-in plugins remain compiled, process-local behavior. These distinctions are summarized in [Domain Events](../domain/domain-events.md) and [Domain Risks](../domain/domain-risks.md).
