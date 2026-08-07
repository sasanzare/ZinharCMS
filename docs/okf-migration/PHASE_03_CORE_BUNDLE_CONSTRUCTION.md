# ZinharCMS Google OKF v0.2 Migration - Phase 3 Core Bundle Construction

**Phase:** 3 - Core Google OKF v0.2 Bundle Construction
**Construction date:** 2026-08-07 (Europe/London)
**Source HEAD:** `b58840e9c227ff9d937b482eced5331122291f82`
**Staging root:** `docs/okf-migration/staging/google-okf-v0.2/`
**Canonical cutover:** Not performed

## Objective and boundary

This phase constructs a new Google OKF v0.2 bundle in the isolated staging
root selected by Phase 2. The legacy `okf/` tree remains the legacy knowledge
base and is outside the construction boundary. `okf-bootstrap/`,
`okf/index.yaml`, application code, dependencies, database migrations, and CI
configuration are also outside the Phase 3 change set.

The construction slice is the 19-target
`CREATE_FROM_VERIFIED_KNOWLEDGE` set from the Phase 2 catalog. Each selected
Concept is bounded to current repository evidence and identifies unresolved
policy questions instead of promoting them to facts. The remaining 35 targets
are explicitly deferred as merge, regeneration, owner-decision, or historical
work. The complete 54-row matrix is in
[PHASE_03_CONSTRUCTION_STATUS.md](PHASE_03_CONSTRUCTION_STATUS.md).

## Selected construction set

The following targets were safe to build directly because the Phase 1 claim
ledger marks their core implementation evidence current and the Phase 2
catalog assigns `CREATE_FROM_VERIFIED_KNOWLEDGE`:

| Target IDs | Constructed area | Direct evidence boundary |
| --- | --- | --- |
| TARGET-001 | Project overview | README, LICENSE, and current release scope; no owner glossary asserted. |
| TARGET-006, TARGET-007 | Architecture | Current Rust/Axum route composition, frontend entry point, preview route, and delivery route; no deployment topology or public-host policy asserted. |
| TARGET-010, TARGET-011 | Backend | Current process initialization, shared state, route/middleware/service/plugin modules; no durable ownership inferred. |
| TARGET-014, TARGET-015 | Frontend | Current React/Vite shell, route tree, API client, volatile access token, and organization projection. |
| TARGET-018, TARGET-020 | API | Current route families and authentication/session behavior; no compatibility or deprecation policy asserted. |
| TARGET-027, TARGET-028, TARGET-029, TARGET-030 | Security | Current authentication, authorization, tenant/RLS, and preview controls; no live deployment or owner policy inferred. |
| TARGET-033, TARGET-034, TARGET-038, TARGET-039 | Domain | Current content workflow, page/preview behavior, Marketplace surface, and plugin boundary; no roadmap or settlement assumptions asserted. |
| TARGET-045 | Development | Current manifests, tests, and CI workflow gates; no deployment or support promise asserted. |
| TARGET-049 | Decisions | Phase 2 choices and pinned official OKF baseline, treated as migration decisions rather than application facts. |

Concepts affected by a non-blocking NOC are marked `draft` and state the
observed implementation separately from the unresolved policy. Concepts with
no material unresolved boundary are marked `stable` within the bounded staging
scope.

## Deferred construction set

* **DEFERRED_MERGE (12):** TARGET-008, TARGET-012, TARGET-016, TARGET-021,
  TARGET-022, TARGET-025, TARGET-031, TARGET-032, TARGET-036, TARGET-037,
  TARGET-040, TARGET-047. These require consolidation of several legacy views
  and current source review before a coherent Concept can be written.
* **DEFERRED_REGENERATE (11):** TARGET-003, TARGET-005, TARGET-009,
  TARGET-013, TARGET-017, TARGET-019, TARGET-023, TARGET-024, TARGET-035,
  TARGET-041, TARGET-042. These require exhaustive inventories, current
  migration/source generation, or deterministic reconciliation rather than a
  short direct narrative.
* **BLOCKED_OWNER_DECISION (7):** TARGET-002, TARGET-004, TARGET-026,
  TARGET-043, TARGET-044, TARGET-046, TARGET-048. Their intended authority,
  ownership, legal/operational policy, or unresolved decision debt cannot be
  safely finalized from repository evidence alone.
* **HISTORICAL_DEFERRED (5):** TARGET-050 through TARGET-054. These preserve
  historical context and must not be promoted into current implementation
  knowledge during the core construction.

There are no `INSUFFICIENT_EVIDENCE` or `OUT_OF_PHASE` rows in this Phase 3
catalog: every non-built target has a more specific controlled strategy.

## Bundle contents

The staging root contains 13 indexes, one reserved root `log.md`, and the 19
constructed Concepts. The root index has only `okf_version: "0.2"` in its
frontmatter. Concept frontmatter uses only the required local metadata fields
and zero custom extensions. Sources point to immutable GitHub blob URLs at the
construction commit. No standalone `.mmd` files were copied or created.

Thirteen Phase 1-preserved visuals are embedded in the Concepts that own their
current meaning: two in system architecture, two in tenant isolation, one in
content workflow, two in page builder/preview, and six across the remaining
preserved security/domain owners as recorded in the manifest. Deferred merge
and regeneration diagrams remain deferred and are not represented by
placeholder files.

## Validation record

The final validation pass is recorded in the Phase 3 handoff checkpoint and the
deterministic [PHASE_03_BUILD_MANIFEST.json](PHASE_03_BUILD_MANIFEST.json).
It passed the Google v0.2 structure, local metadata/type policy, source URL
pinning, bundle-local links, index reachability, embedded Mermaid policy,
54-row status coverage, manifest hashes, legacy safety, application/dependency
scope, whitespace, and unstaged Git state checks:

* 13 indexes, one reserved log, 19 Concepts, 13 embedded Mermaid blocks, and
  zero standalone `.mmd` files were found.
* All 54 target IDs were unique and reconciled to 19 `BUILT` plus 35 explicit
  deferred rows; status totals matched the construction matrix.
* All Concept sources were pinned to the immutable construction commit and all
  manifest evidence paths existed at that commit.
* All bundle-local Markdown links resolved without parent traversal, and all 33
  staging-file SHA-256 hashes reconciled with the manifest.
* `git diff --check` passed, the index remained empty, and legacy/application
  scope checks found no out-of-scope diff.

No application build or runtime test was required or run because Phase 3 makes
no source, dependency, database, or CI change; the documentation checks above
are the applicable validation for this isolated construction.

## Next phase boundary

Phase 4 may expand the isolated staging bundle using the Phase 1 MERGE and
REGENERATE mappings for architecture, backend, frontend, API, database,
security, domain, and operations knowledge. It must retain the same evidence
discipline and keep `/okf/` untouched until an independently authorized
cutover decision.
