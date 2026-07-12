# API

The API exposes authentication, organization tenancy, RBAC-protected content
management, media, the visual Page Builder, Delivery API, workflow, plugins,
webhooks, billing/quota operations, beta/GA records, and Marketplace submission,
review, moderation, and catalog behavior.

## System

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/` | API metadata |
| `GET` | `/health` | Liveness check |
| `GET` | `/ready` | PostgreSQL and Redis readiness |
| `GET` | `/openapi.json` | OpenAPI specification |

## Auth

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/api/auth` | Auth module status and endpoint list |
| `POST` | `/api/auth/register` | Create user; first user becomes `super_admin` |
| `POST` | `/api/auth/login` | Issue access token and HttpOnly refresh cookie; rate-limited by failed IP attempts |
| `POST` | `/api/auth/refresh` | Rotate refresh cookie and issue a new access token |
| `POST` | `/api/auth/logout` | Revoke refresh token and clear refresh cookie |
| `GET` | `/api/auth/me` | Current authenticated user |

Use the access token as `Authorization: Bearer <token>` for protected endpoints. Refresh tokens are issued as the `zinhar_refresh_token` HttpOnly cookie; legacy clients may still send `refresh_token` in the refresh/logout JSON body.

## Authentication And Tenant Boundaries

- Authentication-only routes require `Authorization: Bearer <token>`.
- Tenant-aware routes additionally require `X-Organization-Id`.
- Preview WebSocket clients may send `access_token` and `organization_id` query
  parameters because browser WebSocket APIs cannot set arbitrary headers.
- Tenant middleware requires an active organization and active membership, then
  applies organization/user rate limits and API quota checks.
- All current `/api/marketplace/*` routes are tenant-aware. The catalog is not an
  anonymous public endpoint in the current router composition.

## Content Types

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/api/content-types` | List content types |
| `POST` | `/api/content-types` | Create content type; admin-only |
| `GET` | `/api/content-types/{id}` | Get content type |
| `PUT` | `/api/content-types/{id}` | Update content type; admin-only |
| `DELETE` | `/api/content-types/{id}?confirm=true` | Delete content type; admin-only |

## Entries

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/api/entries/{type_slug}` | List entries with `page`, `per_page`, `status`, `sort` |
| `POST` | `/api/entries/{type_slug}` | Create entry and validate against field schema |
| `GET` | `/api/entries/{type_slug}/{id}` | Get entry |
| `PUT` | `/api/entries/{type_slug}/{id}` | Update entry and increment version |
| `DELETE` | `/api/entries/{type_slug}/{id}` | Delete entry; admin/editor |
| `POST` | `/api/entries/{type_slug}/{id}/submit-review` | Move draft entry to pending review |
| `POST` | `/api/entries/{type_slug}/{id}/publish` | Publish entry; admin/editor |
| `POST` | `/api/entries/{type_slug}/{id}/unpublish` | Move entry back to draft; admin/editor |
| `POST` | `/api/entries/{type_slug}/{id}/reject` | Reject pending entry back to draft; admin/editor |
| `POST` | `/api/entries/{type_slug}/{id}/archive` | Archive published entry; admin/editor |
| `POST` | `/api/entries/{type_slug}/{id}/restore` | Restore archived entry to draft; admin/editor |

Supported sort fields: `created_at`, `updated_at`, `published_at`.
Example: `sort=created_at:desc`.

## Media

| Method | Path | Purpose |
| --- | --- | --- |
| `POST` | `/api/media/upload` | Multipart upload with `file`, optional `alt_text`, optional `caption` |
| `GET` | `/api/media` | List media with optional `mime_type`, `page`, `per_page` |
| `GET` | `/api/media/{id}` | Get media and variants |
| `PUT` | `/api/media/{id}` | Update `alt_text` and `caption` |
| `DELETE` | `/api/media/{id}` | Delete media and variants; admin/editor |

Image uploads for `image/jpeg`, `image/png`, and `image/webp` generate:
`thumbnail`, `small`, `medium`, and `large` WebP variants.

Stored file bytes are served from `GET /uploads/{organization_id}/...` without auth
or tenant middleware. Media metadata APIs remain tenant-protected.

