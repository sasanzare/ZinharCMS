---
okf_document_id: "security-permission-global-administration-plugins"
title: "Global Administration and Built-in Plugins Permission Group"
project: "ZinharCMS"
category: "security-permission-group"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "partially_verified"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
permission_group_id: "global-administration-plugins"
permission_group_name: "Global Administration and Built-in Plugins"
resource_domain: "platform operations and built-in plugins"
permission_scope: "global"
implementation_status: "verified"
primary_sources:
  - "backend/src/services/rbac.rs"
  - "backend/src/routes/plugins.rs"
  - "backend/src/routes/beta.rs"
  - "backend/src/routes/marketplace.rs"
  - "backend/src/routes/marketplace_runtime.rs"
related_documents:
  - "../roles-and-permissions-catalog.md"
  - "../administrative-access.md"
  - "../roles/global-super-admin.md"
  - "../roles/global-admin.md"
related_diagrams:
  - "../diagrams/rbac-model.mmd"
  - "../diagrams/authorization-decision-flow.mmd"
---

# Global Administration and Built-in Plugins Permission Group

## Included Permissions

Named operations include `plugin_reader`, `plugin_manager`, global beta participant/product administration, Marketplace review/moderation/abuse/analytics, payout verification, and global kill-switch control. Global CMS capability helpers also exist but are not the primary current tenant route boundary.

## Resource Domain and Operations

The domain covers platform-wide state, built-in plugin configuration, cross-organization Marketplace governance, and selected beta operations. Read/update/enable/disable plugin endpoints sit in the bearer-protected non-tenant router.

## Scope and Roles

All global roles can read built-in plugins. `admin` and the `super_admin` override can manage them and use platform administration checks. Editor/author/viewer do not pass admin checks.

## Backend, Frontend, and API

Backend enforcement uses `Claims.role`, `require_plugin_*`, or `require_any`. Marketplace/Beta frontend flags show platform controls for admin/super-admin only; those flags are not authoritative. See the built-in plugin, beta, Marketplace review, runtime, finance, and analytics endpoint families.

## Database and Tests

Global tables and explicit bypass transactions may be involved. RLS bypass must be separately justified. Plugin helper tests and Marketplace static contracts provide partial coverage; no exhaustive global route matrix exists.

## Unclear Semantics

The relationship between legacy `roles.permissions`, global helper capabilities, and actual registered global endpoints is `RBAC_MAPPING_UNCLEAR RMU-01`.
