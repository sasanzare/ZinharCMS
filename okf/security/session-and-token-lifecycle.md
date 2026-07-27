---
okf_document_id: "security-session-token-lifecycle"
title: "Session and Token Lifecycle"
project: "ZinharCMS"
category: "security"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "partially_verified"
last_verified_commit: "ff148ff9"
last_verified_date: "2026-07-27"
primary_sources:
  - "backend/src/routes/auth.rs"
  - "backend/src/services/jwt.rs"
  - "backend/src/config.rs"
  - "frontend/src/stores/useAppStore.ts"
  - "frontend/src/services/api.ts"
related_documents:
  - "authentication-architecture.md"
  - "authentication-flows.md"
  - "frontend-security-boundaries.md"
related_diagrams:
  - "diagrams/session-token-lifecycle.mmd"
---

# Session and Token Lifecycle

## Lifecycle Summary

| Artifact | Creation | Storage | Default lifetime | Revocation/expiry |
| --- | --- | --- | --- | --- |
| Access token | Register, login, refresh | Browser memory only; not persisted server-side | 3,600 seconds | Expiry plus current user/role/auth-version rejection |
| Refresh token | Register, login, refresh | HttpOnly browser cookie; hash in PostgreSQL | 604,800 seconds absolute family lifetime | Atomic rotation; reuse compromises/revokes family; logout revokes selected family |
| Claims | Access-token payload | Request extension after current database verification | Same as access token | Recreated per authenticated request |
| Frontend identity projection | Successful auth response | Zustand plus non-secret user/organization cache | No independent timeout | Failed bootstrap, logout, or remote logout clears it |

The listed lifetimes are parser defaults and may be overridden by environment variables.

## Access Token States

Issued access tokens carry `auth_version`. Authentication and tenant middleware
reload current active user, global role, and authentication version. Deactivation,
reactivation, credential changes, and global-role changes invalidate earlier
claims without a token denylist. JWT signing-key rotation remains an operational
area.

## Refresh Rotation

Every login/registration creates a refresh family with absolute expiry. Refresh
locks the token, family, and user, creates exactly one linked successor, and
commits atomically. Concurrent/replayed use marks the family compromised and
revokes it. Logout revokes the cookie-selected family. A retention cleanup job
for expired rows remains deferred.

## Cookie Attributes

`zinhar_refresh_token` is `HttpOnly`, `SameSite=Lax`, scoped to `/api/auth`, and uses configured `Max-Age`. `Secure` is conditional on `COOKIE_SECURE`; no `Domain` attribute is emitted. `COOKIE_SECURITY_UNVERIFIED CSU-01` covers the uninspected deployed value, HTTPS termination, and proxy behavior.

## Frontend Behavior

The API client sends credentials only to the trusted API origin and keeps the
access token in memory. Startup uses the refresh cookie before protected
rendering. In-tab refresh is single-flight; tabs coordinate through Web Locks
and transient BroadcastChannel messages. Only the stable
`access_token_invalid` response triggers one refresh and one request replay.
Logout is broadcast; storage events and browser storage never carry tokens.

## Open Questions

- `SESSION_LIFECYCLE_UNCLEAR SLU-01`: bootstrap, expiry retry, cross-tab logout,
  and current authorization invalidation are implemented; compatibility without
  Web Locks/BroadcastChannel and deployed browser behavior require monitoring.
- `TOKEN_LIFECYCLE_UNCLEAR TLU-01`: family rotation/reuse/concurrency are
  implemented; cleanup retention and signing-key rotation remain open.
- `COOKIE_SECURITY_UNVERIFIED CSU-01`: deployed cookie security cannot be inferred from a configurable flag.