## Component Registry

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/api/component-registry` | List components with optional `category` filter |
| `POST` | `/api/component-registry` | Create custom component; admin-only |
| `GET` | `/api/component-registry/{component_key}` | Get component definition |
| `PUT` | `/api/component-registry/{component_key}` | Update component definition; admin-only |
| `DELETE` | `/api/component-registry/{component_key}?confirm=true` | Delete custom component; admin-only |

System components are seeded for sections, content, layout, media, forms,
navigation, and data categories. Page JSON node `type` values must match a
registered `component_key` such as `hero-banner`.

## Pages

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/api/pages` | List pages with `page`, `per_page`, `status`, `sort` |
| `POST` | `/api/pages` | Create page and first version snapshot |
| `GET` | `/api/pages/{id}` | Get page by UUID |
| `GET` | `/api/pages/slug/{slug}` | Get page by slug |
| `PUT` | `/api/pages/{id}` | Update page JSON and create a new version snapshot |
| `DELETE` | `/api/pages/{id}?confirm=true` | Delete page; editor/admin |
| `POST` | `/api/pages/{id}/submit-review` | Move draft page to pending review |
| `POST` | `/api/pages/{id}/publish` | Publish page; editor/admin |
| `POST` | `/api/pages/{id}/unpublish` | Move page back to draft; editor/admin |
| `POST` | `/api/pages/{id}/reject` | Reject pending page back to draft; editor/admin |
| `POST` | `/api/pages/{id}/archive` | Archive published page; editor/admin |
| `POST` | `/api/pages/{id}/restore` | Restore archived page to draft; editor/admin |
| `GET` | `/api/pages/{id}/versions` | List page JSON snapshots |
| `POST` | `/api/pages/{id}/versions/{version}/restore` | Restore snapshot as a new draft version |
| `GET` | `/api/preview/{page_id}` | Authenticated WebSocket live preview stream; use `Authorization` header or `?access_token=` |

Supported page sort fields: `created_at`, `updated_at`, `published_at`, `title`.
Example: `sort=updated_at:desc`.

Page JSON requires a root layout node:

```json
{
  "version": "1.0",
  "metadata": {
    "title": "Home",
    "description": "Landing page"
  },
  "layout": {
    "id": "root",
    "type": "root",
    "children": []
  }
}
```

## Delivery API

Public endpoints do not require `Authorization` and only expose published records.

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/api/v1/content/{type_slug}` | List published entries with `page`, `per_page`, `sort`, `locale`, `author_id`, `filter=field=value` |
| `GET` | `/api/v1/content/{type_slug}/{id_or_slug}` | Get a published entry by UUID or `data.slug` |
| `GET` | `/api/v1/pages` | List published pages |
| `GET` | `/api/v1/pages/{slug}` | Get a published page by slug |
| `GET` | `/api/v1/settings/public` | Public settings map |
| `GET` | `/api/v1/navigation?locale=fa` | Public navigation items |
| `GET` | `/api/v1/sitemap.xml` | XML sitemap from published pages and entries with `data.slug` |
| `GET` | `/api/v1/robots.txt` | Robots response pointing to the sitemap |

Responses are cached in Redis for 300 seconds. Publish/unpublish and published update/delete operations invalidate related delivery keys.

## Comments

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/api/comments?entity_type=entry&entity_id={uuid}` | List unresolved comments for an entry or page |
| `GET` | `/api/comments?entity_type=page&entity_id={uuid}&include_resolved=true` | List comments including resolved threads |
| `POST` | `/api/comments` | Create comment for an entry or page |
| `GET` | `/api/comments/{id}` | Get comment |
| `POST` | `/api/comments/{id}/resolve` | Resolve comment |
| `DELETE` | `/api/comments/{id}/resolve` | Reopen comment |
| `DELETE` | `/api/comments/{id}` | Delete comment; editor/admin |

