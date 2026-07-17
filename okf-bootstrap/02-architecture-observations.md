# Architecture Observations

## Observed Architecture

ZinharCMS is an observed modular monolith:

- one Rust/Axum backend process;
- one React single-page admin application;
- one PostgreSQL database;
- one Redis instance;
- local filesystem storage for CMS media and Marketplace artifacts;
- optional external Stripe, email-webhook, and customer-webhook integrations.

No separately deployed API gateway, worker, search service, plugin executor, or event broker was found.

## Runtime Containers

| Runtime unit | Responsibility | Evidence |
| --- | --- | --- |
| Browser SPA | Admin UI, local session/organization selection, API calls | frontend/src |
| Axum API | Routing, authorization, business rules, SQL, integrations, static uploads | backend/src |
| PostgreSQL | Durable records, constraints, indexes, RLS | backend/migrations |
| Redis | Delivery cache and rate limits | services/cache.rs, services/rate_limit.rs |
| Filesystem volume | Media bytes and Marketplace package artifacts | UPLOAD_DIR, routes/media.rs, Marketplace package routes |
| Nginx | Production-like static SPA server | frontend/nginx.conf |

## Backend Layers and Dependency Direction

| Layer | Main paths | Observed dependency direction |
| --- | --- | --- |
| Bootstrap/infrastructure | main.rs, config.rs, state.rs, db | Constructs state, runs migrations, composes middleware and routes |
| HTTP boundary | routes/*.rs | Depends on middleware context, services, SQLx, and AppState |
| Middleware | middleware/*.rs | Depends on JWT, quota, rate limit, database, and AppState |
| Domain/application policy | services/*.rs | Called by routes; some services also perform SQL and outbound HTTP |
| Data mapping | models/*.rs plus route DTOs | Used selectively; most DTOs live beside handlers |
| Persistence | migrations plus embedded SQL | Called directly from routes/services through SQLx |
| Extensibility | plugins/*.rs | Called from content/page lifecycle handlers |

The dependency direction is not a strict clean architecture. Route modules contain DTOs, authorization, SQL, transactions, and orchestration. Services contain reusable policy and integration logic. There is no repository layer separating SQL from handlers.

## Route Boundaries

The root router composes three meaningful security stacks:

1. Public routes: root metadata, health, readiness, auth register/login/refresh, Stripe webhook, public delivery API, static uploads.
2. Authenticated routes: logout/me, organization listing/creation/invitation acceptance, plugin registry, product-level beta administration.
3. Tenant-protected routes: CMS, pages, media, comments, billing, Marketplace, tenant organization administration, webhooks, and beta feedback/blockers.

Tenant middleware performs:

1. bearer-token verification;
2. organization selection from X-Organization-Id or preview query input;
3. active membership lookup;
4. organization/user rate limiting;
5. plan API quota accounting except explicit quota-exempt paths;
6. TenantContext injection.

## Request Lifecycle

1. Tower layers add timeout, security headers, compression, CORS, request IDs, and HTTP tracing.
2. The router matches a public, authenticated, or tenant-protected route.
3. Authentication middleware verifies an HMAC-SHA256 bearer token when required.
4. Tenant middleware loads active membership and applies rate/quota gates when required.
5. Handler-level RBAC applies global or organization-role rules.
6. Tenant database helpers set PostgreSQL session/transaction context for forced RLS.
7. Handlers execute SQL, call services/plugins/integrations, and return JSON or WebSocket responses.
8. AppError maps failures to a stable JSON error envelope.

## Primary Data Flows

### Admin CMS Flow

Browser -> API client with bearer token and organization header -> tenant middleware -> RBAC -> tenant/RLS SQL -> JSON response -> Zustand/local page state.

### Public Delivery Flow

Unauthenticated request -> hard-coded active default organization lookup -> organization-scoped RLS connection -> Redis cache lookup/fill -> published JSON, sitemap, or robots response.

### Media Flow

Tenant-authenticated multipart upload -> MIME and size validation -> local file write -> image variants -> PostgreSQL media rows -> public static /uploads URL.

Filesystem and database work are not one atomic transaction.

### Marketplace Flow

Creator/listing metadata -> package upload to local storage -> manifest/static/security/compatibility validation -> persisted version/submission reports -> global-admin review -> catalog visibility -> tenant install/permission approval -> optional purchase/entitlement -> runtime policy/host adapters.

Uploaded package code is never executed.

### Billing Flow

Tenant checkout request -> Stripe HTTP API -> signed public webhook -> bypass transaction -> subscription or Marketplace finance tables -> audit/email side effects.

## Frontend Architecture

- React Router defines one public login route and authenticated admin child routes.
- RequireAuth gates the protected route tree.
- Zustand persists the access token, optional refresh token, user, memberships, and active organization in localStorage.
- The API helper adds Authorization and X-Organization-Id and includes cookies.
- Page components own most feature state and orchestration.
- frontend/src/types/api.ts duplicates backend DTO contracts manually.
- PagesPage.tsx contains the page builder and Marketplace template/component integration in one large route component.
- MarketplacePage.tsx contains several Marketplace subdomains in one large route component.
- The first-party i18n layer supports English and Persian locale metadata and RTL layout behavior.

## Plugin and Extension Architecture

Two distinct extension concepts exist:

- Built-in CMS plugins implement the in-process CmsPlugin trait; the SEO plugin is registered at compile time.
- Marketplace products are metadata/artifact records with host-owned adapter contracts. Component packs, template imports, and allowlisted hooks operate through host code. External package execution is deferred.

These concepts must remain separate in OKF.

## Background Work and Events

- CMS webhooks can be dispatched in transient Tokio tasks and record delivery outcomes.
- Stripe and email events are handled synchronously at API/service boundaries.
- Page preview uses in-process broadcast channels.
- No durable queue, retry scheduler, event bus, or worker process was found.

## Architecture Strengths

- Clear public/authenticated/tenant router composition.
- Defense-in-depth tenant isolation through membership checks, organization predicates, and forced RLS.
- Strong migration-level constraints and indexes.
- Explicit Marketplace non-execution boundary.
- Evidence-rich documentation and diagram traceability.

## Architecture Risks and Ambiguities

- Route modules are large and combine multiple responsibilities.
- Local file storage is publicly served by path and is not protected by database RLS.
- No durable worker exists for retryable integration work.
- Production ingress, TLS termination, backup automation, and deployment target are UNKNOWN.
- The public delivery API always resolves the default organization; the intended production multi-site routing rule is NEEDS_OWNER_CONFIRMATION.
- Startup runs migrations in every backend process; multi-replica rollout coordination is not documented.
- Some side effects happen after primary mutations, with no documented compensation policy.

