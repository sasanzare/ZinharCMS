---
okf_document_id: "api-group-marketplace-runtime"
title: "Marketplace Runtime Route Group"
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
  - "backend/src/routes/marketplace_runtime.rs"
related_documents:
  - "api/route-group-catalog.md"
  - "api/endpoint-catalog.md"
  - "backend/modules/marketplace-runtime-adapters.md"
related_diagrams:
  - "api/diagrams/api-route-map.mmd"
uncertainty_markers:
  - "OPENAPI_IMPLEMENTATION_CONFLICT OIC-01"
---

# Marketplace Runtime Route Group

[Back to the Route Group Catalog](../route-group-catalog.md)

## Boundary

This group contains 6 registered handler-method endpoints from `backend/src/routes/marketplace_runtime.rs`. Its access zone is **Tenant protected**, and its principal path surface is Marketplace permissions, runtime status, and kill-switch paths.

Organization kill switches require owner/admin capability; global kill-switch operations require global administrative authority.

## Endpoint Families

[Marketplace Runtime Security](../endpoints/marketplace-runtime-security.md) provide the task-oriented contracts. The exhaustive registered method/path list is in the [Endpoint Catalog](../endpoint-catalog.md).

## Ownership and Data

- Backend: [Marketplace Runtime Adapters](../../backend/modules/marketplace-runtime-adapters.md)
- Persistence: [relevant database documentation](../../database/entities/marketplace-installations-and-runtime-adapters.md)
- Route registration: `backend/src/routes/marketplace_runtime.rs`

## Frontend Coverage

All six handlers have frontend wrappers.

## OpenAPI and Tests

All six handlers are included, but organization/global role and permission-approval requirements are not represented.

One route-local runtime test and supporting service tests exist; negative authorization and kill-switch precedence need router coverage.

## Change Checklist

Verify router placement, request and response DTOs, role and ownership checks, tenant/RLS behavior, frontend methods and types, OpenAPI annotations, status/error behavior, persistence effects, side effects, and representative tests before changing this group.
