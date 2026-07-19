---
okf_document_id: "api-group-beta-release"
title: "Beta Release Route Group"
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
  - "backend/src/routes/beta.rs"
related_documents:
  - "api/route-group-catalog.md"
  - "api/endpoint-catalog.md"
  - "backend/modules/beta-release-operations.md"
related_diagrams:
  - "api/diagrams/api-route-map.mmd"
uncertainty_markers:
  - "OPENAPI_IMPLEMENTATION_CONFLICT OIC-01"
---

# Beta Release Route Group

[Back to the Route Group Catalog](../route-group-catalog.md)

## Boundary

This group contains 9 registered handler-method endpoints from `backend/src/routes/beta.rs`. Its access zone is **Two authenticated global-admin endpoints; seven tenant endpoints**, and its principal path surface is `/api/beta`.

Product-dashboard and participant upsert use the authenticated subtree and global role checks. Feedback, GA blockers, and the tenant dashboard use tenant context.

## Endpoint Families

[Beta Release Operations](../endpoints/beta-release-operations.md) provide the task-oriented contracts. The exhaustive registered method/path list is in the [Endpoint Catalog](../endpoint-catalog.md).

## Ownership and Data

- Backend: [Beta Release Operations](../../backend/modules/beta-release-operations.md)
- Persistence: [relevant database documentation](../../database/entities/beta-release-records.md)
- Route registration: `backend/src/routes/beta.rs`

## Frontend Coverage

All nine handlers have frontend wrappers.

## OpenAPI and Tests

All nine handlers are included; their actual authentication and tenant split is not expressed through OpenAPI security.

Four route-local unit tests cover selected validation and dashboard behavior; no complete router authorization matrix was found.

## Change Checklist

Verify router placement, request and response DTOs, role and ownership checks, tenant/RLS behavior, frontend methods and types, OpenAPI annotations, status/error behavior, persistence effects, side effects, and representative tests before changing this group.
