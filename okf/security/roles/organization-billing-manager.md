---
okf_document_id: "security-role-organization-billing-manager"
title: "Organization Billing Manager Role"
project: "ZinharCMS"
category: "security-role"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "partially_verified"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
role_id: "billing_manager"
role_name: "Billing Manager"
role_scope: "organization"
assignment_type: "organization membership enum"
implementation_status: "verified"
primary_sources:
  - "backend/src/services/rbac.rs"
  - "backend/src/routes/billing.rs"
  - "backend/migrations/0008_v2_phase_one_organizations.sql"
related_documents:
  - "../roles-and-permissions-catalog.md"
  - "../permissions/billing-and-saas-operations.md"
  - "../tenant-access-control.md"
related_diagrams:
  - "../diagrams/rbac-model.mmd"
  - "../diagrams/tenant-access-control.mmd"
---

# Organization Billing Manager Role

## Identity and Scope

`billing_manager` is a specialized organization role for billing operations.

## Assignment

It is an organization membership enum value assignable through invitation/member-role administration. It has no equivalent global role and is not created by default registration mapping.

## Effective Permissions

The billing helper allows owner, admin, and billing manager for subscription changes, checkout/customer portal, usage access, and usage rebuild paths that call it. The specialized role is not included in content, pages, media, comments, webhooks, organization administration, or Marketplace installation helpers.

## Restrictions

It still requires active membership, tenant context, plan/provider preconditions, quotas, and RLS. Whether every billing read should require this helper is an endpoint-contract question; consult the billing route family.

## Tests

The RBAC matrix verifies the named billing capability. Provider integration and entitlement tests are separate; no full billing-manager end-to-end suite was found.
