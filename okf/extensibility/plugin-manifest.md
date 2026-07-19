---
okf_document_id: "plugin-manifest"
title: "Plugin Manifest"
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
  - "backend/src/services/marketplace_manifest.rs"
  - "docs/V3_MARKETPLACE_MANIFEST_SCHEMA.md"
  - "scripts/marketplace-cli.mjs"
related_documents:
  - "compatibility-and-versioning.md"
  - "plugin-permissions.md"
  - "marketplace-architecture.md"
related_diagrams:
  - "diagrams/plugin-registration-flow.mmd"
---

# Plugin Manifest

Built-in CmsPlugin implementations do not load a file manifest; they return metadata from Rust methods. Marketplace packages use manifest.json, validated by marketplace_manifest.rs.

## Required Marketplace Fields

| Field | Verified rule |
|---|---|
| manifest_version | Must equal 2026-07. |
| name | Required non-empty string. |
| version | Required semantic version. |
| product_type | component_pack, design_template, integration_plugin, or backend_extension. |
| permissions | Unique array containing only the nine allowlisted permission keys. |
| compatibility.min_zinhar_version | Required semantic version. |
| compatibility.max_zinhar_version | Optional semantic version. |
| entry_points | Required non-empty object. |
| assets | Required array. |

The validator does not require author, license, description, dependency, signature, migration, configuration-schema, or executable-runtime fields. Marketplace upload forms and listing records hold separate metadata; they must not be presented as manifest requirements.

Adapter code optionally reads top-level components, hooks, and template values. Their presence is product behavior, not part of the core required-field check.

INFERRED_FROM_MANIFEST: an entry point is treated as an artifact-local path for validation and authorization. It is not proof that the path is executed.

## Complete Field Matrix

| Field | Type | Required | Meaning | Validation | Default | Runtime consumer | Security sensitivity | Evidence |
|---|---|---|---|---|---|---|---|---|
| manifest_version | string | Yes | Marketplace schema version | Exact 2026-07 | None | Manifest validator | Contract downgrade/drift | marketplace_manifest.rs |
| plugin_id | Not defined | No | No manifest plugin identifier field exists | Not applicable | None | None | Identity must not be inferred | Field absent from validator |
| name | string | Yes | Package/product name | Non-empty | None | Validation and stored manifest | Display metadata | marketplace_manifest.rs |
| version | string | Yes | Package version | Semantic-version parser | None | Upload/install/version checks | Update/rollback selection | marketplace_manifest.rs |
| description | Not core-required | No | Listing description is stored separately | No core manifest rule found | None | Marketplace listing UI/API | Content moderation | routes/marketplace.rs |
| author | Not defined | No | Creator identity is stored outside manifest | Not applicable | None | Creator/listing records | Attribution and trust | Marketplace schema/routes |
| license | Not core-required | No | Listing license metadata is separate | Listing validation, not core manifest | None | Catalog/listing | Legal terms | routes/marketplace.rs |
| product_type | string | Yes | Product taxonomy | Four-value allowlist | None | Validation/install/adapters | Determines allowed operations | marketplace_manifest.rs |
| compatibility | object | Yes | Host range | Required valid min; optional valid max | No max | Catalog/install gates | Prevents unsupported install | marketplace_manifest.rs |
| entry_points | object | Yes, non-empty | Artifact-local declared entry paths | Object, non-empty, path safety/package existence elsewhere | None | Validation/runtime authorization | Traversal/execution boundary | manifest/runtime/validation services |
| frontend_entry | No dedicated field | No | May be represented by an entry_points key | Safe local path if used | None | Policy only; no loader | Script execution risk | No frontend loader found |
| backend_entry | No dedicated field | No | May be represented by an entry_points key | Safe local path if used | None | Policy only; no loader | Server execution risk | No backend loader found |
| permissions | array of strings | Yes | Requested host capabilities | Unique values from nine-key allowlist | Empty allowed only if validator accepts provided array | Install snapshot/runtime policy | Data/network authority | marketplace_manifest.rs |
| dependencies | Not core-required | No | No runtime dependency solver contract | Package validator may inspect metadata, but core field absent | None | None | Supply-chain risk | Core validator absence |
| configuration_schema | Not defined | No | No generic plugin config schema | Not applicable | None | None | Secret/schema handling | Validator absence |
| tenant_scope | Not defined in manifest | No | Installation owns tenant scope | Enforced by route/DB, not manifest | Organization installation | Installation/adapters | Cross-tenant access | routes and migrations |
| components | array | Adapter-optional | Declarative component definitions | Adapter validates object fields and props_schema | Empty/no definitions | Component adapter | Registry collision/schema input | marketplace_adapters.rs |
| hooks | array | Adapter-optional | Declarative public-hook definitions | Public type/key/label/config checks | Empty/no definitions | Hook adapter | Capability and UI injection | marketplace_adapters.rs |
| template | object | Adapter-optional | Declarative page template and assets | page_json object and asset mapping | Product-specific fallback key | Template adapter | Page/media writes | marketplace_adapters.rs |
| marketplace_metadata | Separate records | No | Listing, creator, pricing, screenshots, support | Route/domain validation | Database defaults | Catalog/frontend | Moderation/commercial data | Marketplace routes/migrations |
| integrity_or_signature | No manifest field | No | SHA-256 and size are stored with artifact version; no signature field | Artifact checksum/size/path verification | None | Upload/install | Supply-chain integrity | marketplace_package.rs; installation service |
| assets | array | Yes | Declared package assets | Array; package validation checks existence/path/limits | None | Validation/template mapping | Path and content input | marketplace_manifest.rs; validation service |
| migrations | Not defined | No | Package SQL is not supported | Not applicable | None | None | Database compromise risk | No migration runner found |
