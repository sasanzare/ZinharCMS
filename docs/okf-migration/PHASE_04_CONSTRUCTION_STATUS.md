# Phase 4 Construction Status Matrix

**Source HEAD:** `e37e94e2e6960a2547f33bf1ebb4225f818b3a4b`  
**Phase 3 baseline:** 19 Concepts, 13 indexes, 1 root log, 13 embedded Mermaid blocks, 35 deferred targets  
**Phase 4 scope:** `TARGET-008`, `TARGET-012`, and `TARGET-016` only

This matrix is the authoritative Phase 4 disposition for every Phase 2 target.
`BUILT_PHASE_3` identifies a Concept carried forward unchanged from Phase 3;
`BUILT_PHASE_4` identifies a Concept constructed in this phase. Deferred rows
remain represented only in this migration record and are not represented by
placeholder links in the staging bundle.

| ID | Path | Domain | Previous | Phase 4 | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| TARGET-001 | `project/project-overview.md` | project | BUILT | BUILT_PHASE_3 | README, LICENSE, release scope; CLAIM-0004, CLAIM-0006 | High confidence for repository scope; NOC-18 terminology caveat is stated, not promoted. |
| TARGET-002 | `project/terminology-and-glossary.md` | project | BLOCKED_OWNER_DECISION | BLOCKED_OWNER_DECISION | UI/code labels require owner glossary | Owner-approved terms are unavailable; NOC-18. |
| TARGET-003 | `project/repository-and-evidence-map.md` | project | DEFERRED_REGENERATE | DEFERRED_REGENERATE | Repository tree, evidence hierarchy, Phase 1 manifest | Requires deterministic repository regeneration and canonical authority decision; NOC-13. |
| TARGET-004 | `project/documentation-governance.md` | project | BLOCKED_OWNER_DECISION | BLOCKED_OWNER_DECISION | HANDOFF, owner register, Git workflow | Ownership and review authority are unresolved; NOC-13, NOC-15. |
| TARGET-005 | `project/source-lineage.md` | project | DEFERRED_REGENERATE | DEFERRED_REGENERATE | Phase 1 manifest and immutable source paths | Requires complete source-register regeneration and durable ownership; NOC-13, NOC-15. |
| TARGET-006 | `architecture/system-architecture.md` | architecture | BUILT | BUILT_PHASE_3 | backend main/lib, frontend main, Compose; CLAIM-0004, CLAIM-0027 | Current modular-monolith repository boundary is directly observed; deployment details are excluded. |
| TARGET-007 | `architecture/runtime-and-request-boundaries.md` | architecture | BUILT | BUILT_PHASE_3 | route tree, middleware, pages preview, delivery; CLAIM-0005, CLAIM-0011, CLAIM-0019 | Current request layers are direct; intended public tenant routing remains NOC-01. |
| TARGET-008 | `architecture/integrations-and-side-effects.md` | architecture | DEFERRED_MERGE | BUILT_PHASE_4 | Services, cache, webhook, email, file, transaction code | Constructed as one current side-effect model; NOC-02 and NOC-09 remain explicit. |
| TARGET-009 | `architecture/architecture-decision-records.md` | architecture | DEFERRED_REGENERATE | DEFERRED_REGENERATE | Current source, Phase 1 claims, Phase 2 ADRs | Requires source-derived ADR regeneration and canonical authority; NOC-13. |
| TARGET-010 | `backend/backend-runtime.md` | backend | BUILT | BUILT_PHASE_3 | Cargo, main/lib, state, db, error; CLAIM-0004, CLAIM-0006 | Process and shared-state boundary is directly observed. |
| TARGET-011 | `backend/module-boundaries.md` | backend | BUILT | BUILT_PHASE_3 | routes, middleware, services, plugins; CLAIM-0004, CLAIM-0017 | Current module structure is direct; durable ownership is NOC-15 and is explicitly excluded. |
| TARGET-012 | `backend/persistence-services-and-configuration.md` | backend | DEFERRED_MERGE | BUILT_PHASE_4 | db, state, config, service composition; CLAIM-0006 | Constructed as the current persistence/configuration view; NOC-02 limits storage claims. |
| TARGET-013 | `backend/backend-validation.md` | backend | DEFERRED_REGENERATE | DEFERRED_REGENERATE | backend tests, Cargo, backend CI; CLAIM-0020 | Exhaustive validation inventory requires regeneration; NOC-07, NOC-14. |
| TARGET-014 | `frontend/admin-application.md` | frontend | BUILT | BUILT_PHASE_3 | package, main, router; CLAIM-0004, CLAIM-0014 | React/Vite shell and route surface are directly observed. |
| TARGET-015 | `frontend/routing-and-state.md` | frontend | BUILT | BUILT_PHASE_3 | router, API client, store; CLAIM-0014 | Current behavior is direct; compatibility/accessibility policy is NOC-12. |
| TARGET-016 | `frontend/feature-boundaries.md` | frontend | DEFERRED_MERGE | BUILT_PHASE_4 | pages, components, i18n, feature routes; CLAIM-0014 | Constructed as a current page/API feature map; NOC-12 and NOC-18 remain explicit. |
| TARGET-017 | `frontend/frontend-quality-and-testing.md` | frontend | DEFERRED_REGENERATE | DEFERRED_REGENERATE | package scripts, tests, frontend CI; CLAIM-0020 | Requires deterministic complete test/build inventory; NOC-07, NOC-14. |
| TARGET-018 | `api/api-contract-overview.md` | api | BUILT | BUILT_PHASE_3 | route families, OpenAPI route, frontend API; CLAIM-0012, CLAIM-0013, CLAIM-0014 | Family-level current surface is safe; compatibility/versioning is NOC-08. |
| TARGET-019 | `api/route-surface.md` | api | DEFERRED_REGENERATE | DEFERRED_REGENERATE | route modules and annotations; CLAIM-0012, CLAIM-0013 | Exhaustive endpoint output must be regenerated from current source; NOC-08. |
| TARGET-020 | `api/authentication-and-session-contract.md` | api | BUILT | BUILT_PHASE_3 | auth routes/services, migrations 0027-0029; CLAIM-0009, CLAIM-0010 | Current behavior is source-backed; compatibility and policy are NOC-08, NOC-12. |
| TARGET-021 | `api/public-delivery-and-webhooks-contract.md` | api | DEFERRED_MERGE | DEFERRED_MERGE | delivery routes, webhook/cache services; CLAIM-0019, partial delivery evidence | Delivery and side-effect views require merged current contract; NOC-01, NOC-08, NOC-09. |
| TARGET-022 | `api/marketplace-and-extension-contracts.md` | api | DEFERRED_MERGE | DEFERRED_MERGE | Marketplace route modules, API client; CLAIM-0015, CLAIM-0017 | Multiple contracts require one coherent current merge; NOC-10. |
| TARGET-023 | `database/schema-and-migrations.md` | database | DEFERRED_REGENERATE | DEFERRED_REGENERATE | migrations and SQLx startup; CLAIM-0028 | Requires exhaustive source regeneration and deployed-state separation; NOC-03, NOC-06. |
| TARGET-024 | `database/entities-and-relationships.md` | database | DEFERRED_REGENERATE | DEFERRED_REGENERATE | migrations, models, queries, tests; CLAIM-0016, CLAIM-0028 | Requires verified relationship inventory and preserved ER visual reconciliation; NOC-03, NOC-05. |
| TARGET-025 | `database/tenant-data-policy.md` | database | DEFERRED_MERGE | DEFERRED_MERGE | RLS service, migration 0009, tests; CLAIM-0008 | Current controls are built in the dedicated Security Concept; database merge waits on routing/privacy policy; NOC-01, NOC-05. |
| TARGET-026 | `database/data-lifecycle-and-retention.md` | database | BLOCKED_OWNER_DECISION | BLOCKED_OWNER_DECISION | Current schema plus owner/legal policy required | Legal, deletion, hold, and retention authority is unresolved; NOC-05. |
| TARGET-027 | `security/authentication-and-sessions.md` | security | BUILT | BUILT_PHASE_3 | auth routes/services, security migrations/tests; CLAIM-0009, CLAIM-0010 | Current controls are directly observed; long-term session/accessibility policy remains NOC-12. |
| TARGET-028 | `security/authorization-and-rbac.md` | security | BUILT | BUILT_PHASE_3 | RBAC, auth/tenant middleware, protected routes; CLAIM-0007 | Current checks are direct; ownership and support boundaries are NOC-15. |
| TARGET-029 | `security/tenant-isolation.md` | security | BUILT | BUILT_PHASE_3 | tenant middleware, RLS service/migration/tests; CLAIM-0007, CLAIM-0008 | Layered controls are direct; no live deployment proof and NOC-01 routing caveat. |
| TARGET-030 | `security/preview-security.md` | security | BUILT | BUILT_PHASE_3 | pages route, preview tickets, config; CLAIM-0011 | Ticket, origin, protocol, and revalidation controls are directly observed. |
| TARGET-031 | `security/storage-and-file-security.md` | security | DEFERRED_MERGE | DEFERRED_MERGE | file services, media routes, migration 0030, tests; CLAIM-0018 | Several security/storage views require merge and owner retention policy; NOC-02, NOC-05. |
| TARGET-032 | `security/security-posture-and-risks.md` | security | DEFERRED_MERGE | DEFERRED_MERGE | security middleware/services/tests; CLAIM-0018, CLAIM-0029 | Requires merged posture/risk evidence and operations/ownership decisions; NOC-04, NOC-05, NOC-15. |
| TARGET-033 | `domain/content-and-editorial-workflow.md` | domain | BUILT | BUILT_PHASE_3 | content routes/services, workflow, comments, plugins | Current workflow and hooks are directly observed; schema/accessibility policy is NOC-12. |
| TARGET-034 | `domain/page-builder-and-preview.md` | domain | BUILT | BUILT_PHASE_3 | pages routes/services, preview, frontend routes; CLAIM-0011 | Current page/preview path is direct; compatibility policy is NOC-12. |
| TARGET-035 | `domain/media-and-file-storage.md` | domain | DEFERRED_REGENERATE | DEFERRED_REGENERATE | media routes/services, file security, migration 0030; CLAIM-0018 | Requires current source regeneration across media and file controls; NOC-02, NOC-05. |
| TARGET-036 | `domain/public-delivery.md` | domain | DEFERRED_MERGE | DEFERRED_MERGE | delivery routes, cache, organization selection; CLAIM-0019 | Merge with API/webhook and side-effect boundaries; NOC-01, NOC-09. |
| TARGET-037 | `domain/billing-and-quotas.md` | domain | DEFERRED_MERGE | DEFERRED_MERGE | billing routes/services, quota service, migrations | Needs merged current limits and provider/failure policy; NOC-09. |
| TARGET-038 | `domain/marketplace.md` | domain | BUILT | BUILT_PHASE_3 | Marketplace routes/services/migrations 0015-0026; CLAIM-0015, CLAIM-0016 | Current capability and non-execution boundary are direct; roadmap/settlement is NOC-10. |
| TARGET-039 | `domain/extensibility-and-built-in-plugins.md` | domain | BUILT | BUILT_PHASE_3 | plugins, plugin routes, runtime adapters; CLAIM-0017 | Current in-process/plugin boundary is direct; Marketplace and compatibility policy is NOC-10, NOC-12. |
| TARGET-040 | `domain/marketplace-runtime-and-safety-boundary.md` | domain | DEFERRED_MERGE | DEFERRED_MERGE | runtime validation/policy/services/tests; CLAIM-0017 | Merge with Marketplace and extension concepts after policy review; NOC-10. |
| TARGET-041 | `operations/local-and-reference-topology.md` | operations | DEFERRED_REGENERATE | DEFERRED_REGENERATE | Compose, Dockerfiles, config, README; CLAIM-0027 | Requires source regeneration and explicit reference-vs-production boundary; NOC-02, NOC-06. |
| TARGET-042 | `operations/ci-and-release-gates.md` | operations | DEFERRED_REGENERATE | DEFERRED_REGENERATE | CI workflows, package scripts, release docs; CLAIM-0020 | Requires deterministic current gate inventory and release/promotion policy; NOC-06, NOC-14. |
| TARGET-043 | `operations/deployment-and-recovery.md` | operations | BLOCKED_OWNER_DECISION | BLOCKED_OWNER_DECISION | Owner/platform evidence required; Compose is reference only; CLAIM-0027 | Production environments, backup, restore, RPO, and RTO are not evidenced; NOC-03, NOC-06. |
| TARGET-044 | `operations/observability-and-support.md` | operations | BLOCKED_OWNER_DECISION | BLOCKED_OWNER_DECISION | health/tracing/source logs only; CLAIM-0029 | Production collectors, SLOs, dashboards, on-call, and escalation lack owner evidence; NOC-04, NOC-15. |
| TARGET-045 | `development/development-and-testing.md` | development | BUILT | BUILT_PHASE_3 | manifests, tests, README, CI; CLAIM-0020 | Current commands and CI gates are direct; support and contribution policy remain NOC-07, NOC-14. |
| TARGET-046 | `development/contribution-and-change-validation.md` | development | BLOCKED_OWNER_DECISION | BLOCKED_OWNER_DECISION | CI, Git history, owner policy | Review, migration, and documentation authority is unresolved; NOC-14, NOC-15. |
| TARGET-047 | `development/documentation-maintenance.md` | development | DEFERRED_MERGE | DEFERRED_MERGE | Phase 1 artifacts, HANDOFF, Git state | Requires merge of documentation-maintenance views and canonical ownership; NOC-13, NOC-14. |
| TARGET-048 | `decisions/owner-decision-debt.md` | decisions | BLOCKED_OWNER_DECISION | BLOCKED_OWNER_DECISION | owner register and Phase 1 manifest | Decision debt is not resolved by documentation; owner resolutions are required for NOC-01 through NOC-15, NOC-17, NOC-18. |
| TARGET-049 | `decisions/migration-and-architecture-decisions.md` | decisions | BUILT | BUILT_PHASE_3 | Phase 2 ADRs and official OKF baseline; CLAIM-0024, CLAIM-0025 | Phase 2 choices are complete and safe to record as bounded migration decisions. |
| TARGET-050 | `history/phase-0-baseline.md` | history | HISTORICAL_DEFERRED | HISTORICAL_DEFERRED | Phase 0 audit artifacts; CLAIM-0023, CLAIM-0024 | Historical context only; preserve separately from current knowledge. |
| TARGET-051 | `history/phase-1-preservation.md` | history | HISTORICAL_DEFERRED | HISTORICAL_DEFERRED | Phase 1 report and evidence manifest | Historical context only; preserve separately from current knowledge. |
| TARGET-052 | `history/bootstrap-audit-record.md` | history | HISTORICAL_DEFERRED | HISTORICAL_DEFERRED | 14 bootstrap reports and Phase 1 matrix | Historical context only; do not promote bootstrap snapshot claims. |
| TARGET-053 | `history/legacy-conflict-and-completion-record.md` | history | HISTORICAL_DEFERRED | HISTORICAL_DEFERRED | legacy snapshot and stale/contradiction claims | Historical context only; current truth must come from source evidence. |
| TARGET-054 | `history/legacy-structure-and-sequencing.md` | history | HISTORICAL_DEFERRED | HISTORICAL_DEFERRED | bootstrap structure/phase records and Google baseline | Historical context only; no legacy sequencing is treated as final target authority. |

## Phase 4 totals

| Phase 4 status | Count |
| --- | ---: |
| BUILT_PHASE_3 | 19 |
| BUILT_PHASE_4 | 3 |
| UPDATED_PHASE_4 | 0 |
| DEFERRED_MERGE | 9 |
| DEFERRED_REGENERATE | 11 |
| BLOCKED_OWNER_DECISION | 7 |
| HISTORICAL_DEFERRED | 5 |
| INSUFFICIENT_EVIDENCE | 0 |
| OUT_OF_SCOPE | 0 |
| **Total** | **54** |
