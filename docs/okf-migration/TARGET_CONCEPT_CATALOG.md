# ZinharCMS Target Google OKF v0.2 Concept Catalog

**DESIGN ONLY - NOT YET IMPLEMENTED**

The IDs in this catalog are migration-planning identifiers only. They are not
Google OKF Concept IDs and must not be written into future frontmatter. The
future Concept ID is the file path inside `/okf/`, as defined by Google OKF.

The catalog contains 54 proposed Concepts. Every target path appears once in
[TARGET_BUNDLE_TREE.md](TARGET_BUNDLE_TREE.md). Legacy path-level coverage is
in [LEGACY_TO_TARGET_MAPPING.md](LEGACY_TO_TARGET_MAPPING.md); this catalog
does not authorize any build, move, or deletion.

## Build strategy vocabulary

- `CREATE_FROM_VERIFIED_KNOWLEDGE`: a bounded current Concept can be built
  directly from current source-backed evidence, with stated limitations.
- `MERGE_INTO_TARGET`: several legacy views become one coherent Concept;
  current claims still require final source review.
- `REGENERATE_FROM_REPOSITORY`: produce the Concept from current routes,
  migrations, tests, configuration, CI, or a deterministic source inventory.
- `BLOCKED_PENDING_DECISION`: the Concept can be outlined, but its intended
  policy or authority cannot be finalized without the listed NOC decision.
- `PRESERVE_AS_HISTORY`: consolidate historical snapshots without promoting
  them to current implementation truth.

## Catalog

