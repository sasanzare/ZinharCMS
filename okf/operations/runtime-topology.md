---
okf_document_id: "operations-runtime-topology"
title: "Runtime Topology"
project: "ZinharCMS"
category: "operations"
phase: 10
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-19"
primary_sources:
  - "docker-compose.yml"
  - "docker-compose.prod.yml"
  - "backend/src/main.rs"
  - "backend/src/state.rs"
  - "frontend/Dockerfile.prod"
  - "frontend/nginx.conf"
related_documents:
  - "../architecture/overview.md"
  - "../delivery/deployment-workflow.md"
  - "../database/technology-and-configuration.md"
  - "../security/trust-boundaries.md"
  - "troubleshooting.md"
related_diagrams:
  - "diagrams/runtime-topology.mmd"
---

# Runtime Topology

## Topology Classes

- **Local:** Compose provides PostgreSQL, Redis, and pgAdmin; backend and frontend normally run as separate developer processes.
- **Test/CI:** Backend CI runs one test process with PostgreSQL and Redis service containers; frontend CI runs Node/Vitest/build without backend services.
- **Container:** Development Dockerfiles run Cargo/Vite; production Dockerfiles run a compiled non-root backend and Nginx static frontend.
- **Verified deployment:** None. `docker-compose.prod.yml` is configuration evidence only.
- **Production:** `PRODUCTION_TOPOLOGY_UNKNOWN`; do not infer clusters, load balancers, TLS, replicas, regions, DNS, or autoscaling.

## Runtime Component Matrix

| Component | Process/image | Port/protocol | Dependencies | Storage | Configuration | Startup order | Health signal | Failure effect | Evidence |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Backend API | `cms-backend` Axum/Tokio process | Configured `PORT`, default 8080/HTTP | PostgreSQL for startup/migrations; Redis client; filesystem | PostgreSQL, Redis, upload directory, in-memory preview channels | `Config::from_env` | Migrate, seed, construct state, bind | `/health`; `/ready` checks DB/Redis | Startup exits on config/migration/seed/bind failure; request behavior varies by dependency | `backend/src/main.rs`, `state.rs` |
| Frontend dev | Vite | 5173/HTTP | Node dependencies; backend URL for API use | Browser storage and memory | `VITE_API_URL`, Vite config | Independent after dependency install | No repository endpoint | UI unavailable; backend can remain running | `frontend/package.json`, `vite.config.ts` |
| Frontend production-like | Nginx static server | container 80/HTTP; Compose host 5173 | Built bundle; backend is an application dependency | Immutable image filesystem | Build-time `VITE_API_URL`, Nginx config | Compose starts after backend container, not backend health | No container/app health check | Static UI unavailable; backend may remain | Dockerfile/Compose/Nginx |
| PostgreSQL | `postgres:16-alpine` | 5432/PostgreSQL | Persistent volume | `postgres_data` | Compose variables | Before backend in production-like Compose | `pg_isready` container health | Backend startup/readiness and most operations fail | Compose files |
| Redis | `redis:7-alpine` | 6379/Redis | Persistent volume | `redis_data`; AOF in production-like config | Compose command/URL | Before backend in production-like Compose | `redis-cli ping` | `/ready` fails; some cache paths fall back; rate-limit paths may fail | Compose, cache service |
| pgAdmin | `dpage/pgadmin4:latest` | local host 5050 to 80/HTTP | Healthy PostgreSQL | Image-managed state not separately mounted | Local Compose development values | After PostgreSQL health | No pgAdmin health check | Developer UI only | Local Compose |
| Upload/artifact storage | Local path or named `uploads` volume | Filesystem | Backend | Files | `UPLOAD_DIR`, size limit | Available to backend process | No storage-specific health check | Media/package operations can fail; DB/file atomicity absent | Config, routes, Compose |
| External Stripe | HTTPS provider | Outbound/inbound signed webhooks | Optional keys/price IDs | Provider plus local billing records | Stripe variables | Feature-dependent | No provider readiness check | Billing/Marketplace finance branches fail or are unavailable | Config/services/routes |
| Email webhook | HTTPS outbound, optional | Provider URL | `EMAIL_PROVIDER=webhook` | Local delivery records | Email variables | Request-time | Delivery status record | Strict mode can fail request; default log mode avoids provider call | Email service |

See [Runtime Diagram](diagrams/runtime-topology.mmd), [Architecture](../architecture/overview.md), [Deployment](../delivery/deployment-workflow.md), and [Troubleshooting](troubleshooting.md).

