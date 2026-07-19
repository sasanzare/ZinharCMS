---
okf_document_id: "domain-pages-page-builder"
title: "Pages and Page Builder Domain"
project: "ZinharCMS"
category: "domain-detail"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
domain_id: "DOM-PAGE"
domain_name: "Pages and Page Builder"
domain_status: "IMPLEMENTED"
boundary_status: "OVERLAPPING"
primary_sources:
  - "backend/src/routes/pages.rs"
  - "frontend/src/pages/PagesPage.tsx"
  - "backend/migrations/0001_initial_schema.sql"
  - "backend/migrations/0004_phase_two_page_builder.sql"
related_documents:
  - "../domain-catalog.md"
  - "../page-builder-rules.md"
  - "../../frontend/page-builder.md"
related_diagrams:
  - "../diagrams/page-builder-workflow.mmd"
---

# Pages and Page Builder Domain

## Domain Identity

- Domain ID: `DOM-PAGE`
- Terminology: page, `page_json`, layout node, component registry, snapshot, preview channel, Page Builder.
- Implementation: `IMPLEMENTED`; boundary `OVERLAPPING`; confidence High.

## Responsibility

- Verified: persist pages, validate component trees, manage component definitions, create/restore snapshots, expose preview, and run editorial lifecycle.
- Inferred: `page_json` is the canonical current page document.
- Shared: workflow with Content; delivery cache/webhooks; Marketplace component/template adapters; frontend local renderer.
- Unclear: runtime renderer parity, prop-schema enforcement, hierarchy between pages, and concurrent editing policy.

## Core Entities

`pages`, `page_versions`, `component_registry`, process-local preview channels, and Marketplace adapter/import records.

## Core Services

Pages route, workflow, quota, RLS/RBAC, delivery, webhooks, Marketplace adapters, and frontend builder helpers.

## API Surface

Page CRUD/workflow/version, component registry, preview WebSocket, and Marketplace component/template adapter routes. See [Pages Endpoints](../../api/endpoints/pages-workflow-versions-and-preview.md).

## Frontend Surface

`PagesPage` owns list/editor, palette, canvas, property controls, preview, save/autosave, status actions, versions, templates, and WebSocket URL handoff.

## Actors

Organization page readers/writers/managers/publishers/reviewers, component managers, and authenticated preview users.

## Business Rules

`BR-PAGE-001` through `BR-PAGE-007`.

## Invariants

Tenant-unique slug, object documents, positive unique snapshot versions, registered component types, bounded tree, and system component protection.

## State and Lifecycle

Pages share the editorial state machine. Create/update/restore create snapshots; status changes do not. Hard delete cascades snapshots. Version restore creates a new draft snapshot.

## Access Rules

Tenant role capabilities guard writes. System components are read-only through tenant paths. Page `author_id` is attribution, not a resource owner ACL. Preview has separate token/organization query handling.

## Validation Rules

See [Page Builder Rules](../page-builder-rules.md). Backend recursively validates structural tree rules but not every component prop value against its schema.

## Workflows

[Page Builder Save and Version](../workflows/page-builder-save-and-version.md), [Page Version Restoration](../workflows/page-version-restoration.md), and [Editorial Publication](../workflows/editorial-publication.md).

## Side Effects

Transactional page/snapshot writes, in-process preview broadcast, public cache invalidation for published edits/transitions, webhooks, and deletion audit.

## Tests

Frontend tests cover builder shell/palette/property visibility only. Backend recursive validation, snapshots, restore, autosave, drag/drop, preview, publication, and concurrency lack end-to-end coverage.

## Risks and Unknowns

Snapshot numbering concurrency, no optimistic lock/merge UI, incomplete prop validation, process-local preview, deleted component references, and partial Page model/status drift.

Return to the [Domain Catalog](../domain-catalog.md).

