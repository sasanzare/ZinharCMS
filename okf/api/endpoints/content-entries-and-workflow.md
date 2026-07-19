---
okf_document_id: "api-endpoints-content-entries-workflow"
title: "Content Entries and Workflow Endpoints"
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
  - "api/groups/content-and-workflow.md"
  - "backend/modules/content-workflow.md"
  - "database/entities/content-types-and-entries.md"
uncertainty_markers:
  - "AUTHORIZATION_REQUIREMENT_UNCLEAR AZU-01"
  - "RESPONSE_CONTRACT_UNCLEAR RSCU-01"
---

# Content Entries and Workflow Endpoints

[Back to the Endpoint Catalog](../endpoint-catalog.md) · [Owning route group](../groups/content-and-workflow.md)

## Family Boundary

This family contains 11 registered handler-method endpoints. Access is **Tenant protected with writer, reviewer, publisher, and manager capabilities by operation**.

Entry list/create/get/update/delete and submit-review, publish, reject, archive, restore, and unpublish transitions.

Exact method/path, stable endpoint ID, handler, access zone, input extractor, return type, OpenAPI status, and frontend coverage are recorded in the [Endpoint Catalog](../endpoint-catalog.md).

## Request Contract

Type slug and entry UUID identify resources. List supports page/per-page/status/sort. Create/update carries open JSON `data`; delete uses confirmation.

## Response Contract

Entry objects and `{ data, page, per_page }` lists; transitions return the updated entry.

## Ownership and Persistence

- Backend owner: [Content Workflow](../../backend/modules/content-workflow.md)
- Persistence: [relevant entity documentation](../../database/entities/content-types-and-entries.md)
- Route group: [Content and Workflow](../groups/content-and-workflow.md)

## Frontend Contract

Ten operations have wrappers; entry detail GET does not.

## OpenAPI and Verification

All 11 are included without machine-readable role/workflow requirements.

Workflow helpers have tests; route-level transition, audit, webhook, cache, and RLS effects are not comprehensively covered.

## Change Checklist

Review route registration, middleware zone, DTO and Serde behavior, role and ownership checks, tenant/RLS use, success and error statuses, frontend method/type/callers, OpenAPI, persistence and side effects, and representative positive and negative tests.
