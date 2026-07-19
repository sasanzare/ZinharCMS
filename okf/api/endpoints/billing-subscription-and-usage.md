---
okf_document_id: "api-endpoints-billing-subscription-usage"
title: "Billing, Subscription, and Usage Endpoints"
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
  - "api/groups/billing-and-quota.md"
  - "backend/modules/billing-quotas.md"
  - "database/entities/plans-subscriptions-and-usage.md"
uncertainty_markers:
  - "DOCUMENTATION_CODE_CONFLICT DCC-09"
  - "AUTHORIZATION_REQUIREMENT_UNCLEAR AZU-01"
---

# Billing, Subscription, and Usage Endpoints

[Back to the Endpoint Catalog](../endpoint-catalog.md) · [Owning route group](../groups/billing-and-quota.md)

## Family Boundary

This family contains 8 registered handler-method endpoints. Access is **Seven tenant endpoints; one public signed Stripe webhook**.

Plan list, subscription get/change, checkout, customer portal, usage get/rebuild, and inbound Stripe event processing.

Exact method/path, stable endpoint ID, handler, access zone, input extractor, return type, OpenAPI status, and frontend coverage are recorded in the [Endpoint Catalog](../endpoint-catalog.md).

## Request Contract

Plan selection uses JSON. The webhook uses raw bytes and Stripe signature headers. Tenant changes require billing-manager capability.

## Response Contract

Plan arrays, subscription/usage objects, provider session URLs, and webhook processing result.

## Ownership and Persistence

- Backend owner: [Billing Quotas](../../backend/modules/billing-quotas.md)
- Persistence: [relevant entity documentation](../../database/entities/plans-subscriptions-and-usage.md)
- Route group: [Billing and Quota](../groups/billing-and-quota.md)

## Frontend Contract

Seven tenant operations are wrapped under `api.billing`; Stripe invokes the webhook.

## OpenAPI and Verification

All are included, but callback signature and security/tenant requirements are incomplete.

Provider and billing services have selected tests; signed callback/idempotency and role behavior need real HTTP coverage.

## Change Checklist

Review route registration, middleware zone, DTO and Serde behavior, role and ownership checks, tenant/RLS use, success and error statuses, frontend method/type/callers, OpenAPI, persistence and side effects, and representative positive and negative tests.
