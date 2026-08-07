---
type: API Contract
title: Authentication and Session Contract
description: Current API behavior for bearer access, cookie refresh families, sessions, MFA, and step-up authentication.
status: draft
sources:
  - id: source-auth-route
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/routes/auth.rs
    title: backend/src/routes/auth.rs at construction commit
  - id: source-sessions
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/services/sessions.rs
    title: backend/src/services/sessions.rs at construction commit
  - id: source-mfa
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/services/mfa_accounts.rs
    title: backend/src/services/mfa_accounts.rs at construction commit
  - id: source-config
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/config.rs
    title: backend/src/config.rs at construction commit
---

# Current contract behavior

The API returns a bearer access token to the client and handles the refresh
credential through an opaque, `HttpOnly` cookie. Refresh tokens are rotated in
families; sessions can be listed, revoked individually, or revoked together.
Logout validates the browser-origin boundary, revokes the refresh family, and
clears the cookie. The access token is not stored as a refresh credential in
the response body.

Authentication routes cover registration, login, MFA verification, refresh,
logout, current-user/session operations, MFA enrollment/disable/recovery, step-
up challenges, and administrative revocation. MFA account data is encrypted
with the configured AES-256-GCM key ring; TOTP and recovery-code workflows are
implemented in the authentication services. Redis-backed pre-auth and step-up
challenges are single-use and rate-limited within their configured TTLs.

This is a source-backed current behavior summary. API versioning, deprecation,
compatibility guarantees, recovery policy, and long-term session policy require
owner decisions and are intentionally not inferred.

The security control narrative is in [authentication and sessions](/security/authentication-and-sessions.md), and browser-side handling is in [routing and state](/frontend/routing-and-state.md).
