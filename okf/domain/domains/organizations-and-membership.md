---
okf_document_id: "domain-organizations-membership"
title: "Organizations and Membership Domain"
project: "ZinharCMS"
category: "domain-detail"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
domain_id: "DOM-TENANT"
domain_name: "Organizations and Membership"
domain_status: "IMPLEMENTED"
boundary_status: "EXPLICIT"
primary_sources:
  - "backend/src/routes/organizations.rs"
  - "backend/src/middleware/tenant.rs"
  - "backend/src/services/rls.rs"
  - "backend/src/services/rbac.rs"
  - "backend/migrations/0008_v2_phase_one_organizations.sql"
related_documents:
  - "../domain-catalog.md"
  - "../multi-tenancy-behavior.md"
  - "../membership-and-ownership.md"
related_diagrams:
  - "../diagrams/tenant-membership-workflow.mmd"
---

# Organizations and Membership Domain

## Domain Identity

- Domain ID: `DOM-TENANT`
- Terminology: organization, tenant, workspace, membership, invitation, owner, organization role, `TenantContext`.
- Implementation: `IMPLEMENTED`; boundary `EXPLICIT`; confidence High.

## Responsibility

- Verified: tenant creation, active membership, invitations, role assignment, leave/removal, ownership transfer, organization settings/domain/rate-limit administration, and workspace access.
- Inferred: organization is the aggregate root for tenant-owned product data.
- Shared: quotas belong to Billing; isolation to RLS/security; email/audit to operations.
- Unclear: suspension/deletion transitions, domain verification-to-routing, and full tenant teardown/restoration.

## Core Entities

Organizations, organization members, invitations, domains, rate limits, subscriptions, and tenant-owned foreign-key graphs.

## Core Services

Organization routes, tenant middleware, RLS, RBAC, quota, audit, email, and rate-limit services.

## API Surface

Organization list/create/current/workspace, member/invitation/ownership, settings, domain, rate-limit, audit, and delivery operations. See [Organizations Endpoints](../../api/endpoints/organizations-membership-and-invitations.md).

## Frontend Surface

`OrganizationPage`, `WorkspaceRedirectPage`, AppShell organization selector, dashboard/billing/beta organization context.

## Actors

Authenticated user, organization owner/admin/editor/author/viewer/billing manager, invitation recipient, and selected global administration callers.

## Business Rules

`BR-TENANT-001` through `BR-TENANT-007`.

## Invariants

Unique membership, unique pending invite, active request context, and application last-owner guard. See `INV-TENANT-001` through `INV-TENANT-005`.

## State and Lifecycle

Organization, member, and invitation enums are listed in [State Transitions](../state-transitions.md). Invitation transitions are explicit; organization/member status graphs are incomplete.

## Access Rules

Tenant middleware loads role; owner/admin handlers apply stronger rules; forced RLS protects tenant rows. Owner is an organization override, not a global grant. See [Tenant Access Control](../../security/tenant-access-control.md).

## Validation Rules

Name/slug/settings/domain/email/role/rate-limit validation is distributed between route helpers and constraints. Capacity checks use the current plan.

## Workflows

[Organization Provisioning](../workflows/organization-provisioning.md), [Invitation and Membership](../workflows/tenant-invitation-and-membership.md), and [Ownership Transfer](../workflows/organization-ownership-transfer.md).

## Side Effects

Tenant/member/invitation/subscription writes, invitation email, audit records, and later tenant middleware quota/rate-limit use. Core transactions and audit/email are not always atomic.

## Tests

RBAC/quota helpers have unit tests. Full provisioning, invitation, ownership, last-owner concurrency, active-status, and cross-tenant integration tests were not found.

## Risks and Unknowns

Last-owner race, same-tenant child coherence, public tenant routing, domain verification, suspension/deletion lifecycle, and post-commit audit/email behavior.

Return to the [Domain Catalog](../domain-catalog.md).

