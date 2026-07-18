---
okf_document_id: "backend-module-catalog"
title: "Backend Module Catalog"
project: "ZinharCMS"
category: "backend"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
primary_sources:
  - "backend/src/routes"
  - "backend/src/services"
  - "backend/src/middleware"
  - "backend/src/models"
  - "backend/src/plugins"
  - "backend/src/main.rs"
  - "backend/src/state.rs"
related_documents:
  - "backend/README.md"
  - "backend/module-boundaries.md"
  - "backend/dependency-map.md"
  - "backend/testing-map.md"
  - "architecture/components.md"
related_diagrams:
  - "backend/diagrams/backend-module-map.mmd"
  - "backend/diagrams/backend-dependency-flow.mmd"
uncertainty_markers:
  - "MODULE_BOUNDARY_UNCLEAR"
  - "MODULE_OWNERSHIP_UNCLEAR"
  - "RESPONSIBILITY_OVERLAP"
  - "PUBLIC_INTERFACE_UNCLEAR"
---

# Backend Module Catalog

## Selection Rule

Phase 3 identifies 18 significant documentation modules. It consolidates smaller utilities into shared infrastructure and groups fine-grained capabilities when they share the same route, service, persistence, and consumer boundary. This is intentionally not one document per Rust file.

## Summary

| ID | Module | Type | Primary path | Primary responsibility | Implementation | Boundary | Confidence |
|---|---|---|---|---|---|---|---|
| BE-MOD-001 | [Bootstrap and Runtime](modules/bootstrap-runtime.md) | Application module | `backend/src/main.rs` | Process startup and composition | `IMPLEMENTED` | `OBSERVED` | High |
| BE-MOD-002 | [Authentication](modules/authentication.md) | Domain and HTTP module | `backend/src/routes/auth.rs` | Identity and token lifecycle | `IMPLEMENTED` | `OBSERVED` | High |
| BE-MOD-003 | [Tenant Authorization and RLS](modules/tenant-authorization.md) | Shared domain and persistence module | `backend/src/middleware/tenant.rs` | Tenant context, roles, limits, RLS | `IMPLEMENTED` | `OVERLAPPING` | High |
| BE-MOD-004 | [Organizations and SaaS Operations](modules/organizations.md) | Domain and HTTP module | `backend/src/routes/organizations.rs` | Organization control plane | `IMPLEMENTED` | `OVERLAPPING` | High |
| BE-MOD-005 | [Billing and Quotas](modules/billing-quotas.md) | Domain and integration module | `backend/src/routes/billing.rs` | Plans, usage, Stripe, quotas | `IMPLEMENTED` | `OVERLAPPING` | High |
| BE-MOD-006 | [Content Types, Entries, and Workflow](modules/content-workflow.md) | Domain and HTTP module | `backend/src/routes/content.rs` | Schemas, entries, workflow and side effects | `IMPLEMENTED` | `OVERLAPPING` | High |
| BE-MOD-007 | [Editorial Comments](modules/comments.md) | Domain and HTTP module | `backend/src/routes/comments.rs` | Comments on entries/pages | `IMPLEMENTED` | `OBSERVED` | High |
| BE-MOD-008 | [Media](modules/media.md) | Domain, HTTP, and storage module | `backend/src/routes/media.rs` | Media metadata and files | `IMPLEMENTED` | `OVERLAPPING` | High |
| BE-MOD-009 | [Pages, Builder, and Preview](modules/pages-builder-preview.md) | Domain and HTTP module | `backend/src/routes/pages.rs` | Pages, versions, components, preview | `IMPLEMENTED` | `OVERLAPPING` | High |
| BE-MOD-010 | [Public Delivery and Cache](modules/public-delivery-cache.md) | HTTP and infrastructure module | `backend/src/routes/delivery.rs` | Published delivery and cache | `IMPLEMENTED` | `OVERLAPPING` | High |
| BE-MOD-011 | [CMS Webhooks](modules/cms-webhooks.md) | Integration and HTTP module | `backend/src/services/webhooks.rs` | Subscription and outbound delivery | `IMPLEMENTED` | `OBSERVED` | High |
| BE-MOD-012 | [Built-In Plugins](modules/built-in-plugins.md) | Plugin module | `backend/src/plugins/mod.rs` | Trusted in-process hooks | `IMPLEMENTED` | `EXPLICIT` | High |
| BE-MOD-013 | [Beta and Release Operations](modules/beta-release-operations.md) | Application and HTTP module | `backend/src/routes/beta.rs` | Beta records and readiness evidence | `IMPLEMENTED` | `OVERLAPPING` | High |
| BE-MOD-014 | [Marketplace Creator, Submission, Validation, and Review](modules/marketplace-creator-review.md) | Domain and HTTP module | `backend/src/routes/marketplace.rs` | Creator/listing/package/review pipeline | `IMPLEMENTED` | `OVERLAPPING` | High |
| BE-MOD-015 | [Marketplace Catalog and Installation](modules/marketplace-catalog-installation.md) | Domain and HTTP module | `backend/src/services/marketplace_installation.rs` | Catalog and install lifecycle | `IMPLEMENTED` | `OVERLAPPING` | High |
| BE-MOD-016 | [Marketplace Runtime Security and Host Adapters](modules/marketplace-runtime-adapters.md) | Domain policy and adapter module | `backend/src/routes/marketplace_runtime.rs` | Permissions, kill switches, host adapters | `IMPLEMENTED` | `OVERLAPPING` | High |
| BE-MOD-017 | [Marketplace Finance](modules/marketplace-finance.md) | Domain and integration module | `backend/src/routes/marketplace_finance.rs` | Purchases, entitlements, ledger, payouts | `IMPLEMENTED` | `OVERLAPPING` | High |
| BE-MOD-018 | [Marketplace Feedback, Analytics, and Readiness](modules/marketplace-feedback-analytics-readiness.md) | Domain and application module | `backend/src/routes/marketplace_analytics.rs` | Reviews, abuse, analytics, readiness | `IMPLEMENTED` | `OVERLAPPING` | High |

