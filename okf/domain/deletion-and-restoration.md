---
okf_document_id: "domain-deletion-restoration"
title: "Deletion and Restoration"
project: "ZinharCMS"
category: "domain"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/routes"
  - "backend/migrations"
related_documents:
  - "content-lifecycle.md"
  - "revisions-and-versioning.md"
  - "../database/lifecycle-and-auditing.md"
related_diagrams:
  - "diagrams/content-lifecycle.mmd"
---

# Deletion and Restoration

No repository-wide trash, retention, legal-hold, or purge policy exists. Deletion semantics are entity-specific.

| Entity | Operation/access | Mode and related records | Restore/retention/audit | Tests/confidence |
| --- | --- | --- | --- | --- |
| User | No general delete API found | Database FKs cascade roles/tokens/members; attribution often becomes null | No restore or retention policy found | `DELETION_BEHAVIOR_UNCLEAR`; Medium |
| Organization | Status supports `deleted`; no complete delete transition handler found | FK cascades tenant resources if hard-deleted | No restore workflow found | `DELETION_BEHAVIOR_UNCLEAR`; Low workflow confidence |
| Organization member | Owner/admin removal or self-leave | Hard delete membership; last-owner check for owners | Re-invitation can recreate/upsert later; audit after delete | No integration tests; High observed path |
| Invitation | Admin revoke updates status; acceptance/expiry update status | Status lifecycle, not row deletion | Historical row retained; no restore | High |
| Content type | Manager plus `confirm=true` | Hard delete; entries cascade | No restore; type delete audit | No cascade tests; High persistence behavior |
| Content entry | Admin/editor | Hard delete current row | No content-data restoration; audit and published-cache invalidation | No route tests; High |
| Content/page archive | Reviewer | Status transition, not deletion | Archived returns to draft | Workflow tests partial; High |
| Page | Page manager plus `confirm=true` | Hard delete; snapshots cascade | No restore after delete; delete audit; published-cache invalidation | No route tests; High |
| Page version | No delete API | Cascades with page | Restore creates a new snapshot but does not recover deleted page | High |
| Component | Tenant component manager plus confirmation; system rows protected | Hard delete tenant component | No restore; existing pages may then fail validation on later save | No tests; `DELETION_BEHAVIOR_UNCLEAR` |
| Media | Admin/editor | Hard delete DB row/variant rows, then best-effort files | No restore or retention; delete audit | No failure-path tests; High sequence |
| Comment | Comment manager | Hard delete row | No restore; no dedicated audit call | No tests; High |
| Webhook | Manager plus confirmation | Hard delete; deliveries cascade | No restore; no retention policy | No tests; High |
| Public setting/navigation | Storage supports hard delete/replacement, but current management workflow not found | Entity-specific database behavior | No current restore API | `WORKFLOW_UNCLEAR` |
| Subscription/usage/billing event | No tenant delete workflow found | Organization cascade or event history | Retention/recovery unclear | `DELETION_BEHAVIOR_UNCLEAR` |
| Beta records | CRUD updates found; general delete routes not found | Organization cascade | Status closure/resolution substitutes for deletion | Medium |
| CMS plugin | Enable/disable instead of delete | Global row retained and built-ins resynchronized | Re-enable is restoration of behavior | High |
| Marketplace installation | Uninstall/disable/block lifecycle | Soft state with `preserve_organization_data`; row/history retained | Explicit reinstall/enable/update/rollback rules | Service tests; High |
| Marketplace listing/version | Archive/deprecate/block/moderation states | Catalog and immutable artifact history retained | Rollback selects an approved/deprecated version; no general hard-delete workflow | Strong tests; High |
| Marketplace purchase/ledger | Refund/status/append-only finance history | No destructive customer delete path | Entitlement revocation; financial retention policy unclear | Medium |

## Cascades and Restricts

- Tenant-root organization deletion would cascade many tenant rows, but a verified application workflow initiating hard deletion was not found.
- Content type deletion cascades entries; page deletion cascades versions; media deletion cascades variant rows; webhook deletion cascades delivery history.
- Marketplace catalog and finance relationships often use `RESTRICT` to preserve referenced versions/listings.
- User deletion uses a mixture of `CASCADE`, `SET NULL`, and `RESTRICT` depending on whether the relationship is access state, attribution, or sensitive history.

## Failure Boundaries

Database cascades are atomic with the parent delete, but filesystem cleanup, cache invalidation, and audit records are separate effects. Media file removal errors are ignored, and no background orphan cleanup was found. Therefore physical and logical deletion can diverge.

## Restoration Boundaries

“Restore” has three distinct meanings:

1. Archived content/page status returns to draft.
2. Page version restoration replaces current `page_json` and creates a new snapshot.
3. Marketplace rollback/enable/reinstall changes installation/version lifecycle.

None of these is a universal undelete/trash workflow.

