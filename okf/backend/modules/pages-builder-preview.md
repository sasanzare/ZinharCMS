---
okf_document_id: "backend-module-pages-builder-preview"
title: "Pages, Builder, and Preview"
project: "ZinharCMS"
category: "backend-module"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
module_id: "BE-MOD-009"
module_name: "Pages, Builder, and Preview"
module_paths:
  - "backend/src/routes/pages.rs"
  - "backend/src/models/page.rs"
  - "backend/src/state.rs"
module_type: "Domain and HTTP module"
boundary_status: "OVERLAPPING"
primary_sources:
  - "backend/src/routes/pages.rs"
  - "backend/src/models/page.rs"
  - "backend/src/state.rs"
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
  - "RESPONSIBILITY_OVERLAP RO-05"
  - "MODULE_BOUNDARY_UNCLEAR MBU-04"
  - "DEPENDENCY_DIRECTION_UNCLEAR DDU-04"
---

# Pages, Builder, and Preview

## Module Identity

| Field | Value |
|---|---|
| Module ID | `BE-MOD-009` |
| Module type | Domain and HTTP module |
| Implementation status | `IMPLEMENTED` |
| Boundary status | `OVERLAPPING` |
| Confidence | High |
| Source paths | `backend/src/routes/pages.rs`; `backend/src/models/page.rs`; `backend/src/state.rs` |

## Responsibility

Verified responsibility: Manages pages, workflow, versions, component registry, page JSON validation, publication, and WebSocket preview broadcasts.

Shared or inferred responsibility: Uses Delivery invalidation, Webhooks, workflow, audit/quota/RBAC/RLS and shared process-local preview channels; Marketplace adapters import page types/helpers.

No additional business intent is inferred beyond current code, tests, migrations, and registered documentation.

## Owned Source Areas

- `backend/src/routes/pages.rs`
- `backend/src/models/page.rs`
- `backend/src/state.rs`

Ownership means the paths are primary implementation evidence for this documentation module. It does not imply exclusive table, type, or infrastructure ownership where other modules access the same data or service.

## Entry Points

`pages::router`, page/component handlers, `validate_page_json_for_tenant`, preview WebSocket handling, and preview broadcast helpers.

## Internal Structure

One route file owns DTOs, direct SQL, validation, workflow, version snapshots, component operations, and realtime preview.

## Public and Internal Interfaces

Tenant page/component HTTP routes, preview WebSocket, `PageResponse`, and a `pub(crate)` validation helper.

The intended long-term public/internal visibility contract is not separately governed unless the interface is enforced by router composition, Rust visibility, or a trait.

## Dependencies

- Internal backend dependencies: Tenant Authorization, PostgreSQL, Delivery cache, Webhooks, shared `AppState`, workflow, and Marketplace adapters.
- Shared infrastructure: `AppState`, `AppError`, SQLx/PostgreSQL, and cross-cutting services when listed above.
- Persistence: Pages, versions, component registry, page JSON, workflow status, and transient broadcast channels.
- External libraries or services: only those named in the verified responsibility and dependency statements above.

## Consumers

SPA page builder, public Delivery, Comments, Marketplace templates/components, and preview clients.

## Data Concepts

Pages, versions, component registry, page JSON, workflow status, and transient broadcast channels.

This is a structural ownership view, not a table or column reference. Detailed schema documentation is deferred to Phase 5.

## Request and Processing Flows

Page CRUD/workflow to version/cache/webhook effects; preview connection to in-memory broadcast sender.

Detailed endpoint contracts are deferred to Phase 6.

## Error Behavior

Validation, workflow, not-found, and SQL errors use `AppError`; preview channel loss is process-local behavior.

## Configuration

Shared upload/API config only; preview channels have no separate durable transport configuration.

Secret values and local environment contents are intentionally excluded.

## Tests

No colocated backend page-route test block was found; frontend Pages tests and shared workflow tests provide partial evidence.

Coverage percentages are `UNKNOWN` because no coverage report was used.

## Known Risks and Unknowns

Large route responsibility, process-local preview, and cross-imports from Marketplace adapters create overlap.

Relevant markers: `RESPONSIBILITY_OVERLAP RO-05`, `MODULE_BOUNDARY_UNCLEAR MBU-04`, `DEPENDENCY_DIRECTION_UNCLEAR DDU-04`.

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
