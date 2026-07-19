---
okf_document_id: "architecture-runtime-flows"
title: "Runtime Flows"
project: "ZinharCMS"
category: "architecture"
phase: 2
status: "current"
review_status: "verified"
source_of_truth: false
architecture_status: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/main.rs"
  - "backend/src/config.rs"
  - "backend/src/error.rs"
  - "backend/src/routes/mod.rs"
  - "backend/src/routes/auth.rs"
  - "backend/src/middleware/auth.rs"
  - "backend/src/middleware/tenant.rs"
  - "backend/src/routes/delivery.rs"
  - "backend/src/routes/pages.rs"
  - "backend/src/routes/webhooks.rs"
  - "backend/src/services/webhooks.rs"
  - "backend/src/services/rls.rs"
  - "frontend/src/main.tsx"
  - "frontend/src/router.tsx"
  - "frontend/src/services/api.ts"
  - "frontend/src/stores/useAppStore.ts"
  - "frontend/src/components/AppShell.tsx"
  - "frontend/src/pages/PagesPage.tsx"
  - "frontend/src/i18n/I18nProvider.tsx"
related_documents:
  - "architecture/README.md"
  - "architecture/overview.md"
  - "architecture/boundaries.md"
  - "architecture/components.md"
  - "architecture/dependency-model.md"
  - "architecture/integration-points.md"
  - "architecture/architecture-risks.md"
  - "frontend/routing.md"
  - "frontend/state-management.md"
  - "frontend/api-client.md"
  - "frontend/page-builder.md"
related_diagrams:
  - "architecture/diagrams/backend-request-flow.mmd"
  - "architecture/diagrams/frontend-backend-flow.mmd"
  - "architecture/diagrams/system-context.mmd"
  - "frontend/diagrams/frontend-routing-flow.mmd"
  - "frontend/diagrams/frontend-state-flow.mmd"
  - "frontend/diagrams/frontend-api-flow.mmd"
  - "frontend/diagrams/page-builder-flow.mmd"
uncertainty_markers:
  - "UNKNOWN U-08"
  - "NEEDS_OWNER_CONFIRMATION NOC-01"
  - "NEEDS_OWNER_CONFIRMATION NOC-09"
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR ABU-03"
  - "IMPLEMENTATION_STATUS_UNCLEAR ISU-01"
  - "STATE_OWNERSHIP_UNCLEAR SOU-01"
  - "AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01"
  - "API_CONTRACT_UNCLEAR ACU-01"
  - "ROUTING_BEHAVIOR_UNCLEAR RBU-01"
---

# Runtime Flows

## Phase 6 API Request Flow

The verified transport flow is documented in [API Route Architecture](../api/route-architecture.md) and [API Request Lifecycle](../api/diagrams/api-request-lifecycle.mmd). Requests pass global timeout/security/CORS/compression/request-ID/tracing layers, enter one of the public, authenticated, or tenant-protected router zones, run extractor and domain validation, apply role/ownership/workflow rules, use tenant RLS or an explicit global path, perform side effects, and return a typed response or error.

The handler inventory is 17 public, 12 authenticated non-tenant, and 139 tenant-protected endpoints. The page preview flow upgrades `GET /api/preview/{page_id}` to WebSocket after preview-specific token and organization selection.

## Flow Register

