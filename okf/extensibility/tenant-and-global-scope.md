---
okf_document_id: "extensibility-tenant-global-scope"
title: "Tenant and Global Scope"
project: "ZinharCMS"
category: "extensibility"
phase: 9
status: "current"
source_of_truth: false
implementation_view: "observed"
extensibility_status: "mixed"
last_verified_commit: "56d733985fdd7aa3f25ee6981b88cf29c52f65c9"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/migrations/0006_phase_six_workflow_collaboration.sql"
  - "backend/migrations/0008_v2_phase_one_organizations.sql"
  - "backend/migrations/0009_v2_phase_three_rls.sql"
  - "backend/migrations/0015_v3_phase_one_marketplace_foundation.sql"
  - "backend/src/middleware/tenant.rs"
related_documents:
  - "../security/tenant-access-control.md"
  - "../database/multi-tenancy.md"
  - "../database/module-data-ownership.md"
  - "plugin-data-and-migrations.md"
related_diagrams:
  - "diagrams/plugin-data-ownership.mmd"
---

# Tenant and Global Scope

| Data or control | Scope | Enforcement evidence |
|---|---|---|
| cms_plugins metadata/enablement/settings | Global | Protected global routes; no organization_id |
| Compiled plugin implementation | Process-global | Static binary |
| Plugin invocation context | Tenant-aware | PluginContext carries user and organization IDs |
| System Page Builder components | Global-visible | is_system true, null organization |
| Custom Page Builder components | Organization | Tenant route, organization ID, RLS |
| Marketplace listings/versions/catalog | Global publication metadata | Global entities with review state |
| Marketplace installations | Organization | Tenant middleware, RBAC, organization ID, forced RLS |
| Marketplace permission snapshots/hooks/template imports | Organization | Installation ownership and tenant/RLS queries |
| Kill switches | Global or organization | Explicit scope and privileged routes |

A globally enabled CMS plugin runs for entries across organizations when its hook is reached; there is no per-tenant built-in-plugin flag. Marketplace activation is organization-specific.

PLUGIN_TENANT_SCOPE_UNCLEAR remains for generic plugin-owned configuration and data because no ownership contract exists. Current Marketplace tenant ownership is VERIFIED, subject to the database caveat that deployed migration state was not inspected.

## Scope Classifications

| Mechanism | Classification |
|---|---|
| Built-in CmsPlugin code and enablement | GLOBAL |
| Built-in invocation context | MIXED_SCOPE |
| System component definitions | GLOBAL |
| Custom component definitions | TENANT_CONFIGURABLE |
| Marketplace installation | TENANT_INSTALLABLE and TENANT_ACTIVATABLE |
| Marketplace manifest/version | GLOBAL catalog metadata |
| Marketplace hook/component/template availability | TENANT_ACTIVATABLE through installation state |
| Generic user-scoped plugin state | PLUGIN_TENANT_SCOPE_UNCLEAR; no implementation found |

## Scope Dimension Matrix

| Mechanism | Installation | Activation | Configuration | Data | Permission | Visibility | Upgrade | Removal | Isolation | Cross-tenant administration | Tests |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Built-in plugin | Build/deploy global | Global flag | Global settings | Global metadata; shared tenant entry mutation | Global management RBAC; no callback capability | All tenants when enabled | Deployment-wide | Source removal; stale row unclear | No runtime isolation | Global admins manage | No tenant matrix |
| System component | Migration seed | Always available | Migration-owned | Global registry row | Tenant readers; system mutation restricted | All tenants | Application migration | System delete denied | Host data boundary | Application/global administration | Page tests |
| Tenant custom component | Tenant API | Available while row exists | Tenant manager | Tenant registry row under RLS | Component-manager RBAC | Owning tenant | Manual API update | Hard delete | Tenant/RLS boundary | RLS bypass only through authorized host paths | Selected route/static evidence |
| Marketplace installation | Tenant API | active/disabled plus runtime overlay | Version/snapshot/config declarations | Tenant installation/adapter records; global package | Installer/approver/runtime capability | Owning tenant | Explicit pinned update | Soft uninstall | Data isolation through tenant/RLS; no code runtime | Privileged global moderation/kill switches | Pure policy tests; live tenant gap |

An organization_id column alone is not treated as proof of isolation; tenant middleware, RBAC, forced RLS, query context, and integration tests must align.
