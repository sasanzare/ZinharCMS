---
okf_document_id: "repository-map"
title: "Repository Map"
project: "ZinharCMS"
category: "project"
phase: 1
status: "current"
review_status: "verified"
source_of_truth: false
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-19"
primary_sources:
  - ".gitignore"
  - "package.json"
  - "backend/src"
  - "backend/migrations"
  - "frontend/src"
  - "docs"
  - "scripts"
  - ".github/workflows"
  - "docker-compose.yml"
  - "docker-compose.prod.yml"
related_documents:
  - "README.md"
  - "index.yaml"
  - "project/overview.md"
  - "project/navigation-guide.md"
  - "project/glossary.md"
  - "references/source-register.md"
  - "backend/README.md"
  - "backend/module-catalog.md"
  - "backend/dependency-map.md"
  - "frontend/README.md"
  - "frontend/application-catalog.md"
  - "frontend/feature-catalog.md"
  - "frontend/feature-boundaries.md"
  - "frontend/testing-map.md"
uncertainty_markers:
  - "UNKNOWN U-01"
  - "UNKNOWN U-04"
  - "UNKNOWN U-05"
  - "UNKNOWN U-06"
  - "UNKNOWN U-11"
  - "UNKNOWN U-12"
  - "NEEDS_OWNER_CONFIRMATION NOC-08"
  - "NEEDS_OWNER_CONFIRMATION NOC-13"
  - "NEEDS_OWNER_CONFIRMATION NOC-14"
  - "NEEDS_OWNER_CONFIRMATION NOC-15"
  - "NEEDS_OWNER_CONFIRMATION NOC-17"
  - "DOCUMENTATION_CODE_CONFLICT DCC-09"
  - "INFERRED_FROM_STRUCTURE"
  - "FEATURE_BOUNDARY_UNCLEAR FBU-01"
  - "STATE_OWNERSHIP_UNCLEAR SOU-01"
  - "API_CONTRACT_UNCLEAR ACU-01"
---

# Repository Map

## Phase 6 API Knowledge Map

| Knowledge path | Source areas mapped | Purpose |
| --- | --- | --- |
| `okf/api/README.md` and primary API documents | `backend/src/routes`, `backend/src/middleware`, `backend/src/error.rs`, `frontend/src/services/api.ts` | API architecture, cross-cutting contracts, consistency, testing, and risks |
| `okf/api/groups/` | The 17 route-registration groups under `backend/src/routes` plus the root/static surface | Module-oriented route ownership |
| `okf/api/endpoints/` | 21 task-oriented endpoint families | Request, response, access, frontend, backend, persistence, and test navigation |
| `okf/api/diagrams/` | Router composition and request/security/client flows | Visual API navigation |

The authoritative route entry is `backend/src/routes/mod.rs`. Runtime OpenAPI is assembled there and exposed at `/openapi.json`. The administration client is centralized in `frontend/src/services/api.ts`, with manually maintained contracts in `frontend/src/types/api.ts`.

This map tells developers and AI agents where to look. It does not enumerate every file or replace the detailed documentation planned for later OKF phases.

## 1. Repository at a Glance

~~~text
ZinharCMS/
  .github/workflows/       Backend and frontend CI quality gates
  backend/
    migrations/            Forward SQLx migrations 0001 through 0026
    src/
      db/                  Pool and migration bootstrap
      middleware/          Authentication, tenant, and security middleware
      models/              Selected SQLx/domain data structures
      plugins/             Built-in host plugin registry
      routes/              Router groups, DTOs, handlers, and embedded SQL
      services/            Reusable policy, validation, integration, and domain logic
    Cargo.toml             Rust crate manifest
    Dockerfile             Development image
    Dockerfile.prod        Release image
  frontend/
    public/                Static browser assets
    src/
      components/          Shared shell, auth gate, form, and status UI
      hooks/               Shared React hooks
      i18n/                Locale dictionaries and direction handling
      pages/               Route-level administration screens and page tests
      services/            Browser API client
      stores/              Zustand application/session/organization state
      styles/              Shared application CSS
      test/                Vitest setup
      types/               Browser-side API contract types
    package.json           Frontend scripts and dependencies
    nginx.conf             Production-image SPA serving
  docs/                    Existing technical, phase, operations, and Marketplace docs
    diagrams/              Mermaid sources, conventions, evidence, and traceability
    marketplace-samples/   Creator package examples
  okf-bootstrap/           Phase Zero analytical reports
  okf/                     Incremental OKF navigation and knowledge layer
  scripts/                 Marketplace CLI and readiness/load/GA scripts
  docker-compose.yml       Local infrastructure services
  docker-compose.prod.yml  Production-like container definition
  package.json             Root developer command aliases
  README.md                Repository scope and quick start
