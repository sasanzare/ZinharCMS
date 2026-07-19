---
okf_document_id: "api-group-billing-quota"
title: "Billing and Quota Route Group"
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
  - "backend/src/routes/billing.rs"
related_documents:
  - "api/route-group-catalog.md"
  - "api/endpoint-catalog.md"
  - "backend/modules/billing-quotas.md"
related_diagrams:
  - "api/diagrams/api-route-map.mmd"
uncertainty_markers:
  - "OPENAPI_IMPLEMENTATION_CONFLICT OIC-01"
---

# Billing and Quota Route Group

[Back to the Route Group Catalog](../route-group-catalog.md)

## Boundary

This group contains 8 registered handler-method endpoints from `backend/src/routes/billing.rs`. Its access zone is **One public provider callback; seven tenant endpoints**, and its principal path surface is `/api/billing`.

The registered inbound webhook is `/api/billing/stripe/webhook`. Billing paths bypass the tenant middleware's general quota rejection so tenants can recover through billing operations.

## Endpoint Families

[Billing Subscription And Usage](../endpoints/billing-subscription-and-usage.md) provide the task-oriented contracts. The exhaustive registered method/path list is in the [Endpoint Catalog](../endpoint-catalog.md).

## Ownership and Data

- Backend: [Billing Quotas](../../backend/modules/billing-quotas.md)
- Persistence: [relevant database documentation](../../database/entities/plans-subscriptions-and-usage.md)
- Route registration: `backend/src/routes/billing.rs`

## Frontend Coverage

All seven tenant billing handlers have frontend wrappers. Stripe calls the webhook directly.

## OpenAPI and Tests

All eight handlers are included, but signature, bearer, tenant-header, and role requirements are not fully represented.

A route-local billing test and service tests exist; provider callback and tenant-role behavior lack a full HTTP suite.

## Change Checklist

Verify router placement, request and response DTOs, role and ownership checks, tenant/RLS behavior, frontend methods and types, OpenAPI annotations, status/error behavior, persistence effects, side effects, and representative tests before changing this group.
