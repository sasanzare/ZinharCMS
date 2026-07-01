# V3 V2 Marketplace Readiness Audit

This audit records how the V2 SaaS foundation can be reused by the V3 Marketplace. It focuses on organizations, billing, RBAC, and audit logs because those systems define who owns a Marketplace installation, who can pay, who can approve permissions, and how sensitive Marketplace events are traced.

## Summary

V2 is a suitable foundation for V3 Marketplace work. The existing systems are strong enough to start domain modeling, but V3 must add Marketplace-specific entities and permissions instead of overloading V2 content, billing, or plugin primitives.

| Area | V2 readiness | V3 decision |
| --- | --- | --- |
| Organizations | Ready | Marketplace installations are organization-owned. |
| Billing | Partially ready | Organization subscription billing exists; Marketplace purchases and creator payouts need new entities. |
| RBAC | Partially ready | Organization roles exist; Marketplace install and purchase permissions need explicit checks. |
| Audit logs | Ready | Marketplace actions must use the existing audit service with a new action taxonomy. |
| RLS | Ready pattern | Every tenant-owned Marketplace table must use V2 RLS helpers and policies. |

## Organizations Audit

V2 provides:

- `X-Organization-Id` tenant selection.
- `TenantContext` with organization id, slug, name, role, and user id.
- Active organization membership enforcement before tenant routes run.
- Organization membership roles: owner, admin, editor, author, viewer, and billing_manager.
- Organization management APIs for members, invitations, workspace URL, domains, rate limits, audit logs, email deliveries, alerts, leave, and ownership transfer.
- Default organization membership for new and seeded users.

V3 Marketplace usage:

- `marketplace_installations` must be owned by `organization_id`.
- Catalog listings are global, but install state is tenant-owned.
- Reviews, purchases, entitlement records, and install permissions must reference the active organization when the action affects a customer workspace.
- Creator profiles can be user-owned or platform-owned, but installed products must never rely on creator organization context.

Readiness decision:

- Organization ownership is ready.
- V3 must not create Marketplace installs without an active organization context.
- V3 must not use global user role alone for organization install or purchase actions.

## Billing Audit

V2 provides:

- Plan records with limits and optional Stripe price ids.
- One active organization subscription record.
- Manual plan changes for local and testing setups.
- Stripe checkout, customer portal, webhook handling, idempotency, and provider-event ordering for organization subscriptions.
- Rebuildable usage counters for members, content records, media bytes, and API requests.
- Billing audit events for subscription changes, checkout creation, portal creation, and usage rebuilds.

V3 Marketplace usage:

- Organization subscription billing can gate Marketplace availability by plan.
- Marketplace product purchases must not be stored as organization subscription plan changes.
- Paid Marketplace products need purchase, entitlement, refund, commission, ledger, and payout records.
- Stripe Connect or another payout provider belongs to later V3 payment phases.
- Free products can be installed before paid products are enabled.

Readiness decision:

- V2 billing is ready as a customer billing foundation.
- V3 creator payments need separate Marketplace payment tables and webhook handling.
- V3 must distinguish organization subscription entitlement from Marketplace product entitlement.

## RBAC Audit

V2 provides:

- Global roles for platform administration.
- Organization role checks through `require_org_any`.
- Existing organization permissions for content, entries, media, pages, component registry, webhooks, billing, workflow, and comments.
- Billing manager support for billing actions.
- Owner bypass inside organization checks through `require_org_any`.

V3 Marketplace usage:

- Browsing the catalog can be available to any active organization member.
- Installing free products should require owner or admin by default.
- Buying paid products should require owner, admin, or billing_manager.
- Approving requested permissions should require owner or admin.
- Creator submission management is user/creator-scoped and should not depend on organization membership unless submitting on behalf of an organization.
- Marketplace moderation is platform-scoped and should require global admin or super admin.

Readiness decision:

- RBAC helper patterns are ready.
- V3 must add Marketplace-specific role checks instead of reusing content or billing checks directly.
- V3 must store permission approval snapshots at install time.

## Audit Log Audit

V2 provides:

- `audit::record` for tenant-scoped events.
- `audit::record_for_organization` for organization events outside a normal tenant request.
- `audit::record_in_transaction` for transactional writes.
- Organization audit log listing with actor email, action, entity type, entity id, metadata, and timestamp.
- Existing sensitive action coverage for organization, billing, content, page, media, membership, and beta workflows.

V3 Marketplace usage:

- Every submission, review decision, install, disable, uninstall, update, rollback, purchase, refund, payout, report abuse, takedown, and kill switch event must be audited.
- Install and purchase audit records must be organization-scoped.
- Creator and moderation audit records may need platform-level visibility in later phases.
- Metadata must include listing id, version id, package checksum, permission set, decision, and reason where applicable.

Readiness decision:

- Audit storage and tenant-scoped recording are ready.
- V3 must define a Marketplace action taxonomy before API implementation.
- V3 may need a platform audit view for creator and reviewer workflows.

## RLS And Tenant Isolation Audit

V2 provides:

- Tenant context helpers that set `zinhar.organization_id` and active user context.
- Forced RLS coverage for tenant-owned V2 tables.
- Hardening tests for tenant isolation and policy helpers.

V3 Marketplace usage:

- Tenant-owned Marketplace tables must include `organization_id` and forced RLS.
- Global Marketplace tables such as listing and package metadata must not expose unapproved or suspended records through tenant catalog APIs.
- Creator-owned tables need a separate user/creator access model, not only organization RLS.

Readiness decision:

- RLS pattern is ready for organization-owned install, purchase, review, and entitlement tables.
- V3 needs separate policy design for global listing visibility and creator-owned submission data.

## Critical Dependency Status

No critical V2 dependency remains ambiguous for starting V3 phase 1.1. The next phase can design Marketplace domain entities using these fixed decisions:

- Customer install state is organization-owned.
- Creator ownership is user/creator-scoped.
- Paid Marketplace entitlement is separate from organization subscription state.
- Marketplace permissions are approved at install time.
- All sensitive Marketplace changes are audited.
- RLS is required for every tenant-owned Marketplace table.