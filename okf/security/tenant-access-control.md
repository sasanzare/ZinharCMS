---
okf_document_id: "security-tenant-access-control"
title: "Tenant Access Control"
project: "ZinharCMS"
category: "security-tenancy"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "partially_verified"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/middleware/tenant.rs"
  - "backend/src/services/rls.rs"
  - "backend/migrations/0008_v2_phase_one_organizations.sql"
  - "backend/migrations/0009_v2_phase_three_rls.sql"
  - "backend/src/routes/mod.rs"
related_documents:
  - "authorization-architecture.md"
  - "resource-ownership.md"
  - "../api/tenant-context.md"
  - "../database/multi-tenancy.md"
related_diagrams:
  - "diagrams/tenant-access-control.mmd"
---

# Tenant Access Control

## Tenant Selection

Normal tenant requests provide `X-Organization-Id`; preview requests may provide `organization_id` in the query string. The middleware parses the UUID and loads the organization and membership using the authenticated user ID. Only active organizations with active memberships succeed.

The client cannot select its organization role directly. `TenantContext` contains the organization identity, slug, name, database-loaded role, and authenticated user ID.

## Request-Level Controls

After membership resolution, tenant middleware applies Redis-backed organization and organization-user request limits. It also records API quota usage except for billing paths. The endpoint handler then applies operation-specific RBAC.

## Database-Level Controls

Tenant-aware helpers set organization and user session variables before queries. Migrations enable and force RLS across tenant-owned CMS, billing, operations, beta, and Marketplace tables. Standard policies call `app_rls_tenant_matches(organization_id)`. Component registry reads also allow system components; writes do not allow system-component mutation through tenant context.

## Bypass

`begin_bypass_transaction` sets the explicit bypass variable and clears tenant/user context. Selected global administration, analytics, webhook/provider, and Marketplace operations call it. A global `super_admin` does not automatically activate bypass and cannot use a tenant route without active membership.

## Membership Lifecycle

Organization roles are assigned through bootstrap mapping, invitation acceptance, role updates, and ownership transfer. Membership and organization status are evaluated at the start of every tenant request, so suspension blocks new tenant contexts even when the access token remains otherwise valid.

## Verification Limits

`TENANT_ACCESS_UNVERIFIED TAV-01`: migration definitions and query helpers were inspected, but the applied database catalog, database login role privileges, live cross-tenant behavior, Redis isolation, and all handler query paths were not executed in this phase.

Tenant and parent foreign keys do not universally prove same-organization parent-child coherence; retain `TENANT_ISOLATION_UNVERIFIED TIV-01` from Phase 5.

## Phase 8 Tenant Lifecycle

[Multi-Tenancy Behavior](../domain/multi-tenancy-behavior.md) and the tenant workflows document organization provisioning, invitation acceptance, membership mutation, owner protection, and ownership transfer after the initial middleware decision. They preserve the distinction between authentication, active membership, organization role, resource ownership, and RLS. Public delivery tenant selection remains a separate unresolved routing boundary.

## Extensibility Scope

Built-in plugin enablement is global even though invocation receives organization context. Marketplace installations and derived adapter operations are organization-scoped and depend on tenant middleware/RBAC/RLS. Global and organization kill switches overlay runtime readiness. See [Tenant and Global Scope](../extensibility/tenant-and-global-scope.md).
