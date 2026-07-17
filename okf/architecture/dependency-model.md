---
okf_document_id: "architecture-dependency-model"
title: "Dependency Model"
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
  - "backend/src/lib.rs"
  - "backend/src/routes/mod.rs"
  - "backend/src/middleware/auth.rs"
  - "backend/src/middleware/tenant.rs"
  - "backend/src/services/jwt.rs"
  - "backend/src/services/health.rs"
  - "backend/src/services/quota.rs"
  - "backend/src/services/rate_limit.rs"
  - "frontend/src/router.tsx"
  - "frontend/src/services/api.ts"
  - "frontend/src/types/api.ts"
related_documents:
  - "architecture/README.md"
  - "architecture/overview.md"
  - "architecture/boundaries.md"
  - "architecture/components.md"
  - "architecture/architecture-risks.md"
  - "architecture/decisions/decision-register.md"
  - "backend/dependency-map.md"
  - "backend/module-boundaries.md"
  - "backend/shared-infrastructure.md"
related_diagrams:
  - "architecture/diagrams/dependency-direction.mmd"
  - "architecture/diagrams/container-view.mmd"
  - "backend/diagrams/backend-dependency-flow.mmd"
uncertainty_markers:
  - "INFERRED_FROM_STRUCTURE"
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR ABU-01"
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR ABU-02"
  - "DEPENDENCY_DIRECTION_UNCLEAR DDU-01"
  - "DEPENDENCY_DIRECTION_UNCLEAR DDU-02"
  - "DEPENDENCY_DIRECTION_UNCLEAR DDU-03"
  - "DEPENDENCY_DIRECTION_UNCLEAR DDU-04"
  - "DEPENDENCY_DIRECTION_UNCLEAR DDU-05"
---

# Dependency Model

## Dependency Units

The backend has one Cargo package and one application process. Dependencies described here are Rust source-module relationships, not independently versioned package or network-service dependencies. The frontend is a separate JavaScript package and build artifact, connected to the backend only at runtime.

## Dependency Principles

| Principle | Classification | Evidence and qualification |
|---|---|---|
| Compose dependencies at process entry points | `INFERRED_CONVENTION` | `backend/src/main.rs` constructs state/router and `frontend/src/main.tsx` constructs the UI root; no written repository rule was found |
| Route through central backend and frontend integration points | `INFERRED_CONVENTION` | Backend routes are assembled in `routes/mod.rs`; verified frontend `fetch` calls are centralized in `services/api.ts` |
| Apply authentication and tenant context before protected handlers | `EXPLICIT_CONVENTION` | Router groups explicitly attach authentication or tenant middleware |
| Prefer reusable service and plugin modules for shared behavior | `INFERRED_CONVENTION` | Routes call services/plugins, but direct SQL and infrastructure calls mean the convention is incomplete |
| Treat PostgreSQL migrations as schema authority | `EXPLICIT_CONVENTION` | Embedded migrations are executed at startup and define tracked schema/RLS evolution |
| Keep built-in and Marketplace package execution trust separate | `EXPLICIT_CONVENTION` | Code and Marketplace scope agree that arbitrary uploaded server package code is not executed |

No explicit repository-wide clean-layer, domain-driven, command/query, or repository-pattern rule was found.

## Observed Backend Direction

The dominant source direction is:

1. process bootstrap composes shared state and the router;
2. routes depend on middleware, services, plugins, models, shared state, and infrastructure clients;
3. middleware depends on shared state and selected services;
4. services depend on shared state, domain types, other services, and external libraries;
5. routes and services issue SQL through SQLx or RLS helpers;
6. infrastructure adapters call PostgreSQL, Redis, the filesystem, and external HTTPS providers.

This is a useful navigation model, but it is not an enforced layered architecture. Direct SQL and infrastructure access occur in route modules, and source dependencies cross the suggested layer boundaries.

## Verified Direction Exceptions

### DDU-01: Middleware and Service Coupling

Authentication middleware imports JWT verification functions, while `services/jwt.rs` imports the middleware-owned `Claims` type. Tenant middleware invokes quota and rate-limit services, while quota, rate-limit, and RLS services import the middleware-owned `TenantContext` type.

This is verified bidirectional source coupling through types and functions. It is not a Cargo package cycle, but it makes middleware definitions part of service contracts.

### DDU-02: Service to Route Type Dependency

`backend/src/services/health.rs` imports `DependencyCheck` from `backend/src/routes`. This reverses the usual route-to-service direction and couples a reusable dependency check to an HTTP-layer response model.

