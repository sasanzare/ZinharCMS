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
| Access and refresh tokens | Zustand plus API module variables | Browser session across reloads | `localStorage` | Guard, shell logout, API request | `VERIFIED`; `SOU-01` |
| Current user | Zustand | Browser session across reloads | JSON in `localStorage` | Shell, Beta, Marketplace | `VERIFIED` |
| Organization memberships | Zustand | Browser session across reloads | JSON in `localStorage` | Shell, Organization, Billing, Beta, Marketplace | `VERIFIED` |
| Active organization ID | Zustand plus API module variable | Browser session across reloads | `localStorage` | Shell, request header, Pages preview URL | `VERIFIED`; `SOU-01` |
| Locale and direction | `I18nProvider` context | Browser session across reloads | `localStorage` | Entire UI | `VERIFIED` |
| Server responses | Individual pages/hooks | Route component lifetime | None | Owning page | `VERIFIED`; `SOU-02` |
| Form and editor drafts | Individual pages | Route component lifetime | None, except saved backend records | Owning page | `VERIFIED` |
| Page Builder draft | `PagesPage` local state | Route component lifetime | Backend after manual/autosave | Builder | `VERIFIED` |

## Zustand Store

`useAppStore.ts` defines one store. It owns `sidebarCollapsed`, `accessToken`, `refreshToken`, `user`, `organizations`, and `activeOrganizationId`, with actions to toggle the sidebar, establish/clear a session, replace organizations, and select an organization.

At module load it reads browser storage, validates the saved active organization against saved memberships, and initializes the API module's organization variable. Session and organization actions call imperative API setters before updating Zustand.

## Persistence Keys

The implementation uses separate `localStorage` entries for access token, refresh token, user, organizations, active organization ID, and locale. The key names are implementation details and should not be treated as a stable external contract without a compatibility decision.

Invalid JSON for user or organizations is removed and replaced with a fallback. Token strings and active organization are read directly. There is no schema version, migration, expiry metadata, encryption layer, or session-storage alternative.

## State Synchronization

### Session establishment

1. `AuthPage` receives an auth response.
2. `setSession` selects an active organization.
3. API setters update module variables and token/organization storage.
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

The shell attempts backend logout with the stored refresh token and clears local state even if the request fails. API setters remove token and organization storage; the store removes user and organization JSON and clears Zustand state. The route guard then redirects.

## Ownership Ambiguities

### SOU-01: Session and Organization State

Zustand is the reactive owner, the API module holds transport-facing copies, and `localStorage` is the reload source. Correctness depends on all changes using the provided imperative setters. Browser storage mutations from another tab have no verified listener, and no single typed persistence adapter owns migration or validation.

### SOU-02: Server State

Pages own server data independently with `useState`, `useEffect`, and callback loaders. No TanStack Query, SWR, normalized cache, global invalidation bus, or request-deduplication layer was found. Cross-page consistency depends on remount/reload behavior and explicit local refreshes.

## Concurrency and Staleness

- `useHealth` suppresses state updates after unmount but does not abort fetches.
- Page loaders generally use loading flags but no shared request identity or cancellation policy.
- Organization switching remounts the active page to reduce stale tenant state, but background work initiated outside the remounted subtree can still finish independently.
- No multi-tab synchronization, optimistic-update framework, offline queue, or cache reconciliation was found.
- Page Builder autosave uses a 10-second timeout for existing dirty pages and can overlap conceptual user actions; it uses a `saving` flag but has no explicit revision/conflict token in the UI.

## State Rules for Changes

- Use store actions for session and organization changes so transport and persistence copies remain synchronized.
- Treat all page-local API data as disposable on organization change.
- Do not add another token or organization owner without documenting synchronization and failure semantics.
- When introducing shared server state, define tenant-keying, invalidation, cancellation, and logout clearing before moving data.
- Keep localization separate unless a deliberate state-composition decision changes the current provider boundary.

## Related Documents

- [API Client](api-client.md)
- [Authentication and Access](authentication-and-access.md)
- [Loading, Errors, and Notifications](loading-errors-and-notifications.md)
- [Frontend State Flow](diagrams/frontend-state-flow.mmd)
- [Architecture Dependency Model](../architecture/dependency-model.md)

