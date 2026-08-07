# ZinharCMS Legacy-to-Target Google OKF Mapping

**DESIGN ONLY - NOT YET IMPLEMENTED**

This document is the bridge between the complete Phase 1 preservation corpus
and the target design. It maps every Phase 1 document row through its semantic
merge group and target Concept family without executing any migration action.
The authoritative row-level inputs are
[LEGACY_DOCUMENT_DISPOSITION_MATRIX.md](LEGACY_DOCUMENT_DISPOSITION_MATRIX.md),
[CONCEPT_CANDIDATE_MAP.md](CONCEPT_CANDIDATE_MAP.md),
[CLAIM_VERIFICATION_LEDGER.md](CLAIM_VERIFICATION_LEDGER.md), and
[DIAGRAM_PRESERVATION_MAP.md](DIAGRAM_PRESERVATION_MAP.md).

## Mapping rules

The allowed future actions are planning states only:

| Action | Meaning in this design |
| --- | --- |
| `CREATE_FROM_VERIFIED_KNOWLEDGE` | Build a bounded current Concept from current source-backed evidence. |
| `MERGE_INTO_TARGET` | Absorb several legacy views into one or more target Concepts without copying their hierarchy or metadata. |
| `REGENERATE_FROM_REPOSITORY` | Rebuild a catalog or source-sensitive Concept from current routes, migrations, tests, configuration, CI, or owner evidence. |
| `PRESERVE_AS_HISTORY` | Retain historical reasoning and snapshot context as explicitly typed historical knowledge. |
| `OMIT_AFTER_VERIFIED_REDUNDANCY` | Remove a navigation-only legacy row only after its useful links/claims are covered and cutover validation passes. |
| `BLOCKED_PENDING_DECISION` | Keep the target outline/draft but do not promote the unresolved policy to current truth. |

No action below was executed in Phase 2. A legacy row may feed multiple target
Concepts when its claims cross capability boundaries; the row still needs one
auditable disposition and at least one target destination.

## Complete document-row reconciliation

The Phase 1 matrix contains 353 Markdown rows: 339 under `okf/` and 14 under
`okf-bootstrap/`. The exact primary disposition totals reconcile as follows:

| Phase 1 disposition | Rows | Future treatment |
| --- | ---: | --- |
| `MIGRATE` | 168 | `CREATE_FROM_VERIFIED_KNOWLEDGE` or `MERGE_INTO_TARGET` after final source review |
| `MERGE` | 117 | `MERGE_INTO_TARGET` through the 13 group destinations below |
| `REGENERATE` | 35 | `REGENERATE_FROM_REPOSITORY` for source-sensitive catalogs/procedures |
| `PRESERVE_HISTORICAL` | 19 | `PRESERVE_AS_HISTORY` in `history/` or an explicit Decision Concept |
| `DELETE_LEGACY_ONLY` | 14 | `OMIT_AFTER_VERIFIED_REDUNDANCY`; navigation-only rows are never deleted first |
| **Total** | **353** | **353 / 353 mapped** |

The Phase 1 review overlay remains separate: 75 rows require final source or
owner review and 278 do not. The overlay does not change the primary action.

## Semantic merge-group mapping

Every one of the 13 Phase 1 groups is represented. The counts below are the
document rows in the Phase 1 matrix, not target Concept counts. The disposition
columns use the order `MIGRATE`, `MERGE`, `REGENERATE`, `PRESERVE_HISTORICAL`,
and `DELETE_LEGACY_ONLY`.