| Flow | Trigger and entry point | Components and data | Security checkpoints and failure paths | Evidence | Confidence and diagram |
|---|---|---|---|---|---|
| Backend startup | Process start at `backend/src/main.rs` | Config, tracing, SQLx pool, migrations, seed, Redis client, `AppState`, router, listener | Invalid config, migration, pool use, or bind failure stops startup; known default-admin seed is AR-012 | `backend/src/main.rs`; `config.rs`; `state.rs` | High; [Container View](diagrams/container-view.mmd) |
| Frontend startup | Browser loads bundle at `frontend/src/main.tsx` | React root, i18n/provider context, router, pages, store | Route guards redirect based on client state; startup/render errors remain browser failures | `frontend/src/main.tsx`; `router.tsx` | High; [Frontend-Backend Flow](diagrams/frontend-backend-flow.mmd) |
| Incoming backend request | HTTP request enters global layers and `routes/mod.rs` | Request metadata, route parameters/body, claims/context, response or `AppError` | Timeout/security/CORS layers plus route-group middleware; invalid input/dependency errors map to HTTP | `backend/src/main.rs`; `routes/mod.rs`; `error.rs` | High; [Backend Request Flow](diagrams/backend-request-flow.mmd) |
| Authentication request | Login or protected bearer request through auth routes/middleware | Credentials or JWT; user and claims data | Password/JWT verification; invalid credentials/token rejected | `backend/src/routes/auth.rs`; `middleware/auth.rs`; `services/jwt.rs`; `services/password.rs` | High; [Backend Request Flow](diagrams/backend-request-flow.mmd) |
| Tenant request | Protected request with token and organization header | Claims, organization ID, membership, rate/quota state, `TenantContext` | Membership, rate, and quota checks; Redis/DB failure can reject request | `backend/src/middleware/tenant.rs`; `backend/src/services/quota.rs`; `backend/src/services/rate_limit.rs`; `backend/src/services/rls.rs` | High; [Backend Request Flow](diagrams/backend-request-flow.mmd) |
| Frontend-to-backend request | Page calls `frontend/src/services/api.ts` | Typed request data, tokens, organization header, JSON/multipart response | Client guards are advisory; backend enforces auth/tenant; error JSON becomes client error; no automatic refresh retry verified | `frontend/src/services/api.ts`; `frontend/src/stores/useAppStore.ts`; `frontend/src/types/api.ts`; `backend/src/routes/mod.rs` | High; [Frontend-Backend Flow](diagrams/frontend-backend-flow.mmd) |
| Frontend navigation | Browser router matches login or protected child | Stored token, AppShell, eager page, optional workspace context | Missing token redirects to login; unmatched protected path redirects home; no route error boundary | `frontend/src/router.tsx`; `components/RequireAuth.tsx`; `components/AppShell.tsx` | High; [Frontend Routing Flow](../frontend/diagrams/frontend-routing-flow.mmd) |
| Frontend organization switch | Shell or workspace redirect selects membership | Zustand, API module organization ID, browser storage, remounted route page | Unknown ID is ignored; page refetch depends on remount/effects; backend validates header membership | `useAppStore.ts`; `AppShell.tsx`; `WorkspaceRedirectPage.tsx` | High; [Frontend State Flow](../frontend/diagrams/frontend-state-flow.mmd) |
| Frontend Page Builder | User changes palette/canvas/props or page actions | Local page JSON draft, dnd-kit, component registry, page/Marketplace APIs, versions | Save/template/workflow errors are page-local; preview URL carries session query context | `frontend/src/pages/PagesPage.tsx`; page/API types | High; [Page Builder Flow](../frontend/diagrams/page-builder-flow.mmd) |
| Database access | Handler/service issues SQLx call or requests RLS-scoped connection/transaction | SQL parameters, organization/user session context, rows and transaction results | Ownership checks and RLS context where used; SQL/connection failure maps through `AppError` | Route/service queries; `backend/src/services/rls.rs`; migrations | High for mechanisms, medium for complete coverage; [Backend Request Flow](diagrams/backend-request-flow.mmd) |
| Error propagation | Handler, service, database, cache, file, or provider operation returns error | `AppError`, status, stable JSON error/message | Error mapping prevents raw internal response by default; exact source error handling varies | `backend/src/error.rs`; route/service call sites | High; [Backend Request Flow](diagrams/backend-request-flow.mmd) |
| Configuration loading | Backend process constructs `Config` from environment | Server, DB, Redis, JWT, CORS, upload, email, Stripe, and related settings | Missing/invalid required values fail configuration or later integration use; secrets are not documented here | `backend/src/config.rs`; environment templates | High for repository capability, unknown for production values; [Container View](diagrams/container-view.mmd) |
| Background and realtime work | Mutation schedules webhook or preview connection uses broadcast channel | Signed webhook payload/delivery record or preview message/channel | Webhook target validation/signature; preview authorization; process loss interrupts non-durable work | `backend/src/services/webhooks.rs`; `backend/src/routes/pages.rs`; `backend/src/state.rs` | High; [System Context](diagrams/system-context.mmd) |
| Built-in plugin execution | Selected content mutation resolves enabled plugin hooks | Content hook context and validation/transformation results | Tenant request controls precede hooks; plugin failure shares handler lifecycle | `backend/src/plugins`; `backend/src/routes/content.rs` | High; [Container View](diagrams/container-view.mmd) |
| Public content delivery | Public delivery request enters `routes/delivery.rs` | Fixed `default` organization, Redis cache key/value, PostgreSQL content result | No authenticated user; organization mapping is unresolved; cache failure falls back to DB | `backend/src/routes/delivery.rs`; cache and RLS services | High for current flow, low for intended domain routing; [System Context](diagrams/system-context.mmd) |

