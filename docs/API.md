# API

Phase one exposes a functional backend API for authentication, RBAC-protected
content management, entries, and the media library.

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
| `POST` | `/api/entries/{type_slug}/{id}/publish` | Publish entry; admin/editor |
| `POST` | `/api/entries/{type_slug}/{id}/unpublish` | Move entry back to draft; admin/editor |

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
