---
okf_document_id: "extensibility-catalog"
title: "Extensibility Catalog"
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
  - "backend/src/plugins"
  - "backend/src/routes/plugins.rs"
  - "backend/src/routes/pages.rs"
  - "backend/src/routes/marketplace_adapters.rs"
  - "backend/src/services/marketplace_adapters.rs"
  - "backend/src/services/marketplace_runtime.rs"
related_documents:
  - "plugins/seo-auto.md"
  - "extension-points/cms-entry-before-save.md"
  - "extension-points/cms-entry-after-publish.md"
  - "extension-points/page-builder-component-registry.md"
  - "extension-points/marketplace-template-adapter.md"
  - "extension-points/marketplace-public-hook-adapter.md"
  - "extension-points/marketplace-runtime-authorization.md"
related_diagrams:
  - "diagrams/extensibility-context.mmd"
---

# Extensibility Catalog

| ID | Mechanism | Kind | Registration | Scope | Execution | Implementation | Evidence |
|---|---|---|---|---|---|---|---|
| EXT-001 | [SEO Auto Generator](plugins/seo-auto.md) | Built-in plugin | Static Rust vector plus DB metadata sync | Global enablement; tenant context passed | Synchronous in-process | VERIFIED | plugins/mod.rs, plugins/seo.rs |
| EXT-002 | [Entry Before-Save](extension-points/cms-entry-before-save.md) | CMS hook | CmsPlugin trait | Global enabled set; tenant context | Mutating synchronous callback | VERIFIED | plugins/mod.rs, routes/content.rs |
| EXT-003 | [Entry After-Publish](extension-points/cms-entry-after-publish.md) | CMS hook | CmsPlugin trait | Global enabled set; tenant context | Synchronous callback | VERIFIED; no subscriber found | plugins/mod.rs, routes/content.rs |
| EXT-004 | [Page Builder Component Registry](extension-points/page-builder-component-registry.md) | Declarative component schema | API, seed, Marketplace adapter | System-global or organization-owned | Host-rendered/editor metadata | VERIFIED | routes/pages.rs and migrations |
| EXT-005 | [Marketplace Template Adapter](extension-points/marketplace-template-adapter.md) | Declarative import adapter | Active installation manifest | Organization | Host preview/import | VERIFIED | Marketplace adapter route/service |
| EXT-006 | [Marketplace Public Hook Adapter](extension-points/marketplace-public-hook-adapter.md) | Declarative hook descriptor | Active installation manifest | Organization | Authorization only; not executed | PARTIALLY_IMPLEMENTED | Marketplace adapter route/service |
| EXT-007 | [Marketplace Runtime Authorization](extension-points/marketplace-runtime-authorization.md) | Host capability boundary | Static operation table plus manifest | Organization and global controls | Policy decision; not executed | VERIFIED authorization | Marketplace runtime service |

## Counts and Exclusions

Only one concrete CmsPlugin implementation was found: [SEO Auto Generator](plugins/seo-auto.md). Marketplace products are not counted as executable plugins because no package loader or package-code execution path was found.

Six dedicated extension-point documents cover the two CMS hooks, Page Builder registry, template adapter, public hook adapter, and runtime authorization boundary. Ordinary frontend imports and internal service calls are excluded.

Marketplace capabilities are cataloged in [Marketplace Architecture](marketplace-architecture.md) and the dedicated areas beginning with [Creator, Listing, and Version Management](marketplace/creator-listing-version.md).

## Detailed Mechanism Matrix

| Extension ID | Name | Category | Source path | Runtime or build-time | Registration method | Consumer | Contract type | Configuration | Permissions | Tenant scope | Data ownership | Compatibility behavior | Test coverage | Implementation status | Confidence | Dedicated document |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| EXT-001 | SEO Auto Generator | Backend plugin | backend/src/plugins/seo.rs | Build-time registration; runtime callback | Add to builtin_plugins(), then metadata upsert | Content create/update | CmsPlugin before-save method | No consumed settings | Host-process authority; global manager controls enablement | Global enablement, tenant invocation context | Shared content entry mutation; global metadata | Same-build compilation; version 1.0.0 metadata | Two unit tests | IMPLEMENTED | High | [SEO Auto](plugins/seo-auto.md) |
| EXT-002 | Entry Before-Save | Hook | backend/src/plugins/mod.rs; routes/content.rs | Runtime synchronous | Static trait implementation | Content routes | Mutable EntryData callback | None in contract | No per-plugin capability | Global enabled set with tenant context | Mutates shared entry before persistence | Same-build Rust trait | SEO subscriber unit tests | IMPLEMENTED | High | [Before-Save](extension-points/cms-entry-before-save.md) |
| EXT-003 | Entry After-Publish | Hook | backend/src/plugins/mod.rs; routes/content.rs | Runtime synchronous | Static trait implementation | Publish route | Immutable EntryData callback | None | No per-plugin capability | Global enabled set with tenant context | No verified subscriber-owned data | Same-build Rust trait | No subscriber test | PARTIALLY_IMPLEMENTED | High for contract, low for behavior | [After-Publish](extension-points/cms-entry-after-publish.md) |
| EXT-004 | Page Builder Component Registry | Component registry | backend/src/routes/pages.rs; component_registry migrations | Runtime data lookup | Seed, tenant CRUD, Marketplace upsert | Page validator and PagesPage | Key/category/props-schema record | props_schema | Component-manager RBAC for mutation | Global system or organization | Shared registry rows; optional installation link | Key/schema behavior; no renderer protocol version | Backend/frontend selected tests | IMPLEMENTED | High | [Component Registry](extension-points/page-builder-component-registry.md) |
| EXT-005 | Marketplace Template Adapter | Integration adapter | marketplace_adapters route/service | Runtime host action | Active ready design-template manifest | PagesPage and page persistence | template key, page_json, assets | Import request and asset mapping | Page writer plus installation/runtime gates | Organization | Page, page version, import record, audit | Current Page Builder validation | Helper/frontend selected tests | IMPLEMENTED | High | [Template Adapter](extension-points/marketplace-template-adapter.md) |
| EXT-006 | Marketplace Public Hook Adapter | Runtime extension point | marketplace_adapters route/service | Runtime authorization only | Active ready manifest hook declaration | Host API; no verified frontend renderer | Public hook definition and authorization request | Hook config JSON | Installation snapshot and runtime policy | Organization | Manifest and hook registry metadata | Contract version 2026-07; no negotiation | Extraction/route helper tests | PARTIALLY_IMPLEMENTED | High for authorization, low for delivery | [Public Hook](extension-points/marketplace-public-hook-adapter.md) |
| EXT-007 | Marketplace Runtime Authorization | Runtime extension point | marketplace_runtime route/service | Runtime policy decision | Static operation catalog plus manifest | Host adapters/APIs | Operation, entry point, payload, permission | Installation/runtime state | Exact approved capability and kill switches | Organization with global overlay | Installation snapshot and audit/runtime state | Contract version 2026-07 | Five policy tests plus route contract test | PARTIALLY_IMPLEMENTED | High | [Runtime Authorization](extension-points/marketplace-runtime-authorization.md) |
