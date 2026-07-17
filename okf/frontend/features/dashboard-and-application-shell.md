---
okf_document_id: "frontend-feature-dashboard-application-shell"
title: "Dashboard and Application Shell"
project: "ZinharCMS"
category: "frontend-feature"
phase: 4
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "7d25e4cbc53284a78033478e2681d8e9ebeb2fb1"
last_verified_date: "2026-07-17"
feature_id: "FE-FEAT-002"
feature_name: "Dashboard and Application Shell"
feature_paths:
  - "frontend/src/components/AppShell.tsx"
  - "frontend/src/pages/DashboardPage.tsx"
  - "frontend/src/hooks/useHealth.ts"
  - "frontend/src/components/StatusBadge.tsx"
boundary_status: "OVERLAPPING"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "frontend/src/components/AppShell.tsx"
  - "frontend/src/pages/DashboardPage.tsx"
  - "frontend/src/hooks/useHealth.ts"
  - "frontend/src/pages/DashboardPage.test.tsx"
related_documents:
  - "frontend/feature-catalog.md"
  - "frontend/pages-and-layouts.md"
  - "frontend/component-architecture.md"
  - "frontend/state-management.md"
  - "frontend/features/organizations-and-workspaces.md"
related_diagrams:
  - "frontend/diagrams/frontend-application-map.mmd"
  - "frontend/diagrams/frontend-routing-flow.mmd"
uncertainty_markers:
  - "FEATURE_BOUNDARY_UNCLEAR FBU-02"
  - "RESPONSIBILITY_OVERLAP FRO-02"
  - "UI_BEHAVIOR_UNVERIFIED UBU-01"
---

# Dashboard and Application Shell

## Feature Identity

| Field | Value |
|---|---|
| Feature ID | `FE-FEAT-002` |
| Application | `FE-APP-001` |
| Implementation | `IMPLEMENTED` |
| Boundary | `OVERLAPPING` |
| Confidence | High |
| Route | `/` plus layout for every protected route |

## Responsibility

Provides protected application chrome, navigation, route titles, organization and locale switching, readiness status, user display, logout, and a dashboard with system, CMS, and usage summaries.

## Owned Source Areas

- `frontend/src/components/AppShell.tsx`
- `frontend/src/pages/DashboardPage.tsx`
- `frontend/src/hooks/useHealth.ts`
- `frontend/src/components/StatusBadge.tsx`

The shell consumes state and behaviors owned by Authentication, Organizations, Localization, and API integration.

## Entry Points

- Protected route parent through `RequireAuth`.
- Index route `/` for `DashboardPage`.
- Sidebar navigation, organization selector, locale selector, and logout controls.
- Health polling when shell/dashboard mounts.

## Internal Structure

`AppShell` renders sidebar, top bar, and route outlet. A static nav array drives links and title keys. `DashboardPage` composes technology/foundation information, health/readiness, content counts, and billing usage. `useHealth` polls two system methods in parallel.

## State

- Shell: sidebar, user, organizations, active organization from Zustand; locale from context; health from hook.
- Dashboard: health hook plus local stats and billing usage responses/errors.
- Changing organization remounts the outlet subtree through the main-element key.

## Backend Interactions

Shell and dashboard call health/readiness. Dashboard also calls content types, media, pages, entries, and billing usage. Shell logout calls auth logout. Detailed contracts remain in later API documentation.

## Access Control

The shell requires a stored access token. Navigation items are not filtered by role. The user chip displays the global role, while action-level permissions are owned by individual pages and backend enforcement.

## UI Composition

The layout has a collapsible sidebar and top bar with API base URL, organization selector, language selector, readiness status, role chip, and logout. The dashboard uses metric cards, status badges, and panels. Icons come from Lucide.

## Loading and Error Behavior

Health polling preserves previous results after a later error and exposes loading/error state. Dashboard stats and billing usage have separate error states. The shell shows checking/warning status rather than a global failure page. Shell and dashboard can poll the same probes independently.

## Tests

`DashboardPage.test.tsx` mocks `useHealth` and verifies three foundation cards. It does not test API summaries, failures, polling, shell navigation, sidebar, organization remount, locale, or logout.

## Known Risks and Unknowns

- `FBU-02` and `FRO-02`: shell responsibility spans several cross-cutting owners.
- Static navigation can expose destinations the backend later denies.
- Duplicate health polling can occur on the dashboard.
- Responsive, keyboard, and RTL shell behavior remains `UBU-01`.

## Related Documents

- [Pages and Layouts](../pages-and-layouts.md)
- [Component Architecture](../component-architecture.md)
- [State Management](../state-management.md)
- [Organizations and Workspaces](organizations-and-workspaces.md)
- [Frontend Application Map](../diagrams/frontend-application-map.mmd)

