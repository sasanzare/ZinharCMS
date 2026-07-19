---
okf_document_id: "extension-point-marketplace-public-hook-adapter"
title: "Marketplace Public Hook Adapter Extension Point"
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
  - "backend/src/services/marketplace_adapters.rs"
  - "backend/src/routes/marketplace_adapters.rs"
  - "backend/migrations/0021_v3_phase_eight_runtime_adapters.sql"
extension_point_id: "EP-005"
extension_point_name: "Marketplace Public Hook Adapter"
extension_point_category: "declarative_host_hook"
registration_type: "active_installation_manifest"
implementation_status: "authorization_only"
related_documents:
  - "../extension-points.md"
  - "../hooks-and-events.md"
  - "../marketplace/runtime-permissions-adapters.md"
related_diagrams:
  - "../diagrams/plugin-permission-flow.mmd"
---

# Marketplace Public Hook Adapter Extension Point

## Identity

EP-005 exposes sidebar.item, dashboard.widget, form.field, and webhook.adapter definitions from active installation manifests.

| Identity field | Value |
|---|---|
| Extension-point ID | EP-005 |
| Name | Marketplace Public Hook Adapter |
| Category | Declarative host hook |
| Source paths | marketplace_adapters route/service; migration 0021 |
| Implementation status | AUTHORIZATION_ONLY |
| Confidence | High for definition/authorization; low for delivery |

## Purpose

Provide a constrained host contract for discoverable Marketplace integration declarations.

## Contract

Each definition has a public type, key, label, JSON config, and contract version. The route validates requested hook type and context.

## Registration

Definitions are read from active, runtime-ready tenant installation manifests. The current base install gate does not admit new integration_plugin installations, so practical population may depend on pre-existing data or future gate changes.

Creators declare hooks, review approves the version, and tenant installation makes declarations discoverable. Registration is dynamic metadata. Public type and key naming are validated; marketplace_plugin_hooks has per-organization, installation, and hook-key uniqueness. No delivery order or cross-installation duplicate-resolution contract exists.

## Execution

The list endpoint returns definitions. The authorize endpoint checks matching installation/hook/runtime policy and returns allowed or denied with execution: not_executed. No third-party hook callback is invoked.

Execution is a synchronous host authorization request sharing tenant database/runtime state. Denial returns an error/decision; no retry or package side effect occurs.

## Security

Tenant ownership, runtime readiness, public-type allowlist, permission policy, safe entry point, payload size, and kill switches constrain authorization.

## Compatibility

Contract version defaults to 2026-07. No negotiated upgrade behavior or deprecated contract handling was found.

## Tests

Definition extraction and selected authorization helpers are tested. No frontend hook renderer or end-to-end delivery test was found.

## Risks and Unknowns

HOOK_DELIVERY_UNCLEAR and MARKETPLACE_BEHAVIOR_UNCLEAR apply. Product UI must not represent authorization as execution. See [Extension Points](../extension-points.md).