| Group | Domain | Rows | Disposition counts | Target Concepts | Build rationale and blockers |
| --- | --- | ---: | --- | --- | --- |
| MERGE-GROUP-001 | Project overview, README, glossary, navigation, repository map | 5 | 1 / 2 / 1 / 0 / 1 | TARGET-001 to TARGET-005 | Merge identity and navigation; regenerate evidence routing; NOC-13 and NOC-18 affect durable terminology/ownership. |
| MERGE-GROUP-002 | Architecture, boundaries, components, decisions, diagrams | 10 | 7 / 0 / 0 / 1 / 2 | TARGET-006 to TARGET-009 | Keep runtime boundaries separate from side-effect and decision history; NOC-01, NOC-06, NOC-09, and NOC-13 limit policy claims. |
| MERGE-GROUP-003 | Backend overview, modules, persistence, services, tests | 31 | 10 / 20 / 0 / 0 / 1 | TARGET-010 to TARGET-013 | Consolidate module views; regenerate validation evidence; ownership remains unresolved under NOC-15. |
| MERGE-GROUP-004 | API overview, endpoint families, routes, contracts | 57 | 33 / 20 / 3 / 0 / 1 | TARGET-018 to TARGET-022 plus linked security/domain Concepts | Regenerate exhaustive routes; preserve contract boundaries; NOC-08 compatibility and NOC-10 Marketplace scope remain open. |
| MERGE-GROUP-005 | Database guides, entities, schema, relationships, RLS | 34 | 12 / 18 / 3 / 0 / 1 | TARGET-023 to TARGET-026 and TARGET-029 | Regenerate schema/entity views; separate code-level RLS from deployed state; NOC-03 and NOC-05 block recovery/retention claims. |
| MERGE-GROUP-006 | Security, authentication, roles, permissions, tenancy | 40 | 19 / 20 / 0 / 0 / 1 | TARGET-027 to TARGET-032 | Preserve independent security boundaries; exclude contradicted/stale negatives; NOC-01, NOC-02, NOC-04, NOC-05, and NOC-15 remain relevant. |
| MERGE-GROUP-007 | Domain, workflows, tenancy, billing, delivery | 44 | 31 / 11 / 1 / 0 / 1 | TARGET-033 to TARGET-038 and TARGET-036 | Merge by verified capability; separate observed delivery from intended routing; NOC-01, NOC-02, NOC-05, NOC-09, NOC-10, NOC-11, and NOC-12 affect content. |
| MERGE-GROUP-008 | Frontend architecture, features, state, API, testing | 31 | 14 / 15 / 1 / 0 / 1 | TARGET-014 to TARGET-017 and linked API/security Concepts | Keep frontend retrieval units separate from backend authority; regenerate quality catalog; NOC-07, NOC-12, and NOC-14 remain open. |
| MERGE-GROUP-009 | Plugins, extension points, Marketplace runtime | 39 | 36 / 2 / 0 / 0 / 1 | TARGET-022, TARGET-038 to TARGET-040 | Separate trusted in-process plugins from Marketplace adapters and package safety; NOC-10 and NOC-17 remain open. |
| MERGE-GROUP-010 | Operations, Docker/deployment, CI, release, recovery | 26 | 2 / 3 / 19 / 0 / 2 | TARGET-041 to TARGET-044 and linked security Concepts | Regenerate repository topology and CI; block production deployment/recovery/observability claims under NOC-02 through NOC-06 and NOC-15. |
| MERGE-GROUP-011 | Development, maintenance, governance, testing | 21 | 2 / 6 / 7 / 4 / 2 | TARGET-013, TARGET-017, TARGET-042, TARGET-045 to TARGET-049 | Merge workflow guidance, regenerate current gates, preserve conflict/owner history; NOC-13 through NOC-15 remain open. |
| MERGE-GROUP-012 | Bootstrap audit and planning series | 14 | 0 / 0 / 0 / 14 / 0 | TARGET-048 and TARGET-050 to TARGET-054 | Preserve the original audit reasoning, gaps, owner questions, and proposed structure as historical evidence only. |
| MERGE-GROUP-013 | Legacy source register and provenance lineage | 1 | 1 / 0 / 0 / 0 / 0 | TARGET-005 | Translate source semantics to official `sources` and immutable URLs; no central legacy registry survives. |
| **Total** |  | **353** | **168 / 117 / 35 / 19 / 14** | **All 54 target Concepts covered** | **All 13 groups accounted for** |

## Legacy candidate-label crosswalk

