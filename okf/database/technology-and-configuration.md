---
okf_document_id: "database-technology-configuration"
title: "Database Technology and Configuration"
project: "ZinharCMS"
category: "database"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
primary_sources: ["backend/Cargo.toml", "backend/src/config.rs", "backend/src/db/mod.rs", "backend/src/main.rs", "docker-compose.yml", "docker-compose.prod.yml", ".github/workflows"]
related_documents: ["database/overview.md", "database/migrations.md", "backend/configuration-and-state.md"]
related_diagrams: ["database/diagrams/migration-lifecycle.mmd"]
uncertainty_markers: ["UNKNOWN U-02", "UNKNOWN U-04"]
---

# Database Technology and Configuration

## Technology Stack

PostgreSQL is the primary database. Compose and CI use `postgres:16-alpine`, which verifies the repository-tested major version but not the version of every deployment. SQLx 0.8 is the Rust client, pool, transaction, query-mapping, and migration framework. The code uses SQLx macros/functions, raw SQL, limited dynamic SQL, and `QueryBuilder<Postgres>`; no full ORM was found.

## Configuration Register

| Setting | Source | Purpose | Requirement/default | Sensitivity | Scope |
| --- | --- | --- | --- | --- | --- |
| `DATABASE_URL` | backend configuration and startup | PostgreSQL address and authentication material | Required; no code default | Secret-sensitive | All backend environments |
| PostgreSQL image | Compose and CI | Local/test database runtime | `postgres:16-alpine` where declared | Low | Development, CI, Compose production |
| Pool maximum | `backend/src/db/mod.rs` | Bound concurrent pooled connections | 10 | Operational | Backend process |
| Pool creation | `backend/src/db/mod.rs` | Avoid connecting during pool construction | Lazy | Operational | Backend process |
| SQLx migration directory | `backend/src/db/mod.rs` | Embed forward migrations | `./migrations` relative to backend crate | Low | Build/startup |
| Schema/namespace | Connection URL and PostgreSQL defaults | Resolve unqualified objects | No explicit code-level schema selection found | Operational | Connection/session |

Environment files and Compose definitions may bind supporting PostgreSQL variables to construct local credentials. This document intentionally records only names and behavior, not credential values or complete connection strings.

## Pool and Connection Behavior

`PgConnectOptions::from_str(database_url)` parses the URL and `PgPoolOptions` creates a lazy pool with `max_connections(10)`. No code-level minimum connection count, acquisition timeout, idle timeout, maximum lifetime, statement timeout, or connect retry loop was found. Driver parameters may still be supplied in a URL or environment, but deployment-only values are unknown.

TLS mode is not set explicitly in the pool builder. SQLx is compiled with Tokio Rustls support, but compilation support does not prove that a specific connection uses TLS. The connection URL may carry SSL parameters and is therefore security-sensitive.

## Initialization and Migration Execution

Backend startup loads configuration, creates the pool, executes `sqlx::migrate!("./migrations").run(pool)`, and stops startup if migration execution fails. It then runs the default-admin bootstrap before serving. This is an automatic startup migration path; no separate production migration orchestration procedure was verified. SQLx's internal migration bookkeeping is framework-managed and is excluded from the 51 application-table count.

Pool creation is lazy, but migration execution provides the first verified database operation and therefore acts as the startup reachability check. No separate retrying database health gate was found.

## Development, Test, and Container Setup

Local and production Compose definitions run PostgreSQL 16 Alpine alongside the backend. CI also provisions PostgreSQL and Redis services. A dedicated test-database naming convention, schema-per-test approach, reset harness, or reusable integration fixture was not found. The tracked SQL fixture in `docs/V2_PHASE_EIGHT_FIXTURE.sql` is explicitly local/staging-oriented and is not a general automated reset strategy.

## Shutdown and Failure Behavior

HTTP shutdown is graceful. No explicit `PgPool::close` call was observed; dropping application state closes resources as the process terminates. Database migration failure prevents serving, while runtime query failures flow through application error handling. Retry behavior is flow-specific rather than a database-wide policy.

## Security Notes

- Never log or document a populated `DATABASE_URL`.
- Do not infer TLS from the Rustls feature alone.
- Do not reuse bootstrap seed values in documentation or production procedures.
- Confirm migration authority, database roles, URL parameters, and production secret injection outside this repository.
