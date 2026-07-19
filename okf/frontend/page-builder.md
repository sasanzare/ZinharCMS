---
okf_document_id: "frontend-page-builder"
title: "Frontend Page Builder"
project: "ZinharCMS"
category: "frontend"
phase: 4
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources:
  - "frontend/src/pages/PagesPage.tsx"
  - "frontend/src/pages/PagesPage.test.tsx"
  - "frontend/src/services/api.ts"
  - "frontend/src/types/api.ts"
  - "frontend/src/styles/index.css"
  - "frontend/package.json"
related_documents:
  - "frontend/features/pages-and-page-builder.md"
  - "frontend/component-architecture.md"
  - "frontend/state-management.md"
  - "frontend/forms-and-validation.md"
  - "frontend/frontend-risks.md"
  - "backend/modules/pages-builder-preview.md"
  - "backend/modules/marketplace-runtime-adapters.md"
related_diagrams:
  - "frontend/diagrams/page-builder-flow.mmd"
uncertainty_markers:
  - "RESPONSIBILITY_OVERLAP FRO-01"
  - "FEATURE_BOUNDARY_UNCLEAR FBU-01"
  - "AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01"
  - "UI_BEHAVIOR_UNVERIFIED UBU-01"
---

# Frontend Page Builder

## Scope and Status

`VERIFIED`: A functional visual Page Builder is implemented inside `PagesPage.tsx`. This resolves the current-state side of `DOCUMENTATION_CODE_CONFLICT DCC-01`, where older phase narrative could imply that the visual builder was still future. The implementation is not a separate application or package.

## Builder Composition

| Area | Current behavior | Source owner |
|---|---|---|
| Page metadata | Title and slug form; page JSON metadata fields | `PagesPage` local draft |
| Component palette | Search and add system plus active Marketplace component definitions | `PaletteItem`; API load |
| Canvas | Nested sortable nodes, selection, removal, top-level reordering | `BuilderCanvas`; `SortableCanvasNode`; dnd-kit |
| Property editor | Schema-driven boolean/select/text/number/email/JSON/array/rich-text controls | `PropControl` |
| Local preview | React rendering of current page JSON into simple preview blocks | `LivePreview`; `PreviewNode` |
| Persistence | Create/update normalized page JSON; manual save; delayed autosave for existing pages | Page API and local state |
| Versions | Load snapshot list and restore selected version | Page API |
| Workflow | Submit review, publish, archive, or restore based on current status | Page API |
| Marketplace templates | Preview required assets and import a template into a new page | Marketplace adapter API |
| Preview connection handoff | Build and copy a backend WebSocket URL | Clipboard operation |

## Load Flow

The route concurrently loads pages, system component registry entries, Marketplace component adapter entries, and Marketplace installations. It merges system and Marketplace components into one palette and filters active design-template installations for template actions.

No shared server-state cache, request cancellation, or partial-success model is used. Failure of the combined load sets one page-level error.

## Editing Model

The draft contains optional page identity, title, slug, and normalized page JSON. Component changes call `mutatePageJson`, mark the draft dirty, and update an autosave status. A selected node drives the property panel. Top-level nodes can be reordered through `arrayMove`; nested nodes render through recursive canvas functions.

New components are appended to the top-level layout. Dragging a palette item into the builder adds it; dragging existing top-level nodes reorders them. The code supports nested rendering/removal/update helpers, but Phase 4 did not verify arbitrary cross-level drag/drop placement.

## Save and Autosave

- New drafts require explicit save because they lack a page ID.
- Existing dirty drafts schedule autosave after 10 seconds.
- Save normalizes page JSON and calls create or update.
- A successful save replaces the draft from the response, clears dirty state, and reloads the page list.
- A failed save leaves an error and save-failed status.

No explicit optimistic concurrency token, client-side revision comparison, conflict-resolution UI, navigation dirty guard, or undo/redo history was found. Version restore is backend snapshot restoration, not client undo.

## Preview Boundaries

The in-page `LivePreview` is a local React interpretation of page JSON. It does not open the backend WebSocket. Separately, the copy-preview action creates a WebSocket URL by changing the API URL scheme and adding page, access-token, and organization context. An external preview client can use that URL; backend preview channels are documented in [Pages, Builder, and Preview](../backend/modules/pages-builder-preview.md).

The local preview does not establish parity with a public renderer or installed component runtime. Exact rendering compatibility is unverified.

## Marketplace Integration

Active Marketplace component packs contribute definitions to the palette. Active design-template installations can be previewed for required asset mappings and imported after browser prompts collect a page title and slug. Pages therefore crosses the Marketplace host-adapter boundary and carries `RESPONSIBILITY_OVERLAP FRO-01`.

## Implemented and Not Found

| Capability | Status |
|---|---|
| Palette search/add | `IMPLEMENTED` |
| Drag-and-drop top-level reorder | `IMPLEMENTED` |
| Nested node rendering/edit/removal | `IMPLEMENTED` |
| Property schema controls | `IMPLEMENTED` |
| Local live preview | `IMPLEMENTED` |
| Manual save and delayed autosave | `IMPLEMENTED` |
| Page workflow and snapshots | `IMPLEMENTED` |
| Marketplace component/template integration | `IMPLEMENTED` |
| Client undo/redo | Not found; not marked planned |
| Responsive preview modes | Not found; not marked planned |
| Separate layer/tree panel | Not found; not marked planned |
| Cross-device collaborative editing | Not found; not marked planned |
| Browser-opened WebSocket preview inside builder | Not found; URL handoff is implemented |

## Tests

`PagesPage.test.tsx` mocks page/component/Marketplace APIs and verifies the builder shell, system and Marketplace palette entries, empty canvas, and property editor. It does not exercise drag/drop, nested editing, autosave timing, save errors, workflow transitions, version restore, template import, clipboard URL contents, local preview parity, or WebSocket integration.

## Risks

- A single page file owns builder UI, state, persistence, workflow, templates, versions, and preview handoff.
- Preview URLs place session context in query parameters and clipboard content.
- Autosave has no visible conflict-resolution contract.
- Local preview parity with delivery/runtime rendering is unverified.
- Drag/drop and critical persistence paths have little frontend test coverage.

## Related Documents

- [Pages and Page Builder Feature](features/pages-and-page-builder.md)
- [Page Builder Flow](diagrams/page-builder-flow.mmd)
- [State Management](state-management.md)
- [Forms and Validation](forms-and-validation.md)
- [Backend Pages, Builder, and Preview](../backend/modules/pages-builder-preview.md)
- [Backend Marketplace Runtime Adapters](../backend/modules/marketplace-runtime-adapters.md)

## Phase 8 Page Builder Rules

[Page Builder Rules](../domain/page-builder-rules.md) records the current document invariants: one `root`, registered descendants, unique bounded node IDs, object props/styles, maximum depth 12, and maximum 500 nodes. [Page Builder Save and Version](../domain/workflows/page-builder-save-and-version.md) and [Page Version Restoration](../domain/workflows/page-version-restoration.md) distinguish transactional snapshots from publication, cache, and broadcast side effects. Restore resets the page to draft; cache revalidation after restore is not verified.
