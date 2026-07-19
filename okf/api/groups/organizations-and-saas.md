---
okf_document_id: "api-group-organizations-saas"
title: "Organizations and SaaS Route Group"
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
  - "backend/src/routes/organizations.rs"
related_documents:
  - "api/route-group-catalog.md"
  - "api/endpoint-catalog.md"
  - "backend/modules/organizations.md"
related_diagrams:
  - "api/diagrams/api-route-map.mmd"
uncertainty_markers:
  - "OPENAPI_IMPLEMENTATION_CONFLICT OIC-01"
---

# Organizations and SaaS Route Group

[Back to the Route Group Catalog](../route-group-catalog.md)

## Boundary

This group contains 22 registered handler-method endpoints from `backend/src/routes/organizations.rs`. Its access zone is **Three authenticated bootstrap endpoints; 19 tenant endpoints**, and its principal path surface is `/api/organizations` and `/api/organization-invitations/accept`.

Organization list/create and invitation acceptance intentionally work without an existing tenant selection. Current-organization operations require tenant context.

## Endpoint Families

[Organizations Membership And Invitations](../endpoints/organizations-membership-and-invitations.md), [Saas Operations](../endpoints/saas-operations.md) provide the task-oriented contracts. The exhaustive registered method/path list is in the [Endpoint Catalog](../endpoint-catalog.md).

## Ownership and Data

- Backend: [Organizations](../../backend/modules/organizations.md)
- Persistence: [relevant database documentation](../../database/entities/organizations-and-membership.md)
- Route registration: `backend/src/routes/organizations.rs`

## Frontend Coverage

All 22 handlers have frontend wrappers.

## OpenAPI and Tests

All 22 handlers are included, without representing the authenticated-versus-tenant split or detailed organization roles.

Organization services and related frontend features have selected tests; complete invitation, ownership-transfer, and tenant-header HTTP matrices were not found.

## Change Checklist

Verify router placement, request and response DTOs, role and ownership checks, tenant/RLS behavior, frontend methods and types, OpenAPI annotations, status/error behavior, persistence effects, side effects, and representative tests before changing this group.
