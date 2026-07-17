---
okf_document_id: "backend-dependency-map"
title: "Backend Dependency Map"
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
  - "backend/src/lib.rs"
  - "backend/src/routes/mod.rs"
  - "backend/src/services/mod.rs"
  - "backend/src/state.rs"
related_documents:
  - "backend/module-catalog.md"
  - "backend/module-boundaries.md"
  - "backend/request-handling.md"
  - "backend/shared-infrastructure.md"
  - "architecture/dependency-model.md"
related_diagrams:
  - "backend/diagrams/backend-dependency-flow.mmd"
uncertainty_markers:
  - "DEPENDENCY_DIRECTION_UNCLEAR DDU-04"
  - "MODULE_BOUNDARY_UNCLEAR MBU-01"
  - "DEAD_OR_UNUSED_CODE_UNCONFIRMED DU-01"
---

# Backend Dependency Map

## Intended Reading

This map records compile-time imports and visible runtime calls at a module level. It does not claim that every branch executes in every deployment. Because ZinharCMS is one Rust crate, an edge denotes source or runtime dependence rather than a network hop.

## Primary Dependency Edges

| Source | Target | Type | Evidence | Reason | Status | Risk |
|---|---|---|---|---|---|---|
| `main` | Config, DB, services, AppState, router | Bootstrap/runtime | `backend/src/main.rs` | Construct and run the application | Observed | Startup concentrates several responsibilities |
| `routes::router` | All route modules | Composition | `backend/src/routes/mod.rs` | Register public, protected, and tenant-protected surfaces | Observed | One composition file changes with most features |
| Protected routers | Authentication middleware | Request context | `backend/src/routes/mod.rs` | Require verified claims | Observed | Claims type has broad consumers (`MOU-01`) |
| Tenant routers | Tenant middleware | Request context/policy | `backend/src/routes/mod.rs` | Resolve tenant and enforce organization-level checks | Observed | Enforcement is cross-cutting (`MBU-01`) |
| Route handlers | AppState | Shared runtime state | route modules and `backend/src/state.rs` | Access configuration, PostgreSQL, Redis, preview channels | Observed | Broad state access enables hidden coupling |
| Route handlers | Services | Application/domain call | imports in `backend/src/routes/*.rs` | Reuse rules, validation, and integrations | Observed | Direction is generally route to service |
| Route handlers | SQLx/PgPool | Persistence | SQL queries in route modules | Read and mutate PostgreSQL | Observed | Bypasses a repository boundary |
| Service modules | SQLx/PgPool | Persistence | service function signatures and queries | Execute reusable persistence behavior | Observed | Persistence is distributed |
| Tenant middleware/services | RLS/RBAC/rate/quota services | Policy | middleware and service imports | Establish and enforce request policy | Observed | Some resource authorization remains in handlers |
| Content and Pages | Delivery cache behavior | Cross-domain invalidation | route imports/calls | Invalidate public delivery data after writes | Observed | Route-owned infrastructure edge (`DDU-05`) |
| Pages | Marketplace Adapters | Cross-domain application call | `routes/pages.rs`, `services/marketplace_adapters.rs` | Resolve host behavior for builder/runtime integration | Observed | Bidirectional ownership pressure (`DDU-04`) |
| Marketplace services | Marketplace Domain/Policy/Manifest/Package | Domain/service call | `backend/src/services/marketplace_*.rs` | Share package, policy, and lifecycle concepts | Observed | Dense subgraph raises change impact |
| Billing | Stripe billing service | External provider adapter | billing route/service imports | Manage SaaS subscription operations | Observed | Provider behavior overlaps Marketplace Finance |
| Marketplace Finance | Stripe HTTP behavior | External provider integration | finance route/service sources | Process Marketplace financial workflows | Observed | Separate lifecycle must remain explicit |
| Media | Filesystem and media processing | Infrastructure | media route/service sources | Store and inspect upload content | Observed | Database/file consistency is not atomic |
| Delivery and other modules | Redis | Cache/rate infrastructure | cache, rate-limit, and route sources | Cache delivery data and counters | Observed | Redis availability affects multiple paths |
| Webhook/email helpers | HTTP client/provider endpoints | External integration | service sources | Deliver outbound notifications | Observed | Retry/durability guarantees vary or are unclear |
| AppState | Preview broadcast map | Process-local state | `backend/src/state.rs` | Fan out page preview updates | Observed | State is not shared across replicas |
| Health service | Route `DependencyCheck` | Reverse layer import | `backend/src/services/health.rs` | Reuse a response-owned type | Observed import; active use unconfirmed | `DU-01`, `DDU-01` |

