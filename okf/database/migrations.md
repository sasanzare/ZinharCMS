---
okf_document_id: "database-migrations"
title: "Database Migrations"
project: "ZinharCMS"
category: "database"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
primary_sources: ["backend/migrations", "backend/src/db/mod.rs", "backend/src/main.rs"]
related_documents: ["database/schema-catalog.md", "database/database-risks.md", "database/technology-and-configuration.md"]
related_diagrams: ["database/diagrams/migration-lifecycle.mmd"]
uncertainty_markers: ["UNKNOWN U-02", "NEEDS_OWNER_CONFIRMATION NOC-14", "SCHEMA_RUNTIME_STATUS_UNKNOWN SRU-01"]
---

# Database Migrations

## Mechanism and Lifecycle

ZinharCMS has 26 ordered, forward-only SQL migration files in `backend/migrations`, from `0001` through `0026`. SQLx embeds them at build time and executes pending migrations automatically during backend startup. Startup stops on migration error. No down-migration set, separate production migration command, or repository-governed rollback procedure was found.

Never edit an existing applied migration unless project policy explicitly authorizes that exceptional operation. Prefer a new ordered migration, review it against the complete history, test upgrade behavior on representative data, and update the related OKF documents. Applied state in actual environments is `UNKNOWN U-02`.

## Chronological Catalog

Every row is forward-only with no paired down file (`NO_DOWN`). “None persistent” in the removed column means any `DROP` is part of replacing a constraint, policy, trigger, index, or key rather than removing a final schema object.

