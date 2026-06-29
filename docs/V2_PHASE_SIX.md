# V2 Phase 6: Stripe Billing Lifecycle

## Scope

Phase 6 connects the internal plan and quota system to Stripe test mode. The application can create Stripe customers, start checkout for paid plans, open the Stripe customer portal, and process subscription webhooks without applying duplicate events twice.

## Backend

- Added Stripe configuration for secret key, webhook secret, price IDs, and return URLs.
- Added `stripe_product_id` and `stripe_price_id` fields to `plans`.
- Added `billing_events` as an idempotent billing audit log.
- Added Stripe checkout and customer portal endpoints.
- Added a public Stripe webhook endpoint with signature verification.
- Processed `checkout.session.completed`, `customer.subscription.updated`, and `customer.subscription.deleted`.
- Mapped Stripe customers and subscriptions onto `organization_subscriptions`.
- Kept quota calculation resilient by falling back to the Free plan when a subscription is no longer active.

## Frontend

- Billing page paid-plan actions now start Stripe Checkout.
- Billing managers can open the Stripe Customer Portal.
- Enterprise plans with no fixed monthly price render as custom pricing instead of `$0/mo`.
- Plans without a configured Stripe price are shown as unavailable for checkout.

## Environment

Set these values for Stripe test mode:

```env
STRIPE_SECRET_KEY=sk_test_...
STRIPE_WEBHOOK_SECRET=whsec_...
STRIPE_PRO_PRICE_ID=price_...
STRIPE_ENTERPRISE_PRICE_ID=price_...
STRIPE_SUCCESS_URL=http://localhost:5173/billing?billing=success
STRIPE_CANCEL_URL=http://localhost:5173/billing?billing=cancelled
STRIPE_PORTAL_RETURN_URL=http://localhost:5173/billing
```

Plan-specific price IDs can also be stored in the `plans.stripe_price_id` column. Environment values are a local-development fallback.

## Webhook

Stripe should send events to:

```text
POST /api/billing/stripe/webhook
```

The webhook stores every Stripe event in `billing_events` using `(provider, provider_event_id)` as the idempotency key.
