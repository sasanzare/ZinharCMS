---
okf_document_id: "api-group-marketplace-adapters"
title: "Marketplace Adapters Route Group"
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
  - "backend/src/routes/marketplace_adapters.rs"
related_documents:
  - "api/route-group-catalog.md"
  - "api/endpoint-catalog.md"
  - "backend/modules/marketplace-runtime-adapters.md"
related_diagrams:
  - "api/diagrams/api-route-map.mmd"
uncertainty_markers:
  - "OPENAPI_IMPLEMENTATION_CONFLICT OIC-01"
---

# Marketplace Adapters Route Group

[Back to the Route Group Catalog](../route-group-catalog.md)

## Boundary

This group contains 5 registered handler-method endpoints from `backend/src/routes/marketplace_adapters.rs`. Its access zone is **Tenant protected**, and its principal path surface is Marketplace runtime components, hooks, importer, delivery, and editor adapters.

Adapters expose installed-product contributions to CMS host surfaces. Results depend on installation state, permissions, compatibility, and kill switches.

## Endpoint Families

[Marketplace Host Adapters](../endpoints/marketplace-host-adapters.md) provide the task-oriented contracts. The exhaustive registered method/path list is in the [Endpoint Catalog](../endpoint-catalog.md).

## Ownership and Data

- Backend: [Marketplace Runtime Adapters](../../backend/modules/marketplace-runtime-adapters.md)
- Persistence: [relevant database documentation](../../database/entities/marketplace-installations-and-runtime-adapters.md)
- Route registration: `backend/src/routes/marketplace_adapters.rs`

## Frontend Coverage

All five handlers have frontend wrappers.

## OpenAPI and Tests

All five handlers are included; runtime permissions and tenant requirements are not declared as security contracts.

One route-local adapter test and Marketplace adapter/service tests exist; host integration contracts are not fully exercised through HTTP.

## Change Checklist

Verify router placement, request and response DTOs, role and ownership checks, tenant/RLS behavior, frontend methods and types, OpenAPI annotations, status/error behavior, persistence effects, side effects, and representative tests before changing this group.
