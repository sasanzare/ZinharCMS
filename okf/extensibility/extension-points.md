---
okf_document_id: "extension-points"
title: "Extension Points"
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
  - "backend/src/plugins/mod.rs"
  - "backend/src/routes/content.rs"
  - "backend/src/routes/pages.rs"
  - "backend/src/routes/marketplace_adapters.rs"
  - "backend/src/services/marketplace_adapters.rs"
  - "backend/src/services/marketplace_runtime.rs"
related_documents:
  - "backend-extension-points.md"
  - "frontend-extension-points.md"
  - "component-and-block-registration.md"
  - "hooks-and-events.md"
related_diagrams:
  - "diagrams/extensibility-context.mmd"
  - "diagrams/component-registration.mmd"
---

# Extension Points

| ID | Name | Category | Contract | Registration | Caller | Scope | Status | Document |
|---|---|---|---|---|---|---|---|---|
| EP-001 | Entry Before-Save | Backend lifecycle hook | Mutable EntryData and PluginContext | Compiled CmsPlugin | Content create/update | Global plus tenant context | VERIFIED | [Details](extension-points/cms-entry-before-save.md) |
| EP-002 | Entry After-Publish | Backend lifecycle hook | Immutable EntryData and PluginContext | Compiled CmsPlugin | Content publish | Global plus tenant context | VERIFIED; no subscriber | [Details](extension-points/cms-entry-after-publish.md) |
| EP-003 | Page Builder Component Registry | Declarative metadata | Key/name/category/props schema | Seed, API, Marketplace sync | Pages API/frontend | System or tenant | VERIFIED | [Details](extension-points/page-builder-component-registry.md) |
| EP-004 | Marketplace Template Adapter | Declarative page template | Template key/page JSON/assets | Installation manifest | Preview/import routes | Tenant | VERIFIED | [Details](extension-points/marketplace-template-adapter.md) |
| EP-005 | Marketplace Public Hook Adapter | Declarative hook | Public type/key/label/config | Installation manifest | List/authorize routes | Tenant | PARTIALLY_IMPLEMENTED | [Details](extension-points/marketplace-public-hook-adapter.md) |
| EP-006 | Marketplace Runtime Authorization | Capability policy | Operation/entry point/payload | Static table plus manifest | Runtime/adapter routes | Tenant plus global control | VERIFIED policy only | [Details](extension-points/marketplace-runtime-authorization.md) |

Ordinary React imports, Axum route merges, internal audit calls, spawned tasks, CMS webhooks, and service calls are not verified third-party registration surfaces. EXTENSION_POINT_UNVERIFIED applies to any proposed surface not listed above.

## Extension-Point Contract Matrix

| ID | Purpose | Source path | Contract | Registration | Timing | Inputs | Outputs | Error behavior | Ordering | Multiple registration | Permissions | Tenant scope | Stability | Consumers | Tests | Confidence | Dedicated document |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| EP-001 | Mutate entry before save | plugins/mod.rs; routes/content.rs | CmsPlugin mutable callback | Static built-in vector | Create/update before persistence | EntryData, PluginContext | Result and mutated entry | Error aborts request | Vector order | Multiple built-ins allowed by distinct keys; duplicate source keys not checked | Host authority | Global enablement; tenant context | Source-level, unversioned | Content routes | SEO unit tests | High | [Before-Save](extension-points/cms-entry-before-save.md) |
| EP-002 | Notify after publish | plugins/mod.rs; routes/content.rs | CmsPlugin immutable callback | Static built-in vector | Publish orchestration | EntryData, PluginContext | Result | Error propagates | Vector order | Same static behavior; no subscriber found | Host authority | Global enablement; tenant context | Source-level, unversioned | Publish route | No subscriber test | Medium | [After-Publish](extension-points/cms-entry-after-publish.md) |
| EP-003 | Register Page Builder definitions | routes/pages.rs; marketplace_adapters.rs | Component registry record | Seed, CRUD, namespaced upsert | List/save/import validation | Key, name, category, props schema | Registry/API records | Validation/DB error | Database query order is not a contract | Unique global component_key; upsert for Marketplace | Component-manager/page permissions | System-global or tenant | Data contract unversioned | Page validator, PagesPage | Selected route/frontend tests | High | [Component Registry](extension-points/page-builder-component-registry.md) |
| EP-004 | Import declarative page template | marketplace_adapters route/service | Template manifest and request | Active ready installation | Preview or import request | Template key, page_json, assets, title, slug | Preview JSON or PageResponse | Transaction rolls back on host failure | One requested installation/template | One page per import request | Page writer; runtime/install gates | Tenant | Adapter version 2026-07; page schema current | PagesPage | Mapping/frontend tests | High | [Template Adapter](extension-points/marketplace-template-adapter.md) |
| EP-005 | Expose declared host hook | marketplace_adapters route/service | Public type/key/label/config | Active ready installation manifest | List/authorize request | Hook type and context | Definitions or allowed/denied, not_executed | Validation/authorization denial | No delivery order exists | Multiple definitions returned; key uniqueness stored per installation | Runtime permission policy | Tenant | Contract version 2026-07 | Host API; renderer unverified | Extraction/route helper tests | Medium | [Public Hook](extension-points/marketplace-public-hook-adapter.md) |
| EP-006 | Authorize host capability | marketplace_runtime route/service | Static operation policy | Static catalog plus approved manifest | Runtime authorize request | Operation, entry point, JSON payload | Allowed/denied, not_executed | Denial with bounded reason | One decision per request | Operations are unique static definitions | Exact approved permission | Tenant plus global kill switch | Policy version 2026-07 | Runtime route/adapters | Policy and route contract tests | High | [Runtime Authorization](extension-points/marketplace-runtime-authorization.md) |
