---
okf_document_id: "navigation-guide"
title: "Navigation Guide"
project: "ZinharCMS"
category: "project"
phase: 1
status: "current"
review_status: "verified"
source_of_truth: false
last_verified_commit: "eed1e0dbdf6d873457d1165158b3c8fbfd6647e1"
last_verified_date: "2026-07-18"
primary_sources:
  - "README.md"
  - "package.json"
  - "backend/src"
  - "backend/migrations"
  - "frontend/src"
  - "docs"
  - "okf-bootstrap"
related_documents:
  - "README.md"
  - "index.yaml"
  - "project/overview.md"
  - "project/repository-map.md"
  - "project/glossary.md"
  - "references/source-register.md"
  - "architecture/README.md"
  - "architecture/overview.md"
  - "architecture/boundaries.md"
  - "architecture/components.md"
  - "architecture/dependency-model.md"
  - "architecture/runtime-flows.md"
  - "architecture/integration-points.md"
  - "architecture/architecture-risks.md"
  - "architecture/decisions/decision-register.md"
  - "backend/README.md"
  - "backend/module-catalog.md"
  - "backend/testing-map.md"
  - "backend/backend-risks.md"
  - "frontend/README.md"
  - "frontend/feature-catalog.md"
  - "frontend/routing.md"
  - "frontend/state-management.md"
  - "frontend/testing-map.md"
  - "frontend/frontend-risks.md"
uncertainty_markers:
  - "UNKNOWN U-06"
  - "UNKNOWN U-07"
  - "UNKNOWN U-08"
  - "UNKNOWN U-13"
  - "NEEDS_OWNER_CONFIRMATION NOC-01"
  - "NEEDS_OWNER_CONFIRMATION NOC-03"
  - "NEEDS_OWNER_CONFIRMATION NOC-14"
  - "DOCUMENTATION_CODE_CONFLICT DCC-09"
  - "IMPLEMENTATION_STATUS_UNCLEAR ISU-03"
  - "UNKNOWN"
  - "NEEDS_OWNER_CONFIRMATION"
  - "DOCUMENTATION_CODE_CONFLICT"
  - "PLANNED_NOT_IMPLEMENTED"
  - "IMPLEMENTATION_STATUS_UNCLEAR"
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR ABU-01"
  - "DEPENDENCY_DIRECTION_UNCLEAR DDU-01"
  - "FEATURE_BOUNDARY_UNCLEAR FBU-01"
  - "STATE_OWNERSHIP_UNCLEAR SOU-01"
  - "API_CONTRACT_UNCLEAR ACU-01"
  - "AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01"
---

# Navigation Guide

## API and Contract Tasks

- Start at [API Architecture and Contracts](../api/README.md) for the Phase 6 reading order and authority rules.
- Use the [Endpoint Catalog](../api/endpoint-catalog.md) for an exact method, path, handler, access zone, extractor, return type, OpenAPI status, frontend wrapper, and endpoint-family link across all 168 registered handlers.
- Use the [Route Group Catalog](../api/route-group-catalog.md) when the owning Axum module is known.
- Review [Authentication](../api/authentication.md), [Authorization](../api/authorization.md), and [Tenant Context](../api/tenant-context.md) together before moving a route or changing access policy.
- Compare [OpenAPI Consistency](../api/openapi-consistency.md) and [Frontend Contract Map](../api/frontend-contract-map.md) whenever a transport contract changes.

Use this guide to choose the shortest evidence path for a common repository task. The [Repository Map](repository-map.md) provides the fuller structural inventory, and the [Source Register](../references/source-register.md) explains the reliability of the referenced sources.

## Start Here

