---
okf_document_id: "operations-troubleshooting"
title: "Troubleshooting"
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
  - "backend/src/main.rs"
  - "backend/src/config.rs"
  - "backend/src/routes/mod.rs"
  - "package.json"
  - "docker-compose.yml"
  - "frontend/package.json"
related_documents:
  - "../architecture/overview.md"
  - "../delivery/deployment-workflow.md"
  - "../database/README.md"
  - "../security/README.md"
  - "runbook-catalog.md"
  - "operational-risks.md"
related_diagrams:
  - "diagrams/health-check-flow.mmd"
---

# Troubleshooting

Commands below are diagnostic or existing development commands. They are not production authorization. Never delete volumes, reset databases, edit migration history, print secrets, or modify production data as a troubleshooting shortcut.

| Issue | Symptoms and likely verified causes | Diagnostics / relevant signals | Safe corrective action | Destructive warning and escalation | Related documents |
| --- | --- | --- | --- | --- | --- |
| Backend does not start | Missing/invalid configuration, weak JWT secret, PostgreSQL/migration/seed failure, invalid Redis URL, port bind | Read startup context; `docker compose ps`; check `/health` only if bound | Correct local template-derived configuration; start required local services; retry after cause is understood | Never expose URL/secret or edit `_sqlx_migrations`; escalate migration/data errors | [Lifecycle](service-lifecycle.md), [Configuration](environment-configuration.md) |
| Frontend does not start | Dependencies missing, port conflict, Vite/TS error | `npm run dev`, then lint/typecheck from `frontend/` | Install declared dependencies and fix reported source/config issue | `npm install` may alter lock; review diff | [Development](../development/local-environment.md), [Frontend Build](../frontend/configuration-and-build.md) |
| Database connection fails | PostgreSQL down, wrong URL, health failure | Compose service health; backend context; `/ready` | Verify local service and non-secret URL structure | Do not recreate volume or reset DB; escalate unknown production access | [Database Operations](database-operations.md) |
| Migrations fail | Backend exits before bind | Preserve error and migration filename/version; inspect ordered SQL | Reproduce only in approved disposable/dev database after review | Do not edit migration history or production rows; backup/restore is undefined | [Database Deployment](../delivery/database-deployment.md) |
| Authentication fails | 401/403, cookie/CORS/tenant mismatch, rate limit, bootstrap uncertainty | Response code; browser network; configured origin; auth/security docs | Verify correct local origin, cookie mode, credentials, active organization context | Do not copy tokens or use deterministic bootstrap credentials outside development | [Security](../security/README.md) |
| API returns errors | Application error envelope, framework error, timeout, dependency/provider failure | Status/body, `X-Request-Id`, backend trace, `/ready` | Reproduce smallest request with synthetic data; inspect authoritative route contract | Error messages may contain technical detail; redact before sharing | [API Errors](../api/error-contracts.md) |
| Frontend cannot reach backend | Wrong `VITE_API_URL`, CORS mismatch, backend down | Browser network, `/health`, build/dev variable | Align local public API URL and CORS origin; restart development build if build-time value changed | `VITE_*` is public; never put secrets in it | [Frontend API](../frontend/api-client.md) |
| Docker services fail | Engine unavailable, port conflict, unhealthy DB/Redis | `docker compose ps`; Docker engine status; container output | Start Docker engine; resolve local port/config conflict | Do not delete named volumes | [Runtime](runtime-topology.md) |
| Tests fail | Source regression, unavailable tool/service, sandbox process issue | Rerun smallest filter; preserve full output; compare CI environment | Fix scoped cause or report environment blocker | Do not hide/skip failure without recording it | [Testing](../development/testing-workflow.md) |
| Build fails | Rust/TS/Vite/dependency issue | Run format/lint/typecheck/build gate separately | Address first deterministic diagnostic | Do not clean broadly or rewrite lock without review | [Build and Quality](../development/build-and-quality.md) |
| Environment variables missing | Startup config error or Compose expansion error | Compare names only with templates/config code | Add local synthetic value through ignored environment | Never paste real secret values into logs/docs | [Environment](environment-configuration.md) |
| Health checks fail | `/health` unreachable or `/ready` 503 | Separate liveness from PostgreSQL/Redis checks | Restore local dependency availability; preserve error context | Production remediation requires owner/runbook | [Health](health-and-readiness.md) |
| Plugin registration fails | Built-in sync, database state, disabled status, Marketplace component/hook policy | Plugin endpoint/test output; plugin docs; backend logs | Verify compiled plugin inventory and tenant installation state | Do not execute uploaded package code or edit plugin rows directly | [Extensibility](../extensibility/README.md) |
| Page Builder fails to load | API/component registry, auth/tenant, invalid page JSON, frontend error | Browser network, component endpoint, Pages tests, backend response | Verify organization context and registered component definitions | Avoid direct page JSON/database edits | [Page Builder](../frontend/page-builder.md) |
| Logging is missing | `RUST_LOG`, runtime stdout collection, filter level | Check process environment name and direct stdout | Use an appropriate non-secret local filter and preserve process output | Retention/collector is unknown; do not increase production verbosity without approval | [Logging](logging-and-tracing.md) |

Escalation points are ownership classes, not named contacts: backend, frontend, database, security, Marketplace, or operations maintainers. Actual owners remain `OPERATIONAL_OWNERSHIP_UNCLEAR`. See [Runbook Catalog](runbook-catalog.md) and [Operational Risks](operational-risks.md).

