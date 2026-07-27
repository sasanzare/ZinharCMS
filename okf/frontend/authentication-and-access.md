---
okf_document_id: "frontend-authentication-access"
title: "Frontend Authentication and Access"
project: "ZinharCMS"
category: "frontend"
phase: 4
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "eed1e0dbdf6d873457d1165158b3c8fbfd6647e1"
last_verified_date: "2026-07-17"
primary_sources:
  - "frontend/src/pages/AuthPage.tsx"
  - "frontend/src/components/RequireAuth.tsx"
  - "frontend/src/components/AppShell.tsx"
  - "frontend/src/stores/useAppStore.ts"
  - "frontend/src/services/api.ts"
  - "frontend/src/pages/BillingPage.tsx"
  - "frontend/src/pages/BetaPage.tsx"
  - "frontend/src/pages/MarketplacePage.tsx"
  - "frontend/src/pages/OrganizationPage.tsx"
related_documents:
  - "frontend/routing.md"
  - "frontend/state-management.md"
  - "frontend/api-client.md"
  - "frontend/frontend-risks.md"
  - "backend/modules/authentication.md"
  - "backend/modules/tenant-authorization.md"
  - "architecture/boundaries.md"
related_diagrams:
  - "frontend/diagrams/frontend-routing-flow.mmd"
  - "frontend/diagrams/frontend-api-flow.mmd"
uncertainty_markers:
  - "AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01"
  - "STATE_OWNERSHIP_UNCLEAR SOU-01"
  - "NEEDS_OWNER_CONFIRMATION NOC-12"
---

# Frontend Authentication and Access

## Phase 6 Transport Contract

The frontend keeps the access token only in volatile module/store memory and
keeps no JavaScript-readable refresh token. Shared calls marked `auth: true`
attach `Authorization: Bearer` only to the configured API origin and, when
selected, `X-Organization-Id`; requests include credentials for the `HttpOnly`
refresh cookie. A stable invalid-access-token response can trigger one
coordinated refresh and one replay.

Backend details and boundaries are in [API Authentication](../api/authentication.md), [API Authorization](../api/authorization.md), and [API Tenant Context](../api/tenant-context.md). Page-preview WebSockets use a short-lived one-time ticket in `Sec-WebSocket-Protocol`; their URLs contain no credentials or organization context.

## Scope Boundary

This document records browser behavior: authentication forms, session persistence, route admission, organization selection, and visible role-based interaction. It does not certify security controls. Backend authentication, tenant membership, role enforcement, and resource authorization remain authoritative and receive deeper treatment in Phase 7.

## Authentication Flow

1. `/login` displays login or registration mode.
2. Controlled form state is submitted through `api.auth.login` or `api.auth.register`.
3. A successful response is passed to `useAppStore.setSession`.
4. The access token enters volatile memory; non-secret identity projections and
   active organization state may be cached separately.
5. Navigation moves to `/`.
6. `RequireAuth` sees authenticated bootstrap state and renders `AppShell`.
7. Authenticated API methods add bearer and organization headers.

The authentication form source contains pre-populated development credential values. They are not reproduced in OKF. This behavior is recorded as a frontend risk because a production-like bundle could display them.

## Session Restoration and Expiry

On module load, authentication starts as `unknown`. `SessionBootstrap` performs
a coordinated refresh using the `HttpOnly` cookie. Protected routes show a
neutral restoring state until bootstrap becomes authenticated or
unauthenticated. Only a `401 access_token_invalid` response is refreshable, and
the original request is replayed at most once.

One promise coordinates refresh inside a tab. Web Locks provide the primary
cross-tab critical section, while BroadcastChannel carries transient session
and logout events and provides the bounded fallback election. Logout calls the
cookie endpoint and clears local state in every listening tab whether that call
succeeds or fails.

## Route Admission

| Boundary | Browser rule | Limitation |
|---|---|---|
| Public authentication | `/login` has no route guard | An already authenticated user is not globally redirected away by router configuration. |
| Protected application | Authenticated bootstrap state renders `AppShell` | Backend authentication remains authoritative. |
| Feature routes | Every protected route shares the same guard | No route metadata declares roles or permissions. |
| Unknown protected route | Redirect to `/` | No not-found or denied distinction. |

## Visible Role Cues

| Feature | Browser-visible rule | Scope |
|---|---|---|
| Organizations | owner/admin manage; owner-only operations protect ownership transitions | Membership role from loaded organization detail |
| Billing | owner/admin/billing manager enable management actions | Active membership from store |
| Beta | owner/admin/editor manage organization beta items | Active membership from store |
| Beta product dashboard | admin/super-admin | Global user role from store |
| Marketplace installations | owner/admin | Active membership from store |
| Marketplace platform review and global runtime controls | admin/super-admin | Global user role from store |
| Other protected routes | No route-specific UI gate found | Shared token guard only |

`AppShell` displays all navigation items for every client-authenticated user. Some actions are hidden, conditionally rendered, or disabled within pages; other pages rely entirely on backend denial.

## Organization Context

The active organization is selected from stored memberships and sent in `X-Organization-Id` for authenticated client methods. The shell remounts the current page when this value changes. The workspace redirect can select a membership by slug, but ordinary feature URLs do not carry organization identity.

## ABV-01: Frontend Authorization Is Not Verified Security

Role checks are visible source behavior but can be bypassed by direct requests or modified client state. They may also drift from backend rules. Phase 4 therefore uses `AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01` for client-side enforcement claims. Phase 3 backend documents confirm general authentication and tenant enforcement mechanisms but do not convert each frontend cue into a complete route-by-route authorization proof.

## Browser Storage and URL Exposure

Legacy access/refresh storage keys are deleted and ignored. Access tokens remain
in volatile memory; refresh tokens remain only in the `HttpOnly` cookie.
`PagesPage` requests a new one-time preview ticket for every connection or
reconnect and never places credentials or organization IDs in the WebSocket
URL. A same-origin script compromise can still read a live in-memory access
token or transient same-origin session message.

## Access Failure Behavior

- Auth form errors render inline through `StatusBadge`.
- Protected-page authorization errors normally surface as page-local API errors.
- No central access-denied page or status-specific error renderer exists.
- Stable invalid-access-token `401` responses receive one coordinated refresh;
  generic `401` and every `403` are returned without recursive retry.
- Disabled UI controls may provide no reason except nearby copy in selected Marketplace flows.

## Related Documents

- [Routing](routing.md)
- [State Management](state-management.md)
- [API Client](api-client.md)
- [Frontend Risks](frontend-risks.md)
- [Backend Authentication](../backend/modules/authentication.md)
- [Backend Tenant Authorization](../backend/modules/tenant-authorization.md)
- [System Boundaries](../architecture/boundaries.md)
- [Security Authentication Architecture](../security/authentication-architecture.md)
- [Security Session and Token Lifecycle](../security/session-and-token-lifecycle.md)
- [Frontend Security Boundaries](../security/frontend-security-boundaries.md)

## Phase 7 Security Interpretation

The access token remains in volatile memory, while non-secret user,
membership, and active-organization projections may use `localStorage`; the
refresh credential remains in an `HttpOnly` cookie. `RequireAuth` waits for
bootstrap state, and role-based UI cues are `FRONTEND_ONLY_SECURITY_CHECK
FOSC-01`. Backend middleware and handler/RLS decisions remain authoritative.
