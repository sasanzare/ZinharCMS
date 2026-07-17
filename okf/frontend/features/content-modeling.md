---
okf_document_id: "frontend-feature-content-modeling"
title: "Content Modeling"
project: "ZinharCMS"
category: "frontend-feature"
phase: 4
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "7d25e4cbc53284a78033478e2681d8e9ebeb2fb1"
last_verified_date: "2026-07-17"
feature_id: "FE-FEAT-004"
feature_name: "Content Modeling"
feature_paths:
  - "frontend/src/pages/ContentTypesPage.tsx"
  - "frontend/src/services/api.ts"
  - "frontend/src/types/api.ts"
boundary_status: "OBSERVED"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "frontend/src/pages/ContentTypesPage.tsx"
  - "frontend/src/services/api.ts"
  - "frontend/src/types/api.ts"
related_documents:
  - "frontend/feature-catalog.md"
  - "frontend/forms-and-validation.md"
  - "frontend/features/content-entries.md"
  - "backend/modules/content-workflow.md"
related_diagrams:
  - "frontend/diagrams/frontend-api-flow.mmd"
uncertainty_markers:
  - "AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01"
  - "API_CONTRACT_UNCLEAR ACU-01"
  - "NEEDS_OWNER_CONFIRMATION NOC-12"
---

# Content Modeling

## Feature Identity

| Field | Value |
|---|---|
| Feature ID | `FE-FEAT-004` |
| Application | `FE-APP-001` |
| Implementation | `IMPLEMENTED` |
| Boundary | `OBSERVED` |
| Confidence | High |
| Route | `/content-types` |

## Responsibility

Provides the frontend workflow for listing, filtering, creating, editing, and deleting content types and their field schema definitions.

## Owned Source Areas

- Route page: `frontend/src/pages/ContentTypesPage.tsx`.
- Client methods: content-type group in `frontend/src/services/api.ts`.
- Contracts: content-type and `FieldSchema` declarations in `frontend/src/types/api.ts`.

## Entry Points

- Protected route `/content-types`.
- Sidebar content-types link.
- New/edit/delete controls and field-definition editor.

## Internal Structure

One route component owns filtering, selection, model draft, field array editing, loading, persistence, deletion, and feedback. It relies on global semantic form/table components rather than a feature component folder.

## State

Page-local content-type list, search/filter value, selected/editing record, model/field draft, loading/saving state, and error. No global model cache or persisted draft was found.

## Backend Interactions

Uses content-type list, create, update, and delete methods. Detailed schema evolution and endpoint contracts are deferred to Phases 5, 6, and 8. Client shapes are manually declared and carry `ACU-01`.

## Access Control

The route requires a stored token but has no route-specific role declaration or menu filtering. The frontend does not prove who may change schemas; backend enforcement is authoritative (`ABV-01`).

## UI Composition

Composes panels, search/filter controls, a model list/table, editor inputs, repeatable field definitions, action buttons, status badges, and empty states inside `AppShell`.

## Loading and Error Behavior

Initial load and mutations use page-local flags. API failures render a translated fallback or `ApiError.message`. Deletion uses a browser confirmation. No field-level backend error mapping was found.

## Tests

No dedicated frontend content-modeling test was found. Dynamic schema evolution, field ordering, validation, save failure, and delete behavior are not covered by the three current page test files.

## Known Risks and Unknowns

- Intended schema-evolution policy requires `NOC-12`.
- Manual client schemas can drift from backend contracts.
- Page-local validation is limited and backend errors are not mapped to fields.
- Access behavior is not verified as a frontend security contract.

## Related Documents

- [Forms and Validation](../forms-and-validation.md)
- [Content Entries](content-entries.md)
- [API Client](../api-client.md)
- [Backend Content and Workflow](../../backend/modules/content-workflow.md)

