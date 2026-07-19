---
okf_document_id: "extension-point-cms-entry-after-publish"
title: "CMS Entry After-Publish Extension Point"
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
  - "backend/src/plugins/mod.rs"
  - "backend/src/routes/content.rs"
extension_point_id: "EP-002"
extension_point_name: "CMS Entry After-Publish"
extension_point_category: "backend_lifecycle_hook"
registration_type: "static_compiled_trait"
implementation_status: "implemented_no_verified_subscriber"
related_documents:
  - "../extension-points.md"
  - "../hooks-and-events.md"
  - "../../domain/publication-workflow.md"
related_diagrams:
  - "../diagrams/plugin-registration-flow.mmd"
---

# CMS Entry After-Publish Extension Point

## Identity

EP-002 is CmsPlugin.on_entry_after_publish and the entry.after_publish runner.

| Identity field | Value |
|---|---|
| Extension-point ID | EP-002 |
| Name | CMS Entry After-Publish |
| Category | Backend lifecycle hook |
| Source paths | backend/src/plugins/mod.rs; backend/src/routes/content.rs |
| Implementation status | IMPLEMENTED_NO_VERIFIED_SUBSCRIBER |
| Confidence | High for contract; medium for lifecycle timing |

## Purpose

Notify trusted compiled plugins after an entry publication transition.

## Contract

Input is immutable EntryData plus PluginContext. Output is Result; failures propagate to the publish handler.

## Registration

The same static CmsPlugin registration and global enablement mechanism applies.

The backend developer registers source in builtin_plugins(); database rows cannot add implementations. Duplicate-key behavior is not explicitly checked in the vector, order is vector order, route key naming is lowercase slug form, and the hook name is entry.after_publish.

## Execution

The content publish path invokes enabled built-ins synchronously. No built-in plugin currently declares or overrides this hook, so the contract and call site are VERIFIED while subscriber behavior is absent.

Execution shares the host request context and state. A Result error propagates; no retry, timeout, isolation, or persisted failed state exists. No verified subscriber side effect exists.

## Security

Execution occurs in the backend process with no isolation or per-plugin permission boundary.

## Compatibility

Source-level Rust compilation is the only compatibility check.

## Tests

No subscriber, ordering, retry, idempotency, transaction-commit timing, or failure-path test was found.

## Risks and Unknowns

The name after_publish does not prove the database transaction has irreversibly committed before callback side effects. Side-effect durability and retry semantics are PLUGIN_LIFECYCLE_UNCLEAR. See [Extension Points](../extension-points.md).