No general server-side page-template rendering pipeline was verified. The identifiable public content flow is delivery of stored content data; browser rendering behavior belongs to the frontend and later Phase 4 documentation.

## Backend Startup

1. The process loads `.env` when available and initializes the tracing subscriber.
2. `Config` reads environment-controlled server, database, Redis, authentication, CORS, upload, email, Stripe, and related options.
3. The process creates a lazy PostgreSQL pool and a Redis client.
4. Embedded SQL migrations run against PostgreSQL.
5. If the users table is empty, startup creates the known default administrator account and associates it with a default organization.
6. The process creates `AppState`, including process-local preview channels.
7. The route tree and global Tower layer stack are assembled.
8. The server binds to the configured address and waits for graceful shutdown signals.

Migration or bootstrap failure prevents successful startup. The default-account behavior is recorded as a high architecture and security risk in [Architecture Risks](architecture-risks.md).

## Public Request Flow

1. A client sends a request to a public route.
2. Global timeout, security, compression, CORS, request-ID, and tracing layers wrap handling.
3. The public router selects health, readiness, authentication, Stripe webhook, public delivery, or file-serving behavior.
4. The handler validates its route-specific input and calls PostgreSQL, Redis, files, or an integration as required.
5. Success is serialized to the route response; `AppError` maps failures to a status and stable JSON shape.

Public does not mean unvalidated. Stripe callbacks require a valid provider signature, and login still verifies credentials.

## Authenticated Request Flow

1. The central frontend API client adds a bearer token when one is available.
2. Authentication middleware extracts and verifies the JWT.
3. Verified claims are attached to the request.
4. A protected handler performs its own role or ownership checks and executes its operation.
5. The result or mapped application error returns through the global layer stack.

Client-side `RequireAuth` routing is not part of backend authorization. Direct callers must still pass backend middleware and handler checks.

## Tenant-Protected Request Flow

1. The client supplies a bearer token and `X-Organization-Id`.
2. Tenant middleware uses existing claims or verifies the token if claims are absent.
3. It parses the organization identifier and loads the user's membership from PostgreSQL.
4. It invokes Redis-backed rate limiting and database-backed quota enforcement, except where route behavior explicitly exempts billing.
5. It attaches claims and `TenantContext` to the request.
6. The handler performs feature validation and may issue direct SQL, obtain an RLS-scoped connection or transaction, use Redis, write files, invoke plugins, or call an external provider.
7. `AppError` maps failures and the global layers complete the response.

The presence of tenant middleware does not prove every downstream query uses the same RLS path. Detailed query-level isolation verification remains later work.

## Frontend Login and Workspace Flow

1. The router renders a public authentication page.
2. The page calls the central API client.
3. The backend authenticates the user and returns token and user data.
4. The API client and Zustand store persist session and organization-related state; tokens are also stored in `localStorage`.
5. Route guards and workspace redirection select an authenticated or organization-scoped page.
6. Subsequent API calls attach bearer and organization headers.

