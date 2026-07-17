---
okf_document_id: "frontend-feature-beta-operations"
title: "Beta Operations"
project: "ZinharCMS"
category: "frontend-feature"
phase: 4
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "7d25e4cbc53284a78033478e2681d8e9ebeb2fb1"
last_verified_date: "2026-07-17"
feature_id: "FE-FEAT-010"
feature_name: "Beta Operations"
feature_paths:
  - "frontend/src/pages/BetaPage.tsx"
  - "frontend/src/services/api.ts"
  - "frontend/src/types/api.ts"
boundary_status: "OVERLAPPING"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "frontend/src/pages/BetaPage.tsx"
  - "frontend/src/services/api.ts"
  - "frontend/src/types/api.ts"
related_documents:
  - "frontend/feature-catalog.md"
  - "frontend/authentication-and-access.md"
  - "frontend/features/organizations-and-workspaces.md"
  - "backend/modules/beta-release-operations.md"
related_diagrams:
  - "frontend/diagrams/frontend-api-flow.mmd"
uncertainty_markers:
  - "AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01"
  - "IMPLEMENTATION_STATUS_UNCLEAR ISU-03"
  - "RESPONSIBILITY_OVERLAP RO-06"
---

# Beta Operations

## Feature Identity

| Field | Value |
|---|---|
| Feature ID | `FE-FEAT-010` |
| Application | `FE-APP-001` |
| Implementation | `IMPLEMENTED` |
| Boundary | `OVERLAPPING` |
| Confidence | High |
| Route | `/beta` |

## Responsibility

Provides organization beta metrics and feedback, feedback triage, GA blocker operations, beta participant upsert, and a global product dashboard for selected platform roles.

## Owned Source Areas

- Route page: `frontend/src/pages/BetaPage.tsx`.
- Beta client methods and manual types in the central API/type files.

## Entry Points

- Protected `/beta` route and sidebar link.
- Feedback form and triage controls.
- GA blocker create/resolve controls.
- Participant upsert and product dashboard for eligible roles.

## Internal Structure

The page defines category/severity/status vocabularies and three local draft types. Its loader always fetches organization dashboard and feedback, conditionally fetches blockers for manager memberships, and conditionally fetches product data for global admins.

## State

Local dashboard, feedback, blockers, product dashboard, feedback/blocker/participant drafts, load/action state, error, and message. User/global role and active membership come from Zustand.

## Backend Interactions

Uses beta dashboard, feedback list/create/update, blocker list/create/update, participant upsert, and product-dashboard methods. Product readiness state is backend-owned.

## Access Control

Owner/admin/editor memberships manage organization beta items. Global admin/super-admin can view product dashboard/participant controls. These are UI cues; backend enforcement remains authoritative (`ABV-01`).

## UI Composition

Metric cards, feedback form/list, severity badges, blocker form/list, participant controls, and product-level readiness panels inside AppShell.

## Loading and Error Behavior

The main loader conditionally issues role-dependent requests; one failure can set the page error. Mutations update local arrays and refresh dashboard data. Errors/success render in the top status stack. No polling or background synchronization was found.

## Tests

No dedicated Beta page test was found. Role branches, feedback validation, blocker/participant actions, status updates, and readiness display are not covered by the frontend suite.

## Known Risks and Unknowns

- `RO-06`: beta and release-readiness responsibilities are adjacent.
- Repository UI and records do not prove actual beta or GA deployment (`ISU-03`).
- Global and organization roles are combined in one page.
- Role behavior is not security-certified by frontend source.

## Related Documents

- [Organizations and Workspaces](organizations-and-workspaces.md)
- [Authentication and Access](../authentication-and-access.md)
- [Backend Beta and Release Operations](../../backend/modules/beta-release-operations.md)

