# ZangarCMS

ZangarCMS is a Rust/Axum + React headless CMS with a visual page builder roadmap.
This repository currently implements phase zero from the project proposal: a runnable
monorepo foundation, local infrastructure, CI, environment configuration, and the
initial PostgreSQL schema.

## Phase Zero Scope

- `backend/`: Rust 2024 + Axum 0.8 API scaffold with health/readiness endpoints.
- `frontend/`: React 19 + Vite 6 admin workspace scaffold.
- `docker-compose.yml`: PostgreSQL 16, Redis 7, pgAdmin, backend, and frontend.
- `.github/workflows/`: initial backend and frontend CI.
- `backend/migrations/`: database schema based on the proposal ERD.
- `docs/`: architecture, API, and phase-zero notes.

## Quick Start

Copy the environment template and start the local stack:

```powershell
Copy-Item .env.example .env
docker compose up --build
```

Local services:

- Admin UI: http://localhost:5173
- API: http://localhost:8080
- OpenAPI JSON: http://localhost:8080/openapi.json
- pgAdmin: http://localhost:5050
- PostgreSQL: localhost:5432
- Redis: localhost:6379

## Local Development Without Docker

Start the infrastructure:

```powershell
docker compose up -d postgres redis pgadmin
```

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
