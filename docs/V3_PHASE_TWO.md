# V3 Phase 2 Creator Onboarding And Submission Workflow

Phase 2 adds the first usable Marketplace creator workflow on top of the phase 1 domain model. It lets authenticated active users request creator profiles, lets platform administrators update creator verification state, lets creators save complete listing metadata, and lets approved creators upload package versions for review.

## Delivered Scope

- Phase 2.1: Creator profile request with slug, display name, bio, support email, and pending status.
- Phase 2.2: Creator verification statuses: pending, approved, suspended, and rejected.
- Phase 2.3: Listing submission metadata for title, summary, description, category, screenshots, price, license, support URL, and product type.
- Phase 2.4: Package version upload endpoint with manifest validation, checksum calculation, artifact persistence, version creation, and review submission creation.
- Admin panel Marketplace page for creator profile, listing metadata, listing submission, and package version upload.

## API Surface

Tenant-authenticated routes:

- `GET /api/marketplace/creator`
- `POST /api/marketplace/creator`
- `PATCH /api/marketplace/creators/{creator_id}/verification`
- `GET /api/marketplace/listings`
- `POST /api/marketplace/listings`
- `PUT /api/marketplace/listings/{listing_id}`
- `POST /api/marketplace/listings/{listing_id}/submit`
- `POST /api/marketplace/listings/{listing_id}/versions/upload`

The upload endpoint accepts multipart form data:

- `file`: package archive
- `manifest` or `manifest_json`: Marketplace manifest JSON

## Creator Verification Rules

- New creator requests start as `pending`.
- Platform administrators can move creators to `approved`, `suspended`, or `rejected`.
- Only `approved` creators can submit public listings or upload package versions for review.
- Rejected creators can resubmit their profile and return to `pending`.

## Listing Review Rules

A listing cannot be submitted unless it has complete metadata:

- supported product type
- safe listing slug
- title
- summary
- description
- category
- pricing type and valid price
- license
- at least one screenshot URL
- optional support URL when provided

Incomplete listings are rejected by the API before they enter the review queue.

## Version Submission Rules

A package version submission must include:

- valid Marketplace manifest
- manifest product type matching the listing product type
- package file within the size limit
- computed SHA-256 checksum
- deterministic object key
- immutable submitted version artifact data
- linked `marketplace_submissions` review row

## Database Changes

Migration file:

- `backend/migrations/0016_v3_phase_two_creator_submission.sql`

Main changes:

- creator verification metadata and phase 2 status set
- listing description, price, license, screenshots, and submitted metadata
- listing submission queue index
- stricter immutability trigger for submitted package versions

## Acceptance Criteria

- An authenticated active user can request a creator profile.
- Creator statuses match the phase 2 proposal: pending, approved, suspended, rejected.
- An unapproved creator cannot submit a public listing or package version.
- Incomplete listing metadata cannot enter review.
- Every package version upload creates an immutable version connected to a submission row.