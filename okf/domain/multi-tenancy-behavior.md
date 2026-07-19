---
okf_document_id: "domain-multi-tenancy-behavior"
title: "Multi-Tenancy Behavior"
project: "ZinharCMS"
category: "domain"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/routes/organizations.rs"
  - "backend/src/middleware/tenant.rs"
  - "backend/src/services/rls.rs"
  - "backend/src/services/quota.rs"
related_documents:
  - "membership-and-ownership.md"
  - "../security/tenant-access-control.md"
  - "../database/multi-tenancy.md"
related_diagrams:
  - "diagrams/tenant-membership-workflow.mmd"
---

# Multi-Tenancy Behavior

This document separates product behavior from technical isolation. See [Tenant Access Control](../security/tenant-access-control.md), [Database Multi-Tenancy](../database/multi-tenancy.md), and the [tenant membership diagram](diagrams/tenant-membership-workflow.mmd).

## Tenant Creation and Initialization

- Any authenticated user can create an organization with a normalized unique name/slug.
- Organization, active owner membership, and default free/manual subscription are created in one transaction.
- An audit record is attempted after commit.
- No automatic default content types, pages, navigation, settings, media, or Marketplace installations are created by this workflow.

## Tenant Selection and Switching

- The frontend persists an active organization and sends `X-Organization-Id` for tenant APIs.
- Middleware loads the organization and membership from the database; the client does not supply the effective organization role.
- Only active organizations and active memberships produce `TenantContext`.
- Switching tenants is a client selection followed by new tenant-scoped requests, not a token reissue.
- Preview may accept organization context in query parameters under its separate authenticated WebSocket contract.

## Membership and Roles

Membership arises from organization creation, bootstrap migration, invitation acceptance, or an existing row reactivated by acceptance. Roles are `owner`, `admin`, `editor`, `author`, `viewer`, and `billing_manager`. Role assignment, removal, leave, invitation, and ownership rules are detailed in [Membership and Ownership](membership-and-ownership.md).

## Tenant-Scoped Resources and Defaults

Content, pages, versions, tenant components, media, comments, settings/navigation, webhooks, subscriptions/usage, operations, beta records, and tenant Marketplace records carry organization scope. System component rows and global catalogs are explicit exceptions.

Tenant defaults include:

- an organization status of `active`;
- an owner membership of `active`;
- a free/manual subscription when the configured default plan is available;
- default organization rate-limit row on first load/update;
- no typed business-settings inheritance model beyond stored JSON/default columns.

## Quotas and Limits

- Member plus pending invitation capacity is checked during invitation creation; acceptance rechecks current member capacity.
- Content entry/page creation checks shared content capacity.
- Media upload checks byte capacity.
- Tenant middleware records API request usage and applies Redis-backed organization/user rate limits.
- A limit of `-1` means unlimited in plan calculations.
- `past_due` subscriptions remain eligible for current plan limits in current code; the business rationale is `BUSINESS_RULE_UNVERIFIED`.

## Cross-Tenant Administration

Selected global admin and provider workflows use explicit RLS bypass transactions for Marketplace, beta, billing callbacks, and operational aggregation. Bypass authorization is enforced by each caller, not by the low-level helper. A global role does not automatically create tenant membership or allow ordinary tenant routes.

## Tenant Status, Suspension, and Deletion

Tenant middleware enforces active organization and membership state. The database defines organization `suspended`/`deleted` and member `invited`/`suspended`, but complete application transitions into/out of those states were not found: `TENANT_BEHAVIOR_UNCLEAR`.

A hard-deleted organization would cascade a wide tenant graph, but no verified public tenant-deletion workflow, restoration path, or retention policy was found.

## Product Behavior Versus Isolation

| Concern | Product rule | Technical control | Confidence |
| --- | --- | --- | --- |
| Workspace access | User must have active membership | Tenant middleware query | High |
| Role | Stored organization role controls capabilities | RBAC helper calls | High, distributed |
| Row isolation | Resource belongs to selected organization | Tenant filters, session context, forced RLS | High in source; applied behavior unverified |
| Capacity | Current plan limits business operations | Quota service and usage counters | High |
| Ownership | Owner manages owner-only operations | Organization handlers | High except concurrency |
| Public delivery tenant | Current code resolves active `default` organization | Delivery lookup | High code, intended policy unclear |

## Main Uncertainties

- `TENANT_BEHAVIOR_UNCLEAR`: intended custom-domain/public tenant resolution is incomplete.
- `INVARIANT_UNVERIFIED`: same-tenant parent/child coherence is not universally enforced by composite FKs.
- `OWNERSHIP_RULE_UNCLEAR`: last-owner count check has no verified concurrent lock.
- `UNKNOWN`: live database roles, applied RLS catalog, and cross-tenant attack tests were not executed.