| Target ID | Proposed path | Type | Purpose | Legacy sources / groups | Evidence source | Build strategy | Decision blocker |
| --- | --- | --- | --- | --- | --- | --- | --- |
| TARGET-001 | `project/project-overview.md` | Project | Product identity, scope, release boundary, and implemented capability summary. | Project/README views; G001 | `README.md`, `LICENSE`, current release docs | CREATE_FROM_VERIFIED_KNOWLEDGE | NOC-18 terminology caveat only |
| TARGET-002 | `project/terminology-and-glossary.md` | Reference | Owner-approved product terms, abbreviations, and canonical labels. | Project glossary; G001 | Current UI/code usage plus owner glossary when supplied | BLOCKED_PENDING_DECISION | NOC-18 |
| TARGET-003 | `project/repository-and-evidence-map.md` | Reference | Repository navigation and source-of-truth routing for agents and developers. | Project map/navigation; G001, G013 | Repository tree, source hierarchy, Phase 1 evidence | REGENERATE_FROM_REPOSITORY | NOC-13 |
| TARGET-004 | `project/documentation-governance.md` | Operational Guide | Canonical-document ownership, review triggers, retirement, and handoff rules. | Maintenance/project governance; G001, G011 | `HANDOFF.md`, owner register, Git workflow | BLOCKED_PENDING_DECISION | NOC-13, NOC-15 |
| TARGET-005 | `project/source-lineage.md` | Reference | Translation of the legacy source register into current source/provenance practice. | Source register; G013 | Phase 1 manifest, Git history, immutable source paths | REGENERATE_FROM_REPOSITORY | NOC-13, NOC-15 for durable ownership |
| TARGET-006 | `architecture/system-architecture.md` | Architecture | Modular-monolith system boundary and major runtime components. | Architecture overview/components; G002 | `backend/src/main.rs`, `backend/src/lib.rs`, `frontend/src/main.tsx`, Compose | CREATE_FROM_VERIFIED_KNOWLEDGE | None for repository boundary |
| TARGET-007 | `architecture/runtime-and-request-boundaries.md` | Architecture | Public, authenticated, tenant-protected, and preview request boundaries. | Architecture/API boundary views; G002, G004, G006 | `backend/src/routes/mod.rs`, middleware, preview route | CREATE_FROM_VERIFIED_KNOWLEDGE | NOC-01 for intended public tenant routing |
| TARGET-008 | `architecture/integrations-and-side-effects.md` | Architecture | Cache, webhook, email, file, and transaction side-effect boundaries. | Architecture integration/risk views; G002 | Services, routes, cache, webhook, and email code | MERGE_INTO_TARGET | NOC-09 |
| TARGET-009 | `architecture/architecture-decision-records.md` | Architecture | Current architectural constraints and rationale that remain useful after migration. | Architecture decisions and conflicts; G002, G011 | Current source, Phase 1 claims, Phase 2 ADRs | REGENERATE_FROM_REPOSITORY | NOC-13 canonical authority |
| TARGET-010 | `backend/backend-runtime.md` | Component | Rust/Axum/Tokio backend process, bootstrap, shared state, and error boundary. | Backend overview/runtime; G003 | `backend/Cargo.toml`, `backend/src/main.rs`, `backend/src/lib.rs` | CREATE_FROM_VERIFIED_KNOWLEDGE | None |
| TARGET-011 | `backend/module-boundaries.md` | Component | Routes, middleware, services, persistence, plugins, and module direction. | Backend module maps/catalogs; G003 | `backend/src/routes`, `backend/src/middleware`, `backend/src/services`, `backend/src/plugins` | CREATE_FROM_VERIFIED_KNOWLEDGE | NOC-15 ownership caveat |
| TARGET-012 | `backend/persistence-services-and-configuration.md` | Component | Database/Redis access, application state, configuration, and service composition. | Backend persistence/configuration; G003, G005 | `backend/src/db`, `backend/src/state.rs`, `backend/src/config.rs` | MERGE_INTO_TARGET | NOC-02 storage deployment caveat |
| TARGET-013 | `backend/backend-validation.md` | Development Guide | Backend tests, formatting, clippy, migrations, and validation evidence. | Backend testing/maintenance; G003, G011 | `backend/tests`, `Cargo.toml`, CI workflow | REGENERATE_FROM_REPOSITORY | NOC-07, NOC-14 |
| TARGET-014 | `frontend/admin-application.md` | Component | React/Vite administration SPA boundary and application shell. | Frontend architecture/overview; G008 | `frontend/package.json`, `frontend/src/main.tsx`, router | CREATE_FROM_VERIFIED_KNOWLEDGE | None for observed implementation |
| TARGET-015 | `frontend/routing-and-state.md` | Component | Frontend routes, API client, authentication projection, and organization state. | Frontend routing/state/API views; G008, G006 | `frontend/src/router.tsx`, `frontend/src/services/api.ts`, store | CREATE_FROM_VERIFIED_KNOWLEDGE | NOC-12 compatibility/accessibility policy |
| TARGET-016 | `frontend/feature-boundaries.md` | Component | Editorial, builder, media, billing, Marketplace, localization, and settings feature boundaries. | Frontend feature views; G008 | `frontend/src/pages`, components, i18n | MERGE_INTO_TARGET | NOC-12, NOC-18 |
| TARGET-017 | `frontend/frontend-quality-and-testing.md` | Development Guide | Frontend lint, typecheck, tests, build, and browser-quality evidence. | Frontend testing/risks; G008, G011 | Frontend package scripts, tests, CI workflow | REGENERATE_FROM_REPOSITORY | NOC-07, NOC-14 |
| TARGET-018 | `api/api-contract-overview.md` | API Contract | Current API families, authority boundaries, and contract limitations. | API overview/groups; G004 | `backend/src/routes`, `frontend/src/services/api.ts`, current API docs | CREATE_FROM_VERIFIED_KNOWLEDGE | NOC-08 |
| TARGET-019 | `api/route-surface.md` | API Endpoint | Exhaustive route and OpenAPI surface generated from current source. | Endpoint catalogs and route maps; G004 | `backend/src/routes/mod.rs`, route modules, utoipa annotations | REGENERATE_FROM_REPOSITORY | NOC-08 |
| TARGET-020 | `api/authentication-and-session-contract.md` | API Contract | Public authentication, refresh, logout, MFA, recovery, and session contract. | API auth and security session views; G004, G006 | `backend/src/routes/auth.rs`, auth/session services, migrations 0027-0029 | CREATE_FROM_VERIFIED_KNOWLEDGE | NOC-08, NOC-12 |
| TARGET-021 | `api/public-delivery-and-webhooks-contract.md` | API Contract | Published delivery, cache, and webhook interface boundaries. | Public delivery/webhook views; G004, G007 | `backend/src/routes/delivery.rs`, webhook/cache services | MERGE_INTO_TARGET | NOC-01, NOC-08, NOC-09 |
| TARGET-022 | `api/marketplace-and-extension-contracts.md` | API Contract | Marketplace catalog, creator, review, installation, runtime, and extension endpoints. | Marketplace/plugin API views; G004, G009 | Marketplace route modules and API client | MERGE_INTO_TARGET | NOC-10 |
| TARGET-023 | `database/schema-and-migrations.md` | Data Model | Migration-defined PostgreSQL schema and migration ordering. | Database schema/migration catalogs; G005 | `backend/migrations`, SQLx startup code | REGENERATE_FROM_REPOSITORY | NOC-03, NOC-06 for deployed state |
| TARGET-024 | `database/entities-and-relationships.md` | Data Model | Verified entity relationships across identity, content, billing, and Marketplace. | Database entity/relationship views; G005, G007, G009 | Migrations, models, queries, tests | REGENERATE_FROM_REPOSITORY | NOC-03, NOC-05 |
| TARGET-025 | `database/tenant-data-policy.md` | Database Policy | Session context, RLS helper/policy behavior, and tenant data boundary. | Multi-tenancy/RLS views; G005, G006, G007 | `backend/src/services/rls.rs`, migration 0009, RLS tests | MERGE_INTO_TARGET | NOC-01, NOC-05 |
| TARGET-026 | `database/data-lifecycle-and-retention.md` | Database Policy | Data deletion, legal hold, audit, billing, and artifact retention policy boundary. | Database/security risk and lifecycle views; G005, G007, G011 | Current schema plus owner/legal policy | BLOCKED_PENDING_DECISION | NOC-05 |
| TARGET-027 | `security/authentication-and-sessions.md` | Authentication Flow | Bearer access, cookie refresh families, MFA, recovery, Step-Up, and revocation. | Security authentication/session views; G006 | Auth routes/services, security migrations, tests | CREATE_FROM_VERIFIED_KNOWLEDGE | NOC-12 for long-term policy |
| TARGET-028 | `security/authorization-and-rbac.md` | Authorization Policy | Global/organization roles, permission gates, ownership, and support boundaries. | Roles/permissions/RBAC; G006 | `backend/src/services/rbac.rs`, auth/tenant middleware, routes | CREATE_FROM_VERIFIED_KNOWLEDGE | NOC-15 ownership caveat |
| TARGET-029 | `security/tenant-isolation.md` | Security Control | Layered tenant middleware, explicit predicates, PostgreSQL session context, and RLS. | Tenant access/security/database views; G005, G006, G007 | Tenant middleware, RLS service, migration 0009, tests | CREATE_FROM_VERIFIED_KNOWLEDGE | NOC-01; no live deployment proof |
| TARGET-030 | `security/preview-security.md` | Security Control | Short-lived single-use preview tickets, Origin/protocol gates, and revalidation. | Preview/trust-boundary views; G004, G006, G007 | Pages route, preview tickets service, security tests | CREATE_FROM_VERIFIED_KNOWLEDGE | None for observed implementation |
| TARGET-031 | `security/storage-and-file-security.md` | Security Control | Upload, artifact, path, cleanup, and public/private storage security controls. | Security/storage/media/Marketplace views; G006, G007, G009 | File security services, media routes, migration 0030, tests | MERGE_INTO_TARGET | NOC-02, NOC-05 |
| TARGET-032 | `security/security-posture-and-risks.md` | Security Control | Implemented hardening, threat limits, unresolved deployment controls, and security evidence. | Security overview/threat/risk/testing views; G006, G010 | Security middleware/services/tests and Phase 7 evidence | MERGE_INTO_TARGET | NOC-04, NOC-05, NOC-15 |
| TARGET-033 | `domain/content-and-editorial-workflow.md` | Domain Workflow | Content schemas, entries, validation, editorial state, collaboration, and publication. | Content/domain/backend/frontend workflows; G003, G007, G008 | Content routes/services/models, migrations, tests | CREATE_FROM_VERIFIED_KNOWLEDGE | NOC-12 schema/workflow policy |
| TARGET-034 | `domain/page-builder-and-preview.md` | Domain Workflow | Page components, versions, save/restore, publication, and live preview behavior. | Page builder/preview workflows; G004, G007, G008 | Pages routes/models/services and frontend pages | CREATE_FROM_VERIFIED_KNOWLEDGE | NOC-12 compatibility policy |
| TARGET-035 | `domain/media-and-file-storage.md` | Resource Boundary | Media processing, upload/download/streaming, ownership, and artifact boundary. | Media domain/API/security views; G004, G006, G007 | Media routes/services, file security, migration 0030 | REGENERATE_FROM_REPOSITORY | NOC-02, NOC-05 |
| TARGET-036 | `domain/public-delivery.md` | Resource Boundary | Published content delivery, cache behavior, tenant selection, and public limits. | Delivery domain/API views; G004, G007 | Delivery routes, cache service, organizations migration | MERGE_INTO_TARGET | NOC-01, NOC-09 |
| TARGET-037 | `domain/billing-and-quotas.md` | Domain Model | Plans, subscriptions, quotas, usage, billing hooks, and implemented limits. | Billing/quota views; G004, G005, G007 | Billing routes/services, quota service, migrations 0010-0012 | MERGE_INTO_TARGET | NOC-09 provider/failure policy |
| TARGET-038 | `domain/marketplace.md` | Marketplace Domain | Implemented creator, listing, submission, moderation, review, installation, finance, and analytics lifecycle. | Marketplace domain/backend/API/database views; G004, G005, G007, G009 | Marketplace routes/services, migrations 0015-0026, tests | CREATE_FROM_VERIFIED_KNOWLEDGE | NOC-10 roadmap/settlement boundaries |
| TARGET-039 | `domain/extensibility-and-built-in-plugins.md` | Component | Built-in in-process plugins, extension points, hooks, adapters, and host ownership. | Plugin/extension views; G003, G009 | `backend/src/plugins`, plugin routes, runtime adapters | CREATE_FROM_VERIFIED_KNOWLEDGE | NOC-10, NOC-12 |
| TARGET-040 | `domain/marketplace-runtime-and-safety-boundary.md` | Marketplace Domain | Package validation, permissions, kill switches, installation safety, and non-execution boundary. | Marketplace runtime/extensibility/security views; G006, G009 | Marketplace validation/runtime services, policy docs, tests | MERGE_INTO_TARGET | NOC-10 |
| TARGET-041 | `operations/local-and-reference-topology.md` | Operational Guide | Local Compose, container, filesystem, PostgreSQL, Redis, and reference topology. | Operations/deployment/technology views; G010 | Compose, Dockerfiles, config, README | REGENERATE_FROM_REPOSITORY | NOC-02, NOC-06 |
| TARGET-042 | `operations/ci-and-release-gates.md` | Operational Guide | Current CI checks, release gates, source publication boundary, and artifacts. | Delivery/CI/release views; G010, G011 | `.github/workflows`, package scripts, release docs | REGENERATE_FROM_REPOSITORY | NOC-06, NOC-14 |
| TARGET-043 | `operations/deployment-and-recovery.md` | Operational Guide | Production environments, promotion, rollback, backup, restore, RPO, and RTO. | Operations/delivery recovery views; G010 | Owner/platform evidence required; local Compose is not proof | BLOCKED_PENDING_DECISION | NOC-03, NOC-06 |
| TARGET-044 | `operations/observability-and-support.md` | Operational Guide | Production logs, metrics, alerts, SLOs, dashboards, on-call, and escalation. | Operations/maintenance risk views; G010, G011 | Owner/platform evidence required; source health/tracing only | BLOCKED_PENDING_DECISION | NOC-04, NOC-15 |
| TARGET-045 | `development/development-and-testing.md` | Development Guide | Prerequisites, commands, test entry points, and supported local workflow evidence. | Development/testing maps; G003, G008, G011 | Package manifests, CI, tests, README | CREATE_FROM_VERIFIED_KNOWLEDGE | NOC-07, NOC-14 |
| TARGET-046 | `development/contribution-and-change-validation.md` | Development Guide | Branch/review/check, migration, documentation, and change validation policy. | Development/contribution/maintenance views; G011 | CI workflows, Git history, owner policy | BLOCKED_PENDING_DECISION | NOC-14, NOC-15 |
| TARGET-047 | `development/documentation-maintenance.md` | Development Guide | Documentation update triggers, link checks, index/log maintenance, and retirement process. | Maintenance policy/checklist/staleness views; G001, G011 | Phase 1 artifacts, HANDOFF, Git state | MERGE_INTO_TARGET | NOC-13, NOC-14 |
| TARGET-048 | `decisions/owner-decision-debt.md` | Decision | The 17 open NOCs, 15 UNKNOWN labels, affected Concepts, and required resolution evidence. | Bootstrap gaps/owner questions; G011, G012 | `OWNER_DECISION_REGISTER.md`, Phase 1 manifest | BLOCKED_PENDING_DECISION | NOC-01 through NOC-15, NOC-17, NOC-18 |
| TARGET-049 | `decisions/migration-and-architecture-decisions.md` | Decision | Accepted Phase 2 migration choices and future reversible architecture decisions. | Phase 0/1 migration decisions; G001, G002, G011 | Phase 2 ADRs and official specification | CREATE_FROM_VERIFIED_KNOWLEDGE | None for Phase 2 choices |
| TARGET-050 | `history/phase-0-baseline.md` | Historical Record | Phase 0 baseline, legacy gap, reference, and risk audit history. | Phase 0 reports; G012 and Phase 0 artifacts | `PHASE_00_BASELINE_AUDIT.md`, legacy snapshot | PRESERVE_AS_HISTORY | None; historical only |
| TARGET-051 | `history/phase-1-preservation.md` | Historical Record | Phase 1 corpus, claim, disposition, diagram, and owner preservation history. | Phase 1 reports; G012 | Phase 1 report and evidence manifest | PRESERVE_AS_HISTORY | None; historical only |
| TARGET-052 | `history/bootstrap-audit-record.md` | Historical Record | Consolidated record of the 14 original bootstrap audits and their evidence limits. | All `okf-bootstrap/*.md`; G012 | Git snapshot, Phase 1 matrix, claim ledger | PRESERVE_AS_HISTORY | None; historical only |
| TARGET-053 | `history/legacy-conflict-and-completion-record.md` | Historical Record | Legacy conflicts, validation reports, completion claims, and superseded navigation. | Maintenance conflict/completion records; G002, G011, G012 | Legacy snapshot, Phase 1 stale/contradiction claims | PRESERVE_AS_HISTORY | None; historical only |
| TARGET-054 | `history/legacy-structure-and-sequencing.md` | Historical Record | Prior custom OKF structure and migration sequencing as historical design context. | Legacy proposed structure/phases; G012 | Bootstrap structure/phase records, Google baseline | PRESERVE_AS_HISTORY | None; historical only |

## Catalog totals

| Measure | Count |
| --- | ---: |
| Target Concepts | 54 |
| `CREATE_FROM_VERIFIED_KNOWLEDGE` | 19 |
| `MERGE_INTO_TARGET` | 12 |
| `REGENERATE_FROM_REPOSITORY` | 11 |
| `BLOCKED_PENDING_DECISION` | 7 |
| `PRESERVE_AS_HISTORY` | 5 |
| Approved type values used | 19 |
| Target directories including root | 13 |
| Reserved indexes | 13 |
| Reserved logs | 1 |

The totals above are a design contract. If a later construction phase changes a
count, it must update this catalog, the target tree, the mapping, the decision
record, and the Phase 2 handoff checkpoint before the bundle is treated as
ready for cutover.
