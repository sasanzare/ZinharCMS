---
okf_document_id: "database-seeds-fixtures"
title: "Database Seeds and Fixtures"
project: "ZinharCMS"
category: "database"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
primary_sources: ["backend/migrations", "backend/src/db/mod.rs", "docs/V2_PHASE_EIGHT_FIXTURE.sql"]
related_documents: ["database/migrations.md", "database/database-testing.md", "database/multi-tenancy.md"]
uncertainty_markers: ["TEST_ISOLATION_UNCLEAR TIU-01", "NEEDS_OWNER_CONFIRMATION NOC-14"]
---

# Database Seeds and Fixtures

## Migration Seeds

Migration history seeds foundation records needed by the application:

- global roles and permission arrays;
- system component registry definitions;
- default public settings and navigation;
- a built-in SEO plugin;
- a default organization and membership backfill during tenant introduction;
- subscription plans and organization subscription records;
- organization rate-limit defaults;
- SaaS alert-rule definitions;
- the Marketplace permission catalog.

Seed statements commonly use conflict handling to remain repeatable within migration intent. Their values are implementation data, not general documentation examples; consumers must inspect the exact migration and later updates.

## Seed and Fixture Register

| Path / source | Purpose and environment | Entities affected | Idempotency / ordering | Sensitive-data risk | Execution and related tests |
| --- | --- | --- | --- | --- | --- |
| `backend/migrations/0002_seed_foundation_data.sql` and later role upserts | Foundation roles/permissions; all migrated environments | `roles` | Conflict-aware; requires foundation schema | Permission drift risk; no credentials | Executed by SQLx migration order; no dedicated seed test found |
| 0004–0006 migration-embedded defaults | Builder components, public settings/navigation, built-in CMS plugin | component registry, public settings, navigation, CMS plugins, roles | Conflict-aware and ordered after base tables | Configuration may affect runtime behavior | Automatic migration execution; static evidence |
| 0008–0014 migration-embedded tenant/SaaS defaults | Default organization backfill, subscriptions/plans, rate limits, alert rules | organizations, members, CMS tenant columns, plans/subscriptions, rate limits, alerts | Backfill/order-sensitive; selected inserts are conflict-aware | Can create privileged/control-plane defaults | Automatic migrations; manual tenant fixture supports partial checks |
| 0020 permission catalog seed | Global Marketplace permission vocabulary | `marketplace_permission_catalog` | Conflict-aware; requires Marketplace foundation/install tables | Security policy data | Automatic migration; policy unit/static tests only |
| `backend/src/db/mod.rs` called from startup | Conditional default administrator bootstrap | `users` and related bootstrap state | Runs only when user count is zero; not a general idempotent fixture | High credential/account risk; values intentionally omitted | Application startup; no isolated bootstrap DB test found |
| `docs/V2_PHASE_EIGHT_FIXTURE.sql` | Manual local/staging tenant-isolation exercise | organizations, users, memberships, content types/entries, pages, rate limits | Not a reset harness; ordering assumes migrated schema | Synthetic but security-sensitive; not login-ready | Manual execution only; not wired to CI |
| Test-data builders/reset scripts | None found | Not applicable | Unknown | Not applicable | `TEST_ISOLATION_UNCLEAR TIU-01` |

No separate demo-content production seed, test-only factory library, or database reset script was verified. Migration-embedded seeds can execute wherever migrations execute, including production-capable startup; the repository does not expose a separate “production seed” switch.

## Startup Bootstrap

Startup invokes `seed_default_admin`. It inserts the bootstrap administrator only when the user count is zero. This conditional behavior is verified, but credentials and values are intentionally not reproduced. A default account bootstrap is security-sensitive: production ownership, secret injection, rotation, and disabling behavior must be confirmed operationally.

## Tracked Fixture

`docs/V2_PHASE_EIGHT_FIXTURE.sql` is a local/staging tenant-isolation fixture. It covers organizations, users, memberships, content types, content entries, pages, and organization rate limits. Its data is intentionally not login-ready and is not a full reset, production seed, or automated integration-test harness. Do not copy its record values into documentation or production.

## Fixture Safety

- Use synthetic identities and secrets.
- Keep tenant A and tenant B records distinct for isolation assertions.
- Do not assume migration seeds are disposable test fixtures.
- Make reset/cleanup behavior explicit before automation.
- Test seed idempotency only in isolated databases.
- Never run fixture SQL against an unidentified database.

No general fixture factory, schema reset utility, or per-test transaction framework was found (`TEST_ISOLATION_UNCLEAR TIU-01`).
