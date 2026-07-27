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

Login and registration issue a JWT access token and an opaque refresh token.
The JSON `AuthResponse` includes `access_token`, `token_type`, `expires_in`,
the user, organization memberships, and a default organization ID; it never
contains the refresh token. The refresh credential is set only as the
`zinhar_refresh_token` cookie.

The cookie is `HttpOnly`, `SameSite=Lax`, scoped to `/api/auth`, uses the
configured refresh lifetime as `Max-Age`, and adds `Secure` when
`COOKIE_SECURE` is enabled. Refresh rotates a transactional token family.
Logout requires no access token, revokes the cookie-selected family when
present, and deterministically clears the cookie. Refresh/logout validate
browser Origin when present.

## Access Token Transport

Standard protected calls use:

```http
Authorization: Bearer <access-token>
```

`auth_middleware` validates a bearer token and inserts `Claims`.
`tenant_middleware` performs equivalent token validation while also
establishing organization context. Neither middleware accepts token query
parameters. The special preview WebSocket path instead consumes a short-lived
one-time ticket from `Sec-WebSocket-Protocol`.

## Public Authentication Endpoints

`GET /api/auth`, registration, login, refresh, and cookie logout are outside
bearer middleware. Current-user lookup is in the authenticated subtree.
Registration validates a basic email shape, password length of at least eight
characters, and non-empty name. Login is subject to IP-based failure limiting.

## Frontend Session Behavior

The frontend keeps the access token in volatile memory, stores no
JavaScript-readable refresh token, and caches only non-secret identity and
organization state. It attaches bearer and tenant headers only to the trusted
API origin. Refresh is single-flight per tab and coordinated across tabs; only
stable invalid-access-token responses receive one refresh and one replay.

## Boundary Notes

- Authentication proves token validity and user identity; it does not itself grant organization or operation permission.
- Query-string preview tokens can appear in browser or proxy logs and should be treated as a constrained compatibility path.
- `AUTHENTICATION_REQUIREMENT_UNCLEAR ARU-01` applies when a future handler is moved between router subtrees without matching annotations and tests. No registered endpoint had an unresolved access zone in this snapshot.
- Generated OpenAPI does not declare a bearer scheme, so it cannot currently communicate authentication requirements accurately.

## Phase 7 Detail

[Authentication Architecture](../security/authentication-architecture.md), [Authentication Flows](../security/authentication-flows.md), [Session and Token Lifecycle](../security/session-and-token-lifecycle.md), and [Password and Credential Handling](../security/password-and-credential-handling.md) add cryptographic, storage, cookie, bootstrap, rate-limit, and lifecycle evidence. Key markers are `AFU-01`, `SLU-01`, `TLU-01`, `CSU-01`, and `PSE-01`.
