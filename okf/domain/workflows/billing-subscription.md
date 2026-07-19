---
okf_document_id: "workflow-billing-subscription"
title: "Billing Subscription Workflow"
project: "ZinharCMS"
category: "domain-workflow"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
workflow_id: "WF-BILLING"
workflow_name: "Billing Subscription"
workflow_domain: "DOM-BILLING"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "backend/src/routes/billing.rs"
  - "backend/src/services/quota.rs"
  - "backend/src/services/stripe_billing.rs"
related_documents:
  - "../cross-module-workflows.md"
  - "../domains/billing-and-quotas.md"
  - "../../api/endpoints/billing-subscription-and-usage.md"
related_diagrams:
  - "../diagrams/cross-module-orchestration.mmd"
---

# Billing Subscription Workflow

## Workflow Identity

- ID/name/domain: `WF-BILLING`, Billing Subscription, `DOM-BILLING`.
- Trigger/actors: tenant billing manager changes plan/starts checkout; Stripe posts signed event.
- Status/confidence: `IMPLEMENTED`; High internal rules, Medium external end-to-end.

## Preconditions

Tenant mutation requires active membership and billing-manager capability. Target plan must be active/configured. Stripe callback requires signature and parseable supported event metadata.

## Main Flow

Manual plan path:

1. Validate actor and load target plan.
2. Upsert tenant subscription as active/manual.
3. Audit and optionally email updated subscription information.
4. Return current subscription.

Stripe path:

1. Create checkout/customer session through provider for configured plan/customer.
2. Provider later sends signed event.
3. Verify signature and begin bypass transaction.
4. Insert unique processing billing-event row; return idempotently if already processed.
5. Map event to subscription or Marketplace finance handler.
6. Apply only non-older subscription event state.
7. Mark billing event processed/ignored/failed and commit/return.

## Alternative Flows

Customer portal manages provider subscription externally. Usage rebuild recomputes rebuildable metrics. Free/manual plans can change without Stripe checkout.

## Failure Flows

Missing config/price/customer/provider failure rejects checkout/portal. Invalid signature rejects callback. Processing error records failure when possible. External checkout may exist while local callback has not completed.

## State Changes

Subscription changes among allowed provider states. Billing event changes from processing to processed/ignored/failed. Quota behavior reads current qualifying subscription states.

## Data Changes

Subscription/provider IDs/periods, billing event, usage counters, audit/email, and possibly Marketplace purchase/entitlement/ledger.

## Transaction Boundaries

Provider session creation is external. Webhook database application is transactional and idempotent by event ID.

## Side Effects

Stripe HTTP, audit, email, and downstream capacity changes.

## Completion Criteria

Manual change returns persisted subscription. Provider path completes when signed event is recorded and applied/ignored transactionally.

## Tests

Status mapping, timestamp ordering, signature, event time, quota calculations, and config gate tests. No live provider/database callback or UI end-to-end.

## Unknowns and Risks

Reconciliation, dunning, stale/missing timestamps, provider retry operations, `past_due` quota policy rationale, and Marketplace finance overlap.

Return to [Cross-Module Workflows](../cross-module-workflows.md).

