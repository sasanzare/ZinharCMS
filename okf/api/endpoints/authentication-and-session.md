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

This family contains 6 registered handler-method endpoints. Access is **Four public endpoints and two bearer-authenticated endpoints**.

Module discovery, registration, login, refresh-token rotation, logout/revocation, and current-user/session context.

Exact method/path, stable endpoint ID, handler, access zone, input extractor, return type, OpenAPI status, and frontend coverage are recorded in the [Endpoint Catalog](../endpoint-catalog.md).

## Request Contract

Registration and login use JSON. Refresh/logout accept the `zinhar_refresh_token` cookie or JSON token body; logout and `me` require bearer access tokens.

## Response Contract

`AuthResponse`, `LogoutResponse`, `MeResponse`, and module status; auth issuance sets the refresh cookie.

## Ownership and Persistence

- Backend owner: [Authentication](../../backend/modules/authentication.md)
- Persistence: [relevant entity documentation](../../database/entities/identity-and-global-rbac.md)
- Route group: [Authentication](../groups/authentication.md)

## Frontend Contract

Five operations are wrapped under `api.auth`; module discovery is backend-only.

## OpenAPI and Verification

All handlers are listed, but the bearer and cookie security model is absent.

JWT/password/security helpers have tests; full cookie rotation through the router is unverified.

## Change Checklist

Review route registration, middleware zone, DTO and Serde behavior, role and ownership checks, tenant/RLS use, success and error statuses, frontend method/type/callers, OpenAPI, persistence and side effects, and representative positive and negative tests.
