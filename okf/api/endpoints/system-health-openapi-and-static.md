---
okf_document_id: "api-endpoints-system-health-openapi-static"
title: "System, Health, OpenAPI, and Static Endpoints"
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
  - "api/groups/system-and-static.md"
  - "backend/modules/bootstrap-runtime.md"
  - "database/schema-catalog.md"
uncertainty_markers:
  - "OPENAPI_IMPLEMENTATION_CONFLICT OIC-01"
  - "RESPONSE_CONTRACT_UNCLEAR RSCU-02"
---

# System, Health, OpenAPI, and Static Endpoints

[Back to the Endpoint Catalog](../endpoint-catalog.md) · [Owning route group](../groups/system-and-static.md)

## Family Boundary

This family contains 4 registered handler-method endpoints. Access is **Public**.

API discovery, liveness, dependency readiness, runtime OpenAPI JSON, and the separately mounted `/uploads/*` static-file surface.

Exact method/path, stable endpoint ID, handler, access zone, input extractor, return type, OpenAPI status, and frontend coverage are recorded in the [Endpoint Catalog](../endpoint-catalog.md).

## Request Contract

No handler body. Readiness uses application state. Static paths are delegated to `ServeDir`.

## Response Contract

JSON discovery/probe objects; OpenAPI JSON; file responses from `ServeDir`.

## Ownership and Persistence

- Backend owner: [Bootstrap Runtime](../../backend/modules/bootstrap-runtime.md)
- Persistence: [relevant entity documentation](../../database/schema-catalog.md)
- Route group: [System and Static](../groups/system-and-static.md)

## Frontend Contract

The shared client wraps discovery, liveness, and readiness; OpenAPI and static files are not wrapped.

## OpenAPI and Verification

OpenAPI omits its own `/openapi.json` handler and does not represent `/uploads`.

No complete real-router probe and static-file contract suite was found.

## Change Checklist

Review route registration, middleware zone, DTO and Serde behavior, role and ownership checks, tenant/RLS use, success and error statuses, frontend method/type/callers, OpenAPI, persistence and side effects, and representative positive and negative tests.
