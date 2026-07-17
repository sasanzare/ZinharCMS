---
okf_document_id: "architecture-decision-register"
title: "Architecture Decision Register"
project: "ZinharCMS"
category: "architecture"
phase: 2
status: "current"
review_status: "verified"
source_of_truth: false
architecture_status: "mixed"
last_verified_commit: "17e69e266c558c8568ec65524560d52d7cb89d4c"
last_verified_date: "2026-07-17"
primary_sources:
  - "backend/Cargo.toml"
  - "backend/src/main.rs"
  - "backend/src/state.rs"
  - "backend/src/routes/mod.rs"
  - "backend/src/services/cache.rs"
  - "backend/src/services/marketplace_runtime.rs"
  - "frontend/package.json"
  - "frontend/src/services/api.ts"
  - "frontend/src/types/api.ts"
  - "docker-compose.prod.yml"
  - "docs/V3_MARKETPLACE_SCOPE.md"
related_documents:
  - "architecture/README.md"
  - "architecture/overview.md"
  - "architecture/boundaries.md"
  - "architecture/components.md"
  - "architecture/dependency-model.md"
  - "architecture/runtime-flows.md"
  - "architecture/integration-points.md"
  - "architecture/architecture-risks.md"
  - "architecture/decisions/README.md"
related_diagrams:
  - "architecture/diagrams/system-context.mmd"
  - "architecture/diagrams/container-view.mmd"
  - "architecture/diagrams/dependency-direction.mmd"
uncertainty_markers:
  - "INFERRED_FROM_CODE"
  - "INFERRED_FROM_STRUCTURE"
  - "INFERRED_FROM_CONFIGURATION"
  - "UNKNOWN U-01"
  - "NEEDS_OWNER_CONFIRMATION NOC-02"
  - "NEEDS_OWNER_CONFIRMATION NOC-06"
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR ABU-01"
  - "DEPENDENCY_DIRECTION_UNCLEAR DDU-03"
  - "IMPLEMENTATION_STATUS_UNCLEAR ISU-03"
---

# Architecture Decision Register

## Interpretation

These entries reconstruct current choices from the verified repository. Unless an entry is `ACCEPTED_EXPLICIT`, it does not prove who made the choice, when it was approved, which alternatives were evaluated, or why it was selected.

## Decision Summary

| ID | Decision | Status | Decision type | Confidence | Main consequence |
|---|---|---|---|---|---|
| AD-001 | Separate React SPA and HTTP backend | OBSERVED | INFERRED_FROM_CODE | High | Browser and server can build separately but share a runtime contract |
| AD-002 | One Rust modular-monolith backend | OBSERVED | INFERRED_FROM_CODE | High | One deployment unit contains all backend capabilities |
| AD-003 | Process-wide shared `AppState` | OBSERVED | INFERRED_FROM_CODE | High | Dependencies are explicit but broadly reachable |
| AD-004 | PostgreSQL as system of record with startup migrations | OBSERVED | INFERRED_FROM_CODE | High | Schema evolution is applied during server startup |
| AD-005 | Organization tenancy through middleware and RLS support | OBSERVED | INFERRED_FROM_CODE | High | Tenant context is cross-cutting and distributed |
| AD-006 | Redis for cache, rate limiting, and readiness | OBSERVED | INFERRED_FROM_CODE | High | Availability semantics differ by use case |
| AD-007 | Direct SQL in routes and services | OBSERVED | INFERRED_FROM_CODE | High | Query behavior is near features; persistence boundaries are blurred |
| AD-008 | Central frontend API client with manual TypeScript contracts | OBSERVED | INFERRED_FROM_CODE | High | Calls are centralized but contracts can drift |
| AD-009 | Local filesystem for media and package artifacts | OBSERVED | INFERRED_FROM_CODE | High | Simple storage is coupled to server filesystem topology |
| AD-010 | Built-in plugins execute in process; Marketplace packages do not | ACCEPTED | EXPLICIT | High | Extension trust models remain separate |
| AD-011 | Preview and selected side effects use process-local asynchronous state | OBSERVED | INFERRED_FROM_CODE | High | No durable or multi-instance coordination is implied |
| AD-012 | Compose defines a production-like reference topology | OBSERVED | INFERRED_FROM_CONFIGURATION | High | Actual production architecture remains unknown |

## AD-001: Separate SPA and Backend

