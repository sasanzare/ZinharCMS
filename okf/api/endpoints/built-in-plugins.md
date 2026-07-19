---
okf_document_id: "api-endpoints-built-in-plugins"
title: "Built-In Plugin Endpoints"
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
  - "api/groups/built-in-plugins.md"
  - "backend/modules/built-in-plugins.md"
  - "database/entities/component-and-plugin-registry.md"
uncertainty_markers:
  - "AUTHORIZATION_REQUIREMENT_UNCLEAR AZU-01"
---

# Built-In Plugin Endpoints

[Back to the Endpoint Catalog](../endpoint-catalog.md) · [Owning route group](../groups/built-in-plugins.md)

## Family Boundary

This family contains 5 registered handler-method endpoints. Access is **Bearer authenticated, not tenant middleware; handler role checks apply**.

Plugin list/detail, metadata update, enable, and disable.

Exact method/path, stable endpoint ID, handler, access zone, input extractor, return type, OpenAPI status, and frontend coverage are recorded in the [Endpoint Catalog](../endpoint-catalog.md).

## Request Contract

Plugin key is a string path identifier; update uses JSON; enable/disable are bodyless actions.

## Response Contract

Plugin objects or arrays.

## Ownership and Persistence

- Backend owner: [Built In Plugins](../../backend/modules/built-in-plugins.md)
- Persistence: [relevant entity documentation](../../database/entities/component-and-plugin-registry.md)
- Route group: [Built-In Plugins](../groups/built-in-plugins.md)

## Frontend Contract

Four operations have wrappers; plugin detail GET does not.

## OpenAPI and Verification

All handlers are included, but bearer/role requirements are omitted.

Selected plugin logic tests exist; no complete route-level permission suite was found.

## Change Checklist

Review route registration, middleware zone, DTO and Serde behavior, role and ownership checks, tenant/RLS use, success and error statuses, frontend method/type/callers, OpenAPI, persistence and side effects, and representative positive and negative tests.
