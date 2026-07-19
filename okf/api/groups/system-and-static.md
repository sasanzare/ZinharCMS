---
okf_document_id: "api-group-system-static"
title: "System and Static Route Group"
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
  - "backend/src/routes/mod.rs"
related_documents:
  - "api/route-group-catalog.md"
  - "api/endpoint-catalog.md"
  - "backend/modules/bootstrap-runtime.md"
related_diagrams:
  - "api/diagrams/api-route-map.mmd"
uncertainty_markers:
  - "OPENAPI_IMPLEMENTATION_CONFLICT OIC-01"
---

# System and Static Route Group

[Back to the Route Group Catalog](../route-group-catalog.md)

## Boundary

This group contains 4 registered handler-method endpoints from `backend/src/routes/mod.rs`. Its access zone is **Public**, and its principal path surface is `/`, `/health`, `/ready`, `/openapi.json`, and `/uploads/*`.

`/uploads` is a `ServeDir` mount and is excluded from the four-handler count. Readiness checks PostgreSQL and Redis; liveness does not establish dependency readiness.

## Endpoint Families

[System Health Openapi And Static](../endpoints/system-health-openapi-and-static.md) provide the task-oriented contracts. The exhaustive registered method/path list is in the [Endpoint Catalog](../endpoint-catalog.md).

## Ownership and Data

- Backend: [Bootstrap Runtime](../../backend/modules/bootstrap-runtime.md)
- Persistence: [relevant database documentation](../../database/schema-catalog.md)
- Route registration: `backend/src/routes/mod.rs`

## Frontend Coverage

The shared client calls `/`, `/health`, and `/ready`. OpenAPI and static files have no shared JSON client wrapper.

## OpenAPI and Tests

The three system information/probe handlers are included. `/openapi.json` and the static service are not represented as OpenAPI operations.

Configuration and runtime helpers have unit coverage, but no real-router probe/static contract suite was found.

## Change Checklist

Verify router placement, request and response DTOs, role and ownership checks, tenant/RLS behavior, frontend methods and types, OpenAPI annotations, status/error behavior, persistence effects, side effects, and representative tests before changing this group.
