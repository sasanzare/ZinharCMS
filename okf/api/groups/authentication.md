---
okf_document_id: "api-group-authentication"
title: "Authentication Route Group"
project: "ZinharCMS"
category: "api-route-group"
phase: 6
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "eed1e0dbdf6d873457d1165158b3c8fbfd6647e1"
last_verified_date: "2026-07-18"
primary_sources:
  - "backend/src/routes/auth.rs"
related_documents:
  - "api/route-group-catalog.md"
  - "api/endpoint-catalog.md"
  - "backend/modules/authentication.md"
related_diagrams:
  - "api/diagrams/api-route-map.mmd"
uncertainty_markers:
  - "OPENAPI_IMPLEMENTATION_CONFLICT OIC-01"
---

# Authentication Route Group

[Back to the Route Group Catalog](../route-group-catalog.md)

## Boundary

This group contains 6 registered handler-method endpoints from `backend/src/routes/auth.rs`. Its access zone is **Four public; two authenticated**, and its principal path surface is `/api/auth`.

Refresh and logout can obtain the refresh token from the scoped cookie or JSON body. Logout additionally requires a valid access token.

## Endpoint Families

[Authentication And Session](../endpoints/authentication-and-session.md) provide the task-oriented contracts. The exhaustive registered method/path list is in the [Endpoint Catalog](../endpoint-catalog.md).

## Ownership and Data

- Backend: [Authentication](../../backend/modules/authentication.md)
- Persistence: [relevant database documentation](../../database/entities/identity-and-global-rbac.md)
- Route registration: `backend/src/routes/auth.rs`

## Frontend Coverage

Five session operations have frontend wrappers; the module-status discovery endpoint does not.

## OpenAPI and Tests

All six handlers are included, but no bearer security scheme is declared for logout or current-user lookup.

Password, JWT, and security helpers have tests; a complete cookie/header/router lifecycle test was not found.

## Change Checklist

Verify router placement, request and response DTOs, role and ownership checks, tenant/RLS behavior, frontend methods and types, OpenAPI annotations, status/error behavior, persistence effects, side effects, and representative tests before changing this group.
