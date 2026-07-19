---
okf_document_id: "extension-point-page-builder-component-registry"
title: "Page Builder Component Registry Extension Point"
project: "ZinharCMS"
category: "extensibility"
phase: 9
status: "current"
source_of_truth: false
implementation_view: "observed"
extensibility_status: "verified"
last_verified_commit: "56d733985fdd7aa3f25ee6981b88cf29c52f65c9"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/routes/pages.rs"
  - "backend/src/routes/marketplace_adapters.rs"
  - "backend/src/services/marketplace_adapters.rs"
  - "frontend/src/pages/PagesPage.tsx"
  - "backend/migrations/0001_initial_schema.sql"
  - "backend/migrations/0021_v3_phase_eight_runtime_adapters.sql"
extension_point_id: "EP-003"
extension_point_name: "Page Builder Component Registry"
extension_point_category: "declarative_component_registry"
registration_type: "database_api_seed_or_marketplace_upsert"
implementation_status: "implemented"
related_documents:
  - "../extension-points.md"
  - "../component-and-block-registration.md"
  - "../../frontend/page-builder.md"
  - "../../domain/page-builder-rules.md"
related_diagrams:
  - "../diagrams/component-registration.mmd"
---

# Page Builder Component Registry Extension Point

## Identity

EP-003 is the component_registry-backed Page Builder catalog.

| Identity field | Value |
|---|---|
| Extension-point ID | EP-003 |
| Name | Page Builder Component Registry |
| Category | Declarative component registry |
| Source paths | routes/pages.rs; marketplace adapters; component migrations; PagesPage.tsx |
| Implementation status | IMPLEMENTED |
| Confidence | High for registry; medium for renderer completeness |

## Purpose

Expose system, organization-authored, and Marketplace-derived component definitions to page validation and the frontend builder.

## Contract

A record has a stable component key, display name, category, JSON props schema, system flag, organization ownership where applicable, and optional Marketplace installation link.

## Registration

System definitions are seeded. Authorized organization managers use component CRUD APIs. Active ready Marketplace component packs are parsed and upserted with a namespaced key.

Registrars are application migrations, tenant component managers, and trusted Marketplace host adapters. Registration is database-driven. component_key is globally unique and must use lowercase alphanumeric hyphen form. Marketplace upsert resolves a matching key; cross-source collision ownership and ordering are COMPONENT_REGISTRATION_UNCLEAR. Lists do not promise a stable registration order.

## Execution

PagesPage combines regular and Marketplace component responses. Page JSON validation resolves allowed keys from system or tenant records. The host renders and persists pages; package JavaScript is not loaded.

Execution is request-driven host behavior under shared AppState and tenant database context. Validation/SQL errors fail the request or import transaction. Side effects can include registry upsert and page/version/import/audit writes.

## Security

Tenant middleware, component-manager RBAC, organization ownership, and RLS protect custom records. System records are globally visible.

## Compatibility

Props schema and keys form a data contract, but no schema-version negotiation or renderer compatibility protocol was found.

## Tests

Page API/frontend tests cover selected registry and Marketplace interactions. Collision, stale cleanup, cross-tenant integration, and every renderer/schema combination remain gaps.

## Risks and Unknowns

COMPONENT_REGISTRATION_UNCLEAR applies to stale Marketplace rows and renderer availability. See [Extension Points](../extension-points.md).
