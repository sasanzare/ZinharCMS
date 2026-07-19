---
okf_document_id: "security-role-global-super-admin"
title: "Global Super Admin Role"
project: "ZinharCMS"
category: "security-role"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "partially_verified"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
role_id: "super_admin"
role_name: "Super Admin"
role_scope: "global"
assignment_type: "global database role assignment"
implementation_status: "verified"
primary_sources:
  - "backend/src/services/rbac.rs"
  - "backend/src/routes/auth.rs"
  - "backend/src/main.rs"
  - "backend/migrations/0003_phase_one_core.sql"
related_documents:
  - "../roles-and-permissions-catalog.md"
  - "../administrative-access.md"
  - "../permissions/global-administration-and-plugins.md"
related_diagrams:
  - "../diagrams/rbac-model.mmd"
  - "../diagrams/authorization-decision-flow.mmd"
---

# Global Super Admin Role

## Identity and Scope

`super_admin` is the global override role. `require_any` accepts it for every global RBAC check, including checks whose explicit allowlist contains only `admin`.

## Assignment

The empty-database startup bootstrap and the first public registration assign this role. `user_roles` can also represent assignments made through direct database/application behavior, but no general global-role management API was found.

## Effective Permissions

The role passes built-in plugin management, product beta administration, Marketplace review/moderation/analytics, finance verification, global kill-switch, and every named global CMS capability helper. See [Global Administration and Built-in Plugins](../permissions/global-administration-and-plugins.md).

## Restrictions

This role does not automatically select a tenant, create active organization membership, bypass tenant middleware, satisfy resource ownership, satisfy entitlements, or activate RLS bypass. Tenant operations require an active organization role.

## Enforcement and Frontend

Backend global checks use `Claims.role`. The frontend uses this role to show selected beta and Marketplace administration controls; those checks are `FRONTEND_ONLY_SECURITY_CHECK FOSC-01`.

## Tests and Uncertainty

Global override behavior follows the small `require_any` function, but no exhaustive global route matrix was found. Bootstrap behavior carries `POTENTIAL_SECRET_EXPOSURE PSE-01` and `AUTHENTICATION_FLOW_UNCLEAR AFU-01`.
