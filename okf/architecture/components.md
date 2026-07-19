---
okf_document_id: "architecture-components"
title: "Components and Responsibilities"
project: "ZinharCMS"
category: "architecture"
phase: 2
status: "current"
review_status: "verified"
source_of_truth: false
architecture_status: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
primary_sources:
  - "backend/src/main.rs"
  - "backend/src/state.rs"
  - "backend/src/routes/mod.rs"
  - "backend/src/middleware"
  - "backend/src/services"
  - "backend/src/plugins"
  - "frontend/src/main.tsx"
  - "frontend/src/router.tsx"
  - "frontend/src/services/api.ts"
  - "frontend/src/stores/useAppStore.ts"
  - "frontend/src/components"
  - "frontend/src/pages"
  - "frontend/src/i18n"
  - "frontend/src/styles/index.css"
related_documents:
  - "architecture/README.md"
  - "architecture/overview.md"
  - "architecture/boundaries.md"
  - "architecture/dependency-model.md"
  - "architecture/runtime-flows.md"
  - "architecture/integration-points.md"
  - "architecture/architecture-risks.md"
  - "project/repository-map.md"
  - "backend/README.md"
  - "backend/module-catalog.md"
  - "backend/shared-infrastructure.md"
  - "frontend/README.md"
  - "frontend/application-catalog.md"
  - "frontend/feature-catalog.md"
  - "frontend/component-architecture.md"
related_diagrams:
  - "architecture/diagrams/container-view.mmd"
  - "architecture/diagrams/backend-request-flow.mmd"
  - "architecture/diagrams/frontend-backend-flow.mmd"
  - "backend/diagrams/backend-module-map.mmd"
  - "backend/diagrams/application-state-composition.mmd"
  - "frontend/diagrams/frontend-application-map.mmd"
uncertainty_markers:
  - "INFERRED_FROM_CODE"
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR ABU-01"
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR ABU-02"
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR ABU-03"
  - "FEATURE_BOUNDARY_UNCLEAR FBU-01"
  - "COMPONENT_OWNERSHIP_UNCLEAR COU-01"
---

# Components and Responsibilities

## Runtime Components

| Component | Primary responsibilities | State and dependencies |
|---|---|---|
| React SPA | User interface, route composition, client guards, locale context, shared browser state, API requests | Browser storage; backend HTTP and WebSocket endpoints |
| Axum backend | Routing, authentication, tenancy, business behavior, persistence orchestration, plugins, files, integrations | PostgreSQL, Redis, filesystem, external HTTPS targets, process memory |
| PostgreSQL | Durable application and tenant data; migration history; RLS policy substrate | Accessed through SQLx pool and tenant-aware helpers |
| Redis | Delivery caching and tenant rate-limit state; readiness dependency | Accessed through Redis client from request paths |
| Upload filesystem | Media originals/variants and Marketplace package artifacts | Selected by backend configuration and served under `/uploads` |
| Nginx frontend image | Built SPA asset serving and client-side route fallback | Present only in production-like frontend configuration |

## Architectural Component Catalog