~~~

Generated, dependency, cache, runtime-data, and local-secret paths are intentionally excluded from this tree.

## 2. Top-Level Paths

| Path | Purpose | Technology/classification | Responsibilities and entry points | Configuration/tests/docs | Planned OKF document |
| --- | --- | --- | --- | --- | --- |
| backend/ | Runnable API crate | Rust; runtime | main.rs starts the process; lib.rs exposes app composition; routes/mod.rs builds the router | Cargo.toml, Dockerfiles; tests colocated in source; docs/ARCHITECTURE.md and docs/API.md | Phases 2, 3, 6, and 7 |
| backend/migrations/ | Intended database schema history | SQL/PostgreSQL; runtime schema | Sequential forward migrations 0001-0026; seed schema/data and later domains | SQLx runs them from backend/src/db/mod.rs during startup | Phase 5 |
| frontend/ | Administration SPA | React/TypeScript/Vite; runtime | src/main.tsx and src/router.tsx; route pages under src/pages | package.json, Vite/TypeScript/ESLint/Vitest configs, Dockerfiles, nginx.conf; 3 page test files | Phase 4 |
| docs/ | Existing project knowledge and samples | Markdown, Mermaid, SQL/JSON/text; non-runtime except fixtures/samples | API, architecture, phase records, operator guides, Marketplace contracts, sample packages | docs/diagrams contains evidence/traceability; docs/sample-data.sql and V2 fixture support setup/testing | Phases 1-12 link or correct selected files |
| scripts/ | Developer and operational tooling | PowerShell and Node.js; tooling | Marketplace validate/pack/submit CLI; V2 GA, Phase 8 load, and V3 Phase 13-15 checks | Invoked from package.json or their documented phase/runbook sources | Phases 9 and 10 |
| .github/workflows/ | CI definitions | GitHub Actions; tooling | Backend format/Clippy/tests and frontend install/lint/typecheck/tests/build | backend-ci.yml and frontend-ci.yml | Phase 10 |
| okf-bootstrap/ | Phase Zero evidence and plan | Markdown; non-runtime supporting evidence | Repository, architecture, module, documentation, database, API, gap, and phase inventories | Committed at the Phase Zero checkpoint; source/config/code outrank it | Reference input for all phases |
| okf/ | Organized knowledge/navigation layer | Markdown, YAML, and Mermaid; non-runtime | README.md and index.yaml are entry points; project, architecture, and backend knowledge are current | Every Markdown file has front matter; index.yaml registers current/planned knowledge | Phases 1 through 3 current; later phases planned |
| docker-compose.yml | Local infrastructure definition | Docker Compose; runtime/development | PostgreSQL 16, Redis 7, pgAdmin | Uses named volumes and development credentials | Phase 2 and 10 |
| docker-compose.prod.yml | Production-like reference definition | Docker Compose; deployment reference | PostgreSQL, Redis, backend, frontend, and upload volume | Requires environment values and builds production Dockerfiles; actual production use is UNKNOWN U-01 | Phase 2 and 10 |
| package.json | Root command aliases | npm; tooling | Starts Compose infrastructure, backend, frontend, tests, frontend build, and Marketplace CLI | Does not make the repository an npm workspace | Phase 10 |
| .env.example and env.example | Non-secret configuration templates | Environment files; configuration | Database, Redis, JWT, uploads, CORS, rate limits, email, Stripe, and frontend URL names | .env is ignored and must not be copied into documentation | Phase 10 |
| README.md | Repository overview and quick start | Markdown; non-runtime | Current scope summary and basic commands | Source and manifests remain more authoritative | Phase 1 links it; do not duplicate phase history |
| AGENTS.md and HANDOFF.md | Agent protocol and cumulative recovery state | Markdown; operational process | Session recovery, safety, history, and exact next action | HANDOFF history is not canonical product architecture | Phase 10 contribution/operations references only |

