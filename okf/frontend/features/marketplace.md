---
okf_document_id: "frontend-feature-marketplace"
title: "Marketplace"
project: "ZinharCMS"
category: "frontend-feature"
phase: 4
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "7d25e4cbc53284a78033478e2681d8e9ebeb2fb1"
last_verified_date: "2026-07-17"
feature_id: "FE-FEAT-011"
feature_name: "Marketplace"
feature_paths:
  - "frontend/src/pages/MarketplacePage.tsx"
  - "frontend/src/pages/MarketplacePage.test.tsx"
  - "frontend/src/services/api.ts"
  - "frontend/src/types/api.ts"
boundary_status: "OVERLAPPING"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "frontend/src/pages/MarketplacePage.tsx"
  - "frontend/src/pages/MarketplacePage.test.tsx"
  - "frontend/src/services/api.ts"
  - "frontend/src/types/api.ts"
related_documents:
  - "frontend/feature-catalog.md"
  - "frontend/feature-boundaries.md"
  - "frontend/features/pages-and-page-builder.md"
  - "frontend/testing-map.md"
  - "backend/modules/marketplace-creator-review.md"
  - "backend/modules/marketplace-catalog-installation.md"
  - "backend/modules/marketplace-runtime-adapters.md"
  - "backend/modules/marketplace-finance.md"
  - "backend/modules/marketplace-feedback-analytics-readiness.md"
related_diagrams:
  - "frontend/diagrams/frontend-application-map.mmd"
  - "frontend/diagrams/frontend-api-flow.mmd"
uncertainty_markers:
  - "FEATURE_BOUNDARY_UNCLEAR FBU-01"
  - "RESPONSIBILITY_OVERLAP FRO-01"
  - "AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01"
  - "API_CONTRACT_UNCLEAR ACU-01"
---

# Marketplace

## Feature Identity

| Field | Value |
|---|---|
| Feature ID | `FE-FEAT-011` |
| Application | `FE-APP-001` |
| Implementation | `IMPLEMENTED` |
| Boundary | `OVERLAPPING` |
| Confidence | High |
| Route | `/marketplace` |

## Responsibility

Provides customer catalog discovery and details, permission-aware install/update/rollback/uninstall, paid purchase, reviews and abuse reporting, creator onboarding, listing/package submission, platform review/moderation, runtime permission and kill-switch controls, creator finance, and creator/admin analytics.

## Owned Source Areas

- Route page and local helper/rendering logic: `frontend/src/pages/MarketplacePage.tsx`.
- Selected behavior tests: `frontend/src/pages/MarketplacePage.test.tsx`.
- Marketplace and host-adapter methods: central API client.
- Large set of manual Marketplace contracts: `frontend/src/types/api.ts`.

The frontend groups concerns that Phase 3 maps to five backend modules; `FBU-01` is explicit.

## Entry Points

- Protected `/marketplace` route and sidebar link.
- Catalog filters/details and install/purchase flows.
- Installation lifecycle and runtime controls.
- Feedback/review/abuse forms and moderation queues.
- Creator/listing/package and payout flows.
- Platform review/events and analytics sections.
- Page Builder consumes separate Marketplace host-adapter methods.

## Internal Structure

One large page component declares extensive domain drafts, state collections, formatting/status helpers, loaders, mutations, and conditional sections. Multiple effects load catalog, installations, runtime controls, creator state, and role-specific administration data.

## State

Local creator, balances, analytics, listings, submissions, catalog filters/items/detail, installations, purchases, permissions, runtime state, hooks, update approvals, feedback, moderation queues, forms/files, action/loading flags, and messages. User and organization membership roles come from Zustand.

## Backend Interactions

Uses the largest frontend API surface: catalog, creator, listing, package upload, reports/review, installation lifecycle, commerce, payouts, feedback/abuse, runtime policy, kill switches, hooks, and analytics. Page Builder uses adapter components/templates separately.

## Access Control

Organization owner/admin manages installation and customer-review operations. Any authenticated member can submit selected abuse reports. Global admin/super-admin handles review/moderation, admin analytics, and global runtime controls. These client checks are informational and remain `ABV-01`.

## UI Composition

Many panels and tables cover runtime safety, permissions/hooks, catalog, detail/install confirmation, feedback, installations/updates, creator profile/listings/submissions, balances/payouts, creator analytics, platform reports, moderation, events, and admin analytics. `StatusBadge`, global CSS, Lucide, browser prompts, and file inputs are heavily used.

## Loading and Error Behavior

Separate initial loaders exist for catalog, installations, runtime, creator state, and role-specific queues/analytics. Most mutations share action/error/message state. Lifecycle controls require explicit permission/changelog/confirmation state. Errors are still flattened to one visible message rather than section-scoped recovery.

## Tests

Twelve mocked page tests cover permission confirmation, organization-vs-global role gating, paid checkout, duplicate and blocked install behavior, update reapproval, organization kill switch, review eligibility/submission, abuse reporting, global moderation, and creator/admin analytics. Real transport, all creator/submission/finance paths, routing, accessibility, and contract parity remain unverified.

## Known Risks and Unknowns

- `FBU-01`: one frontend feature contains many distinct domain lifecycles.
- Manual API types create large drift exposure (`ACU-01`).
- UI/backend permission equivalence is not certified.
- Large eager module affects bundle and change scope; measured performance is unknown.
- Installed external packages are not evidence of arbitrary server-side code execution; see backend runtime adapter boundaries.

## Related Documents

- [Feature Boundaries](../feature-boundaries.md)
- [Pages and Page Builder](pages-and-page-builder.md)
- [Testing Map](../testing-map.md)
- [Marketplace Creator and Review](../../backend/modules/marketplace-creator-review.md)
- [Marketplace Catalog and Installation](../../backend/modules/marketplace-catalog-installation.md)
- [Marketplace Runtime Adapters](../../backend/modules/marketplace-runtime-adapters.md)
- [Marketplace Finance](../../backend/modules/marketplace-finance.md)
- [Marketplace Feedback and Analytics](../../backend/modules/marketplace-feedback-analytics-readiness.md)

