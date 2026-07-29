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

Provides login and registration UI, keeps the access token in volatile memory,
restores authority through the `HttpOnly` refresh cookie, admits authenticated
bootstrap state to the protected shell, lists/revokes logical sessions in
Settings, and clears/broadcasts session state on current-session revocation or
logout-all. It also owns TOTP enrollment, password-to-MFA login transition,
display-once recovery-code handling, and reusable Step-Up dialogs.

It does not own authoritative credential verification, token validity, tenant membership, or permission enforcement. Those are backend responsibilities.

## Owned Source Areas

- Primary UI: `frontend/src/pages/AuthPage.tsx`.
- Client route admission: `frontend/src/components/RequireAuth.tsx`.
- Session actions and reactive state: `frontend/src/stores/useAppStore.ts`.
- In-memory token transport and auth requests: `frontend/src/services/api.ts`.
- Cross-tab refresh/logout coordination: `frontend/src/services/authSession.ts`.
- Logout integration: `frontend/src/components/AppShell.tsx`.
- Step-Up UI: `frontend/src/components/StepUpDialog.tsx`.

Ownership is shared, which is why the boundary is `OVERLAPPING`.

## Entry Points

- Public route `/login`.
- Login and registration form submissions.
- Protected parent render through `RequireAuth`.
- Shell logout button.
- Module-load restoration from the refresh cookie.

## Internal Structure

`AuthPage` switches mode, owns controlled fields, retains pre-authentication
only in component memory, and calls `setSession` only after AAL1 or completed
AAL2 issuance. `SettingsPage` owns password-confirmed pending enrollment and
display-once recovery codes. `StepUpDialog` creates and verifies an exact-scope
challenge before invoking one pending sensitive action. `SessionBootstrap` performs coordinated
cookie refresh. `RequireAuth` waits for a definitive bootstrap result before
choosing `AppShell` or login redirect. The store synchronizes volatile access
state, non-secret cached identity, and Zustand. The shell calls cookie logout,
then clears and broadcasts client state.

## State

- Local: form mode, credential/profile inputs, pending MFA/Step-Up values,
  display-once recovery codes, pending state, and error.
- Global: access token, refresh token, user, organization memberships, active organization.
- Persistent: non-secret user/membership/active-organization projections in
  `localStorage`; no token strings or session inventory.
- Access credentials remain in memory and cross-tab authentication messages are
  transient.

## Backend Interactions

Uses auth register, login, refresh, logout, current-user, session-list,
session-revoke, and logout-all client methods. One stable invalid-access-token
response can trigger a coordinated refresh and single replay.
MFA users additionally use enrollment, confirmation, MFA verification,
recovery replacement, disable, and Step-Up client methods.

## Access Control

The browser guard waits for explicit bootstrap state. It is not a security
boundary and is marked `AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01`. Backend
authentication and authorization reject invalid or insufficient requests
independently.

## UI Composition

The public authentication page owns its layout, mode controls, fields, locale selector, submit button, and inline status. Protected content uses `AppShell`. Current source pre-populates development credential fields; values are intentionally excluded from OKF and tracked as FE-RISK-002.

## Loading and Error Behavior

Submission disables or changes the submit experience through local pending state. `ApiError.message` or a translated fallback renders in a danger `StatusBadge`. There is no structured field-error mapping. Logout clears locally even when the backend call fails.

## Tests

Dedicated frontend tests cover volatile credential storage, coordinated
bootstrap/refresh/logout, plaintext session rendering, individual/current
session revocation, logout-all, pre-authentication without early session
creation, display-once enrollment state, and trusted-origin Step-Up headers.
Live browser tests cover enrollment, TOTP/recovery login, replay denial,
Step-Up, disable, and empty browser storage. Backend tests remain authoritative
for cryptographic, atomic, and revocation semantics.

## Known Risks and Unknowns

- `SOU-01`: reactive, transport, and persistent session ownership is distributed.
- Deployed-browser compatibility and monitoring for Web Locks/BroadcastChannel
  remain operational concerns.
- Development credential defaults can appear in built UI.
- Intended browser session recovery policy requires `NOC-12`.

## Related Documents

- [Authentication and Access](../authentication-and-access.md)
- [Routing](../routing.md)
- [State Management](../state-management.md)
- [API Client](../api-client.md)
- [Backend Authentication](../../backend/modules/authentication.md)
- [Backend Tenant Authorization](../../backend/modules/tenant-authorization.md)

