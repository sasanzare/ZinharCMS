---
okf_document_id: "extensibility-terminology"
title: "Extensibility Terminology"
project: "ZinharCMS"
category: "extensibility"
phase: 9
status: "current"
source_of_truth: false
implementation_view: "observed"
extensibility_status: "verified"
last_verified_commit: "56d733985fdd7aa3f25ee6981b88cf29c52f65c9"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/plugins/mod.rs"
  - "backend/src/services/marketplace_manifest.rs"
  - "backend/src/services/marketplace_adapters.rs"
related_documents:
  - "overview.md"
  - "extensibility-catalog.md"
related_diagrams:
  - "diagrams/extensibility-context.mmd"
---

# Extensibility Terminology

| Term | Verified project-specific definition | Source | Related terms | Ambiguity | Status |
|---|---|---|---|---|---|
| Plugin | A Rust type implementing CmsPlugin and returned by builtin_plugins(). | backend/src/plugins/mod.rs | Built-in plugin, registration | Marketplace product names may include plugin without executable loading. | VERIFIED |
| Built-in plugin | Trusted plugin compiled into the backend binary. | backend/src/plugins/seo.rs | Plugin, module | None for current implementation. | VERIFIED |
| Extension | Umbrella documentation term for a verified plugin, registry, hook, or host adapter. | Phase 9 evidence synthesis | Extension point, integration | No source-level Extension interface exists. | INFERRED_FROM_CODE |
| Extension point | A host contract with verified registration and consumption behavior. | backend/src/plugins/mod.rs; Marketplace adapters | Hook, registry | Proposed internal functions are excluded. | VERIFIED |
| Module | Ordinary Rust/TypeScript source unit; not a plugin unless it implements a verified contract. | backend/src/routes/mod.rs | Plugin, service | Some historical documentation uses module broadly. | VERIFIED |
| Package | Uploaded Marketplace artifact plus manifest and stored version metadata. | marketplace_package.rs | Marketplace item, version | It is not a loaded code module. | VERIFIED |
| Add-on | No distinct repository contract found. | Repository search | Plugin, package | Product vocabulary requires owner confirmation. | NEEDS_OWNER_CONFIRMATION |
| Integration | Host-owned connection such as Marketplace hook declaration, webhook, or provider path. | marketplace_adapters.rs; webhooks.rs | Adapter, provider | Not every integration is an extension point. | AMBIGUOUS |
| Adapter | Trusted backend logic interpreting declarative component, template, or hook data. | marketplace_adapters.rs | Integration, package | No third-party adapter implementation is loaded. | VERIFIED |
| Provider | External host integration, such as Stripe or email; no plugin provider registry exists. | backend/src/services | Adapter | Provider extensibility is unverified. | AMBIGUOUS |
| Component | Page Builder registry record with key, category, and props schema. | routes/pages.rs | Block, widget | Does not imply executable frontend code. | VERIFIED |
| Block | Page Builder usage of a registered component in serialized page JSON. | routes/pages.rs; PagesPage.tsx | Component | Source uses component more consistently than block. | INFERRED_FROM_CODE |
| Widget | Marketplace public hook type dashboard.widget; no general renderer registry found. | marketplace_adapters.rs | Component, hook | Delivery and rendering are unclear. | AMBIGUOUS |
| Template | Declarative Marketplace page_json plus assets imported by the host. | marketplace_adapters.rs | Component, package | Versioned schema policy is incomplete. | VERIFIED |
| Theme | No verified theme extension contract found. | Repository search | Template | Planned/product meaning is unclear. | PLANNED_NOT_IMPLEMENTED |
| Hook | Named callback boundary; CMS hooks execute built-ins, Marketplace hooks are declarative authorizations. | plugins/mod.rs; marketplace_adapters.rs | Event, callback | Delivery differs by mechanism. | VERIFIED |
| Event | Audit, webhook, or internal workflow signal; not automatically a plugin hook. | audit.rs; webhooks.rs | Hook, queue message | No central extension event bus exists. | AMBIGUOUS |
| Marketplace item | Reviewed listing/version exposed through catalog rules. | routes/marketplace.rs | Package, installation | Item is documentation vocabulary, not a schema type. | INFERRED_FROM_CODE |
| Installation | Organization-owned version pin, lifecycle state, and permission snapshot. | marketplace_installation.rs | Activation, removal | It does not prove executable code is loaded. | VERIFIED |
| Registration | Static inclusion, database metadata sync, or declarative registry upsert depending on the mechanism. | plugins/mod.rs; routes/plugins.rs; marketplace_adapters.rs | Discovery, activation | There is no single universal registry. | VERIFIED |
| Activation | Global built-in enablement or organization installation state active. | routes/plugins.rs; marketplace_installation.rs | Enablement, installation | The two scopes must not be merged. | VERIFIED |
| Enablement | Persistent eligibility for invocation or adapter use. | cms_plugins; marketplace_installations | Activation, deactivation | Route/UI availability remains host-static. | VERIFIED |
| Deactivation | Built-in disabled flag or Marketplace disabled state; data is retained. | routes/plugins.rs; routes/marketplace.rs | Removal | Kill-switch blocking is a separate state. | VERIFIED |
| Removal | Marketplace soft uninstall; no built-in uninstall API exists. | marketplace_installation.rs | Deactivation, data retention | Derived-row and artifact cleanup are unclear. | VERIFIED |
| Compatibility | Manifest/host constraints and install gates; compiled plugins rely on same-build compatibility. | marketplace_manifest.rs; marketplace_installation.rs | Version, manifest | No general plugin ABI policy exists. | VERIFIED |
| Capability | One allowlisted host runtime operation mapped to a permission. | marketplace_runtime.rs | Permission | Authorization does not execute the operation. | VERIFIED |
| Permission | Marketplace dot-delimited capability request or separate CMS RBAC management authority. | marketplace_manifest.rs; rbac.rs | Capability, role | Permission vocabularies are distinct. | VERIFIED |
| Manifest | Marketplace manifest.json validated against schema version 2026-07. | marketplace_manifest.rs | Package, entry point | Built-in plugins do not use it. | VERIFIED |

Do not impose generic meanings over these project-specific definitions.