## Detailed Entries

### BE-MOD-001 — Bootstrap and Runtime

- Source path: `backend/src/main.rs` and the additional verified paths in the [individual module document](modules/bootstrap-runtime.md).
- Module type: Application module
- Primary responsibility: Process startup and composition.
- Secondary responsibilities: cross-cutting orchestration is recorded in the individual document and boundary matrix.
- Entry points and public interfaces: main; app; router.
- Main dependencies and consumers: All route modules and infrastructure.
- Data concepts: System/config/state.
- Routes or handlers: see the registered router/handler paths in the individual document; Phase 3 does not enumerate endpoint contracts.
- Tests: No separate process suite.
- Related documentation: [module document](modules/bootstrap-runtime.md), [boundaries](module-boundaries.md), [dependencies](dependency-map.md), and [testing map](testing-map.md).
- Implementation status: `IMPLEMENTED`
- Boundary status: `OBSERVED`
- Confidence: High

### BE-MOD-002 — Authentication

- Source path: `backend/src/routes/auth.rs` and the additional verified paths in the [individual module document](modules/authentication.md).
- Module type: Domain and HTTP module
- Primary responsibility: Identity and token lifecycle.
- Secondary responsibilities: cross-cutting orchestration is recorded in the individual document and boundary matrix.
- Entry points and public interfaces: public/protected routers; Claims.
- Main dependencies and consumers: Tenancy and protected modules.
- Data concepts: Users/roles/tokens.
- Routes or handlers: see the registered router/handler paths in the individual document; Phase 3 does not enumerate endpoint contracts.
- Tests: Security tests; route gap.
- Related documentation: [module document](modules/authentication.md), [boundaries](module-boundaries.md), [dependencies](dependency-map.md), and [testing map](testing-map.md).
- Implementation status: `IMPLEMENTED`
- Boundary status: `OBSERVED`
- Confidence: High

### BE-MOD-003 — Tenant Authorization and RLS

