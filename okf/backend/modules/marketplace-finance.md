---
okf_document_id: "backend-module-marketplace-finance"
title: "Marketplace Finance"
project: "ZinharCMS"
category: "backend-module"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
module_id: "BE-MOD-017"
module_name: "Marketplace Finance"
module_paths:
  - "backend/src/routes/marketplace_finance.rs"
  - "backend/src/services/marketplace_finance.rs"
  - "backend/src/services/stripe_billing.rs"
module_type: "Domain and integration module"
boundary_status: "OVERLAPPING"
primary_sources:
  - "backend/src/routes/marketplace_finance.rs"
  - "backend/src/services/marketplace_finance.rs"
  - "backend/src/services/stripe_billing.rs"
related_documents:
  - "backend/README.md"
  - "backend/module-catalog.md"
  - "backend/module-boundaries.md"
  - "backend/dependency-map.md"
  - "backend/testing-map.md"
  - "backend/backend-risks.md"
  - "architecture/components.md"
  - "architecture/boundaries.md"
  - "architecture/dependency-model.md"
related_diagrams:
  - "backend/diagrams/backend-module-map.mmd"
  - "backend/diagrams/backend-dependency-flow.mmd"
uncertainty_markers:
  - "RESPONSIBILITY_OVERLAP RO-03"
  - "PLANNED_NOT_IMPLEMENTED PNI-02"
  - "PLANNED_NOT_IMPLEMENTED PNI-03"
  - "NEEDS_OWNER_CONFIRMATION"
---

# Marketplace Finance

## Module Identity

| Field | Value |
|---|---|
| Module ID | `BE-MOD-017` |
| Module type | Domain and integration module |
| Implementation status | `IMPLEMENTED` |
| Boundary status | `OVERLAPPING` |
| Confidence | High |
| Source paths | `backend/src/routes/marketplace_finance.rs`; `backend/src/services/marketplace_finance.rs`; `backend/src/services/stripe_billing.rs` |

## Responsibility

Verified responsibility: Handles free/paid checkout, purchase and entitlement state, revenue ledger views, refund effects, payout onboarding/verification, creator balance, and payout requests.

Shared or inferred responsibility: Shares Stripe service and provider events with Billing; entitlements gate Marketplace installation; audit and tenant data are cross-cutting.

No additional business intent is inferred beyond current code, tests, migrations, and registered documentation.

## Owned Source Areas

- `backend/src/routes/marketplace_finance.rs`
- `backend/src/services/marketplace_finance.rs`
- `backend/src/services/stripe_billing.rs`

Ownership means the paths are primary implementation evidence for this documentation module. It does not imply exclusive table, type, or infrastructure ownership where other modules access the same data or service.

## Entry Points

`marketplace_finance::router`, finance handlers/service functions, and Marketplace branches in Stripe event processing.

## Internal Structure

Routes perform SQL/orchestration; finance service owns calculations/state helpers; Stripe service owns provider API/callback logic.

## Public and Internal Interfaces

Tenant/admin Marketplace finance routes and internal finance/provider service functions.

The intended long-term public/internal visibility contract is not separately governed unless the interface is enforced by router composition, Rust visibility, or a trait.

## Dependencies

- Internal backend dependencies: Billing/Stripe, Tenant Authorization, Marketplace Catalog/Installation, audit/RLS/RBAC, PostgreSQL, external Stripe.
- Shared infrastructure: `AppState`, `AppError`, SQLx/PostgreSQL, and cross-cutting services when listed above.
- Persistence: Purchases, entitlements, revenue ledger, refund effects, payout accounts, balances, and payout requests.
- External libraries or services: only those named in the verified responsibility and dependency statements above.

## Consumers

Marketplace buyers/creators/admins, installation entitlement checks, analytics, and billing webhook processing.

## Data Concepts

Purchases, entitlements, revenue ledger, refund effects, payout accounts, balances, and payout requests.

This is a structural ownership view, not a table or column reference. Detailed schema documentation is deferred to Phase 5.

## Request and Processing Flows

Checkout to Stripe/provider callback and entitlement; refund to ledger/entitlement effects; creator onboarding and payout request.

Detailed endpoint contracts are deferred to Phase 6.

## Error Behavior

Missing provider configuration, provider errors, permission failures and finance conflicts map through `AppError`.

## Configuration

Shared Stripe configuration; no automated transfer/settlement provider configuration is verified.

Secret values and local environment contents are intentionally excluded.

## Tests

Finance service and Stripe service include tests; route-specific integration coverage is primarily static/colocated service evidence.

Coverage percentages are `UNKNOWN` because no coverage report was used.

## Known Risks and Unknowns

Billing/Marketplace ownership overlaps. Automated transfers, partial refunds, disputes, tax and settlement remain planned or owner-defined.

Relevant markers: `RESPONSIBILITY_OVERLAP RO-03`, `PLANNED_NOT_IMPLEMENTED PNI-02`, `PLANNED_NOT_IMPLEMENTED PNI-03`, `NEEDS_OWNER_CONFIRMATION`.

## Related Documents

- [Backend Module Catalog](../module-catalog.md)
- [Module Boundaries](../module-boundaries.md)
- [Dependency Map](../dependency-map.md)
- [Testing Map](../testing-map.md)
- [Backend Risks](../backend-risks.md)
- [Architecture Components](../../architecture/components.md)
- [Architecture Boundaries](../../architecture/boundaries.md)
- [Architecture Dependency Model](../../architecture/dependency-model.md)
- [Backend Module Map](../diagrams/backend-module-map.mmd)
- [Backend Dependency Flow](../diagrams/backend-dependency-flow.mmd)