## Plugins

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/api/plugins` | List built-in CMS plugins |
| `GET` | `/api/plugins/{plugin_key}` | Get plugin details |
| `PUT` | `/api/plugins/{plugin_key}` | Update plugin enabled state; admin-only |
| `POST` | `/api/plugins/{plugin_key}/enable` | Enable plugin; admin-only |
| `POST` | `/api/plugins/{plugin_key}/disable` | Disable plugin; admin-only |

The `seo-auto` plugin runs on `entry.before_save` and fills `data.slug` from `data.title` when `data.slug` is empty.

## Security Controls

| Area | Control |
| --- | --- |
| Auth | Failed login attempts are limited by IP address. Default: 5 failures per 15 minutes. |
| Auth | Refresh tokens are stored in `HttpOnly` cookies at `/api/auth`. |
| API | CORS is restricted to `CORS_ORIGIN` and supports credentialed requests. |
| API | Responses include CSP, frame, content-type, referrer, and permissions policy headers. |
| Content | Entry `richtext` fields are sanitized before validation and storage. |
| Webhooks | Webhook URLs reject credentials, localhost, private IP ranges, and metadata hosts. |
| Uploads | File type is detected from content signatures and must match the declared MIME type. |

## Webhooks

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/api/webhooks` | List webhook subscriptions; admin-only |
| `POST` | `/api/webhooks` | Create webhook subscription; admin-only |
| `GET` | `/api/webhooks/{id}` | Get webhook subscription; admin-only |
| `PUT` | `/api/webhooks/{id}` | Update webhook subscription; admin-only |
| `DELETE` | `/api/webhooks/{id}?confirm=true` | Delete webhook subscription; admin-only |
| `GET` | `/api/webhooks/{id}/deliveries` | Recent delivery attempts |
| `POST` | `/api/webhooks/{id}/test` | Send a signed test payload |

Supported events: `entry.publish`, `entry.unpublish`, `page.publish`, `page.unpublish`.
Webhook requests include `X-CMS-Event` and `X-CMS-Signature` headers.

Webhook delivery uses one transient asynchronous attempt per event/subscription.
Delivery rows are persisted, but no durable retry queue or worker is implemented.

## Organizations

The following routes require authentication but do not require an active
organization header:

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/api/organizations` | List current user memberships |
| `POST` | `/api/organizations` | Create an organization and owner membership |
| `POST` | `/api/organization-invitations/accept` | Accept a pending invitation token |

The following routes require `X-Organization-Id`:

| Method | Path | Purpose |
| --- | --- | --- |
| `GET`, `PUT` | `/api/organizations/current` | Read or update current organization |
| `GET` | `/api/organizations/current/members` | List members |
| `PATCH`, `DELETE` | `/api/organizations/current/members/{user_id}` | Change role or remove member |
| `GET`, `POST` | `/api/organizations/current/invitations` | List or create invitations |
| `DELETE` | `/api/organizations/current/invitations/{invitation_id}` | Revoke invitation |
| `GET` | `/api/organizations/current/workspace` | Resolve workspace URL/slug |
| `GET`, `POST` | `/api/organizations/current/domains` | List or create domain metadata |
| `DELETE` | `/api/organizations/current/domains/{domain_id}` | Delete domain metadata |
| `GET`, `PUT` | `/api/organizations/current/rate-limit` | Read or update rate limits |
| `GET` | `/api/organizations/current/audit-logs` | List tenant audit records |
| `GET` | `/api/organizations/current/email-deliveries` | List email delivery records |
| `GET` | `/api/organizations/current/alerts` | List seeded SaaS alert definitions |
| `POST` | `/api/organizations/current/leave` | Leave current organization |
| `POST` | `/api/organizations/current/transfer-ownership` | Transfer organization ownership |

Organization roles are `owner`, `admin`, `editor`, `author`, `viewer`, and
`billing_manager`. They are distinct from global user roles.

## Billing And Quotas

All routes except the Stripe webhook require tenant context.

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/api/billing/plans` | List active plans and Stripe checkout availability |
| `GET`, `PUT` | `/api/billing/subscription` | Read or manually change organization plan |
| `POST` | `/api/billing/checkout` | Create Stripe subscription checkout session |
| `POST` | `/api/billing/portal` | Create Stripe customer portal session |
| `GET` | `/api/billing/usage` | Read current plan, usage, and quota state |
| `POST` | `/api/billing/usage/rebuild` | Rebuild members/content/media counters |
| `POST` | `/api/billing/stripe/webhook` | Public signed Stripe webhook endpoint |