The Phase 1 matrix's `Future Concept Candidate` column contains 24 labels.
These counts sum to all 353 rows. Multiple labels may resolve to the same
target Concept because the candidate column is a source-side classification,
not a target taxonomy.

| Phase 1 candidate label | Rows | Target destination(s) |
| --- | ---: | --- |
| API | 57 | TARGET-018 to TARGET-022 |
| Architecture | 10 | TARGET-006 to TARGET-009 |
| Authentication | 8 | TARGET-020, TARGET-027 |
| Authorization | 20 | TARGET-028, TARGET-029 |
| Backend | 31 | TARGET-010 to TARGET-013 |
| BillingAndQuotas | 2 | TARGET-037 |
| ContentWorkflow | 25 | TARGET-033, with links to TARGET-034 and TARGET-036 |
| Database | 34 | TARGET-023 to TARGET-026 |
| DecisionDebt | 3 | TARGET-048 |
| DevelopmentTesting | 11 | TARGET-013, TARGET-017, TARGET-045, TARGET-046 |
| Extensibility | 28 | TARGET-039, TARGET-040 |
| Frontend | 31 | TARGET-014 to TARGET-017 |
| HistoricalAudit | 13 | TARGET-050 to TARGET-054 |
| MaintenanceGovernance | 8 | TARGET-004, TARGET-047, TARGET-049 |
| Marketplace | 15 | TARGET-022, TARGET-038, TARGET-040 |
| Media | 2 | TARGET-035, TARGET-031 |
| OperationsDeployment | 26 | TARGET-041 to TARGET-044 |
| PageBuilder | 4 | TARGET-034 |
| Project | 5 | TARGET-001 to TARGET-005 |
| ProvenanceAndAudit | 1 | TARGET-005, with history links to TARGET-051 |
| PublicDelivery | 2 | TARGET-021, TARGET-036 |
| SecurityPosture | 12 | TARGET-031, TARGET-032 |
| TenantIsolation | 5 | TARGET-025, TARGET-029, TARGET-036 |
| **Total** | **353** | **Every Phase 1 candidate row has a destination** |

## Claim-status treatment

The 60 material claims are not treated as if document disposition were claim
verification. A single legacy file can supply both current and historical
claims.

| Claim status | Count | Target treatment |
| --- | ---: | --- |
| `VERIFIED_CURRENT` | 20 | May seed stable current Concepts after source snapshot review. |
| `VERIFIED_HISTORICAL` | 6 | Move to history/provenance context; never use as current baseline. |
| `PARTIALLY_VERIFIED` | 12 | Split observed behavior from policy/deployment gaps; current portion may be draft/stable with caveats. |
| `STALE` | 7 | Preserve as history or regenerate; exclude stale phase/count/negative claims from current content. |
| `CONTRADICTED` | 2 | Do not migrate the contradicted assertion; record the correction boundary in history/claim evidence and rebuild from current source. |
| `UNVERIFIED` | 8 | Keep as explicit evidence gaps or blocked Concepts; never promote to stable fact. |
| `NON_FACTUAL` | 5 | Represent as decision/planning/history context, not implementation truth. |
| **Total** | **60** | **All claim statuses have an explicit treatment** |

The most important corrections are the stale negative MFA/session claims and
the contradicted bearer-required logout claim. Current auth source and tests
are authoritative for the future authentication Concepts.

## Diagram reconciliation

The Phase 1 map contains all 50 legacy `.mmd` paths. The future bundle embeds
Mermaid in Concept bodies; it does not copy `.mmd` files into `/okf/`.

### Preserve: 19 unique visual relationships

Each path below remains a distinct visual block in its owning Concept after
source/render review:

