---
okf_document_id: "workflow-marketplace-product-publication"
title: "Marketplace Product Publication Workflow"
project: "ZinharCMS"
category: "domain-workflow"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
workflow_id: "WF-MARKET-PUBLISH"
workflow_name: "Marketplace Product Publication"
workflow_domain: "DOM-MARKETPLACE"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "backend/src/routes/marketplace.rs"
  - "backend/src/services/marketplace_submission.rs"
  - "backend/src/services/marketplace_validation.rs"
  - "backend/src/services/marketplace_review.rs"
related_documents:
  - "../cross-module-workflows.md"
  - "../domains/marketplace.md"
  - "../../backend/modules/marketplace-creator-review.md"
related_diagrams:
  - "../diagrams/cross-module-orchestration.mmd"
---

# Marketplace Product Publication Workflow

## Workflow Identity

- ID/name/domain: `WF-MARKET-PUBLISH`, Marketplace Product Publication, `DOM-MARKETPLACE`.
- Trigger/actors: approved creator uploads/submits; global admin validates/reviews/moderates.
- Status/confidence: `IMPLEMENTED`; High policy, Medium complete file/DB flow.

## Preconditions

Authenticated creator owns an approved creator profile/listing; listing metadata and product type are supported; manifest/package/version are valid enough to persist; global admin required for review decision.

## Main Flow

1. Creator creates/updates listing review metadata.
2. Upload stores a version artifact under creator/listing/version/checksum object key and records checksum/size/manifest.
3. Submission creates queued review record and moves version/listing into submitted lifecycle.
4. Validation inspects manifest, ZIP structure/assets, risky patterns, compatibility, and install eligibility; persist report/risk/status.
5. Global reviewer loads queue/evidence and records approve, changes-requested, reject, or block decision.
6. Approval updates submission/version/listing publication states and appends review event.
7. Approved compatible versions become catalog/install candidates; moderation can later suspend/block/unpublish/deprecate according to route rules.

## Alternative Flows

Validation warning can remain reviewable; plan incompatibility disables installation eligibility without necessarily blocking review. Creator can submit a later immutable version for changes.

## Failure Flows

Invalid creator/listing/manifest/package/path/checksum/semantic version rejects or produces failed validation report. High risk or blocked validation cannot be approved. File and DB operations do not share a filesystem transaction.

## State Changes

Creator, listing, version validation/status, submission review status, and review-event history. Exact states are in [State Transitions](../state-transitions.md).

## Data Changes

Creator/listing/version/submission/validation/review-event rows and artifact file.

## Transaction Boundaries

Specific review/state writes use transactions, while upload filesystem/provider boundaries are separate. Artifact immutability is enforced by trigger after protected states.

## Side Effects

Artifact storage, review history, catalog visibility, and later installation eligibility. Uploaded code is not executed.

## Completion Criteria

Approval state and valid version/listing catalog state persist; publication does not imply installation or execution.

## Tests

Strong manifest/package/validation/submission/review/static security tests. Full creator-to-catalog database/file/UI flow and operational moderation are not end-to-end tested.

## Unknowns and Risks

Multi-stage partial failure, dense status ownership, artifact retention, moderation notification, exact compatibility product policy, and Phase 9 extensibility boundary.

Return to [Cross-Module Workflows](../cross-module-workflows.md).

