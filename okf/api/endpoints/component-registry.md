---
okf_document_id: "api-endpoints-component-registry"
title: "Component Registry Endpoints"
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
  - "database/entities/component-and-plugin-registry.md"
uncertainty_markers:
  - "AUTHORIZATION_REQUIREMENT_UNCLEAR AZU-01"
---

# Component Registry Endpoints

[Back to the Endpoint Catalog](../endpoint-catalog.md) · [Owning route group](../groups/pages-components-and-preview.md)

## Family Boundary

This family contains 5 registered handler-method endpoints. Access is **Tenant protected; mutation requires component-management capability**.

Component list, create, get, update, and delete.

Exact method/path, stable endpoint ID, handler, access zone, input extractor, return type, OpenAPI status, and frontend coverage are recorded in the [Endpoint Catalog](../endpoint-catalog.md).

## Request Contract

List accepts optional category. Component keys identify detail/mutation paths; JSON DTOs define metadata and schemas.

## Response Contract

Component objects or arrays.

## Ownership and Persistence

- Backend owner: [Pages Builder Preview](../../backend/modules/pages-builder-preview.md)
- Persistence: [relevant entity documentation](../../database/entities/component-and-plugin-registry.md)
- Route group: [Pages, Components, and Preview](../groups/pages-components-and-preview.md)

## Frontend Contract

Only component listing has a shared-client wrapper; CRUD details and mutations do not.

## OpenAPI and Verification

All five handlers are included without role and tenant declarations.

Component/page validation has indirect tests; no complete component registry HTTP suite was found.

## Change Checklist

Review route registration, middleware zone, DTO and Serde behavior, role and ownership checks, tenant/RLS use, success and error statuses, frontend method/type/callers, OpenAPI, persistence and side effects, and representative positive and negative tests.
