---
okf_document_id: "marketplace-area-installation-update-rollback"
title: "Marketplace Installation, Update, and Rollback"
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
  - "backend/src/routes/marketplace.rs"
  - "backend/src/services/marketplace_installation.rs"
  - "backend/migrations/0019_v3_phase_six_installation_lifecycle.sql"
marketplace_area_id: "MPA-003"
marketplace_area_name: "Installation Update and Rollback"
implementation_status: "implemented_with_product_type_gate"
related_documents:
  - "../marketplace-architecture.md"
  - "../installation-and-removal.md"
  - "../plugin-lifecycle.md"
related_diagrams:
  - "../diagrams/marketplace-installation-flow.mmd"
---

# Marketplace Installation, Update, and Rollback

Install is organization-owned, RBAC-protected, artifact-verified, version-pinned, permission-approved, kill-switch-aware, and audited. Paid products require a matching completed entitlement. The current MVP product-type gate admits only component packs and design templates.

Enable, disable, and soft uninstall use explicit lifecycle transitions. Update requires a newer compatible approved version and changelog confirmation. A changed permission set requires an authorized exact approval snapshot. Rollback uses a same-listing stored target, validates compatibility/artifact, and checks retained permissions.

Uninstall preserves organization data and history. Automatic update is not implemented. Database-backed concurrency, repeated-action idempotency, and derived component cleanup remain testing gaps.

See [Marketplace Architecture](../marketplace-architecture.md).

## Purpose

Maintain an auditable tenant-owned, explicitly versioned product installation lifecycle without executing uploaded code.

## Entities

marketplace_installations, listing/version, entitlement, permission snapshot, rollback version, runtime state, and audit records.

## Backend Module

routes/marketplace.rs, marketplace_installation.rs, marketplace_runtime.rs, quota, RBAC, RLS, and audit services.

## APIs

List/install, update check, enable, disable, uninstall, update, rollback, runtime status, and authorization routes.

## Frontend Feature

MarketplacePage displays installed products, permission confirmation, compatibility/update data, lifecycle controls, and runtime state.

## Permissions

Organization Marketplace installer manages lifecycle; an authorized approver confirms exact permissions or changes; paid paths require entitlement.

## Tenant Scope

Installation state is organization-owned under tenant context/RLS. Global/organization kill switches can deny installation or runtime eligibility.

## Workflows

MP-WF-06 through MP-WF-09.

## Tests

Installation, runtime, route helper, artifact filesystem, and MarketplacePage tests cover transitions and selected denials; no full live tenant DB/concurrency suite exists.

## Risks

Repeated-action concurrency, derived component cleanup, artifact retention, provider entitlement timing, and live RLS remain gaps.

## Implementation Status

IMPLEMENTED_WITH_PRODUCT_TYPE_GATE: component_pack and design_template are installable; integration_plugin and backend_extension are rejected by the base gate.
