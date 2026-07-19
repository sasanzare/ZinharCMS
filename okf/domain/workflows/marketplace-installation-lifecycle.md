---
okf_document_id: "workflow-marketplace-installation-lifecycle"
title: "Marketplace Installation Lifecycle Workflow"
project: "ZinharCMS"
category: "domain-workflow"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
workflow_id: "WF-MARKET-INSTALL"
workflow_name: "Marketplace Installation Lifecycle"
workflow_domain: "DOM-MARKETPLACE"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "backend/src/routes/marketplace.rs"
  - "backend/src/routes/marketplace_runtime.rs"
  - "backend/src/services/marketplace_installation.rs"
  - "backend/src/services/marketplace_runtime.rs"
related_documents:
  - "../cross-module-workflows.md"
  - "../domains/marketplace.md"
  - "../../security/permissions/marketplace-management.md"
related_diagrams:
  - "../diagrams/page-builder-workflow.mmd"
---

# Marketplace Installation Lifecycle Workflow

## Workflow Identity

- ID/name/domain: `WF-MARKET-INSTALL`, Marketplace Installation Lifecycle, `DOM-MARKETPLACE`.
- Trigger/actor: tenant Marketplace manager explicitly installs, updates, rolls back, disables/enables, uninstalls, or operates kill switch.
- Status/confidence: `IMPLEMENTED`; High.

## Preconditions

Authentication, active tenant, required organization role, supported product type, approved/install-eligible version, no conflicting active installation, exact permission approval, organization confirmation, and entitlement for paid product.

## Main Flow

1. Load listing/version/catalog compatibility and manifest permissions.
2. Verify tenant role, product type, eligibility/entitlement, and explicit confirmations.
3. Require approved permission list exactly matching manifest.
4. Create pinned active installation and permission records.
5. Runtime authorization checks installation active, runtime ready, operation known/declared, and permission approved.
6. Update requires explicit target version/changelog confirmation; permission changes require reapproval.
7. Rollback selects an allowed earlier version explicitly.
8. Disable blocks use without uninstall; enable restores allowed active use.
9. Kill switch toggles runtime `ready`/`blocked` independently with reason.
10. Uninstall changes lifecycle while preserving organization data according to persisted cleanup policy.

## Alternative Flows

Active component packs extend Page Builder palette. Design templates preview asset requirements and import through host adapter. Public hooks require registered safe hook type and runtime permission.

## Failure Flows

Role, tenant, duplicate, unsupported product, compatibility, entitlement, confirmation, permission mismatch/escalation, blocked/disabled/runtime, payload, or adapter validation rejects. No automatic fallback update executes.

## State Changes

Installation `active`, `disabled`, `uninstalled`, `rollback_pending`, `blocked`; runtime `ready`/`blocked`; approved permission and pinned version metadata change explicitly.

## Data Changes

Installation/permission/kill-switch/update/rollback/import/hook records plus audit/history depending on operation.

## Transaction Boundaries

Lifecycle service uses operation-specific transactions. Page/template imports cross Pages and Marketplace tables and must be reviewed at handler call sites.

## Side Effects

Page Builder component availability, template-created pages/assets mappings, public host-hook authorization. No uploaded package code execution or background auto-update.

## Completion Criteria

Requested explicit lifecycle state/version/permission set persists and runtime status reflects it.

## Tests

Strong service tests for permissions, changes, lifecycle, semantic versions, supported products, concurrency helpers; frontend tests for gates, duplicates, blocked state, reapproval, kill switch.

## Unknowns and Risks

Full live RLS/DB adapter workflow, recovery from partial template import, compatibility change policy, cleanup semantics, and future sandbox/runtime remain Phase 9 topics.

Return to [Cross-Module Workflows](../cross-module-workflows.md).

