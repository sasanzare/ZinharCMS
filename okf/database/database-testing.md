---
okf_document_id: "database-testing"
title: "Database Testing"
project: "ZinharCMS"
category: "database"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-18"
primary_sources: ["backend/src", "backend/migrations", ".github/workflows", "docs/V2_PHASE_EIGHT_FIXTURE.sql"]
related_documents: ["database/seeds-and-fixtures.md", "database/multi-tenancy.md", "backend/testing-map.md"]
uncertainty_markers: ["TEST_ISOLATION_UNCLEAR TIU-01", "TEST_COVERAGE_GAP TCG-01", "DOCUMENTATION_CODE_CONFLICT DCC-12"]
---

# Database Testing

## Verified Test Evidence

CI provisions PostgreSQL and Redis services, but service availability alone is not proof that tests execute database assertions. No `backend/tests` integration suite, reusable database fixture/reset harness, schema-per-test framework, or comprehensive real-PostgreSQL test layer was found. Colocated backend tests are primarily pure/static checks; identified Tokio tests in Marketplace installation focus on filesystem artifacts rather than a shared database harness.

`docs/V2_PHASE_EIGHT_FIXTURE.sql` supports manual/local tenant-isolation exercises but is not integrated as an automated test. The hardening verification service checks selected migrations and RLS tables, but its 24-table list trails the 32-table migration intent (`DCC-12`).

## Coverage Matrix

| Entity or concern | Test/evidence path | Test type and behavior covered | Database dependency | Confidence | Apparent gap |
| --- | --- | --- | --- | --- | --- |
| Migration presence/order | `backend/migrations`; `backend/src/db/mod.rs` | Static source and embedded-runner wiring | No DB execution in Phase 5 | High for wiring | Clean install, supported upgrade, failure rollback |
| Startup migration failure | `backend/src/main.rs` | Control-flow inspection: startup stops on error | Would require real PostgreSQL | High from code | Runtime failure injection |
| Constraints/cascades | Migration SQL | Static definitions | None for documentation check | High definitions | Runtime violation and cascade matrix |
| Page revision transaction | pages route tests/source | Unit/static and transaction code inspection | Existing tests do not form shared DB harness | Medium | Commit/rollback/concurrency assertions |
| Media lifecycle | media processing tests/source | Filesystem/unit processing | No shared DB dependency | Medium for processing | File/row atomicity and cleanup |
| Tenant/RLS | `docs/V2_PHASE_EIGHT_FIXTURE.sql`; RLS migrations; hardening service | Manual fixture plus partial static verification | Manual/real DB only when explicitly run | Medium for intent | Cross-tenant CRUD, bypass, pool leakage, same-tenant FK checks |
| Billing/Stripe | billing/Stripe service tests/source | Pure signing/rule and flow inspection | Provider/DB integration not established | Medium | Event idempotency, row locks, reconciliation |
| Marketplace artifact installation | Marketplace installation Tokio tests | Filesystem artifact verification | No shared PostgreSQL harness | High for exercised files | Installation transaction/RLS/entitlement integration |
| Marketplace finance | finance service tests/source | Pure calculations/transitions | No real-DB concurrency suite found | Medium | `FOR UPDATE`, uniqueness races, append-only trigger |
| Marketplace feedback/abuse | feedback services/source | Pure rules and transaction inspection | No shared DB harness | Medium | Review uniqueness, report/notification rollback, RLS |
| Model/schema compatibility | Phase 5 migration/model comparison | Static mapping review | None | High for `MMC-01`/`MMC-02` | Automated compatibility gate |
| Seed idempotency | migration SQL and startup seed source | Static conflict/conditional pattern review | No isolated repeat-run test found | Medium | Repeated migration/bootstrap behavior |
| Performance/index usage | 109 explicit index names; migration 0026 | Static DDL review | No representative DB plan benchmark | High for presence | Workload/query-plan evidence |
| Test database creation/cleanup/parallelism | CI service definitions and repository search | PostgreSQL service provisioning only | Containerized CI service exists | Low behavior confidence | Database naming, reset, cleanup, isolation, parallel tests |
| Backup/restore | Repository search | No test/script evidence | Unknown | Low | RPO/RTO and restore test (`UNKNOWN U-04`) |

SQLx and Tokio are available test-time technologies, but no dedicated database testing framework beyond application SQLx usage was found. Mock-database usage was not found; most tests avoid database access rather than substitute a database mock.

## Required Regression Scenarios

Future database tests should use isolated PostgreSQL 16 instances and cover clean migration, upgrade from supported snapshots, seed repeatability, FK/check/unique failures, enum/model mapping, transaction rollback, concurrent finance idempotency, ledger/version triggers, and file/provider compensation boundaries.

Tenant tests should exercise read/write/delete from the owning tenant, a different tenant, no context, malformed context, and privileged bypass. They should also verify same-tenant parent-child coherence and that session context cannot leak through pooled connection reuse.

## Phase 5 Validation Boundary

Phase 5 performed static documentation validation only. It did not run migrations, connect to a database, mutate fixture data, or claim runtime schema equivalence. This preserves the documentation-only scope and leaves `TEST_ISOLATION_UNCLEAR TIU-01` open.

## Related Testing Navigation

Read the backend [Testing Map](../backend/testing-map.md), [Organizations and Membership](entities/organizations-and-membership.md), [Pages and Versions](entities/pages-and-versions.md), [Marketplace Purchases and Entitlements](entities/marketplace-purchases-and-entitlements.md), and [Marketplace Installations and Runtime Adapters](entities/marketplace-installations-and-runtime-adapters.md) before designing database coverage.

Phase 7 [Security Testing](../security/security-testing.md) confirms that current RLS assurance includes static migration contracts but no live exhaustive cross-tenant CRUD matrix. Test pooled-connection context reset, bypass preconditions, same-tenant parent coherence, global/system rows, and every major tenant entity before claiming runtime isolation.

## Development and Deployment Test Gap

Backend CI supplies PostgreSQL, but no dedicated disposable database lifecycle, migration replay/downgrade, schema-drift, restore, backup, concurrency, or production-like deployment test job exists. Startup migration behavior and selected static contracts are not substitutes for these tests. See [Testing Workflow](../development/testing-workflow.md), [Database Deployment](../delivery/database-deployment.md), and [Database Operations](../operations/database-operations.md).
