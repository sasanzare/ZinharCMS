# V3 Marketplace Phase 10: Customer Feedback and Abuse Reporting

Phase 10 implements the two proposal subphases without changing Marketplace
purchase, entitlement, package-review, or runtime-execution boundaries.

## 10.1 Rating and customer reviews

- `marketplace_product_reviews` stores one review per organization and listing,
  a star rating from one to five, review text, version context, author, and a
  `pending`, `published`, or `rejected` moderation state.
- An organization owner or admin can submit or replace its review only when the
  organization has an installed product or a completed Marketplace purchase.
  This check is performed in the tenant transaction; the browser restriction is
  informational and never authorizes the action.
- Re-submission returns the review to `pending` and clears prior moderation.
  Published reviews alone affect catalog averages and appear in catalog detail.
- Global Marketplace admins publish or reject reviews and every submission or
  moderation action is audit logged.
- A global pending-review queue makes review moderation discoverable without
  exposing tenant identifiers or internal moderation metadata to catalog users.

## 10.2 Abuse reports

- Any authenticated tenant may submit a listing/version report with a violation
  type (`malware`, `copyright`, `spam`, `fraud`, `privacy`, or `other`),
  low/medium/high/critical severity, JSON-object evidence, and description.
- `marketplace_abuse_reports` is a forced-RLS tenant table. Global Marketplace
  admins use the cross-organization moderation queue and can investigate,
  resolve, or dismiss a report.
- A critical report gets `notification_status = created`, a timestamp, a
  persisted unread `marketplace_internal_notifications` record addressed to the
  global admin role, and a `marketplace.abuse_report.critical_notification`
  audit record in the same transaction. Delivery to an external pager/email
  worker remains future operations work.

## API

| Method | Route | Purpose |
| --- | --- | --- |
| `GET`/`POST` | `/api/marketplace/listings/{listing_id}/reviews` | Read published/current-org reviews or submit a customer review. |
| `GET` | `/api/marketplace/reviews` | Read the pending customer-review queue (global admin). |
| `PATCH` | `/api/marketplace/reviews/{review_id}/moderation` | Publish or reject a customer review (global admin). |
| `POST` | `/api/marketplace/listings/{listing_id}/reports` | Submit a product abuse report. |
| `GET` | `/api/marketplace/reports` | Read the severity-prioritized abuse moderation queue (global admin). |
| `PATCH` | `/api/marketplace/reports/{report_id}` | Investigate, resolve, or dismiss a report (global admin). |

## Deliberate boundaries

- A review does not alter a listing's approval, risk, installability, or payout.
- Uploaded package code is still never executed.
- Notification delivery, automatic takedown, evidence file uploads, and appeal
  workflow are deferred; the moderation queue makes those later additions
  auditable rather than silent.
