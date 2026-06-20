# Phase Five Implementation

Phase five goal: public delivery APIs, cache invalidation, SEO endpoints, and webhook delivery.

## Delivered

- Public Delivery API under `/api/v1` without admin authentication.
- Published content endpoints for collection and UUID/data-slug detail lookups.
- Published page endpoints with extracted page metadata for SEO consumers.
- Public settings and navigation endpoints backed by PostgreSQL seed data.
- Redis JSON response cache with a 5 minute TTL and PostgreSQL fallback when Redis is unavailable.
- Cache invalidation on page and entry publish/unpublish, plus published entry/page update/delete paths.
- `sitemap.xml` and `robots.txt` generated from published pages and entries with a `slug` data field.
- Admin webhook CRUD, delivery log listing, and test sending.
- HMAC-SHA256 webhook signatures via `X-CMS-Signature` and event names via `X-CMS-Event`.

## Public Endpoints

| Method | Path | Notes |
| --- | --- | --- |
| `GET` | `/api/v1/content/{type_slug}` | Published entries only; supports `page`, `per_page`, `sort`, `locale`, `author_id`, `filter=field=value`. |
| `GET` | `/api/v1/content/{type_slug}/{id_or_slug}` | Published entry by UUID or `data.slug`. |
| `GET` | `/api/v1/pages` | Published pages only. |
| `GET` | `/api/v1/pages/{slug}` | Published page by slug. |
| `GET` | `/api/v1/settings/public` | Public key/value settings. |
| `GET` | `/api/v1/navigation?locale=fa` | Public navigation items. |
| `GET` | `/api/v1/sitemap.xml` | SEO sitemap. |
| `GET` | `/api/v1/robots.txt` | SEO robots response. |

## Webhook Events

- `entry.publish`
- `entry.unpublish`
- `page.publish`
- `page.unpublish`