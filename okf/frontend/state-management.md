---
okf_document_id: "frontend-state-management"
title: "Frontend State Management"
project: "ZinharCMS"
category: "frontend"
phase: 4
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "7d25e4cbc53284a78033478e2681d8e9ebeb2fb1"
last_verified_date: "2026-07-17"
primary_sources:
  - "frontend/src/stores/useAppStore.ts"
  - "frontend/src/services/api.ts"
  - "frontend/src/i18n/I18nProvider.tsx"
  - "frontend/src/pages"
  - "frontend/src/components/AppShell.tsx"
related_documents:
  - "frontend/feature-boundaries.md"
  - "frontend/api-client.md"
  - "frontend/authentication-and-access.md"
  - "frontend/loading-errors-and-notifications.md"
  - "architecture/dependency-model.md"
related_diagrams:
  - "frontend/diagrams/frontend-state-flow.mmd"
uncertainty_markers:
  - "STATE_OWNERSHIP_UNCLEAR SOU-01"
  - "STATE_OWNERSHIP_UNCLEAR SOU-02"
  - "DUPLICATED_CONTRACT DC-01"
  - "AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01"
---

# Frontend State Management

## State Domains

| Domain | Owner | Lifetime | Persistence | Main consumers | Status |
|---|---|---|---|---|---|
| Sidebar collapse | Zustand `useAppStore` | SPA session | None | `AppShell` | `VERIFIED` |
| Access token | Zustand plus API module variable | Current document only | None; volatile memory | Guard and API request | `VERIFIED`; `SOU-01` |
| Refresh token | Backend-managed cookie | Session-family lifetime | `HttpOnly` cookie | Refresh and logout endpoints | `VERIFIED` |
| Current user | Zustand | Browser session across reloads | JSON in `localStorage` | Shell, Beta, Marketplace | `VERIFIED` |
| Organization memberships | Zustand | Browser session across reloads | JSON in `localStorage` | Shell, Organization, Billing, Beta, Marketplace | `VERIFIED` |
| Active organization ID | Zustand plus API module variable | Browser session across reloads | `localStorage` | Shell, request header, preview-ticket request | `VERIFIED`; `SOU-01` |
| Locale and direction | `I18nProvider` context | Browser session across reloads | `localStorage` | Entire UI | `VERIFIED` |
| Server responses | Individual pages/hooks | Route component lifetime | None | Owning page | `VERIFIED`; `SOU-02` |
| Form and editor drafts | Individual pages | Route component lifetime | None, except saved backend records | Owning page | `VERIFIED` |
| Page Builder draft | `PagesPage` local state | Route component lifetime | Backend after manual/autosave | Builder | `VERIFIED` |

## Zustand Store

`useAppStore.ts` defines one store. It owns `sidebarCollapsed`, the volatile
`accessToken`, explicit `authStatus`, `user`, `organizations`, and
`activeOrganizationId`, with actions to toggle the sidebar, bootstrap,
establish/clear a session, replace organizations, and select an organization.

At module load it reads only non-secret cached identity/organization state,
validates the saved active organization against saved memberships, initializes
the API module's organization variable, and starts authentication as `unknown`.
`SessionBootstrap` restores authority from the `HttpOnly` refresh cookie before
protected rendering.

## Persistence Keys

The implementation uses separate `localStorage` entries only for non-secret
user projection, organizations, active organization ID, and locale. Legacy
access/refresh token keys are deleted on startup and are never authentication
inputs.

Invalid JSON for user or organizations is removed and replaced with a fallback.
There is no schema version or migration for the non-secret cached projections.
The access token and its lifetime are volatile; the refresh credential is not
available to JavaScript.

## State Synchronization

### Session establishment

1. `AuthPage` receives an auth response.
2. `setSession` selects an active organization.
3. API setters update the volatile token and organization state.
4. The store writes user and organizations and updates Zustand state.
5. Protected routing reacts to the access token.

### Organization change

1. The shell or workspace redirect calls `setActiveOrganization`.
2. The store confirms the ID exists in current memberships.
3. The API setter updates its module variable and persistent ID.
4. Zustand updates the active ID.
5. The shell changes the `main` key, remounting the active page.
6. Page effects reload data using the new request header.

### Logout

The shell calls the cookie-authenticated backend logout endpoint and clears
volatile access state and cached identity even if the request fails. A transient
BroadcastChannel logout event clears listening tabs and stops preview
reconnects. The route guard then redirects after state becomes unauthenticated.

## Ownership Ambiguities

### SOU-01: Session and Organization State

Zustand is the reactive owner, the API module holds the transport-facing
volatile access token, and the refresh cookie is the authoritative reload
source. Correctness depends on all session changes using the coordinator and
store actions. Web Locks and BroadcastChannel coordinate refresh/logout without
using storage events or persisting credentials.

### SOU-02: Server State

Pages own server data independently with `useState`, `useEffect`, and callback loaders. No TanStack Query, SWR, normalized cache, global invalidation bus, or request-deduplication layer was found. Cross-page consistency depends on remount/reload behavior and explicit local refreshes.

## Concurrency and Staleness

- `useHealth` suppresses state updates after unmount but does not abort fetches.
- Page loaders generally use loading flags but no shared request identity or cancellation policy.
- Organization switching remounts the active page to reduce stale tenant state, but background work initiated outside the remounted subtree can still finish independently.
- Authentication refresh/logout has bounded cross-tab coordination. Other
  client state has no general multi-tab reconciliation.
- Page Builder autosave uses a 10-second timeout for existing dirty pages and can overlap conceptual user actions; it uses a `saving` flag but has no explicit revision/conflict token in the UI.

## State Rules for Changes

- Use store actions for session and organization changes so transport and persistence copies remain synchronized.
- Treat all page-local API data as disposable on organization change.
- Do not persist authentication tokens or add another token/organization owner
  without documenting synchronization and failure semantics.
- When introducing shared server state, define tenant-keying, invalidation, cancellation, and logout clearing before moving data.
- Keep localization separate unless a deliberate state-composition decision changes the current provider boundary.

## Related Documents

- [API Client](api-client.md)
- [Authentication and Access](authentication-and-access.md)
- [Loading, Errors, and Notifications](loading-errors-and-notifications.md)
- [Frontend State Flow](diagrams/frontend-state-flow.mmd)
- [Architecture Dependency Model](../architecture/dependency-model.md)

