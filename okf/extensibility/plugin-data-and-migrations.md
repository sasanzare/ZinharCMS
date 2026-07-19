---
okf_document_id: "plugin-data-migrations"
title: "Plugin Data and Migrations"
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
  - "backend/migrations/0015_v3_phase_one_marketplace_foundation.sql"
  - "backend/migrations/0019_v3_phase_six_installation_lifecycle.sql"
  - "backend/migrations/0020_v3_phase_seven_permission_sandbox_kill_switch.sql"
  - "backend/migrations/0021_v3_phase_eight_runtime_adapters.sql"
related_documents:
  - "../database/module-data-ownership.md"
  - "../database/migrations.md"
  - "../backend/persistence-access.md"
  - "tenant-and-global-scope.md"
related_diagrams:
  - "diagrams/plugin-data-ownership.mmd"
---

# Plugin Data and Migrations

Built-in plugin metadata lives in the global cms_plugins table. SEO Auto has no dedicated table, file, migration callback, or tenant-owned data. Its only business effect is mutation of an entry slug before persistence.

Marketplace data spans creators, listings, versions, submissions, validation/review events, installations, permissions, kill switches, component links, template imports, hooks, purchases, entitlements, ledger/payouts, reviews, reports, analytics/readiness evidence, and audit records. Organization-owned tables use tenant context/RLS according to the database documentation.

Migrations are application-owned and run at backend startup. No package-supplied migration runner or per-plugin schema namespace exists. A package manifest cannot claim authority to execute SQL.

Uninstall preserves organization data and installation history. The explicit cleanup policy is preserve_organization_data. Retention of uploaded artifacts and removal of Marketplace-derived component_registry rows are PLUGIN_DATA_OWNERSHIP_UNCLEAR and PLUGIN_REMOVAL_BEHAVIOR_UNCLEAR.

See [Database Ownership](../database/module-data-ownership.md) and [Migrations](../database/migrations.md).

## Data Ownership Matrix

| Plugin/area | Schema objects | Owning module | Scope | Migration source | Read/write paths | Deactivation | Removal | Compatibility | Tests | Confidence |
|---|---|---|---|---|---|---|---|---|---|---|
| Built-in registry | cms_plugins | Plugin routes and runners | Global | 0006_phase_six_workflow_collaboration.sql | routes/plugins.rs reads/writes; plugins/mod.rs reads enabled keys | Row/settings retained; callback skipped | No uninstall; source removal may leave row | Metadata version only | No route/DB tests | High schema; medium lifecycle |
| SEO Auto | Shared content_entries slug | Content routes plus SEO callback | Tenant content with global plugin enablement | Existing content schema; no plugin migration | Callback mutates EntryData, host writes entry | No further mutation while disabled | Existing entries retained | Same-build code | Two unit tests | High |
| Marketplace catalog/package | creators, listings, versions, submissions, reports/events | Marketplace routes/services | Global catalog/creator ownership | 0015-0018 and later additive migrations | Creator/reviewer/catalog routes | Not governed by installation state | Delist/deprecate/moderate; artifact retention policy incomplete | Manifest/schema/version gates | Manifest/review tests | High |
| Marketplace installation/runtime | installations, permission catalog, kill switches | Installation/runtime services | Tenant plus global controls | 0019-0021 | Tenant lifecycle/runtime routes under RLS; privileged global controls | Data retained, adapters ineligible | Soft uninstall preserves row and organization data | Version pin/update/rollback snapshots | Lifecycle/runtime tests | High |
| Marketplace components | component_registry.marketplace_installation_id | Pages/Marketplace adapters | Tenant or system | 0001, 0004, 0008, 0009, 0021 | Pages CRUD/list; adapter upsert | Stale-row behavior unclear | FK sets installation reference null on deletion, but install is soft; cleanup unclear | Key/schema unversioned | Selected tests | Medium |
| Marketplace templates/hooks | marketplace_template_imports, marketplace_plugin_hooks | Marketplace adapters/Pages | Tenant | 0021 | Adapter preview/import/list/authorize | Imports/pages retained; inactive hooks not returned | Data retention/cleanup incomplete | Adapter contract 2026-07 | Helper tests | Medium |
| Marketplace commerce/feedback | purchases, entitlements, ledger, payouts, reviews, reports | Finance/feedback/analytics | Tenant/creator/global moderation | 0022-0026 | Host routes/services | Independent historical/business state retained | Not deleted by uninstall | Listing/version references and policy | Pure/service tests | Medium to high |

## Migration and Recovery Findings

- Migration ordering is application-wide filename order, not plugin-declared order.
- Install, update, activation, deactivation, and removal do not execute schema migrations.
- No plugin-owned table namespace or collision detector exists.
- Backup/restoration, artifact retention, and restoration compatibility are UNKNOWN at the operational layer.
- Shared table changes require normal application migration review and tenant/RLS testing.
