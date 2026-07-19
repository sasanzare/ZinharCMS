---
okf_document_id: "workflow-page-builder-save-version"
title: "Page Builder Save and Version Workflow"
project: "ZinharCMS"
category: "domain-workflow"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
workflow_id: "WF-PAGE-SAVE"
workflow_name: "Page Builder Save and Version"
workflow_domain: "DOM-PAGE"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "backend/src/routes/pages.rs"
  - "frontend/src/pages/PagesPage.tsx"
related_documents:
  - "../cross-module-workflows.md"
  - "../page-builder-rules.md"
  - "../../frontend/page-builder.md"
related_diagrams:
  - "../diagrams/page-builder-workflow.mmd"
---

# Page Builder Save and Version Workflow

## Workflow Identity

- ID/name/domain: `WF-PAGE-SAVE`, Page Builder Save and Version, `DOM-PAGE`.
- Trigger/actor: tenant page writer manually saves, or frontend autosaves an existing dirty draft.
- Status/confidence: `IMPLEMENTED`; High backend, Medium full UI workflow.

## Preconditions

Authentication, active tenant, page-writer role, content capacity for create, valid title/slug, object page document, registered component types, and bounded valid tree.

## Main Flow

1. Frontend loads pages, system/tenant components, active Marketplace components/templates, and installations.
2. User edits metadata/layout/properties in local draft.
3. Manual save is available; existing dirty pages also schedule a delayed save.
4. Backend validates title, slug, metadata, root, nodes, registry types, depth, and count.
5. Begin tenant transaction.
6. Insert/update page.
7. Insert snapshot numbered `MAX(version)+1` for current `page_json`.
8. Commit.
9. Broadcast page JSON to preview subscribers.
10. If updated page is published, invalidate public page cache.
11. Frontend replaces draft with response, clears dirty state, and reloads list/version context.

## Alternative Flows

Marketplace template import creates a page through a host adapter after asset mapping. Local preview renders the unsaved draft without backend WebSocket. New pages cannot autosave before an ID exists.

## Failure Flows

Combined frontend load uses one error state. Backend validation/uniqueness/quota/transaction failure preserves no committed page write. Post-commit broadcast/cache failure does not roll back. Frontend save failure leaves dirty/error state.

## State Changes

Create initializes draft. Ordinary save preserves workflow status. Published page content can be edited and saved without returning to draft or review.

## Data Changes

Page row and one page-version snapshot per successful create/update; optional Marketplace import records in the template path.

## Transaction Boundaries

Page and snapshot are atomic. Preview/cache and frontend reload are outside. Marketplace template import has its own route transaction rules.

## Side Effects

Preview broadcast, best-effort cache invalidation, and UI reload. No publication webhook on save.

## Completion Criteria

Page and snapshot commit and page response returns. Preview/cache are secondary effects.

## Tests

Frontend test covers shell/palette/property panel only. Save, autosave timing, backend validator, transaction rollback, snapshot numbering, published edit, preview, and cache paths are untested end-to-end.

## Unknowns and Risks

No optimistic lock, merge/dirty-navigation guard, snapshot concurrency race, prop-schema gap, local/public renderer parity, and published-edit review policy.

Return to [Cross-Module Workflows](../cross-module-workflows.md).

