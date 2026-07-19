---
okf_document_id: "marketplace-area-runtime-permissions-adapters"
title: "Marketplace Runtime Permissions and Host Adapters"
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
  - "backend/src/services/marketplace_runtime.rs"
  - "backend/src/routes/marketplace_runtime.rs"
  - "backend/src/services/marketplace_adapters.rs"
  - "backend/src/routes/marketplace_adapters.rs"
  - "backend/migrations/0020_v3_phase_seven_permission_sandbox_kill_switch.sql"
  - "backend/migrations/0021_v3_phase_eight_runtime_adapters.sql"
marketplace_area_id: "MPA-004"
marketplace_area_name: "Runtime Permissions and Host Adapters"
implementation_status: "authorization_and_declarative_adapters_implemented"
related_documents:
  - "../marketplace-architecture.md"
  - "../plugin-permissions.md"
  - "../isolation-and-trust.md"
  - "../extension-points/marketplace-runtime-authorization.md"
related_diagrams:
  - "../diagrams/plugin-permission-flow.mmd"
  - "../diagrams/component-registration.mmd"
---

# Marketplace Runtime Permissions and Host Adapters

The permission catalog and runtime policy bind host operations to one required approved permission, product types, and safe manifest entry points. Active/ready state, payload size, and kill switches are checked.

Host adapters implement three declarative surfaces: component definition synchronization, design-template preview/import, and public hook listing/authorization. The host performs database and Page Builder operations. Uploaded code is not invoked.

The runtime response explicitly states not_executed. Therefore this area is implemented as policy and declarative adapters, but arbitrary plugin runtime, isolation, scheduling, resource quotas, and hook delivery remain MARKETPLACE_NOT_IMPLEMENTED or HOOK_DELIVERY_UNCLEAR.

See [Marketplace Architecture](../marketplace-architecture.md).

## Purpose

Constrain declarative Marketplace capabilities and translate supported declarations into trusted host actions.

## Entities

marketplace_permission_catalog, marketplace_installations permission/runtime fields, marketplace_kill_switches, component_registry installation link, marketplace_template_imports, and marketplace_plugin_hooks.

## Backend Module

marketplace_runtime.rs plus runtime routes and marketplace_adapters.rs plus adapter routes.

## APIs

Permission catalog, runtime status/authorize, kill-switch actions, runtime components, template preview/import, hook list, and hook authorize.

## Frontend Feature

MarketplacePage manages runtime state and permissions; PagesPage consumes components and design templates. No public-hook renderer was found.

## Permissions

Exact installation snapshot, static operation-to-capability mapping, product type, safe entry point, 64-KiB payload cap, runtime readiness, and kill switches. Host Page Builder operations also enforce normal RBAC/RLS/quota.

## Tenant Scope

Installations, adapter reads/writes, components, imports, and hooks are organization-scoped; permission definitions/global kill switches are platform scope.

## Workflows

MP-WF-10, MP-WF-11, and the runtime checks inside MP-WF-06 through MP-WF-09.

## Tests

Runtime policy and adapter extraction/mapping tests cover allowed/denied cases. Real host-operation execution, hook delivery, registry cleanup, and DB/RLS integration remain gaps.

## Risks

The policy is not a sandbox. HOOK_DELIVERY_UNCLEAR, COMPONENT_REGISTRATION_UNCLEAR, and PLUGIN_ISOLATION_UNVERIFIED remain.

## Implementation Status

PARTIALLY_IMPLEMENTED: authorization and declarative host adapters exist; uploaded package code execution does not.
