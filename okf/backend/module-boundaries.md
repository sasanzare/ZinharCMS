---
okf_document_id: "backend-module-boundaries"
title: "Backend Module Boundaries"
project: "ZinharCMS"
category: "backend"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "mixed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
primary_sources:
  - "backend/src/routes/mod.rs"
  - "backend/src/services/mod.rs"
  - "backend/src/middleware/mod.rs"
  - "backend/src/models/mod.rs"
  - "backend/src/state.rs"
related_documents:
  - "backend/README.md"
  - "backend/module-catalog.md"
  - "backend/dependency-map.md"
  - "backend/services-and-domain.md"
  - "architecture/boundaries.md"
related_diagrams:
  - "backend/diagrams/backend-module-map.mmd"
  - "backend/diagrams/backend-dependency-flow.mmd"
uncertainty_markers:
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR"
  - "MODULE_BOUNDARY_UNCLEAR MBU-01"
  - "MODULE_OWNERSHIP_UNCLEAR MOU-01"
  - "RESPONSIBILITY_OVERLAP RO-01"
  - "PROPOSED_NOT_IMPLEMENTED"
---

# Backend Module Boundaries

## Boundary Model

The backend is one Rust crate and one deployable process. Its module boundaries are source-level responsibility boundaries, not independently deployed services. Axum route modules are the most visible entry boundaries; service modules provide reusable policy, validation, integration, and orchestration; middleware supplies request context and cross-cutting enforcement; models are partial persistence/transport structures; direct SQL remains distributed through routes and services.

Boundary labels in this document mean:

- `OBSERVED`: a distinct source area and responsibility are directly visible.
- `CROSS_CUTTING`: the capability intentionally participates in several request paths.
- `OVERLAPPING`: responsibility is implemented in more than one significant module.
- `ARCHITECTURAL_BOUNDARY_UNCLEAR`: evidence does not identify one authoritative owner.

## Enforcement Model

Rust module visibility and function signatures provide compile-time encapsulation, while router composition limits which middleware groups expose handlers. There is no separate crate boundary, dependency-rule linter, repository abstraction, or runtime service boundary enforcing the catalog below. Accordingly, these are observed documentation boundaries and not stronger guarantees than the code provides.

## Ownership Matrix

| Module | Primary ownership | Shared or external ownership | Boundary status | Principal marker |
|---|---|---|---|---|
| BE-MOD-001 Bootstrap and Runtime | Process startup, configuration load, migration run, state construction, global HTTP layers, shutdown | Default user and organization bootstrap touches Authentication and Organizations | `OVERLAPPING` | `BRU-01` |
| BE-MOD-002 Authentication | Registration, credential login, token lifecycle, bearer claims | Organization membership bootstrap and request claims | `OVERLAPPING` | `MOU-01` |
| BE-MOD-003 Tenant Authorization and RLS | Tenant context, RBAC, rate/quota checks, RLS helpers | Every tenant-protected domain supplies resource-specific authorization | `CROSS_CUTTING` | `MBU-01` |
| BE-MOD-004 Organizations and SaaS Operations | Organizations, memberships, invitations, organization lifecycle | Authentication bootstrap and Billing quotas | `OVERLAPPING` | `RO-02` |
| BE-MOD-005 Billing and Quotas | CMS subscription plans, checkout/portal, quota policy | Marketplace Finance also integrates with Stripe | `OVERLAPPING` | `RO-03` |
| BE-MOD-006 Content Types, Entries, and Workflow | Content schemas, entries, validation, editorial workflow | Delivery invalidation and Pages content references | `OVERLAPPING` | `MBU-02` |
| BE-MOD-007 Editorial Comments | Comment threads and moderation for editorial resources | Resource-specific authorization crosses content and pages | `OVERLAPPING` | `MOU-03` |
| BE-MOD-008 Media | Upload metadata, file handling, media processing | Filesystem lifecycle and public upload serving | `OVERLAPPING` | `MOU-04` |
| BE-MOD-009 Pages, Builder, and Preview | Page records, builder payloads, preview broadcasts | Delivery cache invalidation and Marketplace host adapters | `OVERLAPPING` | `RO-05` |
| BE-MOD-010 Public Delivery and Cache | Public content/page delivery and cache interaction | Content and Pages initiate invalidation | `CROSS_CUTTING` | `BRU-03` |
| BE-MOD-011 CMS Webhooks | CMS webhook registrations and delivery helpers | Domain modules determine event production; durable dispatch is unclear | `OVERLAPPING` | `MBU-05` |
| BE-MOD-012 Built-In Plugins | Built-in plugin registration and SEO behavior | Tenant routing and page/content data | `OBSERVED` | `BRU-04` |
| BE-MOD-013 Beta and Release Operations | Beta invitations, operational hardening, readiness checks | Bootstrap health endpoints and Marketplace release readiness | `OVERLAPPING` | `RO-06` |
| BE-MOD-014 Marketplace Creator, Submission, Validation, and Review | Creator package workflow through review | Catalog publication and runtime policy consume accepted artifacts | `OVERLAPPING` | `RO-07` |
| BE-MOD-015 Marketplace Catalog and Installation | Catalog discovery, installation lifecycle, entitlements at install time | Finance owns purchases; runtime owns execution | `OVERLAPPING` | `MOU-05` |
| BE-MOD-016 Marketplace Runtime Security and Host Adapters | Runtime policy, host adapter access, execution controls | Pages imports adapter behavior and catalog supplies artifacts | `OVERLAPPING` | `RO-09` |
| BE-MOD-017 Marketplace Finance | Purchases, entitlements, ledger, payout behavior | CMS Billing separately owns SaaS subscriptions | `OVERLAPPING` | `RO-03` |
| BE-MOD-018 Marketplace Feedback, Analytics, and Readiness | Reviews, abuse reports, analytics, performance and launch readiness | Catalog/creator data and operational release checks | `OVERLAPPING` | `RO-10` |

