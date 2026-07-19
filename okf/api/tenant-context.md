---
okf_document_id: "api-tenant-context"
title: "API Tenant Context"
project: "ZinharCMS"
category: "api-security"
phase: 6
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "eed1e0dbdf6d873457d1165158b3c8fbfd6647e1"
last_verified_date: "2026-07-18"
primary_sources:
  - "backend/src/middleware/tenant.rs"
  - "backend/src/routes/mod.rs"
  - "backend/src/services/rls.rs"
  - "frontend/src/services/api.ts"
related_documents:
  - "api/authentication.md"
  - "api/authorization.md"
  - "database/multi-tenancy.md"
  - "backend/modules/tenant-authorization.md"
related_diagrams:
  - "api/diagrams/api-request-lifecycle.mmd"
uncertainty_markers:
  - "TENANT_CONTEXT_UNCLEAR TCU-01"
---

# API Tenant Context

## Selection Contract

Normal tenant-protected HTTP requests require:

```http
X-Organization-Id: <organization-uuid>
```

The middleware validates the header as a UUID and verifies that the authenticated user has an active membership. It then inserts `TenantContext`, including the organization, user, and organization role. Browser preview WebSockets may use the `organization_id` query parameter instead.

## Middleware Responsibilities

The tenant layer performs authentication, membership resolution, organization rate limiting, and quota checks. Billing paths are excluded from the general quota gate so an over-quota tenant can still inspect or change billing. The layer then makes `Claims` and `TenantContext` available to handlers.

## Persistence Propagation

Tenant-aware handlers normally obtain a tenant-scoped PostgreSQL connection or transaction from `services::rls`. That helper establishes database session context used by row-level security. Some operations deliberately use pool-level or global queries for cross-tenant administration, public delivery organization resolution, authentication, or provider callbacks; those paths require separate scrutiny.

## Frontend Ownership

The frontend stores the selected organization ID in local storage. Its shared request helper attaches `X-Organization-Id` to authenticated calls when an organization has been selected. Organization bootstrap and invitation-acceptance endpoints are authenticated but not tenant-protected, allowing a user to establish membership before selecting a tenant.

## Public Delivery Distinction

Public `/api/v1/*` delivery routes do not accept tenant context from the caller. They resolve the repository-defined default public organization internally. This is a product-specific delivery rule, not header-based multitenancy.

## Uncertainty Rule

Use `TENANT_CONTEXT_UNCLEAR TCU-01` when a route accesses organization-owned data without a demonstrable middleware, ownership, or explicit cross-tenant administration path. No registered endpoint had an unresolved router zone in this snapshot, but route-level query and RLS behavior must still be reviewed during changes.

## Phase 7 Detail

[Tenant Access Control](../security/tenant-access-control.md) and its [diagram](../security/diagrams/tenant-access-control.mmd) show membership loading, organization/user rate limiting, quota, role checks, tenant SQL context, forced RLS, and explicit bypass. Repository evidence is verified; live deployed isolation remains `TENANT_ACCESS_UNVERIFIED TAV-01`.