| Component | Type; runtime status; confidence | Source and entry point | Responsibility and exposed interfaces | Data, configuration, and main dependencies | Related tests and diagram | Future OKF phase |
|---|---|---|---|---|---|---|
| React administration SPA | UI application; implemented; `VERIFIED` | `frontend/src`; entry `frontend/src/main.tsx` | Browser routes, pages, state, presentation; exposes browser UI and backend requests | User/session/org browser state; Vite environment; depends on React, router, Zustand, i18n, central API client | Page tests and `frontend/src/test/setup.ts`; [Frontend-Backend Flow](diagrams/frontend-backend-flow.mmd) | [Phase 4 Frontend Architecture](../frontend/README.md) |
| Rust/Axum backend | Application; implemented; `VERIFIED` | `backend/src`; entry `backend/src/main.rs` | HTTP/WebSocket routing, identity, tenant enforcement, orchestration, persistence, integrations; exposes public/auth/tenant routes | All application data through PostgreSQL plus Redis/files/memory; environment config; depends on Axum, Tokio, SQLx, Redis and HTTP clients | Colocated `backend/src` tests; [Container View](diagrams/container-view.mmd), [Backend Request Flow](diagrams/backend-request-flow.mmd) | Phase 3 backend modules and Phase 6 API |
| Authentication and tenant pipeline | Module/middleware; implemented; `HIGH` | `backend/src/middleware/auth.rs`; `backend/src/middleware/tenant.rs`; router attachment in `routes/mod.rs` | Verify bearer claims, organization membership, rate and quota context; exposes `Claims` and `TenantContext` to handlers | Users, organizations, memberships, rate and quota state; JWT/config; depends on PostgreSQL, Redis and services | Colocated middleware/service tests where present; [Backend Request Flow](diagrams/backend-request-flow.mmd) | Phase 7 security and Phase 8 tenancy |
| PostgreSQL persistence | Infrastructure service; configured and required; `VERIFIED` | `backend/migrations`; initialized in `backend/src/main.rs` | Durable records, transactions, constraints, and RLS; exposes SQL through SQLx pool/connections | Application relational data; `DATABASE_URL`; depends on PostgreSQL runtime and migrations | Migration execution plus colocated query tests; [Container View](diagrams/container-view.mmd) | Phase 5 database |
| Redis support | Infrastructure service/adapter; configured and required by selected flows; `VERIFIED` | `backend/src/services/cache.rs`, `rate_limit.rs`, health/readiness composition | Delivery cache, rate-limit state, readiness check; exposes Redis operations to backend code | Cache and limiter keys; `REDIS_URL`; depends on Redis client/server | Colocated service tests where present; [Container View](diagrams/container-view.mmd) | Phase 10 operations |
| Upload storage and media processing | Adapter/module; implemented; `HIGH` | `backend/src/routes/media.rs`; `backend/src/services/media_processing.rs` | Store originals/variants and serve uploaded files; exposes media HTTP and static-file behavior | Media metadata in PostgreSQL and bytes under `UPLOAD_DIR`; upload limits; depends on filesystem and image library | Colocated route/service tests where present; [System Context](diagrams/system-context.mmd) | Phase 5 data and Phase 7 upload security |
| Built-in plugin host | Plugin host; implemented; `VERIFIED` | `backend/src/plugins/mod.rs`; invoked by content routes | Register enabled trusted plugins and execute content hooks; exposes `CmsPlugin` contract | Plugin configuration and content hook context; depends on backend process and content flows | Colocated plugin tests where present; [Container View](diagrams/container-view.mmd) | Phase 9 plugins and extensibility |
| Marketplace policy and host adapters | Service/adapter group; implemented without arbitrary package execution; `HIGH` | `backend/src/services/marketplace_runtime.rs`; `backend/src/services/marketplace_adapters.rs`; Marketplace routes | Validate installation, entitlement, runtime policy and allowlisted host capabilities; exposes Marketplace HTTP results and adapter calls | Marketplace records and package artifacts; runtime policy/config; depends on PostgreSQL, files, Stripe, backend capabilities | Marketplace backend tests and frontend Marketplace page test; [System Context](diagrams/system-context.mmd) | Phase 9 Marketplace and extensibility |
| External integration adapters | Adapter group; configuration-dependent; `HIGH` | Stripe/email services and webhook routes | Stripe HTTPS/callback, email webhook, and signed CMS webhook delivery | Billing/email/webhook rows and secrets/config names; depends on network/provider targets | Colocated route/service tests where present; [System Context](diagrams/system-context.mmd) | Phase 6 contracts and Phase 10 operations |

Runtime status describes repository implementation, not actual production availability. A `HIGH` confidence boundary is well supported but still has ownership or deployment uncertainty; `VERIFIED` means the component and role are directly established by multiple primary sources.

## Backend Bootstrap and Shared State

