---
okf_document_id: "api-endpoints-content-types"
title: "Content Type Endpoints"
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
  - "REQUEST_CONTRACT_UNCLEAR RCU-01"
---

# Content Type Endpoints

[Back to the Endpoint Catalog](../endpoint-catalog.md) · [Owning route group](../groups/content-and-workflow.md)

## Family Boundary

This family contains 5 registered handler-method endpoints. Access is **Tenant protected; mutation requires content-type management capability**.

List, create, get, update, and confirmation-gated delete of content-type definitions.

Exact method/path, stable endpoint ID, handler, access zone, input extractor, return type, OpenAPI status, and frontend coverage are recorded in the [Endpoint Catalog](../endpoint-catalog.md).

## Request Contract

Create/update JSON includes the content-type definition; detail/delete use UUID paths; delete requires `confirm=true`.

## Response Contract

Content type objects or arrays; mutations return the resulting/deleted object.

## Ownership and Persistence

- Backend owner: [Content Workflow](../../backend/modules/content-workflow.md)
- Persistence: [relevant entity documentation](../../database/entities/content-types-and-entries.md)
- Route group: [Content and Workflow](../groups/content-and-workflow.md)

## Frontend Contract

List/create/update/delete have wrappers; detail GET does not.

## OpenAPI and Verification

All five are included without bearer, tenant, or role declarations.

Validation helpers have indirect coverage; no complete CRUD contract suite was found.

## Change Checklist

Review route registration, middleware zone, DTO and Serde behavior, role and ownership checks, tenant/RLS use, success and error statuses, frontend method/type/callers, OpenAPI, persistence and side effects, and representative positive and negative tests.
