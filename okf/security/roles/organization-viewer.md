---
okf_document_id: "security-role-organization-viewer"
title: "Organization Viewer Role"
project: "ZinharCMS"
category: "security-role"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "partially_verified"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
role_id: "viewer"
role_name: "Viewer"
role_scope: "organization"
assignment_type: "organization membership enum"
implementation_status: "verified"
primary_sources:
  - "backend/src/services/rbac.rs"
  - "backend/src/routes/comments.rs"
  - "backend/migrations/0008_v2_phase_one_organizations.sql"
related_documents:
  - "../roles-and-permissions-catalog.md"
  - "../permissions/content-workflow-and-comments.md"
  - "../tenant-access-control.md"
related_diagrams:
  - "../diagrams/rbac-model.mmd"
  - "../diagrams/tenant-access-control.mmd"
---

# Organization Viewer Role

## Identity and Scope

Organization `viewer` is the least-privileged general tenant role.

## Effective Permissions

The named organization helpers explicitly allow comment reading. Read-only handlers without a capability check may also be reachable once tenant middleware establishes membership; endpoint-specific contracts must be consulted. Viewer is denied write, publish, management, billing, organization administration, and Marketplace control helpers.

## Important Semantics

There is no single `reader` helper applied to every tenant GET endpoint. Membership and RLS provide the base read boundary, while handler checks vary. `AUTHORIZATION_ENFORCEMENT_UNCLEAR AEU-01` applies to any assumption that viewer has a uniform read-only matrix.

## Tests

The organization RBAC matrix verifies selected denials and comment-reader allowance. No exhaustive read endpoint matrix was found.
