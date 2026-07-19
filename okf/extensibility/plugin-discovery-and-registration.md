---
okf_document_id: "plugin-discovery-registration"
title: "Plugin Discovery and Registration"
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
  - "backend/src/routes/plugins.rs"
  - "backend/src/services/marketplace_adapters.rs"
  - "backend/src/routes/marketplace_adapters.rs"
related_documents:
  - "plugin-architecture.md"
  - "plugin-lifecycle.md"
  - "component-and-block-registration.md"
related_diagrams:
  - "diagrams/plugin-registration-flow.mmd"
---

# Plugin Discovery and Registration

## Built-in Plugins

There is no filesystem or package scan. builtin_plugins() creates a fixed vector of CmsPlugin objects. Listing plugins calls sync_builtin_plugins(), which upserts key, name, version, description, and hooks into cms_plugins. Conflict updates preserve the existing is_enabled value. Hook runners query enabled keys, then match those keys against the compiled vector.

| Question | Answer |
|---|---|
| Discovery source | Static Rust function |
| Registration timing | Build-time inclusion; metadata sync during list operation |
| Duplicate handling | Database key conflict updates metadata |
| Missing built-in cleanup | No deletion/reconciliation found |
| Load order | Current vector order |
| Dynamic packages | Not loaded |

## Marketplace Registration

Marketplace upload validates and stores an artifact/version. Installation pins an approved version and permissions. Component and hook adapters read manifest JSON for active, ready tenant installations. Component definitions are synchronized into component_registry; hook definitions are returned by host routes. No package code is registered into the process.

PLUGIN_REGISTRATION_UNCLEAR: package registration as executable code is not present. COMPONENT_REGISTRATION_UNCLEAR: stale registry-row cleanup needs owner confirmation; current upsert behavior is the observed contract.

## Discovery Mechanism Matrix

| Mechanism | Trigger | Source location | Validation | Duplicate handling | Ordering | Failure behavior | Logging/audit | Tenant scope | Tests | Confidence |
|---|---|---|---|---|---|---|---|---|---|---|
| Built-in source discovery | Backend build/start and each runner construction | backend/src/plugins/mod.rs | Rust compilation and trait conformance | No explicit duplicate-key check | builtin_plugins() vector order | Build fails or callback error propagates | General tracing only; no plugin log contract | Global code | SEO unit tests | High |
| Built-in metadata sync | GET plugin list | backend/src/routes/plugins.rs | Plugin key validation and SQL constraints | Key upsert updates metadata and preserves is_enabled | Vector/upsert order; response query order not a contract | API/DB error | No dedicated sync audit found | Global | No route test found | High |
| Marketplace manifest discovery | Version upload/submission | marketplace route and manifest service | Required fields, types, semver, permissions, paths/package checks | Version/listing constraints and database keys | Request-driven | Validation report and rejection | Submission/review events | Global catalog metadata | Manifest/package/validation tests | High |
| Marketplace installation registration | Tenant install request | routes/marketplace.rs | Eligibility, compatibility, entitlement, exact permissions, artifact, kill switch | Active-install uniqueness | One transaction | Transaction error; no code loaded | Installation audit | Tenant | Lifecycle/helper tests | High |
| Component definition registration | Components API call or Marketplace component list/sync | routes/pages.rs; routes/marketplace_adapters.rs | Key/schema and active-ready installation checks | Unique component_key and upsert | Query/manifest order not stable contract | Validation/DB error | Import/installation audit; no generic component audit guarantee | System or tenant | Adapter/frontend selected tests | Medium |
| Public hook registration | Active-ready manifest is read | marketplace adapters | Four-type allowlist, key/label/config | Per-installation stored key uniqueness; returned definition behavior | No delivery order because execution absent | Authorization denial | Runtime decision path | Tenant | Adapter tests | Medium |
| Directory/package runtime scan | None found | Repository-wide search | None | None | None | Not applicable | None | None | None | High confidence absent |
| Dynamic dependency injection | None found | Repository-wide search | None | None | None | Not applicable | None | None | None | High confidence absent |

PLUGIN_LOADING_UNCLEAR is resolved for current source as no dynamic runtime loader. Any future package executor remains PLANNED_NOT_IMPLEMENTED.
