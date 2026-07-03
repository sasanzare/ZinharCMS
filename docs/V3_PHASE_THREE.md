# V3 Phase 3 Package Validation, Security Scan, And Compatibility

Phase 3 adds the automated review layer that runs after a creator uploads a package version and before a reviewer can approve it for publication. The goal is to make every submitted package produce a visible validation report for both the creator and Marketplace reviewers.

## Delivered Scope

- Phase 3.1: Static validation pipeline for manifest shape, package file tree, asset limits, semantic versioning, entry points, and dependency metadata.
- Phase 3.2: Initial security scan for forbidden files, external references, suspicious permissions, executable artifacts, backend extensions, and unsafe dependency declarations.
- Phase 3.3: Compatibility report for minimum and maximum ZinharCMS version, required Marketplace features, and required organization plan.
- Stored validation, security, and compatibility reports on each package version.
- Stored the same review report on each Marketplace submission so creator and reviewer views can render the report without recalculating package state.
- Creator-facing and reviewer-facing API access to submitted validation reports.
- Admin panel Marketplace UI sections that show validation reports as JSON for creator submissions and reviewer inspection.

## Backend API Surface

- `GET /api/marketplace/listings/{listing_id}/submissions`
- `GET /api/marketplace/review/reports`
- `POST /api/marketplace/listings/{listing_id}/versions/upload`

The upload endpoint now runs phase 3 validation before it creates the version and submission rows. The response includes the submitted version, the review submission, and the validation report.

## Static Validation

The static validation pipeline checks:

- manifest schema and semantic version shape
- uploaded package is a ZIP archive with a readable central directory
- file count and uncompressed size limits
- unsafe paths such as absolute paths, Windows paths, and `..` traversal
- manifest assets exist in the package file tree
- manifest entry points exist in the package file tree
- dependency metadata is bounded and has a supported shape

A package with failed static validation is stored with `validation_status = 'failed'`, `status = 'blocked'`, and a blocked submission so the creator and reviewer can inspect the failure report.

## Initial Security Scan

The initial security scan flags:

- forbidden secret files such as `.env`, private keys, npm credentials, and cloud credentials
- executable or script artifacts such as `.exe`, `.dll`, `.sh`, `.ps1`, and `.jar`
- backend extensions, which remain blocked until the sandbox phase
- external script or URL references in manifest metadata
- suspicious permissions such as external network access, webhook sending, settings access, and write permissions
- unsafe dependency declarations such as URL, git, wildcard, or latest dependencies

High or critical risk submissions are blocked from publication and remain visible to human reviewers.

## Compatibility Check

The compatibility report records:

- current ZinharCMS Marketplace compatibility target
- `min_zinhar_version`
- `max_zinhar_version`
- `required_features`
- `required_plan`
- whether the active organization would be install eligible

This phase does not implement installation yet. It produces the compatibility contract that the future install button must use to disable incompatible products.

## Database Changes

Migration `0017_v3_phase_three_validation_pipeline.sql` adds these package version columns:

- `validation_status`
- `validation_report`
- `security_risk_level`
- `compatibility_report`

The migration also adds constraints and indexes for validation queues and compatibility lookups.

## Acceptance Coverage

- Static validation report is visible to creators and reviewers.
- Package file tree, asset limits, version, and dependency metadata are checked before review.
- High-risk packages are blocked before publication and routed into the human review surface.
- Compatibility reports include ZinharCMS version bounds, required features, and plan requirements.
- Incompatible organizations have a machine-readable `install_eligible = false` value for the future install button.