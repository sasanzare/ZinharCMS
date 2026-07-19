---
okf_document_id: "domain-plugins-components"
title: "Plugins and Components Domain"
project: "ZinharCMS"
category: "domain-detail"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
domain_id: "DOM-PLUGIN"
domain_name: "Plugins and Components"
domain_status: "PARTIALLY_IMPLEMENTED"
boundary_status: "OVERLAPPING"
primary_sources:
  - "backend/src/plugins"
  - "backend/src/routes/plugins.rs"
  - "backend/src/routes/pages.rs"
  - "backend/src/services/marketplace_adapters.rs"
  - "backend/migrations/0006_phase_six_workflow_collaboration.sql"
related_documents:
  - "../domain-catalog.md"
  - "../page-builder-rules.md"
  - "../../security/permissions/marketplace-runtime-capabilities.md"
related_diagrams:
  - "../diagrams/page-builder-workflow.mmd"
---

# Plugins and Components Domain

## Domain Identity

- Domain ID: `DOM-PLUGIN`
- Terminology: `CmsPlugin`, built-in plugin, hook, component registry, `component_key`, Marketplace component/template/public hook adapter.
- Implementation: `PARTIALLY_IMPLEMENTED`; boundary `OVERLAPPING`; confidence High.

## Responsibility

- Verified: compile/register built-in CMS plugins, enable/disable them globally, invoke content hooks, manage system/tenant component definitions, and expose host-owned Marketplace adapters.
- Inferred: component definitions are the Page Builder vocabulary.
- Shared: Content save/publish, Pages validation/UI, Marketplace installation/runtime permissions.
- Unclear: tenant-specific plugin settings, compatibility/version policy, arbitrary third-party execution, and component prop runtime semantics.

## Core Entities

`cms_plugins`, `component_registry`, Marketplace installations/permissions/template imports/hook registrations, and compiled plugin implementations.

## Core Services

Plugin route/trait, SEO plugin, pages component handlers, Marketplace adapters/runtime policy.

## API Surface

Global plugin read/enable/disable, tenant component registry CRUD, and Marketplace component/template/hook endpoints.

## Frontend Surface

Workflow plugin toggle panel, Page Builder palette/property controls, and Marketplace adapter/template controls.

## Actors

Global plugin reader/manager, organization component/page manager, tenant Marketplace manager, and content writer/publisher whose action triggers hooks.

## Business Rules

`BR-PLUGIN-001` through `BR-PLUGIN-003`, plus `BR-PAGE-005` and Marketplace runtime rules.

## Invariants

Unique slug-shaped plugin/component keys, object prop schemas, system component write protection, exact Marketplace permission approval, and non-execution of uploaded code in current architecture.

## State and Lifecycle

Built-in plugin state is enabled/disabled. Component rows are created/updated/hard-deleted. Marketplace adapters depend on active/pinned installations and ready runtime state.

## Access Rules

Global plugin RBAC is separate from tenant component/Page Builder RBAC. Marketplace adapter operations require tenant role, installation, manifest permission, and runtime policy.

## Validation Rules

Plugin key syntax, component key/category/props schemas, Marketplace manifest permissions, namespacing, public hook types, and template asset mapping.

## Workflows

[Content Entry Save](../workflows/content-entry-save.md), [Page Builder Save](../workflows/page-builder-save-and-version.md), and [Marketplace Installation Lifecycle](../workflows/marketplace-installation-lifecycle.md).

## Side Effects

Synchronous entry mutation/hook execution, global plugin metadata synchronization, component DB writes, template page/import writes, and runtime authorization records.

## Tests

SEO hook, Marketplace adapter/runtime, permission, and frontend installation tests exist. Built-in enablement scope, content-route orchestration, component CRUD/validation, template transaction, and Page Builder runtime rendering are gaps.

## Risks and Unknowns

Global plugin scope, no general sandbox, prop-schema enforcement gap, deleted component references, boundary overlap with Marketplace/Pages, and uploaded package non-execution relying on host-owned architecture rather than a general runtime.

Return to the [Domain Catalog](../domain-catalog.md).

