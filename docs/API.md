# API

The API exposes a functional backend for authentication, RBAC-protected content
management, entries, the media library, visual page builder, delivery API,
workflow, comments, plugins, and webhooks.

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
| `GET` | `/api/auth` | Auth module planned endpoints |
| `POST` | `/api/auth/register` | Create user; first user becomes `super_admin` |
| `POST` | `/api/auth/login` | Issue access and refresh tokens |
| `POST` | `/api/auth/refresh` | Rotate refresh token and issue a new access token |
| `POST` | `/api/auth/logout` | Revoke refresh token |
| `GET` | `/api/auth/me` | Current authenticated user |

Use the access token as `Authorization: Bearer <token>` for protected endpoints.

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