- Source path: `backend/src/middleware/tenant.rs` and the additional verified paths in the [individual module document](modules/tenant-authorization.md).
- Module type: Shared domain and persistence module
- Primary responsibility: Tenant context, roles, limits, RLS.
- Secondary responsibilities: cross-cutting orchestration is recorded in the individual document and boundary matrix.
- Entry points and public interfaces: tenant middleware; service helpers.
- Main dependencies and consumers: All tenant modules.
- Data concepts: Membership/limits/RLS.
- Routes or handlers: see the registered router/handler paths in the individual document; Phase 3 does not enumerate endpoint contracts.
- Tests: RBAC/quota/rate tests.
- Related documentation: [module document](modules/tenant-authorization.md), [boundaries](module-boundaries.md), [dependencies](dependency-map.md), and [testing map](testing-map.md).
- Implementation status: `IMPLEMENTED`
- Boundary status: `OVERLAPPING`
- Confidence: High

### BE-MOD-004 — Organizations and SaaS Operations

- Source path: `backend/src/routes/organizations.rs` and the additional verified paths in the [individual module document](modules/organizations.md).
- Module type: Domain and HTTP module
- Primary responsibility: Organization control plane.
- Secondary responsibilities: cross-cutting orchestration is recorded in the individual document and boundary matrix.
- Entry points and public interfaces: protected/tenant routers.
- Main dependencies and consumers: Auth, billing, beta, tenants.
- Data concepts: Organizations/members/domains/audit.
- Routes or handlers: see the registered router/handler paths in the individual document; Phase 3 does not enumerate endpoint contracts.
- Tests: Email tests; route gap.
- Related documentation: [module document](modules/organizations.md), [boundaries](module-boundaries.md), [dependencies](dependency-map.md), and [testing map](testing-map.md).
- Implementation status: `IMPLEMENTED`
- Boundary status: `OVERLAPPING`
- Confidence: High

### BE-MOD-005 — Billing and Quotas

- Source path: `backend/src/routes/billing.rs` and the additional verified paths in the [individual module document](modules/billing-quotas.md).
- Module type: Domain and integration module
- Primary responsibility: Plans, usage, Stripe, quotas.
- Secondary responsibilities: cross-cutting orchestration is recorded in the individual document and boundary matrix.
- Entry points and public interfaces: billing routers; quota/Stripe services.
- Main dependencies and consumers: Tenant/features/Marketplace finance.
- Data concepts: Plans/subscriptions/usage/events.
- Routes or handlers: see the registered router/handler paths in the individual document; Phase 3 does not enumerate endpoint contracts.
- Tests: Billing/quota/Stripe tests.
- Related documentation: [module document](modules/billing-quotas.md), [boundaries](module-boundaries.md), [dependencies](dependency-map.md), and [testing map](testing-map.md).
- Implementation status: `IMPLEMENTED`
- Boundary status: `OVERLAPPING`
- Confidence: High

### BE-MOD-006 — Content Types, Entries, and Workflow

- Source path: `backend/src/routes/content.rs` and the additional verified paths in the [individual module document](modules/content-workflow.md).
- Module type: Domain and HTTP module
- Primary responsibility: Schemas, entries, workflow and side effects.
- Secondary responsibilities: cross-cutting orchestration is recorded in the individual document and boundary matrix.
- Entry points and public interfaces: content router; validation/workflow helpers.
- Main dependencies and consumers: Delivery, webhooks, plugins, comments.
- Data concepts: Content types/entries/status.
- Routes or handlers: see the registered router/handler paths in the individual document; Phase 3 does not enumerate endpoint contracts.
- Tests: Validation/workflow tests.
- Related documentation: [module document](modules/content-workflow.md), [boundaries](module-boundaries.md), [dependencies](dependency-map.md), and [testing map](testing-map.md).
- Implementation status: `IMPLEMENTED`
- Boundary status: `OVERLAPPING`
- Confidence: High

### BE-MOD-007 — Editorial Comments

- Source path: `backend/src/routes/comments.rs` and the additional verified paths in the [individual module document](modules/comments.md).
- Module type: Domain and HTTP module
- Primary responsibility: Comments on entries/pages.
- Secondary responsibilities: cross-cutting orchestration is recorded in the individual document and boundary matrix.
- Entry points and public interfaces: comments router and DTOs.
- Main dependencies and consumers: Content/pages/editorial UI.
- Data concepts: Comments/entity references.
- Routes or handlers: see the registered router/handler paths in the individual document; Phase 3 does not enumerate endpoint contracts.
- Tests: No colocated tests found.
- Related documentation: [module document](modules/comments.md), [boundaries](module-boundaries.md), [dependencies](dependency-map.md), and [testing map](testing-map.md).
- Implementation status: `IMPLEMENTED`
- Boundary status: `OBSERVED`
- Confidence: High