The central client parses non-success responses into frontend errors. No verified automatic refresh-and-retry interceptor was found; refresh is an explicit API operation.

## Frontend Startup and Navigation Flow

1. The browser loads `index.html` and the Vite entry bundle.
2. `main.tsx` mounts a strict React root, then `I18nProvider`, then the router.
3. The i18n provider resolves stored/browser/default locale and later synchronizes document `lang`, `dir`, and locale storage.
4. The router renders `/login` directly or passes any other route through `RequireAuth`.
5. The guard renders `AppShell` only when the Zustand access-token string is present; it does not verify or refresh the token.
6. The shell renders static navigation, organization and locale selectors, readiness, identity, logout, and the matched eager page.
7. Page effects load route-owned server data.
8. An unmatched protected child redirects to `/`; there is no dedicated route error/not-found component.

The production-like Nginx configuration provides history fallback, but equivalent behavior in an actual host remains `ROUTING_BEHAVIOR_UNCLEAR RBU-01`.

## Frontend Organization Transition Flow

1. Login/session restoration supplies membership summaries and a selected organization.
2. The shell selector changes organization by ID, or `/workspace/:slug` resolves a membership and redirects to `/`.
3. The Zustand action validates the ID against memberships.
4. The action updates the API module's active organization, persistent storage, and reactive state.
5. `AppShell` changes the key on its main route area, remounting the page subtree.
6. Page effects reload data; authenticated client methods add the new organization header.
7. Backend tenant middleware remains authoritative for membership and scope.

State is distributed across Zustand, API module variables, and browser storage under `SOU-01`. No cross-tab storage synchronization or shared tenant-keyed server cache was found.

## Frontend API Request Flow

1. A route page or shared hook calls a method on the central API object.
2. The request helper serializes JSON or passes `FormData`, opts into browser credentials, and conditionally attaches bearer and organization headers.
3. The backend handles the request under its public, authenticated, or tenant route stack.
4. A successful body is parsed as JSON and cast to the handwritten TypeScript type.
5. A non-success body is reduced to status plus message and thrown as `ApiError`.
6. The caller normally places the result, loading state, or error message in route-local state.

No automatic refresh/replay, abort, timeout, retry, query cache, or runtime response validation was found. Manual client/server contract duplication remains `ACU-01`/`DC-01`/`DDU-03`.

## Frontend Page Builder Flow

1. `/pages` loads pages, system components, Marketplace components, and Marketplace installations.
2. System and Marketplace definitions form the palette; active design templates form the template list.
3. The user creates or selects a page draft, then adds, selects, edits, removes, or reorders component nodes.
4. Every mutation normalizes page JSON, marks the draft dirty, and updates a local React preview.
5. New pages require manual first save; existing dirty pages schedule save after ten seconds.
6. Page actions can transition workflow, load/restore versions, preview/import templates, or copy a backend preview WebSocket URL.
7. The in-page local preview does not open that WebSocket and does not prove public-renderer parity.

See [Frontend Page Builder](../frontend/page-builder.md) for implemented and unverified capabilities.

## Public Delivery Flow

1. A public consumer requests delivered content.
2. The handler resolves the active organization by the fixed slug `default`.
3. It attempts to read a JSON value from Redis.
4. On cache miss or cache failure, it obtains the organization context and reads PostgreSQL through tenant-aware logic.
5. It returns the serialized result and attempts cache population or later invalidation on relevant mutations.

The domain-to-organization routing lifecycle is not implemented or established by current evidence. This flow is limited by `U-08`, `NOC-01`, and `ISU-01`.

## Media Upload Flow

1. An authenticated tenant request sends multipart content to a media endpoint, subject to the tenant route body limit.
2. The handler validates ownership, upload metadata, file type, and size constraints.
3. It creates organization-scoped directories below `UPLOAD_DIR` and writes the original.
4. Image work runs through blocking-task isolation and creates WebP variants where applicable.
5. Media metadata is written to PostgreSQL and returned to the caller.

