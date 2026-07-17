# Implementation Phases

## Sequencing Rules

- Each phase begins from repository source of truth and the unresolved marker registers.
- Existing authoritative documents are linked or corrected; they are not silently duplicated.
- A phase is not complete until its source paths, verification date, conflicts, and remaining unknowns are recorded.
- Owner decisions are required only where a document would otherwise make a policy or production claim.
- Later phases must not erase UNKNOWN or NEEDS_OWNER_CONFIRMATION markers without evidence.

## Phase 1 - Project Overview and Repository Map

| Field | Plan |
| --- | --- |
| Objective | Establish the OKF navigation, authority model, current product scope, repository map, glossary, and documentation map |
| Inputs | Phase Zero reports 00, 03, 04, 09, and 10; README; AGENTS; Git inventory |
| Outputs | okf/README.md, okf/index.yaml, project/overview.md, repository-map.md, glossary.md, documentation-map.md, initial reference registers |
| Prerequisites | Explicit authorization to start Phase One; no owner answer is required for repository-derived content |
| Risks | Turning historical phase language into current claims; inventing glossary preferences; duplicating README |
| Definition of done | Every top-level knowledge area is navigable; source authority is explicit; repository counts are reproducible; all unresolved markers are carried forward |
| Complexity | Medium |

## Phase 2 - Architecture

| Field | Plan |
| --- | --- |
| Objective | Document system context, runtime containers, backend/frontend components, request/data flows, trust boundaries, dependencies, and decisions |
| Inputs | Architecture observations, runtime code, Docker/CI, diagrams 02-09, 21-22, 31, owner decisions for production claims |
| Outputs | architecture directory documents and selected updated diagram links |
| Prerequisites | Phase 1 indexes; NOC-01, NOC-02, and NOC-06 are required only to finalize production topology |
| Risks | Presenting Docker Compose as production; conflating built-in plugins with Marketplace packages; omitting transient task/process limitations |
| Definition of done | Logical and deployment views are separated; all trust boundaries and external dependencies are sourced; production unknowns remain visible |
| Complexity | High |

## Phase 3 - Backend Modules

| Field | Plan |
| --- | --- |
| Objective | Consolidate the 22-module responsibility, dependency, data, API, test, and ownership map |
| Inputs | Module inventory, backend source, migrations, route map, phase documents |
| Outputs | modules/index.md, backend.md, cms.md, saas-operations.md, marketplace.md |
| Prerequisites | Phase 1 and architecture terminology |
| Risks | Treating files as modules; overlooking cross-cutting RLS/quota/audit behavior; assigning owners without evidence |
| Definition of done | Every runtime capability has a stable module entry, inbound/outbound dependencies, source paths, tests, and unresolved risks |
| Complexity | High |

## Phase 4 - Frontend

| Field | Plan |
| --- | --- |
| Objective | Document routing, shell, authentication/organization state, API client, forms, page builder, localization, accessibility, and test boundaries |
| Inputs | frontend source and manifest, diagrams 10-12, docs/I18N.md, backend API contracts |
| Outputs | frontend directory documents and DCC-02 resolution proposal |
| Prerequisites | Phase 1 glossary and Phase 3 module map |
| Risks | Overstating translation coverage; ignoring manual DTO drift and token-expiry behavior; documenting large page internals as intended architecture |
| Definition of done | Every route and shared platform concern is mapped; builder and localization limits are explicit; API/state flows are verified against code |
| Complexity | Medium |

## Phase 5 - Database

| Field | Plan |
| --- | --- |
| Objective | Produce the 51-table schema index, relationship views, RLS matrix, state/constraint catalog, migration/seed workflow, and retention matrix |
| Inputs | All 26 migrations, database inventory, rls.rs, services, diagrams 13-19 and 22 |
| Outputs | database directory documents and updated domain ERD plan |
| Prerequisites | Phase 3 module boundaries; deployed-state claims require U-02 evidence |
| Risks | Counting SQL templates as distinct policies; omitting eight later RLS tables; inventing retention or rollback guarantees |
| Definition of done | Every table has domain, ownership, keys, RLS, source migration, lifecycle, and related module; parser/generated checks are reproducible |
| Complexity | High |

## Phase 6 - APIs

| Field | Plan |
| --- | --- |
| Objective | Build a complete route and contract index, security-boundary matrix, error/validation conventions, realtime/provider contracts, and OpenAPI coverage report |
| Inputs | Composed router, all handlers, AppError, Utoipa registry, frontend client, docs/API.md |
| Outputs | api directory documents; DCC-09 correction proposal; missing OpenAPI annotation list |
| Prerequisites | Phases 3 and 5; NOC-08 required only for final versioning policy |
| Risks | Counting annotations instead of reachable routes; describing tenant-authenticated catalog as anonymous; missing WebSocket or raw file surfaces |
| Definition of done | Every reachable route is classified by method, path, boundary, authorization, tenant behavior, DTO, errors, and OpenAPI status |
| Complexity | High |

## Phase 7 - Authentication, Authorization, and Security

