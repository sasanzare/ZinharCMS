---
okf_document_id: "security-permission-organization-administration"
title: "Organization Administration Permission Group"
project: "ZinharCMS"
category: "security-permission-group"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "partially_verified"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
permission_group_id: "organization-administration"
permission_group_name: "Organization Administration"
resource_domain: "organizations, memberships, invitations, domains, limits, audit, and alerts"
permission_scope: "organization"
implementation_status: "verified"
primary_sources:
  - "backend/src/routes/organizations.rs"
  - "backend/src/services/rbac.rs"
  - "backend/src/middleware/tenant.rs"
related_documents:
  - "../roles-and-permissions-catalog.md"
  - "../tenant-access-control.md"
  - "../roles/organization-owner.md"
  - "../roles/organization-admin.md"
related_diagrams:
  - "../diagrams/authorization-decision-flow.mmd"
  - "../diagrams/tenant-access-control.mmd"
---

# Organization Administration Permission Group

## Included Permissions and Operations

Current organization update, member/invitation listing and mutation, workspace access, domain management, rate-limit configuration, audit/email/alert inspection, leave, and ownership transfer. Organization create/list and invitation acceptance are authenticated non-tenant bootstrap paths with their own subject checks.

## Scope and Roles

Most administrative tenant operations use an owner/admin policy. Owner override is generic; admin is explicitly allowed. Some self-service membership and workspace reads have subject-specific rules rather than the same admin gate.

## Backend Enforcement and API

`routes/organizations.rs` contains `require_org_admin`, role-assignment restrictions, owner lifecycle checks, invitation-token checks, and tenant-scoped queries. Tenant middleware supplies active membership. See [Organizations Endpoint Family](../../api/endpoints/organizations-membership-and-invitations.md).

## Frontend Checks

`OrganizationPage` conditionally renders management actions from local membership state. Those checks are `FRONTEND_ONLY_SECURITY_CHECK FOSC-01`.

## Database Implications

Operations affect organizations, memberships, invitations, domains, rate limits, audit logs, email deliveries, and alerts. Tenant tables use RLS; organization/member bootstrap queries require explicit scoping.

## Tests and Unclear Semantics

Selected RBAC and email/service tests exist. A complete role-assignment escalation matrix and live tenant test are absent: `AUTHORIZATION_ENFORCEMENT_UNCLEAR AEU-01`, `TENANT_ACCESS_UNVERIFIED TAV-01`.
