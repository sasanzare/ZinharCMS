---
okf_document_id: "api-group-pages-components-preview"
title: "Pages, Components, and Preview Route Group"
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
  - "backend/src/routes/pages.rs"
related_documents:
  - "api/route-group-catalog.md"
  - "api/endpoint-catalog.md"
  - "backend/modules/pages-builder-preview.md"
related_diagrams:
  - "api/diagrams/api-route-map.mmd"
uncertainty_markers:
  - "OPENAPI_IMPLEMENTATION_CONFLICT OIC-01"
---

# Pages, Components, and Preview Route Group

[Back to the Route Group Catalog](../route-group-catalog.md)

## Boundary

This group contains 20 registered handler-method endpoints from `backend/src/routes/pages.rs`. Its access zone is **Tenant protected**, and its principal path surface is `/api/pages`, `/api/component-registry`, and `/api/preview`.

The preview route is in the tenant subtree but has query-parameter exceptions in authentication and tenant middleware. Component keys are referenced inside page JSON.

## Endpoint Families

[Pages Workflow Versions And Preview](../endpoints/pages-workflow-versions-and-preview.md), [Component Registry](../endpoints/component-registry.md) provide the task-oriented contracts. The exhaustive registered method/path list is in the [Endpoint Catalog](../endpoint-catalog.md).

## Ownership and Data

- Backend: [Pages Builder Preview](../../backend/modules/pages-builder-preview.md)
- Persistence: [relevant database documentation](../../database/entities/pages-and-versions.md)
- Route registration: `backend/src/routes/pages.rs`

## Frontend Coverage

Twelve page operations and component listing have JSON wrappers. Page detail/by-slug, component mutations/details, and WebSocket preview use other paths or are uncovered.

## OpenAPI and Tests

All 20 handlers are included. WebSocket protocol, preview query authentication, and role requirements are not fully modeled.

Page validation/workflow and frontend builder tests exist; no complete HTTP plus WebSocket lifecycle suite was found.

## Change Checklist

Verify router placement, request and response DTOs, role and ownership checks, tenant/RLS behavior, frontend methods and types, OpenAPI annotations, status/error behavior, persistence effects, side effects, and representative tests before changing this group.
