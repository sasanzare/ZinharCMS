---
okf_document_id: "security-role-global-viewer"
title: "Global Viewer Role"
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
role_scope: "global"
assignment_type: "global database role assignment"
implementation_status: "verified"
primary_sources:
  - "backend/src/services/rbac.rs"
  - "backend/src/routes/plugins.rs"
  - "backend/migrations/0002_seed_foundation_data.sql"
related_documents:
  - "../roles-and-permissions-catalog.md"
  - "../permissions/global-administration-and-plugins.md"
  - "../permissions/content-workflow-and-comments.md"
related_diagrams:
  - "../diagrams/rbac-model.mmd"
---

# Global Viewer Role

## Identity and Scope

`viewer` is the least-privileged named global role in the hard-coded priority order.

## Assignment

It is stored in `roles` and assigned through `user_roles`. Login/refresh choose it only after higher-priority recognized assignments.

## Effective Permissions

Named global helpers allow comment reading and built-in plugin reading. Other global write, publish, management, and platform administration helpers deny it.

## Current Surface and Restrictions

Tenant content readability is governed by organization membership and handler checks, not this global name. The role does not create tenant access, ownership, entitlement, or RLS bypass.

## Uncertainty

Legacy permission arrays list additional read concepts, but runtime helpers do not load them. This is `RBAC_MAPPING_UNCLEAR RMU-01`; no complete global viewer route test exists.
