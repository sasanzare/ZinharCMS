---
okf_document_id: "plugin-configuration"
title: "Plugin Configuration"
project: "ZinharCMS"
category: "extensibility"
phase: 9
status: "current"
source_of_truth: false
implementation_view: "observed"
extensibility_status: "partially_verified"
last_verified_commit: "56d733985fdd7aa3f25ee6981b88cf29c52f65c9"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/routes/plugins.rs"
  - "backend/migrations/0006_phase_six_workflow_collaboration.sql"
  - "backend/src/services/marketplace_adapters.rs"
related_documents:
  - "plugin-architecture.md"
  - "plugin-manifest.md"
  - "tenant-and-global-scope.md"
related_diagrams:
  - "diagrams/plugin-data-ownership.mmd"
---

# Plugin Configuration

cms_plugins.settings is a global JSON object exposed through the plugin update API. The current CmsPlugin callback contract receives only EntryData and PluginContext; it does not receive settings. The SEO plugin does not read settings. Settings persistence is VERIFIED, while runtime consumption is PLUGIN_CONFIGURATION_UNCLEAR.

Marketplace configuration is distributed:

- manifest JSON defines permissions, compatibility, entry points, assets, and optional adapter definitions;
- an installation stores version, approved permissions, lifecycle state, runtime state, and metadata;
- hook definitions can include a JSON config object;
- component definitions include props_schema;
- template imports accept a template key, asset mapping, title, and slug.

No general configuration schema, secrets contract, override hierarchy, configuration migration hook, or per-plugin validation interface was found. Secrets must not be assumed safe inside manifest or installation JSON.

## Configuration Group Matrix

| Setting group | Scope | Storage | Default | Override order | Validation | Edit permission | Secret sensitivity | Runtime consumer | Restart | Tests | Confidence |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cms_plugins.settings | Global | PostgreSQL JSON object | Empty object from schema/route behavior | No override hierarchy found | Must be JSON object through request/schema path | Global plugin manager | No secret classification or redaction contract | No current CmsPlugin consumer | No for persistence; behavior unavailable | No route/runtime test | Medium |
| Built-in metadata | Global | Source methods synchronized to DB | Source values | Source sync overwrites metadata, preserves enablement | Key/name/version/hook types | Source developer; admin can update selected fields | Low | Admin API and runner key match | New source requires deploy | SEO metadata indirectly exercised | High |
| Marketplace manifest | Product version | marketplace_versions manifest JSON | No implicit manifest default beyond adapter-specific behavior | Installed version pins one manifest | Manifest/package validation | Creator upload followed by review | Must not contain secrets; no secret field contract | Install gates, runtime policy, adapters | No | Manifest/validation tests | High |
| Installation snapshot | Tenant | marketplace_installations JSON/state columns | Active, pinned, approved values at install | Update replaces only through explicit flow | Exact permissions, lifecycle, compatibility | Installer/permission approver | Permission and entitlement sensitive | Runtime policy/adapters | No | Lifecycle tests | High |
| Component props schema | Global or tenant | component_registry JSON | Empty object for missing Marketplace props schema | Marketplace upsert or manager edit | Object/key/database checks | Component manager or host adapter | Should not carry secrets | Page Builder host | No | Adapter/Page tests | Medium |
| Public hook config | Tenant installation manifest | Manifest and derived hook rows | Empty object | Version update changes declaration | Must be JSON object and public type | Creator plus install/review approval chain | Potential endpoint/config sensitivity; no secret schema | List/authorize only | No | Adapter tests | Medium |
| Template import input | Tenant request | Page/page version/import record | Adapter template fallback where defined | Request template key and asset map | Slug, assets, quota, page JSON | Page writer | Asset IDs and content data | Host import | No | Helper/frontend tests | High |
| Plugin-specific environment | Process | Environment/config | No plugin-specific variables found | None | None | Deployment authority | Potentially secret | No verified plugin consumer | Process restart generally required | None | High confidence absent |

Global, tenant, and user-specific generic plugin settings are not interchangeable. No user-scoped plugin configuration was found.