## 3. Backend Navigation

Start with the Phase 3 [Backend Documentation](../backend/README.md). Use the [Module Catalog](../backend/module-catalog.md) to select one of the 18 significant module documents, then use the structural guides below to reach source evidence. The catalog is not duplicated here.

| Need | Inspect first | Notes |
| --- | --- | --- |
| Backend structure and module ownership | ../backend/overview.md; ../backend/module-catalog.md; ../backend/module-boundaries.md | One crate and process with documented source-level responsibility boundaries |
| Dependency and shared infrastructure | ../backend/dependency-map.md; ../backend/shared-infrastructure.md | Includes reverse-layer imports and high-coupling shared contracts |
| Request, error, and test behavior | ../backend/request-handling.md; ../backend/error-handling.md; ../backend/testing-map.md | Phase 3 maps patterns and gaps without becoming an endpoint reference |
| Application entry point | ../../backend/src/main.rs; ../../backend/src/lib.rs | Loads configuration, prepares dependencies, runs migrations, builds layers/router, binds listener, and handles shutdown |
| Route registration | ../../backend/src/routes/mod.rs | Composes public, authentication-only, and tenant-protected route stacks and generated OpenAPI registration |
| Request handlers and DTOs | ../../backend/src/routes/*.rs | Most request/response types and many SQL queries are colocated with handlers |
| Services and reusable policy | ../../backend/src/services/*.rs | Validation, workflow, JWT/password, RBAC/RLS, cache, quota, integrations, and Marketplace policies |
| Domain logic | ../../backend/src/services plus relevant route file | No dedicated domain directory exists; orchestration and rules are distributed |
| Data access | ../../backend/src/routes; ../../backend/src/services; ../../backend/src/db/mod.rs | SQLx queries are handwritten; no repository abstraction layer was found |
| Models | ../../backend/src/models | Selected SQLx/domain structs only; not a complete model of every table or DTO |
| Configuration | ../../backend/src/config.rs; ../../.env.example | Environment-driven Config; do not inspect or document local .env values |
| Authentication | ../../backend/src/routes/auth.rs; ../../backend/src/middleware/auth.rs; ../../backend/src/services/password.rs; ../../backend/src/services/jwt.rs; ../../backend/src/services/security.rs | Bearer access token plus refresh-token lifecycle |
| Authorization and tenancy | ../../backend/src/middleware/tenant.rs; ../../backend/src/services/rbac.rs; ../../backend/src/services/rls.rs | Organization selection, membership, global/org roles, quota/rate gates, and PostgreSQL RLS context |
| Background processing | ../../backend/src/services/webhooks.rs; ../../backend/src/state.rs | No durable worker exists; webhook work and preview broadcast are process-local |
| Tests | Search for test attributes under ../../backend/src | Tests are colocated with modules; no standalone backend/tests directory was found |
| Migrations | ../../backend/migrations | Intended schema authority; startup runner is backend/src/db/mod.rs |
| Built-in plugins | ../../backend/src/plugins | Host-owned CmsPlugin trait, registry, and SEO implementation; separate from Marketplace products |

## 4. Frontend Navigation

| Need | Inspect first | Notes |
| --- | --- | --- |
| Application entry | ../../frontend/src/main.tsx | Mounts RouterProvider inside I18nProvider and imports global styles |
| Routing | ../../frontend/src/router.tsx | Defines login plus authenticated administration child routes |
| Pages/views | ../../frontend/src/pages | Route-level feature screens; MarketplacePage, PagesPage, and OrganizationPage span multiple concerns |
| Shared components | ../../frontend/src/components | AppShell, RequireAuth, DynamicForm, and StatusBadge |
| State management | ../../frontend/src/stores/useAppStore.ts | The only current Zustand store; session, user, organizations, active organization, and shell state |
| API client | ../../frontend/src/services/api.ts | Central fetch wrapper, bearer/organization headers, credentials, and all domain client groups |
| API types | ../../frontend/src/types/api.ts | Manually maintained TypeScript request/response contracts |
| Authentication state/UI | ../../frontend/src/pages/AuthPage.tsx; ../../frontend/src/components/RequireAuth.tsx; ../../frontend/src/stores/useAppStore.ts; ../../frontend/src/services/api.ts | Browser persistence and route guard behavior |
| Localization | ../../frontend/src/i18n | English/Persian dictionaries, provider, hooks, locale metadata, and RTL/LTR behavior |
| Styling | ../../frontend/src/styles/index.css; ../../frontend/vite.config.ts | Shared CSS plus Tailwind Vite plugin |
| Assets | ../../frontend/public | Static browser assets, currently including the Vazirmatn font |
| Tests | ../../frontend/src/pages/DashboardPage.test.tsx; ../../frontend/src/pages/PagesPage.test.tsx; ../../frontend/src/pages/MarketplacePage.test.tsx; ../../frontend/src/test/setup.ts | Vitest/Testing Library in jsdom |
| Build configuration | ../../frontend/package.json; ../../frontend/vite.config.ts; ../../frontend/tsconfig.json; ../../frontend/tsconfig.app.json; ../../frontend/tsconfig.node.json; ../../frontend/eslint.config.js; ../../frontend/vitest.config.ts; ../../frontend/Dockerfile; ../../frontend/Dockerfile.prod; ../../frontend/nginx.conf | CI uses Node 22; Dockerfiles use Node 24 |
| Frontend OKF entry | ../frontend/README.md | Phase 4 current-state frontend architecture and reading order |
| Frontend application map | ../frontend/application-catalog.md; ../frontend/diagrams/frontend-application-map.mmd | One verified management SPA; source directories are not separate applications |
| Frontend feature ownership | ../frontend/feature-catalog.md; ../frontend/feature-boundaries.md; ../frontend/features | Thirteen significant features with route, source, state, API, access, and test ownership |
| Frontend risks and tests | ../frontend/frontend-risks.md; ../frontend/testing-map.md | Evidence-based risk register and observed three-file test surface |

## 5. Database Navigation

| Need | Path | Status |
| --- | --- | --- |
| Database documentation entry | ../database/README.md | Phase 5 reading order and change-safety rules |
| Complete schema inventory | ../database/schema-catalog.md | 51 application tables and other migration-defined objects |
| Domain entity selection | ../database/entity-catalog.md; ../database/entities | 18 significant entity groups |
| Relationships and ownership | ../database/relationships.md; ../database/module-data-ownership.md | 108 intended active FKs consolidated into 55 relationship groups |
| Tenant isolation | ../database/multi-tenancy.md | 32 forced-RLS tables and context/bypass behavior |
| Migration and constraint safety | ../database/migrations.md; ../database/constraints-and-indexes.md | 26 forward migrations and 109 explicit index names |
| Migration history/schema | ../../backend/migrations | Primary intended schema evidence; 26 forward migrations |
| Migration runner/pool | ../../backend/src/db/mod.rs | SQLx pool configuration and embedded migration runner |
| Selected models/entities | ../../backend/src/models | Partial model layer; many rows/DTOs live in route/service files |
| Query code | ../../backend/src/routes and ../../backend/src/services | Handwritten SQLx SQL is distributed across domain files |
| Baseline seed data | ../../backend/migrations/0002_seed_foundation_data.sql | Database seed migration |
| Startup admin seed | ../../backend/src/main.rs | Seeds a default admin only when no users exist; verify environment implications before use |
| Database configuration | ../../backend/src/config.rs; ../../.env.example; Compose files | DATABASE_URL and service configuration |
| Tenant/RLS fixture | ../../docs/V2_PHASE_EIGHT_FIXTURE.sql | V2 hardening fixture |

Use the Phase 5 catalog first, then verify every change against current migrations and query code. Runtime migration state remains unknown.

## 6. API Navigation

| Need | Path | Notes |
| --- | --- | --- |
| Reachable route composition | ../../backend/src/routes/mod.rs | Start here; it defines the security stacks and merged domain routers |
| Domain routes | ../../backend/src/routes/*.rs | Method/path registration, handlers, and DTOs by domain |
| Public API versioning | ../../backend/src/routes/delivery.rs | Public delivery uses /api/v1; administrative APIs use /api; policy is NEEDS_OWNER_CONFIRMATION NOC-08 |
| Request/response types | Route modules; ../../frontend/src/types/api.ts | Backend types are authoritative; frontend types are manually synchronized |
| Errors | ../../backend/src/error.rs | AppError and the error/message JSON envelope |
| Authentication middleware | ../../backend/src/middleware/auth.rs | JWT extraction/verification; preview supports query-token paths |
| Tenant/authorization middleware | ../../backend/src/middleware/tenant.rs; ../../backend/src/services/rbac.rs; ../../backend/src/services/rls.rs | Tenant selection, membership, role helpers, and database context |
| OpenAPI | ../../backend/src/routes/mod.rs and route annotations | Generated endpoint is /openapi.json; coverage is partial |
| Manual API guide | ../../docs/API.md | Broad behavior reference with DOCUMENTATION_CODE_CONFLICT DCC-09 |
| API tests | Colocated backend tests and frontend page tests | No comprehensive standalone API integration suite directory was found |

## 7. Documentation Navigation

| Need | Path |
| --- | --- |
| Project-level overview | ../../README.md |
| Technical architecture | ../../docs/ARCHITECTURE.md |
| Manual API guide | ../../docs/API.md |
| Localization guidance | ../../docs/I18N.md |
| Original CMS phase records | ../../docs/PHASE_*.md |
| V2 SaaS phases/guides/runbook/release notes | ../../docs/V2_*.md |
| V3 Marketplace scope, policy, domain, phases, guide, runbook, release notes | ../../docs/V3_*.md and ../../docs/MARKETPLACE_CREATOR_GUIDE.md |
| Mermaid diagrams | ../../docs/diagrams/*.mmd |
| Diagram conventions/catalog/evidence | ../../docs/diagrams/README.md; ../../docs/diagrams/DIAGRAM_CONVENTIONS.md; ../../docs/diagrams/TRACEABILITY.md; ../../docs/diagrams/FILE_EVIDENCE_INDEX.md |
| Deployment and operations guides | ../../docs/V2_OPERATIONS_RUNBOOK.md; ../../docs/V3_MARKETPLACE_OPERATIONS_RUNBOOK.md |
| Security material | ../../docs/PHASE_SEVEN.md; ../../docs/V2_PHASE_EIGHT.md; ../../docs/V3_PHASE_THIRTEEN.md; Marketplace policy/scope docs |
| Troubleshooting and recovery | V2/V3 operations runbooks; ../../HANDOFF.md for current operational checkpoint only |
| Contribution guide/ownership | No CONTRIBUTING.md or CODEOWNERS file found; UNKNOWN U-12, NEEDS_OWNER_CONFIRMATION NOC-14, and NEEDS_OWNER_CONFIRMATION NOC-15 |
| Phase Zero reports | ../../okf-bootstrap |
| Current OKF | ../README.md; ../index.yaml; this project directory; ../references |

## 8. Infrastructure and Operations Navigation

| Need | Path | Finding |
| --- | --- | --- |
| Local containers | ../../docker-compose.yml | PostgreSQL, Redis, and pgAdmin |
| Production-like containers | ../../docker-compose.prod.yml | PostgreSQL, Redis, backend, frontend, and upload volume; actual production use is UNKNOWN U-01 |
| Backend images | ../../backend/Dockerfile; ../../backend/Dockerfile.prod | Development and release builds |
| Frontend images/reverse proxy | ../../frontend/Dockerfile; ../../frontend/Dockerfile.prod; ../../frontend/nginx.conf | Vite development image and Nginx static SPA image; Nginx does not proxy the API |
| CI | ../../.github/workflows/backend-ci.yml; ../../.github/workflows/frontend-ci.yml | Quality checks only; no deployment job |
| Operational scripts | ../../scripts | GA/readiness/load scripts and Marketplace CLI |
| Environment templates | ../../.env.example; ../../env.example | Variable names only; local .env may contain secrets and is not a knowledge source |
| Logging | ../../backend/src/main.rs; ../../backend/src/config.rs | tracing subscriber and RUST_LOG; no central collector configuration found |
| Observability | /health and /ready handlers; readiness/load scripts | No metrics exporter, APM, dashboards, or SLO configuration found |
| Backups | No repository backup script or platform configuration found | UNKNOWN U-04 |
| Database migrations | ../../backend/migrations; ../../backend/src/db/mod.rs | Applied automatically at backend startup |
| Deployment pipeline | No workflow or script found | UNKNOWN U-06 |

## 9. Common Navigation Scenarios

### I need to change authentication.

Read [Project Overview](overview.md), then inspect backend/src/routes/auth.rs, backend/src/middleware/auth.rs, backend/src/services/jwt.rs, backend/src/services/password.rs, backend/src/services/security.rs, frontend/src/pages/AuthPage.tsx, frontend/src/stores/useAppStore.ts, frontend/src/services/api.ts, backend/migrations/0001_initial_schema.sql, backend/migrations/0007_phase_seven_security.sql, and related tests. Security details remain planned for OKF Phase 7.

### I need to add a backend API endpoint.

Start with backend/src/routes/mod.rs, then the nearest domain route module, AppError, relevant middleware/service/model/migration, Utoipa registration, frontend API/types if consumed by the SPA, colocated tests, and docs/API.md. Verify whether the route belongs to the public, authentication-only, or tenant stack.

### I need to add a frontend page.

Start with [Frontend Routing](../frontend/routing.md), [Pages and Layouts](../frontend/pages-and-layouts.md), and the owning [Feature Catalog](../frontend/feature-catalog.md) entry. Then inspect frontend/src/router.tsx, frontend/src/components/AppShell.tsx, the nearest page under frontend/src/pages, frontend/src/services/api.ts, frontend/src/types/api.ts, frontend/src/stores/useAppStore.ts, frontend/src/i18n/messages.ts, frontend/src/styles/index.css, existing page tests, and frontend build/test configuration.

### I need to modify a database entity.

Inspect the complete backend/migrations sequence affecting the entity, SQL queries in routes/services, selected models, RLS helpers/policies, tests, fixtures, and related API/frontend contracts. Do not edit an old applied migration without an explicit migration strategy; detailed database workflow is documented in Phase 5.

### I need to understand a module.

Use [Navigation Guide](navigation-guide.md), identify the capability in [Project Overview](overview.md#4-major-capabilities), then select the owning [Backend Module Catalog](../backend/module-catalog.md) or [Frontend Feature Catalog](../frontend/feature-catalog.md) entry. Follow route registration to the relevant handler/service/migration/frontend page, then read tests and current documentation.

### I need to debug a failing test.

Identify whether the failure is backend or frontend. For backend tests, start with the colocated test module, backend/Cargo.toml, .github/workflows/backend-ci.yml, required database/Redis configuration, and backend/migrations. For frontend tests, inspect the matching page test, frontend/src/test/setup.ts, frontend/vitest.config.ts, API mocks, and .github/workflows/frontend-ci.yml. Check HANDOFF.md only for current known environment/tooling issues; verify them before assuming they still apply.

## 10. Generated and Ignored Content

The following paths should not normally be used as project-knowledge sources:

| Path/pattern | Reason |
| --- | --- |
| backend/target and root target | Rust build output |
| frontend/node_modules | Installed dependencies |
| frontend/dist | Frontend production build output |
| frontend/.vite when generated | Vite cache |
| marketplace-dist | Ignored Marketplace ZIP output generated by the CLI |
| backend/uploads and root uploads when present | Runtime media/artifact data, not source |
| .env and .env.* except .env.example | Local configuration may contain secrets; ignored by Git |
| *.log, including frontend/dev-server*.log | Runtime/tool output |
| *.tmp | Temporary files |
| .agents | Local untracked agent directory with no current repository knowledge |

.gitignore and the component .dockerignore files are the authority for exclusion behavior. UNKNOWN U-11 and NEEDS_OWNER_CONFIRMATION NOC-17 cover the retention policy for ignored Marketplace distribution archives.

## Related Documents

- [Project Overview](overview.md)
- [Navigation Guide](navigation-guide.md)
- [Project Glossary](glossary.md)
- [OKF Entry Point](../README.md)
- [Machine-Readable Index](../index.yaml)
- [Source Register](../references/source-register.md)
- [Frontend Architecture](../frontend/README.md)
- [Frontend Application Catalog](../frontend/application-catalog.md)
- [Frontend Feature Catalog](../frontend/feature-catalog.md)
- [Frontend Testing Map](../frontend/testing-map.md)
- [Database Architecture](../database/README.md)
- [Database Schema Catalog](../database/schema-catalog.md)
- [Database Entity Catalog](../database/entity-catalog.md)

## Security Documentation and Evidence Map

The Phase 7 entry point is [Authentication, Authorization, and Security Architecture](../security/README.md). Primary implementation evidence is under `backend/src/routes/auth.rs`, `backend/src/middleware/`, `backend/src/services/{jwt,password,security,rbac,rls,audit}.rs`, identity/organization/RLS/security migrations, and `frontend/src/{services,stores,components,pages}`. Role and permission navigation is in the [Roles and Permissions Catalog](../security/roles-and-permissions-catalog.md); threats and gaps are in [Security Risks](../security/security-risks.md).

## Business Rules and Domain Workflow Map

The Phase 8 entry point is [Business Rules and Domain Workflows](../domain/README.md). `okf/domain/domains/` maps ten significant domains to backend routes, services, models, migrations, frontend consumers, and adjacent OKF documents. `okf/domain/workflows/` records 14 end-to-end workflows. `okf/domain/diagrams/` contains six visual summaries. Start with the [Domain Catalog](../domain/domain-catalog.md), [Business Rule Catalog](../domain/business-rule-catalog.md), and [Cross-Module Workflows](../domain/cross-module-workflows.md); use [Domain Risks](../domain/domain-risks.md) for incomplete or non-atomic behavior.

## Phase 9 Repository Map

The Phase 9 entry point is [Plugins, Marketplace, and Extensibility](../extensibility/README.md). The extensibility/plugins directory contains only verified concrete plugins, extensibility/extension-points contains significant verified contracts, extensibility/marketplace covers implemented Marketplace areas, and extensibility/diagrams contains seven source-aligned Mermaid views.

## Development, Delivery, Operations, and Maintenance Paths

| Path | Verified role |
| --- | --- |
| `package.json` | Root orchestration for infrastructure, component dev/test/build, and Marketplace CLI |
| `backend/Cargo.toml`, `backend/Cargo.lock` | Backend package, dependencies, and lock |
| `frontend/package.json`, `frontend/package-lock.json` | Frontend scripts, dependencies, and lock |
| `.github/workflows/` | Two path-filtered CI workflows; no release or deployment workflow |
| `backend/Dockerfile*`, `frontend/Dockerfile*` | Development and production-like image builds |
| `docker-compose.yml` | Local PostgreSQL, Redis, and pgAdmin |
| `docker-compose.prod.yml` | Production-like PostgreSQL, Redis, backend, frontend, and volume assembly; not deployed-state evidence |
| `frontend/nginx.conf` | Static SPA fallback; no API proxy/TLS configuration |
| `.env.example`, `env.example`, `backend/src/config.rs` | Environment variable names, defaults, and validation |
| `backend/src/main.rs`, `backend/src/db/mod.rs` | Startup, migrations, seed, middleware, bind, graceful shutdown |
| `scripts/` | Marketplace CLI, load smoke, beta/readiness, and GA checks; no deploy/backup/restore script |
| `docs/V2_OPERATIONS_RUNBOOK.md`, `docs/V3_MARKETPLACE_OPERATIONS_RUNBOOK.md` | Operational intent with recovery/deployment assumptions that require verification |
| `okf/development/`, `okf/delivery/`, `okf/operations/`, `okf/maintenance/` | Phase 10 knowledge, diagrams, validation, conflicts, and maintenance policy |

See [Development](../development/README.md), [Delivery](../delivery/README.md), [Operations](../operations/README.md), and [Maintenance](../maintenance/README.md).