### Detailed Source Ownership

`Shared` means more than one module participates; `none found` means Phase 3 did not find a dedicated artifact and does not prove that no indirect behavior exists.

| Module | Routes and handlers | Services and domain logic | Models, queries, and migrations | Configuration | Tests | Events and background work |
|---|---|---|---|---|---|---|
| BE-MOD-001 | `main.rs`, `lib.rs`, root/health/readiness/OpenAPI composition | Startup orchestration; password/RBAC helpers are consumed | Startup SQL, migration runner; no dedicated model | Owns `Config` load and global HTTP settings | Config unit tests; startup flow has no direct test | Shutdown signal and tracing; no durable job registration |
| BE-MOD-002 | Auth public/protected routers and auth middleware | JWT, password, and security helpers | User model; auth/session/user SQL and related migrations | JWT expiry/secret, cookie, login-rate values | Selected security tests; no auth route test module | Token/session lifecycle; no background worker found |
| BE-MOD-003 | Tenant middleware; policy consumed by tenant handlers | RBAC, RLS, quota, rate-limit, security policy | Direct policy queries and RLS migrations; no dedicated aggregate model | Organization rate-limit values | Quota, rate, RBAC, and security unit tests | Request-scoped enforcement; no durable job |
| BE-MOD-004 | Organization protected and tenant routers | Route orchestration plus shared email/audit/policy helpers | Organization model; route SQL and organization migrations | Shared application/email link configuration where invitations use it | No organization test module found | Invitation/notification side effects are shared; no dedicated worker |
| BE-MOD-005 | Billing public and tenant routers, provider callback behavior | Stripe billing and quota helpers | Route/service SQL and billing migrations | Stripe credentials, prices, and return URLs | Billing, Stripe, and quota tests | Provider callbacks and subscription lifecycle; no general billing worker found |
| BE-MOD-006 | Content handlers | Entry validation, workflow, cache/webhook/plugin calls | Content model; content SQL and migrations | No dedicated environment area found | Workflow tests; route/entry integration gaps | Publication hooks, invalidation, and webhook side effects are shared |
| BE-MOD-007 | Comment handlers | Mostly route-local policy/orchestration | Comment SQL and migrations; no dedicated shared model | No dedicated configuration found | No comments test module found | No dedicated event or background-task owner found |
| BE-MOD-008 | Media handlers and static upload exposure shared with router | Media processing service | Media model, media SQL/migrations, upload files | Upload directory and maximum size | No media test module found | Processing uses operation-local work; no durable media worker found |
| BE-MOD-009 | Page, builder, and preview handlers | Cache and Marketplace adapter integration; route-local page rules | Page model, page SQL/migrations, in-memory channels | No page-specific environment values found | No pages test module found | Preview broadcast is process-local; publication side effects are shared |
| BE-MOD-010 | Public delivery handlers | Cache service and route-local delivery selection | Delivery SQL reads plus Redis cache; no dedicated migration owner | Redis connection is shared | Delivery route tests | Cache populate/invalidate actions; no durable worker |
| BE-MOD-011 | CMS webhook handlers | Webhook signing/delivery service | Webhook SQL and migrations | Target configuration is persisted; no dedicated global target variable | Webhook service tests | Outbound delivery uses feature-specific spawned work without a durable queue |
| BE-MOD-012 | Plugin router plus hooks invoked by domain routes | `CmsPlugin` contract, registry, and SEO implementation | Indirect content/page access; no plugin-owned migration identified | Plugin/domain configuration is feature-owned, not a distinct env block | SEO plugin tests | Before-save/after-publish hooks; lifecycle shares the web process |
| BE-MOD-013 | Beta protected and tenant handlers plus root readiness consumers | Hardening, GA readiness, and release-phase checks | Beta/operational SQL and migrations where used | Shared app/email/provider configuration | Beta, hardening, and GA readiness tests | Readiness evaluations are request/CI driven; deployed scheduling is unknown |
| BE-MOD-014 | Marketplace creator/submission/review portions of Marketplace routes | Domain, manifest, package, validation, submission, review, policy services | Route/service queries and Marketplace migrations; artifact files | Shared upload/application/provider configuration where consumed | Strong pure-rule/service tests plus route helpers | Submission/review lifecycle effects; no general durable workflow engine |
| BE-MOD-015 | Marketplace catalog/installation portions of Marketplace routes | Catalog and installation services | Catalog/install/entitlement SQL, migrations, package files | Shared upload/provider settings where consumed | Catalog and installation service tests | Install/uninstall lifecycle; durable compensation is not established |
| BE-MOD-016 | Marketplace runtime and adapter routers | Runtime policy, host adapters, performance/security helpers | Runtime/adapter queries and migrations where used | No dedicated runtime environment block found | Runtime and adapter route/service tests | Request-scoped host operations and kill switches; no uploaded-code worker |
| BE-MOD-017 | Marketplace finance router | Marketplace finance plus shared Stripe behavior | Finance/ledger/payout SQL and migrations | Stripe credentials/provider settings | Finance and Stripe unit tests | Provider/finance operations; settlement automation remains separately marked |
| BE-MOD-018 | Marketplace analytics router and feedback/readiness portions of Marketplace routes | Feedback, analytics, performance, readiness, and phase services | Feedback/analytics/readiness SQL and migrations | No dedicated analytics backend configuration found | Extensive pure-rule readiness/analytics/feedback tests | Readiness is request/CI evaluated; production telemetry pipeline is unverified |

