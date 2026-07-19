---
okf_document_id: "domain-marketplace"
title: "Marketplace Domain"
project: "ZinharCMS"
category: "domain-detail"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
domain_id: "DOM-MARKETPLACE"
domain_name: "Marketplace"
domain_status: "PARTIALLY_IMPLEMENTED"
boundary_status: "OVERLAPPING"
primary_sources:
  - "backend/src/routes/marketplace.rs"
  - "backend/src/routes/marketplace_runtime.rs"
  - "backend/src/routes/marketplace_adapters.rs"
  - "backend/src/routes/marketplace_finance.rs"
  - "backend/src/services/marketplace_domain.rs"
  - "backend/src/services/marketplace_installation.rs"
  - "backend/src/services/marketplace_runtime.rs"
  - "backend/migrations/0015_v3_phase_one_marketplace_foundation.sql"
related_documents:
  - "../domain-catalog.md"
  - "../workflows/marketplace-product-publication.md"
  - "../workflows/marketplace-installation-lifecycle.md"
  - "../workflows/marketplace-purchase-and-entitlement.md"
related_diagrams:
  - "../diagrams/cross-module-orchestration.mmd"
---

# Marketplace Domain

## Domain Identity

- Domain ID: `DOM-MARKETPLACE`
- Terminology: creator, listing, product type, version, manifest, artifact, submission, review, installation, permission, runtime, adapter, purchase, entitlement, ledger, payout, review, abuse report.
- Implementation: `PARTIALLY_IMPLEMENTED`; boundary `OVERLAPPING`; confidence High.

## Responsibility

- Verified: creator/catalog submission, package storage/validation/review, catalog/search, install/update/rollback/uninstall, runtime permission/kill switch, host adapters, one-time purchase/entitlement, feedback/abuse, analytics, and readiness checks.
- Inferred: Marketplace is a collection of related bounded contexts rather than one cohesive aggregate.
- Shared: Organizations/RBAC/RLS, Billing/Stripe, Pages/components, files, audit, beta/readiness.
- Unclear: arbitrary execution/sandbox, automated payouts, dispute/partial-refund automation, and production release state.

## Core Entities

Marketplace entity groups `DB-ENT-013` through `DB-ENT-018`: creators; catalog/version/submission/review events; installations/runtime adapters; purchases/entitlements; ledger/payouts; reviews/abuse/notifications.

## Core Services

Marketplace domain/policy/manifest/package/submission/validation/review/catalog/installation/runtime/adapters/finance/feedback/analytics/performance/readiness services and four route families.

## API Surface

Creator/catalog/review, installation, runtime, adapter, finance, feedback, analytics, and readiness endpoints under `/api/marketplace*`. See the Phase 6 Marketplace endpoint documents.

## Frontend Surface

`MarketplacePage` owns customer catalog/install/purchase/review, creator submission/payout/analytics, and administrator review/risk/analytics operations.

## Actors

Authenticated catalog reader, organization member/admin/billing actor, approved creator owner, global admin/reviewer, provider callback, and installed host adapter caller.

## Business Rules

`BR-MARKET-001` through `BR-MARKET-008` and security permission rules.

## Invariants

Artifact immutability, manifest/version/listing coherence, bounded risk/validation/status values, pinned installation, exact permission approval, finance amount equality, purchase-entitlement uniqueness, and feedback bounds.

## State and Lifecycle

Creator, listing, version, submission, installation/runtime, purchase/entitlement, payout, review, abuse, and notification states are cataloged in [State Transitions](../state-transitions.md). Several paths are explicit, but no single state machine covers the whole domain.

## Access Rules

Global Marketplace administration, creator ownership, tenant organization role, entitlement/eligibility, exact manifest permission, runtime operation mapping, and forced tenant RLS are combined. See [Marketplace Management Permissions](../../security/permissions/marketplace-management.md) and [Runtime Capabilities](../../security/permissions/marketplace-runtime-capabilities.md).

## Validation Rules

Creator/listing metadata, manifest schema, semantic version, package size/path/ZIP contents, risk/compatibility, review decisions, permission deltas, runtime payload/entry points, pricing/payout state, rating/abuse taxonomy, and analytics bounds.

## Workflows

[Product Publication](../workflows/marketplace-product-publication.md), [Installation Lifecycle](../workflows/marketplace-installation-lifecycle.md), and [Purchase and Entitlement](../workflows/marketplace-purchase-and-entitlement.md).

## Side Effects

Artifact files, many transactional rows, review/audit/internal notification history, Stripe sessions/callbacks, Page Builder imports/adapters, and operational analytics. Uploaded package code is not executed.

## Tests

Marketplace has the strongest focused service/static/frontend test set. Full live database/provider/file/tenant flows, automatic payout, arbitrary execution, and deployment readiness are not covered by those unit/static contracts.

## Risks and Unknowns

Dense boundary overlap, handler/service duplication, multiple status graphs, external provider and file transactions, RLS bypass callers, incomplete automation, and Phase 9 need for dedicated extensibility documentation.

Return to the [Domain Catalog](../domain-catalog.md).

