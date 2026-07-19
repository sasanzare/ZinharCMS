---
okf_document_id: "security-role-global-editor"
title: "Global Editor Role"
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
role_scope: "global"
assignment_type: "global database role assignment"
implementation_status: "verified"
primary_sources:
  - "backend/src/services/rbac.rs"
  - "backend/src/routes/plugins.rs"
  - "backend/migrations/0002_seed_foundation_data.sql"
related_documents:
  - "../roles-and-permissions-catalog.md"
  - "../permissions/content-workflow-and-comments.md"
  - "../permissions/pages-media-components-and-webhooks.md"
related_diagrams:
  - "../diagrams/rbac-model.mmd"
---

# Global Editor Role

## Identity and Scope

`editor` is a global editorial role and is separate from organization `editor`.

## Assignment

It is stored and assigned through `roles` and `user_roles`. Login/refresh select it after super admin and admin when a user has multiple global roles.

## Effective Permissions

Named global helpers permit entry/page writing and publishing, media writing, workflow review, comment reading/writing/management, page management, and built-in plugin reading. Content-type, component-registry, webhook, and plugin management helpers do not allow it.

## Current Surface Caveat

Current CMS routes primarily use organization helpers, so the global editorial helper matrix is implemented code but not proof of an equivalent registered global CMS route. `RBAC_MAPPING_UNCLEAR RMU-01` applies because legacy stored permission arrays are not evaluated.

## Restrictions and Tests

This role grants no automatic tenant membership or RLS bypass. No complete global-editor route test matrix was found.
