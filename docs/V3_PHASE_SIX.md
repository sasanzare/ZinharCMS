# V3 Phase 6 Marketplace Installation Lifecycle

Phase 6 turns the approved Marketplace registry into an organization-owned installation lifecycle. It records and manages installed products without executing uploaded code. The initial installable MVP remains limited to free Component Packs and Design Templates.

## 6.1 Install API

Delivered scope:

- `POST /api/marketplace/installations` installs one approved package version for the active organization.
- `GET /api/marketplace/installations` lists current non-uninstalled products for any active organization member.
- Install, permission approval, and every lifecycle mutation require organization `owner` or `admin`.
- The install request contains `listing_id`, `version_id`, and the exact `approved_permissions` array shown to the administrator.
- The installation stores the installed version, active status, immutable permission snapshot, approver, approval time, pinned-version flag, install actor, timestamps, and cleanup policy.
- The mutation and its organization-scoped audit record commit in the same tenant transaction.

Install gates are re-evaluated at mutation time:

- listing status is `approved`
- version status is `approved`
- validation status is `passed` or `warning`
- security risk is `low` or `medium`
- current organization plan/version/features remain compatible
- approved permissions exactly match the version manifest
- artifact exists under `UPLOAD_DIR`, has the stored size, and matches the approved SHA-256 checksum
- product type is `component_pack` or `design_template`
- pricing type is `free`

Paid and custom products fail with a deterministic entitlement-required conflict. V2 organization subscription billing is not treated as a Marketplace purchase entitlement.

## 6.2 Enable, Disable, And Uninstall

Delivered endpoints:

- `POST /api/marketplace/installations/{installation_id}/enable`
- `POST /api/marketplace/installations/{installation_id}/disable`
- `POST /api/marketplace/installations/{installation_id}/uninstall`

Lifecycle rules:

- `active -> disabled`
- `disabled -> active`, after safety, compatibility, permission snapshot, and artifact integrity are rechecked
- `active|disabled|blocked -> uninstalled`
- blocked or uninstalled installations cannot be enabled or updated
- uninstall is soft: the installation and audit history remain stored while the installed-app list excludes it
- `cleanup_policy` is always `preserve_organization_data`; uninstall never deletes organization content, media, pages, settings, or other dependent data

The schema stores `enabled_at`, `disabled_at`, `uninstalled_at`, `version_changed_at`, and the existing `updated_at` lifecycle timestamps.

## 6.3 Update And Rollback

Delivered endpoints:

- `GET /api/marketplace/installations/{installation_id}/updates`
- `POST /api/marketplace/installations/{installation_id}/update`
- `POST /api/marketplace/installations/{installation_id}/rollback`

Update behavior:

- update checks compare semantic-version precedence rather than upload timestamps
- only a strictly newer approved, validated, low/medium-risk, compatible version is eligible
- the update request must set `changelog_confirmed = true`
- installations remain pinned to their explicit `version_id`; no background auto-update occurs
- the previous version is stored as `rollback_version_id`
- permission changes require a fresh exact approval snapshot from an owner/admin
- the target artifact is size/checksum verified before the version changes

Rollback behavior:

- rollback is limited by a composite foreign key to a version from the same listing
- the previous target may be `approved` or `deprecated`; deprecated prevents new installation but does not remove the promised safe rollback path
- blocked, rejected, failed, high-risk, critical-risk, incompatible, missing, or tampered versions cannot be restored
- the earlier approved permission snapshot is preserved in installation metadata and restored when valid
- rollback swaps the current and rollback versions, allowing a controlled reversal of the rollback

## Migration

Migration `0019_v3_phase_six_installation_lifecycle.sql` is additive and leaves applied Marketplace migrations unchanged. It adds lifecycle timestamps, `cleanup_policy`, `version_pinned`, an organization/update index, and a same-listing composite foreign key for `rollback_version_id`.

## Tenant Isolation And Moderation

- Installation reads and writes use forced PostgreSQL RLS with tenant-scoped connections or transactions.
- Audit records use `marketplace.installation.install`, `enable`, `disable`, `uninstall`, `update`, and `rollback` action names.
- Catalog popularity counts use a narrowly scoped bypass transaction and return only aggregate active-install counts.
- Authorized global-admin emergency blocking uses an explicit bypass transaction so it can block matching installations across organizations despite forced RLS.

## Deferred Boundaries

Phase 6 does not implement Marketplace purchases, paid entitlements, arbitrary package execution, plugin sandboxing, runtime permission enforcement, or automatic updates. Purchase/entitlement remains Phase 9 work; runtime permission and sandbox enforcement remains Phase 7/8 work.

## Acceptance

- Installation fails without approval, exact permission consent, compatibility, safe review status, or intact artifact bytes.
- Paid/custom and unsupported runtime product types cannot enter the Phase 6 install lifecycle.
- Uninstall preserves organization data and an auditable soft-uninstall record.
- Updates require a newer semantic version and changelog confirmation.
- Permission changes cannot be introduced by update without fresh approval.
- A safe compatible previous version can be restored, including a deprecated version retained specifically for rollback.
