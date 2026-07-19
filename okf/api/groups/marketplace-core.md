---
okf_document_id: "api-group-marketplace-core"
title: "Marketplace Core Route Group"
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
  - "backend/src/routes/marketplace.rs"
related_documents:
  - "api/route-group-catalog.md"
  - "api/endpoint-catalog.md"
  - "backend/modules/marketplace-creator-review.md"
related_diagrams:
  - "api/diagrams/api-route-map.mmd"
uncertainty_markers:
  - "OPENAPI_IMPLEMENTATION_CONFLICT OIC-01"
---

# Marketplace Core Route Group

[Back to the Route Group Catalog](../route-group-catalog.md)

## Boundary

This group contains 31 registered handler-method endpoints from `backend/src/routes/marketplace.rs`. Its access zone is **Tenant protected**, and its principal path surface is `/api/marketplace` creator, catalog, listing, review, installation, feedback, and purchase paths.

This file spans several business aggregates and role domains. Use endpoint families and backend module documents rather than treating the source file as one bounded domain.

## Endpoint Families

[Marketplace Creator Catalog And Review](../endpoints/marketplace-creator-catalog-and-review.md), [Marketplace Installation Lifecycle](../endpoints/marketplace-installation-lifecycle.md), [Marketplace Commerce And Payouts](../endpoints/marketplace-commerce-and-payouts.md), [Marketplace Feedback And Analytics](../endpoints/marketplace-feedback-and-analytics.md) provide the task-oriented contracts. The exhaustive registered method/path list is in the [Endpoint Catalog](../endpoint-catalog.md).

## Ownership and Data

- Backend: [Marketplace Creator Review](../../backend/modules/marketplace-creator-review.md)
- Persistence: [relevant database documentation](../../database/entities/marketplace-catalog-and-review-pipeline.md)
- Route registration: `backend/src/routes/marketplace.rs`

## Frontend Coverage

Most core handlers have frontend wrappers; creator verification is intentionally backend-only in the shared client map.

## OpenAPI and Tests

Sixteen creator/catalog/review handlers are registered but absent from OpenAPI; the remaining core handlers are included.

Three route-local tests and extensive Marketplace service tests exist; full route/auth/transaction contracts remain incomplete.

## Change Checklist

Verify router placement, request and response DTOs, role and ownership checks, tenant/RLS behavior, frontend methods and types, OpenAPI annotations, status/error behavior, persistence effects, side effects, and representative tests before changing this group.