- Status: `OBSERVED`
- Decision type: `INFERRED_FROM_CODE`
- Context: The administration UI and backend use separate manifests, entry points, build tools, and runtime processes.
- Evidence: `frontend/package.json`, `frontend/src/main.tsx`, `frontend/src/services/api.ts`, `backend/Cargo.toml`, `backend/src/main.rs`
- Observed decision: Build and run the browser administration UI separately from the Rust HTTP backend.
- Decision rationale: UNKNOWN
- Consequences: Separate toolchains and deployment artifacts; runtime contract crosses HTTP, JSON, multipart, and WebSocket boundaries; cross-application types are not compile-time linked.
- Confidence: High
- Owner confirmation status: `REQUIRED_FOR_RATIONALE`
- Related components: React SPA, central API client, Axum backend
- Related documents: [Architecture Overview](../overview.md), [System Boundaries](../boundaries.md), [Runtime Flows](../runtime-flows.md)
- Review trigger: A server-rendered UI, generated client package, or combined deployment artifact is introduced.

## AD-002: Modular-Monolith Backend

- Status: `OBSERVED`
- Decision type: `INFERRED_FROM_CODE`
- Context: All backend capabilities are modules of one Cargo package and are composed into one Axum router.
- Evidence: `backend/Cargo.toml`, `backend/src/lib.rs`, `backend/src/routes/mod.rs`
- Observed decision: Implement backend capabilities as modules in one Rust crate and server process.
- Decision rationale: UNKNOWN
- Consequences: Simple composition and in-process calls; shared failure and scaling boundary; module boundaries are conventions rather than deployment boundaries.
- Confidence: High
- Owner confirmation status: `REQUIRED_FOR_RATIONALE`
- Related components: Axum backend, route modules, services, plugins
- Related documents: [Architecture Overview](../overview.md), [Components and Responsibilities](../components.md), [Dependency Model](../dependency-model.md)
- Review trigger: A capability becomes a separate package, process, worker, or network service.

## AD-003: Shared Application State

- Status: `OBSERVED`
- Decision type: `INFERRED_FROM_CODE`
- Context: Request handlers require common access to configuration, PostgreSQL, Redis, and preview channels.
- Evidence: `backend/src/state.rs`, `backend/src/main.rs`
- Observed decision: Inject configuration, SQL pool, Redis client, and preview channels through one cloned `AppState`.
- Decision rationale: UNKNOWN
- Consequences: Central construction and test substitution are possible; broad state access increases infrastructure coupling.
- Confidence: High
- Owner confirmation status: `REQUIRED_FOR_RATIONALE`
- Related components: Backend bootstrap, `AppState`, request handlers, preview channels
- Related documents: [Components and Responsibilities](../components.md), [Dependency Model](../dependency-model.md), [Architecture Risks](../architecture-risks.md)
- Review trigger: State is decomposed into capability-specific interfaces or moved across process boundaries.

## AD-004: PostgreSQL and Startup Migrations

- Status: `OBSERVED`
- Decision type: `INFERRED_FROM_CODE`
- Context: Application domains require durable relational data and an ordered schema-evolution mechanism.
- Evidence: `backend/src/main.rs`, `backend/src/db/mod.rs`, `backend/migrations`
- Observed decision: Use PostgreSQL as durable system of record and run embedded SQL migrations during backend startup.
- Decision rationale: UNKNOWN
- Consequences: Application startup can advance schema and fail on migration error; deployment coordination must account for schema compatibility.
- Confidence: High
- Owner confirmation status: `REQUIRED_FOR_RATIONALE`
- Related components: Backend bootstrap, SQLx pool, PostgreSQL, migrations
- Related documents: [Architecture Overview](../overview.md), [Runtime Flows](../runtime-flows.md), [Integration Points](../integration-points.md)
- Review trigger: Migrations move to a separate release step or another durable store is introduced.

## AD-005: Organization Tenant Context

