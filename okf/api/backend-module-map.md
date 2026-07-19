---
okf_document_id: "api-backend-module-map"
title: "API Backend Module Map"
project: "ZinharCMS"
category: "api"
phase: 6
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "eed1e0dbdf6d873457d1165158b3c8fbfd6647e1"
last_verified_date: "2026-07-18"
primary_sources:
  - "backend/src/routes"
  - "backend/src/services"
  - "okf/backend/module-catalog.md"
related_documents:
  - "api/route-group-catalog.md"
  - "api/endpoint-catalog.md"
  - "backend/module-catalog.md"
  - "database/entity-catalog.md"
uncertainty_markers:
  - "MODULE_OWNERSHIP_UNCLEAR MOU-01"
---

# API Backend Module Map

## Route-to-Backend Ownership

| API group | Registration source | Backend owner document | Primary persistence document |
| --- | --- | --- | --- |
| System and static | `routes/mod.rs` | [Bootstrap and Runtime](../backend/modules/bootstrap-runtime.md) | [Schema Catalog](../database/schema-catalog.md) |
| Authentication | `routes/auth.rs` | [Authentication](../backend/modules/authentication.md) | [Identity and Global RBAC](../database/entities/identity-and-global-rbac.md) |
| Beta release | `routes/beta.rs` | [Beta and Release Operations](../backend/modules/beta-release-operations.md) | [Beta Release Records](../database/entities/beta-release-records.md) |
| Billing and quota | `routes/billing.rs` | [Billing and Quotas](../backend/modules/billing-quotas.md) | [Plans, Subscriptions, and Usage](../database/entities/plans-subscriptions-and-usage.md) |
| Comments | `routes/comments.rs` | [Comments](../backend/modules/comments.md) | [Editorial Comments](../database/entities/editorial-comments.md) |
| Content | `routes/content.rs` | [Content and Workflow](../backend/modules/content-workflow.md) | [Content Types and Entries](../database/entities/content-types-and-entries.md) |
| Delivery | `routes/delivery.rs` | [Public Delivery and Cache](../backend/modules/public-delivery-cache.md) | [Public Settings and Navigation](../database/entities/public-settings-and-navigation.md) |
| Marketplace creator/review | `routes/marketplace.rs` | [Marketplace Creator and Review](../backend/modules/marketplace-creator-review.md) | [Catalog and Review Pipeline](../database/entities/marketplace-catalog-and-review-pipeline.md) |
| Marketplace install/catalog | `routes/marketplace.rs` | [Marketplace Catalog and Installation](../backend/modules/marketplace-catalog-installation.md) | [Installations and Runtime Adapters](../database/entities/marketplace-installations-and-runtime-adapters.md) |
| Marketplace adapters/runtime | `routes/marketplace_adapters.rs`, `routes/marketplace_runtime.rs` | [Marketplace Runtime and Adapters](../backend/modules/marketplace-runtime-adapters.md) | [Installations and Runtime Adapters](../database/entities/marketplace-installations-and-runtime-adapters.md) |
| Marketplace finance | `routes/marketplace_finance.rs` | [Marketplace Finance](../backend/modules/marketplace-finance.md) | [Ledger and Payouts](../database/entities/marketplace-ledger-and-payouts.md) |
| Marketplace feedback/analytics | `routes/marketplace.rs`, `routes/marketplace_analytics.rs` | [Marketplace Feedback and Analytics](../backend/modules/marketplace-feedback-analytics-readiness.md) | [Reviews and Abuse](../database/entities/marketplace-reviews-and-abuse.md) |
| Media | `routes/media.rs` | [Media](../backend/modules/media.md) | [Media and Variants](../database/entities/media-and-variants.md) |
| Organizations and SaaS | `routes/organizations.rs` | [Organizations](../backend/modules/organizations.md) | [Organizations and Membership](../database/entities/organizations-and-membership.md) |
| Pages/components/preview | `routes/pages.rs` | [Pages, Builder, and Preview](../backend/modules/pages-builder-preview.md) | [Pages and Versions](../database/entities/pages-and-versions.md) |
| Built-in plugins | `routes/plugins.rs` | [Built-In Plugins](../backend/modules/built-in-plugins.md) | [Component and Plugin Registry](../database/entities/component-and-plugin-registry.md) |
| CMS webhooks | `routes/webhooks.rs` | [CMS Webhooks](../backend/modules/cms-webhooks.md) | [CMS Webhooks and Deliveries](../database/entities/cms-webhooks-and-deliveries.md) |

## Dependency Pattern

Route modules own transport DTOs and orchestration. Shared services own authentication primitives, RBAC, RLS, quota, audit, caching, email, webhook delivery, Stripe integration, and Marketplace validation/runtime behavior. SQL is split between routes and services, so a route contract change may cross both layers.

`MODULE_OWNERSHIP_UNCLEAR MOU-01` applies when a contract spans multiple Marketplace route/service modules without an obvious change owner. The endpoint-family documents provide the task-oriented view for those cross-module capabilities.
