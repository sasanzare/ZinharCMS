# V3 Marketplace Phase 9 — Monetization

Phase 9 adds Marketplace product finance without coupling it to organization
subscription billing. Migration `0022_v3_phase_nine_marketplace_finance.sql` is
the source of truth for purchases, entitlements, revenue ledger entries, payout
accounts, and payout records.

## 9.1 Free products

`POST /api/marketplace/purchases/checkout` creates a completed zero-value
purchase, receipt, active organization entitlement, and zero-value ledger entry
for an approved free listing/version. Free Component Packs and Design Templates
can still be installed without a paid purchase; install-time compatibility,
artifact integrity, and exact permission approval remain mandatory.

## 9.2 Paid products

The same checkout endpoint creates a pending organization purchase and a Stripe
one-time Checkout Session for a paid listing. Price, currency, tax metadata,
receipt number, provider identifiers, and status are persisted independently of
SaaS subscription tables. A signed `checkout.session.completed` webhook grants
the entitlement only when payment status, amount, currency, and purchase metadata
match. Paid install, re-enable, update, and rollback all require an active
entitlement. Refund revokes it.

`GET /api/marketplace/purchases` returns organization receipts and payment state.
Custom pricing and subscription add-ons remain outside this one-time-purchase
implementation.

## 9.3 Revenue split ledger

Every completed purchase has one idempotent `purchase` ledger entry. The default
commission is 20% of the pre-tax amount; the remainder is the creator share.
Full refunds create one reversing `refund` entry and revoke the entitlement.
Unique `(purchase_id, entry_type)` entries and Stripe event idempotency prevent
duplicate financial effects. `GET /api/marketplace/revenue-ledger` exposes the
tenant-scoped audit trail.

## 9.4 Payout provider onboarding

Creator owners can register a Stripe Connect-style provider account through
`GET/POST /api/marketplace/creators/{creator_id}/payout`. The account starts in
`pending`. A global admin records provider-attested state through
`POST /api/marketplace/creators/{creator_id}/payout/verify`. Verification is
rejected unless the creator is approved, provider details are submitted, and
payouts are enabled. An unverified or restricted creator is therefore not payout
eligible. Actual transfer scheduling/execution remains a later operational step.

## Acceptance

- Free products can be entitled and installed without a paid transaction.
- Paid products cannot be installed before a successful, matching payment.
- Each completed payment and full refund has one auditable ledger effect.
- Organization subscription billing and Marketplace purchases remain separate.
- Unverified creators cannot become payout eligible.
- Tenant purchase, entitlement, and ledger rows use forced RLS.
