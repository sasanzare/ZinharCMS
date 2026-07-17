---
okf_document_id: "backend-module-beta-release-operations"
title: "Beta and Release Operations"
project: "ZinharCMS"
category: "backend-module"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
module_id: "BE-MOD-013"
module_name: "Beta and Release Operations"
module_paths:
  - "backend/src/routes/beta.rs"
  - "backend/src/services/ga_readiness.rs"
  - "backend/src/services/marketplace_phase_fourteen.rs"
  - "backend/src/services/marketplace_phase_fifteen.rs"
module_type: "Application and HTTP module"
boundary_status: "OVERLAPPING"
primary_sources:
  - "backend/src/routes/beta.rs"
  - "backend/src/services/ga_readiness.rs"
  - "backend/src/services/marketplace_phase_fourteen.rs"
  - "backend/src/services/marketplace_phase_fifteen.rs"
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
  - "IMPLEMENTATION_STATUS_UNCLEAR ISU-03"
  - "NEEDS_OWNER_CONFIRMATION NOC-06"
  - "RESPONSIBILITY_OVERLAP RO-06"
---

# Beta and Release Operations

## Module Identity

| Field | Value |
|---|---|
| Module ID | `BE-MOD-013` |
| Module type | Application and HTTP module |
| Implementation status | `IMPLEMENTED` |
| Boundary status | `OVERLAPPING` |
| Confidence | High |
| Source paths | `backend/src/routes/beta.rs`; `backend/src/services/ga_readiness.rs`; `backend/src/services/marketplace_phase_fourteen.rs`; `backend/src/services/marketplace_phase_fifteen.rs` |

## Responsibility

Verified responsibility: Records beta participants, feedback and GA blockers, exposes product/organization dashboards, and supplies static readiness contracts for later Marketplace beta/GA documentation.

Shared or inferred responsibility: Depends on Organizations, quotas, audit and Marketplace operational documents; readiness services are test/document contracts rather than independent runtime workers.

No additional business intent is inferred beyond current code, tests, migrations, and registered documentation.

## Owned Source Areas

- `backend/src/routes/beta.rs`
- `backend/src/services/ga_readiness.rs`
- `backend/src/services/marketplace_phase_fourteen.rs`
- `backend/src/services/marketplace_phase_fifteen.rs`

Ownership means the paths are primary implementation evidence for this documentation module. It does not imply exclusive table, type, or infrastructure ownership where other modules access the same data or service.

## Entry Points

`beta::router`, `beta::protected_router`, beta handlers, and readiness contract tests.

## Internal Structure

Route SQL/DTOs implement operational records; readiness modules contain static verification tests.

## Public and Internal Interfaces

Authenticated/tenant beta routes and dashboard DTOs. No separate release-control service interface exists.

The intended long-term public/internal visibility contract is not separately governed unless the interface is enforced by router composition, Rust visibility, or a trait.

## Dependencies

- Internal backend dependencies: Tenant Authorization, Organizations, PostgreSQL, audit/quota/RBAC/RLS, and tracked docs/scripts.
- Shared infrastructure: `AppState`, `AppError`, SQLx/PostgreSQL, and cross-cutting services when listed above.
- Persistence: Participants, feedback, blockers, dashboards, usage projections, and documentation evidence.
- External libraries or services: only those named in the verified responsibility and dependency statements above.

## Consumers

Beta administration UI, tenant feedback flows, product operators, and CI/test runs.

## Data Concepts

Participants, feedback, blockers, dashboards, usage projections, and documentation evidence.

This is a structural ownership view, not a table or column reference. Detailed schema documentation is deferred to Phase 5.

## Request and Processing Flows

Admin/tenant request to beta records/dashboard; tests to repository readiness assertions.

Detailed endpoint contracts are deferred to Phase 6.

## Error Behavior

Route validation/permissions/SQL use `AppError`; documentation-contract test failures fail test execution only.

## Configuration

No dedicated runtime feature flag was found; beta/release status remains repository/document evidence.

Secret values and local environment contents are intentionally excluded.

## Tests

`routes/beta.rs` has four tests; GA/readiness/phase services contain static contract tests.

Coverage percentages are `UNKNOWN` because no coverage report was used.

## Known Risks and Unknowns

Actual beta/GA deployment, sign-off authority, and runtime ownership cannot be inferred from readiness assets.

Relevant markers: `IMPLEMENTATION_STATUS_UNCLEAR ISU-03`, `NEEDS_OWNER_CONFIRMATION NOC-06`, `RESPONSIBILITY_OVERLAP RO-06`.

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