Stripe webhook processing verifies the signature, deduplicates provider event IDs,
and ignores timestamped subscription events older than the current local event.
Stripe billing applies to organization subscriptions, not Marketplace purchases or
creator payouts.

## Beta And GA Operations

Tenant-aware beta routes:

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/api/beta/dashboard` | Current organization beta dashboard |
| `GET`, `POST` | `/api/beta/feedback` | List or create feedback |
| `PATCH` | `/api/beta/feedback/{feedback_id}` | Update feedback status/severity |
| `GET`, `POST` | `/api/beta/ga-blockers` | List or create GA blockers |
| `PATCH` | `/api/beta/ga-blockers/{blocker_id}` | Update blocker status/ownership |

Authentication-only global administration routes:

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/api/beta/product-dashboard` | Cross-organization beta dashboard |
| `PUT` | `/api/beta/participants/{organization_id}` | Upsert beta participant state |

GA release readiness itself is implemented through documentation, static tests,
and `scripts/v2-ga-check.ps1`; there is no GA readiness runtime endpoint.

## Marketplace

Every Marketplace route currently requires authentication and
`X-Organization-Id`.

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/api/marketplace/catalog` | Search/filter approved compatible catalog items |
| `GET` | `/api/marketplace/catalog/{listing_slug}` | Read compatible listing detail and versions |
| `GET`, `POST` | `/api/marketplace/listings/{listing_id}/reviews` | Read published/current-organization customer reviews or submit a gated review |
| `GET` | `/api/marketplace/reviews` | Read pending customer reviews for moderation (global admin) |
| `PATCH` | `/api/marketplace/reviews/{review_id}/moderation` | Publish or reject a customer review (global admin) |
| `POST` | `/api/marketplace/listings/{listing_id}/reports` | Submit a Marketplace abuse report with evidence |
| `GET` | `/api/marketplace/reports` | Read severity-prioritized abuse reports (global admin) |
| `PATCH` | `/api/marketplace/reports/{report_id}` | Investigate, resolve, or dismiss an abuse report (global admin) |
| `GET` | `/api/marketplace/creators/{creator_id}/analytics` | Read creator-owned product installs, revenue, conversion, ratings, reports, and error signals |
| `GET` | `/api/marketplace/analytics/admin` | Read internal Marketplace health and risky-product analytics (global admin) |
| `GET`, `POST` | `/api/marketplace/installations` | List current installs or install an approved free product |
| `GET` | `/api/marketplace/installations/{installation_id}/updates` | Check for a newer compatible approved version |
| `POST` | `/api/marketplace/installations/{installation_id}/enable` | Re-enable a safe disabled installation |
| `POST` | `/api/marketplace/installations/{installation_id}/disable` | Disable an active installation |
| `POST` | `/api/marketplace/installations/{installation_id}/uninstall` | Soft-uninstall while preserving organization data |
| `POST` | `/api/marketplace/installations/{installation_id}/update` | Confirm changelog and update to a newer pinned version |
| `POST` | `/api/marketplace/installations/{installation_id}/rollback` | Restore the safe compatible rollback version |
| `GET` | `/api/marketplace/permissions` | Read the enabled Marketplace permission catalog |
| `GET` | `/api/marketplace/runtime/status` | Read global/organization runtime safety and kill-switch status |
| `POST` | `/api/marketplace/installations/{installation_id}/runtime/authorize` | Return an allowlisted sandbox host API decision without executing package code |
| `POST` | `/api/marketplace/kill-switches/organization` | Activate an organization runtime kill switch; owner/admin |
| `POST` | `/api/marketplace/kill-switches/global` | Activate a global runtime kill switch; global admin |
| `POST` | `/api/marketplace/kill-switches/{kill_switch_id}/lift` | Lift an authorized organization or global kill switch |
| `GET` | `/api/marketplace/runtime/components` | Materialize active Component Pack definitions for the organization Page Builder palette |
| `POST` | `/api/marketplace/templates/{installation_id}/preview` | Preview a Design Template with tenant-owned asset mapping |
| `POST` | `/api/marketplace/templates/{installation_id}/import` | Clone a Design Template into a new organization page and version snapshot |
| `GET` | `/api/marketplace/hooks` | List public hooks declared by active Integration Plugins |
| `POST` | `/api/marketplace/hooks/{hook_type}/authorize` | Authorize a public hook contract without executing package code |
| `GET`, `POST` | `/api/marketplace/creator` | Read/request current user creator profile |
| `PATCH` | `/api/marketplace/creators/{creator_id}/verification` | Global-admin creator verification |
| `GET`, `POST` | `/api/marketplace/listings` | List creator listings or create draft |
| `PUT` | `/api/marketplace/listings/{listing_id}` | Update editable creator listing |
| `POST` | `/api/marketplace/listings/{listing_id}/submit` | Submit complete listing metadata |
| `POST` | `/api/marketplace/listings/{listing_id}/versions/upload` | Upload ZIP plus manifest and run validation |
| `GET` | `/api/marketplace/listings/{listing_id}/submissions` | List creator submission reports |
| `GET` | `/api/marketplace/review/queue` | Global-admin review queue |
| `GET` | `/api/marketplace/review/events` | Global-admin review/moderation event history |
| `GET` | `/api/marketplace/review/reports` | Global-admin validation report list |
| `PATCH` | `/api/marketplace/review/submissions/{submission_id}` | Approve, reject, or request changes |
| `POST` | `/api/marketplace/review/listings/{listing_id}/moderation` | Suspend, unpublish version, or emergency block |

Packages are stored on the local filesystem under `UPLOAD_DIR`. Validation,
security, and compatibility reports are stored on `marketplace_versions` and
`marketplace_submissions`.

Phase 6 implements tenant-aware install, enable, disable, soft-uninstall, update
check, pinned update, and rollback endpoints for Component Packs and Design
Templates. Mutations require organization owner/admin, exact permission approval,
current compatibility, safe review state, and artifact size/SHA integrity. Phase 9
adds active-entitlement enforcement for paid product lifecycle operations. Custom
pricing remains unsupported.

Phase 6 installation paths and schemas are registered in generated `/openapi.json`.
Earlier Marketplace creator, submission, review, moderation, and catalog paths
remain documented manually until their legacy handlers gain OpenAPI annotations.

Phase 7 adds the permission catalog, allowlisted runtime authorization decision,
runtime status, and global/organization kill-switch paths to generated OpenAPI.
The authorization endpoint is a policy decision only: uploaded package code is
not executed by the backend.

Phase 8 adds the Component Pack registry, Template preview/import, and public
Plugin Hook authorization paths to generated OpenAPI. Template imports verify
organization-owned media mappings and create independent page/version records.

Phase 9 adds these generated OpenAPI paths:

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/api/marketplace/purchases` | List organization purchases and receipts |
| `POST` | `/api/marketplace/purchases/checkout` | Grant a free entitlement or create paid Stripe Checkout |
| `GET` | `/api/marketplace/revenue-ledger` | List tenant-scoped purchase/refund financial effects |
| `GET/POST` | `/api/marketplace/creators/{creator_id}/payout` | Read or start creator-owned payout onboarding |
| `POST` | `/api/marketplace/creators/{creator_id}/payout/verify` | Record provider-attested payout state as global admin |

The existing signed `/api/billing/webhook` endpoint processes both subscription
events and metadata-scoped Marketplace payment/refund events while persisting
their effects in separate domain tables.

Phase 11 adds generated OpenAPI paths for creator-owned analytics and
global-admin Marketplace health analytics. These are read-only aggregate
projections over existing Marketplace install, purchase, ledger, review, report,
version, submission, and review-event tables.

Phase 12 does not add backend routes. It adds creator-side tooling in
`scripts/marketplace-cli.mjs` for validating, packing, and submitting packages
to the existing `POST /api/marketplace/listings/{listing_id}/versions/upload`
multipart API. The CLI sends the same `manifest` and `file` fields used by the
admin UI; backend validation and review remain authoritative.
