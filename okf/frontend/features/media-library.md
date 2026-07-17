---
okf_document_id: "frontend-feature-media-library"
title: "Media Library"
project: "ZinharCMS"
category: "frontend-feature"
phase: 4
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "7d25e4cbc53284a78033478e2681d8e9ebeb2fb1"
last_verified_date: "2026-07-17"
feature_id: "FE-FEAT-006"
feature_name: "Media Library"
feature_paths:
  - "frontend/src/pages/MediaPage.tsx"
  - "frontend/src/services/api.ts"
  - "frontend/src/types/api.ts"
boundary_status: "OBSERVED"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "frontend/src/pages/MediaPage.tsx"
  - "frontend/src/services/api.ts"
  - "frontend/src/types/api.ts"
related_documents:
  - "frontend/feature-catalog.md"
  - "frontend/api-client.md"
  - "frontend/loading-errors-and-notifications.md"
  - "backend/modules/media.md"
related_diagrams:
  - "frontend/diagrams/frontend-api-flow.mmd"
uncertainty_markers:
  - "AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01"
  - "API_CONTRACT_UNCLEAR ACU-01"
  - "UI_BEHAVIOR_UNVERIFIED UBU-01"
---

# Media Library

## Feature Identity

| Field | Value |
|---|---|
| Feature ID | `FE-FEAT-006` |
| Application | `FE-APP-001` |
| Implementation | `IMPLEMENTED` |
| Boundary | `OBSERVED` |
| Confidence | High |
| Route | `/media` |

## Responsibility

Provides media listing, browser file selection and upload, metadata editing, deletion, and display of backend media records.

## Owned Source Areas

- Route page: `frontend/src/pages/MediaPage.tsx`.
- Multipart and CRUD methods: media group in `frontend/src/services/api.ts`.
- Media response/request declarations: `frontend/src/types/api.ts`.

## Entry Points

- Protected `/media` route and sidebar link.
- File picker/upload action.
- Media edit and delete actions.
- Dashboard consumes media counts but not the Media page component.

## Internal Structure

One route component owns media fetch, selected files, upload, record selection, metadata draft, update/delete actions, and presentation. No reusable media-picker component or feature folder was found.

## State

Local media records, selected file, editing record/draft, loading/upload/action state, error, and success message. Browser `File` state is transient. Saved binary and metadata state is backend-owned.

## Backend Interactions

Uses list, multipart upload, metadata update, and delete methods. The browser does not own processing, variants, filesystem persistence, or compensation behavior; see the backend Media module.

## Access Control

The route is token-gated with no route-specific role visibility. File and record authorization is backend-owned and remains `ABV-01` from the frontend perspective.

## UI Composition

Uses AppShell, file picker, upload controls, record cards/list, metadata form, status badges, icon buttons, and empty state. Media URLs and variants come from backend responses.

## Loading and Error Behavior

Initial list and mutation states are local. Upload/update/delete failures render page-level messages. Destructive deletion uses a browser confirmation. No upload progress percentage, cancellation, retry/resume, or offline queue was found.

## Tests

No dedicated Media page, multipart client, file constraint, preview, metadata, or deletion frontend test was found.

## Known Risks and Unknowns

- Browser types can drift from media responses (`ACU-01`).
- Upload progress and cancellation behavior are absent.
- UI presence does not prove production storage topology or atomic media lifecycle.
- Responsive media-grid behavior remains `UBU-01`.

## Related Documents

- [API Client](../api-client.md)
- [Loading, Errors, and Notifications](../loading-errors-and-notifications.md)
- [Backend Media](../../backend/modules/media.md)
- [Frontend API Flow](../diagrams/frontend-api-flow.mmd)

