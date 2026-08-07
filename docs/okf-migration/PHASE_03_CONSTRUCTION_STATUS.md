# Phase 3 Construction Status Matrix

**Source HEAD:** `b58840e9c227ff9d937b482eced5331122291f82`
**Catalog size:** 54 targets
**Constructed:** 19
**Deferred:** 35

This matrix is the authoritative Phase 3 disposition for every Phase 2 target.
`BUILT` means a bounded Concept was constructed in the isolated staging root.
Other statuses are controlled deferrals; no deferred row is represented by a
placeholder Concept link.

| Target ID | Target path | Type | Evidence and Phase 1 claims | Legacy / Phase 1 inputs | Confidence, blocker, or safe-construction reason | Status |
| --- | --- | --- | --- | --- | --- | --- |
| TARGET-001 | `project/project-overview.md` | Project | README, LICENSE, release scope; CLAIM-0004, CLAIM-0006 | Project/README views; G001 | High confidence for repository scope; NOC-18 terminology caveat is stated, not promoted. | BUILT |
| TARGET-002 | `project/terminology-and-glossary.md` | Reference | UI/code labels require owner glossary | Project glossary; G001 | Owner-approved terms are unavailable; NOC-18. | BLOCKED_OWNER_DECISION |
| TARGET-003 | `project/repository-and-evidence-map.md` | Reference | Repository tree, evidence hierarchy, Phase 1 manifest | Project map/navigation; G001, G013 | Requires deterministic repository regeneration and canonical authority decision; NOC-13. | DEFERRED_REGENERATE |
| TARGET-004 | `project/documentation-governance.md` | Operational Guide | HANDOFF, owner register, Git workflow | Governance and maintenance views; G001, G011 | Ownership and review authority are unresolved; NOC-13, NOC-15. | BLOCKED_OWNER_DECISION |
| TARGET-005 | `project/source-lineage.md` | Reference | Phase 1 manifest and immutable source paths | Source register; G013 | Requires complete source-register regeneration and durable ownership; NOC-13, NOC-15. | DEFERRED_REGENERATE |
| TARGET-006 | `architecture/system-architecture.md` | Architecture | backend main/lib, frontend main, Compose; CLAIM-0004, CLAIM-0027 | Architecture overview/components; G002 | Current modular-monolith repository boundary is directly observed; deployment details are excluded. | BUILT |
| TARGET-007 | `architecture/runtime-and-request-boundaries.md` | Architecture | route tree, middleware, pages preview, delivery; CLAIM-0005, CLAIM-0011, CLAIM-0019 | Architecture/API boundary views; G002, G004, G006 | Current request layers are direct; intended public tenant routing remains NOC-01. | BUILT |
| TARGET-008 | `architecture/integrations-and-side-effects.md` | Architecture | Services, cache, webhook, email, file, transaction code | Architecture integration/risk views; G002 | Multiple legacy views need one current side-effect model; NOC-09. | DEFERRED_MERGE |
| TARGET-009 | `architecture/architecture-decision-records.md` | Architecture | Current source, Phase 1 claims, Phase 2 ADRs | Architecture decisions/conflicts; G002, G011 | Requires source-derived ADR regeneration and canonical authority; NOC-13. | DEFERRED_REGENERATE |
| TARGET-010 | `backend/backend-runtime.md` | Component | Cargo, main/lib, state, db, error; CLAIM-0004, CLAIM-0006 | Backend runtime/overview; G003 | Process and shared-state boundary is directly observed. | BUILT |
| TARGET-011 | `backend/module-boundaries.md` | Component | routes, middleware, services, plugins; CLAIM-0004, CLAIM-0017 | Backend module maps/catalogs; G003 | Current module structure is direct; durable ownership is NOC-15 and is explicitly excluded. | BUILT |
| TARGET-012 | `backend/persistence-services-and-configuration.md` | Component | db, state, config, service composition; CLAIM-0006 | Backend persistence/configuration; G003, G005 | Needs a merged persistence/configuration view and storage deployment caveat; NOC-02. | DEFERRED_MERGE |
| TARGET-013 | `backend/backend-validation.md` | Development Guide | backend tests, Cargo, backend CI; CLAIM-0020 | Backend testing/maintenance; G003, G011 | Exhaustive validation inventory requires regeneration; NOC-07, NOC-14. | DEFERRED_REGENERATE |
| TARGET-014 | `frontend/admin-application.md` | Component | package, main, router; CLAIM-0004, CLAIM-0014 | Frontend architecture/overview; G008 | React/Vite shell and route surface are directly observed. | BUILT |
| TARGET-015 | `frontend/routing-and-state.md` | Component | router, API client, store; CLAIM-0014 | Frontend routing/state/API views; G008, G006 | Current behavior is direct; compatibility/accessibility policy is NOC-12. | BUILT |
| TARGET-016 | `frontend/feature-boundaries.md` | Component | pages, components, i18n, feature routes | Frontend feature views; G008 | Several feature views require coherent merge and owner policy; NOC-12, NOC-18. | DEFERRED_MERGE |
| TARGET-017 | `frontend/frontend-quality-and-testing.md` | Development Guide | package scripts, tests, frontend CI; CLAIM-0020 | Frontend testing/risks; G008, G011 | Requires deterministic complete test/build inventory; NOC-07, NOC-14. | DEFERRED_REGENERATE |
| TARGET-018 | `api/api-contract-overview.md` | API Contract | route families, OpenAPI route, frontend API; CLAIM-0012, CLAIM-0013, CLAIM-0014 | API overview/groups; G004 | Family-level current surface is safe; compatibility/versioning is NOC-08. | BUILT |
| TARGET-019 | `api/route-surface.md` | API Endpoint | route modules and annotations; CLAIM-0012, CLAIM-0013 | Endpoint catalogs/maps; G004 | Exhaustive endpoint output must be regenerated from current source; NOC-08. | DEFERRED_REGENERATE |
| TARGET-020 | `api/authentication-and-session-contract.md` | API Contract | auth routes/services, migrations 0027-0029; CLAIM-0009, CLAIM-0010 | API auth/security session views; G004, G006 | Current behavior is source-backed; compatibility and policy are NOC-08, NOC-12. | BUILT |
| TARGET-021 | `api/public-delivery-and-webhooks-contract.md` | API Contract | delivery routes, webhook/cache services; CLAIM-0019, partial delivery evidence | Public delivery/webhook views; G004, G007 | Delivery and side-effect views require merged current contract; NOC-01, NOC-08, NOC-09. | DEFERRED_MERGE |
| TARGET-022 | `api/marketplace-and-extension-contracts.md` | API Contract | Marketplace route modules, API client; CLAIM-0015, CLAIM-0017 | Marketplace/plugin API views; G004, G009 | Multiple contracts require one coherent current merge; NOC-10. | DEFERRED_MERGE |
| TARGET-023 | `database/schema-and-migrations.md` | Data Model | migrations and SQLx startup; CLAIM-0028 | Database schema/migration catalogs; G005 | Requires exhaustive source regeneration and deployed-state separation; NOC-03, NOC-06. | DEFERRED_REGENERATE |
| TARGET-024 | `database/entities-and-relationships.md` | Data Model | migrations, models, queries, tests; CLAIM-0016, CLAIM-0028 | Database entity/relationship views; G005, G007, G009 | Requires verified relationship inventory and preserved ER visual reconciliation; NOC-03, NOC-05. | DEFERRED_REGENERATE |
| TARGET-025 | `database/tenant-data-policy.md` | Database Policy | RLS service, migration 0009, tests; CLAIM-0008 | Multi-tenancy/RLS views; G005, G006, G007 | Current controls are built in the dedicated Security Concept; database merge waits on routing/privacy policy; NOC-01, NOC-05. | DEFERRED_MERGE |
| TARGET-026 | `database/data-lifecycle-and-retention.md` | Database Policy | Current schema plus owner/legal policy required | Database/security lifecycle views; G005, G007, G011 | Legal, deletion, hold, and retention authority is unresolved; NOC-05. | BLOCKED_OWNER_DECISION |
| TARGET-027 | `security/authentication-and-sessions.md` | Authentication Flow | auth routes/services, security migrations/tests; CLAIM-0009, CLAIM-0010 | Security authentication/session views; G006 | Current controls are directly observed; long-term session/accessibility policy remains NOC-12. | BUILT |
| TARGET-028 | `security/authorization-and-rbac.md` | Authorization Policy | RBAC, auth/tenant middleware, protected routes; CLAIM-0007 | Roles/permissions/RBAC views; G006 | Current checks are direct; ownership and support boundaries are NOC-15. | BUILT |
| TARGET-029 | `security/tenant-isolation.md` | Security Control | tenant middleware, RLS service/migration/tests; CLAIM-0007, CLAIM-0008 | Tenant access/security/database views; G005, G006, G007 | Layered controls are direct; no live deployment proof and NOC-01 routing caveat. | BUILT |
| TARGET-030 | `security/preview-security.md` | Security Control | pages route, preview tickets, config; CLAIM-0011 | Preview/trust-boundary views; G004, G006, G007 | Ticket, origin, protocol, and revalidation controls are directly observed. | BUILT |
| TARGET-031 | `security/storage-and-file-security.md` | Security Control | file services, media routes, migration 0030, tests; CLAIM-0018 | Storage/media/Marketplace security views; G006, G007, G009 | Several security/storage views require merge and owner retention policy; NOC-02, NOC-05. | DEFERRED_MERGE |
| TARGET-032 | `security/security-posture-and-risks.md` | Security Control | security middleware/services/tests; CLAIM-0018, CLAIM-0029 | Security overview/threat/risk/testing views; G006, G010 | Requires merged posture/risk evidence and operations/ownership decisions; NOC-04, NOC-05, NOC-15. | DEFERRED_MERGE |
| TARGET-033 | `domain/content-and-editorial-workflow.md` | Domain Workflow | content routes/services, workflow, comments, plugins | Content/domain/backend/frontend workflows; G003, G007, G008 | Current workflow and hooks are directly observed; schema/accessibility policy is NOC-12. | BUILT |
| TARGET-034 | `domain/page-builder-and-preview.md` | Domain Workflow | pages routes/services, preview, frontend routes; CLAIM-0011 | Page builder/preview workflows; G004, G007, G008 | Current page/preview path is direct; compatibility policy is NOC-12. | BUILT |
| TARGET-035 | `domain/media-and-file-storage.md` | Resource Boundary | media routes/services, file security, migration 0030; CLAIM-0018 | Media/API/security views; G004, G006, G007 | Requires current source regeneration across media and file controls; NOC-02, NOC-05. | DEFERRED_REGENERATE |
| TARGET-036 | `domain/public-delivery.md` | Resource Boundary | delivery routes, cache, organization selection; CLAIM-0019 | Delivery domain/API views; G004, G007 | Merge with API/webhook and side-effect boundaries; NOC-01, NOC-09. | DEFERRED_MERGE |
| TARGET-037 | `domain/billing-and-quotas.md` | Domain Model | billing routes/services, quota service, migrations | Billing/quota views; G004, G005, G007 | Needs merged current limits and provider/failure policy; NOC-09. | DEFERRED_MERGE |
| TARGET-038 | `domain/marketplace.md` | Marketplace Domain | Marketplace routes/services/migrations 0015-0026; CLAIM-0015, CLAIM-0016 | Marketplace domain/backend/API/database views; G004, G005, G007, G009 | Current capability and non-execution boundary are direct; roadmap/settlement is NOC-10. | BUILT |
| TARGET-039 | `domain/extensibility-and-built-in-plugins.md` | Component | plugins, plugin routes, runtime adapters; CLAIM-0017 | Plugin/extension views; G003, G009 | Current in-process/plugin boundary is direct; Marketplace and compatibility policy is NOC-10, NOC-12. | BUILT |
| TARGET-040 | `domain/marketplace-runtime-and-safety-boundary.md` | Marketplace Domain | runtime validation/policy/services/tests; CLAIM-0017 | Marketplace runtime/extensibility/security views; G006, G009 | Merge with Marketplace and extension concepts after policy review; NOC-10. | DEFERRED_MERGE |
| TARGET-041 | `operations/local-and-reference-topology.md` | Operational Guide | Compose, Dockerfiles, config, README; CLAIM-0027 | Operations/deployment/technology views; G010 | Requires source regeneration and explicit reference-vs-production boundary; NOC-02, NOC-06. | DEFERRED_REGENERATE |
| TARGET-042 | `operations/ci-and-release-gates.md` | Operational Guide | CI workflows, package scripts, release docs; CLAIM-0020 | Delivery/CI/release views; G010, G011 | Requires deterministic current gate inventory and release/promotion policy; NOC-06, NOC-14. | DEFERRED_REGENERATE |
| TARGET-043 | `operations/deployment-and-recovery.md` | Operational Guide | Owner/platform evidence required; Compose is reference only; CLAIM-0027 | Operations/recovery views; G010 | Production environments, backup, restore, RPO, and RTO are not evidenced; NOC-03, NOC-06. | BLOCKED_OWNER_DECISION |
| TARGET-044 | `operations/observability-and-support.md` | Operational Guide | health/tracing/source logs only; CLAIM-0029 | Operations/maintenance risk views; G010, G011 | Production collectors, SLOs, dashboards, on-call, and escalation lack owner evidence; NOC-04, NOC-15. | BLOCKED_OWNER_DECISION |
| TARGET-045 | `development/development-and-testing.md` | Development Guide | manifests, tests, README, CI; CLAIM-0020 | Development/testing maps; G003, G008, G011 | Current commands and CI gates are direct; support and contribution policy remain NOC-07, NOC-14. | BUILT |
| TARGET-046 | `development/contribution-and-change-validation.md` | Development Guide | CI, Git history, owner policy | Contribution/maintenance views; G011 | Review, migration, and documentation authority is unresolved; NOC-14, NOC-15. | BLOCKED_OWNER_DECISION |
| TARGET-047 | `development/documentation-maintenance.md` | Development Guide | Phase 1 artifacts, HANDOFF, Git state | Maintenance/checklist/staleness views; G001, G011 | Requires merge of documentation-maintenance views and canonical ownership; NOC-13, NOC-14. | DEFERRED_MERGE |
| TARGET-048 | `decisions/owner-decision-debt.md` | Decision | owner register and Phase 1 manifest | Bootstrap gaps/owner questions; G011, G012 | Decision debt is not resolved by documentation; owner resolutions are required for NOC-01 through NOC-15, NOC-17, NOC-18. | BLOCKED_OWNER_DECISION |
| TARGET-049 | `decisions/migration-and-architecture-decisions.md` | Decision | Phase 2 ADRs and official OKF baseline; CLAIM-0024, CLAIM-0025 | Phase 0/1 migration decisions; G001, G002, G011 | Phase 2 choices are complete and safe to record as bounded migration decisions. | BUILT |
| TARGET-050 | `history/phase-0-baseline.md` | Historical Record | Phase 0 audit artifacts; CLAIM-0023, CLAIM-0024 | Phase 0 reports; G012 | Historical context only; preserve separately from current knowledge. | HISTORICAL_DEFERRED |
| TARGET-051 | `history/phase-1-preservation.md` | Historical Record | Phase 1 report and evidence manifest | Phase 1 reports; G012 | Historical context only; preserve separately from current knowledge. | HISTORICAL_DEFERRED |
| TARGET-052 | `history/bootstrap-audit-record.md` | Historical Record | 14 bootstrap reports and Phase 1 matrix | All `okf-bootstrap` audits; G012 | Historical context only; do not promote bootstrap snapshot claims. | HISTORICAL_DEFERRED |
| TARGET-053 | `history/legacy-conflict-and-completion-record.md` | Historical Record | legacy snapshot and stale/contradiction claims | Legacy conflict/completion records; G002, G011, G012 | Historical context only; current truth must come from source evidence. | HISTORICAL_DEFERRED |
| TARGET-054 | `history/legacy-structure-and-sequencing.md` | Historical Record | bootstrap structure/phase records and Google baseline | Legacy proposed structure/phases; G012 | Historical context only; no legacy sequencing is treated as final target authority. | HISTORICAL_DEFERRED |

## Disposition totals

| Status | Count |
| --- | ---: |
| BUILT | 19 |
| DEFERRED_MERGE | 12 |
| DEFERRED_REGENERATE | 11 |
| BLOCKED_OWNER_DECISION | 7 |
| HISTORICAL_DEFERRED | 5 |
| INSUFFICIENT_EVIDENCE | 0 |
| OUT_OF_PHASE | 0 |
| **Total** | **54** |