| Legacy diagram | Owning target Concept |
| --- | --- |
| `okf/architecture/diagrams/container-view.mmd` | TARGET-006 |
| `okf/architecture/diagrams/system-context.mmd` | TARGET-006 |
| `okf/database/diagrams/entity-relationship-overview.mmd` | TARGET-024 |
| `okf/database/diagrams/migration-lifecycle.mmd` | TARGET-023 |
| `okf/database/diagrams/tenant-isolation.mmd` | TARGET-025 |
| `okf/delivery/diagrams/ci-pipeline.mmd` | TARGET-042 |
| `okf/domain/diagrams/content-lifecycle.mmd` | TARGET-033 |
| `okf/domain/diagrams/page-builder-workflow.mmd` | TARGET-034 |
| `okf/domain/diagrams/publication-workflow.mmd` | TARGET-036 |
| `okf/domain/diagrams/tenant-membership-workflow.mmd` | TARGET-029 |
| `okf/extensibility/diagrams/marketplace-installation-flow.mmd` | TARGET-038 |
| `okf/extensibility/diagrams/plugin-data-ownership.mmd` | TARGET-039 |
| `okf/frontend/diagrams/page-builder-flow.mmd` | TARGET-034 |
| `okf/security/diagrams/authentication-flow.mmd` | TARGET-027 |
| `okf/security/diagrams/authorization-decision-flow.mmd` | TARGET-028 |
| `okf/security/diagrams/rbac-model.mmd` | TARGET-028 |
| `okf/security/diagrams/session-token-lifecycle.mmd` | TARGET-027 |
| `okf/security/diagrams/tenant-access-control.mmd` | TARGET-029 |
| `okf/security/diagrams/trust-boundaries.mmd` | TARGET-032 |

### Merge: 19 legacy diagrams into 8 future visual blocks

| Future visual | Legacy diagrams | Owning target |
| --- | --- | --- |
| DG-MERGE-01 API contract boundary | `api-route-map.mmd`, `authentication-flow.mmd`, `authorization-flow.mmd`, `frontend-api-contract-flow.mmd` | TARGET-018 |
| DG-MERGE-02 architecture dependency boundary | `dependency-direction.mmd`, `frontend-backend-flow.mmd` | TARGET-007 |
| DG-MERGE-03 backend composition | `application-state-composition.mmd`, `backend-dependency-flow.mmd`, `backend-module-map.mmd` | TARGET-011 |
| DG-MERGE-04 database ownership/domain | `database-domain-map.mmd`, `module-data-ownership.mmd` | TARGET-024 |
| DG-MERGE-05 domain orchestration | `cross-module-orchestration.mmd`, `domain-map.mmd` | TARGET-033 |
| DG-MERGE-06 extension context and registration | `extensibility-context.mmd`, `plugin-permission-flow.mmd`, `plugin-registration-flow.mmd` | TARGET-040 |
| DG-MERGE-07 frontend application flow | `frontend-api-flow.mmd`, `frontend-application-map.mmd` | TARGET-015 |
| DG-MERGE-08 reference operations topology | `runtime-topology.mmd` | TARGET-041 |

All paths in this table are under their Phase 1 domain directory as recorded
in [DIAGRAM_PRESERVATION_MAP.md](DIAGRAM_PRESERVATION_MAP.md).

### Regenerate: 11 legacy diagrams into 6 future visual blocks

| Future visual | Legacy diagrams | Owning target |
| --- | --- | --- |
| DG-REGEN-01 API request lifecycle | `okf/api/diagrams/api-request-lifecycle.mmd` | TARGET-019 |
| DG-REGEN-02 backend request lifecycle | `okf/backend/diagrams/backend-request-lifecycle.mmd` | TARGET-010 |
| DG-REGEN-03 deployment and release flow | `okf/delivery/diagrams/deployment-flow.mmd`, `okf/delivery/diagrams/release-flow.mmd` | TARGET-042 |
| DG-REGEN-04 extension lifecycle and registration | `okf/extensibility/diagrams/component-registration.mmd`, `okf/extensibility/diagrams/plugin-lifecycle.mmd` | TARGET-040 |
| DG-REGEN-05 frontend routing and state | `okf/frontend/diagrams/frontend-routing-flow.mmd`, `okf/frontend/diagrams/frontend-state-flow.mmd` | TARGET-015 |
| DG-REGEN-06 operations evidence flow | `okf/operations/diagrams/backup-restore-flow.mmd`, `okf/operations/diagrams/health-check-flow.mmd`, `okf/operations/diagrams/observability-flow.mmd` | TARGET-043 and TARGET-044 |

