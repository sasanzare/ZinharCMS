---
okf_document_id: "api-endpoints-marketplace-creator-catalog-review"
title: "Marketplace Creator, Catalog, and Review Endpoints"
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
  - "backend/modules/marketplace-creator-review.md"
  - "database/entities/marketplace-catalog-and-review-pipeline.md"
uncertainty_markers:
  - "OPENAPI_IMPLEMENTATION_CONFLICT OIC-01"
  - "AUTHORIZATION_REQUIREMENT_UNCLEAR AZU-01"
---

# Marketplace Creator, Catalog, and Review Endpoints

[Back to the Endpoint Catalog](../endpoint-catalog.md) · [Owning route group](../groups/marketplace-core.md)

## Family Boundary

This family contains 16 registered handler-method endpoints. Access is **Tenant protected with creator ownership and global review/verification rules**.

Catalog list/detail; creator state/request/verification; creator listing CRUD/submission/version upload/history; review queue/events/reports; submission review and listing moderation.

Exact method/path, stable endpoint ID, handler, access zone, input extractor, return type, OpenAPI status, and frontend coverage are recorded in the [Endpoint Catalog](../endpoint-catalog.md).

## Request Contract

Catalog uses search/category/product/pricing filters. Listing and review actions use JSON, UUID/slug paths, or multipart `file` plus `manifest` for version upload.

## Response Contract

Catalog, creator, listing, submission, validation report, event, and moderation objects or arrays.

## Ownership and Persistence

- Backend owner: [Marketplace Creator Review](../../backend/modules/marketplace-creator-review.md)
- Persistence: [relevant entity documentation](../../database/entities/marketplace-catalog-and-review-pipeline.md)
- Route group: [Marketplace Core](../groups/marketplace-core.md)

## Frontend Contract

All except creator verification have wrappers. Several review operations map directly to dedicated Marketplace UI workflows.

## OpenAPI and Verification

All 16 registered handlers in this family are missing from OpenAPI.

Route and service tests cover selected validation/review rules; full creator/admin/tenant matrix and multipart contract are incomplete.

## Change Checklist

Review route registration, middleware zone, DTO and Serde behavior, role and ownership checks, tenant/RLS use, success and error statuses, frontend method/type/callers, OpenAPI, persistence and side effects, and representative positive and negative tests.
