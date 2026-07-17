---
okf_document_id: "backend-module-marketplace-runtime-adapters"
title: "Marketplace Runtime Security and Host Adapters"
project: "ZinharCMS"
category: "backend-module"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
module_id: "BE-MOD-016"
module_name: "Marketplace Runtime Security and Host Adapters"
module_paths:
  - "backend/src/routes/marketplace_runtime.rs"
  - "backend/src/routes/marketplace_adapters.rs"
  - "backend/src/services/marketplace_runtime.rs"
  - "backend/src/services/marketplace_adapters.rs"
  - "backend/src/services/marketplace_policy.rs"
module_type: "Domain policy and adapter module"
boundary_status: "OVERLAPPING"
primary_sources:
  - "backend/src/routes/marketplace_runtime.rs"
  - "backend/src/routes/marketplace_adapters.rs"
  - "backend/src/services/marketplace_runtime.rs"
  - "backend/src/services/marketplace_adapters.rs"
  - "backend/src/services/marketplace_policy.rs"
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
  - "DEPENDENCY_DIRECTION_UNCLEAR DDU-04"
  - "RESPONSIBILITY_OVERLAP RO-09"
  - "PLANNED_NOT_IMPLEMENTED PNI-01"
---

# Marketplace Runtime Security and Host Adapters

## Module Identity

| Field | Value |
|---|---|
| Module ID | `BE-MOD-016` |
| Module type | Domain policy and adapter module |
| Implementation status | `IMPLEMENTED` |
| Boundary status | `OVERLAPPING` |
| Confidence | High |
| Source paths | `backend/src/routes/marketplace_runtime.rs`; `backend/src/routes/marketplace_adapters.rs`; `backend/src/services/marketplace_runtime.rs`; `backend/src/services/marketplace_adapters.rs`; `backend/src/services/marketplace_policy.rs` |

## Responsibility

Verified responsibility: Lists permission policy, authorizes runtime requests, manages organization/global kill switches, exposes installed components, previews/imports templates, and authorizes allowlisted hooks without executing uploaded package code.

Shared or inferred responsibility: Adapters import page response/validation helpers and mutate page/component data; runtime state depends on installations and audit.

No additional business intent is inferred beyond current code, tests, migrations, and registered documentation.

## Owned Source Areas

- `backend/src/routes/marketplace_runtime.rs`
- `backend/src/routes/marketplace_adapters.rs`
- `backend/src/services/marketplace_runtime.rs`
- `backend/src/services/marketplace_adapters.rs`
- `backend/src/services/marketplace_policy.rs`

Ownership means the paths are primary implementation evidence for this documentation module. It does not imply exclusive table, type, or infrastructure ownership where other modules access the same data or service.

## Entry Points

Runtime and adapter routers; authorization/policy functions; host-owned component/template/hook adapters.

## Internal Structure

Route modules own SQL and HTTP DTOs; services own permission decision structs, adapter validation, and policy constants.

## Public and Internal Interfaces

Tenant runtime/permission/kill-switch/template/component/hook routes plus policy/adapter service functions.

The intended long-term public/internal visibility contract is not separately governed unless the interface is enforced by router composition, Rust visibility, or a trait.

## Dependencies

- Internal backend dependencies: Marketplace Catalog/Installation, Pages, Media, Content validation, Tenant Authorization, audit/quota/RBAC/RLS, PostgreSQL.
- Shared infrastructure: `AppState`, `AppError`, SQLx/PostgreSQL, and cross-cutting services when listed above.
- Persistence: Permission catalog, installation approvals, kill switches, components, template imports, pages/assets, hooks.
- External libraries or services: only those named in the verified responsibility and dependency statements above.

## Consumers

Marketplace UI, installed products, page builder/template import, administrators, and QA/readiness tests.

## Data Concepts

Permission catalog, installation approvals, kill switches, components, template imports, pages/assets, hooks.

This is a structural ownership view, not a table or column reference. Detailed schema documentation is deferred to Phase 5.

## Request and Processing Flows

Installation and requested capability to policy allow/deny; host adapter request to validated host-owned mutation.

Detailed endpoint contracts are deferred to Phase 6.

## Error Behavior

Policy denials and inactive/killed installations return authorization/conflict errors; adapter validation/SQL use `AppError`.

## Configuration

Compile-time permission/host policy constants; no uploaded-code executor or sandbox configuration.

Secret values and local environment contents are intentionally excluded.

## Tests

Runtime/policy/adapter services and both route modules contain unit/static tests; external execution tests are intentionally absent.

Coverage percentages are `UNKNOWN` because no coverage report was used.

## Known Risks and Unknowns

Cross-route imports from Pages and runtime routes are reverse/cross-feature dependencies. Fine-grained revocation/external execution remains unimplemented.

Relevant markers: `DEPENDENCY_DIRECTION_UNCLEAR DDU-04`, `RESPONSIBILITY_OVERLAP RO-09`, `PLANNED_NOT_IMPLEMENTED PNI-01`.

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
