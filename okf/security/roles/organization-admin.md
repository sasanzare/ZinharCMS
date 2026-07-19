---
okf_document_id: "security-role-organization-admin"
title: "Organization Admin Role"
project: "ZinharCMS"
category: "security-role"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "partially_verified"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
role_id: "admin"
role_name: "Admin"
role_scope: "organization"
assignment_type: "organization membership enum"
implementation_status: "verified"
primary_sources:
  - "backend/src/services/rbac.rs"
  - "backend/src/routes/organizations.rs"
  - "backend/migrations/0008_v2_phase_one_organizations.sql"
related_documents:
  - "../roles-and-permissions-catalog.md"
  - "../permissions/organization-administration.md"
  - "../permissions/marketplace-management.md"
related_diagrams:
  - "../diagrams/rbac-model.mmd"
  - "../diagrams/tenant-access-control.mmd"
---

# Organization Admin Role

## Identity and Scope

Organization `admin` is a privileged tenant role and is distinct from global `admin`.

## Assignment

It can be assigned through invitation/member-role flows and is the default-organization mapping for global admin. Tenant middleware loads it from active membership.

## Effective Permissions

Admin is explicitly allowed by every named organization helper: organization administration, content/workflow, page/media/component/webhook management, comments, billing, Marketplace installation/permission approval, and organization kill-switch management.

## Restrictions

Unlike owner, admin has no generic override; it passes because helpers list it. Organization ownership transfer and owner lifecycle rules remain separate. It grants no global administrative permissions.

## Frontend and Tests

Multiple pages expose admin controls from local membership state. Backend checks remain authoritative. The RBAC unit matrix covers selected admin capabilities, but no exhaustive endpoint matrix exists.
