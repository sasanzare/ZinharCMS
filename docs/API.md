# API

Phase zero exposes system and module-status endpoints. Functional CRUD endpoints
are intentionally deferred to later phases, but their modules and OpenAPI surface
are already in place.

## System

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/` | API metadata |
| `GET` | `/health` | Liveness check |
| `GET` | `/ready` | PostgreSQL and Redis readiness |
| `GET` | `/openapi.json` | OpenAPI specification |

## Module Status

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/api/auth` | Auth module planned endpoints |
| `GET` | `/api/content` | Content module planned resources |
| `GET` | `/api/media` | Media module planned features |
| `GET` | `/api/pages` | Page module planned features |

## Planned Phase-One Auth Endpoints

| Method | Path |
| --- | --- |
| `POST` | `/api/auth/register` |
| `POST` | `/api/auth/login` |
| `POST` | `/api/auth/refresh` |
| `POST` | `/api/auth/logout` |
| `GET` | `/api/auth/me` |

## Planned Content Endpoints

| Method | Path |
| --- | --- |
| `GET` | `/api/content-types` |
| `POST` | `/api/content-types` |
| `GET` | `/api/content-types/:id` |
| `PUT` | `/api/content-types/:id` |
| `DELETE` | `/api/content-types/:id` |
| `GET` | `/api/entries/:type_slug` |
| `POST` | `/api/entries/:type_slug` |
| `GET` | `/api/entries/:type_slug/:id` |
| `PUT` | `/api/entries/:type_slug/:id` |
| `DELETE` | `/api/entries/:type_slug/:id` |
| `POST` | `/api/entries/:type_slug/:id/publish` |
