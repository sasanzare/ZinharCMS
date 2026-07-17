---
okf_document_id: "backend-module-marketplace-creator-review"
title: "Marketplace Creator, Submission, Validation, and Review"
project: "ZinharCMS"
category: "backend-module"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
module_id: "BE-MOD-014"
module_name: "Marketplace Creator, Submission, Validation, and Review"
module_paths:
  - "backend/src/routes/marketplace.rs"
  - "backend/src/services/marketplace_domain.rs"
  - "backend/src/services/marketplace_manifest.rs"
  - "backend/src/services/marketplace_package.rs"
  - "backend/src/services/marketplace_submission.rs"
  - "backend/src/services/marketplace_validation.rs"
  - "backend/src/services/marketplace_review.rs"
module_type: "Domain and HTTP module"
boundary_status: "OVERLAPPING"
primary_sources:
  - "backend/src/routes/marketplace.rs"
  - "backend/src/services/marketplace_domain.rs"
  - "backend/src/services/marketplace_manifest.rs"
  - "backend/src/services/marketplace_package.rs"
  - "backend/src/services/marketplace_submission.rs"
  - "backend/src/services/marketplace_validation.rs"
  - "backend/src/services/marketplace_review.rs"
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
  - "MODULE_BOUNDARY_UNCLEAR MBU-06"
  - "RESPONSIBILITY_OVERLAP RO-07"
  - "BUSINESS_RULE_UNVERIFIED BRU-04"
---

# Marketplace Creator, Submission, Validation, and Review

## Module Identity

| Field | Value |
|---|---|
| Module ID | `BE-MOD-014` |
| Module type | Domain and HTTP module |
| Implementation status | `IMPLEMENTED` |
| Boundary status | `OVERLAPPING` |
| Confidence | High |
| Source paths | `backend/src/routes/marketplace.rs`; `backend/src/services/marketplace_domain.rs`; `backend/src/services/marketplace_manifest.rs`; `backend/src/services/marketplace_package.rs`; `backend/src/services/marketplace_submission.rs`; `backend/src/services/marketplace_validation.rs`; `backend/src/services/marketplace_review.rs` |

## Responsibility

Verified responsibility: Manages creator profiles/listings, package and manifest upload, submissions, validation/security/compatibility reports, review queue/decisions, moderation, and review events.

Shared or inferred responsibility: Shares the very large Marketplace route module with catalog, installation, feedback, and abuse behavior; package storage overlaps filesystem infrastructure.

No additional business intent is inferred beyond current code, tests, migrations, and registered documentation.

## Owned Source Areas

- `backend/src/routes/marketplace.rs`
- `backend/src/services/marketplace_domain.rs`
- `backend/src/services/marketplace_manifest.rs`
- `backend/src/services/marketplace_package.rs`
- `backend/src/services/marketplace_submission.rs`
- `backend/src/services/marketplace_validation.rs`
- `backend/src/services/marketplace_review.rs`

Ownership means the paths are primary implementation evidence for this documentation module. It does not imply exclusive table, type, or infrastructure ownership where other modules access the same data or service.

## Entry Points

Creator/listing/submission/review handlers in `marketplace::router` plus manifest/package/submission/validation/review service functions.

## Internal Structure

Route handlers own SQL/transactions/files and DTOs; services own reusable manifest, artifact, state-transition, and evaluation policy.

## Public and Internal Interfaces

Tenant/authenticated Marketplace routes and service functions/constants for package/report/review decisions.

The intended long-term public/internal visibility contract is not separately governed unless the interface is enforced by router composition, Rust visibility, or a trait.

## Dependencies

- Internal backend dependencies: Authentication, Tenant Authorization, audit/quota/RBAC/RLS, PostgreSQL, filesystem, Marketplace policy, and validation libraries.
- Shared infrastructure: `AppState`, `AppError`, SQLx/PostgreSQL, and cross-cutting services when listed above.
- Persistence: Creators, listings, versions, artifacts, submissions, validation reports, review decisions/events.
- External libraries or services: only those named in the verified responsibility and dependency statements above.

## Consumers

Creators, Marketplace administrators, catalog/install eligibility, readiness tests, and CLI/package workflow.

## Data Concepts

Creators, listings, versions, artifacts, submissions, validation reports, review decisions/events.

This is a structural ownership view, not a table or column reference. Detailed schema documentation is deferred to Phase 5.

## Request and Processing Flows

Creator/listing to artifact upload; static evaluation to persisted reports; submission to admin review/moderation.

Detailed endpoint contracts are deferred to Phase 6.

## Error Behavior

Validation/review services return typed/app errors; file and database changes can fail independently.

## Configuration

Upload directory/size and Marketplace constants; no arbitrary package execution setting.

Secret values and local environment contents are intentionally excluded.

## Tests

Marketplace route has selected tests; manifest/package/submission/validation/review/domain services contain extensive unit/static tests.

Coverage percentages are `UNKNOWN` because no coverage report was used.

## Known Risks and Unknowns

Route ownership is broad; artifact cleanup and appeal/restoration policy are unclear. Business rationale behind policy thresholds is unverified.

Relevant markers: `MODULE_BOUNDARY_UNCLEAR MBU-06`, `RESPONSIBILITY_OVERLAP RO-07`, `BUSINESS_RULE_UNVERIFIED BRU-04`.

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
