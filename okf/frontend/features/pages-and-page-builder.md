---
okf_document_id: "frontend-feature-pages-page-builder"
title: "Pages and Page Builder"
project: "ZinharCMS"
category: "frontend-feature"
phase: 4
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "7d25e4cbc53284a78033478e2681d8e9ebeb2fb1"
last_verified_date: "2026-07-17"
feature_id: "FE-FEAT-007"
feature_name: "Pages and Page Builder"
feature_paths:
  - "frontend/src/pages/PagesPage.tsx"
  - "frontend/src/pages/PagesPage.test.tsx"
  - "frontend/src/services/api.ts"
  - "frontend/src/types/api.ts"
  - "frontend/src/styles/index.css"
boundary_status: "OVERLAPPING"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "frontend/src/pages/PagesPage.tsx"
  - "frontend/src/pages/PagesPage.test.tsx"
  - "frontend/src/services/api.ts"
  - "frontend/src/types/api.ts"
related_documents:
  - "frontend/page-builder.md"
  - "frontend/state-management.md"
  - "frontend/forms-and-validation.md"
  - "frontend/features/editorial-workflow-and-collaboration.md"
  - "frontend/features/marketplace.md"
  - "backend/modules/pages-builder-preview.md"
  - "backend/modules/marketplace-runtime-adapters.md"
related_diagrams:
  - "frontend/diagrams/page-builder-flow.mmd"
  - "frontend/diagrams/frontend-api-flow.mmd"
uncertainty_markers:
  - "RESPONSIBILITY_OVERLAP FRO-01"
  - "AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01"
  - "UI_BEHAVIOR_UNVERIFIED UBU-01"
---

# Pages and Page Builder

## Feature Identity

| Field | Value |
|---|---|
| Feature ID | `FE-FEAT-007` |
| Application | `FE-APP-001` |
| Implementation | `IMPLEMENTED` |
| Boundary | `OVERLAPPING` |
| Confidence | High |
| Route | `/pages` |

## Responsibility

Provides page CRUD, visual composition, component property editing, local preview, delayed autosave, page snapshots and restore, workflow transitions, Marketplace component/template integration, and preview WebSocket URL handoff.

## Owned Source Areas

- Route and page-local builder components: `frontend/src/pages/PagesPage.tsx`.
- Selected UI test: `frontend/src/pages/PagesPage.test.tsx`.
- Page/component/adapter client methods and page JSON contracts: `services/api.ts`, `types/api.ts`.
- Builder layout rules: relevant sections of `styles/index.css`.

## Entry Points

- Protected `/pages` route and sidebar link.
- New/edit page controls, palette drag/add, canvas/property interactions.
- Save/autosave, workflow, versions, template, and preview-copy actions.
- Workflow page can also mutate page publication state.

## Internal Structure

One route file declares the page component plus palette, sortable node, canvas, local preview, preview node, and property-control components. Recursive helpers normalize, find, update, remove, and count page nodes. dnd-kit supplies pointer sensors and sortable behavior.

## State

Local pages, components, templates, previews, versions, draft page JSON, selected node/page, filter, drag ID, dirty/autosave, request, and error state. Global token and active organization are read only for preview URL construction. Existing pages autosave after ten seconds of dirty state.

## Backend Interactions

Uses pages, component registry, Marketplace adapter, and Marketplace installation methods. Saves normalized page JSON; loads/restores versions; transitions workflow; imports templates. It copies rather than opens the backend preview socket.

## Access Control

The route has no specific client role gate. Backend page mutation, workflow, component, template, and preview authorization remains authoritative. The copied preview URL carries client session context and is security-sensitive follow-up.

## UI Composition

Three-column builder with component palette, canvas plus local preview, and property/metadata panel; page metadata save row; Marketplace template panels; page list; version list; status and errors. Responsive and RTL rules are global.

## Loading and Error Behavior

Initial load combines four requests; one failure yields one page error. Save state distinguishes pending, saved, failed, and manual modes. Template/version/workflow failures also feed the same error. No conflict resolution, global retry, or request cancellation exists.

## Tests

One test verifies shell rendering and palette composition with mocked APIs. Critical drag/drop, autosave, persistence, workflow, restore, template, clipboard, and preview behavior is not covered.

## Known Risks and Unknowns

- `FRO-01`: Pages overlaps workflow and Marketplace adapters.
- One file owns many responsibilities.
- Local preview parity with delivery/runtime rendering is unverified.
- No client undo/redo or save-conflict contract was found.
- Preview query context and sparse tests are high-priority risks.

## Related Documents

- [Page Builder](../page-builder.md)
- [Editorial Workflow and Collaboration](editorial-workflow-and-collaboration.md)
- [Marketplace](marketplace.md)
- [Page Builder Flow](../diagrams/page-builder-flow.mmd)
- [Backend Pages, Builder, and Preview](../../backend/modules/pages-builder-preview.md)

