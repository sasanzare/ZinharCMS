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

This family contains 10 registered handler-method endpoints. Access is **Five
public/cookie-boundary endpoints and five bearer-authenticated endpoints**.

Module discovery, registration, login, refresh-token rotation, current-family
logout, session inventory, owned-session revocation, logout-all,
super-admin-targeted bulk revocation, and current-user context.

Exact method/path, stable endpoint ID, handler, access zone, input extractor, return type, OpenAPI status, and frontend coverage are recorded in the [Endpoint Catalog](../endpoint-catalog.md).

## Request Contract

Registration and login use JSON. Refresh/logout accept only the
`zinhar_refresh_token` cookie and validate browser Origin when present. Session
inventory uses bearer authentication and pagination. Owned revoke and
logout-all use bearer authentication and also validate browser Origin because
they can clear or invalidate cookie-backed sessions. Privileged bulk revocation
uses bearer authentication and an authoritative current `super_admin` role.

## Response Contract

`AuthResponse`, `LogoutResponse`, `MeResponse`, `SessionPage`,
`RevokeSessionResponse`, `LogoutAllResponse`, and module status; auth issuance
sets the refresh cookie.

## Ownership and Persistence

- Backend owner: [Authentication](../../backend/modules/authentication.md)
- Persistence: [relevant entity documentation](../../database/entities/identity-and-global-rbac.md)
- Route group: [Authentication](../groups/authentication.md)

## Frontend Contract

Login, registration, refresh, logout, current-user, session inventory,
owned-session revoke, and logout-all are wrapped under `api.auth`; module
discovery and privileged bulk revocation are backend-only.

## OpenAPI and Verification

All handlers are listed, but the bearer and cookie security model is absent.

JWT/password/session helpers have unit and live transactional tests. Phase 3
adds cookie Origin tests and authenticated browser bootstrap/logout evidence;
an exhaustive router-level cookie matrix remains deferred.

## Change Checklist

Review route registration, middleware zone, DTO and Serde behavior, role and ownership checks, tenant/RLS use, success and error statuses, frontend method/type/callers, OpenAPI, persistence and side effects, and representative positive and negative tests.
