---
okf_document_id: "marketplace-area-creator-listing-version"
title: "Marketplace Creator, Listing, and Version Management"
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
  - "backend/src/routes/marketplace.rs"
  - "backend/src/services/marketplace_domain.rs"
  - "backend/src/services/marketplace_submission.rs"
  - "backend/migrations/0015_v3_phase_one_marketplace_foundation.sql"
  - "frontend/src/pages/MarketplacePage.tsx"
marketplace_area_id: "MPA-001"
marketplace_area_name: "Creator Listing and Version Management"
implementation_status: "implemented"
related_documents:
  - "../marketplace-architecture.md"
  - "../marketplace-workflows.md"
  - "package-validation-review.md"
related_diagrams:
  - "../diagrams/marketplace-installation-flow.mmd"
---

# Marketplace Creator, Listing, and Version Management

Verified routes support creator profile creation/update, administrator verification, creator-owned listing creation/update/submission, artifact and manifest upload, version records, and submission history. The frontend Marketplace page exposes corresponding forms and queues.

Creator/listing/version/submission data is global Marketplace publication data, while authenticated roles and ownership rules govern mutation. Upload computes/stores artifact metadata and enters validation/review; it does not install or execute the package.

Key boundaries are creator verification, listing ownership, product type consistency between listing and manifest, immutable checksum evidence, and explicit submission/review states. Provider storage deployment and concurrent upload behavior remain UNKNOWN.

See [Marketplace Architecture](../marketplace-architecture.md).

## Purpose

Establish creator identity and move product metadata plus immutable artifact evidence toward review.

## Entities

marketplace_creators, marketplace_listings, marketplace_versions, marketplace_submissions, artifact metadata, and audit/review records.

## Backend Module

routes/marketplace.rs with marketplace_domain.rs, marketplace_submission.rs, marketplace_manifest.rs, and marketplace_package.rs.

## APIs

Creator profile and verification, listing create/update/submit, version upload, submission history, catalog/detail, and creator-owned list routes.

## Frontend Feature

MarketplacePage provides creator profile, listing, upload, and submission/review views.

## Permissions

Authenticated creator ownership gates listing/version changes; global Marketplace administrators verify creators and review submissions.

## Tenant Scope

Creator/listing/version publication data is global Marketplace state. Active organization context affects customer compatibility/install flows, not creator ownership.

## Workflows

MP-WF-01 through MP-WF-04 in [Marketplace Workflows](../marketplace-workflows.md).

## Tests

Marketplace domain, manifest, package, submission, policy, and MarketplacePage tests cover selected positive/negative rules. Real storage/DB concurrency remains unverified.

## Risks

Artifact provenance, creator ownership races, upload/storage atomicity, and moderation operations require integration and operational verification.

## Implementation Status

IMPLEMENTED for host-owned creator/listing/version/submission behavior; package execution is not part of this area.
