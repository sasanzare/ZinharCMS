---
okf_document_id: "api-authorization"
title: "API Authorization"
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
  - "backend/src/services/rbac.rs"
  - "backend/src/routes"
  - "backend/src/middleware/tenant.rs"
related_documents:
  - "api/authentication.md"
  - "api/tenant-context.md"
  - "backend/modules/tenant-authorization.md"
  - "database/entities/identity-and-global-rbac.md"
related_diagrams:
  - "api/diagrams/authorization-flow.mmd"
uncertainty_markers:
  - "AUTHORIZATION_REQUIREMENT_UNCLEAR AZU-01"
---

# API Authorization

## Role Domains

ZinharCMS has two related role domains:

- Global roles: `super_admin`, `admin`, `editor`, `author`, `viewer`.
- Organization roles: `owner`, `admin`, `editor`, `author`, `viewer`, `billing_manager`.

The tenant middleware verifies active membership but does not authorize every operation. Route handlers call RBAC helpers or perform ownership/global-admin checks.

## Organization Capability Matrix

| Capability | Allowed organization roles |
| --- | --- |
| Content type management | owner, admin, editor |
| Entry writing | owner, admin, editor, author |
| Entry publishing/review | owner, admin, editor |
| Media writing | owner, admin, editor, author |
| Page writing | owner, admin, editor, author |
| Page publishing/management | owner, admin, editor |
| Component management | owner, admin, editor |
| CMS webhook management | owner, admin |
| Billing management | owner, admin, billing_manager |
| Marketplace install, permission approval, organization kill switch | owner, admin |
| Workflow review | owner, admin, editor |
| Comment read | owner, admin, editor, author, viewer |
| Comment write | owner, admin, editor, author |
| Comment manage | owner, admin, editor |

Some list/detail operations require only active membership and do not call a narrower capability helper. Marketplace creator, review, verification, finance, global kill-switch, and beta participant operations add global role or resource-ownership rules in their handlers.

## Decision Sequence

1. Validate identity.
2. For tenant routes, validate active organization membership and construct `TenantContext`.
3. Apply route-level global or organization role checks.
4. Apply creator/resource ownership, workflow state, permission approval, or provider-state checks.
5. Run tenant-scoped persistence under RLS where the route uses an RLS connection.

RLS is defense in depth and data scoping; it is not a substitute for capability checks.

## Maintenance Rule

Do not infer authorization from HTTP method, frontend visibility, DTO extractors, or OpenAPI. Review router placement, RBAC calls, ownership queries, and RLS context together. Use `AUTHORIZATION_REQUIREMENT_UNCLEAR AZU-01` if those sources do not establish a coherent decision. Detailed threat analysis and policy hardening belong in Phase 7.

## Phase 7 Detail

[Authorization Architecture](../security/authorization-architecture.md), [RBAC Model](../security/rbac-model.md), [Roles and Permissions Catalog](../security/roles-and-permissions-catalog.md), [Resource Ownership](../security/resource-ownership.md), and [Administrative Access](../security/administrative-access.md) now map the full decision layers. Stored `roles.permissions` arrays are not read by inspected runtime RBAC (`RBAC_MAPPING_UNCLEAR RMU-01`).

## Plugin and Marketplace Authorization

Global plugin-manager RBAC protects built-in registry actions. Tenant Marketplace actions combine active membership, organization RBAC, ownership/RLS, lifecycle gates, exact permission approval, and kill switches. Built-in callback execution itself has no per-plugin capability check. See [Plugin Permissions](../extensibility/plugin-permissions.md).
