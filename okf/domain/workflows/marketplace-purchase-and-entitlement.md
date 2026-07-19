---
okf_document_id: "workflow-marketplace-purchase-entitlement"
title: "Marketplace Purchase and Entitlement Workflow"
project: "ZinharCMS"
category: "domain-workflow"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
workflow_id: "WF-MARKET-PURCHASE"
workflow_name: "Marketplace Purchase and Entitlement"
workflow_domain: "DOM-MARKETPLACE"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "backend/src/routes/marketplace_finance.rs"
  - "backend/src/services/marketplace_finance.rs"
  - "backend/src/services/stripe_billing.rs"
  - "backend/migrations/0022_v3_phase_nine_marketplace_finance.sql"
related_documents:
  - "../cross-module-workflows.md"
  - "../domains/marketplace.md"
  - "../domains/billing-and-quotas.md"
related_diagrams:
  - "../diagrams/cross-module-orchestration.mmd"
---

# Marketplace Purchase and Entitlement Workflow

## Workflow Identity

- ID/name/domain: `WF-MARKET-PURCHASE`, Marketplace Purchase and Entitlement, `DOM-MARKETPLACE`.
- Trigger/actors: tenant billing actor starts checkout; Stripe confirms/refunds; creator/admin views finance.
- Status/confidence: `IMPLEMENTED`; High finance invariants, Medium provider end-to-end.

## Preconditions

Authenticated active tenant; approved paid listing/version; pricing/currency/amount valid; no reusable pending/completed checkout conflict; provider configuration; creator payout account rules where relevant.

## Main Flow

1. Validate listing/version price and tenant actor.
2. Create or reuse pending purchase/receipt context.
3. Request Stripe checkout session with purchase metadata.
4. Return checkout URL while purchase remains pending.
5. Signed provider event enters idempotent billing-event transaction.
6. Lock purchase, verify paid status, and mark completed.
7. Grant one active entitlement for purchase.
8. Append revenue/refund ledger entries according to tax/platform/creator split.
9. Purchase/entitlement becomes eligible for installation and customer review.
10. Refund callback marks purchase refunded and revokes active entitlement transactionally.

## Alternative Flows

Free products bypass paid purchase and use install eligibility. Existing completed purchase/entitlement supports later installation. Creator can onboard payout account; global admin verifies it. Payout records exist but automated transfer is not implemented.

## Failure Flows

Invalid price/currency/payout, provider configuration/HTTP, unpaid callback, wrong metadata, duplicate event, or DB constraint can reject/ignore/fail. A provider session can exist before local completion callback.

## State Changes

Purchase pending/completed/failed/refunded/canceled; entitlement active/revoked; billing event processing terminal status; payout account/payout states on separate flows.

## Data Changes

Purchase, entitlement, billing event, revenue ledger, payout account/payout, and audit/provider metadata rows.

## Transaction Boundaries

Checkout provider call crosses committed/local state. Provider completion/refund application uses a bypass transaction with row locking and idempotent billing event.

## Side Effects

Stripe HTTP, receipt/provider identifiers, install/review eligibility. Automated payout transfer and partial refund/dispute automation are absent.

## Completion Criteria

Paid completion transaction commits purchase, entitlement, and ledger effects. Checkout URL creation alone is not purchase completion.

## Tests

Amount split, invalid pricing/payout, Stripe signature/status/order, static lifecycle/security contracts, and frontend paid checkout tests. No live Stripe/database settlement flow.

## Unknowns and Risks

Reconciliation, partial refunds/disputes, automated payout settlement, provider/local divergence, financial retention, and operational recovery.

Return to [Cross-Module Workflows](../cross-module-workflows.md).

