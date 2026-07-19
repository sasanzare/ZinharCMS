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
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
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
| Access token | Register, login, refresh | Browser `localStorage`; not persisted server-side | 3,600 seconds | Expiry only; no server denylist |
| Refresh token | Register, login, refresh | Browser cookie; hash in PostgreSQL; frontend has legacy local-storage support | 604,800 seconds | Individual row revocation or expiry |
| Claims | Access-token payload | Request extension after verification | Same as access token | Recreated per authenticated request |
| Frontend session projection | Successful auth response | Zustand plus `localStorage` | No independent timeout | Explicit local clear/logout |

The listed lifetimes are parser defaults and may be overridden by environment variables.

## Access Token States

Issued access tokens transition directly from valid to expired. There is no persisted session entity, access-token revocation list, JWT ID, user token version, or signing-key identifier. A deactivated user or changed global role may continue using an already-issued token until expiry where a handler does not reload account state.

## Refresh Rotation

Every successful token-pair issue creates a new refresh row. Refresh marks the presented row revoked and creates another row. Logout revokes only the presented refresh token. Old expired and revoked rows have no repository-defined cleanup or retention job.

## Cookie Attributes

`zinhar_refresh_token` is `HttpOnly`, `SameSite=Lax`, scoped to `/api/auth`, and uses configured `Max-Age`. `Secure` is conditional on `COOKIE_SECURE`; no `Domain` attribute is emitted. `COOKIE_SECURITY_UNVERIFIED CSU-01` covers the uninspected deployed value, HTTPS termination, and proxy behavior.

## Frontend Behavior

The API client sends `credentials: include`, stores the access token in `localStorage`, and maintains functions for a refresh token in `localStorage`. The current backend response sets `refresh_token` to `null`, so normal current browser flow relies on the cookie. There is an explicit refresh API function but no shared automatic refresh/retry or expiry timer.

## Open Questions

- `SESSION_LIFECYCLE_UNCLEAR SLU-01`: the UI has no verified automatic handling for expired tokens, cross-tab logout, or server-side session invalidation.
- `TOKEN_LIFECYCLE_UNCLEAR TLU-01`: refresh reuse detection, family revocation, concurrency guarantees, cleanup, and key rotation are absent or unclear.
- `COOKIE_SECURITY_UNVERIFIED CSU-01`: deployed cookie security cannot be inferred from a configurable flag.
