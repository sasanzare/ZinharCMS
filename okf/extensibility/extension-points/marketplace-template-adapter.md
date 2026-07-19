---
okf_document_id: "extension-point-marketplace-template-adapter"
title: "Marketplace Template Adapter Extension Point"
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
  - "backend/src/services/marketplace_adapters.rs"
  - "backend/src/routes/marketplace_adapters.rs"
  - "frontend/src/pages/PagesPage.tsx"
  - "backend/migrations/0021_v3_phase_eight_runtime_adapters.sql"
extension_point_id: "EP-004"
extension_point_name: "Marketplace Template Adapter"
extension_point_category: "declarative_page_import"
registration_type: "active_installation_manifest"
implementation_status: "implemented"
related_documents:
  - "../extension-points.md"
  - "../marketplace/runtime-permissions-adapters.md"
  - "../../domain/page-builder-rules.md"
related_diagrams:
  - "../diagrams/component-registration.mmd"
  - "../diagrams/marketplace-installation-flow.mmd"
---

# Marketplace Template Adapter Extension Point

## Identity

EP-004 interprets a design-template manifest and creates a host-owned page.

| Identity field | Value |
|---|---|
| Extension-point ID | EP-004 |
| Name | Marketplace Template Adapter |
| Category | Declarative page import |
| Source paths | marketplace_adapters route/service; PagesPage.tsx; migration 0021 |
| Implementation status | IMPLEMENTED |
| Confidence | High |

## Purpose

Preview a declared page template, map package asset references to tenant media, and import the resulting page without executing package code.

## Contract

The manifest template provides a key, page_json object, and asset declarations. Request data can select a template key and supply asset mapping; import additionally requires title and slug.

## Registration

The template is available through an active, runtime-ready, tenant-owned design_template installation.

The creator declares the template in a reviewed manifest; the tenant installs the version; the host discovers it by installation ID. Registration is dynamic data registration, not dynamic code loading. Template keys select definitions; duplicate-key behavior within a manifest is not separately specified, and no cross-installation order is promised.

## Execution

Preview validates and returns transformed JSON without persistence. Import checks page-writer RBAC and quota, validates assets and page JSON, inserts page and version 1, records marketplace_template_imports and audit data, then commits.

The host executes synchronously in the request and shares tenant transaction state. Errors abort preview or roll back import. Side effects are page/version/import/audit records; package code does not run.

## Security

Tenant context/RLS, installation ownership, runtime readiness, media ownership, slug validation, Page Builder validation, and quota are enforced by host code.

## Compatibility

Template JSON must satisfy the current Page Builder validator and registered component keys.

## Tests

Adapter mapping helpers have unit coverage and PagesPage tests cover frontend calls. Real database/RLS transaction and rollback tests were not found.

## Risks and Unknowns

Schema evolution, asset replacement, collision, and imported-page provenance retention need explicit compatibility tests. See [Extension Points](../extension-points.md).
