# V3 Phase 0.1 Marketplace Scope And Policy Lock

Phase 0.1 starts V3 by locking what the Marketplace is allowed to accept before schema design, creator onboarding, package uploads, review queues, installation, or payments are built.

## Objective

Define exactly which product types ZinharCMS V3 accepts, which product types are deferred, and which submissions must be rejected as unsupported or unsafe.

## Delivered Scope

- Added the Marketplace scope document.
- Added the Marketplace product taxonomy.
- Added the initial Marketplace review and moderation policy.
- Added backend static tests that keep the V3 phase 0.1 policy outputs present.

## Required Decisions

- V3 starts with registry-owned Marketplace products. Uploaded files are never executed directly.
- Every accepted product must have an owner, type, version, manifest, checksum, compatibility range, review state, and permission model.
- Component Pack and Design Template are the first P0 product types.
- Integration Plugin is allowed as P1 only through reviewed public integration contracts.
- Backend Extension is P2 and blocked from production execution until a sandbox and permission boundary are ready.
- Unsupported products are rejected before review or placed in quarantine when risk is unclear.

## Acceptance Criteria

- The team can classify a submission as Component Pack, Design Template, Integration Plugin, Backend Extension, or Unsupported.
- The team can explain why a submission is approved, rejected, quarantined, or sent back for changes.
- The initial V3 Marketplace scope is documented before domain model or migration work starts.
- Security, permission, review, takedown, and unsupported-product rules are explicit.

## Related Documents

- `docs/V3_MARKETPLACE_SCOPE.md`
- `docs/V3_PRODUCT_TAXONOMY.md`
- `docs/V3_MARKETPLACE_POLICY.md`

## Non-Goals

This phase does not create database tables, APIs, UI pages, package uploads, payment flows, or runtime execution. Those belong to later V3 phases.