---
okf_document_id: "api-group-editorial-comments"
title: "Editorial Comments Route Group"
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
  - "backend/src/routes/comments.rs"
related_documents:
  - "api/route-group-catalog.md"
  - "api/endpoint-catalog.md"
  - "backend/modules/comments.md"
related_diagrams:
  - "api/diagrams/api-route-map.mmd"
uncertainty_markers:
  - "OPENAPI_IMPLEMENTATION_CONFLICT OIC-01"
---

# Editorial Comments Route Group

[Back to the Route Group Catalog](../route-group-catalog.md)

## Boundary

This group contains 6 registered handler-method endpoints from `backend/src/routes/comments.rs`. Its access zone is **Tenant protected**, and its principal path surface is `/api/comments`.

Read, write, and management capabilities differ by organization role. Entity association uses `entity_type` and `entity_id` and must remain tenant coherent.

## Endpoint Families

[Editorial Comments](../endpoints/editorial-comments.md) provide the task-oriented contracts. The exhaustive registered method/path list is in the [Endpoint Catalog](../endpoint-catalog.md).

## Ownership and Data

- Backend: [Comments](../../backend/modules/comments.md)
- Persistence: [relevant database documentation](../../database/entities/editorial-comments.md)
- Route registration: `backend/src/routes/comments.rs`

## Frontend Coverage

Five mutation/list operations have wrappers; the single-comment detail handler does not.

## OpenAPI and Tests

All six handlers are included, without bearer, tenant, or role declarations.

No route-local comment contract tests were found; service and frontend feature tests provide only indirect evidence.

## Change Checklist

Verify router placement, request and response DTOs, role and ownership checks, tenant/RLS behavior, frontend methods and types, OpenAPI annotations, status/error behavior, persistence effects, side effects, and representative tests before changing this group.
