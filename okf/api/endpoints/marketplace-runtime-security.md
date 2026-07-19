---
okf_document_id: "api-endpoints-marketplace-runtime-security"
title: "Marketplace Runtime Security Endpoints"
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
  - "api/groups/marketplace-runtime.md"
  - "backend/modules/marketplace-runtime-adapters.md"
  - "database/entities/marketplace-installations-and-runtime-adapters.md"
uncertainty_markers:
  - "AUTHORIZATION_REQUIREMENT_UNCLEAR AZU-01"
---

# Marketplace Runtime Security Endpoints

[Back to the Endpoint Catalog](../endpoint-catalog.md) · [Owning route group](../groups/marketplace-runtime.md)

## Family Boundary

This family contains 6 registered handler-method endpoints. Access is **Tenant protected with organization or global administrative capability depending on operation**.

Permission catalog, runtime status, runtime authorization, organization/global kill-switch activation, and kill-switch lift.

Exact method/path, stable endpoint ID, handler, access zone, input extractor, return type, OpenAPI status, and frontend coverage are recorded in the [Endpoint Catalog](../endpoint-catalog.md).

## Request Contract

Runtime authorization and kill-switch operations use JSON and installation/kill-switch UUID paths as defined by handlers.

## Response Contract

Permission catalog, runtime status, authorization decision, and kill-switch objects.

## Ownership and Persistence

- Backend owner: [Marketplace Runtime Adapters](../../backend/modules/marketplace-runtime-adapters.md)
- Persistence: [relevant entity documentation](../../database/entities/marketplace-installations-and-runtime-adapters.md)
- Route group: [Marketplace Runtime](../groups/marketplace-runtime.md)

## Frontend Contract

All six operations have wrappers.

## OpenAPI and Verification

All are included, but role, approved-permission, compatibility, and kill-switch precedence are incomplete.

Selected runtime service and route tests exist; negative authorization and precedence need broader HTTP coverage.

## Change Checklist

Review route registration, middleware zone, DTO and Serde behavior, role and ownership checks, tenant/RLS use, success and error statuses, frontend method/type/callers, OpenAPI, persistence and side effects, and representative positive and negative tests.