### BE-MOD-008 — Media

- Source path: `backend/src/routes/media.rs` and the additional verified paths in the [individual module document](modules/media.md).
- Module type: Domain, HTTP, and storage module
- Primary responsibility: Media metadata and files.
- Secondary responsibilities: cross-cutting orchestration is recorded in the individual document and boundary matrix.
- Entry points and public interfaces: media router; processing helpers.
- Main dependencies and consumers: Pages/Marketplace/public files.
- Data concepts: Media/variants/files.
- Routes or handlers: see the registered router/handler paths in the individual document; Phase 3 does not enumerate endpoint contracts.
- Tests: No complete compensation test.
- Related documentation: [module document](modules/media.md), [boundaries](module-boundaries.md), [dependencies](dependency-map.md), and [testing map](testing-map.md).
- Implementation status: `IMPLEMENTED`
- Boundary status: `OVERLAPPING`
- Confidence: High

### BE-MOD-009 — Pages, Builder, and Preview

- Source path: `backend/src/routes/pages.rs` and the additional verified paths in the [individual module document](modules/pages-builder-preview.md).
- Module type: Domain and HTTP module
- Primary responsibility: Pages, versions, components, preview.
- Secondary responsibilities: cross-cutting orchestration is recorded in the individual document and boundary matrix.
- Entry points and public interfaces: pages router; validation; WebSocket.
- Main dependencies and consumers: Delivery/comments/Marketplace.
- Data concepts: Pages/versions/components/channels.
- Routes or handlers: see the registered router/handler paths in the individual document; Phase 3 does not enumerate endpoint contracts.
- Tests: Workflow and frontend tests.
- Related documentation: [module document](modules/pages-builder-preview.md), [boundaries](module-boundaries.md), [dependencies](dependency-map.md), and [testing map](testing-map.md).
- Implementation status: `IMPLEMENTED`
- Boundary status: `OVERLAPPING`
- Confidence: High

### BE-MOD-010 — Public Delivery and Cache

- Source path: `backend/src/routes/delivery.rs` and the additional verified paths in the [individual module document](modules/public-delivery-cache.md).
- Module type: HTTP and infrastructure module
- Primary responsibility: Published delivery and cache.
- Secondary responsibilities: cross-cutting orchestration is recorded in the individual document and boundary matrix.
- Entry points and public interfaces: delivery router; invalidation functions.
- Main dependencies and consumers: Public clients/content/pages.
- Data concepts: Published projections/cache.
- Routes or handlers: see the registered router/handler paths in the individual document; Phase 3 does not enumerate endpoint contracts.
- Tests: Three route tests.
- Related documentation: [module document](modules/public-delivery-cache.md), [boundaries](module-boundaries.md), [dependencies](dependency-map.md), and [testing map](testing-map.md).
- Implementation status: `IMPLEMENTED`
- Boundary status: `OVERLAPPING`
- Confidence: High

### BE-MOD-011 — CMS Webhooks

- Source path: `backend/src/services/webhooks.rs` and the additional verified paths in the [individual module document](modules/cms-webhooks.md).
- Module type: Integration and HTTP module
- Primary responsibility: Subscription and outbound delivery.
- Secondary responsibilities: cross-cutting orchestration is recorded in the individual document and boundary matrix.
- Entry points and public interfaces: webhook router/service.
- Main dependencies and consumers: Content/pages/external targets.
- Data concepts: Subscriptions/deliveries.
- Routes or handlers: see the registered router/handler paths in the individual document; Phase 3 does not enumerate endpoint contracts.
- Tests: Three service tests.
- Related documentation: [module document](modules/cms-webhooks.md), [boundaries](module-boundaries.md), [dependencies](dependency-map.md), and [testing map](testing-map.md).
- Implementation status: `IMPLEMENTED`
- Boundary status: `OBSERVED`
- Confidence: High

### BE-MOD-012 — Built-In Plugins

