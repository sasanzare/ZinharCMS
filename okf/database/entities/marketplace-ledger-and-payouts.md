---
okf_document_id: "database-entity-marketplace-ledger-payouts"
title: "Marketplace Ledger and Payouts"
project: "ZinharCMS"
category: "database-entity"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
entity_id: "DB-ENT-017"
entity_name: "Marketplace Ledger and Payouts"
entity_domain: "Marketplace Finance"
schema_objects: ["marketplace_revenue_ledger", "marketplace_payout_accounts", "marketplace_payouts"]
owning_module: "Marketplace Finance"
tenant_scope: "mixed"
implementation_status: "PARTIALLY_IMPLEMENTED"
confidence: "high"
primary_sources: ["backend/migrations/0022_v3_phase_nine_marketplace_finance.sql", "backend/migrations/0023_v3_phase_nine_finance_hardening.sql", "backend/src/services/marketplace_finance.rs"]
related_documents: ["database/transactions-and-consistency.md", "database/lifecycle-and-auditing.md", "backend/modules/marketplace-finance.md"]
related_diagrams: ["database/diagrams/entity-relationship-overview.mmd"]
uncertainty_markers: ["PLANNED_NOT_IMPLEMENTED PNI-02", "DATA_LIFECYCLE_UNCLEAR DLU-01"]
---

# Marketplace Ledger and Payouts

## Entity Identity

`DB-ENT-017` groups tenant-attributed Marketplace revenue ledger entries and global creator payout account/batch state. Ledger and payout recordkeeping are implemented; automated transfer settlement is `PNI-02`.

## Purpose

Verified purpose: record revenue allocation, expose creator balance inputs, store provider payout account linkage, and track payout status. It does not prove money movement.

## Storage Structure

| Field group | Type/null/default/constraint | Meaning and evidence |
| --- | --- | --- |
| ledger org/purchase/creator | UUID FKs | Tenant sale and creator attribution |
| entry type/amount/currency | Checked text/numeric | Append-only financial movement |
| provider event/idempotency | Sensitive text/partial unique | External correlation |
| payout account creator/provider/status | UUID/text/checks | Creator settlement setup |
| payout creator/amount/status/provider transfer | UUID FK/numeric/text/time | Payout lifecycle record |

## Identifiers

UUID PKs identify rows. Non-null provider event identifiers use partial uniqueness. Provider account IDs are external correlations, not public IDs.

## Relationships

Ledger rows reference organizations, purchases, creators, and listings. Payout accounts and payouts separately reference creators; no payout-to-account FK is defined.

## Ownership

Marketplace finance owns all writes; creator dashboards and provider webhook/checkout logic consume the data.

## Tenant Isolation

Ledger is forced RLS; creator payout accounts/payouts are global creator finance records protected by privileged application paths.

## Lifecycle

The ledger rejects UPDATE and DELETE through `trg_marketplace_revenue_ledger_append_only`. Payout status is mutable workflow state. Automated settlement is not implemented.

## Constraints and Indexes

Amount/currency/type checks, FKs, provider idempotency, append-only trigger, and creator/status/date indexes protect finance paths.

## Persistence Mapping

Marketplace finance services use SQLx transactions and local row types. Free checkout writes purchase, entitlement, ledger, and audit atomically.

## Security and Privacy

Amounts, provider account/event IDs, creator identities, and payout states are highly sensitive financial data.

## Known Risks and Unknowns

Settlement execution, reconciliation, refund/dispute accounting, retention, and accounting governance are unresolved.

## Related Documents

See [Marketplace Purchases and Entitlements](marketplace-purchases-and-entitlements.md), [Transactions and Consistency](../transactions-and-consistency.md), and [Lifecycle and Auditing](../lifecycle-and-auditing.md).

Return to the [Entity Catalog](../entity-catalog.md) or continue with the owning [Marketplace Finance module](../../backend/modules/marketplace-finance.md).
