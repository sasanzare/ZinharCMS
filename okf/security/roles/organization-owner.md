---
okf_document_id: "security-role-organization-owner"
title: "Organization Owner Role"
project: "ZinharCMS"
category: "security-role"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "partially_verified"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
role_id: "owner"
role_name: "Owner"
role_scope: "organization"
assignment_type: "organization membership enum and organization ownership lifecycle"
implementation_status: "verified"
primary_sources:
  - "backend/src/services/rbac.rs"
  - "backend/src/routes/organizations.rs"
  - "backend/migrations/0008_v2_phase_one_organizations.sql"
related_documents:
  - "../roles-and-permissions-catalog.md"
  - "../tenant-access-control.md"
  - "../permissions/organization-administration.md"
related_diagrams:
  - "../diagrams/rbac-model.mmd"
  - "../diagrams/tenant-access-control.mmd"
---

# Organization Owner Role

## Identity and Scope

`owner` is the organization RBAC override. `require_org_any` accepts it for every organization capability helper.

## Assignment

Default bootstrap maps global super admin to owner. Organization creation and ownership transfer establish owner state; membership and the organization `owner_id` participate in lifecycle constraints.

## Effective Permissions

Owner can perform every documented organization capability: administration, content/workflow, pages/media/components/webhooks, comments, billing, Marketplace installation/permission approval, and organization kill-switch management.

## Restrictions

Owner is scoped to one organization. It grants no global Marketplace review, global beta administration, built-in plugin management, global kill switch, or automatic RLS bypass.

## Enforcement and Tests

Tenant middleware must first load an active owner membership. The RBAC matrix confirms owner passes selected capabilities. Ownership-transfer/leave/remove handlers add lifecycle constraints beyond the generic override.