### Drop: 1 navigation-redundant diagram

`okf/architecture/diagrams/backend-request-flow.mmd` is the one DROP candidate.
Its request sequence is covered by the architecture/runtime and backend
request-lifecycle targets. The path is not deleted in Phase 2; its reason for
omission is preserved here and must be checked after the replacement visuals
are verified.

The final visual estimate is 19 preserved blocks + 8 merged blocks + 6
regenerated blocks = **33 embedded Mermaid visual blocks**.

## Owner-decision crosswalk

Open decisions are content blockers, not reasons to redesign unrelated parts of
the bundle. The detailed questions remain in
[OWNER_DECISION_REGISTER.md](OWNER_DECISION_REGISTER.md).

| NOC | Affected target Concepts | Required future evidence |
| --- | --- | --- |
| NOC-01 | TARGET-007, 021, 025, 029, 036 | Owner-approved public tenant/host/custom-domain routing rule and tests/configuration |
| NOC-02 | TARGET-012, 031, 035, 041 | Storage architecture, asset classes, authorization, and deployment configuration |
| NOC-03 | TARGET-023, 024, 026, 043 | Applied schema, backup/restore policy, RPO/RTO, retention, and restore evidence |
| NOC-04 | TARGET-032, 044 | Production monitoring, SLOs, alert ownership, and escalation record |
| NOC-05 | TARGET-024, 025, 026, 031, 032, 035 | Approved privacy, retention, residency, audit, and deletion/legal-hold policy |
| NOC-06 | TARGET-009, 023, 041, 042, 043 | Environment matrix, promotion, release, rollback, and accountable owner |
| NOC-07 | TARGET-013, 017, 041, 045 | Published support matrix and update policy |
| NOC-08 | TARGET-018, 019, 020, 021 | Versioning, deprecation, compatibility, and support-window policy |
| NOC-09 | TARGET-008, 021, 036, 037 | Retry, compensation, idempotency, and user-visible side-effect policy |
| NOC-10 | TARGET-022, 038, 039, 040 | Marketplace scope, settlement, disputes, external execution, and cleanup policy |
| NOC-11 | TARGET-007, 025, 036, 041 | Domain-verification lifecycle and routing integration evidence |
| NOC-12 | TARGET-015, 016, 020, 027, 033, 034, 039 | Approved schema/workflow, session, page compatibility, accessibility, and extension policies |
| NOC-13 | TARGET-003, 004, 005, 009, 047 | Canonical document/owner map, retirement criteria, and review triggers |
| NOC-14 | TARGET-013, 017, 042, 045, 046, 047 | Contribution policy and enforced checks |
| NOC-15 | TARGET-004, 011, 028, 032, 044, 046, 047 | Accountable module, operations, security, and documentation ownership map |
| NOC-16 | TARGET-001 and TARGET-050/TARGET-052 historical sections | Existing `LICENSE` and aligned package metadata; no open blocker |
| NOC-17 | TARGET-038, TARGET-040, TARGET-047 | Marketplace archive retention/regeneration policy |
| NOC-18 | TARGET-001, TARGET-002, TARGET-016 | Owner-approved terminology/glossary |

The 15 UNKNOWN labels are retained through the same register and attached to
their NOC rows; none is converted into a type, status, source, or fact.

## Reconciliation contract for future construction

Before any legacy removal, a future validator must prove:

- matrix rows = 353 and every row has a target ID/action;
- disposition totals remain 168/117/35/19/14;
- all 13 merge groups are present with matching row totals;
- all 50 diagram paths are present with one of PRESERVE/MERGE/REGENERATE/DROP;
- all 60 claim statuses have a current/history/decision treatment;
- all 18 NOC records are present, with 17 still open unless new owner evidence
  resolves them;
- all 54 catalog target paths reconcile with the target tree;
- no target Concept is created solely from an unsupported legacy claim.

This contract is intentionally stronger than Google OKF conformance because it
guards knowledge preservation during a repository migration.
