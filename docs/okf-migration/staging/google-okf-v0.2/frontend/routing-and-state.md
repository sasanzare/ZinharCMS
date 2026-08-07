---
type: Component
title: Routing and State
description: Client route protection, API session bootstrap, volatile access-token handling, and organization context.
status: draft
sources:
  - id: source-router
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/frontend/src/router.tsx
    title: frontend/src/router.tsx at construction commit
  - id: source-api
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/frontend/src/services/api.ts
    title: frontend/src/services/api.ts at construction commit
  - id: source-store
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/frontend/src/stores/useAppStore.ts
    title: frontend/src/stores/useAppStore.ts at construction commit
---

# Client session and navigation

The React router guards the authenticated feature tree and directs anonymous
users to login. The API client obtains its origin from `VITE_API_URL`, sends
credentials only to the trusted API origin, attaches an in-memory bearer access
token, and sends `X-Organization-Id` when an active organization is selected.

The access token is held in module memory. The client explicitly removes the
older access/refresh storage keys, while local storage retains user,
organization, and active-organization projections used to restore UI context.
Refresh is attempted once after the designated access-token failure response;
the refresh credential is handled through the server cookie contract.

The store bootstraps the session, tracks active organization, clears the
session projection, and publishes browser-session changes. This is an observed
implementation description, not an owner-approved browser privacy or session
retention policy. The server side is described in [authentication and sessions](/security/authentication-and-sessions.md), and the route families are summarized in [API contract overview](/api/api-contract-overview.md).