### DDU-03: Cross-Application Contract Duplication

The Rust backend owns runtime request and response behavior, while `frontend/src/types/api.ts` manually describes the client view of many contracts. No generated client, shared schema package, or automated equivalence boundary was found. The dependency is behavioral rather than a compile-time source dependency, so contract drift can compile independently.

### DDU-04: Pages and Marketplace Adapter Coupling

Pages imports Marketplace host-adapter behavior, while `backend/src/services/marketplace_adapters.rs` imports page-owned types. This creates bidirectional source pressure across apparent domain and route/service boundaries. The deeper evidence is registered in [Backend Module Boundaries](../backend/module-boundaries.md) and the [Backend Dependency Map](../backend/dependency-map.md).

### DDU-05: Domain Routes to Delivery-Owned Invalidation

Content and Pages invoke cache invalidation behavior owned by the Delivery route area. The operation is cross-cutting infrastructure, but its current source ownership creates a route-to-route dependency and ambiguous maintenance boundary.

## Persistence Direction

There is no repository or data-access package that all domains must use. Route handlers and services call SQLx directly; some use tenant-aware RLS helpers and some carry out explicit ownership checks. Migrations define schema and policies but are not imported as typed domain contracts. This structure keeps query behavior near features but blurs persistence ownership and makes cross-cutting isolation review more expensive.

## Shared State Coupling

`AppState` exposes configuration, SQL pool, Redis client, and preview channels to request code. This makes dependency construction explicit at the process boundary, but it also gives many modules broad infrastructure reach. Configuration is likewise a shared cross-cutting dependency rather than a set of narrowly typed component options.

## Frontend Direction

The dominant frontend direction is:

1. `main.tsx` initializes i18n and the router;
2. the router depends on guards, layout, and pages;
3. pages depend on shared components, state, i18n, API methods, and API types;
4. the Zustand store depends on API types and exposes setters used by the API module;
5. the API module depends on types, browser storage, and `fetch`;
6. the API module calls the backend over HTTP or WebSocket URLs.

All observed `fetch` calls are centralized in the API module. However, shared token and organization state crosses module boundaries through both the store and `localStorage`, so ownership of session persistence is distributed.

## Runtime Dependency Semantics

| Dependency | Direction | Failure behavior observed |
|---|---|---|
| Backend to PostgreSQL | Required | Startup migration or request operations fail; readiness reports failure |
| Backend to Redis for readiness | Required for ready state | Readiness fails when Redis cannot be reached |
| Backend to Redis for delivery cache | Optional optimization | Cache failure falls back to PostgreSQL; invalidation is best effort |
| Backend to Redis for tenant rate limit | Request-critical | Failure becomes service unavailable |
| Backend to filesystem | Request-critical for affected media/package operations | File operation can fail independently of database work |
| Backend to Stripe | Request-critical for billing operation | External response is mapped to application failure |
| Backend to email webhook | Mode-dependent | Strict mode can fail the originating operation |
| Backend to CMS webhook receiver | Spawned best-effort task | Delivery result is recorded, but no durable retry worker is present |

## Dependency Rules for AI Agents

These rules describe review constraints derived from current evidence and risks; they are not claims that the repository already enforces every rule:

- Preserve the verified dominant dependency direction and inspect actual imports before changing it.
- Do not introduce a reverse-layer dependency without recording the owning layer, contract, evidence, and risk.
- Do not access another module's internals when an existing public function, type, service, or plugin interface already provides the capability.
- Keep transport response types out of reusable service contracts where practical; do not extend DDU-02 silently.
- Treat `Claims` and `TenantContext` as cross-cutting contracts whose ownership needs an explicit decision.
- Verify Rust route types and TypeScript client types together before duplicating or changing a contract.
- Route new frontend HTTP calls through the central API client unless an explicit, documented boundary requires otherwise.
- Review every new direct SQL path for organization ownership, transaction scope, and RLS behavior.
- Treat spawned external side effects as non-durable unless a durable mechanism is explicitly implemented.
- Update architecture documents, diagrams, risks, decisions, and `okf/index.yaml` when adding a major dependency or changing direction.

For the backend-only dependency inventory, including module-to-module, shared-state, persistence, and external edges, use the [Backend Dependency Map](../backend/dependency-map.md) and [Backend Dependency Flow Diagram](../backend/diagrams/backend-dependency-flow.mmd).
