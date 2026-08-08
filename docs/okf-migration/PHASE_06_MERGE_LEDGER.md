# Phase 6 Merge Ledger

**Date:** 2026-08-08
**Source HEAD:** `eb050a0010ccd721446f5d2ac4de4863679a9564`
**Scope:** `TARGET-047` and `MERGE-GROUP-011` only

This ledger records semantic routing for every primary legacy input mapped to
the Phase 6 documentation-maintenance Concept. It does not delete, rename, or
rewrite any legacy file. Historical and owner-only material remains outside
the current Concept boundary.

## Outcome vocabulary

* `KNOWLEDGE_MERGED` - current source-backed maintenance knowledge was retained.
* `DUPLICATE_OMITTED` - a navigation wrapper or repeated checklist added no unique current claim.
* `STALE_OMITTED` - a currentness/status claim was not supported by the current snapshot.
* `CONTRADICTED_OMITTED` - current repository evidence directly contradicted the legacy statement.
* `UNVERIFIED_OMITTED` - a claim was not promoted without current evidence.
* `HISTORICAL_DEFERRED` - a phase snapshot remains historical and outside current Concepts.
* `OWNER_DECISION_DEFERRED` - the input depends on an unresolved owner or governance decision.
* `OUT_OF_TARGET_SCOPE` - the input belongs to another target.
* `NO_UNIQUE_KNOWLEDGE` - no distinct source-backed claim remained after consolidation.

## Input reconciliation

| Target ID | Merge Group | Legacy Input | Material Knowledge | Current Evidence | Outcome | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| TARGET-047 | MERGE-GROUP-011 | `okf/maintenance/README.md` | Maintenance directory purpose and reading navigation | `README.md`; staging indexes; migration policies | DUPLICATE_OMITTED | Navigation was reconstructed from the current target tree; the legacy wrapper and invalid metadata were not copied. |
| TARGET-047 | MERGE-GROUP-011 | `okf/maintenance/change-impact-matrix.md` | Documentation review triggers for source, configuration, migration, test, CI, and operational changes | `AGENTS.md`; `README.md`; `docs/okf-migration/OKF_TYPE_AND_METADATA_POLICY.md` | KNOWLEDGE_MERGED | The trigger categories were consolidated into one bounded table and marked documented policy. |
| TARGET-047 | MERGE-GROUP-011 | `okf/maintenance/documentation-ownership.md` | Absence of a named ownership map and the need to preserve owner dependencies | `README.md`; `AGENTS.md`; `HANDOFF.md` | OWNER_DECISION_DEFERRED | The absence fact is retained; canonical ownership, review authority, and named people remain unresolved. |
| TARGET-047 | MERGE-GROUP-011 | `okf/maintenance/final-completion-report.md` | Legacy Phase 10 inventory, completion status, and open operational questions | `README.md`; current Git state; current migration reports | HISTORICAL_DEFERRED | The 131c4f snapshot and its inventory counts were not promoted; current limitations are re-established from current evidence. |
| TARGET-047 | MERGE-GROUP-011 | `okf/maintenance/okf-update-policy.md` | Same-change documentation review, source precedence, and no-deployment-from-docs boundary | `AGENTS.md`; `README.md`; migration policy documents | KNOWLEDGE_MERGED | Preserved only as a documented maintenance boundary; it is not presented as an application or CI guarantee. |
| TARGET-047 | MERGE-GROUP-011 | `okf/maintenance/review-checklist.md` | Review checklist covering metadata, links, sources, diagrams, scope, and safety | `docs/okf-migration/OKF_VALIDATION_CONTRACT.md`; Phase 6 validation results | DUPLICATE_OMITTED | The current phase validation contract and report own the active checks; no legacy checklist was copied as enforcement. |
| TARGET-047 | MERGE-GROUP-011 | `okf/maintenance/staleness-detection.md` | Static/manual drift checks and limits of automatic staleness detection | `AGENTS.md`; `HANDOFF.md`; `OKF_VALIDATION_CONTRACT.md`; repository script inventory | KNOWLEDGE_MERGED | Current evidence confirms no tracked general-purpose OKF validator or staleness job; this limitation is explicit. |
| TARGET-047 | MERGE-GROUP-011 | `okf/maintenance/validation-report.md` | Historical Phase 10 validation inventory and unresolved findings | Current staging validator results; current repository evidence | HISTORICAL_DEFERRED | Historical counts and old commit claims remain migration history and were not reused as current validation. |

All eight mapped primary inputs have an explicit outcome. The related legacy
conflict and owner-question documents remain routed to their historical or
decision targets and are not silently absorbed here.

## Merge reconciliation

`MERGE-GROUP-011` is now accounted for by the built documentation-maintenance
Concept, explicit omission/deferment outcomes above, and the carried or later
targets in the complete status matrix. The remaining target-level deferred
merge count is zero; this does not authorize regeneration, owner resolution,
historical reconstruction, or canonical cutover.
