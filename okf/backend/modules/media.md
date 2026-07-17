---
okf_document_id: "backend-module-media"
title: "Media"
project: "ZinharCMS"
category: "backend-module"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
module_id: "BE-MOD-008"
module_name: "Media"
module_paths:
  - "backend/src/routes/media.rs"
  - "backend/src/models/media.rs"
  - "backend/src/services/media_processing.rs"
module_type: "Domain, HTTP, and storage module"
boundary_status: "OVERLAPPING"
primary_sources:
  - "backend/src/routes/media.rs"
  - "backend/src/models/media.rs"
  - "backend/src/services/media_processing.rs"
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
  - "MODULE_BOUNDARY_UNCLEAR MBU-03"
  - "MODULE_OWNERSHIP_UNCLEAR MOU-04"
  - "NEEDS_OWNER_CONFIRMATION NOC-02"
---

# Media

## Module Identity

| Field | Value |
|---|---|
| Module ID | `BE-MOD-008` |
| Module type | Domain, HTTP, and storage module |
| Implementation status | `IMPLEMENTED` |
| Boundary status | `OVERLAPPING` |
| Confidence | High |
| Source paths | `backend/src/routes/media.rs`; `backend/src/models/media.rs`; `backend/src/services/media_processing.rs` |

## Responsibility

Verified responsibility: Validates multipart uploads, stores original files, creates image variants, persists media metadata, lists/updates/deletes media, and exposes public file URLs.

Shared or inferred responsibility: Filesystem state and PostgreSQL metadata share ownership across handler and processing service; `/uploads` serving is registered in the root router.

No additional business intent is inferred beyond current code, tests, migrations, and registered documentation.

## Owned Source Areas

- `backend/src/routes/media.rs`
- `backend/src/models/media.rs`
- `backend/src/services/media_processing.rs`

Ownership means the paths are primary implementation evidence for this documentation module. It does not imply exclusive table, type, or infrastructure ownership where other modules access the same data or service.

## Entry Points

`media::router`, media handlers, `process_image_variants`, and supported-image helpers.

## Internal Structure

Routes perform validation, quota/RBAC/RLS, filesystem and SQL orchestration; the service isolates blocking image work.

## Public and Internal Interfaces

Tenant media routes/DTOs and public static file URLs; internal media-processing functions.

The intended long-term public/internal visibility contract is not separately governed unless the interface is enforced by router composition, Rust visibility, or a trait.

## Dependencies

- Internal backend dependencies: Tenant Authorization, quota/audit, PostgreSQL, local filesystem, image crate, Config upload paths/limits, and root static service.
- Shared infrastructure: `AppState`, `AppError`, SQLx/PostgreSQL, and cross-cutting services when listed above.
- Persistence: Media metadata, variants, organization-scoped file paths, and public URLs.
- External libraries or services: only those named in the verified responsibility and dependency statements above.

## Consumers

SPA media library, Pages, Marketplace template adapters, and public file consumers.

## Data Concepts

Media metadata, variants, organization-scoped file paths, and public URLs.

This is a structural ownership view, not a table or column reference. Detailed schema documentation is deferred to Phase 5.

## Request and Processing Flows

Multipart input to validation/file write/variant generation/SQL; deletion to row and best-effort file removal.

Detailed endpoint contracts are deferred to Phase 6.

## Error Behavior

Unsupported media and processing failures map to validation/internal errors; file and database operations are not atomic.

## Configuration

`UPLOAD_DIR` and `MAX_UPLOAD_SIZE`; no object-store configuration exists.

Secret values and local environment contents are intentionally excluded.

## Tests

Media-processing helpers are exercised indirectly/static; no complete media route compensation integration test was found.

Coverage percentages are `UNKNOWN` because no coverage report was used.

## Known Risks and Unknowns

File/database ownership, public-byte authorization, cleanup, and multi-instance storage topology remain unclear.

Relevant markers: `MODULE_BOUNDARY_UNCLEAR MBU-03`, `MODULE_OWNERSHIP_UNCLEAR MOU-04`, `NEEDS_OWNER_CONFIRMATION NOC-02`.

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
