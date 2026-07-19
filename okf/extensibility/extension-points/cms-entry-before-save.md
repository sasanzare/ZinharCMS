---
okf_document_id: "extension-point-cms-entry-before-save"
title: "CMS Entry Before-Save Extension Point"
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
  - "backend/src/routes/content.rs"
  - "backend/src/plugins/seo.rs"
extension_point_id: "EP-001"
extension_point_name: "CMS Entry Before-Save"
extension_point_category: "backend_lifecycle_hook"
registration_type: "static_compiled_trait"
implementation_status: "implemented"
related_documents:
  - "../extension-points.md"
  - "../plugins/seo-auto.md"
  - "../hooks-and-events.md"
related_diagrams:
  - "../diagrams/plugin-registration-flow.mmd"
---

# CMS Entry Before-Save Extension Point

## Identity

EP-001 is the CmsPlugin.on_entry_before_save callback and the entry.before_save hook name.

| Identity field | Value |
|---|---|
| Extension-point ID | EP-001 |
| Name | CMS Entry Before-Save |
| Category | Backend lifecycle hook |
| Source paths | backend/src/plugins/mod.rs; backend/src/routes/content.rs |
| Implementation status | IMPLEMENTED |
| Confidence | High |

## Purpose

Allow trusted compiled plugins to inspect or mutate EntryData before create/update persistence.

## Contract

Input is mutable EntryData plus PluginContext containing optional user and organization IDs. Output is Result; an error aborts the calling request path.

## Registration

Implement CmsPlugin, report the hook name, and add the object to builtin_plugins(). Database enablement alone cannot add code.

- Registrar: backend developer through source control and deployment.
- Registration type: static/build-time.
- Duplicate behavior: no explicit duplicate plugin-key check was found; the database key is unique during metadata sync.
- Ordering: builtin_plugins() vector order.
- Naming: plugin keys use lowercase slug validation in management routes; the hook name is entry.before_save.

## Execution

Create and update handlers invoke the runner synchronously. Enabled keys are read from cms_plugins, built-ins run in vector order, and mutations feed validation/persistence.

Execution shares the request task and host state. A Result error aborts the caller; there is no retry, timeout, failure state, or isolation. The verified side effect is mutation of EntryData before host persistence.

## Security

The callback is in-process trusted code without capability isolation. Tenant context is informative, not a sandbox.

## Compatibility

The Rust trait is compiled with the host. No external ABI/version negotiation exists.

## Tests

SEO Auto tests exercise one subscriber. No real route, transaction, ordering, multi-plugin, failure, or disablement integration test was found.

## Risks and Unknowns

A callback can add latency, fail the request, or mutate business data. Ordering becomes observable if more plugins are added. See [Extension Points](../extension-points.md).
