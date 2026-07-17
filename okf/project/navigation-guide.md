---
okf_document_id: "navigation-guide"
title: "Navigation Guide"
project: "ZinharCMS"
category: "project"
phase: 1
status: "current"
review_status: "verified"
source_of_truth: false
last_verified_commit: "17e69e266c558c8568ec65524560d52d7cb89d4c"
last_verified_date: "2026-07-17"
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
---

# Navigation Guide

Use this guide to choose the shortest evidence path for a common repository task. The [Repository Map](repository-map.md) provides the fuller structural inventory, and the [Source Register](../references/source-register.md) explains the reliability of the referenced sources.

## Start Here

1. Read the root [README](../../README.md) for the project introduction and local-development commands.
2. Read the [Project Overview](overview.md) for the evidence-based Phase 1 summary and system boundary.
3. Use the [Repository Map](repository-map.md) to locate application, infrastructure, test, and documentation areas.
4. Check the [Project Glossary](glossary.md) before introducing or interpreting domain terminology.
5. Read the [Architecture Overview](../architecture/overview.md) for system classification, runtime shape, and deployment-evidence limits.
6. Use the [System Architecture](../architecture/README.md) reading order for boundaries, components, dependencies, flows, integrations, risks, decisions, and diagrams.
7. Consult the [Source Register](../references/source-register.md) to distinguish primary evidence from supporting or conflicting documentation.
8. Review the [Machine-Readable Index](../index.yaml) for document status, relationships, and uncertainty markers.
9. When a claim remains unresolved, follow its marker into [Phase Zero Knowledge Gaps](../../okf-bootstrap/09-knowledge-gaps.md) or [Owner Questions](../../okf-bootstrap/12-owner-questions.md).

## Navigate by Task

| Task | Start with | Continue with | Important note |
|---|---|---|---|
| Understand the product and implemented scope | `README.md` | `okf/project/overview.md`; `docs/V3_MARKETPLACE_SCOPE.md` | Treat product claims as supporting evidence until confirmed in code, configuration, or migrations. |
| Understand the current architecture | `okf/architecture/overview.md` | `okf/architecture/diagrams/system-context.mmd`; `okf/architecture/diagrams/container-view.mmd` | The backend is one modular-monolith process, not a microservice set. |
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
| Trace an HTTP endpoint | `backend/src/routes/mod.rs` | The matching file under `backend/src/routes`; `backend/src/error.rs` | `/api` and `/api/v1` are separate route families; do not infer a general versioning policy from the public prefix alone. |
| Trace authentication | `backend/src/middleware/auth.rs` | `backend/src/routes/auth.rs`; `backend/src/services/jwt.rs`; `backend/src/services/password.rs`; `frontend/src/components/RequireAuth.tsx` | Authentication and organization selection are separate stages. |
| Trace tenant resolution | `backend/src/middleware/tenant.rs` | `backend/src/services/rls.rs`; `backend/src/db/mod.rs`; `backend/migrations/0009_v2_phase_three_rls.sql` | Verify both request-level ownership checks and database RLS behavior. |
| Trace authorization | `backend/src/services/rbac.rs` | Relevant route handler; role and membership migrations | Distinguish global roles, organization roles, and Marketplace permissions. |
| Understand the database | `backend/migrations` | `backend/src/db/mod.rs`; route query code | The migrations are the schema authority; narrative schema diagrams may be simplified or stale. |
| Understand core CMS behavior | `backend/src/routes/content.rs`; `backend/src/routes/pages.rs` | `frontend/src/pages/ContentTypesPage.tsx`; `frontend/src/pages/EntriesPage.tsx`; `frontend/src/pages/PagesPage.tsx` | Follow resource ownership and state transitions before relying on phase labels. |
| Understand Marketplace behavior | `backend/src/routes/marketplace.rs` | `backend/src/routes`; `backend/src/services/marketplace_runtime.rs`; migrations `0015` through `0026` | Uploaded Marketplace code is not an official arbitrary server-side execution mechanism. |
| Understand frontend routes and state | `frontend/src/router.tsx` | `frontend/src/components/AppShell.tsx`; `frontend/src/stores/useAppStore.ts`; `frontend/src/pages` | The single current Zustand store contains shared application state. |
| Understand localization and RTL | `frontend/src/i18n` | `frontend/src/styles/index.css`; localized page usage | Existing narrative i18n documentation conflicts with current source details under DCC-02. |
| Find backend tests | Search `#[cfg(test)]` and `#[test]` under `backend/src` | Module-local test sections; `.github/workflows/backend-ci.yml` | Backend tests are colocated with source; there is no current `backend/tests` directory. |
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
