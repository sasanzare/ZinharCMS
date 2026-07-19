---
okf_document_id: "operations-external-dependencies"
title: "External Dependencies"
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
  - "backend/src/config.rs"
  - "backend/src/main.rs"
  - "backend/src/services/cache.rs"
  - "backend/src/services/email.rs"
  - "backend/src/services/webhooks.rs"
  - "docker-compose.yml"
related_documents:
  - "runtime-topology.md"
  - "environment-configuration.md"
  - "health-and-readiness.md"
  - "../delivery/container-builds.md"
  - "../security/trust-boundaries.md"
  - "troubleshooting.md"
related_diagrams:
  - "diagrams/runtime-topology.mmd"
---

# External Dependencies

| Dependency | Purpose/protocol | Configuration/authentication | Startup requirement | Failure/retry behavior | Health check | Local substitute | Confidence |
| --- | --- | --- | --- | --- | --- | --- | --- |
| PostgreSQL | System of record, SQL over TCP | `DATABASE_URL`; database credentials | Required for migrations/seed/startup | Startup fails; request errors later; no app retry policy documented | `/ready` query and Compose `pg_isready` | PostgreSQL 16 Compose service | `VERIFIED` |
| Redis | Cache/rate-limit state, Redis protocol | `REDIS_URL`; auth may be encoded by environment | URL required/has default; reachability not startup-gated | Cache reads fall back and invalidation can no-op; other consumers vary | `/ready` ping and Compose `redis-cli ping` | Redis 7 Compose service | `VERIFIED` |
| Local filesystem/volume | Media and Marketplace artifact bytes | `UPLOAD_DIR`; OS/container permissions | Required when file operations occur | Operation-specific errors; DB/filesystem changes are not atomic | No dedicated check | Local `./uploads` | `VERIFIED` |
| Stripe | Billing and Marketplace finance over HTTPS plus signed callbacks | Optional secret/webhook/price variables | Feature-dependent | Services record/fail provider operations; webhook idempotency exists; provider-wide health not checked | None | No emulator; tests use synthetic contracts | `VERIFIED` integration, production state unknown |
| Email webhook provider | Transactional email over HTTPS | Provider, sender, optional URL; no auth field beyond URL/config | Optional | One request; result persisted; strict mode can fail caller; no retry worker | Delivery records only | Default `log` provider | `VERIFIED` |
| Tenant CMS webhook destinations | Outbound HTTPS to tenant-configured endpoints with HMAC | Per-webhook URL/secret in database | Event-dependent | One spawned attempt, 10-second timeout, result persisted; no durable retry | Delivery records only | No localhost target allowed by validation | `VERIFIED` |
| Nginx | Static frontend and SPA fallback | Image config; no backend proxy | Production frontend container only | Container/process behavior; no app-specific retry | None | Vite dev server | `VERIFIED` configuration |
| npm/Cargo registries | Build-time dependency acquisition | Tool defaults and external credentials if needed | Fresh install/build | Tool-defined retries/failure | None | Local caches | `INFERRED_FROM_CONFIGURATION` |

No verified object storage service, durable queue, search engine, external identity provider, container registry, monitoring vendor, alert manager, backup service, or CDN integration exists. See [Trust Boundaries](../security/trust-boundaries.md), [Runtime Topology](runtime-topology.md), [Health](health-and-readiness.md), and [Troubleshooting](troubleshooting.md).

