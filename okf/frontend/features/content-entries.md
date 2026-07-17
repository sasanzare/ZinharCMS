---
okf_document_id: "frontend-feature-content-entries"
title: "Content Entries"
project: "ZinharCMS"
category: "frontend-feature"
phase: 4
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "7d25e4cbc53284a78033478e2681d8e9ebeb2fb1"
last_verified_date: "2026-07-17"
feature_id: "FE-FEAT-005"
feature_name: "Content Entries"
feature_paths:
  - "frontend/src/pages/EntriesPage.tsx"
  - "frontend/src/components/DynamicForm.tsx"
  - "frontend/src/services/api.ts"
  - "frontend/src/types/api.ts"
boundary_status: "OVERLAPPING"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "frontend/src/pages/EntriesPage.tsx"
  - "frontend/src/components/DynamicForm.tsx"
  - "frontend/src/services/api.ts"
related_documents:
  - "frontend/feature-catalog.md"
  - "frontend/forms-and-validation.md"
  - "frontend/features/content-modeling.md"
  - "frontend/features/editorial-workflow-and-collaboration.md"
  - "backend/modules/content-workflow.md"
related_diagrams:
  - "frontend/diagrams/frontend-api-flow.mmd"
uncertainty_markers:
  - "RESPONSIBILITY_OVERLAP FRO-01"
  - "AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01"
  - "API_CONTRACT_UNCLEAR ACU-01"
---

# Content Entries

## Feature Identity

| Field | Value |
|---|---|
| Feature ID | `FE-FEAT-005` |
| Application | `FE-APP-001` |
| Implementation | `IMPLEMENTED` |
| Boundary | `OVERLAPPING` |
| Confidence | High |
| Route | `/entries` |

## Responsibility

Provides content-type selection, entry listing and filtering, generated entry editing, create/update/delete actions, and visible workflow status transitions.

## Owned Source Areas

- Route page: `frontend/src/pages/EntriesPage.tsx`.
- Generated field UI: `frontend/src/components/DynamicForm.tsx`.
- Content-type and entry client groups/types in `services/api.ts` and `types/api.ts`.

`DynamicForm` is shared technical code, while workflow decisions are also exposed by the Workflow feature.

## Entry Points

- Protected `/entries` route and sidebar link.
- Content-type selector.
- Entry create/edit/status/delete controls.
- Workflow page as a secondary consumer of the same entry lifecycle.

## Internal Structure

The page loads content types, selects one, loads matching entries, builds a JSON draft, and delegates schema-driven fields to `DynamicForm`. It owns list/editor composition, lifecycle actions, and refreshes after mutations.

## State

Local content types, selected type, entry list, selected/editing entry, JSON draft, search/filter state, loading/saving, and error. There is no cross-route entry cache or draft persistence.

## Backend Interactions

Uses content-type reads and entry list/create/update/delete/workflow methods. Backend validation, plugin hooks, webhooks, and delivery invalidation are outside this frontend boundary and documented in the backend module.

## Access Control

Token presence admits the route. No route-level role metadata or page role check was found. Backend entry ownership, status transition, and tenant enforcement remain authoritative (`ABV-01`).

## UI Composition

AppShell, content-type selector, list/table, status badges, editor fields, DynamicForm, action buttons, and empty/loading states. Field UI is determined by backend-provided field definitions.

## Loading and Error Behavior

Type and entry loads, saves, status updates, and deletion use page-local errors. A failed request renders one error string. No field error map, retry policy, or navigation dirty guard was found.

## Tests

No dedicated Entries or DynamicForm frontend test was found. Type switching, conversions, required fields, status transitions, deletion, and tenant remount behavior remain untested in the current frontend suite.

## Known Risks and Unknowns

- Entry UI overlaps Content Modeling and Editorial Workflow.
- Dynamic field conversion applies only a subset of schema semantics.
- Client types can drift (`ACU-01`).
- Business meaning and allowed transitions are not inferred from button sequence.

## Related Documents

- [Content Modeling](content-modeling.md)
- [Editorial Workflow and Collaboration](editorial-workflow-and-collaboration.md)
- [Forms and Validation](../forms-and-validation.md)
- [Backend Content and Workflow](../../backend/modules/content-workflow.md)

