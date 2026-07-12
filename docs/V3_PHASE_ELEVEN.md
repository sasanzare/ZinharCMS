# V3 Marketplace Phase 11: Analytics

Phase 11 implements the two proposal subphases using existing Marketplace
records. No new telemetry table is introduced in this phase; the analytics APIs
aggregate installs, purchases, revenue ledger entries, reviews, reports, package
validation state, and review events already persisted by phases 1 through 10.

## 11.1 Creator analytics

- `GET /api/marketplace/creators/{creator_id}/analytics` returns creator-owned
  product performance.
- The endpoint uses a bypass transaction only after verifying
  `marketplace_creators.user_id = claims.sub`, so only the creator owner can view
  analytics for that profile.
- The response includes listing count, total installs, active installs, purchase
  attempts, completed purchases, refunds, gross revenue, creator revenue,
  conversion rate, error count, and per-product breakdowns.
- Conversion is calculated as completed purchases divided by all purchase
  attempts, bounded between zero and one.
- Error count is a persisted-signal count from blocked/failed/high-risk versions,
  failed purchase attempts, and abuse reports. Runtime exception telemetry is not
  counted because arbitrary package execution and runtime error ingestion are not
  implemented.

## 11.2 Marketplace admin analytics

- `GET /api/marketplace/analytics/admin` returns internal Marketplace health
  analytics for global admins and super admins.
- The response includes 30-day submission count/rate, average approval time,
  total/active installs, refunds, reports, critical reports, blocked packages,
  and a ranked risky-products list.
- Risky products are ranked from persisted operational signals: critical reports,
  blocked/failed/high-risk versions, refunds, failed purchase attempts, and active
  install exposure.
- Submission rate, approval time, installs, refunds, reports, and blocked packages
  are the core admin health metrics.
- The admin UI displays the health metrics and the risky/repetitive product list
  in the Marketplace page for global admins.

## API

| Method | Route | Purpose |
| --- | --- | --- |
| `GET` | `/api/marketplace/creators/{creator_id}/analytics` | Creator-owned installs, revenue, conversion, ratings, reports, and error signals. |
| `GET` | `/api/marketplace/analytics/admin` | Internal Marketplace submission, approval, install, refund, report, and blocked-package signals. |

## Data sources

| Metric area | Source |
| --- | --- |
| Installs | `marketplace_installations` |
| Purchase attempts and refunds | `marketplace_purchases` |
| Gross and creator revenue | `marketplace_revenue_ledger` |
| Ratings | `marketplace_product_reviews` with `status = published` |
| Abuse reports | `marketplace_abuse_reports` |
| Package errors and blocked packages | `marketplace_versions.validation_status`, `marketplace_versions.security_risk_level`, and `marketplace_versions.status` |
| Submission rate and approval time | `marketplace_submissions` and `marketplace_review_events` |

## Deliberate boundaries

- Analytics are read-only projections; they do not mutate listings, payouts,
  installation state, or moderation state.
- Runtime execution errors are not collected because uploaded Marketplace code is
  still never executed.
- External warehouse/export, custom dashboards, time-series retention policy, and
  anomaly alerting remain later operations work.
