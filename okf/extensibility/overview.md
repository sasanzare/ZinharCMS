---
okf_document_id: "extensibility-overview"
title: "Extensibility Overview"
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
  - "backend/src/plugins/seo.rs"
  - "backend/src/services/marketplace_manifest.rs"
  - "backend/src/services/marketplace_runtime.rs"
  - "backend/src/services/marketplace_adapters.rs"
  - "backend/src/routes/marketplace_adapters.rs"
related_documents:
  - "extensibility-catalog.md"
  - "plugin-architecture.md"
  - "marketplace-architecture.md"
related_diagrams:
  - "diagrams/extensibility-context.mmd"
---

# Extensibility Overview

## Extensibility Status

| Classification | Status | Evidence |
|---|---|---|
| STATIC_EXTENSION_REGISTRATION | VERIFIED | builtin_plugins() constructs a fixed Rust vector. |
| BUILT_IN_EXTENSIONS_ONLY | VERIFIED | Only SeoAutoPlugin implements the inspected executable plugin contract. |
| DYNAMIC_PLUGIN_LOADING | Not implemented | No shared-library, WASM, script, archive, or process loader was found. |
| COMPONENT_REGISTRY_ONLY | VERIFIED | Page Builder and Marketplace components are database/manifest schemas consumed by host code. |
| MARKETPLACE_INTEGRATED | VERIFIED | Creator, catalog, review, install, update, finance, feedback, analytics, runtime policy, and adapter routes are registered. |
| PARTIALLY_IMPLEMENTED | VERIFIED | Marketplace supports declarative integrations but does not execute uploaded package code. |

The repository contains three mechanisms that must not be conflated:

1. A global, trusted, statically compiled CMS plugin registry.
2. A tenant-aware Page Builder component metadata registry.
3. A Marketplace lifecycle and declarative host-adapter system.

## Main Extensibility Areas

| Area | Verified mechanism | Classification |
|---|---|---|
| Backend hooks | Two CmsPlugin entry callbacks | Static, trusted, in-process |
| Backend routes and services | Fixed host modules for plugin administration and Marketplace | Internal modules, not plugin-registered routes |
| Page Builder blocks | System, tenant, and Marketplace-derived component metadata | COMPONENT_REGISTRY_ONLY |
| Frontend components | Host React UI consumes registry and Marketplace APIs | Host-rendered; no dynamic module loader |
| Settings | Global cms_plugins.settings plus distributed Marketplace metadata | Partially consumed |
| Events and hooks | CMS callbacks, declarative public hooks, audit/webhook mechanisms | Mixed execution semantics |
| Marketplace packages | Validated artifacts, versions, installations, host adapters | MARKETPLACE_INTEGRATED and PARTIALLY_IMPLEMENTED |
| Storage/auth/search/media providers | No public plugin registration contract found | EXTENSION_POINT_UNVERIFIED |

## Verified Inventory

- Concrete executable plugins: **1** (seo-auto).
- CMS hook contracts: **2** (entry.before_save and entry.after_publish).
- Public Marketplace hook types: **4** (sidebar.item, dashboard.widget, form.field, webhook.adapter).
- Marketplace product types accepted by the manifest validator: **4**.
- Product types admitted by the inspected base installation gate: **2** (component_pack and design_template).
- Runtime permission keys: **9**.
- Runtime operation definitions: **10** host operations mapped to nine permissions and supported product types.
- Dedicated significant extension-point documents: **6**.

## Runtime Shape

The CMS calls enabled built-ins synchronously inside entry create/update and publish orchestration. Marketplace installation stores an organization-owned version and permission snapshot. The runtime authorization service verifies policy and returns execution: not_executed. Host adapters extract JSON component, template, and hook definitions and perform host-owned actions.

## Main Runtime Components

- The Axum host and CmsPlugin trait execute compiled callbacks.
- builtin_plugins() supplies the build-time registry.
- cms_plugins supplies global metadata and enablement.
- Marketplace routes/services own upload, review, install, finance, feedback, analytics, policy, and adapters.
- PostgreSQL owns catalog and tenant installation state; the upload directory owns package artifacts.
- PagesPage and MarketplacePage are fixed host consumers.

## Trust Model

Built-in plugins are fully trusted, static build-time code executed in-process without isolation. Marketplace artifacts are untrusted input that is validated and stored. Declarative component, template, and hook data is interpreted by trusted host code. No out-of-process, WASM, container, or dynamic-package execution boundary was found.

## Main Unknowns

- PLUGIN_LIFECYCLE_UNCLEAR: stale built-in registry reconciliation and code removal.
- PLUGIN_CONFIGURATION_UNCLEAR: runtime consumption of stored plugin settings.
- PLUGIN_COMPATIBILITY_UNCLEAR: stable CmsPlugin API and host-version policy.
- COMPONENT_REGISTRATION_UNCLEAR: derived-row cleanup, collisions, and renderer completeness.
- HOOK_DELIVERY_UNCLEAR: public Marketplace hook execution/rendering.
- PLUGIN_TRUST_MODEL_UNCLEAR: signing and isolation requirements for any future executor.
- MARKETPLACE_BEHAVIOR_UNCLEAR: deployed provider/storage state and live end-to-end behavior.

## Status Boundaries

- VERIFIED: current behavior is directly supported by inspected source or migrations.
- INFERRED_FROM_CODE: intent is not stated, but consistent code structure supports the conclusion.
- UNKNOWN or a status ending in UNCLEAR: current evidence cannot establish a guarantee.
- PLANNED_NOT_IMPLEMENTED: historical or taxonomy documents describe a future capability absent from current runtime.

See [Terminology](terminology.md) for controlled vocabulary and [Risks](extensibility-risks.md) for unresolved boundaries.
