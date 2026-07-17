---
okf_document_id: "frontend-feature-organizations-workspaces"
title: "Organizations and Workspaces"
project: "ZinharCMS"
category: "frontend-feature"
phase: 4
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "7d25e4cbc53284a78033478e2681d8e9ebeb2fb1"
last_verified_date: "2026-07-17"
feature_id: "FE-FEAT-003"
feature_name: "Organizations and Workspaces"
feature_paths:
  - "frontend/src/pages/OrganizationPage.tsx"
  - "frontend/src/pages/WorkspaceRedirectPage.tsx"
  - "frontend/src/components/AppShell.tsx"
  - "frontend/src/stores/useAppStore.ts"
boundary_status: "OVERLAPPING"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "frontend/src/pages/OrganizationPage.tsx"
  - "frontend/src/pages/WorkspaceRedirectPage.tsx"
  - "frontend/src/components/AppShell.tsx"
  - "frontend/src/stores/useAppStore.ts"
related_documents:
  - "frontend/feature-catalog.md"
  - "frontend/feature-boundaries.md"
  - "frontend/routing.md"
  - "frontend/state-management.md"
  - "backend/modules/organizations.md"
  - "backend/modules/tenant-authorization.md"
related_diagrams:
  - "frontend/diagrams/frontend-state-flow.mmd"
  - "frontend/diagrams/frontend-api-flow.mmd"
uncertainty_markers:
  - "STATE_OWNERSHIP_UNCLEAR SOU-01"
  - "AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01"
  - "IMPLEMENTATION_STATUS_UNCLEAR ISU-01"
---

# Organizations and Workspaces

## Feature Identity

| Field | Value |
|---|---|
| Feature ID | `FE-FEAT-003` |
| Application | `FE-APP-001` |
| Implementation | `IMPLEMENTED` |
| Boundary | `OVERLAPPING` |
| Confidence | High |
| Routes | `/organization`; `/workspace/:slug`; shell selector |

## Responsibility

Provides organization selection and the frontend control plane for organization details, workspace links, members, invitations, ownership, domains, rate limits, audit logs, email deliveries, alert definitions, organization creation, and invitation acceptance.

## Owned Source Areas

- Main control plane: `frontend/src/pages/OrganizationPage.tsx`.
- Slug context selection: `frontend/src/pages/WorkspaceRedirectPage.tsx`.
- Persistent context and membership list: `frontend/src/stores/useAppStore.ts`.
- Global selector/remount behavior: `frontend/src/components/AppShell.tsx`.

## Entry Points

- `/organization` route and sidebar link.
- `/workspace/:slug` redirect route.
- Organization selector in the shell.
- Session establishment, which provides initial memberships/default organization.

## Internal Structure

`OrganizationPage` is one large route component with domain-specific draft types and many loaders/mutations. It loads the active organization's detail and, for managers, operational sections. The workspace redirect resolves a slug and changes the store context. The shell selector changes context directly by ID.

## State

- Global: membership summaries and active organization ID.
- Local: organization detail, members, invites, workspace, domains, rate limit, audit logs, email delivery records, alerts, role drafts, create/invite/domain/rate-limit drafts, action/loading/error/message state.
- Organization changes remount route content, but the persistent membership list remains.

## Backend Interactions

Uses the organization API group for list/detail, current workspace, membership/invitation lifecycle, ownership transfer/leave, domains, limits, audits, email deliveries, alerts, and create/accept flows. Schema shapes are manually mirrored in frontend types.

## Access Control

The page treats owner/admin as managers, restricts ownership options to owners, and disables actions based on membership/lifecycle. These cues are not backend authorization proof and carry `ABV-01`. The shell selector permits only IDs already present in stored memberships.

## UI Composition

Uses many full-width panels, metric grids, form grids, tables, status badges, icon buttons, confirmation dialogs, clipboard actions, and empty states. Operational and member-management sections appear based on loaded role/context.

## Loading and Error Behavior

One main load/action model serves many sections. Success and errors render in a shared top status stack. Some destructive actions use browser confirmation. A failed subsection can affect the broader workspace load because data orchestration is concentrated.

## Tests

No dedicated frontend test was found for organization switching, role controls, ownership rules, domains, rate limits, invitation flows, operational panels, or stale tenant data.

## Known Risks and Unknowns

- State ownership crosses store, API header context, shell remount, and page data (`SOU-01`).
- Domain verification-to-public-routing remains `ISU-01`; the UI record is not proof of live routing.
- Role cues can drift from backend rules (`ABV-01`).
- Large component responsibility raises regression risk.

## Related Documents

- [Feature Boundaries](../feature-boundaries.md)
- [Routing](../routing.md)
- [State Management](../state-management.md)
- [Backend Organizations](../../backend/modules/organizations.md)
- [Backend Tenant Authorization](../../backend/modules/tenant-authorization.md)

