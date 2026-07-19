---
okf_document_id: "api-endpoints-editorial-comments"
title: "Editorial Comment Endpoints"
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
  - "api/groups/editorial-comments.md"
  - "backend/modules/comments.md"
  - "database/entities/editorial-comments.md"
uncertainty_markers:
  - "AUTHORIZATION_REQUIREMENT_UNCLEAR AZU-01"
---

# Editorial Comment Endpoints

[Back to the Endpoint Catalog](../endpoint-catalog.md) · [Owning route group](../groups/editorial-comments.md)

## Family Boundary

This family contains 6 registered handler-method endpoints. Access is **Tenant protected with distinct read, write, and management capabilities**.

Comment list/create/detail, resolve, unresolve, and delete.

Exact method/path, stable endpoint ID, handler, access zone, input extractor, return type, OpenAPI status, and frontend coverage are recorded in the [Endpoint Catalog](../endpoint-catalog.md).

## Request Contract

List filters by `entity_type`, `entity_id`, and optional resolution inclusion. Comment ID paths use UUIDs; create uses JSON.

## Response Contract

Comment objects or arrays.

## Ownership and Persistence

- Backend owner: [Comments](../../backend/modules/comments.md)
- Persistence: [relevant entity documentation](../../database/entities/editorial-comments.md)
- Route group: [Editorial Comments](../groups/editorial-comments.md)

## Frontend Contract

Five operations have wrappers; comment detail GET does not.

## OpenAPI and Verification

All six handlers are included without role/tenant declarations.

No complete route-level comment and tenant-coherence suite was found.

## Change Checklist

Review route registration, middleware zone, DTO and Serde behavior, role and ownership checks, tenant/RLS use, success and error statuses, frontend method/type/callers, OpenAPI, persistence and side effects, and representative positive and negative tests.
