---
okf_document_id: "api-group-marketplace-analytics"
title: "Marketplace Analytics Route Group"
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
  - "backend/src/routes/marketplace_analytics.rs"
related_documents:
  - "api/route-group-catalog.md"
  - "api/endpoint-catalog.md"
  - "backend/modules/marketplace-feedback-analytics-readiness.md"
related_diagrams:
  - "api/diagrams/api-route-map.mmd"
uncertainty_markers:
  - "OPENAPI_IMPLEMENTATION_CONFLICT OIC-01"
---

# Marketplace Analytics Route Group

[Back to the Route Group Catalog](../route-group-catalog.md)

## Boundary

This group contains 2 registered handler-method endpoints from `backend/src/routes/marketplace_analytics.rs`. Its access zone is **Tenant protected**, and its principal path surface is `/api/marketplace/analytics`.

Creator and administrator views have different authorization and aggregation scopes. Values are derived views rather than independent write models.

## Endpoint Families

[Marketplace Feedback And Analytics](../endpoints/marketplace-feedback-and-analytics.md) provide the task-oriented contracts. The exhaustive registered method/path list is in the [Endpoint Catalog](../endpoint-catalog.md).

## Ownership and Data

- Backend: [Marketplace Feedback Analytics Readiness](../../backend/modules/marketplace-feedback-analytics-readiness.md)
- Persistence: [relevant database documentation](../../database/entities/marketplace-reviews-and-abuse.md)
- Route registration: `backend/src/routes/marketplace_analytics.rs`

## Frontend Coverage

Both creator and administrative analytics handlers have frontend wrappers.

## OpenAPI and Tests

Both handlers are included, without machine-readable role requirements.

Analytics services have selected tests; a route-level role and aggregation contract suite was not found.

## Change Checklist

Verify router placement, request and response DTOs, role and ownership checks, tenant/RLS behavior, frontend methods and types, OpenAPI annotations, status/error behavior, persistence effects, side effects, and representative tests before changing this group.
