---
okf_document_id: "marketplace-area-commerce-entitlements"
title: "Marketplace Commerce and Entitlements"
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
  - "backend/src/routes/marketplace_finance.rs"
  - "backend/src/services/marketplace_finance.rs"
  - "backend/migrations/0022_v3_phase_nine_marketplace_finance.sql"
  - "frontend/src/pages/MarketplacePage.tsx"
marketplace_area_id: "MPA-005"
marketplace_area_name: "Commerce and Entitlements"
implementation_status: "implemented_contracts_deployment_unverified"
related_documents:
  - "../marketplace-architecture.md"
  - "../marketplace-workflows.md"
  - "../../domain/domains/marketplace.md"
related_diagrams:
  - "../diagrams/marketplace-installation-flow.mmd"
---

# Marketplace Commerce and Entitlements

Marketplace finance routes cover checkout, purchases, creator payout onboarding/verification, balances, revenue ledger, and payout requests. Paid installation and update paths require matching entitlement state. Free products can use installation without a paid transaction.

Database migrations model purchases, entitlements, ledger entries, payout accounts, and payouts with tenant or creator ownership as appropriate. Stripe-facing behavior is host-owned; a Marketplace package never receives payment credentials.

Code and contracts are VERIFIED, but live provider configuration, webhook delivery, reconciliation, refunds in all edge cases, and payout operations were not executed during Phase 9. IMPLEMENTATION_STATUS_UNCLEAR applies only to deployment/provider state, not route existence.

See [Marketplace Architecture](../marketplace-architecture.md).

## Purpose

Connect paid Marketplace acquisition to tenant entitlement, installation eligibility, creator revenue, and payouts through host-owned provider behavior.

## Entities

marketplace_purchases, marketplace_entitlements, marketplace_revenue_ledger, marketplace_payout_accounts, marketplace_payouts, listings, versions, and installations.

## Backend Module

routes/marketplace_finance.rs, marketplace_finance.rs, Stripe billing integration, and paid gates in routes/marketplace.rs.

## APIs

Checkout, purchase list, payout onboarding/verification, creator balance, revenue ledger, payout request, and paid install/update checks.

## Frontend Feature

MarketplacePage exposes paid checkout, purchase state, and creator finance views/actions.

## Permissions

Organization Marketplace/billing roles initiate tenant purchases; creator identity owns payout data; sensitive provider/admin behavior remains host-controlled.

## Tenant Scope

Purchases and entitlements are organization-owned; ledger/payout records are creator/platform finance state.

## Workflows

MP-WF-06 paid branch and MP-WF-12 purchase/entitlement.

## Tests

Finance unit tests and MarketplacePage tests cover pricing splits, invalid state, UI checkout, and phase contracts. Live provider, webhook, reconciliation, and concurrency are unverified.

## Risks

Provider deployment/configuration, external-call transaction ordering, refund/entitlement races, payout operations, privacy, and reconciliation require Phase 10 operational testing.

## Implementation Status

IMPLEMENTED_CONTRACTS_DEPLOYMENT_UNVERIFIED.