| Source | Responsibility |
|---|---|
| `backend/src/main.rs` | Process bootstrap, tracing, configuration, connection clients, migrations, initial administrator seed, router, global middleware, listener, shutdown |
| `backend/src/config.rs` | Environment-driven application configuration |
| `backend/src/state.rs` | Shared `AppState`: configuration, SQL pool, Redis client, and in-memory page-preview broadcast channels |
| `backend/src/db/mod.rs` | Database connection support and embedded migration access |
| `backend/src/error.rs` | Stable application error to HTTP status and JSON mapping |

The shared state object is dependency injection at process scope. It does not isolate domains: all handlers receiving it can reach the database, Redis, configuration, and preview state.

## Backend Routing Components

`backend/src/routes/mod.rs` is the HTTP composition root. It groups route modules by public, authenticated, or tenant-protected access. Major route responsibilities include:

| Route area | Responsibility |
|---|---|
| Authentication | Registration, login, token refresh, current-user operations, and protected identity actions |
| Organizations and beta | Organization lifecycle, membership-related operations, beta-access operations |
| Content and comments | Content types, entries, state transitions, and comment behavior |
| Pages | Page definitions, versions, visual-builder data, preview, and publication behavior |
| Media | Upload validation, image processing, file persistence, and media metadata |
| Billing | Plan, subscription, usage, Stripe checkout, and Stripe webhook behavior |
| Webhooks | CMS webhook registration, delivery dispatch, and delivery records |
| Plugins | Built-in plugin registry and plugin-facing operations |
| Marketplace | Catalog, creator, review, package, installation, entitlement, runtime-host, finance, feedback, analytics, and operational surfaces |
| Delivery | Public content delivery with Redis cache and PostgreSQL fallback |
| Health and readiness | Process liveness and PostgreSQL/Redis dependency checks |

The route directory is not a pure transport layer. It also contains SQL, ownership checks, file work, integration calls, and orchestration.

## Middleware Components

| Component | Responsibility |
|---|---|
| Authentication middleware | Extract and verify bearer access tokens; attach claims |
| Tenant middleware | Resolve organization, verify membership, enforce rate and quota checks, attach claims and tenant context |
| Security middleware | Add security-related response headers and request protections |
| Global Tower layers | Timeout, compression, CORS, request identifiers, and tracing |

Global layers apply around the assembled router. Exact call-order claims should be verified against Tower layer composition before using order as a security assumption.

## Service Components

The service directory holds reusable operations such as JWT and password support, RBAC, RLS, quota and rate limiting, health checks, caching, email, Stripe access, media processing, Marketplace policy/runtime/adapters, and related domain helpers. Services are reusable source modules inside the same backend process; they are not network services.

Service boundaries are uneven. Some services are infrastructure adapters, some express domain policy, and some depend on route- or middleware-owned types. Phase 3 verifies these responsibilities in the [Backend Module Catalog](../backend/module-catalog.md), [Services and Domain Logic](../backend/services-and-domain.md), and [Shared Infrastructure](../backend/shared-infrastructure.md) without asserting a clean application/domain/infrastructure layering model.

## Plugin Components

`backend/src/plugins/mod.rs` defines the built-in plugin contract and registry. The SEO plugin is an observed in-process implementation. Content flows invoke plugin hooks before save and after publish. Built-in plugin failure behavior therefore shares the backend request or spawned-task lifecycle.

The Marketplace runtime and adapters are separate service modules that validate requested host capabilities. They do not constitute an uploaded-code execution engine.

## Frontend Components

| Source area | Responsibility |
|---|---|
| `frontend/src/main.tsx` | React root, providers, global styles, and router startup |
| `frontend/src/router.tsx` | Route tree for public, authenticated, and organization-scoped pages |
| `frontend/src/components` | Shared shell, authentication guard, generated form, and status presentation |
| `frontend/src/pages` | Feature-oriented screens for CMS, SaaS, and Marketplace capabilities |
| `frontend/src/stores/useAppStore.ts` | Shared user, token, organization, and application state |
| `frontend/src/services/api.ts` | Central HTTP request construction, authorization and organization headers, response parsing, and endpoint methods |
| `frontend/src/types/api.ts` | Manually maintained TypeScript representation of backend contracts |
| `frontend/src/i18n` | Locale messages, direction, and localization context |

