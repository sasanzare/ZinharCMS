---
okf_document_id: "api-group-cms-webhooks"
title: "CMS Webhooks Route Group"
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
  - "backend/src/routes/webhooks.rs"
related_documents:
  - "api/route-group-catalog.md"
  - "api/endpoint-catalog.md"
  - "backend/modules/cms-webhooks.md"
related_diagrams:
  - "api/diagrams/api-route-map.mmd"
uncertainty_markers:
  - "OPENAPI_IMPLEMENTATION_CONFLICT OIC-01"
---

# CMS Webhooks Route Group

[Back to the Route Group Catalog](../route-group-catalog.md)

## Boundary

This group contains 7 registered handler-method endpoints from `backend/src/routes/webhooks.rs`. Its access zone is **Tenant protected**, and its principal path surface is `/api/webhooks`.

This is the CMS outbound webhook management surface and is separate from the public inbound Stripe webhook.

## Endpoint Families

[Cms Webhooks](../endpoints/cms-webhooks.md) provide the task-oriented contracts. The exhaustive registered method/path list is in the [Endpoint Catalog](../endpoint-catalog.md).

## Ownership and Data

- Backend: [Cms Webhooks](../../backend/modules/cms-webhooks.md)
- Persistence: [relevant database documentation](../../database/entities/cms-webhooks-and-deliveries.md)
- Route registration: `backend/src/routes/webhooks.rs`

## Frontend Coverage

Six handlers have wrappers; webhook detail GET does not.

## OpenAPI and Tests

All seven handlers are included, without bearer, tenant, role, delivery-signature, or retry semantics as security contracts.

Webhook service logic has selected tests; no full CRUD-to-outbound-delivery HTTP contract suite was found.

## Change Checklist

Verify router placement, request and response DTOs, role and ownership checks, tenant/RLS behavior, frontend methods and types, OpenAPI annotations, status/error behavior, persistence effects, side effects, and representative tests before changing this group.
