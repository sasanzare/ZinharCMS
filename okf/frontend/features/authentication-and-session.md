---
okf_document_id: "frontend-feature-authentication-session"
title: "Authentication and Session"
project: "ZinharCMS"
category: "frontend-feature"
phase: 4
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "7d25e4cbc53284a78033478e2681d8e9ebeb2fb1"
last_verified_date: "2026-07-17"
feature_id: "FE-FEAT-001"
feature_name: "Authentication and Session"
feature_paths:
  - "frontend/src/pages/AuthPage.tsx"
  - "frontend/src/components/RequireAuth.tsx"
  - "frontend/src/components/AppShell.tsx"
  - "frontend/src/stores/useAppStore.ts"
  - "frontend/src/services/api.ts"
boundary_status: "OVERLAPPING"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "frontend/src/pages/AuthPage.tsx"
  - "frontend/src/components/RequireAuth.tsx"
  - "frontend/src/stores/useAppStore.ts"
  - "frontend/src/services/api.ts"
related_documents:
  - "frontend/feature-catalog.md"
  - "frontend/authentication-and-access.md"
  - "frontend/routing.md"
  - "frontend/state-management.md"
  - "frontend/api-client.md"
  - "backend/modules/authentication.md"
  - "backend/modules/tenant-authorization.md"
related_diagrams:
  - "frontend/diagrams/frontend-routing-flow.mmd"
  - "frontend/diagrams/frontend-api-flow.mmd"
uncertainty_markers:
  - "AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01"
  - "STATE_OWNERSHIP_UNCLEAR SOU-01"
  - "NEEDS_OWNER_CONFIRMATION NOC-12"
---

# Authentication and Session

## Feature Identity

| Field | Value |
|---|---|
| Feature ID | `FE-FEAT-001` |
| Application | `FE-APP-001` |
| Implementation | `IMPLEMENTED` |
| Boundary | `OVERLAPPING` |
| Confidence | High |
| Routes | `/login`; protected route parent |

## Responsibility

Provides login and registration UI, stores the returned browser session, admits token-bearing users to the protected shell, restores persisted client state, and clears session state on logout.

It does not own authoritative credential verification, token validity, tenant membership, or permission enforcement. Those are backend responsibilities.

## Owned Source Areas

- Primary UI: `frontend/src/pages/AuthPage.tsx`.
- Client route admission: `frontend/src/components/RequireAuth.tsx`.
- Session actions and reactive state: `frontend/src/stores/useAppStore.ts`.
- Token persistence and auth requests: `frontend/src/services/api.ts`.
- Logout integration: `frontend/src/components/AppShell.tsx`.

Ownership is shared, which is why the boundary is `OVERLAPPING`.

## Entry Points

- Public route `/login`.
- Login and registration form submissions.
- Protected parent render through `RequireAuth`.
- Shell logout button.
- Module-load restoration from browser storage.

## Internal Structure

`AuthPage` switches mode, owns controlled fields, calls auth methods, and passes successful responses to `setSession`. `RequireAuth` chooses between `AppShell` and login redirect. The store synchronizes API setters, persistent data, and Zustand. The shell calls backend logout and then clears client state.

## State

- Local: form mode, credential/profile inputs, pending state, and error.
- Global: access token, refresh token, user, organization memberships, active organization.
- Persistent: token strings, user JSON, memberships JSON, and active organization in `localStorage`.
- No verified token expiry timer, cross-tab synchronization, or persistent schema version.

## Backend Interactions

Uses auth register, login, refresh, logout, and current-user client methods. Refresh exists as a callable method but is not part of an automatic interceptor. Detailed endpoint contracts are deferred to Phase 6.

## Access Control

The browser guard checks only token presence. It is not a security boundary and is marked `AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01`. Backend authentication and authorization must reject invalid or insufficient requests independently.

## UI Composition

The public authentication page owns its layout, mode controls, fields, locale selector, submit button, and inline status. Protected content uses `AppShell`. Current source pre-populates development credential fields; values are intentionally excluded from OKF and tracked as FE-RISK-002.

## Loading and Error Behavior

Submission disables or changes the submit experience through local pending state. `ApiError.message` or a translated fallback renders in a danger `StatusBadge`. There is no structured field-error mapping. Logout clears locally even when the backend call fails.

## Tests

No dedicated authentication, session-store, guard, persistence, expiry, refresh, or logout frontend test was found. Backend auth tests do not substitute for browser behavior coverage.

## Known Risks and Unknowns

- `SOU-01`: reactive, transport, and persistent session ownership is distributed.
- No global session-expiry recovery is implemented.
- Tokens persist in browser storage.
- Development credential defaults can appear in built UI.
- Intended browser session recovery policy requires `NOC-12`.

## Related Documents

- [Authentication and Access](../authentication-and-access.md)
- [Routing](../routing.md)
- [State Management](../state-management.md)
- [API Client](../api-client.md)
- [Backend Authentication](../../backend/modules/authentication.md)
- [Backend Tenant Authorization](../../backend/modules/tenant-authorization.md)