- Source path: `backend/src/plugins/mod.rs` and the additional verified paths in the [individual module document](modules/built-in-plugins.md).
- Module type: Plugin module
- Primary responsibility: Trusted in-process hooks.
- Secondary responsibilities: cross-cutting orchestration is recorded in the individual document and boundary matrix.
- Entry points and public interfaces: CmsPlugin; plugin router.
- Main dependencies and consumers: Content/admin UI.
- Data concepts: Plugin registry/context.
- Routes or handlers: see the registered router/handler paths in the individual document; Phase 3 does not enumerate endpoint contracts.
- Tests: SEO tests.
- Related documentation: [module document](modules/built-in-plugins.md), [boundaries](module-boundaries.md), [dependencies](dependency-map.md), and [testing map](testing-map.md).
- Implementation status: `IMPLEMENTED`
- Boundary status: `EXPLICIT`
- Confidence: High

### BE-MOD-013 — Beta and Release Operations

- Source path: `backend/src/routes/beta.rs` and the additional verified paths in the [individual module document](modules/beta-release-operations.md).
- Module type: Application and HTTP module
- Primary responsibility: Beta records and readiness evidence.
- Secondary responsibilities: cross-cutting orchestration is recorded in the individual document and boundary matrix.
- Entry points and public interfaces: beta routers; contract tests.
- Main dependencies and consumers: Organizations/operators/CI.
- Data concepts: Feedback/blockers/readiness.
- Routes or handlers: see the registered router/handler paths in the individual document; Phase 3 does not enumerate endpoint contracts.
- Tests: Route and contract tests.
- Related documentation: [module document](modules/beta-release-operations.md), [boundaries](module-boundaries.md), [dependencies](dependency-map.md), and [testing map](testing-map.md).
- Implementation status: `IMPLEMENTED`
- Boundary status: `OVERLAPPING`
- Confidence: High

### BE-MOD-014 — Marketplace Creator, Submission, Validation, and Review

- Source path: `backend/src/routes/marketplace.rs` and the additional verified paths in the [individual module document](modules/marketplace-creator-review.md).
- Module type: Domain and HTTP module
- Primary responsibility: Creator/listing/package/review pipeline.
- Secondary responsibilities: cross-cutting orchestration is recorded in the individual document and boundary matrix.
- Entry points and public interfaces: Marketplace handlers/services.
- Main dependencies and consumers: Catalog/admin/CLI.
- Data concepts: Creators/listings/versions/reports.
- Routes or handlers: see the registered router/handler paths in the individual document; Phase 3 does not enumerate endpoint contracts.
- Tests: Extensive service tests.
- Related documentation: [module document](modules/marketplace-creator-review.md), [boundaries](module-boundaries.md), [dependencies](dependency-map.md), and [testing map](testing-map.md).
- Implementation status: `IMPLEMENTED`
- Boundary status: `OVERLAPPING`
- Confidence: High

### BE-MOD-015 — Marketplace Catalog and Installation

- Source path: `backend/src/services/marketplace_installation.rs` and the additional verified paths in the [individual module document](modules/marketplace-catalog-installation.md).
- Module type: Domain and HTTP module
- Primary responsibility: Catalog and install lifecycle.
- Secondary responsibilities: cross-cutting orchestration is recorded in the individual document and boundary matrix.
- Entry points and public interfaces: Catalog/install handlers/services.
- Main dependencies and consumers: Runtime/finance/UI.
- Data concepts: Catalog projections/installations.
- Routes or handlers: see the registered router/handler paths in the individual document; Phase 3 does not enumerate endpoint contracts.
- Tests: Catalog/performance/install tests.
- Related documentation: [module document](modules/marketplace-catalog-installation.md), [boundaries](module-boundaries.md), [dependencies](dependency-map.md), and [testing map](testing-map.md).
- Implementation status: `IMPLEMENTED`
- Boundary status: `OVERLAPPING`
- Confidence: High

### BE-MOD-016 — Marketplace Runtime Security and Host Adapters

