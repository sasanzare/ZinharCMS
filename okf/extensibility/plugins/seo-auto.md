---
okf_document_id: "plugin-seo-auto"
title: "SEO Auto Generator Plugin"
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
  - "backend/src/plugins/seo.rs"
  - "backend/src/plugins/mod.rs"
  - "backend/src/routes/content.rs"
  - "backend/src/routes/plugins.rs"
plugin_id: "seo-auto"
plugin_name: "SEO Auto Generator"
plugin_type: "built_in_backend_plugin"
plugin_scope: "global_enablement_with_tenant_invocation_context"
implementation_status: "implemented"
compatibility_status: "compiled_with_host_no_declared_range"
related_documents:
  - "../extensibility-catalog.md"
  - "../plugin-architecture.md"
  - "../plugin-lifecycle.md"
  - "../extension-points/cms-entry-before-save.md"
related_diagrams:
  - "../diagrams/plugin-registration-flow.mmd"
  - "../diagrams/plugin-lifecycle.mmd"
---

# SEO Auto Generator Plugin

## Plugin Identity

| Field | Value |
|---|---|
| Key | seo-auto |
| Name | SEO Auto Generator |
| Reported version | 1.0.0 |
| Type | Trusted compiled Rust built-in |
| Scope | Global enable flag; tenant context at callback |
| Hook | entry.before_save |
| Source path | backend/src/plugins/seo.rs |
| Implementation status | IMPLEMENTED |
| Confidence | High |

## Purpose and Behavior

When an entry has no slug, the plugin derives an ASCII-normalized lowercase slug from the title, collapses non-alphanumeric runs to hyphens, trims hyphens, and uses untitled when no ASCII slug remains. An existing slug is preserved.

## Purpose

Provide a verified before-save default for missing content-entry slugs.

## Manifest

SEO Auto has no file manifest. Metadata is returned by Rust methods. [Plugin Manifest](../plugin-manifest.md) documents the separate Marketplace manifest contract.

## Registration and Execution

SeoAutoPlugin is instantiated directly by builtin_plugins(). Plugin-list synchronization upserts metadata into cms_plugins. Content create/update call the synchronous before-save runner after constructing EntryData and before database persistence.

## Configuration, Permissions, and Data

No plugin-specific configuration is consumed, no permission declaration exists, and no dedicated storage or migration is present. The callback runs with host-process authority. PluginContext contains user_id and org_id, but SEO Auto does not use them.

## Compatibility

Version 1.0.0 is metadata only. Compatibility follows same-repository compilation; no host-version range or plugin ABI contract exists.

## Tests

Two unit tests cover ASCII slug normalization and missing-slug mutation. No route/database integration, disablement, error-propagation, Unicode policy, or tenant-context test was found.

## Lifecycle

Installation is build/deploy registration. Metadata is synchronized through the plugin-list route. Activation/deactivation uses the global enabled flag. Update requires code deployment and metadata sync. No application uninstall exists; source removal can leave stale registry metadata.

## Extension Points Used

- [EP-001 CMS Entry Before-Save](../extension-points/cms-entry-before-save.md)
- It does not use EP-002 Entry After-Publish.

## Configuration

No setting group is consumed. Global cms_plugins.settings may be persisted but is disconnected from the callback.

## Permissions

No manifest capability is requested. The callback executes as trusted host code. Global plugin-manager RBAC protects enable/disable/update APIs.

## Tenant Scope

Enablement is GLOBAL. Invocation receives tenant/user context but the implementation ignores it and applies to entries in every tenant while enabled.

## Data Ownership

No plugin-owned table, file, asset, migration, or configuration schema exists. The plugin mutates shared EntryData.slug before the host persists content_entries.

## Frontend Surface

No route, page, menu, component, block, or dynamic frontend module is registered. Generic plugin administration can display its metadata/state.

## Backend Surface

The Rust implementation and static registry are the only plugin-specific backend surfaces. Generic plugin routes manage metadata and enablement; content routes trigger the callback. No job/event queue is registered.

## Isolation and Trust

The plugin is trusted, synchronous, in-process code with NO_RUNTIME_ISOLATION.

## Risks and Unknowns

Unicode-only titles become untitled; product intent is NEEDS_OWNER_CONFIRMATION. Settings exposure is not connected to behavior. Global enablement affects all tenants. See the [Extensibility Catalog](../extensibility-catalog.md).
