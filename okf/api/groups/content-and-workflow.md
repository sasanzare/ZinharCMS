---
okf_document_id: "api-group-content-workflow"
title: "Content and Workflow Route Group"
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
  - "backend/src/routes/content.rs"
related_documents:
  - "api/route-group-catalog.md"
  - "api/endpoint-catalog.md"
  - "backend/modules/content-workflow.md"
related_diagrams:
  - "api/diagrams/api-route-map.mmd"
uncertainty_markers:
  - "OPENAPI_IMPLEMENTATION_CONFLICT OIC-01"
---

# Content and Workflow Route Group

[Back to the Route Group Catalog](../route-group-catalog.md)

## Boundary

This group contains 16 registered handler-method endpoints from `backend/src/routes/content.rs`. Its access zone is **Tenant protected**, and its principal path surface is `/api/content-types` and `/api/entries`.

Content-type mutation and entry workflow use different RBAC capabilities. Entry list pagination is page-based and workflow mutations return the resulting resource.

## Endpoint Families

[Content Types](../endpoints/content-types.md), [Content Entries And Workflow](../endpoints/content-entries-and-workflow.md) provide the task-oriented contracts. The exhaustive registered method/path list is in the [Endpoint Catalog](../endpoint-catalog.md).

## Ownership and Data

- Backend: [Content Workflow](../../backend/modules/content-workflow.md)
- Persistence: [relevant database documentation](../../database/entities/content-types-and-entries.md)
- Route registration: `backend/src/routes/content.rs`

## Frontend Coverage

Fourteen handlers have shared-client wrappers; content-type and entry detail GET handlers do not.

## OpenAPI and Tests

All 16 handlers are included, without bearer, tenant-header, or role declarations.

Workflow and validation services have tests; no complete content router contract suite was found.

## Change Checklist

Verify router placement, request and response DTOs, role and ownership checks, tenant/RLS behavior, frontend methods and types, OpenAPI annotations, status/error behavior, persistence effects, side effects, and representative tests before changing this group.
