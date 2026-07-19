---
okf_document_id: "marketplace-architecture"
title: "Marketplace Architecture"
project: "ZinharCMS"
category: "extensibility"
phase: 9
status: "current"
source_of_truth: false
implementation_view: "observed"
extensibility_status: "partially_verified"
last_verified_commit: "56d733985fdd7aa3f25ee6981b88cf29c52f65c9"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/routes/marketplace.rs"
  - "backend/src/routes/marketplace_adapters.rs"
  - "backend/src/routes/marketplace_runtime.rs"
  - "backend/src/routes/marketplace_finance.rs"
  - "backend/src/routes/marketplace_analytics.rs"
  - "backend/src/services"
  - "backend/migrations/0015_v3_phase_one_marketplace_foundation.sql"
  - "frontend/src/pages/MarketplacePage.tsx"
related_documents:
  - "marketplace-workflows.md"
  - "marketplace/creator-listing-version.md"
  - "marketplace/package-validation-review.md"
  - "marketplace/installation-update-rollback.md"
  - "marketplace/runtime-permissions-adapters.md"
  - "marketplace/commerce-entitlements.md"
  - "marketplace/feedback-moderation-analytics.md"
related_diagrams:
  - "diagrams/extensibility-context.mmd"
  - "diagrams/marketplace-installation-flow.mmd"
---

# Marketplace Architecture

Marketplace is a substantial integrated subsystem, not a package execution engine. Static host routes and services own every observed action.

| Area | Verified behavior | Status |
|---|---|---|
| Creator and listing | Profiles, listing metadata, uploads, submissions | IMPLEMENTED |
| Validation and review | Manifest/package checks, reports, decisions, moderation | IMPLEMENTED |
| Catalog | Eligible approved projection and detail | IMPLEMENTED |
| Installation | Tenant install, enable/disable, update/rollback, uninstall | IMPLEMENTED with product-type gate |
| Runtime policy | Permissions, safe entry points, payload cap, kill switches | IMPLEMENTED authorization only |
| Host adapters | Components, templates, public-hook authorization | PARTIALLY_IMPLEMENTED |
| Commerce | Checkout/purchases, entitlement checks, ledger/payouts | IMPLEMENTED contracts; provider/deployment state unverified |
| Feedback and analytics | Reviews, abuse reports, moderation, creator/admin analytics | IMPLEMENTED |
| Package execution | Arbitrary uploaded code execution | MARKETPLACE_NOT_IMPLEMENTED |

Global listing/version data feeds tenant-owned installations. Tenant middleware, RBAC, RLS, permission snapshots, artifact integrity checks, and kill switches form the control chain. Frontend pages consume fixed API endpoints.

See [Marketplace Workflows](marketplace-workflows.md) and begin the dedicated area sequence with [Creator, Listing, and Version Management](marketplace/creator-listing-version.md).

## Marketplace Capability Inventory

| Capability | Backend | Data entities | API/frontend | Tenant availability | Status |
|---|---|---|---|---|---|
| Publishers/creators | Marketplace core/domain/submission | creators | Creator profile UI/API | Creator/global review identity | IMPLEMENTED |
| Listings/categories/search | Catalog/core routes/services | listings and catalog projections | MarketplacePage catalog/search/detail | Global catalog, tenant compatibility | IMPLEMENTED |
| Packages/versions | Upload/package/manifest services | versions and artifacts | Creator upload UI/API | Global reviewed version metadata | IMPLEMENTED |
| Compatibility | Manifest/catalog/validation/install services | manifest/report JSON | Catalog badge and install/update gates | Evaluated for tenant plan/host | IMPLEMENTED |
| Verification/moderation | Review/policy routes/services | submissions, validation reports, review events | Admin queues/actions | Global administrative | IMPLEMENTED |
| Installation/activation/removal | Installation service and core routes | installations | MarketplacePage controls | Tenant | IMPLEMENTED for component_pack/design_template |
| Updates/rollback | Installation service/routes | pinned and rollback version fields | MarketplacePage controls | Tenant | IMPLEMENTED |
| Runtime permissions | Runtime service/routes | permission catalog, snapshots, kill switches | Runtime status/control UI | Tenant plus global overlay | PARTIALLY_IMPLEMENTED; decision only |
| Components/templates/hooks | Host adapters | component registry, imports, hooks | PagesPage and adapter APIs | Tenant | PARTIALLY_IMPLEMENTED |
| Pricing/purchases/entitlements | Finance/core install gates | purchases and entitlements | Checkout/install UI/API | Tenant | IMPLEMENTED contracts; provider state unverified |
| Licensing | Listing metadata | listing fields | Catalog/creator UI | Global metadata | IMPLEMENTED metadata; enforcement unclear |
| Ratings/reviews/abuse | Feedback/core routes | reviews and abuse reports | MarketplacePage | Tenant eligibility/global moderation | IMPLEMENTED |
| Analytics | Analytics routes/services | aggregate queries across Marketplace entities | Creator/admin panels | Creator/global admin | IMPLEMENTED |
| Package retrieval | Stored artifact lookup/verification | version artifact fields | No direct plugin runtime retrieval | Host-only | IMPLEMENTED for integrity check |
| Signatures | No signature chain found | None | None | None | MARKETPLACE_NOT_IMPLEMENTED |
| Arbitrary package execution | No loader/runner found | None | Authorization says not_executed | None | MARKETPLACE_NOT_IMPLEMENTED |

Overall marketplace status is PARTIALLY_IMPLEMENTED: commercial and governance workflows are substantial, while executable plugin runtime and several declarative delivery surfaces remain absent or partial.
