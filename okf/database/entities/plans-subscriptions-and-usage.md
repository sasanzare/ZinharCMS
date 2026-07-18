---
okf_document_id: "database-entity-plans-subscriptions-usage"
title: "Plans, Subscriptions, and Usage"
project: "ZinharCMS"
category: "database-entity"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
entity_id: "DB-ENT-010"
entity_name: "Plans, Subscriptions, and Usage"
entity_domain: "SaaS Billing"
schema_objects: ["plans", "organization_subscriptions", "usage_counters", "billing_events"]
owning_module: "Billing"
tenant_scope: "mixed"
implementation_status: "IMPLEMENTED"
confidence: "high"
primary_sources: ["backend/migrations/0010_v2_phase_five_billing_quota.sql", "backend/migrations/0011_v2_phase_six_stripe_billing.sql", "backend/src/routes/billing.rs", "backend/src/services/stripe_billing.rs"]
related_documents: ["database/transactions-and-consistency.md", "database/multi-tenancy.md", "backend/modules/billing-quotas.md"]
related_diagrams: ["database/diagrams/database-domain-map.mmd"]
uncertainty_markers: ["TRANSACTION_BOUNDARY_UNCLEAR TBU-03", "CONSTRAINT_COVERAGE_UNCLEAR CCU-02"]
---

# Plans, Subscriptions, and Usage

## Entity Identity

`DB-ENT-010` groups the global plan catalog, one current subscription per organization, tenant usage counters, and provider billing-event history.

## Purpose

Verified purpose: select plans, track provider-backed subscription state, meter period/metric usage, and process billing events idempotently. Product pricing rationale is not inferred.

## Storage Structure

| Field group | Type/null/default/constraint | Meaning and evidence |
| --- | --- | --- |
| plan ID/key/name/limits/pricing | UUID/text/numeric/JSON-like fields | Global commercial catalog |
| subscription org/plan/status | Organization PK, plan FK, enum | Current tenant subscription |
| provider IDs/period times | Sensitive text/time, uniqueness | External provider linkage |
| usage org/period/metric/value | UUID/date/text/numeric, composite uniqueness | Metered usage |
| billing event provider/event/status/payload | Text/JSONB/time, unique provider event | Idempotent processing record |

## Identifiers

Plans use migration-defined UUID/key identity. Subscription identity is `organization_id`. Usage is tenant/period/metric-qualified. Billing provider-event identity is unique.

## Relationships

Subscriptions connect organizations to plans. Usage and billing events belong to organizations; events may logically reference subscription/provider records.

## Ownership

Billing and Stripe services own writes. Organizations initializes subscription state; middleware/quotas read plans and usage.

## Tenant Isolation

Subscription, usage, and billing events are forced RLS. Plans are global catalog rows. Provider webhook processing uses privileged/bypass transactions.

## Lifecycle

Subscription uses a PostgreSQL status enum and period/provider timestamps. Usage is period-scoped. Billing events preserve processed/failed history.

## Constraints and Indexes

Organization subscription PK, tenant/period/metric uniqueness, provider-event uniqueness, FKs, checks, and period/status indexes support correctness.

## Persistence Mapping

Billing routes and Stripe services use direct SQLx transactions. Provider event handling locks purchase records in the separate Marketplace finance domain where applicable.

## Security and Privacy

Provider customer/subscription/event identifiers and payloads are sensitive. Never document real provider data.

## Known Risks and Unknowns

Provider reconciliation, status-text/application alignment, retention, and operational recovery are unclear.

## Related Documents

See [Transactions and Consistency](../transactions-and-consistency.md), [Multi-Tenancy](../multi-tenancy.md), and [Marketplace Ledger and Payouts](marketplace-ledger-and-payouts.md).

Return to the [Entity Catalog](../entity-catalog.md) or continue with the owning [Billing and Quotas module](../../backend/modules/billing-quotas.md).
