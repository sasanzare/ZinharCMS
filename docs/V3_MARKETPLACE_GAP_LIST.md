# V3 Marketplace Gap List

This gap list is the actionable output of V3 phase 0.2. It records what V2 does not yet provide for Marketplace installation, creator payment, and permissions.

## Status Summary

No critical V2 dependency remains ambiguous. The gaps below are known V3 implementation work, not blockers to phase 1.1 domain modeling.

## Phase 1 Update

Phase 1 resolves the base schema and integrity gaps for `marketplace_installations`, package versions, manifest metadata, package object keys, SHA-256 checksums, size limits, and tenant-owned installation RLS. Runtime install APIs, permission enforcement, paid purchases, entitlement, ledger, payout, and sandbox controls remain open for later V3 phases.

## Phase 3 Update

Phase 3 resolves the package validation pipeline gap for uploaded versions. Static validation, initial security scanning, and compatibility reporting now run during version upload, persist reports on `marketplace_versions` and `marketplace_submissions`, and expose creator/reviewer report APIs. Runtime install enforcement, catalog filtering, and reviewer approval decisions remain open for later phases.

## Phase 4 Update

Phase 4 resolves the initial human review and moderation workflow gap. Reviewers can open queue items, approve, reject, request changes, suspend listings, unpublish versions, and emergency block products. Later phases still need public catalog visibility, install runtime enforcement, abuse reporting, and full incident runbooks.

## Plugin Install Gaps

| Gap | Severity | Required V3 work | Target phase |
| --- | --- | --- | --- |
| Marketplace installation runtime API does not exist | P0 | Phase 1 added `marketplace_installations`; later phases must add tenant-aware install, disable, uninstall, update, and rollback APIs. | 6.1, 6.2, 6.3 |
| Public catalog visibility is not implemented | P0 | Phase 4 added reviewer decisions and moderation statuses; later phases must expose only approved, compatible listings through catalog APIs and UI. | 4.1, 5.1 |
| Install compatibility enforcement is not implemented | P0 | Phase 3 stores machine-readable compatibility and `install_eligible`; later phases must enforce it in catalog display and install APIs. | 3.3, 6.1 |
| Install rollback API is not implemented | P1 | Phase 1 added rollback version metadata; later phases must implement rollback execution and safety checks. | 6.3 |
| Installed product lifecycle runtime events are not implemented | P0 | Phase 1 defined install statuses; later phases must implement install, disable, uninstall, update, rollback, suspend, and kill-switch actions. | 6.1, 6.2, 7.3 |
| Tenant-owned RLS policies for later Marketplace tables do not exist | P0 | Phase 1 added forced RLS for installations; later purchase, entitlement, review, and organization-scoped product state tables must also use forced RLS. | 9.2, 10.1 |

## Creator Payment Gaps

| Gap | Severity | Required V3 work | Target phase |
| --- | --- | --- | --- |
| Creator payout and advanced profile operations do not exist | P0 | Phase 2 added creator request and verification APIs; later phases must add payout eligibility, provider onboarding, and advanced creator operations. | 2.1, 2.2 |
| Marketplace purchase table does not exist | P1 | Add purchase records separate from organization subscriptions. | 9.2 |
| Marketplace entitlement table does not exist | P1 | Add product entitlement linked to purchase, organization, listing, and version. | 9.2 |
| Revenue split ledger does not exist | P1 | Add ledger rows for gross amount, platform commission, creator share, refunds, and settlement state. | 9.3 |
| Payout provider is not connected | P1 | Add Stripe Connect or payout provider onboarding and payout status tracking. | 9.4 |
| Refund policy is not modeled | P1 | Define refund impact on entitlement, reviews, revenue ledger, and creator balance. | 9.2, 9.3 |
| Creator self-purchase prevention is not modeled | P1 | Detect and block creator-owned organization purchases where policy disallows them. | 9.2 |

## Permission Gaps

| Gap | Severity | Required V3 work | Target phase |
| --- | --- | --- | --- |
| Marketplace permission catalog does not exist | P0 | Define allowed permissions such as content read/write, page read/write, media read/write, webhook send, external network, and settings access. | 7.1 |
| Install-time permission approval is not enforced by runtime | P0 | Phase 1 added installation permission snapshot fields; later phases must enforce approval before runtime actions. | 7.1, 6.1 |
| Marketplace RBAC helpers do not exist | P0 | Add helpers for catalog browsing, free install, paid purchase, permission approval, creator submission, and platform moderation. | 7.1 |
| Permission escalation on update is not modeled | P0 | Require reapproval when a new version asks for broader permissions. | 6.3, 7.1 |
| Runtime permission enforcement is not implemented | P0 | Enforce approved permissions before product runtime actions. | 7.2, 8.1, 8.3 |
| Emergency permission revocation is not implemented | P0 | Support kill switch and permission revocation for malicious or broken products. | 7.3 |

## Audit And Operations Gaps

| Gap | Severity | Required V3 work | Target phase |
| --- | --- | --- | --- |
| Marketplace audit action taxonomy does not exist | P0 | Reserve action names for submission, review, install, update, rollback, purchase, payout, report abuse, takedown, and kill switch. | 1.1, 4.2, 6.1 |
| Platform-level Marketplace audit view is partial | P1 | Phase 4 adds review event logs and audit records; later analytics must aggregate creators, listings, decisions, reports, and blocked packages. | 4.1, 11.2 |
| Marketplace incident runbook does not exist | P1 | Phase 4 adds emergency block primitives; later documentation must define support flow for malicious product, broken install, refund, dispute, and emergency block. | 15.1 |

## Go/No-Go For Phase 1.1

Go for phase 1.1 domain modeling.

Reasons:

- Organization ownership model is clear.
- Billing boundary between subscriptions and Marketplace entitlements is clear.
- RBAC extension points are clear.
- Audit storage and event strategy are clear.
- RLS requirements for tenant-owned Marketplace tables are clear.

No-go conditions for later implementation:

- Do not implement install APIs before installation and permission snapshot tables exist.
- Do not implement paid products before purchase and entitlement tables exist.
- Do not enable backend extension runtime before sandbox and kill-switch controls exist.
- Do not publish unreviewed products in the catalog.