Filesystem and PostgreSQL changes do not share one atomic commit. Failure and cleanup behavior must be reviewed when changing this flow.

## Plugin Hook Flow

1. A content mutation reaches a tenant-protected content handler.
2. Enabled built-in plugins are resolved inside the backend process.
3. Before-save hooks can validate or transform the pending operation.
4. The content mutation is persisted.
5. After-publish hooks run for applicable publication events.

The exact transaction and failure boundary must be verified in the changed handler before relying on a hook as guaranteed delivery.

## CMS Webhook Delivery Flow

1. A qualifying mutation identifies active webhook subscriptions.
2. The webhook service spawns an in-process asynchronous task.
3. The task signs the payload and sends it to an HTTP or HTTPS target with a timeout.
4. One delivered or failed attempt is recorded in PostgreSQL.

No durable queue, independent worker, or automatic retry scheduler was found. Process termination can interrupt an accepted but unfinished delivery. The required guarantee remains `NOC-09`.

## Page Preview Flow

1. `PagesPage` can construct and copy a WebSocket URL containing preview authorization and organization parameters; an external preview client opens it.
2. The backend authorizes access to the page and locates or creates a broadcast channel in `AppState`.
3. Editor changes publish messages to the process-local channel.
4. Connected preview clients receive broadcast messages.

Channels are neither persisted nor shared through Redis. A connection to a different backend instance cannot be assumed to observe the same channel.

## Shutdown Flow

The listener responds to supported process signals and performs graceful server shutdown. Repository evidence does not establish coordinated draining of spawned webhook tasks, preview channels, or a multi-instance deployment.

## Phase 5 Database Runtime Flows

Startup parses `DATABASE_URL`, creates a lazy SQLx pool, applies embedded migrations, and runs the conditional administrator bootstrap before serving. A migration or bootstrap failure stops startup. No database retry loop or separate migration job was found.

Tenant transactions set organization/user/bypass context locally before queries; forced RLS evaluates that context. Privileged catalog/provider/admin operations use bypass transactions. Page writes persist page/version state before post-commit broadcast and invalidation. Paid checkout, media storage, organization audit, and webhook dispatch cross database boundaries as documented in [Transactions and Consistency](../database/transactions-and-consistency.md).

## Related Flow Diagrams

- [Backend Request Flow](diagrams/backend-request-flow.mmd)
- [Frontend-Backend Flow](diagrams/frontend-backend-flow.mmd)
- [System Context](diagrams/system-context.mmd)
- [Frontend Routing Flow](../frontend/diagrams/frontend-routing-flow.mmd)
- [Frontend State Flow](../frontend/diagrams/frontend-state-flow.mmd)
- [Frontend API Flow](../frontend/diagrams/frontend-api-flow.mmd)
- [Page Builder Flow](../frontend/diagrams/page-builder-flow.mmd)
- [Tenant Isolation Flow](../database/diagrams/tenant-isolation.mmd)
- [Migration Lifecycle](../database/diagrams/migration-lifecycle.mmd)

## Security Runtime Flows

Phase 7 adds the verified [Authentication Flow](../security/diagrams/authentication-flow.mmd), [Session and Token Lifecycle](../security/diagrams/session-token-lifecycle.mmd), [Authorization Decision Flow](../security/diagrams/authorization-decision-flow.mmd), and [Tenant Access Control](../security/diagrams/tenant-access-control.mmd). These diagrams separate global claims, organization membership, resource checks, and RLS rather than collapsing them into one permission decision.

## Phase 8 Domain Runtime Flows

[Cross-Module Workflows](../domain/cross-module-workflows.md) connects request, transaction, filesystem, cache, broadcast, email, provider, webhook, and Marketplace effects. The [Cross-Module Orchestration](../domain/diagrams/cross-module-orchestration.mmd) diagram distinguishes committed database changes from best-effort or spawned side effects. Detailed sequences are maintained in `domain/workflows/`; they do not imply a durable workflow engine, queue, or outbox.
