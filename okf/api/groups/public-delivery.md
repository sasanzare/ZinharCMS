---
okf_document_id: "api-group-public-delivery"
title: "Public Delivery Route Group"
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
  - "backend/src/routes/delivery.rs"
related_documents:
  - "api/route-group-catalog.md"
  - "api/endpoint-catalog.md"
  - "backend/modules/public-delivery-cache.md"
related_diagrams:
  - "api/diagrams/api-route-map.mmd"
uncertainty_markers:
  - "OPENAPI_IMPLEMENTATION_CONFLICT OIC-01"
---

# Public Delivery Route Group

[Back to the Route Group Catalog](../route-group-catalog.md)

## Boundary

This group contains 8 registered handler-method endpoints from `backend/src/routes/delivery.rs`. Its access zone is **Public**, and its principal path surface is `/api/v1`.

Routes resolve the default public organization internally and expose only published content. Delivery cache invalidation is triggered by related mutations.

## Endpoint Families

[Public Delivery](../endpoints/public-delivery.md) provide the task-oriented contracts. The exhaustive registered method/path list is in the [Endpoint Catalog](../endpoint-catalog.md).

## Ownership and Data

- Backend: [Public Delivery Cache](../../backend/modules/public-delivery-cache.md)
- Persistence: [relevant database documentation](../../database/entities/public-settings-and-navigation.md)
- Route registration: `backend/src/routes/delivery.rs`

## Frontend Coverage

None of the eight endpoints uses the administration shared client.

## OpenAPI and Tests

Six JSON handlers are included. Sitemap and robots handlers are registered but missing from OpenAPI.

Three route-local tests cover filter parsing and XML escaping; end-to-end cache, tenant selection, and representation tests were not found.

## Change Checklist

Verify router placement, request and response DTOs, role and ownership checks, tenant/RLS behavior, frontend methods and types, OpenAPI annotations, status/error behavior, persistence effects, side effects, and representative tests before changing this group.
