# V3 Marketplace Domain Model

This document defines the V3 Marketplace entities introduced in phase 1.1 and clarifies ownership boundaries before API and UI work starts.

## Ownership Rules

- Creator ownership is user/creator-scoped.
- Listing and package version metadata are global Marketplace records.
- Installation state is organization-owned and always references `organization_id`.
- Purchase and entitlement state will be organization-owned when paid products are implemented.
- Review and moderation decisions are platform-scoped but must be auditable.
- A creator does not automatically get access to any organization that installs their product.

## Entities

### Creator

A Creator represents a user who can publish Marketplace products.

Key fields:

- `id`
- `user_id`
- `slug`
- `display_name`
- `status`
- `payout_status`
- `support_email`
- `metadata`

Relationships:

- one Creator belongs to one user
- one Creator owns many Listings
- one Creator can later receive ledger and payout records

### Listing

A Listing is the public Marketplace product record.

Key fields:

- `id`
- `creator_id`
- `product_type`
- `title`
- `slug`
- `summary`
- `category`
- `status`
- `pricing_type`
- `metadata`

Relationships:

- one Listing belongs to one Creator
- one Listing has many Versions
- one Listing can be installed by many Organizations
- one Listing can later have purchases, reviews, reports, and analytics

### Package

A Package is the artifact boundary for a Listing. In phase 1 it is represented through Version artifact fields rather than a separate table.

Key package data:

- object key
- file name
- content type
- size in bytes
- SHA-256 checksum
- storage metadata

Rules:

- package artifacts are stored in the registry, not executed directly
- checksum is required before review or install
- object keys must be deterministic and safe

### Version

A Version is an immutable package version for a Listing.

Key fields:

- `id`
- `listing_id`
- `version`
- `manifest_schema_version`
- `manifest_json`
- `artifact_object_key`
- `artifact_sha256`
- `artifact_size_bytes`
- `artifact_file_name`
- `artifact_content_type`
- `storage_metadata`
- `status`

Relationships:

- one Version belongs to one Listing
- one Version can have many Submissions
- one Version can be installed by many Organizations

Rules:

- `listing_id` plus `version` is unique
- manifest must contain the required fields
- checksum must be a valid lowercase SHA-256 hex value
- artifact size must be positive and within the configured limit
- approved, deprecated, or blocked package artifact data is immutable

### Submission

A Submission is a review request for one Version.

Key fields:

- `id`
- `version_id`
- `submitted_by`
- `review_status`
- `risk_level`
- `review_notes`
- `validation_report`
- `reviewed_by`
- `reviewed_at`

Relationships:

- one Submission belongs to one Version
- one Submission can be reviewed by one platform reviewer

Rules:

- submissions move through queued, validating, changes requested, approved, rejected, blocked, or canceled states
- validation reports are JSON objects
- review decisions must be audited in later API phases

### Installation

An Installation is the organization-owned state of a Marketplace product.

Key fields:

- `id`
- `organization_id`
- `listing_id`
- `version_id`
- `installed_by`
- `status`
- `permissions_json`
- `permission_approved_by`
- `permission_approved_at`
- `rollback_version_id`
- `metadata`
- `installed_at`

Relationships:

- one Installation belongs to one Organization
- one Installation references one Listing and one Version from that Listing
- one Organization can have at most one non-uninstalled Installation per Listing

Rules:

- installation rows are tenant-owned
- installation rows require forced RLS
- permissions are captured as an install-time snapshot
- rollback state references a Marketplace Version

### Purchase

A Purchase represents paid Marketplace entitlement for an Organization. It is domain-modeled in phase 1 but physically deferred to the paid-product phases.

Expected future fields:

- `organization_id`
- `listing_id`
- `version_id`
- `provider_payment_id`
- `amount_cents`
- `currency`
- `status`
- `entitlement_status`

Rules:

- purchase entitlement is separate from V2 organization subscription state
- successful payment must be confirmed by provider webhook
- purchase, refund, ledger, and payout effects must be auditable

## Phase 1 Migration Boundary

Phase 1 creates the tables needed for creator/listing/version/submission/install foundations. Paid purchase, entitlement, ledger, review, report, and payout tables are intentionally deferred to their dedicated V3 phases.