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
## Phase 5 Update

Phase 5 resolves the public catalog visibility and catalog compatibility filtering gaps. The catalog API and admin UI now expose only approved, safe, install-eligible products for the active organization, with search, filters, and listing details that show permissions before install. Later phases still need install runtime APIs, permission approval enforcement, abuse reporting, and full incident runbooks.

## Phase 6 Update

Phase 6 resolves the tenant-aware install runtime, catalog install action, install-time compatibility and permission approval gates, enable/disable/soft-uninstall lifecycle, semver update, safe rollback, Marketplace owner/admin RBAC helpers, artifact re-verification, and organization-scoped lifecycle audit taxonomy. Purchase and paid entitlement remain open for Phase 9; Phase 7 now supplies the policy-only runtime permission boundary and kill-switch controls while concrete execution adapters remain later work.

## Phase 7 Update

Phase 7 resolves the permission catalog, allowlisted runtime policy boundary,
bounded sandbox host API decision, global/organization kill switches, runtime
status blocking, and kill-switch audit taxonomy. The backend still does not
execute uploaded package code; component/template adapters and concrete
integration hooks remain later runtime work.

## Phase 9 Update

Phase 9 resolves free/paid purchase records, organization entitlements, Stripe
one-time checkout completion, full-refund entitlement revocation, revenue split
ledger entries, and creator payout onboarding/verification. Subscription-style
Marketplace add-ons, partial refunds, and automated payout transfer execution
remain later work.

## Phase 8 Update

Phase 8 resolves the first host-owned adapter surfaces: installed Component Pack
definitions now feed the organization Page Builder registry, Design Templates
can be previewed and cloned with organization-owned media mappings, and active
Integration Plugins expose only the four public hook contracts. Adapter
authorization remains policy-only and never executes uploaded package code.

## Plugin Install Gaps

| Gap | Severity | Required V3 work | Target phase |
| --- | --- | --- | --- |
| Marketplace installation runtime API does not exist | Resolved | Phase 6 adds tenant-aware install, enable, disable, soft-uninstall, update, and rollback APIs. | 6.1, 6.2, 6.3 |
| Catalog install action is not implemented | Resolved | Phase 6 connects catalog versions to the organization-owned installation API. | 6.1, 6.2 |
| Install compatibility enforcement is incomplete | Resolved | Phase 6 re-evaluates the active plan, version/features, review state, and artifact integrity at mutation time. | 6.1 |
| Install rollback API is not implemented | Resolved | Phase 6 adds same-listing, safe, compatible, artifact-verified rollback. | 6.3 |
| Installed product lifecycle runtime events are not implemented | Resolved | Phase 6 implements install, enable, disable, uninstall, update, and rollback; the later runtime kill switch remains Phase 7.3 work. | 6.1, 6.2, 7.3 |
| Tenant-owned RLS policies for later Marketplace tables do not exist | Resolved for Phase 9 | Forced RLS protects installations, purchases, entitlements, and revenue ledger rows. | 9.2, 10.1 |

## Creator Payment Gaps

| Gap | Severity | Required V3 work | Target phase |
| --- | --- | --- | --- |
| Creator payout and advanced profile operations do not exist | Partially resolved | Phase 9 adds provider account onboarding and eligibility verification; transfer scheduling remains open. | 2.1, 2.2, 9.4 |
| Marketplace purchase table does not exist | Resolved | Phase 9 stores Marketplace purchases separately from organization subscriptions. | 9.2 |
| Marketplace entitlement table does not exist | Resolved | Phase 9 links active/revoked entitlement to organization, purchase, listing, and version. | 9.2 |
| Revenue split ledger does not exist | Resolved | Phase 9 records idempotent purchase and full-refund effects. | 9.3 |
| Payout provider is not connected | Partially resolved | Phase 9 adds Stripe Connect-style onboarding and verification state; transfer execution remains open. | 9.4 |
| Refund policy is not modeled | Partially resolved | Full refund revokes entitlement and reverses the ledger; partial refunds remain unsupported. | 9.2, 9.3 |
| Creator self-purchase prevention is not modeled | P1 | Detect and block creator-owned organization purchases where policy disallows them. | 9.2 |

## Permission Gaps

| Gap | Severity | Required V3 work | Target phase |
| --- | --- | --- | --- |
| Marketplace permission catalog does not exist | Resolved | Phase 7.1 adds a seeded permission catalog with category, risk, product-type, and runtime-operation metadata. | 7.1 |
| Install-time permission approval is not enforced by runtime | Resolved | Phase 6 requires an exact owner/admin-approved manifest permission snapshot before install and permission-changing updates. | 7.1, 6.1 |
| Marketplace RBAC helpers do not exist | Resolved for Phase 7 controls | Phase 6 owner/admin install helpers are extended with organization kill-switch management; global kill-switch mutations require global admin. | 7.1, 7.3 |
| Permission escalation on update is not modeled | Resolved | Phase 6 requires fresh exact approval whenever a target version changes the permission set. | 6.3, 7.1 |
| Runtime permission enforcement is not implemented | Resolved for policy boundary | Phase 7.2 denies non-allowlisted operations, undeclared entry points, unapproved permissions, unsafe paths, inactive installations, and oversized payloads; Phase 8 adapters re-check active installation and kill-switch state; no uploaded code executes. | 7.2, 8.1, 8.3 |
| Emergency permission revocation is not implemented | Resolved for global/org kill switch | Phase 7.3 adds global and organization kill switches, runtime blocking, lift behavior, status messages, and audit records. Fine-grained permission revocation remains future runtime work. | 7.3 |

## Audit And Operations Gaps

| Gap | Severity | Required V3 work | Target phase |
| --- | --- | --- | --- |
| Marketplace audit action taxonomy does not exist | Partially resolved | Phase 6 defines organization-scoped install, enable, disable, uninstall, update, and rollback actions; purchase, payout, abuse, and kill-switch actions remain later-phase work. | 1.1, 4.2, 6.1 |
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