| ID and file path | Purpose / related entities | Objects created | Objects altered | Objects removed | Data migration | Reversible status | Risk |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 0001 `backend/migrations/0001_initial_schema.sql` | Identity, content, pages, components, media | 11 tables, 2 enums, 2 extensions, indexes | None | None | No | `NO_DOWN` | `MEDIUM` foundation |
| 0002 `backend/migrations/0002_seed_foundation_data.sql` | Foundation roles/data | None | Seed rows | None | Yes, roles/foundation | `NO_DOWN` | `LOW` |
| 0003 `backend/migrations/0003_phase_one_core.sql` | Media caption and role permissions | None | `media`; role rows | None | Yes, role upserts | `NO_DOWN` | `LOW` |
| 0004 `backend/migrations/0004_phase_two_page_builder.sql` | Component registry/page builder | Component-key index/constraints | `component_registry`; role rows | None | Yes, key backfill and component/role upserts | `NO_DOWN` | `MEDIUM` |
| 0005 `backend/migrations/0005_phase_five_delivery_api.sql` | Webhooks, deliveries, public settings, navigation | 4 tables and indexes | None | None | Yes, public defaults where defined | `NO_DOWN` | `MEDIUM` |
| 0006 `backend/migrations/0006_phase_six_workflow_collaboration.sql` | Workflow, comments, CMS plugins | 2 tables and indexes | 2 enums; role rows | None | Yes, plugin/permission upserts | `NO_DOWN` | `MEDIUM` enum change |
| 0007 `backend/migrations/0007_phase_seven_security.sql` | Login security history | `login_attempts` and indexes | None | None | No | `NO_DOWN` | `LOW` |
| 0008 `backend/migrations/0008_v2_phase_one_organizations.sql` | Tenant root and tenant retrofit | 3 tables, enum types, propagation functions/triggers, indexes | 12 existing domain tables and keys | None persistent; replacement drops | Yes, default tenant and organization backfills | `NO_DOWN` | `HIGH` |
| 0009 `backend/migrations/0009_v2_phase_three_rls.sql` | Tenant RLS | RLS helper functions and policy set | Enables/forces RLS on 12 tables | Temporary procedure and replacement policies | No row migration | `NO_DOWN` | `HIGH` |
| 0010 `backend/migrations/0010_v2_phase_five_billing_quota.sql` | Plans, subscriptions, usage | 3 tables, subscription enum, policies/indexes | None | Replacement policies only | Yes, plan/subscription seed | `NO_DOWN` | `MEDIUM` |
| 0011 `backend/migrations/0011_v2_phase_six_stripe_billing.sql` | Provider billing events | `billing_events`, indexes, policies | `plans` provider fields | Replacement policies only | No material backfill identified | `NO_DOWN` | `MEDIUM` |
| 0012 `backend/migrations/0012_v2_phase_seven_saas_ops.sql` | Domains, rate limits, audit, email, alerts | 5 tables, indexes, policies | RLS coverage | Replacement policies only | Yes, operational defaults | `NO_DOWN` | `MEDIUM` |
| 0013 `backend/migrations/0013_v2_phase_eight_hardening.sql` | Provider event hardening | Provider-created index | Subscriptions and billing events | None | No row migration | `NO_DOWN` | `LOW` |
| 0014 `backend/migrations/0014_v2_phase_nine_beta_release.sql` | Beta participation, feedback, blockers | 3 tables, indexes, policies | RLS coverage | Replacement policies only | No material backfill identified | `NO_DOWN` | `MEDIUM` |
| 0015 `backend/migrations/0015_v3_phase_one_marketplace_foundation.sql` | Marketplace creator/catalog/submission/install | 5 tables, immutable trigger, indexes, installation policies | RLS on installations | Replacement trigger/policies | No material backfill identified | `NO_DOWN` | `HIGH` domain foundation |
| 0016 `backend/migrations/0016_v3_phase_two_creator_submission.sql` | Creator and submission workflow | Queue indexes; revised immutable trigger | Creators/listings/version behavior | Replaced trigger | Yes, status normalization/backfill | `NO_DOWN` | `HIGH` |
| 0017 `backend/migrations/0017_v3_phase_three_validation_pipeline.sql` | Version validation and risk | Validation indexes | `marketplace_versions` validation fields/constraints | Replacement constraints | Yes, validation/risk backfill | `NO_DOWN` | `HIGH` |
| 0018 `backend/migrations/0018_v3_phase_four_review_moderation.sql` | Review/moderation history | `marketplace_review_events` and indexes | Marketplace review state | None persistent | No material backfill identified | `NO_DOWN` | `MEDIUM` |
| 0019 `backend/migrations/0019_v3_phase_six_installation_lifecycle.sql` | Installation lifecycle | Installation index/constraints | `marketplace_installations` lifecycle fields | Replaced constraints/FK | Yes, installation lifecycle normalization | `NO_DOWN` | `MEDIUM` |
| 0020 `backend/migrations/0020_v3_phase_seven_permission_sandbox_kill_switch.sql` | Permission catalog and kill switches | 2 tables, indexes, policies | Installation runtime fields | Replacement policies | Yes, permission catalog seed | `NO_DOWN` | `MEDIUM` |
| 0021 `backend/migrations/0021_v3_phase_eight_runtime_adapters.sql` | Template and plugin host adapters | 2 tables, indexes, policies | `component_registry` installation link | Replacement policies | No | `NO_DOWN` | `MEDIUM` |
| 0022 `backend/migrations/0022_v3_phase_nine_marketplace_finance.sql` | Purchases, entitlements, ledger, payout records | 5 tables, indexes, policies | RLS on 3 tenant finance tables | Replacement policies | No | `NO_DOWN` | `HIGH` financial |
| 0023 `backend/migrations/0023_v3_phase_nine_finance_hardening.sql` | Finance idempotency and immutability | Provider index, append-only function/trigger, payout index | Ledger and payout provider fields | Recreated ledger index/trigger | No material backfill identified | `NO_DOWN` | `HIGH` |
| 0024 `backend/migrations/0024_v3_phase_ten_ratings_abuse.sql` | Product reviews and abuse reports | 2 tables, indexes, policies | RLS on both tables | Replacement policies | No | `NO_DOWN` | `MEDIUM` |
| 0025 `backend/migrations/0025_v3_phase_ten_internal_notifications.sql` | Critical abuse notifications | `marketplace_internal_notifications` and queue index | None | None | No | `NO_DOWN` | `LOW` |
| 0026 `backend/migrations/0026_v3_phase_thirteen_marketplace_qa_performance.sql` | Marketplace search/performance support | `pg_trgm` and 11 indexes | None | None | No | `NO_DOWN` | `MEDIUM` index-build impact |

No explicit application migration execution command is defined as a separate production workflow. Startup calls the embedded SQLx runner. SQLx may transactionally apply eligible migration statements, but the repository does not document per-file transaction exceptions, production lock handling, or a test migration command; those behaviors remain unverified rather than inferred.

## Destructive and Replacement Operations

No `DROP TABLE` or `DROP COLUMN` statement was found. Drops are targeted replacements for constraints, policies, triggers, indexes, or a primary key. That does not make every migration low-risk: table rewrites, backfills, `NOT NULL`, enum changes, RLS activation, key replacement, and large index creation can block or fail on production data.

The most operationally sensitive files are 0008, 0009, 0016, 0017, and 0023. Migration 0004 also performs a backfill before adding stronger component-key constraints. Migration 0006 demonstrates that similarly named workflow concepts use separate PostgreSQL enums.

## Validation and Rollback Expectations

Repository evidence does not define maintenance windows, lock budgeting, preflight queries, deployment sequencing, or rollback ownership (`NOC-14`). A safe future change should include forward and compatibility analysis, data-volume-aware backfill planning, RLS-policy tests, query/model compatibility checks, and an explicit operational fallback. Schema rollback cannot be assumed from the absence of down files; application rollback may also become incompatible after a forward schema change.
