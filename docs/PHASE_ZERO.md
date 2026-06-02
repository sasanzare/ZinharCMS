# Phase Zero Implementation

Phase zero goal: a fully runnable development foundation that comes up with one
command and establishes the project contracts for future backend and frontend work.

## Completed Deliverables

- Monorepo structure for backend, frontend, docs, Docker, and CI.
- Rust 2024 + Axum 0.8 backend scaffold.
- React 19 + Vite 6 frontend scaffold.
- PostgreSQL 16 and Redis 7 local infrastructure.
- pgAdmin local database management service.
- PostgreSQL schema and foundation seed migration.
- Environment templates: `.env.example` and `env.example`.
- Backend and frontend Dockerfiles for development and production.
- GitHub Actions workflows for backend and frontend CI.
- Initial architecture and API documentation.

## One-Command Development Stack

```powershell
Copy-Item .env.example .env
docker compose up --build
```

## Environment Defaults

| Variable | Default |
| --- | --- |
| `DATABASE_URL` | `postgresql://cms_user:cms_pass@localhost:5432/cms_dev` |
| `REDIS_URL` | `redis://localhost:6379` |
| `JWT_ACCESS_EXPIRY` | `3600` |
| `JWT_REFRESH_EXPIRY` | `604800` |
| `UPLOAD_DIR` | `./uploads` |
| `MAX_UPLOAD_SIZE` | `52428800` |
| `CORS_ORIGIN` | `http://localhost:5173` |
| `PORT` | `8080` |
| `VITE_API_URL` | `http://localhost:8080` |

## Quality Gates

- Backend: `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-features`.
- Frontend: `npm run lint`, `npm run typecheck`, `npm test`, `npm run build`.
