---
okf_document_id: "api-endpoints-authentication-session"
title: "Authentication and Session Endpoints"
project: "ZinharCMS"
category: "api-endpoint-family"
phase: 6
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "eed1e0dbdf6d873457d1165158b3c8fbfd6647e1"
last_verified_date: "2026-07-18"
primary_sources:
  - "backend/src/routes"
related_documents:
  - "api/endpoint-catalog.md"
  - "api/groups/authentication.md"
  - "backend/modules/authentication.md"
  - "database/entities/identity-and-global-rbac.md"
uncertainty_markers:
  - "AUTHENTICATION_REQUIREMENT_UNCLEAR ARU-01"
---

# Authentication and Session Endpoints

[Back to the Endpoint Catalog](../endpoint-catalog.md) · [Owning route group](../groups/authentication.md)

## Family Boundary

This family contains 18 registered handler-method endpoints. Access is **six
public/cookie-boundary endpoints and twelve bearer-authenticated endpoints**.

Module discovery, registration, login, refresh-token rotation, current-family
logout, session inventory, owned-session revocation, logout-all,
super-admin-targeted bulk revocation, current-user context, TOTP enrollment,
MFA login completion, recovery-code replacement, MFA disable, and Step-Up.

Exact method/path, stable endpoint ID, handler, access zone, input extractor, return type, OpenAPI status, and frontend coverage are recorded in the [Endpoint Catalog](../endpoint-catalog.md).

## Request Contract

Registration and login use JSON. Refresh/logout accept only the
`zinhar_refresh_token` cookie and validate browser Origin when present. Session
inventory uses bearer authentication and pagination. Owned revoke and
logout-all use bearer authentication and also validate browser Origin because
they can clear or invalidate cookie-backed sessions. Privileged bulk revocation
uses bearer authentication, an authoritative current `super_admin` role, and
Step-Up. MFA enrollment requires password confirmation. MFA disable,
recovery-code replacement, session revocation, and logout-all require an exact
scope-bound `X-Step-Up-Token`.

## Response Contract

`AuthResponse`, `MfaLoginRequiredResponse`, MFA status/enrollment/recovery
responses, Step-Up challenge/grant responses, `LogoutResponse`, `MeResponse`,
`SessionPage`, `RevokeSessionResponse`, `LogoutAllResponse`, and module status.
Only completed AAL1 or AAL2 auth issuance sets the refresh cookie.

## Ownership and Persistence

- Backend owner: [Authentication](../../backend/modules/authentication.md)
- Persistence: [relevant entity documentation](../../database/entities/identity-and-global-rbac.md)
- Route group: [Authentication](../groups/authentication.md)

## Frontend Contract

Login, registration, refresh, logout, current-user, session inventory,
owned-session revoke, logout-all, MFA enrollment/management, MFA login
completion, and Step-Up are wrapped under `api.auth`; module discovery and
privileged bulk revocation are backend-only.

## OpenAPI and Verification

All handlers are listed, but the bearer and cookie security model is absent.

JWT/password/session/MFA helpers have unit and live transactional tests. Phase 6
adds live PostgreSQL/Redis concurrency, replay, migration, browser enrollment,
recovery, Step-Up, storage, and disable evidence. An exhaustive router-level
cookie and every-sensitive-route matrix remains deferred.

## Change Checklist

Review route registration, middleware zone, DTO and Serde behavior, role and ownership checks, tenant/RLS use, success and error statuses, frontend method/type/callers, OpenAPI, persistence and side effects, and representative positive and negative tests.
