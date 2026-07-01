# V3 Marketplace V2 Dependency Matrix

This matrix turns the V2 audit into concrete dependencies for V3 Marketplace planning.

| Dependency | Existing V2 asset | V3 use | Status | Next phase impact |
| --- | --- | --- | --- | --- |
| Active organization context | `TenantContext`, `X-Organization-Id`, active membership lookup | Tenant-owned install, entitlement, review, and purchase actions | Ready | Phase 1.1 can model organization-owned Marketplace tables. |
| Organization membership | `organization_members`, invitations, ownership transfer | Decide who can install, buy, approve permissions, and manage installed products | Ready with gaps | Phase 1.1 must define Marketplace role checks. |
| Organization billing | `plans`, `organization_subscriptions`, usage counters | Gate Marketplace availability by customer plan | Ready | Phase 9 can extend billing without changing V2 subscriptions. |
| Stripe lifecycle | checkout, portal, webhook idempotency, provider event ordering | Process paid Marketplace products and payout events later | Partial | Phase 9 needs separate purchase and payout flows. |
| RBAC helpers | `require_org_any`, billing/content/page manager checks | Create Marketplace-specific access helpers | Pattern ready | Phase 7.1 must define install, purchase, permission, creator, and moderation checks. |
| Audit log service | `audit::record`, `record_for_organization`, `record_in_transaction` | Trace submission, review, install, purchase, payout, and takedown | Ready | Phase 1.1 should reserve audit action names. |
| RLS helpers | tenant and organization connections, forced RLS pattern | Isolate tenant-owned Marketplace rows | Ready pattern | Phase 1.3 migrations must add RLS for install/purchase/review rows. |
| Package storage | upload/media paths exist, but no package registry | Store Marketplace artifacts, manifests, and checksums | Gap | Phase 1.4 must add package storage policy. |
| Plugin runtime | built-in plugin registry exists, no external runtime isolation | Execute or activate installed products | Gap | Phase 7 and 8 must add permission and sandbox boundaries. |
| Creator identity | users exist, no creator profile or verification state | Creator portal, submission ownership, payout eligibility | Gap | Phase 2.1 and 2.2 must add creator profile and verification. |
| Marketplace payment entitlement | organization subscription exists, no product entitlement | Paid product purchase and access | Gap | Phase 9.2 must add purchase and entitlement model. |

## Fixed Decisions For Phase 1.1

- Marketplace listing metadata is global.
- Marketplace installation state is tenant-owned and requires `organization_id`.
- Creator profile is user/creator-scoped, not organization-owned by default.
- Marketplace purchase entitlement is separate from organization subscription.
- Marketplace permission approval is captured per installation and per version.
- Marketplace audit events reuse V2 audit storage but need new action names.
- Marketplace package artifacts require checksum verification before install.