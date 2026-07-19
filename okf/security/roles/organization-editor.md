---
okf_document_id: "security-role-organization-editor"
title: "Organization Editor Role"
project: "ZinharCMS"
category: "security-role"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "partially_verified"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
role_id: "editor"
role_name: "Editor"
role_scope: "organization"
assignment_type: "organization membership enum"
implementation_status: "verified"
primary_sources:
  - "backend/src/services/rbac.rs"
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

# Organization Editor Role

## Identity and Scope

Organization `editor` is an editorial tenant role.

## Assignment

It can be assigned through organization membership/invitation flows and is mapped from global editor for default-organization bootstrap/migration behavior.

## Effective Permissions

Editor can manage content types, write/publish/review entries, write/publish/manage pages, write media, manage components, read/write/manage comments, and manage selected beta feedback/blocker operations. It cannot manage organization webhooks, billing, organization membership/configuration, or Marketplace installation/kill-switch operations.

## Restrictions

Resource lifecycle and workflow preconditions still apply. Editor has no global editor privileges merely because the role name matches.

## Tests

The RBAC matrix verifies selected editor capability results. Route-specific lifecycle behavior has separate unit/static coverage but no complete role-action integration matrix.