- Source path: `backend/src/routes/marketplace_runtime.rs` and the additional verified paths in the [individual module document](modules/marketplace-runtime-adapters.md).
- Module type: Domain policy and adapter module
- Primary responsibility: Permissions, kill switches, host adapters.
- Secondary responsibilities: cross-cutting orchestration is recorded in the individual document and boundary matrix.
- Entry points and public interfaces: Runtime/adapter routers/services.
- Main dependencies and consumers: Pages/media/content/installations.
- Data concepts: Permissions/switches/imports/hooks.
- Routes or handlers: see the registered router/handler paths in the individual document; Phase 3 does not enumerate endpoint contracts.
- Tests: Runtime/adapter tests.
- Related documentation: [module document](modules/marketplace-runtime-adapters.md), [boundaries](module-boundaries.md), [dependencies](dependency-map.md), and [testing map](testing-map.md).
- Implementation status: `IMPLEMENTED`
- Boundary status: `OVERLAPPING`
- Confidence: High

### BE-MOD-017 — Marketplace Finance

- Source path: `backend/src/routes/marketplace_finance.rs` and the additional verified paths in the [individual module document](modules/marketplace-finance.md).
- Module type: Domain and integration module
- Primary responsibility: Purchases, entitlements, ledger, payouts.
- Secondary responsibilities: cross-cutting orchestration is recorded in the individual document and boundary matrix.
- Entry points and public interfaces: Finance router/service/Stripe.
- Main dependencies and consumers: Billing/installations/analytics.
- Data concepts: Finance and payout records.
- Routes or handlers: see the registered router/handler paths in the individual document; Phase 3 does not enumerate endpoint contracts.
- Tests: Finance/Stripe tests.
- Related documentation: [module document](modules/marketplace-finance.md), [boundaries](module-boundaries.md), [dependencies](dependency-map.md), and [testing map](testing-map.md).
- Implementation status: `IMPLEMENTED`
- Boundary status: `OVERLAPPING`
- Confidence: High

### BE-MOD-018 — Marketplace Feedback, Analytics, and Readiness

- Source path: `backend/src/routes/marketplace_analytics.rs` and the additional verified paths in the [individual module document](modules/marketplace-feedback-analytics-readiness.md).
- Module type: Domain and application module
- Primary responsibility: Reviews, abuse, analytics, readiness.
- Secondary responsibilities: cross-cutting orchestration is recorded in the individual document and boundary matrix.
- Entry points and public interfaces: Feedback/analytics routes/services/tests.
- Main dependencies and consumers: Customers/creators/admins/CI.
- Data concepts: Reviews/reports/projections.
- Routes or handlers: see the registered router/handler paths in the individual document; Phase 3 does not enumerate endpoint contracts.
- Tests: Feedback/analytics/readiness tests.
- Related documentation: [module document](modules/marketplace-feedback-analytics-readiness.md), [boundaries](module-boundaries.md), [dependencies](dependency-map.md), and [testing map](testing-map.md).
- Implementation status: `IMPLEMENTED`
- Boundary status: `OVERLAPPING`
- Confidence: High


## Phase 5 Persistence Cross-Reference

| Backend module family | Primary database entity documents |
| --- | --- |
| Authentication and organizations | [Identity and Global RBAC](../database/entities/identity-and-global-rbac.md); [Organizations and Membership](../database/entities/organizations-and-membership.md) |
| Content, workflow, pages, plugins, media, delivery | [Content Types and Entries](../database/entities/content-types-and-entries.md); [Pages and Versions](../database/entities/pages-and-versions.md); [Component and Plugin Registry](../database/entities/component-and-plugin-registry.md); [Media and Variants](../database/entities/media-and-variants.md); [Editorial Comments](../database/entities/editorial-comments.md); [Public Settings and Navigation](../database/entities/public-settings-and-navigation.md); [CMS Webhooks and Deliveries](../database/entities/cms-webhooks-and-deliveries.md) |
| Billing, SaaS, and beta | [Plans, Subscriptions, and Usage](../database/entities/plans-subscriptions-and-usage.md); [SaaS Operations and Audit](../database/entities/saas-operations-and-audit.md); [Beta Release Records](../database/entities/beta-release-records.md) |
| Marketplace | [Marketplace entity catalog](../database/entity-catalog.md#catalog) covering creator, catalog/review, runtime, commerce, ledger/payout, and trust aggregates |

## Catalog Maintenance

Add a module only when it has a distinct domain/feature responsibility, independent router/service logic, meaningful interface, owned persistence behavior, dedicated tests, or clear architectural significance. Small cross-cutting helpers belong in [Shared Infrastructure](shared-infrastructure.md). Update the module document, boundaries, dependency map, testing map, diagrams, and index together.
