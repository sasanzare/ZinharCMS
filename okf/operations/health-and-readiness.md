---
okf_document_id: "operations-health-readiness"
title: "Health and Readiness"
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
  - "backend/src/routes/mod.rs"
  - "backend/src/main.rs"
  - "docker-compose.yml"
  - "docker-compose.prod.yml"
  - "scripts/v2-ga-check.ps1"
  - "scripts/marketplace-phase15-ga-check.ps1"
related_documents:
  - "runtime-topology.md"
  - "service-lifecycle.md"
  - "../api/endpoints/system-health-openapi-and-static.md"
  - "../delivery/deployment-workflow.md"
  - "../security/browser-and-http-security.md"
  - "troubleshooting.md"
related_diagrams:
  - "diagrams/health-check-flow.mmd"
---

# Health and Readiness

| Signal | Meaning | Checks | Response/status | Authentication | Container/CI/deployment use | Tests/evidence |
| --- | --- | --- | --- | --- | --- | --- |
| `GET /health` | Process/application route is alive after bind | No dependency | 200 JSON with `status: ok` and crate version | Public | Readiness scripts call it; not a container health check | Route/OpenAPI/static contract evidence |
| `GET /ready` | Application can reach required data/cache dependencies at request time | PostgreSQL `SELECT 1`; Redis connection and `PING` | 200 with `status: ready` and two checks; 503 application error when either fails | Public | Readiness scripts call it; not wired into Compose backend health | Route/OpenAPI/static contract and prior local smoke |
| PostgreSQL Compose health | Database container accepting configured DB/user | `pg_isready` | Docker health state | Internal container command | Backend `depends_on: service_healthy` in production-like Compose | Compose definition |
| Redis Compose health | Redis responds | `redis-cli ping` | Docker health state | Internal container command | Backend dependency gate in production-like Compose | Compose definition |
| Frontend/backend container liveness | No Docker health check | None | `HEALTH_CHECK_STATUS_UNCLEAR` | N/A | No automatic gate | Compose scan |

## Semantic Boundary

- **Alive:** `/health` proves the Axum route is serving; it does not prove PostgreSQL, Redis, storage, Stripe, email, webhooks, migrations in another replica, or frontend availability.
- **Ready:** `/ready` proves only PostgreSQL and Redis checks at that instant.
- **Dependencies available:** Storage and external providers are not included. There is no separate `/live` endpoint.

Readiness failure builds a degraded response internally, then converts it to the shared service-unavailable error. The resulting public error envelope is not the same successful `ReadyResponse` schema and may contain serialized dependency messages. No redaction guarantee or deployment-edge response test exists.

CI workflow steps do not call application health endpoints. GA/readiness PowerShell scripts can call them against a caller-supplied API URL. Actual load-balancer or orchestrator probes are `READINESS_STATUS_UNCLEAR`.

See [Health Flow](diagrams/health-check-flow.mmd), [System Endpoints](../api/endpoints/system-health-openapi-and-static.md), [Deployment](../delivery/deployment-workflow.md), and [Troubleshooting](troubleshooting.md).

