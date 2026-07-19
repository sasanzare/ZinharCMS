---
okf_document_id: "api-endpoints-cms-webhooks"
title: "CMS Webhook Endpoints"
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
  - "api/groups/cms-webhooks.md"
  - "backend/modules/cms-webhooks.md"
  - "database/entities/cms-webhooks-and-deliveries.md"
uncertainty_markers:
  - "AUTHORIZATION_REQUIREMENT_UNCLEAR AZU-01"
---

# CMS Webhook Endpoints

[Back to the Endpoint Catalog](../endpoint-catalog.md) · [Owning route group](../groups/cms-webhooks.md)

## Family Boundary

This family contains 7 registered handler-method endpoints. Access is **Tenant protected; webhook management capability required**.

Webhook list/create/detail/update/delete, bounded delivery history, and test delivery.

Exact method/path, stable endpoint ID, handler, access zone, input extractor, return type, OpenAPI status, and frontend coverage are recorded in the [Endpoint Catalog](../endpoint-catalog.md).

## Request Contract

Webhook JSON includes URL/events and optional active state. UUID paths identify webhooks; delete requires confirmation; deliveries accept `limit`.

## Response Contract

Webhook and delivery objects/arrays; test returns delivery outcome.

## Ownership and Persistence

- Backend owner: [Cms Webhooks](../../backend/modules/cms-webhooks.md)
- Persistence: [relevant entity documentation](../../database/entities/cms-webhooks-and-deliveries.md)
- Route group: [CMS Webhooks](../groups/cms-webhooks.md)

## Frontend Contract

Six operations have wrappers; webhook detail GET does not.

## OpenAPI and Verification

All handlers are included, but outbound signature and retry semantics are not complete contract definitions.

Selected service tests exist; CRUD-to-outbound HTTP delivery and retry behavior lacks full coverage.

## Change Checklist

Review route registration, middleware zone, DTO and Serde behavior, role and ownership checks, tenant/RLS use, success and error statuses, frontend method/type/callers, OpenAPI, persistence and side effects, and representative positive and negative tests.