| Field | Plan |
| --- | --- |
| Objective | Consolidate authentication/session behavior, role matrices, tenant isolation, upload/artifact controls, signatures, secrets, and threat model |
| Inputs | Middleware, auth/RBAC/RLS services, migrations, security tests, diagrams 03/22/24/26/34/40 |
| Outputs | security directory documents and prioritized control gaps |
| Prerequisites | API and database matrices; owner input for production secrets, retention, and response obligations |
| Risks | Equating application controls with production security posture; exposing secrets; missing public asset or global-admin paths |
| Definition of done | Control claims have code/test evidence; identities and roles are mapped; tenant bypass is explicit; operational unknowns remain marked |
| Complexity | High |

## Phase 8 - Business Rules and Multi-Tenancy

| Field | Plan |
| --- | --- |
| Objective | Extract current content/page workflows, organization/plan rules, quotas/billing, delivery/cache, tenancy, and failure semantics |
| Inputs | Workflow, quota, billing, delivery, cache, organization services, migrations, tests, state diagrams |
| Outputs | business directory documents and state-transition tables |
| Prerequisites | Phases 3, 5, 6, and 7 |
| Risks | Treating handler order as a guaranteed transaction; omitting post-commit side effects; inventing product meaning for state values |
| Definition of done | Executable rules are separated from owner policy; state preconditions/effects/errors are documented; compensation gaps are explicit |
| Complexity | High |

## Phase 9 - Plugins, Marketplace, and Extensibility

| Field | Plan |
| --- | --- |
| Objective | Document built-in plugins separately from Marketplace publication, install, runtime policy, adapters, finance, feedback, analytics, and readiness |
| Inputs | Marketplace migrations/services/routes/tests, phase records, diagrams 19-20 and 25/30/33-42 |
| Outputs | extensions directory documents; resolutions for DCC-03, DCC-04, DCC-06, DCC-07, DCC-08, and DCC-10 |
| Prerequisites | Phases 3, 5, 6, 7, and 8; NOC-10 for roadmap boundaries |
| Risks | Claiming uploaded code execution; preserving obsolete deferment statements; overstating automated payout/dispute support |
| Definition of done | Every lifecycle has current states, gates, data, APIs, tests, and deferred boundaries; stale Marketplace diagrams are corrected or retired |
| Complexity | High |

## Phase 10 - Deployment, Operations, Development, and Testing

| Field | Plan |
| --- | --- |
| Objective | Document local setup, contribution, quality gates, configuration, deployment, observability, backup/recovery, incidents, and troubleshooting |
| Inputs | Dockerfiles, Compose, CI, manifests, scripts, runbooks, convention inventory, platform/owner evidence |
| Outputs | development and operations directories |
| Prerequisites | Architecture and security docs; NOC-02 through NOC-07, NOC-14, and NOC-15 for final production statements |
| Risks | Presenting reference containers as production; promising unavailable monitoring/recovery; copying secrets or stale commands |
| Definition of done | Local procedures are reproducible; production claims cite real configuration; tests and release gates are explicit; unresolved operations sections remain marked draft |
| Complexity | High |

## Phase 11 - Mermaid Diagrams

| Field | Plan |
| --- | --- |
| Objective | Select, correct, title, parse, render, and index the minimum useful diagram set |
| Inputs | Mermaid audit, final architecture/module/database/API/security/business/extension docs, existing 43 diagrams |
| Outputs | diagrams/index.md, validation-report.md, updated linked Mermaid sources where authorized |
| Prerequisites | Content phases 2 through 10 are substantially stable; a Mermaid parser/renderer version is available |
| Risks | Duplicating diagrams, rendering stale facts, giant unreadable ERDs, parser-version drift |
| Definition of done | Every selected diagram parses and renders; purpose and owner are indexed; conflicts are resolved; omitted diagrams have a reason |
| Complexity | Medium |

## Phase 12 - Validation and Synchronization

| Field | Plan |
| --- | --- |
| Objective | Verify coverage, source links, marker resolution, internal links, generated inventories, diagram rendering, and repository synchronization |
| Inputs | Entire OKF, code, migrations, router, OpenAPI, tests, existing docs, owner decisions |
| Outputs | Final validation report, current index metadata, remaining gap register, synchronization procedure |
| Prerequisites | Phases 1 through 11 |
| Risks | Declaring completion with unresolved high-priority claims; updating generated counts manually; hiding conflicts instead of resolving them |
| Definition of done | All required sections are current or explicitly draft; every claim has authority; automated checks pass; all remaining markers have owners and next actions |
| Complexity | High |

## Phase Dependencies

~~~text
Phase 1
  -> Phase 2
  -> Phase 3 -> Phase 4
             -> Phase 5 -> Phase 6 -> Phase 7 -> Phase 8 -> Phase 9
  -> Phase 10 depends on architecture and security evidence
  -> Phase 11 depends on stable content from Phases 2-10
  -> Phase 12 validates and synchronizes all prior phases
~~~

## Exact Start of the Next Phase

After explicit authorization, Phase One should begin by creating okf/README.md and okf/index.yaml, then project/overview.md, repository-map.md, glossary.md, and documentation-map.md. It should import the marker registers from Phase Zero without resolving them by assumption. It should not alter product code, migrations, APIs, or existing diagrams.
