---
okf_document_id: "api-endpoints-marketplace-feedback-analytics"
title: "Marketplace Feedback and Analytics Endpoints"
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
  - "api/groups/marketplace-core.md"
  - "backend/modules/marketplace-feedback-analytics-readiness.md"
  - "database/entities/marketplace-reviews-and-abuse.md"
uncertainty_markers:
  - "AUTHORIZATION_REQUIREMENT_UNCLEAR AZU-01"
---

# Marketplace Feedback and Analytics Endpoints

[Back to the Endpoint Catalog](../endpoint-catalog.md) · [Owning route group](../groups/marketplace-core.md)

## Family Boundary

This family contains 9 registered handler-method endpoints. Access is **Tenant protected with customer, creator, moderator, and global-admin rules by operation**.

Product review list/create/moderation queue/moderation, abuse report create/list/resolve, creator analytics, and administrative analytics.

Exact method/path, stable endpoint ID, handler, access zone, input extractor, return type, OpenAPI status, and frontend coverage are recorded in the [Endpoint Catalog](../endpoint-catalog.md).

## Request Contract

Listing/review/report/creator UUID paths and JSON review/report/moderation payloads define writes; analytics are queryless aggregate reads.

## Response Contract

Review/report objects or arrays and creator/admin analytics aggregates. Review/report creation uses 201.

## Ownership and Persistence

- Backend owner: [Marketplace Feedback Analytics Readiness](../../backend/modules/marketplace-feedback-analytics-readiness.md)
- Persistence: [relevant entity documentation](../../database/entities/marketplace-reviews-and-abuse.md)
- Route group: [Marketplace Core and Analytics](../groups/marketplace-core.md)

## Frontend Contract

All nine operations have wrappers.

## OpenAPI and Verification

All nine are included, though role and ownership policies are not represented.

Marketplace feedback/analytics service tests exist; complete moderation, ownership, and aggregate HTTP contracts are missing.

## Change Checklist

Review route registration, middleware zone, DTO and Serde behavior, role and ownership checks, tenant/RLS use, success and error statuses, frontend method/type/callers, OpenAPI, persistence and side effects, and representative positive and negative tests.
