---
okf_document_id: "security-rbac-model"
title: "RBAC Model"
project: "ZinharCMS"
category: "security-rbac"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "partially_verified"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/services/rbac.rs"
  - "backend/migrations/0001_initial_schema.sql"
  - "backend/migrations/0003_phase_one_core.sql"
  - "backend/migrations/0008_v2_phase_one_organizations.sql"
  - "backend/src/routes/auth.rs"
related_documents:
  - "authorization-architecture.md"
  - "roles-and-permissions-catalog.md"
  - "administrative-access.md"
related_diagrams:
  - "diagrams/rbac-model.mmd"
---

# RBAC Model

## RBAC Entities

Global assignment uses `users`, `roles`, and the many-to-many `user_roles` table. Organization assignment uses `organization_members`, whose primary key is `(organization_id, user_id)` and whose role is an enum. Access tokens carry one selected global role; tenant middleware loads one role for the selected organization.

## Assignment Model

The first registered user and empty-database bootstrap user receive `super_admin`; later self-registrations receive `author`. Registration also maps the selected global role into a default-organization role. Organization invitation, member-role update, ownership transfer, and leave/remove flows can change organization assignments.

The login/refresh queries select the highest-priority global role in this order: `super_admin`, `admin`, `editor`, `author`, `viewer`. Multiple global role rows are permitted by the schema, but only one role is embedded in an access token.

## Effective Permission Resolution

`require_any` allows global `super_admin` or an explicitly allowed global role. `require_org_any` allows organization `owner` or an explicitly allowed organization role. Named helpers wrap these primitives. There are no explicit deny rules, custom runtime roles, inheritance graph, or role/permission cache.

## Capability Matrix

| Capability | Global roles | Organization roles |
| --- | --- | --- |
| Content-type management | super admin, admin | owner, admin, editor |
| Entry write | super admin, admin, editor, author | owner, admin, editor, author |
| Entry publish/review | super admin, admin, editor | owner, admin, editor |
| Media write | super admin, admin, editor, author | owner, admin, editor, author |
| Page write | super admin, admin, editor, author | owner, admin, editor, author |
| Page publish/manage | super admin, admin, editor | owner, admin, editor |
| Component registry | super admin, admin | owner, admin, editor |
| Webhooks | super admin, admin | owner, admin |
| Comments read | all five global roles | owner, admin, editor, author, viewer |
| Comments write | super admin, admin, editor, author | owner, admin, editor, author |
| Comments manage | super admin, admin, editor | owner, admin, editor |
| Built-in plugin read/manage | all / super admin and admin | Not organization-scoped |
| Billing | Not represented by global helper | owner, admin, billing manager |
| Marketplace install/permission/kill switch | Platform admin checks are separate | owner, admin |

Global CMS capability helpers remain in `rbac.rs`, but current tenant CMS routes use organization helpers. Treat the global helper set as implemented code, not proof of a currently registered global CMS surface.

## Stored Permission Arrays

The `roles.permissions` column contains legacy permission strings and wildcards from migrations. No inspected authorization path reads the column. `RBAC_MAPPING_UNCLEAR RMU-01` prohibits treating those arrays as effective runtime grants.

## Test Evidence

A unit matrix checks selected organization capabilities across all six organization roles. Individual service/static contract tests cover additional Marketplace and ownership checks. No complete global matrix or route-level end-to-end RBAC suite was found: `SECURITY_TEST_COVERAGE_UNCLEAR STCU-01`.
