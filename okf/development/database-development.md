---
okf_document_id: "development-database"
title: "Database Development"
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
  - "backend/migrations"
  - "backend/src/db/mod.rs"
  - "backend/src/main.rs"
  - "docker-compose.yml"
related_documents:
  - "local-environment.md"
  - "commands.md"
  - "../database/README.md"
  - "../database/migrations.md"
  - "../database/database-testing.md"
  - "../delivery/database-deployment.md"
related_diagrams: []
---

# Database Development

## Supported Behavior

| Concern | Repository-supported behavior | Status and caution |
| --- | --- | --- |
| Start development database | `docker compose up -d postgres` or the root infrastructure script | `VERIFIED`; creates/uses a persistent named volume |
| Create database | PostgreSQL image initializes configured database/user on a new volume | `VERIFIED`; environment-driven, not a project migration command |
| Create migration | Add the next ordered `.sql` file under `backend/migrations` | `INFERRED_FROM_STRUCTURE`; no generator command or naming policy script |
| Apply migrations | Backend startup calls `sqlx::migrate!("./migrations").run(pool)` | `VERIFIED`; **schema mutation** occurs before serving traffic |
| Revert migration | No down-migration files or command found | `NOT_IMPLEMENTED`; do not invent downgrade SQL |
| Reset development database | No repository-owned reset command found | `COMMAND_STATUS_UNCLEAR`; any volume/database deletion is destructive |
| Seed data | Startup inserts bootstrap data when the users table is empty | `VERIFIED`; contains deterministic development credential material (`POTENTIAL_SECRET_EXPOSURE PSE-01`) |
| Test database | Backend CI supplies `cms_dev` PostgreSQL; no per-test database harness | `PARTIALLY_DEFINED` |
| Schema verification | SQLx migration history is authoritative for defined schema; runtime applied state is environment-specific | `SCHEMA_RUNTIME_STATUS_UNKNOWN SRU-01` |
| Model synchronization | Manual review; shared Rust models are partial projections | `MIGRATION_MODEL_CONFLICT MMC-01/MMC-02` |
| Data migration testing | No dedicated replay/rollback harness | `TEST_COVERAGE_GAP` |

## Migration Review Checklist

1. Confirm the numeric ordering and unique migration name.
2. Review locks, table rewrites, defaults, indexes, constraints, enum changes, and data transformations.
3. Review tenant ownership, `organization_id`, RLS enable/force policies, and bypass requirements.
4. Confirm application code can tolerate the transition; no repository-wide expand/contract policy exists.
5. Run the relevant backend tests and apply only to an approved disposable or development database.
6. Verify `_sqlx_migrations`, expected objects, and application readiness.
7. Do not claim rollback safety; restore behavior is undefined without an owner-approved backup and restore process.

Phase 10 did not execute database commands. See [Database Architecture](../database/README.md), [Migrations](../database/migrations.md), and [Database Deployment](../delivery/database-deployment.md).

