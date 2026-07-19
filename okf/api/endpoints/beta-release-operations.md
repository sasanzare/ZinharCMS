---
okf_document_id: "api-endpoints-beta-release"
title: "Beta Release Operations Endpoints"
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
  - "api/groups/beta-release.md"
  - "backend/modules/beta-release-operations.md"
  - "database/entities/beta-release-records.md"
uncertainty_markers:
  - "AUTHORIZATION_REQUIREMENT_UNCLEAR AZU-01"
---

# Beta Release Operations Endpoints

[Back to the Endpoint Catalog](../endpoint-catalog.md) · [Owning route group](../groups/beta-release.md)

## Family Boundary

This family contains 9 registered handler-method endpoints. Access is **Seven tenant endpoints; two authenticated global-administration endpoints**.

Tenant beta dashboard, feedback list/create/update, GA-blocker list/create/update, product dashboard, and beta participant upsert.

Exact method/path, stable endpoint ID, handler, access zone, input extractor, return type, OpenAPI status, and frontend coverage are recorded in the [Endpoint Catalog](../endpoint-catalog.md).

## Request Contract

Feedback/blocker JSON DTOs use validated category, severity, priority, and status values. Lists accept bounded `limit`.

## Response Contract

Dashboard aggregates, feedback/blocker objects or arrays, product dashboard, and participant state.

## Ownership and Persistence

- Backend owner: [Beta Release Operations](../../backend/modules/beta-release-operations.md)
- Persistence: [relevant entity documentation](../../database/entities/beta-release-records.md)
- Route group: [Beta Release](../groups/beta-release.md)

## Frontend Contract

All nine operations have wrappers under `api.beta`.

## OpenAPI and Verification

All are included; the split between global roles and tenant roles is omitted.

Four route-local tests cover selected normalization and dashboards, not the full authorization matrix.

## Change Checklist

Review route registration, middleware zone, DTO and Serde behavior, role and ownership checks, tenant/RLS use, success and error statuses, frontend method/type/callers, OpenAPI, persistence and side effects, and representative positive and negative tests.
