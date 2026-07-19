---
okf_document_id: "marketplace-area-package-validation-review"
title: "Marketplace Package Validation and Review"
project: "ZinharCMS"
category: "extensibility"
phase: 9
status: "current"
source_of_truth: false
implementation_view: "observed"
extensibility_status: "verified"
last_verified_commit: "56d733985fdd7aa3f25ee6981b88cf29c52f65c9"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/services/marketplace_manifest.rs"
  - "backend/src/services/marketplace_package.rs"
  - "backend/src/services/marketplace_validation.rs"
  - "backend/src/services/marketplace_review.rs"
  - "backend/src/routes/marketplace.rs"
marketplace_area_id: "MPA-002"
marketplace_area_name: "Package Validation and Review"
implementation_status: "implemented"
related_documents:
  - "../marketplace-architecture.md"
  - "../plugin-manifest.md"
  - "creator-listing-version.md"
related_diagrams:
  - "../diagrams/marketplace-installation-flow.mmd"
---

# Marketplace Package Validation and Review

The pipeline validates manifest shape, semantic versions, supported permissions/product types, package tree, path safety, entry-point and asset existence, size/checksum evidence, adapter declarations, compatibility, and security findings. Results are stored as validation reports and feed review eligibility.

Human review supports approve, reject, request changes, listing suspension, version unpublishing, and emergency blocking. Review and moderation events are auditable. Approved, compatible, sufficiently low-risk versions can reach the catalog/install gate.

Validation does not prove package code safe to execute, because no code executor exists. Signature-chain verification, external malware-scanner deployment, and complete supply-chain provenance are PLUGIN_TRUST_MODEL_UNCLEAR.

See [Marketplace Architecture](../marketplace-architecture.md).

## Purpose

Prevent ineligible or unsafe package versions from reaching the catalog/install path and record human governance decisions.

## Entities

marketplace_versions, marketplace_submissions, validation reports, review events, listing/version status, security risk, and artifact fields.

## Backend Module

marketplace_manifest.rs, marketplace_package.rs, marketplace_validation.rs, marketplace_review.rs, marketplace_policy.rs, and routes/marketplace.rs.

## APIs

Version upload, submission/review queue, validation-report views, review decision, listing moderation, version unpublish, and emergency block routes.

## Frontend Feature

MarketplacePage exposes creator validation reports and administrative review/moderation queues.

## Permissions

Creators upload owned listings; global Marketplace reviewers/admins decide and moderate. Install permissions are separate.

## Tenant Scope

Review/catalog state is global. Compatibility reports can include active organization plan context.

## Workflows

MP-WF-03, MP-WF-04, and MP-WF-14.

## Tests

Manifest, package, validation, policy, review, phase-thirteen security, and frontend review tests cover selected gates.

## Risks

No package signature chain or executable safety certification exists. External scanner deployment, review SLA, false positives, and artifact/storage operations remain UNKNOWN.

## Implementation Status

IMPLEMENTED validation and review pipeline; execution safety is not certified.
