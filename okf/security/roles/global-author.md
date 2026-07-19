---
okf_document_id: "security-role-global-author"
title: "Global Author Role"
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
role_scope: "global"
assignment_type: "global database role assignment and default self-registration role"
implementation_status: "verified"
primary_sources:
  - "backend/src/services/rbac.rs"
  - "backend/src/routes/auth.rs"
  - "backend/migrations/0003_phase_one_core.sql"
related_documents:
  - "../roles-and-permissions-catalog.md"
  - "../permissions/content-workflow-and-comments.md"
  - "../permissions/pages-media-components-and-webhooks.md"
related_diagrams:
  - "../diagrams/rbac-model.mmd"
---

# Global Author Role

## Identity and Scope

`author` is the default global role for self-registration after the first user.

## Assignment

Registration inserts a `user_roles` assignment and maps the role to organization `author` in the default organization. Later organization memberships remain independent.

## Effective Permissions

Named global helpers permit entry, page, and media writing; comment reading/writing; and built-in plugin reading. They do not permit publishing/review, comment management, content-type/component/webhook/plugin management, or platform administration.

## Current Surface and Restrictions

Current tenant CMS routes use the organization role, not this global role. Global author cannot bypass membership, tenant RBAC, ownership, or RLS.

## Uncertainty

Public registration as the default assignment mechanism is verified, but account approval/verification intent is `AUTHENTICATION_FLOW_UNCLEAR AFU-01`. Stored `roles.permissions` is not effective runtime policy: `RBAC_MAPPING_UNCLEAR RMU-01`.
