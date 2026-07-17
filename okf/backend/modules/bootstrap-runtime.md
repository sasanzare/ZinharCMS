---
okf_document_id: "backend-module-bootstrap-runtime"
title: "Bootstrap and Runtime"
project: "ZinharCMS"
category: "backend-module"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
module_id: "BE-MOD-001"
module_name: "Bootstrap and Runtime"
module_paths:
  - "backend/src/main.rs"
  - "backend/src/lib.rs"
  - "backend/src/routes/mod.rs"
  - "backend/src/config.rs"
  - "backend/src/state.rs"
  - "backend/src/db/mod.rs"
module_type: "Application module"
boundary_status: "OBSERVED"
primary_sources:
  - "backend/src/main.rs"
  - "backend/src/lib.rs"
  - "backend/src/routes/mod.rs"
  - "backend/src/config.rs"
  - "backend/src/state.rs"
  - "backend/src/db/mod.rs"
related_documents:
  - "backend/README.md"
  - "backend/module-catalog.md"
  - "backend/module-boundaries.md"
  - "backend/dependency-map.md"
  - "backend/testing-map.md"
  - "backend/backend-risks.md"
  - "architecture/components.md"
  - "architecture/boundaries.md"
  - "architecture/dependency-model.md"
related_diagrams:
  - "backend/diagrams/backend-module-map.mmd"
  - "backend/diagrams/backend-dependency-flow.mmd"
uncertainty_markers:
  - "NEEDS_OWNER_CONFIRMATION"
  - "DEAD_OR_UNUSED_CODE_UNCONFIRMED DU-01"
---

# Bootstrap and Runtime

## Module Identity

| Field | Value |
|---|---|
| Module ID | `BE-MOD-001` |
| Module type | Application module |
| Implementation status | `IMPLEMENTED` |
| Boundary status | `OBSERVED` |
| Confidence | High |
| Source paths | `backend/src/main.rs`; `backend/src/lib.rs`; `backend/src/routes/mod.rs`; `backend/src/config.rs`; `backend/src/state.rs`; `backend/src/db/mod.rs` |

## Responsibility

Verified responsibility: Starts the backend process, loads configuration, prepares infrastructure clients, runs migrations, seeds the initial administrator when required, builds `AppState`, composes routes and global layers, serves HTTP, and handles graceful shutdown.

Shared or inferred responsibility: Health/readiness and OpenAPI composition live in the root route module; configuration, state, and database helpers are shared with every feature.

No additional business intent is inferred beyond current code, tests, migrations, and registered documentation.

## Owned Source Areas

- `backend/src/main.rs`
- `backend/src/lib.rs`
- `backend/src/routes/mod.rs`
- `backend/src/config.rs`
- `backend/src/state.rs`
- `backend/src/db/mod.rs`

Ownership means the paths are primary implementation evidence for this documentation module. It does not imply exclusive table, type, or infrastructure ownership where other modules access the same data or service.

## Entry Points

`main`, `cms_backend::app`, `routes::router`, `Config::from_env`, and `AppState::new`.

## Internal Structure

Process bootstrap is in `main.rs`; the library exposes modules and `app`; `routes/mod.rs` composes public, authenticated, and tenant-protected routers.

## Public and Internal Interfaces

The network listener and root system endpoints are external interfaces. `AppState`, `Config`, and `app` are the principal internal interfaces.

The intended long-term public/internal visibility contract is not separately governed unless the interface is enforced by router composition, Rust visibility, or a trait.

## Dependencies

- Internal backend dependencies: Axum, Tokio, Tower HTTP, SQLx/PostgreSQL, Redis, tracing, configuration, and all registered route modules.
- Shared infrastructure: `AppState`, `AppError`, SQLx/PostgreSQL, and cross-cutting services when listed above.
- Persistence: Configuration values, migration state, initial identity/organization membership, readiness results, and process-local preview channels.
- External libraries or services: only those named in the verified responsibility and dependency statements above.

## Consumers

The operating environment starts the executable; every handler consumes composed state or router behavior.

## Data Concepts

Configuration values, migration state, initial identity/organization membership, readiness results, and process-local preview channels.

This is a structural ownership view, not a table or column reference. Detailed schema documentation is deferred to Phase 5.

## Request and Processing Flows

Environment to `Config`; clients and migrations to `AppState`; route composition to listener; signals to graceful shutdown.

Detailed endpoint contracts are deferred to Phase 6.

## Error Behavior

Startup uses `anyhow::Context`; request failures use `AppError`. Initialization errors stop the process.

## Configuration

All fields in `Config`; environment names and defaults are cataloged in `configuration-and-state.md` without secret values.

Secret values and local environment contents are intentionally excluded.

## Tests

Configuration test construction exists; readiness and route composition are partly covered through colocated/static tests, but no separate process-start integration suite was found.

Coverage percentages are `UNKNOWN` because no coverage report was used.

## Known Risks and Unknowns

Startup migration coordination for multiple replicas and bootstrap credential policy require owner confirmation. `services/health.rs` has no verified runtime caller beyond module registration.

Relevant markers: `NEEDS_OWNER_CONFIRMATION`, `DEAD_OR_UNUSED_CODE_UNCONFIRMED DU-01`.

## Related Documents

- [Backend Module Catalog](../module-catalog.md)
- [Module Boundaries](../module-boundaries.md)
- [Dependency Map](../dependency-map.md)
- [Testing Map](../testing-map.md)
- [Backend Risks](../backend-risks.md)
- [Architecture Components](../../architecture/components.md)
- [Architecture Boundaries](../../architecture/boundaries.md)
- [Architecture Dependency Model](../../architecture/dependency-model.md)
- [Backend Module Map](../diagrams/backend-module-map.mmd)
- [Backend Dependency Flow](../diagrams/backend-dependency-flow.mmd)
