---
okf_document_id: "security-role-organization-author"
title: "Organization Author Role"
project: "ZinharCMS"
category: "security-role"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "partially_verified"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
role_id: "author"
role_name: "Author"
role_scope: "organization"
assignment_type: "organization membership enum and default later-registration mapping"
implementation_status: "verified"
primary_sources:
  - "backend/src/services/rbac.rs"
  - "backend/src/routes/auth.rs"
  - "backend/src/routes/content.rs"
  - "backend/src/routes/pages.rs"
related_documents:
  - "../roles-and-permissions-catalog.md"
  - "../permissions/content-workflow-and-comments.md"
  - "../permissions/pages-media-components-and-webhooks.md"
related_diagrams:
  - "../diagrams/rbac-model.mmd"
  - "../diagrams/tenant-access-control.mmd"
---

# Organization Author Role

## Identity and Scope

Organization `author` is the standard tenant creation role and the default-organization mapping for later public registrations.

## Effective Permissions

Author can write entries, pages, and media and can read/write comments. Author cannot manage content types, publish/review, manage pages/components/comments, manage webhooks, administer the organization, manage billing, or control Marketplace installations.

## Ownership Semantics

Entry/page author fields record attribution, but current organization author permissions are not a universal update-own-only ACL. Role and lifecycle checks are authoritative. This distinction is `RESOURCE_OWNERSHIP_UNVERIFIED ROU-01` where a product rule is not explicit.

## Restrictions and Tests

Tenant membership, RLS, resource existence, workflow state, and quotas still apply. The selected RBAC matrix checks entry-writing allowance and publisher denial; no exhaustive endpoint matrix exists.
