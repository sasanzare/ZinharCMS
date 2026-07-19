---
okf_document_id: "api-endpoints-marketplace-host-adapters"
title: "Marketplace Host Adapter Endpoints"
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
  - "api/groups/marketplace-adapters.md"
  - "backend/modules/marketplace-runtime-adapters.md"
  - "database/entities/marketplace-installations-and-runtime-adapters.md"
uncertainty_markers:
  - "AUTHORIZATION_REQUIREMENT_UNCLEAR AZU-01"
---

# Marketplace Host Adapter Endpoints

[Back to the Endpoint Catalog](../endpoint-catalog.md) · [Owning route group](../groups/marketplace-adapters.md)

## Family Boundary

This family contains 5 registered handler-method endpoints. Access is **Tenant protected**.

Runtime components, template preview/import, hook list, and hook authorization.

Exact method/path, stable endpoint ID, handler, access zone, input extractor, return type, OpenAPI status, and frontend coverage are recorded in the [Endpoint Catalog](../endpoint-catalog.md).

## Request Contract

Template operations use installation UUID paths and JSON. Hook authorization uses hook type plus request context.

## Response Contract

Component/hook arrays, preview/import results, and authorization decisions.

## Ownership and Persistence

- Backend owner: [Marketplace Runtime Adapters](../../backend/modules/marketplace-runtime-adapters.md)
- Persistence: [relevant entity documentation](../../database/entities/marketplace-installations-and-runtime-adapters.md)
- Route group: [Marketplace Adapters](../groups/marketplace-adapters.md)

## Frontend Contract

All five operations have wrappers under `api.marketplaceAdapters`.

## OpenAPI and Verification

All handlers are included; host/runtime prerequisites are not fully represented.

One route-local adapter test plus service tests exist; host integration and tenant isolation are not fully exercised through HTTP.

## Change Checklist

Review route registration, middleware zone, DTO and Serde behavior, role and ownership checks, tenant/RLS use, success and error statuses, frontend method/type/callers, OpenAPI, persistence and side effects, and representative positive and negative tests.
