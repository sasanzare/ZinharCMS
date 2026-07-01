# V2 Billing Guide

This guide explains the billing behavior expected for V2 General Availability.

## Plans

V2 ships with plan definitions for Free, Pro, and Enterprise-style deployments. Plan limits can include:

- organization members
- pages and entries
- media storage
- API requests

Enterprise pricing and limits are deployment-specific. Production deployments can present Enterprise as a custom/contact-sales plan instead of a fixed public price.

## Subscription State

Each organization has one active subscription record. The subscription controls:

- current plan
- subscription status
- billing period
- Stripe customer and subscription references, when configured
- quota behavior

If subscription data is missing, the admin UI should clearly show an unavailable or fallback state rather than silently granting paid access.

## Stripe Webhooks

Stripe webhook processing must be idempotent. Repeated or out-of-order events should not corrupt subscription state.

For webhook issues:

1. Check that the webhook signing secret is configured.
2. Confirm the event exists in `billing_events`.
3. Confirm duplicate events are ignored safely.
4. Confirm older events do not overwrite newer subscription state.
5. Confirm billing audit logs include the attempted state change.

## Plan Changes

Plan upgrades and downgrades should:

- write a billing audit record
- update subscription state
- refresh plan limits
- keep quota counters visible
- avoid blocking the organization when the change is still pending

Manual plan changes should be restricted to authorized platform administrators.

## Quota Enforcement

Quota counters are rebuilt from live organization data and stored in usage counters. Rebuild counters when:

- imported data appears missing from billing usage
- media storage totals look incorrect
- a plan change was applied but limits still show stale values
- support receives a quota exceeded report that does not match visible usage

Do not bypass quota checks permanently. Use documented temporary overrides only when approved.

## Common Billing Incidents

### Organization Shows The Wrong Plan

1. Refresh billing from the admin UI.
2. Check the latest Stripe event time.
3. Check `billing_events` for failed or ignored events.
4. Confirm the active subscription row matches the intended plan.

### Valid Customer Is Blocked By Quota

1. Rebuild usage counters.
2. Compare actual usage to the active plan.
3. Check pending plan changes.
4. Apply a temporary support override only if approved.

### Webhook Signature Failure

1. Confirm the configured signing secret belongs to the correct Stripe endpoint.
2. Confirm the request body is not modified by a proxy.
3. Replay the event from Stripe after the secret is corrected.

### Payment Or Checkout Failure

1. Confirm Stripe customer and subscription IDs.
2. Review the Stripe dashboard for failed payment or checkout session state.
3. Keep the organization on the previous valid plan unless policy requires suspension.