## Shared Infrastructure Dependencies

Most domain modules depend on `AppError`, `AppState`, authentication claims, tenant context, SQLx, Serde, Utoipa annotations, UUIDs, and time types. Many also depend on audit, cache, email, webhook, RBAC, RLS, rate-limit, or quota helpers. These edges are legitimate cross-cutting dependencies but their ownership is convention-based rather than enforced by a separate infrastructure crate.

## Persistence Dependencies

PostgreSQL is the system-of-record dependency visible across authentication, organizations, billing, content, comments, media metadata, pages, webhooks, and Marketplace modules. Redis supports cache and limit behavior. The filesystem stores uploads. Preview broadcast channels are process-local. Detailed tables and queries are deferred to Phase 4.

## External Dependencies

| Dependency | Current backend use | Configuration evidence | Availability behavior |
|---|---|---|---|
| PostgreSQL | Persistent data and migrations | `DATABASE_URL` | Required during migration and request persistence |
| Redis | Delivery cache and request/security counters | `REDIS_URL` | Client creation occurs at startup; operation behavior is call-site specific |
| Stripe | CMS billing and Marketplace finance | optional Stripe variables | Some features are unavailable or degraded when not configured |
| Email webhook/log provider | Notification delivery | email provider variables | Behavior depends on provider and failure mode |
| Local filesystem | Media storage and `/uploads` serving | `UPLOAD_DIR` | Shared-storage behavior across replicas is unverified |
| Outbound webhook targets | CMS event delivery | persisted webhook configuration | Delivery guarantees require operation-specific review |

## High-Coupling Areas

- `backend/src/routes/marketplace.rs` and the Marketplace service family form the densest feature dependency area.
- `AppState` grants most handlers direct infrastructure access.
- Tenant context and authorization are consumed by nearly every tenant route.
- Content, Pages, Delivery, Webhooks, and Plugins coordinate around publication changes.
- Organizations, Authentication, Billing, Beta operations, and Marketplace entitlements share user/tenant lifecycle concepts.

## Circular or Reverse-Layer Exceptions

No separate-crate dependency cycle exists because these modules compile inside one crate. Source-level reverse or bidirectional pressure is nevertheless visible:

- Services importing route-owned types (`services/health.rs` and `services/marketplace_adapters.rs`).
- Pages importing Marketplace adapter behavior while the adapter service imports Pages types (`DDU-04`).
- Domain routes calling cache invalidation behavior owned by Delivery routes (`DDU-05`).

These are documented exceptions, not proof of runtime recursion.

## Unclear Dependencies

`DEAD_OR_UNUSED_CODE_UNCONFIRMED DU-01`: `backend/src/services/health.rs` is registered in `services/mod.rs`, but no verified production caller was found during Phase 3 inspection. Registration alone is not proof of active use or dead code.

Deployment-specific provider topology, Redis failure policy across all call sites, horizontal preview behavior, and durable webhook processing remain `UNKNOWN` without runtime evidence.

## Related Documentation

See the [Backend Dependency Flow Diagram](diagrams/backend-dependency-flow.mmd), [Backend Module Boundaries](module-boundaries.md), [Shared Infrastructure](shared-infrastructure.md), [Persistence Access](persistence-access.md), and the system-level [Dependency Model](../architecture/dependency-model.md).
