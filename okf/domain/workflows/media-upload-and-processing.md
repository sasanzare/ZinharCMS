---
okf_document_id: "workflow-media-upload-processing"
title: "Media Upload and Processing Workflow"
project: "ZinharCMS"
category: "domain-workflow"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
workflow_id: "WF-MEDIA-UPLOAD"
workflow_name: "Media Upload and Processing"
workflow_domain: "DOM-MEDIA"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "backend/src/routes/media.rs"
  - "backend/src/services/media_processing.rs"
  - "backend/src/services/quota.rs"
related_documents:
  - "../cross-module-workflows.md"
  - "../domains/media-library.md"
  - "../deletion-and-restoration.md"
related_diagrams:
  - "../diagrams/cross-module-orchestration.mmd"
---

# Media Upload and Processing Workflow

## Workflow Identity

- ID/name/domain: `WF-MEDIA-UPLOAD`, Media Upload and Processing, `DOM-MEDIA`.
- Trigger/actor: organization media writer submits multipart file.
- Status/confidence: `IMPLEMENTED`; High behavior, Low atomicity assurance.

## Preconditions

Authentication, active tenant, media-writer role, required file field, configured max size, detected allowed content type, optional declaration match, and plan media capacity.

## Main Flow

1. Parse multipart file, alt text, and caption; ignore unknown fields.
2. Sanitize display filename and derive content MIME from bytes.
3. Enforce configured size and tenant plan capacity.
4. Create tenant source and variant directories.
5. Generate UUID-based stored filename and write source bytes.
6. Insert media metadata row.
7. For JPEG/PNG/WebP, decode and generate four WebP variants in blocking task.
8. Write each variant file and insert each variant row.
9. Return media plus variants.

## Alternative Flows

PDF/text uploads create no variants. Missing declaration defaults to `application/octet-stream` and is accepted when content detection succeeds.

## Failure Flows

Validation/quota failures occur before file write. Directory/file/DB/image/variant failure can occur after earlier filesystem or database state exists. No compensation transaction removes partial artifacts.

## State Changes

No status entity. Source/variant existence is the lifecycle state.

## Data Changes

Source file, media row, up to four variant files, and four variant rows.

## Transaction Boundaries

No database transaction spans the media and all variant inserts, and filesystem cannot participate in PostgreSQL atomicity.

## Side Effects

Filesystem writes and CPU-bound image decoding/resizing. No queue/retry/audit on successful upload.

## Completion Criteria

All required file and row writes finish and response includes variants. Partial failure does not meet completion but may leave state.

## Tests

No dedicated upload/type/quota/variant/partial-failure integration tests found.

## Unknowns and Risks

Orphans, resource exhaustion, content scanning depth, production storage, media-reference deletion, and repair/reconciliation.

Return to [Cross-Module Workflows](../cross-module-workflows.md).

