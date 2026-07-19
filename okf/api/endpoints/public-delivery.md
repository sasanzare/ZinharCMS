---
okf_document_id: "api-endpoints-public-delivery"
title: "Public Delivery Endpoints"
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
  - "api/groups/public-delivery.md"
  - "backend/modules/public-delivery-cache.md"
  - "database/entities/public-settings-and-navigation.md"
uncertainty_markers:
  - "OPENAPI_IMPLEMENTATION_CONFLICT OIC-01"
  - "VERSIONING_BEHAVIOR_UNCLEAR VBU-01"
---

# Public Delivery Endpoints

[Back to the Endpoint Catalog](../endpoint-catalog.md) · [Owning route group](../groups/public-delivery.md)

## Family Boundary

This family contains 8 registered handler-method endpoints. Access is **Public; default public organization resolved internally**.

Published content list/detail, published page list/detail, public settings, navigation, sitemap XML, and robots text.

Exact method/path, stable endpoint ID, handler, access zone, input extractor, return type, OpenAPI status, and frontend coverage are recorded in the [Endpoint Catalog](../endpoint-catalog.md).

## Request Contract

Lists support page/per-page/sort/locale/author/filter; details accept slug or UUID-or-slug identifiers where defined.

## Response Contract

Public JSON DTOs and page wrappers, XML sitemap, and text robots response.

## Ownership and Persistence

- Backend owner: [Public Delivery Cache](../../backend/modules/public-delivery-cache.md)
- Persistence: [relevant entity documentation](../../database/entities/public-settings-and-navigation.md)
- Route group: [Public Delivery](../groups/public-delivery.md)

## Frontend Contract

No endpoint is wrapped by the administration shared API client.

## OpenAPI and Verification

Sitemap and robots are missing; six JSON handlers are included.

Three route-local parser/escaping tests exist; cache and full representation contracts remain incomplete.

## Change Checklist

Review route registration, middleware zone, DTO and Serde behavior, role and ownership checks, tenant/RLS use, success and error statuses, frontend method/type/callers, OpenAPI, persistence and side effects, and representative positive and negative tests.
