---
okf_document_id: "api-endpoints-pages-workflow-preview"
title: "Pages, Workflow, Versions, and Preview Endpoints"
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
  - "api/groups/pages-components-and-preview.md"
  - "backend/modules/pages-builder-preview.md"
  - "database/entities/pages-and-versions.md"
uncertainty_markers:
  - "AUTHENTICATION_REQUIREMENT_UNCLEAR ARU-01"
  - "RESPONSE_CONTRACT_UNCLEAR RSCU-01"
---

# Pages, Workflow, Versions, and Preview Endpoints

[Back to the Endpoint Catalog](../endpoint-catalog.md) · [Owning route group](../groups/pages-components-and-preview.md)

## Family Boundary

This family contains 15 registered handler-method endpoints. Access is **Tenant protected; preview has query-token/query-tenant transport exceptions**.

Page list/create/detail/by-slug/update/delete, workflow transitions, version list/restore, and WebSocket preview.

Exact method/path, stable endpoint ID, handler, access zone, input extractor, return type, OpenAPI status, and frontend coverage are recorded in the [Endpoint Catalog](../endpoint-catalog.md).

## Request Contract

Page JSON is validated against component keys. Lists support page/per-page/status/sort. Delete requires confirmation. Preview uses page UUID plus bearer/header or preview query alternatives.

## Response Contract

Page/version objects, paged page lists, and WebSocket text messages containing serialized page JSON.

## Ownership and Persistence

- Backend owner: [Pages Builder Preview](../../backend/modules/pages-builder-preview.md)
- Persistence: [relevant entity documentation](../../database/entities/pages-and-versions.md)
- Route group: [Pages, Components, and Preview](../groups/pages-components-and-preview.md)

## Frontend Contract

Twelve JSON operations have wrappers; page detail/by-slug and WebSocket preview are consumed outside the shared request map or uncovered.

## OpenAPI and Verification

All handlers are listed, but the WebSocket protocol and preview query credentials are incomplete.

Page/domain/frontend builder tests exist; no full router-plus-WebSocket lifecycle suite was found.

## Change Checklist

Review route registration, middleware zone, DTO and Serde behavior, role and ownership checks, tenant/RLS use, success and error statuses, frontend method/type/callers, OpenAPI, persistence and side effects, and representative positive and negative tests.