1. Read the root [README](../../README.md) for the project introduction and local-development commands.
2. Read the [Project Overview](overview.md) for the evidence-based Phase 1 summary and system boundary.
3. Use the [Repository Map](repository-map.md) to locate application, infrastructure, test, and documentation areas.
4. Check the [Project Glossary](glossary.md) before introducing or interpreting domain terminology.
5. Read the [Architecture Overview](../architecture/overview.md) for system classification, runtime shape, and deployment-evidence limits.
6. Use the [System Architecture](../architecture/README.md) reading order for boundaries, components, dependencies, flows, integrations, risks, decisions, and diagrams.
7. Use [Backend Documentation](../backend/README.md) and the [Module Catalog](../backend/module-catalog.md) for backend-specific work.
8. Use [Frontend Architecture](../frontend/README.md) and the [Feature Catalog](../frontend/feature-catalog.md) for frontend-specific work.
9. Use [Database Architecture](../database/README.md), the [Schema Catalog](../database/schema-catalog.md), and the [Entity Catalog](../database/entity-catalog.md) for persistence work.
10. Consult the [Source Register](../references/source-register.md) to distinguish primary evidence from supporting or conflicting documentation.
11. Review the [Machine-Readable Index](../index.yaml) for document status, relationships, and uncertainty markers.
12. When a claim remains unresolved, follow its marker into [Phase Zero Knowledge Gaps](../../okf-bootstrap/09-knowledge-gaps.md) or [Owner Questions](../../okf-bootstrap/12-owner-questions.md).

## Navigate by Task