- Status: `OBSERVED`
- Decision type: `INFERRED_FROM_CODE`
- Context: Organization-owned CMS, SaaS, and Marketplace data requires request and database tenant context.
- Evidence: `backend/src/middleware/tenant.rs`, `backend/src/services/rls.rs`, `backend/migrations/0009_v2_phase_three_rls.sql`
- Observed decision: Represent the tenant as an organization and enforce access through request middleware, membership checks, route/service checks, and RLS-aware database context.
- Decision rationale: UNKNOWN
- Consequences: Organization identity must propagate through requests and data access; mixed enforcement paths require careful coverage review.
- Confidence: High
- Owner confirmation status: `REQUIRED_FOR_RATIONALE`
- Related components: Tenant middleware, membership queries, RLS helpers, tenant-protected routes
- Related documents: [System Boundaries](../boundaries.md), [Runtime Flows](../runtime-flows.md), [Architecture Risks](../architecture-risks.md)
- Review trigger: Tenant identity, membership semantics, or RLS strategy changes.

## AD-006: Redis with Use-Case-Specific Failure Semantics

- Status: `OBSERVED`
- Decision type: `INFERRED_FROM_CODE`
- Context: Delivery caching, request rate enforcement, and readiness need shared low-latency state or dependency checks.
- Evidence: `backend/src/services/cache.rs`, `backend/src/services/rate_limit.rs`, `backend/src/routes/mod.rs`, `backend/src/routes/delivery.rs`
- Observed decision: Use Redis for delivery caching, tenant rate limiting, and readiness checks.
- Decision rationale: UNKNOWN
- Consequences: Delivery can fall back to PostgreSQL, while rate-limit and readiness paths can reject traffic; Redis is both optimization and operational dependency.
- Confidence: High
- Owner confirmation status: `REQUIRED_FOR_RATIONALE`
- Related components: Redis client, delivery route, rate-limit service, readiness route
- Related documents: [Dependency Model](../dependency-model.md), [Runtime Flows](../runtime-flows.md), [Integration Points](../integration-points.md)
- Review trigger: Redis responsibilities split, a new cache/broker is added, or failure policy is unified.

## AD-007: Direct SQL Data Access

- Status: `OBSERVED`
- Decision type: `INFERRED_FROM_CODE`
- Context: Feature modules implement varied queries, ownership checks, and transactions without a shared repository abstraction.
- Evidence: `backend/src/routes`, `backend/src/services`, `backend/src/services/rls.rs`
- Observed decision: Permit feature route and service modules to issue SQLx queries directly rather than requiring a repository layer.
- Decision rationale: UNKNOWN
- Consequences: Queries remain close to handlers and services; data ownership, reuse, transaction boundaries, and tenant review are distributed.
- Confidence: High
- Owner confirmation status: `REQUIRED_FOR_RATIONALE`
- Related components: Route handlers, services, SQLx pool, RLS helpers
- Related documents: [System Boundaries](../boundaries.md), [Dependency Model](../dependency-model.md), [Architecture Risks](../architecture-risks.md)
- Review trigger: Repository interfaces, query modules, or an enforced domain layer are introduced.

## AD-008: Central API Client and Manual Contracts

- Status: `OBSERVED`
- Decision type: `INFERRED_FROM_CODE`
- Context: Browser features need consistent request headers, error parsing, and request and response types.
- Evidence: `frontend/src/services/api.ts`, `frontend/src/types/api.ts`, backend route request and response types
- Observed decision: Centralize browser HTTP calls in one API module while manually maintaining TypeScript contract types.
- Decision rationale: UNKNOWN
- Consequences: Header and error behavior are centralized; Rust and TypeScript shapes can diverge without compile-time failure.
- Confidence: High
- Owner confirmation status: `REQUIRED_FOR_RATIONALE`
- Related components: Frontend pages, Zustand store, central API client, backend routes
- Related documents: [Dependency Model](../dependency-model.md), [Runtime Flows](../runtime-flows.md), [Architecture Risks](../architecture-risks.md)
- Review trigger: OpenAPI generation, a shared schema, or a generated client becomes authoritative.

## AD-009: Local Artifact Storage

- Status: `OBSERVED`
- Decision type: `INFERRED_FROM_CODE`
- Context: Media and Marketplace package flows require storage for binary artifacts outside relational rows.
- Evidence: `backend/src/routes/media.rs`, `backend/src/routes/marketplace.rs`, `backend/src/config.rs`, `backend/src/routes/mod.rs`
- Observed decision: Store media and Marketplace package artifacts beneath `UPLOAD_DIR` and serve the directory under `/uploads`.
- Decision rationale: UNKNOWN
- Consequences: File durability and sharing depend on host or volume topology; files and database records lack one atomic transaction; public-file policy needs explicit review.
- Confidence: High
- Owner confirmation status: `REQUIRED_FOR_RATIONALE_AND_PRODUCTION_TOPOLOGY`
- Related components: Media routes, Marketplace routes, upload filesystem, static file service
- Related documents: [System Boundaries](../boundaries.md), [Integration Points](../integration-points.md), [Architecture Risks](../architecture-risks.md)
- Review trigger: Object storage, private download mediation, CDN delivery, or shared-volume governance is introduced.