## Public Interfaces

HTTP handlers registered by `backend/src/routes/mod.rs`, the OpenAPI document generated there, the `/uploads` static service, and the page preview WebSocket behavior form externally reachable interfaces. Phase 3 records their owning modules without enumerating endpoint contracts.

| Module | Observed public or route-facing interface |
|---|---|
| BE-MOD-001 | Process listener, root, health, readiness, and OpenAPI routes |
| BE-MOD-002 | Public/protected authentication routers and bearer-claims middleware contract |
| BE-MOD-003 | Tenant middleware contract (`Claims` and `TenantContext`), not an independent product endpoint family |
| BE-MOD-004 | Protected organization and tenant organization routers |
| BE-MOD-005 | Public plan/provider-callback and tenant billing routers |
| BE-MOD-006 | Tenant content-type, entry, validation, and workflow handlers |
| BE-MOD-007 | Tenant editorial-comment handlers |
| BE-MOD-008 | Tenant media handlers and shared static upload serving |
| BE-MOD-009 | Tenant page/builder handlers and preview WebSocket interface |
| BE-MOD-010 | Public delivery router and cache-backed response behavior |
| BE-MOD-011 | Tenant CMS webhook management handlers and signed outbound webhook contract |
| BE-MOD-012 | Protected plugin router and in-process `CmsPlugin` hook contract |
| BE-MOD-013 | Protected/tenant beta handlers and readiness evaluation outputs |
| BE-MOD-014 | Creator, package, submission, validation, and review handlers within Marketplace routing |
| BE-MOD-015 | Marketplace catalog and installation handlers |
| BE-MOD-016 | Marketplace runtime and host-adapter handlers/policy contract |
| BE-MOD-017 | Marketplace finance handlers and provider-facing operations |
| BE-MOD-018 | Marketplace feedback, analytics, performance, and readiness handlers/results |

