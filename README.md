# ZinharCMS

ZinharCMS is a Rust/Axum + React headless CMS with a visual page builder roadmap.
This repository currently implements phase zero, the phase-one backend core, and
the phase-two backend page builder engine from the project proposal: a runnable
monorepo foundation, local infrastructure, CI, environment configuration, auth,
RBAC, content type CRUD, entry CRUD, media library APIs, page JSON storage,
component registry, page versioning, live preview streaming, and a React admin panel for those backend capabilities.

## Phase Zero Scope

- `backend/`: Rust 2024 + Axum 0.8 API scaffold with health/readiness endpoints.
- `backend/src/routes/`: phase-one auth, content, entry, and media APIs.
- `frontend/`: React 19 + Vite 6 admin workspace scaffold.
- `docker-compose.yml`: PostgreSQL 16, Redis 7, and pgAdmin only.
- `.github/workflows/`: initial backend and frontend CI.
- `backend/migrations/`: database schema based on the proposal ERD.
- `docs/`: architecture, API, and phase-zero notes.

## Phase One Scope

- Auth: register, login, refresh, logout, and current-user endpoints.
- Security: Argon2id password hashing and HMAC-SHA256 access tokens.
- RBAC: `super_admin`, `admin`, `editor`, `author`, and `viewer` role checks.
- Content Types: admin-managed field schemas stored in PostgreSQL `JSONB`.
- Entries: CRUD, schema validation, pagination, sorting, publish, and unpublish.
- Media: multipart upload, metadata editing, listing, details, deletion, and image variants.

## Phase Two Scope

- Component Registry: seeded system components plus custom component CRUD.
- Pages: JSON layout CRUD, slug lookup, pagination, status filtering, publish, and unpublish.
- Validation: page metadata, layout tree, registered component types, props/styles objects, and size/depth limits.
- Versions: snapshots on create/update/restore, version history, and restore-to-draft.
- Live Preview: authenticated WebSocket stream at `/api/preview/{page_id}`.

## Phase Three Scope

- Auth UI: login/register, stored session, protected admin routes, and logout.
- Dashboard: backend health plus live CMS counts.
- Content Types: schema list/create/edit/delete with a field builder.
- Entries: dynamic forms generated from content type field schemas, CRUD, publish, and unpublish.
- Media: upload, search, metadata editing, copy URL, and delete.
- Pages: JSON editor for phase-two pages, publish/unpublish, versions, restore, and preview WebSocket URL copy.

## Phase Four Scope

- Page Builder: component palette, drag-and-drop canvas, sortable blocks, and generated props editor.
- Preview: live local preview of the current page JSON plus WebSocket preview URL copy for saved pages.
- Persistence: manual save for new pages and debounced autosave for existing page drafts.
- Compatibility: uses the existing `page_json`, page versions, publish/unpublish, and component registry APIs.

## Phase Five Scope

- Delivery API: public `/api/v1` endpoints for published content, pages, settings, and navigation.
- SEO: sitemap and robots endpoints generated from published pages and entry slugs.
- Cache: Redis-backed delivery responses with publish/update invalidation and PostgreSQL fallback when Redis is unavailable.
- Webhooks: admin-managed subscriptions for entry/page publish and unpublish events with HMAC signatures and delivery logs.
## Quick Start

Copy the environment template and start the local stack:

```powershell
Copy-Item .env.example .env
docker compose up -d postgres redis pgadmin
```

Local services:

- pgAdmin: http://localhost:5050
- PostgreSQL: localhost:5432
- Redis: localhost:6379
- API: http://localhost:8080
- Admin UI: http://localhost:5173

## Local Development Without Docker

Start the infrastructure:

Run the backend:

```powershell
cd backend
cargo run
```

Run the frontend:

```powershell
cd frontend
npm install
npm run dev
```
