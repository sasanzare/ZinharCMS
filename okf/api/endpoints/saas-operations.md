---
okf_document_id: "api-endpoints-saas-operations"
title: "SaaS Operations Endpoints"
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
  - "api/groups/organizations-and-saas.md"
  - "backend/modules/organizations.md"
  - "database/entities/saas-operations-and-audit.md"
uncertainty_markers:
  - "AUTHORIZATION_REQUIREMENT_UNCLEAR AZU-01"
---

# SaaS Operations Endpoints

[Back to the Endpoint Catalog](../endpoint-catalog.md) · [Owning route group](../groups/organizations-and-saas.md)

## Family Boundary

This family contains 9 registered handler-method endpoints. Access is **Tenant protected, primarily organization administrators**.

Workspace access, domain list/create/delete, rate-limit get/update, audit logs, email deliveries, and alert rules.

Exact method/path, stable endpoint ID, handler, access zone, input extractor, return type, OpenAPI status, and frontend coverage are recorded in the [Endpoint Catalog](../endpoint-catalog.md).

## Request Contract

Domain and rate-limit changes use JSON; domain IDs use UUID paths; audit and email lists accept bounded `limit`.

## Response Contract

Workspace, domain, rate-limit, audit, delivery, and alert objects or arrays.

## Ownership and Persistence

- Backend owner: [Organizations](../../backend/modules/organizations.md)
- Persistence: [relevant entity documentation](../../database/entities/saas-operations-and-audit.md)
- Route group: [Organizations and SaaS](../groups/organizations-and-saas.md)

## Frontend Contract

All nine operations have wrappers under `api.organizations`.

## OpenAPI and Verification

All are included without bearer, tenant, or role security declarations.

No comprehensive HTTP suite demonstrates administrator-only access and cross-tenant isolation.

## Change Checklist

Review route registration, middleware zone, DTO and Serde behavior, role and ownership checks, tenant/RLS use, success and error statuses, frontend method/type/callers, OpenAPI, persistence and side effects, and representative positive and negative tests.
