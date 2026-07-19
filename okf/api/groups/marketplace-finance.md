---
okf_document_id: "api-group-marketplace-finance"
title: "Marketplace Finance Route Group"
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
  - "backend/src/routes/marketplace_finance.rs"
related_documents:
  - "api/route-group-catalog.md"
  - "api/endpoint-catalog.md"
  - "backend/modules/marketplace-finance.md"
related_diagrams:
  - "api/diagrams/api-route-map.mmd"
uncertainty_markers:
  - "OPENAPI_IMPLEMENTATION_CONFLICT OIC-01"
---

# Marketplace Finance Route Group

[Back to the Route Group Catalog](../route-group-catalog.md)

## Boundary

This group contains 8 registered handler-method endpoints from `backend/src/routes/marketplace_finance.rs`. Its access zone is **Tenant protected**, and its principal path surface is Marketplace ledger, payout, refund, and finance-operation paths.

Finance operations combine organization roles, global administration, creator ownership, idempotency, ledger consistency, and Stripe state.

## Endpoint Families

[Marketplace Commerce And Payouts](../endpoints/marketplace-commerce-and-payouts.md) provide the task-oriented contracts. The exhaustive registered method/path list is in the [Endpoint Catalog](../endpoint-catalog.md).

## Ownership and Data

- Backend: [Marketplace Finance](../../backend/modules/marketplace-finance.md)
- Persistence: [relevant database documentation](../../database/entities/marketplace-ledger-and-payouts.md)
- Route registration: `backend/src/routes/marketplace_finance.rs`

## Frontend Coverage

Six handlers have wrappers. Revenue-ledger listing and payout-account verification do not.

## OpenAPI and Tests

All eight handlers are included, without provider/account prerequisites or role security declarations.

Finance service tests cover selected ledger and payout rules; a provider-backed HTTP contract suite was not found.

## Change Checklist

Verify router placement, request and response DTOs, role and ownership checks, tenant/RLS behavior, frontend methods and types, OpenAPI annotations, status/error behavior, persistence effects, side effects, and representative tests before changing this group.
