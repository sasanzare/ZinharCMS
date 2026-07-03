# V3 Phase 1 Marketplace Domain, Manifest, Migration, And Package Storage

Phase 1 turns the V3 Marketplace policy and V2 readiness audit into the first concrete Marketplace foundation: domain model, manifest contract, base database migration, and package artifact integrity rules.

## Delivered Scope

- Phase 1.1: Marketplace domain model for Creator, Listing, Package, Version, Submission, Installation, and Purchase.
- Phase 1.2: Standard manifest contract with required fields, permissions, compatibility, entry points, and assets.
- Phase 1.3: Base Marketplace migration for creators, listings, versions, submissions, and installations.
- Phase 1.4: Package artifact object-key, checksum, size-limit, and metadata rules.
- Backend validation helpers for Marketplace manifests and package checksums.
- Static backend tests that assert the migration and documentation stay aligned with the proposal.

## Phase 1.1 Domain Model

The domain model is documented in `docs/V3_MARKETPLACE_DOMAIN_MODEL.md`.

Core entities:

- Creator
- Listing
- Package
- Version
- Submission
- Installation
- Purchase

The first physical migration creates Creator, Listing, Version, Submission, and Installation tables. Purchase is modeled now but intentionally left for the paid-product phases because phase 9 introduces Marketplace purchase, entitlement, ledger, and payout behavior.

## Phase 1.2 Manifest Standard

The manifest contract is documented in `docs/V3_MARKETPLACE_MANIFEST_SCHEMA.md` and validated by `backend/src/services/marketplace_manifest.rs`.

Required manifest fields:

- `manifest_version`
- `name`
- `version`
- `type`
- `permissions`
- `compatibility`
- `entry_points`
- `assets`

Invalid manifests must be rejected before a package version can move into review.

## Phase 1.3 Base Migration

Migration file:

- `backend/migrations/0015_v3_phase_one_marketplace_foundation.sql`

Created tables:

- `marketplace_creators`
- `marketplace_listings`
- `marketplace_versions`
- `marketplace_submissions`
- `marketplace_installations`

Important constraints:

- creator and listing slug format checks
- supported product type checks
- supported review and install status checks
- semantic version format check
- required manifest fields check
- SHA-256 checksum format check
- package size limit check
- unique listing/version pair
- active installation uniqueness per organization/listing
- forced RLS on tenant-owned installations

## Phase 1.4 Package Storage And Checksum

The package storage contract is documented in `docs/V3_PACKAGE_STORAGE.md` and implemented in `backend/src/services/marketplace_package.rs`.

Package versions require:

- object key
- SHA-256 checksum
- artifact size in bytes
- file name
- content type
- storage metadata

A package without a valid SHA-256 checksum cannot be submitted for review or installed.

## Acceptance Criteria

- The relationship between owner, creator, organization, listing, version, installation, and future purchase is documented.
- Manifest validation rejects missing required fields and unsupported permissions.
- Base migration creates the required Marketplace tables with key constraints and indexes.
- Tenant-owned installations have forced RLS and tenant policies.
- Package artifact storage requires object key, checksum, size limit, and metadata.
- Approved package version artifact data is immutable.

## Non-Goals

This phase does not add public API endpoints, frontend Marketplace pages, creator onboarding UI, upload endpoints, install runtime behavior, paid purchases, payout logic, or plugin sandboxing. Those belong to later V3 phases.