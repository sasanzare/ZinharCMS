---
okf_document_id: "api-endpoints-marketplace-installation-lifecycle"
title: "Marketplace Installation Lifecycle Endpoints"
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
  - "backend/modules/marketplace-catalog-installation.md"
  - "database/entities/marketplace-installations-and-runtime-adapters.md"
uncertainty_markers:
  - "AUTHORIZATION_REQUIREMENT_UNCLEAR AZU-01"
---

# Marketplace Installation Lifecycle Endpoints

[Back to the Endpoint Catalog](../endpoint-catalog.md) · [Owning route group](../groups/marketplace-core.md)

## Family Boundary

This family contains 8 registered handler-method endpoints. Access is **Tenant protected; install and lifecycle mutation require organization owner/admin capability**.

Installation list/install, update check, enable, disable, uninstall, update, and rollback.

Exact method/path, stable endpoint ID, handler, access zone, input extractor, return type, OpenAPI status, and frontend coverage are recorded in the [Endpoint Catalog](../endpoint-catalog.md).

## Request Contract

Install uses JSON selection and permission approval. Lifecycle actions use installation UUIDs and action-specific payload/confirmation rules.

## Response Contract

Installation objects/arrays, update availability, and branch-specific creation status for installation.

## Ownership and Persistence

- Backend owner: [Marketplace Catalog Installation](../../backend/modules/marketplace-catalog-installation.md)
- Persistence: [relevant entity documentation](../../database/entities/marketplace-installations-and-runtime-adapters.md)
- Route group: [Marketplace Core](../groups/marketplace-core.md)

## Frontend Contract

All eight operations have wrappers under `api.marketplace`.

## OpenAPI and Verification

All handlers are included, but permission approval and lifecycle prerequisites are not machine-readable security contracts.

Marketplace service tests cover compatibility, artifact, permissions, and lifecycle rules; route-level side effects and role failures remain incomplete.

## Change Checklist

Review route registration, middleware zone, DTO and Serde behavior, role and ownership checks, tenant/RLS use, success and error statuses, frontend method/type/callers, OpenAPI, persistence and side effects, and representative positive and negative tests.
