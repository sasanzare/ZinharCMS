# ZinharCMS

ZinharCMS is a Rust/Axum + React headless CMS with a visual page builder roadmap.
This repository currently implements phase zero and the phase-one backend core from
the project proposal: a runnable monorepo foundation, local infrastructure, CI,
environment configuration, the initial PostgreSQL schema, auth, RBAC, content type
CRUD, entry CRUD, and media library APIs.

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