| Task | Start with | Continue with | Important note |
|---|---|---|---|
| Understand the product and implemented scope | `README.md` | `okf/project/overview.md`; `docs/V3_MARKETPLACE_SCOPE.md` | Treat product claims as supporting evidence until confirmed in code, configuration, or migrations. |
| Understand the current architecture | `okf/architecture/overview.md` | `okf/architecture/diagrams/system-context.mmd`; `okf/architecture/diagrams/container-view.mmd` | The backend is one modular-monolith process, not a microservice set. |
| Understand a backend module | `okf/backend/module-catalog.md` | Owning document under `okf/backend/modules`; `okf/backend/module-boundaries.md` | Module boundaries are source-level responsibilities inside one crate. |
| Change a backend handler | Owning `okf/backend/modules` document | `okf/backend/request-handling.md`; matching `backend/src/routes` file; related service, migration, and tests | Handlers often contain policy, SQL, and side effects; do not treat them as uniformly thin controllers. |
| Change service or domain logic | `okf/backend/services-and-domain.md` | Owning module document; matching `backend/src/services` and route consumers | Rules are distributed; search handlers, middleware, migrations, and tests as well as services. |
| Change application state or configuration | `okf/backend/configuration-and-state.md` | `backend/src/state.rs`; `backend/src/config.rs`; `backend/src/main.rs`; state-composition diagram | `AppState` has four verified fields and preview channels are process-local. |
| Find backend persistence code | `okf/database/persistence-mapping.md` | `okf/backend/persistence-access.md`; owning entity and route/service files | No mandatory repository layer exists; use migrations before partial models. |
| Find backend tests | `okf/backend/testing-map.md` | Owning module document; colocated `#[cfg(test)]` blocks; `.github/workflows/backend-ci.yml` | No separate `backend/tests` directory was found and no coverage percentage is claimed. |
| Investigate backend errors | `okf/backend/error-handling.md` | `backend/src/error.rs`; failing route/service; `okf/backend/backend-risks.md` | Framework, timeout, WebSocket, and some provider paths can differ from `ErrorBody`. |
| Add a significant backend module | `okf/backend/module-catalog.md#selection-rule` | `okf/backend/module-boundaries.md`; `dependency-map.md`; `testing-map.md`; diagrams; `okf/index.yaml` | Add a module document only when it meets the catalog criteria; keep small helpers in shared infrastructure. |
| Identify a system or trust boundary | `okf/architecture/boundaries.md` | `okf/architecture/components.md`; `backend/src/routes/mod.rs` | Tenant enforcement is distributed across middleware, handlers, services, and RLS-aware paths. |
| Modify backend request flow | `okf/architecture/runtime-flows.md`; `okf/architecture/diagrams/backend-request-flow.mmd` | `backend/src/main.rs`; `backend/src/routes/mod.rs`; relevant middleware, handler, and service | Select public, authenticated, or tenant-protected flow before changing composition. |
| Modify frontend-backend communication | `okf/architecture/diagrams/frontend-backend-flow.mmd`; `frontend/src/services/api.ts` | `frontend/src/types/api.ts`; matching backend route and response types | Validate both manually maintained contract representations. |
| Add a major integration | `okf/architecture/integration-points.md` | `okf/architecture/boundaries.md`; configuration, adapter, route, migration, and failure behavior | Configuration capability is not proof of live production wiring. |
| Review dependency direction | `okf/architecture/dependency-model.md` | `okf/architecture/diagrams/dependency-direction.mmd`; exact Rust and TypeScript imports | Preserve DDU markers where the apparent layer direction is reversed or duplicated. |
| Investigate architecture risks | `okf/architecture/architecture-risks.md` | `okf/architecture/decisions/decision-register.md`; affected implementation | Risk ratings are evidence-based design risks, not production incident claims. |
| Check an existing architecture choice | `okf/architecture/decisions/decision-register.md` | Entry evidence and review trigger | Most decisions are observed and inferred; rationale remains unknown unless explicitly sourced. |
| Run the local stack | `README.md`; `package.json` | `docker-compose.yml`; `.env.example`; `env.example` | The root `npm run dev` invokes the default Compose file, which currently defines infrastructure services rather than both application processes. |
| Start backend development | `backend/Cargo.toml`; `backend/src/main.rs` | `backend/src/lib.rs`; `backend/src/state.rs`; `backend/src/routes/mod.rs` | Backend configuration is environment-driven. |
| Start frontend development | `frontend/package.json`; `frontend/src/main.tsx` | `frontend/src/router.tsx`; `frontend/src/services/api.ts`; `frontend/src/stores/useAppStore.ts` | Vite configuration and environment variables control the API base behavior. |
| Understand the frontend application | `okf/frontend/overview.md` | `okf/frontend/application-catalog.md`; `okf/frontend/diagrams/frontend-application-map.mmd` | The repository contains one management SPA, not multiple frontend applications or packages. |
| Change a frontend feature | `okf/frontend/feature-catalog.md` | Owning document under `okf/frontend/features`; `okf/frontend/feature-boundaries.md`; route page and tests | Route pages are the dominant observed feature boundaries; shared state, API types, and CSS cross them. |
| Change frontend routing or layout | `okf/frontend/routing.md` | `okf/frontend/pages-and-layouts.md`; `okf/frontend/component-architecture.md`; routing-flow diagram | All current routes are eager; token presence is a client admission cue, not authorization. |
| Change frontend state or session behavior | `okf/frontend/state-management.md` | `okf/frontend/authentication-and-access.md`; `useAppStore.ts`; `api.ts`; state-flow diagram | Keep Zustand, API module state, and browser persistence synchronized. |
| Change frontend API integration | `okf/frontend/api-client.md` | Matching frontend types, backend route/models, API-flow diagram | Manual TypeScript contracts do not prove server compatibility; preserve ACU-01/DC-01 until verified. |
| Change forms or validation | `okf/frontend/forms-and-validation.md` | Owning page; `DynamicForm.tsx`; backend validation | React Hook Form and Zod are declared but have no verified source use under DCC-11/DU-02. |
| Change the Page Builder | `okf/frontend/page-builder.md` | `okf/frontend/features/pages-and-page-builder.md`; page-builder flow; backend Pages and Marketplace adapter modules | Local preview, copied WebSocket URL, versions, workflow, and Marketplace adapters are distinct flows. |
| Review frontend risks or tests | `okf/frontend/frontend-risks.md` | `okf/frontend/testing-map.md`; owning feature document | Coverage percentages and browser quality remain unknown without generated or runtime evidence. |
| Trace an HTTP endpoint | `backend/src/routes/mod.rs` | The matching file under `backend/src/routes`; `backend/src/error.rs` | `/api` and `/api/v1` are separate route families; do not infer a general versioning policy from the public prefix alone. |
| Trace authentication | `backend/src/middleware/auth.rs` | `backend/src/routes/auth.rs`; `backend/src/services/jwt.rs`; `backend/src/services/password.rs`; `frontend/src/components/RequireAuth.tsx` | Authentication and organization selection are separate stages. |
| Trace tenant resolution | `backend/src/middleware/tenant.rs` | `backend/src/services/rls.rs`; `backend/src/db/mod.rs`; `backend/migrations/0009_v2_phase_three_rls.sql` | Verify both request-level ownership checks and database RLS behavior. |
| Trace authorization | `backend/src/services/rbac.rs` | Relevant route handler; role and membership migrations | Distinguish global roles, organization roles, and Marketplace permissions. |
| Understand the database | `okf/database/README.md` | `okf/database/schema-catalog.md`; owning entity document; current migrations and query code | OKF is navigation, not an executable schema; deployed migration state is unknown. |
| Find a database entity | `okf/database/entity-catalog.md` | Matching document under `okf/database/entities`; `schema-catalog.md` | Entity groups are documentation aggregates and can map to several tables. |
| Change a database entity or relationship | `okf/database/entity-catalog.md` | Owning entity document; `relationships.md`; `constraints-and-indexes.md`; chronological migrations | Create a forward migration in an authorized implementation phase; never silently edit applied history. |
| Create a database migration | `okf/database/migrations.md` | `backend/migrations`; `migration-lifecycle.mmd`; model/query/test consumers | Phase 5 is documentation-only; use a new ordered file only in an authorized implementation phase. |
| Find a constraint or index | `okf/database/constraints-and-indexes.md` | Final chronological migration definition; relevant entity document | Later migrations can replace earlier definitions; do not optimize from name similarity alone. |
| Understand database ownership | `okf/database/module-data-ownership.md` | `okf/backend/module-catalog.md`; route/service SQL callers | Lifecycle owner does not imply exclusive read/write access. |
| Find transaction logic | `okf/database/transactions-and-consistency.md` | Owning route/service; RLS transaction helper; external side effects | Database, filesystem, provider, cache, and webhook boundaries differ. |
| Change tenant-scoped persistence | `okf/database/multi-tenancy.md` | `tenant-isolation.mmd`; RLS migrations; middleware/service/query writers | Verify membership, explicit filters, RLS context, bypass paths, and parent-child tenant coherence. |
| Debug database tests | `okf/database/database-testing.md` | `okf/backend/testing-map.md`; CI services; fixtures and owning entity | PostgreSQL provisioning is not proof that a test executes real database assertions. |
| Understand core CMS behavior | `backend/src/routes/content.rs`; `backend/src/routes/pages.rs` | `frontend/src/pages/ContentTypesPage.tsx`; `frontend/src/pages/EntriesPage.tsx`; `frontend/src/pages/PagesPage.tsx` | Follow resource ownership and state transitions before relying on phase labels. |
| Understand Marketplace behavior | `backend/src/routes/marketplace.rs` | `backend/src/routes`; `backend/src/services/marketplace_runtime.rs`; migrations `0015` through `0026` | Uploaded Marketplace code is not an official arbitrary server-side execution mechanism. |
| Understand frontend routes and state | `frontend/src/router.tsx` | `frontend/src/components/AppShell.tsx`; `frontend/src/stores/useAppStore.ts`; `frontend/src/pages` | The single current Zustand store contains shared application state. |
| Understand localization and RTL | `frontend/src/i18n` | `frontend/src/styles/index.css`; localized page usage | Existing narrative i18n documentation conflicts with current source details under DCC-02. |
| Find frontend tests | `frontend/src/pages/DashboardPage.test.tsx` | `frontend/src/pages/MarketplacePage.test.tsx`; `frontend/src/pages/PagesPage.test.tsx`; `frontend/src/test/setup.ts` | Vitest and Testing Library configuration is in the frontend package and Vite setup. |
| Inspect CI gates | `.github/workflows/backend-ci.yml` | `.github/workflows/frontend-ci.yml` | CI covers checks and builds; no production deployment workflow was found. |
| Inspect deployment-related configuration | `docker-compose.prod.yml` | `backend/Dockerfile`; `frontend/Dockerfile`; `frontend/nginx.conf` | These files are production-like configuration, not proof of an actual deployment, beta, or GA state. |
| Find architecture diagrams | `docs/diagrams/README.md` | `docs/diagrams/TRACEABILITY.md`; selected `.mmd` files | Check diagram evidence and conflict notes before reusing a diagram as current behavior. |
| Review known gaps and conflicts | `okf-bootstrap/09-knowledge-gaps.md` | `okf-bootstrap/04-documentation-audit.md`; `okf-bootstrap/12-owner-questions.md`; `okf/index.yaml` | Keep unresolved markers visible in derived work. |

