---
okf_document_id: "frontend-feature-editorial-workflow-collaboration"
title: "Editorial Workflow and Collaboration"
project: "ZinharCMS"
category: "frontend-feature"
phase: 4
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "7d25e4cbc53284a78033478e2681d8e9ebeb2fb1"
last_verified_date: "2026-07-17"
feature_id: "FE-FEAT-008"
feature_name: "Editorial Workflow and Collaboration"
feature_paths:
  - "frontend/src/pages/WorkflowPage.tsx"
  - "frontend/src/services/api.ts"
  - "frontend/src/types/api.ts"
boundary_status: "OVERLAPPING"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "frontend/src/pages/WorkflowPage.tsx"
  - "frontend/src/services/api.ts"
  - "frontend/src/types/api.ts"
related_documents:
  - "frontend/feature-catalog.md"
  - "frontend/features/content-entries.md"
  - "frontend/features/pages-and-page-builder.md"
  - "backend/modules/content-workflow.md"
  - "backend/modules/comments.md"
  - "backend/modules/built-in-plugins.md"
related_diagrams:
  - "frontend/diagrams/frontend-api-flow.mmd"
uncertainty_markers:
  - "RESPONSIBILITY_OVERLAP FRO-01"
  - "AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01"
  - "API_CONTRACT_UNCLEAR ACU-01"
---

# Editorial Workflow and Collaboration

## Feature Identity

| Field | Value |
|---|---|
| Feature ID | `FE-FEAT-008` |
| Application | `FE-APP-001` |
| Implementation | `IMPLEMENTED` |
| Boundary | `OVERLAPPING` |
| Confidence | High |
| Route | `/workflow` |

## Responsibility

Aggregates pending entry and page review items, provides approval/rejection actions, displays and creates/resolves editorial comments, and exposes built-in plugin enable/disable controls.

## Owned Source Areas

- Route page: `frontend/src/pages/WorkflowPage.tsx`.
- Entry/page/comment/plugin methods and manual contracts in the central API/type files.

The feature does not exclusively own entry/page status or plugin execution; it is an orchestration surface.

## Entry Points

- Protected `/workflow` route and sidebar link.
- Review queue selection and approve/reject buttons.
- Comment composer/resolution controls.
- Plugin toggle controls.

## Internal Structure

The route loads content types, pending entries, pending pages, and plugins; normalizes entries/pages into a shared review-item view; loads comments for the selected resource; and dispatches resource-specific publication/rejection methods.

## State

Local review items, selected key/item, comments, resolved filter, comment draft, plugins, loading/comment-loading, and error state. No cross-route workflow cache or collaboration presence state exists.

## Backend Interactions

Uses content types, entries, pages, comments, and plugins API groups. Backend modules own status rules, resource authorization, side effects, comment persistence, and plugin hooks.

## Access Control

The page is token-gated but has no explicit frontend reviewer role test. Approval/rejection and plugin buttons are visible when an item/plugin exists. Backend role and resource checks must enforce authorization (`ABV-01`).

## UI Composition

Review-list panel, selected-item detail/actions, comment filter/composer/list, and plugin panel. Uses status badges, Lucide icons, global workflow classes, and translated labels.

## Loading and Error Behavior

One primary loader populates review and plugin data. Comment loading has a separate flag. Publish/reject/comment/plugin errors use one page error. No optimistic rollback, polling, realtime collaboration, or concurrent edit warning was found.

## Tests

No dedicated Workflow page test was found. Queue aggregation, resource dispatch, comments, resolved filter, plugin controls, authorization failures, and concurrent changes are uncovered by the current frontend suite.

## Known Risks and Unknowns

- Workflow behavior overlaps Entries and Pages (`FRO-01`).
- A unified UI item hides distinct resource contracts; manual types can drift.
- Browser controls do not certify reviewer authorization.
- Realtime comments/presence and conflict behavior were not found and are not marked planned.

## Related Documents

- [Content Entries](content-entries.md)
- [Pages and Page Builder](pages-and-page-builder.md)
- [Backend Content and Workflow](../../backend/modules/content-workflow.md)
- [Backend Comments](../../backend/modules/comments.md)
- [Backend Built-In Plugins](../../backend/modules/built-in-plugins.md)

