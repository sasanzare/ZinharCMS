---
okf_document_id: "backend-module-billing-quotas"
title: "Billing and Quotas"
project: "ZinharCMS"
category: "backend-module"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
module_id: "BE-MOD-005"
module_name: "Billing and Quotas"
module_paths:
  - "backend/src/routes/billing.rs"
  - "backend/src/services/quota.rs"
  - "backend/src/services/stripe_billing.rs"
module_type: "Domain and integration module"
boundary_status: "OVERLAPPING"
primary_sources:
  - "backend/src/routes/billing.rs"
  - "backend/src/services/quota.rs"
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
  - "BUSINESS_RULE_UNVERIFIED BRU-02"
  - "PLANNED_NOT_IMPLEMENTED PNI-03"
---

# Billing and Quotas

## Module Identity

| Field | Value |
|---|---|
| Module ID | `BE-MOD-005` |
| Module type | Domain and integration module |
| Implementation status | `IMPLEMENTED` |
| Boundary status | `OVERLAPPING` |
| Confidence | High |
| Source paths | `backend/src/routes/billing.rs`; `backend/src/services/quota.rs`; `backend/src/services/stripe_billing.rs` |

## Responsibility

Verified responsibility: Lists plans and usage, changes subscription state, starts Stripe checkout/customer portal flows, verifies Stripe callbacks, rebuilds usage, and supplies reusable plan/quota policy.

Shared or inferred responsibility: Stripe billing also supports Marketplace finance; quota checks are invoked from tenant middleware and feature routes.

No additional business intent is inferred beyond current code, tests, migrations, and registered documentation.

## Owned Source Areas

- `backend/src/routes/billing.rs`
- `backend/src/services/quota.rs`
- `backend/src/services/stripe_billing.rs`

Ownership means the paths are primary implementation evidence for this documentation module. It does not imply exclusive table, type, or infrastructure ownership where other modules access the same data or service.

## Entry Points

`billing::router`, `billing::public_router`, quota service functions, and Stripe billing client/webhook functions.

## Internal Structure

Routes orchestrate HTTP/SQL/audit/email; quota owns plan limits and usage checks; Stripe service owns provider requests and event handling.

## Public and Internal Interfaces

Billing routers and DTOs, `PlanLimits`, quota check/rebuild functions, Stripe request and webhook functions.

The intended long-term public/internal visibility contract is not separately governed unless the interface is enforced by router composition, Rust visibility, or a trait.

## Dependencies

- Internal backend dependencies: Tenant Authorization/RLS/RBAC, Organizations, audit/email, PostgreSQL, Config, reqwest, and Stripe.
- Shared infrastructure: `AppState`, `AppError`, SQLx/PostgreSQL, and cross-cutting services when listed above.
- Persistence: Plans, subscriptions, usage counters, billing events, provider identifiers, and finance records touched by Stripe events.
- External libraries or services: only those named in the verified responsibility and dependency statements above.

## Consumers

Tenant middleware, billing UI/API, media/content/pages/Marketplace quotas, and Marketplace finance.

## Data Concepts

Plans, subscriptions, usage counters, billing events, provider identifiers, and finance records touched by Stripe events.

This is a structural ownership view, not a table or column reference. Detailed schema documentation is deferred to Phase 5.

## Request and Processing Flows

Tenant billing request to SQL/service/provider; signed callback to provider event processing and database updates.

Detailed endpoint contracts are deferred to Phase 6.

## Error Behavior

Missing provider configuration and provider failures map to `AppError`; webhook signature failures reject callbacks.

## Configuration

Stripe keys/webhook secret, success/cancel/portal URLs, price IDs, and email configuration; no secret values are recorded.

Secret values and local environment contents are intentionally excluded.

## Tests

Billing route has a static test; quota and Stripe services contain unit/static contract tests.

Coverage percentages are `UNKNOWN` because no coverage report was used.

## Known Risks and Unknowns

Manual/provider state transitions and finance sharing blur ownership. Dunning, settlement, and partial-refund intent are not established.

Relevant markers: `RESPONSIBILITY_OVERLAP RO-03`, `BUSINESS_RULE_UNVERIFIED BRU-02`, `PLANNED_NOT_IMPLEMENTED PNI-03`.

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
