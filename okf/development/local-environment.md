---
okf_document_id: "development-local-environment"
title: "Local Environment"
project: "ZinharCMS"
category: "development"
phase: 10
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-19"
primary_sources:
  - "README.md"
  - "package.json"
  - "docker-compose.yml"
  - ".env.example"
  - "backend/src/main.rs"
  - "frontend/vite.config.ts"
related_documents:
  - "prerequisites.md"
  - "commands.md"
  - "database-development.md"
  - "../operations/health-and-readiness.md"
  - "../security/secrets-and-configuration.md"
related_diagrams: []
---

# Local Environment

## Verified Setup Sequence

| Step | Working directory | Command or action | Prerequisites | Expected result | Failure indicators | Related configuration | Security notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Checkout | Parent of repository | Repository-specific clone command is not defined | Git and repository access | Working tree contains manifests | Missing files or inaccessible remote | Repository URL in OKF metadata | Do not embed access tokens in clone URLs |
| Create environment file | Repository root | `Copy-Item .env.example .env` (PowerShell, documented) | Tracked template | Ignored root `.env` exists | Missing template or denied write | `.env.example`, `.gitignore` | Replace placeholders locally; never commit `.env` |
| Install frontend dependencies | `frontend/` | `npm install` | Node and npm | `node_modules` populated; lock may be evaluated | npm nonzero exit | `frontend/package.json`, `frontend/package-lock.json` | Dependency lifecycle scripts execute with developer privileges |
| Start infrastructure | Repository root | `docker compose up -d postgres redis pgadmin` | Docker and Compose | PostgreSQL, Redis, pgAdmin containers start | Engine unavailable, port conflict, unhealthy service | `docker-compose.yml` | Development credentials in Compose are not production credentials |
| Create database | Repository root | No separate command: the PostgreSQL container initializes `cms_dev` from environment | Empty/new named volume | Database and user created by image entrypoint | PostgreSQL health check fails | `docker-compose.yml` | Do not reuse development values outside local/CI contexts |
| Apply migrations | Backend startup | No separate local command: `cargo run` invokes embedded SQLx migrations | Reachable PostgreSQL and valid `DATABASE_URL` | `_sqlx_migrations` advances in order | Backend exits with migration context error | `backend/src/db/mod.rs`, `backend/src/main.rs` | Startup mutates schema; review migrations first |
| Seed initial data | Backend startup | No separate command: startup seeds only when the users table is empty | Successful migrations | Bootstrap records are inserted once | Backend exits with seed context error | `backend/src/main.rs` | Source contains deterministic development bootstrap credentials; do not reproduce them and require owner review (`POTENTIAL_SECRET_EXPOSURE PSE-01`) |
| Start backend | Repository root | `npm run dev:backend` | Environment, PostgreSQL, Redis URL | API binds configured `PORT`, default `8080` | Configuration, migration, seed, bind, or dependency errors | `package.json`, `backend/src/config.rs` | Avoid printing secret values while diagnosing configuration |
| Start frontend | Repository root | `npm run dev:frontend` | Installed frontend dependencies | Vite listens on `0.0.0.0:5173` | npm/Vite build or port failure | `package.json`, `frontend/vite.config.ts` | `VITE_*` values are browser-visible build inputs |
| Verify liveness | Any | Request `http://localhost:8080/health` | Running backend | HTTP 200 with status and version | Connection failure or non-200 | Backend default port and health route | Public endpoint; response contains no credential |
| Verify readiness | Any | Request `http://localhost:8080/ready` | Running backend, PostgreSQL, Redis | HTTP 200; both dependencies reachable | HTTP 503 or dependency detail | `backend/src/routes/mod.rs` | Error text may reveal technical details; do not publish raw production output |

## Local URLs Defined by Repository Evidence

| Surface | URL | Evidence |
| --- | --- | --- |
| PostgreSQL | `localhost:5432` | `docker-compose.yml` |
| Redis | `localhost:6379` | `docker-compose.yml` |
| pgAdmin | `http://localhost:5050` | `docker-compose.yml`, README |
| API | `http://localhost:8080` | `PORT` default and README |
| Admin UI | `http://localhost:5173` | Vite config and README |

## Root `dev` Script Limitation

`npm run dev` executes `docker compose up --build`. The local Compose file contains only PostgreSQL, Redis, and pgAdmin; it does not define backend or frontend services. Therefore this root script starts the infrastructure in the foreground, not the complete application (`DOCUMENTATION_CODE_CONFLICT` risk if described as a full stack command).

## Shutdown and Reset

The backend handles Ctrl+C and Unix terminate signals with graceful Axum shutdown. No repository script defines complete local shutdown or reset. Docker volume deletion, database reset, and generated-file cleanup are intentionally not prescribed here because they are destructive and no repository-owned safe reset workflow exists. Stop processes through their owning terminal/container tool and preserve named volumes unless the owner explicitly authorizes data removal.

See [Database Development](database-development.md), [Health and Readiness](../operations/health-and-readiness.md), and [Secrets and Configuration](../security/secrets-and-configuration.md).

