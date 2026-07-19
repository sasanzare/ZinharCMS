---
okf_document_id: "security-authentication-flows"
title: "Authentication Flows"
project: "ZinharCMS"
category: "security"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "partially_verified"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/routes/auth.rs"
  - "backend/src/services/security.rs"
  - "frontend/src/pages/AuthPage.tsx"
  - "frontend/src/services/api.ts"
related_documents:
  - "authentication-architecture.md"
  - "session-and-token-lifecycle.md"
  - "audit-and-security-events.md"
related_diagrams:
  - "diagrams/authentication-flow.mmd"
---

# Authentication Flows

## Registration

1. The handler checks a basic email shape, password length of at least eight, and a non-empty name.
2. It normalizes the email, hashes the password, and counts existing users.
3. The first registered user receives global `super_admin`; later users receive global `author`.
4. The user and role assignment are inserted in one transaction and attached to the default organization using a mapped organization role.
5. The server creates an access token and refresh token, persists the refresh hash, returns account/membership context, and sets the refresh cookie.

Registration is public and has no verified registration-specific rate limit, invitation requirement, email verification, CAPTCHA, or approval gate.

## Login

1. Email is normalized and client IP is derived from the first valid `X-Forwarded-For` value or socket address.
2. Failed attempts inside the configured window are counted by IP before credential lookup.
3. An active user and highest-priority assigned global role are loaded.
4. The password is verified; success or failure is recorded.
5. Success issues the same token/cookie response as registration.

The error text is the same for missing users and incorrect passwords. `RATE_LIMITING_STATUS_UNCLEAR RLSU-01` applies because proxy trust configuration is not defined and successful attempts do not clear prior failures.

## Refresh

The server prefers a non-empty JSON refresh token over the cookie. It hashes the token, requires an unrevoked/unexpired row and active user, selects the highest-priority global role, revokes the old row, and issues a new access/refresh pair. Rotation is not one database transaction, so concurrent refresh semantics and failure between revocation and issue remain `TOKEN_LIFECYCLE_UNCLEAR TLU-01`.

## Logout

Logout requires a valid access token. If a refresh token is supplied, its row is revoked; the refresh cookie is cleared regardless. Other refresh tokens for the user and already-issued access tokens remain valid.

## Current User and Frontend Entry

`GET /api/auth/me` loads the active user and current active organization memberships. `RequireAuth` admits the application shell whenever an access-token string exists locally; it does not validate token signature or expiry. Backend middleware remains authoritative.

## Absent or Unclear Flows

Password reset, credential change, account deactivation by users, session listing, logout-all, MFA, email verification, and external identity providers were not found. This is `AUTHENTICATION_FLOW_UNCLEAR AFU-01`, not proof that the product rejects those capabilities permanently.