## AD-010: Separate Built-In and Marketplace Execution Models

- Status: `ACCEPTED`
- Decision type: `EXPLICIT`
- Context: Built-in trusted extensions and creator-supplied Marketplace artifacts require different execution and trust boundaries.
- Evidence: `backend/src/plugins/mod.rs`, `backend/src/plugins/seo.rs`, `backend/src/services/marketplace_runtime.rs`, `docs/V3_MARKETPLACE_SCOPE.md`
- Observed decision: Execute trusted built-in Rust plugins in the backend process while not executing arbitrary uploaded Marketplace package code.
- Decision rationale: The Marketplace scope explicitly excludes arbitrary uploaded server-side code execution, and current runtime behavior agrees.
- Consequences: Built-in plugins share backend trust; Marketplace artifacts are governed through metadata, entitlement, installation, and allowlisted host capabilities instead of a general execution sandbox.
- Confidence: High
- Owner confirmation status: `NOT_REQUIRED_FOR_CURRENT_CLASSIFICATION`
- Related components: Built-in plugin registry, SEO plugin, Marketplace runtime, Marketplace host adapters
- Related documents: [System Boundaries](../boundaries.md), [Components and Responsibilities](../components.md), [Integration Points](../integration-points.md)
- Review trigger: Any sandboxed, remote, client-only, or server-side Marketplace execution engine is proposed or implemented.

## AD-011: Process-Local Transient Work

- Status: `OBSERVED`
- Decision type: `INFERRED_FROM_CODE`
- Context: Page preview requires realtime fan-out and CMS webhook delivery requires work after selected mutations.
- Evidence: `backend/src/state.rs`, `backend/src/routes/pages.rs`, `backend/src/services/webhooks.rs`
- Observed decision: Use in-memory broadcast channels for page preview and spawned backend tasks for selected webhook delivery.
- Decision rationale: UNKNOWN
- Consequences: State and tasks are not durable or automatically shared across backend replicas; process shutdown can interrupt work.
- Confidence: High
- Owner confirmation status: `REQUIRED_FOR_RATIONALE_AND_DELIVERY_GUARANTEE`
- Related components: `AppState` preview channels, page routes, webhook routes, spawned tasks
- Related documents: [System Boundaries](../boundaries.md), [Runtime Flows](../runtime-flows.md), [Architecture Risks](../architecture-risks.md)
- Review trigger: Durable jobs, a broker, shared pub/sub, explicit drain behavior, or multi-instance coordination is added.

## AD-012: Production-Like Compose Reference

- Status: `OBSERVED`
- Decision type: `INFERRED_FROM_CONFIGURATION`
- Context: The repository needs a runnable production-like composition for the built backend and frontend artifacts and their data dependencies.
- Evidence: `docker-compose.prod.yml`, `backend/Dockerfile.prod`, `frontend/Dockerfile.prod`, `frontend/nginx.conf`
- Observed decision: Provide a reference topology with PostgreSQL, Redis, backend, and Nginx-served frontend containers.
- Decision rationale: UNKNOWN
- Consequences: Contributors can inspect a deployable shape, but the file does not establish production provider, TLS, routing, secrets, replicas, promotion, backup, or release state.
- Confidence: High for repository configuration; low for actual production topology
- Owner confirmation status: `REQUIRED_FOR_RATIONALE_AND_PRODUCTION_TOPOLOGY`
- Related components: PostgreSQL container, Redis container, backend image, Nginx frontend image
- Related documents: [Architecture Overview](../overview.md), [System Boundaries](../boundaries.md), [Integration Points](../integration-points.md), [Architecture Risks](../architecture-risks.md)
- Review trigger: An authoritative deployment platform or environment topology is documented and verified.

## Open Decision Needs

This register intentionally does not invent decisions for public domain routing, production deployment, storage ownership, backup and recovery, observability, post-mutation delivery guarantees, or API contract generation. Those topics remain owner questions or later-phase work.
