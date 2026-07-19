---
okf_document_id: "api-group-built-in-plugins"
title: "Built-In Plugins Route Group"
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
  - "backend/src/routes/plugins.rs"
related_documents:
  - "api/route-group-catalog.md"
  - "api/endpoint-catalog.md"
  - "backend/modules/built-in-plugins.md"
related_diagrams:
  - "api/diagrams/api-route-map.mmd"
uncertainty_markers:
  - "OPENAPI_IMPLEMENTATION_CONFLICT OIC-01"
---

# Built-In Plugins Route Group

[Back to the Route Group Catalog](../route-group-catalog.md)

## Boundary

This group contains 5 registered handler-method endpoints from `backend/src/routes/plugins.rs`. Its access zone is **Authenticated, not tenant middleware**, and its principal path surface is `/api/plugins`.

These are repository built-in plugin registry operations, distinct from Marketplace-installed product runtime. Verify handler-level role checks because the router supplies identity but no tenant context.

## Endpoint Families

[Built In Plugins](../endpoints/built-in-plugins.md) provide the task-oriented contracts. The exhaustive registered method/path list is in the [Endpoint Catalog](../endpoint-catalog.md).

## Ownership and Data

- Backend: [Built In Plugins](../../backend/modules/built-in-plugins.md)
- Persistence: [relevant database documentation](../../database/entities/component-and-plugin-registry.md)
- Route registration: `backend/src/routes/plugins.rs`

## Frontend Coverage

Four handlers have wrappers; plugin detail GET does not.

## OpenAPI and Tests

All five handlers are included, but bearer requirements are not declared.

Plugin services have selected tests; a complete authenticated route and permission matrix was not found.

## Change Checklist

Verify router placement, request and response DTOs, role and ownership checks, tenant/RLS behavior, frontend methods and types, OpenAPI annotations, status/error behavior, persistence effects, side effects, and representative tests before changing this group.
