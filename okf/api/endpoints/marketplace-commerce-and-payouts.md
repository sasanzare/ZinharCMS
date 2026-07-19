---
okf_document_id: "api-endpoints-marketplace-commerce-payouts"
title: "Marketplace Commerce and Payout Endpoints"
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
  - "api/groups/marketplace-finance.md"
  - "backend/modules/marketplace-finance.md"
  - "database/entities/marketplace-ledger-and-payouts.md"
uncertainty_markers:
  - "RESPONSE_CONTRACT_UNCLEAR RSCU-01"
  - "AUTHORIZATION_REQUIREMENT_UNCLEAR AZU-01"
---

# Marketplace Commerce and Payout Endpoints

[Back to the Endpoint Catalog](../endpoint-catalog.md) · [Owning route group](../groups/marketplace-finance.md)

## Family Boundary

This family contains 8 registered handler-method endpoints. Access is **Tenant protected with purchaser, creator-owner, billing, and global-admin rules by operation**.

Purchase list/checkout, revenue ledger, payout account get/onboard/verify, creator balance, and payout request.

Exact method/path, stable endpoint ID, handler, access zone, input extractor, return type, OpenAPI status, and frontend coverage are recorded in the [Endpoint Catalog](../endpoint-catalog.md).

## Request Contract

Checkout and payout actions use JSON; creator UUID paths select payout accounts. Provider state and idempotency participate in validation.

## Response Contract

Purchases, checkout result, ledger entries, payout account, balance, and payout request. Checkout status can be 200 or 201; payout request is 201.

## Ownership and Persistence

- Backend owner: [Marketplace Finance](../../backend/modules/marketplace-finance.md)
- Persistence: [relevant entity documentation](../../database/entities/marketplace-ledger-and-payouts.md)
- Route group: [Marketplace Finance](../groups/marketplace-finance.md)

## Frontend Contract

Six handlers have wrappers; revenue ledger and payout verification do not.

## OpenAPI and Verification

All handlers are included but provider prerequisites, role matrix, and branch-specific behavior are incomplete.

Finance service coverage exists; provider-backed HTTP/idempotency/authorization contracts remain incomplete.

## Change Checklist

Review route registration, middleware zone, DTO and Serde behavior, role and ownership checks, tenant/RLS use, success and error statuses, frontend method/type/callers, OpenAPI, persistence and side effects, and representative positive and negative tests.
