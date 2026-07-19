---
okf_document_id: "api-endpoints-organizations-membership"
title: "Organizations, Membership, and Invitations Endpoints"
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
  - "database/entities/organizations-and-membership.md"
uncertainty_markers:
  - "AUTHORIZATION_REQUIREMENT_UNCLEAR AZU-01"
  - "TENANT_CONTEXT_UNCLEAR TCU-01"
---

# Organizations, Membership, and Invitations Endpoints

[Back to the Endpoint Catalog](../endpoint-catalog.md) · [Owning route group](../groups/organizations-and-saas.md)

## Family Boundary

This family contains 13 registered handler-method endpoints. Access is **Three authenticated bootstrap endpoints; ten tenant-protected membership endpoints**.

Organization list/create/current detail/update, member list/update/removal, invitation list/create/revocation/acceptance, leave, and ownership transfer.

Exact method/path, stable endpoint ID, handler, access zone, input extractor, return type, OpenAPI status, and frontend coverage are recorded in the [Endpoint Catalog](../endpoint-catalog.md).

## Request Contract

JSON DTOs carry organization/member/invitation changes. UUID path parameters identify users and invitations. Tenant operations require `X-Organization-Id`.

## Response Contract

Organization, membership, member, invitation, and action result objects or arrays.

## Ownership and Persistence

- Backend owner: [Organizations](../../backend/modules/organizations.md)
- Persistence: [relevant entity documentation](../../database/entities/organizations-and-membership.md)
- Route group: [Organizations and SaaS](../groups/organizations-and-saas.md)

## Frontend Contract

All 13 operations have wrappers under `api.organizations`.

## OpenAPI and Verification

Handlers are included, but router-zone and organization-role requirements are not expressed.

Selected service/UI tests exist; negative membership, invitation, and ownership-transfer HTTP matrices are missing.

## Change Checklist

Review route registration, middleware zone, DTO and Serde behavior, role and ownership checks, tenant/RLS use, success and error statuses, frontend method/type/callers, OpenAPI, persistence and side effects, and representative positive and negative tests.
