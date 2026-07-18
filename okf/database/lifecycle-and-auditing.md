---
okf_document_id: "database-lifecycle-auditing"
title: "Database Lifecycle and Auditing"
project: "ZinharCMS"
category: "database"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "mixed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
primary_sources: ["backend/migrations", "backend/src/routes", "backend/src/services"]
related_documents: ["database/entity-catalog.md", "database/transactions-and-consistency.md", "database/database-risks.md"]
uncertainty_markers: ["SOFT_DELETE_BEHAVIOR_UNCLEAR SDBU-01", "DATA_LIFECYCLE_UNCLEAR DLU-01", "DATA_LIFECYCLE_UNCLEAR DLU-02", "UNKNOWN U-07"]
---

# Database Lifecycle and Auditing

## Lifecycle Patterns

There is no uniform soft-delete column or deletion framework. Lifecycle is expressed through a mixture of hard deletion, status enums or checked text, archive states, timestamps, immutable history, and provider event records.

| Pattern | Objects | Verified behavior |
| --- | --- | --- |
| Publication/archive | content entries, pages | Draft/review/published/archive state and publication timestamps |
| Revision history | `page_versions` | Snapshot rows created with page mutations |
| Editorial history | comments | Body, resolution, and modification fields |
| Delivery history | webhook deliveries, email deliveries | Attempts and delivery outcomes persist |
| Security history | login attempts, refresh tokens | Attempt and revocation/expiry state |
| Commercial history | billing events, purchases, ledger, payouts | Provider/idempotency/status records; ledger append-only |
| Moderation history | submissions, review events, abuse reports | State transitions and actor/reason data |
| Soft-uninstall-like | Marketplace installations | Status and `uninstalled_at`; organization content preservation is intended |
| Immutable artifact | Marketplace versions | Trigger rejects artifact mutation in protected lifecycle states |

## Lifecycle Evidence Matrix

| Pattern | Entities and schema fields | Enforcement location | Query behavior | Test coverage | Confidence / inconsistency |
| --- | --- | --- | --- | --- | --- |
| Created/updated timestamps | Most mutable tables use `created_at` and often `updated_at` | Defaults plus application updates | Common sort/filter fields | Static/unit evidence; no universal DB test | High presence; not universal |
| Publication/archive | Content entries/pages: `status`, `published_at` | PostgreSQL enums, routes/workflow services | Delivery selects published state; admin flows select workflow state | Pure/route evidence only | High; shared page enum drift `MMC-01` |
| Revision | `page_versions.version`, `page_json`, `snapshot_at`, `created_by` | Page transaction and unique `(page_id, version)` | List/restore by page and version | No dedicated real-DB rollback suite found | High implementation confidence |
| Data version | `content_entries.version` | Constraint plus application writes | Returned/updated in content flows | No platform optimistic-lock test | Medium; not a verified compare-and-swap contract |
| Resolution/moderation | Comment resolution, beta feedback/blockers, Marketplace submissions/reports/reviews | Status checks and route/service transitions | Queue/status filters | Primarily pure/static tests | High fields; retention inconsistent |
| Audit/history | `audit_logs`, review events, billing events, delivery records, login attempts | Shared/domain service writes and DB constraints | Actor/action/status/time retrieval | No completeness test across all mutations | Medium; not every mutation auto-audited |
| Soft-uninstall/archive | Marketplace installation `status`, `uninstalled_at`; CMS archived statuses | Service transitions and status checks | Active partial indexes exclude uninstalled/archived forms as defined | Installation unit/filesystem tests; no full DB suite | High for install; no universal soft delete (`SDBU-01`) |
| Immutability/append-only | Marketplace version protected statuses; revenue ledger | Database triggers | Writes fail on protected update/delete | No dedicated trigger integration suite found | High migration evidence |
| Expiry/revocation | Refresh tokens, invitations, entitlements/provider periods | Timestamps/status plus application checks | Active/expiry filters in relevant services | No common time-control harness found | Medium, entity-specific |
| Cascading hard deletion | Organization, content/page/media, catalog FKs | FK `ON DELETE` actions | Delete handlers plus database cascades/restrict/set-null | Cascade matrix tests not found | High definitions; operational impact untested |
| Retention/purge | Historical/security/audit/provider tables | None found repository-wide | No common purge query/job | None found | `DATA_LIFECYCLE_UNCLEAR DLU-01` |

No common `deleted_at`, `deleted_by`, or `updated_by` convention spans all entities. Actor columns are entity-specific (`created_by`, `author_id`, `actor_id`, reviewer/resolver fields), and restoration is verified for pages but not generalized.

## Audit Mechanisms

`audit_logs` is the general tenant audit table and records actor, action, logical entity type/ID, metadata, and time. Many important workflows call an audit service, but not every table mutation is automatically captured by a database trigger. `marketplace_review_events`, `billing_events`, webhook/email delivery records, login attempts, page versions, and internal notifications are domain-specific histories rather than substitutes for the general audit stream.

The Marketplace revenue ledger is append-only at the database trigger layer. Marketplace version artifact fields are immutable after protected statuses. These are narrow invariants, not a universal temporal-data model.

## Deletion Behavior

Hard-delete endpoints and FK cascades/restrictions exist in several domains. Content/page archival and installation uninstalling preserve selected records, while settings/navigation and other configuration may be replaced or deleted. The authoritative action for a table is its current route/service SQL plus final FK definitions. Mixed behavior is `SOFT_DELETE_BEHAVIOR_UNCLEAR SDBU-01` and `DATA_LIFECYCLE_UNCLEAR DLU-02`, not evidence that one policy applies globally.

## Timestamps and Retention

`created_at`, `updated_at`, status-specific timestamps, expiry times, publication times, resolution times, and provider processing times are common but not universal. No repository-wide retention, anonymization, legal-hold, archival, or purge schedule was found. This is `DATA_LIFECYCLE_UNCLEAR DLU-01`, `UNKNOWN U-07`, and `NEEDS_OWNER_CONFIRMATION NOC-05`.

Backup/restore and historical recovery expectations are also unknown (`UNKNOWN U-04`, `NOC-03`). Do not add deletion jobs, retention windows, or PII examples based on this document.
