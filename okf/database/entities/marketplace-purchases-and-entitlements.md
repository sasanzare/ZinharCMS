---
okf_document_id: "database-entity-marketplace-purchases-entitlements"
title: "Marketplace Purchases and Entitlements"
project: "ZinharCMS"
category: "database-entity"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
entity_id: "DB-ENT-016"
entity_name: "Marketplace Purchases and Entitlements"
entity_domain: "Marketplace Commerce"
schema_objects: ["marketplace_purchases", "marketplace_entitlements"]
owning_module: "Marketplace Finance"
tenant_scope: "tenant"
implementation_status: "IMPLEMENTED"
confidence: "high"
primary_sources: ["backend/migrations/0022_v3_phase_nine_marketplace_finance.sql", "backend/migrations/0023_v3_phase_nine_finance_hardening.sql", "backend/src/services/marketplace_finance.rs", "backend/src/services/stripe_billing.rs"]
related_documents: ["database/transactions-and-consistency.md", "database/multi-tenancy.md", "backend/modules/marketplace-finance.md"]
related_diagrams: ["database/diagrams/entity-relationship-overview.mmd"]
uncertainty_markers: ["TRANSACTION_BOUNDARY_UNCLEAR TBU-03", "CONSTRAINT_COVERAGE_UNCLEAR CCU-01"]
---

# Marketplace Purchases and Entitlements

## Entity Identity

`DB-ENT-016` represents tenant purchase state and the resulting right to use a Marketplace listing/version.

## Purpose

Verified purpose: persist free or provider-backed checkout, idempotent receipts/provider IDs, and entitlement grants. Refund/dispute automation is not implemented as a complete workflow.

## Storage Structure

| Field group | Type/null/default/constraint | Meaning and evidence |
| --- | --- | --- |
| purchase org/user/listing/version | UUID FKs | Buyer and product context |
| amount/currency/status/receipt | Numeric/text/checks/unique | Commercial and idempotency state |
| provider checkout/payment IDs | Sensitive text/partial unique | External correlation |
| entitlement org/purchase/listing/version | UUID FKs | Granted access |
| entitlement status/expiry/source | Checked text/time | Access lifecycle |

## Identifiers

Both tables use UUID PKs. Receipt and non-null provider checkout identifiers have uniqueness. Active entitlement/listing combinations use partial uniqueness.

## Relationships

Purchases connect organization, optional buyer, listing, and version. Entitlements connect organization/listing/version back to one purchase and are consumed by installation flows.

## Ownership

Marketplace finance owns writes; Stripe processing and installation authorization are cross-module participants.

## Tenant Isolation

Both tables are forced RLS. Global catalog references and tenant buyer context coexist; same-tenant user/purchase/entitlement coherence requires tests.

## Lifecycle

Free checkout creates purchase and entitlement atomically. Paid checkout commits pending state before the provider call and later reconciles success/failure (`TBU-03`).

## Constraints and Indexes

FKs, amount/currency/status checks, receipt/provider uniqueness, active entitlement partial uniqueness, and tenant/status indexes protect commercial flows.

## Persistence Mapping

Finance and Stripe services use direct SQLx transactions and `FOR UPDATE` in concurrency-sensitive provider-event processing.

## Security and Privacy

Payment correlation IDs, amounts, buyer identities, and entitlement state are sensitive commercial data.

## Known Risks and Unknowns

Provider reconciliation, compensation, refund/dispute behavior, tenant coherence, and retention are unresolved.

## Related Documents

See [Transactions and Consistency](../transactions-and-consistency.md), [Marketplace Ledger and Payouts](marketplace-ledger-and-payouts.md), and [Database Risks](../database-risks.md).

Return to the [Entity Catalog](../entity-catalog.md) or continue with the owning [Marketplace Finance module](../../backend/modules/marketplace-finance.md).
