---
okf_document_id: "security-permission-content-workflow-comments"
title: "Content, Workflow, and Comments Permission Group"
project: "ZinharCMS"
category: "security-permission-group"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "partially_verified"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
permission_group_id: "content-workflow-comments"
permission_group_name: "Content, Workflow, and Comments"
resource_domain: "content types, content entries, workflow, and editorial comments"
permission_scope: "organization with legacy global helpers"
implementation_status: "verified"
primary_sources:
  - "backend/src/services/rbac.rs"
  - "backend/src/routes/content.rs"
  - "backend/src/routes/comments.rs"
  - "backend/src/services/workflow.rs"
related_documents:
  - "../roles-and-permissions-catalog.md"
  - "../resource-ownership.md"
  - "../../api/endpoints/content-entries-and-workflow.md"
  - "../../api/endpoints/editorial-comments.md"
related_diagrams:
  - "../diagrams/authorization-decision-flow.mmd"
  - "../diagrams/rbac-model.mmd"
---

# Content, Workflow, and Comments Permission Group

## Included Permissions

Named capabilities are content-type management, entry writing, entry publishing, workflow review, comment reading, comment writing, and comment management. Role checks are capability names, not stored permission strings.

## Resource Domain and Operations

Operations include content-type CRUD; entry read/create/update/delete and workflow transitions; and comment list/create/get/resolve/unresolve/delete.

## Scope and Roles

- Content types: owner/admin/editor.
- Entry write: owner/admin/editor/author.
- Entry publish/review: owner/admin/editor.
- Comment read: owner/admin/editor/author/viewer.
- Comment write: owner/admin/editor/author.
- Comment manage: owner/admin/editor.

## Enforcement and Frontend

Backend route handlers call named organization helpers and workflow validators after tenant middleware. Frontend buttons and pages may expose actions optimistically but remain `FRONTEND_ONLY_SECURITY_CHECK FOSC-01`.

## API and Database

See the content/workflow and comments endpoint families. Content types, entries, and comments are organization-scoped and forced-RLS protected. `author_id` is not a universal ownership ACL.

## Tests and Unclear Semantics

RBAC selected matrix, workflow tests, entry validation, sanitizer tests, and static contracts provide partial coverage. `RESOURCE_OWNERSHIP_UNVERIFIED ROU-01` applies to author-versus-role behavior; `PERMISSION_SEMANTICS_UNCLEAR PSU-01` applies to legacy stored permission strings.
