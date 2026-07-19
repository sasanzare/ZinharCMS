---
okf_document_id: "api-authentication"
title: "API Authentication"
project: "ZinharCMS"
category: "api-security"
phase: 6
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "eed1e0dbdf6d873457d1165158b3c8fbfd6647e1"
last_verified_date: "2026-07-18"
primary_sources:
  - "backend/src/routes/auth.rs"
  - "backend/src/middleware/auth.rs"
  - "backend/src/services/jwt.rs"
  - "frontend/src/services/api.ts"
related_documents:
  - "api/authorization.md"
  - "api/tenant-context.md"
  - "api/endpoints/authentication-and-session.md"
  - "frontend/authentication-and-access.md"
related_diagrams:
  - "api/diagrams/authentication-flow.mmd"
uncertainty_markers:
  - "AUTHENTICATION_REQUIREMENT_UNCLEAR ARU-01"
---

# API Authentication

## Token Model

Login and registration issue a JWT access token and a rotating opaque refresh token. The JSON `AuthResponse` includes `access_token`, optional `refresh_token`, `token_type`, `expires_in`, the user, organization memberships, and a default organization ID. The refresh token is also set as the `zinhar_refresh_token` cookie.

The cookie is `HttpOnly`, `SameSite=Lax`, scoped to `/api/auth`, uses the configured refresh lifetime as `Max-Age`, and adds `Secure` when `COOKIE_SECURE` is enabled. Refresh revokes the prior stored token and issues a new pair. Logout requires an access token, revokes the supplied refresh token when present, and clears the cookie.

## Access Token Transport

Standard protected calls use:

```http
Authorization: Bearer <access-token>
```

`auth_middleware` validates the token and inserts `Claims`. `tenant_middleware` performs equivalent token validation while also establishing organization context. The preview WebSocket path may receive the token through `access_token` or `token` query parameters because browser WebSocket construction cannot set arbitrary headers.

## Public Authentication Endpoints

`GET /api/auth`, registration, login, and refresh are public. Logout and current-user lookup are in the authenticated subtree. Registration validates a basic email shape, password length of at least eight characters, and non-empty name. Login is subject to IP-based failure limiting.

## Frontend Session Behavior

The frontend stores the access token, refresh token, and selected organization ID in local storage and sends credentials with requests. It attaches bearer and tenant headers only when a call is marked `auth: true`. It exposes explicit refresh but has no shared automatic refresh or retry interceptor.

## Boundary Notes

- Authentication proves token validity and user identity; it does not itself grant organization or operation permission.
- Query-string preview tokens can appear in browser or proxy logs and should be treated as a constrained compatibility path.
- `AUTHENTICATION_REQUIREMENT_UNCLEAR ARU-01` applies when a future handler is moved between router subtrees without matching annotations and tests. No registered endpoint had an unresolved access zone in this snapshot.
- Generated OpenAPI does not declare a bearer scheme, so it cannot currently communicate authentication requirements accurately.
