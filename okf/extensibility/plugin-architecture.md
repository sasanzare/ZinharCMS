---
okf_document_id: "plugin-architecture"
title: "Plugin Architecture"
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
  - "backend/src/plugins/seo.rs"
  - "backend/src/routes/plugins.rs"
  - "backend/src/routes/content.rs"
  - "backend/migrations/0006_phase_six_workflow_collaboration.sql"
related_documents:
  - "plugin-discovery-and-registration.md"
  - "plugin-lifecycle.md"
  - "plugin-permissions.md"
  - "plugin-data-and-migrations.md"
  - "compatibility-and-versioning.md"
  - "isolation-and-trust.md"
related_diagrams:
  - "diagrams/plugin-registration-flow.mmd"
  - "diagrams/plugin-lifecycle.mmd"
---

# Plugin Architecture

## Components

| Component | Responsibility |
|---|---|
| CmsPlugin | Metadata and two entry lifecycle callback methods. |
| builtin_plugins() | Constructs the fixed set of executable plugin objects. |
| cms_plugins | Stores synchronized metadata, hook names, settings, is_enabled, and is_system. |
| Plugin routes | Synchronize, list, inspect, update metadata/settings, enable, and disable global plugin records. |
| Content routes | Invoke before-save and after-publish runners at verified orchestration points. |

## Invocation Model

Each runner queries enabled keys, constructs all compiled built-ins, filters by key, and calls callbacks in vector order. The before-save callback can mutate EntryData. The after-publish callback receives an immutable entry. Errors propagate through content orchestration; no retry, timeout, circuit breaker, asynchronous queue, or per-plugin failure isolation was found.

## Architectural Boundaries

This is not a dynamic plugin host. A new executable plugin requires Rust implementation, inclusion in builtin_plugins(), compilation, deployment, and subsequent registry synchronization. The database row does not supply executable code.

PLUGIN_REGISTRATION_UNCLEAR is resolved for the current built-in mechanism: registration is explicit in source. PLUGIN_LIFECYCLE_UNCLEAR remains for upgrade/removal semantics across deployments because registry reconciliation never deletes absent rows. See [Registration](plugin-discovery-and-registration.md), [Lifecycle](plugin-lifecycle.md), [Permissions](plugin-permissions.md), [Data](plugin-data-and-migrations.md), [Compatibility](compatibility-and-versioning.md), and [Isolation](isolation-and-trust.md).

## Architecture Responsibility Matrix

| Concern | Verified behavior | Status |
|---|---|---|
| Plugin host | Axum backend content orchestration and plugin runners | VERIFIED |
| Plugin interface | CmsPlugin trait with metadata and two callbacks | VERIFIED |
| Plugin package | No package format for built-ins; Marketplace package is declarative input | MIXED |
| Plugin identity | Static key, name, version, description, and hooks | VERIFIED |
| Plugin metadata | Synchronized to global cms_plugins | VERIFIED |
| Plugin loading | Compiled into application; no dynamic loader | VERIFIED |
| Plugin registration | Explicit builtin_plugins() vector | VERIFIED |
| Plugin execution | Synchronous and in-process | VERIFIED |
| Plugin state | Global enabled flag plus metadata/settings | VERIFIED |
| Plugin configuration | JSON settings stored but not passed to callback | PLUGIN_CONFIGURATION_UNCLEAR |
| Plugin persistence | Global registry; SEO mutates shared entry data | VERIFIED |
| Plugin permissions | Management RBAC only; callback has host authority | PLUGIN_PERMISSION_UNCLEAR |
| Plugin lifecycle | Sync, enable, disable, metadata update; no uninstall | PARTIALLY_IMPLEMENTED |
| Plugin errors | Result propagates to content handler | VERIFIED |
| Plugin logs | No plugin-specific logging or telemetry contract found | UNKNOWN |
| Plugin tests | SEO unit tests; no registry/route integration suite | PARTIALLY_IMPLEMENTED |
| Trust boundary | Trusted process-local application code | NO_RUNTIME_ISOLATION |

Plugins are compiled into the application, executed in-process, and represented in the database by metadata. They are not loaded from packages, dynamically linked, or process-isolated.
