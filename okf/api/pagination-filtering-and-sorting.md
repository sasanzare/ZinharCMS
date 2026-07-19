---
okf_document_id: "api-pagination-filtering-sorting"
title: "API Pagination, Filtering, and Sorting"
project: "ZinharCMS"
category: "api"
phase: 6
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "eed1e0dbdf6d873457d1165158b3c8fbfd6647e1"
last_verified_date: "2026-07-18"
primary_sources:
  - "backend/src/routes/content.rs"
  - "backend/src/routes/pages.rs"
  - "backend/src/routes/media.rs"
  - "backend/src/routes/delivery.rs"
  - "backend/src/routes/beta.rs"
  - "backend/src/routes/organizations.rs"
  - "backend/src/routes/webhooks.rs"
related_documents:
  - "api/request-contracts.md"
  - "api/endpoints/content-entries-and-workflow.md"
  - "api/endpoints/public-delivery.md"
uncertainty_markers:
  - "PAGINATION_BEHAVIOR_UNCLEAR PBU-01"
  - "FILTERING_BEHAVIOR_UNCLEAR FBU-01"
---

# API Pagination, Filtering, and Sorting

## No Global Pagination Contract

Pagination is implemented independently by route families. There is no shared cursor, total-count field, link header, or common query/response trait. Callers must use each family contract.

## Page-Based Families

| Family | Query fields | Defaults and bounds | Response |
| --- | --- | --- | --- |
| Content entries | `page`, `per_page`, `status`, `sort` | page 1 minimum; per-page default 20, range 1–100 | `{ data, page, per_page }` |
| Pages | `page`, `per_page`, `status`, `sort` | page 1 minimum; per-page default 20, range 1–100 | `{ data, page, per_page }` |
| Media | `page`, `per_page`, `mime_type` | page 1 minimum; per-page default 20, range 1–100 | `{ data, page, per_page }` |
| Public content/pages | `page`, `per_page`, `sort`, `locale`, `author_id`, `filter` | page 1 minimum; per-page default 20, range 1–100 | `{ data, page, per_page }` |

None of these wrappers includes `total`, `total_pages`, `next`, or `previous`. A short page is the only built-in end signal.

## Sorting

The syntax is `field[:direction]`; direction defaults to `desc` and accepts `asc` or `desc` case-insensitively.

- Content entries: `created_at`, `updated_at`, `published_at`; default `created_at:desc`.
- Pages: the same fields plus `title`; default `created_at:desc`.
- Public delivery: the same fields plus `title`; default `created_at:desc`. Content and page queries map permitted fields to their respective SQL columns.

Unsupported fields or directions return 422 validation errors.

## Filtering

- Content entry and page lists accept workflow `status` values validated by their domain status rules.
- Media accepts an exact `mime_type` match.
- Public delivery accepts `locale`, `author_id`, and one JSON-field filter encoded as `field=value` or `field:value`. The field must use lowercase letters, digits, and underscores and start with a lowercase letter or underscore; the value must be non-empty.
- Public locale is trimmed, 2–8 ASCII alphabetic or hyphen characters, or omitted when blank.
- Marketplace catalog filters by `search`, `category`, `product_type`, and `pricing_type` but does not expose page parameters in `CatalogQuery`.

## Limit-Only Families

- Beta feedback and blockers: `limit`, default 50, clamped 1–100.
- Organization audit log: `limit`, default 50, clamped 1–100.
- Organization email deliveries: `limit`, default 25, clamped 1–100.
- CMS webhook deliveries: `limit`, default 20, clamped 1–100.

These responses are raw arrays and do not return the effective limit or a continuation token.

## Gaps

Some potentially large lists have no exposed pagination. `PAGINATION_BEHAVIOR_UNCLEAR PBU-01` and `FILTERING_BEHAVIOR_UNCLEAR FBU-01` should be used before claiming stable behavior for undocumented families. Any standardization would be an implementation decision outside Phase 6.
