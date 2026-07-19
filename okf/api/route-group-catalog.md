---
okf_document_id: "api-route-group-catalog"
title: "API Route Group Catalog"
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
  - "backend/src/routes/mod.rs"
  - "backend/src/routes"
related_documents:
  - "api/README.md"
  - "api/endpoint-catalog.md"
  - "backend/module-catalog.md"
related_diagrams:
  - "api/diagrams/api-route-map.mmd"
uncertainty_markers: []
---

# API Route Group Catalog

## Selection Rule

A significant route group corresponds to one route-registration module or one coherent subset of the root router. Counts include registered handler-method pairs, not route declaration statements. Static `/uploads` is documented with the system group but excluded from handler counts.

| Group | Handler endpoints | Access zone | Primary prefix or surface | Document |
| --- | ---: | --- | --- | --- |
| System and static | 4 | Public | `/`, probes, OpenAPI, `/uploads` | [System and Static](groups/system-and-static.md) |
| Authentication | 6 | Public / authenticated | `/api/auth` | [Authentication](groups/authentication.md) |
| Beta release | 9 | Authenticated / tenant | `/api/beta` | [Beta Release](groups/beta-release.md) |
| Billing and quota | 8 | Public / tenant | `/api/billing` | [Billing and Quota](groups/billing-and-quota.md) |
| Editorial comments | 6 | Tenant | `/api/comments` | [Editorial Comments](groups/editorial-comments.md) |
| Content and workflow | 16 | Tenant | `/api/content-types`, `/api/entries` | [Content and Workflow](groups/content-and-workflow.md) |
| Public delivery | 8 | Public | `/api/v1` | [Public Delivery](groups/public-delivery.md) |
| Marketplace core | 31 | Tenant | `/api/marketplace` | [Marketplace Core](groups/marketplace-core.md) |
| Marketplace adapters | 5 | Tenant | `/api/marketplace` | [Marketplace Adapters](groups/marketplace-adapters.md) |
| Marketplace analytics | 2 | Tenant | `/api/marketplace/analytics` | [Marketplace Analytics](groups/marketplace-analytics.md) |
| Marketplace finance | 8 | Tenant | `/api/marketplace` | [Marketplace Finance](groups/marketplace-finance.md) |
| Marketplace runtime | 6 | Tenant | `/api/marketplace` | [Marketplace Runtime](groups/marketplace-runtime.md) |
| Media | 5 | Tenant | `/api/media` | [Media](groups/media.md) |
| Organizations and SaaS | 22 | Authenticated / tenant | `/api/organizations` | [Organizations and SaaS](groups/organizations-and-saas.md) |
| Pages, components, and preview | 20 | Tenant | `/api/pages`, `/api/component-registry`, `/api/preview` | [Pages, Components, and Preview](groups/pages-components-and-preview.md) |
| Built-in plugins | 5 | Authenticated | `/api/plugins` | [Built-In Plugins](groups/built-in-plugins.md) |
| CMS webhooks | 7 | Tenant | `/api/webhooks` | [CMS Webhooks](groups/cms-webhooks.md) |

Total: 168 registered handler-method endpoints across 17 groups.

## Navigation

Each group document links back here, identifies its registration source, access zone, endpoint families, backend owner, persistence documentation, frontend coverage, OpenAPI status, tests, and group-specific uncertainties. The exhaustive method/path inventory is in the [Endpoint Catalog](endpoint-catalog.md).