## AI Agent Rules

1. Read `AGENTS.md` and `HANDOFF.md` completely before changing the repository.
2. Inspect `git status`, `git diff`, and recent commits; preserve unrelated and uncommitted work.
3. Treat code, migrations, manifests, environment templates, and executable configuration as stronger evidence than narrative documentation.
4. Verify every referenced path and exact filename before using it; do not guess repository structure.
5. Use the [Project Glossary](glossary.md) and preserve the documented distinction between verified, inferred, ambiguous, and planned terminology.
6. Carry forward all relevant `UNKNOWN`, `NEEDS_OWNER_CONFIRMATION`, `DOCUMENTATION_CODE_CONFLICT`, `PLANNED_NOT_IMPLEMENTED`, and `IMPLEMENTATION_STATUS_UNCLEAR` markers.
7. Never convert a future plan, production-like configuration, or diagram into a claim of implemented or deployed behavior without primary evidence.
8. Do not expose secrets, credentials, tokens, private certificates, or local `.env` contents in repository documentation.
9. Keep repository documentation in English and use English file and folder names.
10. Update `HANDOFF.md` at meaningful milestones when the active task permits changes outside its scoped output directory.
11. Do not stage, commit, push, reset, clean, or discard changes without explicit user authorization.
12. Run validations appropriate to the changed files and report only tests or checks that were actually executed.

