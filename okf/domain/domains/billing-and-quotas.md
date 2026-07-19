---
okf_document_id: "domain-billing-quotas"
title: "Billing and Quotas Domain"
project: "ZinharCMS"
category: "domain-detail"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
domain_id: "DOM-BILLING"
domain_name: "Billing and Quotas"
domain_status: "IMPLEMENTED"
boundary_status: "OVERLAPPING"
primary_sources:
  - "backend/src/routes/billing.rs"
  - "backend/src/services/quota.rs"
  - "backend/src/services/stripe_billing.rs"
  - "backend/migrations/0010_v2_phase_five_billing_quota.sql"
  - "backend/migrations/0011_v2_phase_six_stripe_billing.sql"
related_documents:
  - "../domain-catalog.md"
  - "../workflows/billing-subscription.md"
  - "../../database/entities/plans-subscriptions-and-usage.md"
related_diagrams:
  - "../diagrams/cross-module-orchestration.mmd"
---

# Billing and Quotas Domain

## Domain Identity

- Domain ID: `DOM-BILLING`
- Terminology: plan, subscription, usage counter, quota, checkout, portal, billing event, provider event.
- Implementation: `IMPLEMENTED`; boundary `OVERLAPPING`; confidence High.

## Responsibility

- Verified: plan listing, current subscription, manual change, Stripe checkout/portal/callback, usage summary/rebuild, and reusable capacity/API usage checks.
- Inferred: one subscription row represents current tenant plan state.
- Shared: quotas are consumed by tenant middleware and content/page/media/member/Marketplace operations; Stripe also updates Marketplace finance.
- Unclear: dunning, reconciliation, disputes, partial refunds, and external settlement guarantees.

## Core Entities

`plans`, `organization_subscriptions`, `usage_counters`, `billing_events`, and related Marketplace purchases/entitlements/ledger.

## Core Services

Billing route, quota service, Stripe service, audit/email, tenant/RLS/RBAC, and configuration.

## API Surface

Plans, subscription/change, checkout, portal, usage/rebuild, and signed Stripe callback. See [Billing Endpoints](../../api/endpoints/billing-subscription-and-usage.md).

## Frontend Surface

`BillingPage` plan/subscription/usage/checkout/portal controls and organization usage views.

## Actors

Organization billing manager/owner, authenticated readers, Stripe provider callback, and global/creator Marketplace finance actors.

## Business Rules

`BR-BILLING-001` through `BR-BILLING-004`, plus Marketplace finance rules.

## Invariants

One subscription per organization, nonnegative monthly usage per supported metric, and unique provider events.

## State and Lifecycle

Subscription and billing-event states are mapped in [State Transitions](../state-transitions.md). Provider event ordering prevents older timestamped events from overwriting newer subscription state.

## Access Rules

Tenant billing mutations require billing-manager capability; signed webhook is public but verifies provider signature and uses explicit bypass transaction. Tenant reads remain RLS-scoped.

## Validation Rules

Plan availability/configuration, quota limits, usage metrics/month, checkout metadata, provider signatures, event structure, status mapping, and finance amount equations.

## Workflows

[Billing Subscription](../workflows/billing-subscription.md) and [Marketplace Purchase and Entitlement](../workflows/marketplace-purchase-and-entitlement.md).

## Side Effects

External Stripe sessions, subscription/event/usage writes, audit/email, and Marketplace entitlement/ledger effects. Checkout and callbacks cross separate trust/transaction boundaries.

## Tests

Quota calculation and Stripe signature/status/time ordering have focused tests. Live provider, database transaction, callback replay concurrency, tenant authorization, and UI end-to-end coverage are absent.

## Risks and Unknowns

`past_due` capacity policy rationale, external reconciliation, provider response branches, finance overlap, checkout-before-provider partial state, and retention of provider payloads.

Return to the [Domain Catalog](../domain-catalog.md).

