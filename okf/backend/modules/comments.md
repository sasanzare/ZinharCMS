---
okf_document_id: "backend-module-comments"
title: "Editorial Comments"
project: "ZinharCMS"
category: "backend-module"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
module_id: "BE-MOD-007"
module_name: "Editorial Comments"
module_paths:
  - "backend/src/routes/comments.rs"
module_type: "Domain and HTTP module"
boundary_status: "OBSERVED"
primary_sources:
  - "backend/src/routes/comments.rs"
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
  - "MODULE_OWNERSHIP_UNCLEAR MOU-03"
  - "PUBLIC_INTERFACE_UNCLEAR PIU-01"
---

# Editorial Comments

## Module Identity

| Field | Value |
|---|---|
| Module ID | `BE-MOD-007` |
| Module type | Domain and HTTP module |
| Implementation status | `IMPLEMENTED` |
| Boundary status | `OBSERVED` |
| Confidence | High |
| Source paths | `backend/src/routes/comments.rs` |

## Responsibility

Verified responsibility: Creates, lists, reads, resolves, reopens, and deletes comments attached to entry or page entities under tenant and role checks.

Shared or inferred responsibility: Entity existence is checked against Content or Pages data using application logic rather than a module service interface.

No additional business intent is inferred beyond current code, tests, migrations, and registered documentation.

## Owned Source Areas

- `backend/src/routes/comments.rs`

Ownership means the paths are primary implementation evidence for this documentation module. It does not imply exclusive table, type, or infrastructure ownership where other modules access the same data or service.

## Entry Points

`comments::router` and its route handlers.

## Internal Structure

The route file contains DTOs, direct SQL, permission checks, entity validation, and response mapping.

## Public and Internal Interfaces

Tenant comment HTTP routes and comment request/response DTOs.

The intended long-term public/internal visibility contract is not separately governed unless the interface is enforced by router composition, Rust visibility, or a trait.

## Dependencies

- Internal backend dependencies: Tenant Authorization/RBAC/RLS, PostgreSQL, and the entry/page data model.
- Shared infrastructure: `AppState`, `AppError`, SQLx/PostgreSQL, and cross-cutting services when listed above.
- Persistence: Comments plus polymorphic references to content entries or pages.
- External libraries or services: only those named in the verified responsibility and dependency statements above.

## Consumers

Editorial SPA flows and users collaborating on content or pages.

## Data Concepts

Comments plus polymorphic references to content entries or pages.

This is a structural ownership view, not a table or column reference. Detailed schema documentation is deferred to Phase 5.

## Request and Processing Flows

Tenant request to entity existence check and comment query/mutation; resolve/reopen updates status.

Detailed endpoint contracts are deferred to Phase 6.

## Error Behavior

Unknown entity or comment maps to not found; permission, validation, and SQL errors use `AppError`.

## Configuration

No module-specific configuration or feature flag was found.

Secret values and local environment contents are intentionally excluded.

## Tests

No colocated comment route test block or separate backend integration-test directory was found.

Coverage percentages are `UNKNOWN` because no coverage report was used.

## Known Risks and Unknowns

The module directly queries related entity data and lacks a database foreign-key boundary for polymorphic references.

Relevant markers: `MODULE_OWNERSHIP_UNCLEAR MOU-03`, `PUBLIC_INTERFACE_UNCLEAR PIU-01`.

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
