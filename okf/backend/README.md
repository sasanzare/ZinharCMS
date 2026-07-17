---
okf_document_id: "backend-readme"
title: "Backend Documentation"
project: "ZinharCMS"
category: "backend"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
primary_sources:
  - "backend/Cargo.toml"
  - "backend/src"
  - "backend/migrations"
related_documents:
  - "README.md"
  - "backend/overview.md"
  - "backend/module-catalog.md"
  - "backend/module-boundaries.md"
  - "backend/dependency-map.md"
  - "backend/request-handling.md"
  - "backend/services-and-domain.md"
  - "backend/persistence-access.md"
  - "backend/configuration-and-state.md"
  - "backend/shared-infrastructure.md"
  - "backend/error-handling.md"
  - "backend/testing-map.md"
  - "backend/backend-risks.md"
  - "architecture/overview.md"
  - "architecture/components.md"
  - "architecture/boundaries.md"
  - "architecture/dependency-model.md"
related_diagrams:
  - "backend/diagrams/backend-module-map.mmd"
  - "backend/diagrams/backend-request-lifecycle.mmd"
  - "backend/diagrams/backend-dependency-flow.mmd"
  - "backend/diagrams/application-state-composition.mmd"
uncertainty_markers:
  - "MODULE_BOUNDARY_UNCLEAR"
  - "MODULE_OWNERSHIP_UNCLEAR"
  - "DEPENDENCY_DIRECTION_UNCLEAR"
  - "RESPONSIBILITY_OVERLAP"
  - "BUSINESS_RULE_UNVERIFIED"
---

# Backend Documentation

## Purpose and Scope

This directory is the Phase 3 navigation and structural knowledge layer for the current Rust/Axum backend. It documents significant capability modules, responsibilities, interfaces, dependencies, request structure, logic placement, persistence access, configuration/state, shared infrastructure, errors, tests, and risks.

Current source code, manifests, tests, migrations, initialization, and route registration remain authoritative. Module documents summarize evidence; they do not replace Rust modules or create enforceable ownership. This is not an endpoint reference, a complete database reference, detailed security documentation, or detailed business-rule documentation.

## Reading Order

1. [Backend Overview](overview.md)
2. [Module Catalog](module-catalog.md)
3. The relevant individual module document below
4. [Module Boundaries](module-boundaries.md)
5. [Dependency Map](dependency-map.md)
6. [Request Handling](request-handling.md)
7. [Services and Domain Logic](services-and-domain.md)
8. [Persistence Access](persistence-access.md)
9. [Configuration and State](configuration-and-state.md)
10. [Shared Infrastructure](shared-infrastructure.md)
11. [Error Handling](error-handling.md)
12. [Testing Map](testing-map.md)
13. [Backend Risks](backend-risks.md)

## Primary Documents

- [Backend Overview](overview.md)
- [Module Catalog](module-catalog.md)
- [Module Boundaries](module-boundaries.md)
- [Dependency Map](dependency-map.md)
- [Request Handling](request-handling.md)
- [Services and Domain Logic](services-and-domain.md)
- [Persistence Access](persistence-access.md)
- [Configuration and State](configuration-and-state.md)
- [Shared Infrastructure](shared-infrastructure.md)
- [Error Handling](error-handling.md)
- [Testing Map](testing-map.md)
- [Backend Risks](backend-risks.md)

## Significant Modules

- [BE-MOD-001 — Bootstrap and Runtime](modules/bootstrap-runtime.md)
- [BE-MOD-002 — Authentication](modules/authentication.md)
- [BE-MOD-003 — Tenant Authorization and RLS](modules/tenant-authorization.md)
- [BE-MOD-004 — Organizations and SaaS Operations](modules/organizations.md)
- [BE-MOD-005 — Billing and Quotas](modules/billing-quotas.md)
- [BE-MOD-006 — Content Types, Entries, and Workflow](modules/content-workflow.md)
- [BE-MOD-007 — Editorial Comments](modules/comments.md)
- [BE-MOD-008 — Media](modules/media.md)
- [BE-MOD-009 — Pages, Builder, and Preview](modules/pages-builder-preview.md)
- [BE-MOD-010 — Public Delivery and Cache](modules/public-delivery-cache.md)
- [BE-MOD-011 — CMS Webhooks](modules/cms-webhooks.md)
- [BE-MOD-012 — Built-In Plugins](modules/built-in-plugins.md)
- [BE-MOD-013 — Beta and Release Operations](modules/beta-release-operations.md)
- [BE-MOD-014 — Marketplace Creator, Submission, Validation, and Review](modules/marketplace-creator-review.md)
- [BE-MOD-015 — Marketplace Catalog and Installation](modules/marketplace-catalog-installation.md)
- [BE-MOD-016 — Marketplace Runtime Security and Host Adapters](modules/marketplace-runtime-adapters.md)
- [BE-MOD-017 — Marketplace Finance](modules/marketplace-finance.md)
- [BE-MOD-018 — Marketplace Feedback, Analytics, and Readiness](modules/marketplace-feedback-analytics-readiness.md)

## Diagrams

- [Backend Module Map](diagrams/backend-module-map.mmd)
- [Backend Request Lifecycle](diagrams/backend-request-lifecycle.mmd)
- [Backend Dependency Flow](diagrams/backend-dependency-flow.mmd)
- [Application State Composition](diagrams/application-state-composition.mmd)

## For AI Coding Agents

1. Read [Module Catalog](module-catalog.md).
2. Locate the target module document.
3. Read [Module Boundaries](module-boundaries.md).
4. Check [Dependency Map](dependency-map.md).
5. Inspect the relevant source code and tests.
6. Verify API behavior against route registration.
7. Verify persistence behavior against models, queries, and migrations.
8. Avoid introducing cross-module dependencies without evidence.
9. Update related backend OKF documents when changing responsibilities or interfaces.
10. Never invent business rules or security guarantees.

## Related Architecture

- [Architecture Overview](../architecture/overview.md)
- [Architecture Components](../architecture/components.md)
- [Architecture Boundaries](../architecture/boundaries.md)
- [Architecture Dependency Model](../architecture/dependency-model.md)
- [Architecture Runtime Flows](../architecture/runtime-flows.md)

## Deferred Topics

Phase 5 owns detailed schemas; Phase 6 owns endpoint contracts; Phase 7 owns detailed authentication/authorization/security; Phase 8 owns business rules and tenancy; Phase 9 owns complete plugin/Marketplace extensibility; Phase 10 owns operations; Phase 4 owns frontend architecture. No later phase is started here.
