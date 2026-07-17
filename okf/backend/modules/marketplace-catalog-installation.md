---
okf_document_id: "backend-module-marketplace-catalog-installation"
title: "Marketplace Catalog and Installation"
project: "ZinharCMS"
category: "backend-module"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
module_id: "BE-MOD-015"
module_name: "Marketplace Catalog and Installation"
module_paths:
  - "backend/src/routes/marketplace.rs"
  - "backend/src/services/marketplace_catalog.rs"
  - "backend/src/services/marketplace_performance.rs"
  - "backend/src/services/marketplace_installation.rs"
module_type: "Domain and HTTP module"
boundary_status: "OVERLAPPING"
primary_sources:
  - "backend/src/routes/marketplace.rs"
  - "backend/src/services/marketplace_catalog.rs"
  - "backend/src/services/marketplace_performance.rs"
  - "backend/src/services/marketplace_installation.rs"
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
  - "DOCUMENTATION_CODE_CONFLICT DCC-06"
  - "RESPONSIBILITY_OVERLAP RO-08"
  - "MODULE_OWNERSHIP_UNCLEAR MOU-05"
---

# Marketplace Catalog and Installation

## Module Identity

| Field | Value |
|---|---|
| Module ID | `BE-MOD-015` |
| Module type | Domain and HTTP module |
| Implementation status | `IMPLEMENTED` |
| Boundary status | `OVERLAPPING` |
| Confidence | High |
| Source paths | `backend/src/routes/marketplace.rs`; `backend/src/services/marketplace_catalog.rs`; `backend/src/services/marketplace_performance.rs`; `backend/src/services/marketplace_installation.rs` |

## Responsibility

Verified responsibility: Builds organization-aware catalog views and compatibility results, lists installations, enforces install/update/rollback eligibility, and manages installation lifecycle state.

Shared or inferred responsibility: Shares route/data with creator/review and finance entitlements; catalog visibility and installation gates depend on several Marketplace services.

No additional business intent is inferred beyond current code, tests, migrations, and registered documentation.

## Owned Source Areas

- `backend/src/routes/marketplace.rs`
- `backend/src/services/marketplace_catalog.rs`
- `backend/src/services/marketplace_performance.rs`
- `backend/src/services/marketplace_installation.rs`

Ownership means the paths are primary implementation evidence for this documentation module. It does not imply exclusive table, type, or infrastructure ownership where other modules access the same data or service.

## Entry Points

Catalog and installation handlers in `marketplace::router`; catalog/installation/performance service functions.

## Internal Structure

Catalog projection helpers, cache-header policy, lifecycle validation, artifact checksum and permission checks support direct handler SQL.

## Public and Internal Interfaces

Catalog and installation HTTP routes plus reusable compatibility/lifecycle functions and DTOs.

The intended long-term public/internal visibility contract is not separately governed unless the interface is enforced by router composition, Rust visibility, or a trait.

## Dependencies

- Internal backend dependencies: Tenant Authorization, PostgreSQL, creator/review records, package artifacts, entitlements, runtime policy, quotas and audit.
- Shared infrastructure: `AppState`, `AppError`, SQLx/PostgreSQL, and cross-cutting services when listed above.
- Persistence: Listings/versions/reviews projections, installations, permissions, version history, and entitlement checks.
- External libraries or services: only those named in the verified responsibility and dependency statements above.

## Consumers

Marketplace UI, Runtime/Adapters, Finance entitlement gates, analytics, and readiness tests.

## Data Concepts

Listings/versions/reviews projections, installations, permissions, version history, and entitlement checks.

This is a structural ownership view, not a table or column reference. Detailed schema documentation is deferred to Phase 5.

## Request and Processing Flows

Query to approved compatible catalog; installation request to eligibility/artifact/entitlement checks and lifecycle mutation.

Detailed endpoint contracts are deferred to Phase 6.

## Error Behavior

Compatibility/lifecycle conflicts map to validation/conflict errors; SQL and artifact failures propagate through `AppError`.

## Configuration

Marketplace version/permission constants and cache-header policy; no background auto-update configuration.

Secret values and local environment contents are intentionally excluded.

## Tests

Catalog/performance and nine installation service tests plus Marketplace route contract tests.

Coverage percentages are `UNKNOWN` because no coverage report was used.

## Known Risks and Unknowns

The catalog is named public in some concepts but is tenant-protected. Route sharing and paid entitlement coupling blur ownership.

Relevant markers: `DOCUMENTATION_CODE_CONFLICT DCC-06`, `RESPONSIBILITY_OVERLAP RO-08`, `MODULE_OWNERSHIP_UNCLEAR MOU-05`.

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
