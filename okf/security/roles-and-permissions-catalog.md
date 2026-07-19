---
okf_document_id: "security-roles-permissions-catalog"
title: "Roles and Permissions Catalog"
project: "ZinharCMS"
category: "security-rbac"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "mixed"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/services/rbac.rs"
  - "backend/src/routes"
  - "backend/migrations/0001_initial_schema.sql"
  - "backend/migrations/0008_v2_phase_one_organizations.sql"
  - "backend/migrations/0020_v3_phase_seven_permission_sandbox_kill_switch.sql"
related_documents:
  - "rbac-model.md"
  - "authorization-architecture.md"
  - "tenant-access-control.md"
related_diagrams:
  - "diagrams/rbac-model.mmd"
---

# Roles and Permissions Catalog

## Global Roles

| Role | Main observed use | Document |
| --- | --- | --- |
| `super_admin` | Global override, platform administration, first-user bootstrap | [Global Super Admin](roles/global-super-admin.md) |
| `admin` | Platform administration and built-in plugin management | [Global Admin](roles/global-admin.md) |
| `editor` | Legacy global editorial capabilities; plugin read | [Global Editor](roles/global-editor.md) |
| `author` | Default later-registration role; plugin read | [Global Author](roles/global-author.md) |
| `viewer` | Global read capabilities; plugin read | [Global Viewer](roles/global-viewer.md) |

## Organization Roles

| Role | Main observed use | Document |
| --- | --- | --- |
| `owner` | Organization override and ownership lifecycle | [Organization Owner](roles/organization-owner.md) |
| `admin` | Organization administration and most privileged tenant operations | [Organization Admin](roles/organization-admin.md) |
| `editor` | Content/page management and workflow | [Organization Editor](roles/organization-editor.md) |
| `author` | Content/page/media creation and comment writing | [Organization Author](roles/organization-author.md) |
| `viewer` | Read-oriented tenant and comment access | [Organization Viewer](roles/organization-viewer.md) |
| `billing_manager` | Billing/subscription/usage operations | [Organization Billing Manager](roles/organization-billing-manager.md) |

## Permission Groups

| Group | Scope | Document |
| --- | --- | --- |
| Authentication and session | Public and authenticated identity | [Authentication and Session](permissions/authentication-and-session.md) |
| Global administration and plugins | Global | [Global Administration and Built-in Plugins](permissions/global-administration-and-plugins.md) |
| Organization administration | Organization | [Organization Administration](permissions/organization-administration.md) |
| Content, workflow, comments | Organization; legacy global helpers also exist | [Content, Workflow, and Comments](permissions/content-workflow-and-comments.md) |
| Pages, media, components, webhooks | Organization; legacy global helpers also exist | [Pages, Media, Components, and Webhooks](permissions/pages-media-components-and-webhooks.md) |
| Billing and SaaS operations | Organization and selected global product operations | [Billing and SaaS Operations](permissions/billing-and-saas-operations.md) |
| Marketplace management | Creator, global platform, and organization | [Marketplace Management](permissions/marketplace-management.md) |
| Marketplace runtime capabilities | Installation/organization | [Marketplace Runtime Capabilities](permissions/marketplace-runtime-capabilities.md) |

## Vocabulary Warning

Three permission representations coexist:

- role names and named capability helpers in `rbac.rs`, which are authoritative for current RBAC decisions;
- colon-delimited strings and `*` in `roles.permissions`, which are stored but not used by inspected enforcement;
- nine dot-delimited Marketplace permission keys, which are validated, approved, persisted per installation, and mapped to runtime operations.

This is `PERMISSION_SEMANTICS_UNCLEAR PSU-01`. Do not translate one vocabulary into another without implementation and migration changes.