Feature pages depend on shared UI, the store, types, i18n, and the central API client. No separate frontend domain package or generated API SDK is present.

## Phase 4 Frontend Component Detail

| Component boundary | Primary paths | Verified responsibility | Status and deeper map |
|---|---|---|---|
| Management SPA | `frontend/`; `main.tsx` | One browser application, provider startup, bundle and static-host boundary | `EXPLICIT`; [Application Catalog](../frontend/application-catalog.md) |
| Routing and protected layout | `router.tsx`; `RequireAuth.tsx`; `AppShell.tsx` | Public login, token-presence admission, protected navigation/layout, eager page composition | `EXPLICIT` route tree; [Routing](../frontend/routing.md) |
| Route pages | `frontend/src/pages` | Dominant feature UI, API orchestration, server data, drafts, loading, and errors | `OBSERVED` to `OVERLAPPING`; [Feature Catalog](../frontend/feature-catalog.md) |
| Shared components and hook | `frontend/src/components`; `hooks/useHealth.ts` | Shell, guard, DynamicForm, StatusBadge, and health polling | `OBSERVED`; [Component Architecture](../frontend/component-architecture.md) |
| State and persistence | `useAppStore.ts`; `I18nProvider.tsx`; API setters | Session/organization/shell, locale/direction, and transport context | Mixed ownership; [State Management](../frontend/state-management.md) |
| Backend integration and contracts | `services/api.ts`; `types/api.ts` | Central fetch/multipart/error behavior and manual browser types | `OBSERVED`; contract authority unclear under ACU-01/DC-01 |
| Page Builder | `PagesPage.tsx`; dnd-kit | Palette, sortable canvas, property editor, local preview, save/versions/workflow/template/preview handoff | `OVERLAPPING`; [Page Builder](../frontend/page-builder.md) |
| Styling and localization | `styles/index.css`; `i18n`; font asset | Global semantic classes, responsive/RTL rules, locale selection and messages | Informal style system; [Styling](../frontend/styling-and-design-system.md) |

Phase 4 found no separate public frontend application, design-system package, frontend feature packages, generated client, route-level lazy modules, Storybook, or Error Boundary. Absence from current source is not a statement about unobserved external systems or future intent.

## Phase 5 Database Component Detail

PostgreSQL is one shared-database, shared-namespace persistence component. SQLx supplies a lazy pool capped at 10 connections, direct query mapping, explicit transactions, and an embedded startup migrator. The intended migration-defined schema has 51 application tables grouped into 18 domain entities. Tenant enforcement combines request membership, explicit predicates, PostgreSQL context, and forced RLS on 32 tables. See the [Database Overview](../database/overview.md), [Schema Catalog](../database/schema-catalog.md), and [Database Domain Map](../database/diagrams/database-domain-map.mmd).

Persistence ownership is not a separate application component: routes and services access SQLx directly (`PBU-01`). Filesystem media, provider calls, Redis invalidation, process-local preview, and spawned webhook work cross PostgreSQL transaction boundaries.

## Missing Runtime Components

The following are not verified as current components: durable worker, queue or broker, search cluster, object store, CDN adapter, external identity provider, API gateway, metrics exporter, APM collector, deployment controller, or data warehouse. Their absence from the repository must not be turned into a statement about unobserved production infrastructure.

For the deeper backend-only component view, see the [Backend Overview](../backend/overview.md), [18-module catalog](../backend/module-catalog.md), [Backend Module Map](../backend/diagrams/backend-module-map.mmd), and [Application State Composition](../backend/diagrams/application-state-composition.mmd).

## Extensibility Components

The backend contains a compiled plugin registry, global plugin metadata, Marketplace host services, runtime policy, and adapter routes. Page Builder consumes system, tenant, and Marketplace-derived component metadata. See the [Extensibility Context](../extensibility/diagrams/extensibility-context.mmd).