## Internal Interfaces

Rust functions, DTOs, service helpers, middleware extensions, `AppState`, database queries, Redis operations, and broadcast channels are internal interfaces. Many route DTOs are `pub` because Utoipa or tests use them; that visibility does not establish a stable cross-crate API.

Direct cross-module access is most visible through middleware-owned `Claims`/`TenantContext`, shared `AppState` and `AppError`, route-owned page and dependency-check types imported by services, Delivery-owned invalidation invoked by Content/Pages, Marketplace domain/policy/package types, and SQL access to related domain data. These interfaces are tracked in the [Dependency Map](dependency-map.md); Rust visibility alone does not assign architectural ownership.

## Communication Rules Observed in Code

1. Router groups select public, authenticated, or tenant-context request paths.
2. Middleware adds claims or tenant context before handlers consume them.
3. Handlers may call services, query PostgreSQL directly, interact with Redis, access the filesystem, or call providers.
4. Services may call other services and, in some cases, import route-owned types.
5. Domain changes can trigger cache invalidation, audit records, email, Stripe, or webhook side effects.

## Observed Boundary Violations and Exceptions

- Route modules contain SQL and business rules, so HTTP, application, and persistence responsibilities are not consistently separated (`BRU-01`).
- `backend/src/services/health.rs` imports the route-owned `DependencyCheck` type from `backend/src/routes/mod.rs`; ownership and active use are unconfirmed (`DU-01`).
- `backend/src/services/marketplace_adapters.rs` imports types from `backend/src/routes/pages.rs`, and Pages imports Marketplace adapter functions (`DDU-04`). This is a concrete cross-domain coupling rather than a documented layering rule.
- Content and Pages call delivery invalidation behavior associated with a route module (`DDU-05`).
- `Claims` is defined by authentication middleware but used broadly as authorization context (`MOU-01`); `TenantContext` is defined by tenant middleware and consumed by most domain routes (`MOU-02`).
- CMS Billing and Marketplace Finance are distinct business domains but both own Stripe-facing behavior (`RO-03`).

## Phase 5 Data-Ownership Boundaries

The database groups 51 tables into 18 significant entity aggregates, but SQL write ownership remains distributed. Auth owns global identity; Organizations owns the tenant root; Content, Pages, Media, Delivery, Billing, Beta, and Marketplace families own their primary lifecycles; shared audit, entitlement, subscription, and catalog records have cross-module readers/writers. Use the [Database Ownership Map](../database/module-data-ownership.md) rather than assuming directory ownership is exclusive.

Direct SQL, RLS context, provider/filesystem work, and post-commit side effects reinforce `PBU-01` and `EOU-01`. A schema change must review every consuming module and the owning entity document.

## Inferred Future Boundary Rules

The following are recommendations only and are `PROPOSED_NOT_IMPLEMENTED`:

- Keep route modules responsible for HTTP extraction and response mapping, with reusable business rules owned by services or explicit domain modules.
- Move shared request and service contracts out of route modules before adding new cross-domain consumers.
- Give cache invalidation, event publication, and provider clients explicit infrastructure ownership.
- Assign one policy owner for tenant authorization and document which checks remain resource-specific.
- Keep CMS subscription billing and Marketplace transaction finance separate while sharing only a neutral provider adapter.

These rules must not be treated as implemented architecture until verified in code.

## Related Documentation

Use the [module catalog](module-catalog.md) for module identity, the [dependency map](dependency-map.md) for concrete edges, [services and domain](services-and-domain.md) for responsibility placement, [Database Module Data Ownership](../database/module-data-ownership.md) for table ownership, and [Phase 2 boundaries](../architecture/boundaries.md) for system-level boundaries.