## When Documentation Is Missing

1. Search the repository with exact terms and path-aware patterns.
2. Inspect the nearest executable source, migration, manifest, environment template, or workflow.
3. Cross-check the relevant Phase Zero inventory and its cited evidence.
4. Record the conclusion as verified, inferred, ambiguous, planned, or implementation-status unclear.
5. Add or reuse an uncertainty marker rather than filling the gap with an assumption.
6. Propose an owner confirmation or a later OKF phase update; do not silently manufacture a source of truth.

## Related Documents

- [OKF Entry Point](../README.md)
- [Machine-Readable Index](../index.yaml)
- [Project Overview](overview.md)
- [Repository Map](repository-map.md)
- [Project Glossary](glossary.md)
- [Source Register](../references/source-register.md)
- [System Architecture](../architecture/README.md)
- [Architecture Overview](../architecture/overview.md)
- [System Boundaries](../architecture/boundaries.md)
- [Components and Responsibilities](../architecture/components.md)
- [Dependency Model](../architecture/dependency-model.md)
- [Runtime Flows](../architecture/runtime-flows.md)
- [Integration Points](../architecture/integration-points.md)
- [Architecture Risks](../architecture/architecture-risks.md)
- [Architecture Decision Register](../architecture/decisions/decision-register.md)
- [Backend Documentation](../backend/README.md)
- [Backend Module Catalog](../backend/module-catalog.md)
- [Backend Testing Map](../backend/testing-map.md)
- [Backend Risk Register](../backend/backend-risks.md)
- [Frontend Architecture](../frontend/README.md)
- [Frontend Feature Catalog](../frontend/feature-catalog.md)
- [Frontend Testing Map](../frontend/testing-map.md)
- [Frontend Risk Register](../frontend/frontend-risks.md)
- [Database Architecture](../database/README.md)
- [Database Schema Catalog](../database/schema-catalog.md)
- [Database Entity Catalog](../database/entity-catalog.md)
- [Database Risk Register](../database/database-risks.md)
