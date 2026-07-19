---
okf_document_id: "security-role-global-admin"
title: "Global Admin Role"
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
role_scope: "global"
assignment_type: "global database role assignment"
implementation_status: "verified"
primary_sources:
  - "backend/src/services/rbac.rs"
  - "backend/src/routes/beta.rs"
  - "backend/src/routes/marketplace.rs"
  - "backend/src/routes/plugins.rs"
related_documents:
  - "../roles-and-permissions-catalog.md"
  - "../administrative-access.md"
  - "../permissions/global-administration-and-plugins.md"
related_diagrams:
  - "../diagrams/rbac-model.mmd"
  - "../diagrams/authorization-decision-flow.mmd"
---

# Global Admin Role

## Identity and Scope

`admin` is the standard global platform-administration role. It is distinct from organization `admin` despite sharing the same string.

## Assignment

The role is stored in `roles` and assigned through `user_roles`. Registration does not assign it by default. No general global-role administration endpoint was found.

## Effective Permissions

Observed checks allow global admin to manage built-in plugins, global beta product operations, Marketplace review/moderation/analytics, selected finance verification, and global kill switches. Global CMS helper functions also allow content-type/component/webhook/plugin management and editorial operations.

## Restrictions

Global admin is not a tenant membership. Tenant routes still require active organization context and the applicable organization role. Global admin also does not automatically satisfy Marketplace creator ownership.

## Enforcement, Frontend, and Tests

Backend checks use the token claim and `require_any`. Frontend Marketplace and beta flags expose matching controls but are not authoritative. No exhaustive global admin endpoint test matrix was found: `AUTHORIZATION_ENFORCEMENT_UNCLEAR AEU-01`.
