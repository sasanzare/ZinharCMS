---
okf_document_id: "api-group-media"
title: "Media Route Group"
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
  - "backend/src/routes/media.rs"
related_documents:
  - "api/route-group-catalog.md"
  - "api/endpoint-catalog.md"
  - "backend/modules/media.md"
related_diagrams:
  - "api/diagrams/api-route-map.mmd"
uncertainty_markers:
  - "OPENAPI_IMPLEMENTATION_CONFLICT OIC-01"
---

# Media Route Group

[Back to the Route Group Catalog](../route-group-catalog.md)

## Boundary

This group contains 5 registered handler-method endpoints from `backend/src/routes/media.rs`. Its access zone is **Tenant protected**, and its principal path surface is `/api/media`.

Upload enforces the configured file maximum in addition to the tenant router body limit. Stored files become publicly reachable under `/uploads`.

## Endpoint Families

[Media Library](../endpoints/media-library.md) provide the task-oriented contracts. The exhaustive registered method/path list is in the [Endpoint Catalog](../endpoint-catalog.md).

## Ownership and Data

- Backend: [Media](../../backend/modules/media.md)
- Persistence: [relevant database documentation](../../database/entities/media-and-variants.md)
- Route registration: `backend/src/routes/media.rs`

## Frontend Coverage

Four handlers have wrappers; media detail GET does not.

## OpenAPI and Tests

All five handlers are included, but multipart fields, size behavior, tenant headers, and role requirements are incomplete.

Upload-type and media services have selected tests; no full multipart/static-download contract suite was found.

## Change Checklist

Verify router placement, request and response DTOs, role and ownership checks, tenant/RLS behavior, frontend methods and types, OpenAPI annotations, status/error behavior, persistence effects, side effects, and representative tests before changing this group.
