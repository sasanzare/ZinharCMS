---
okf_document_id: "source-register"
title: "Source Register"
project: "ZinharCMS"
category: "references"
phase: 1
status: "current"
review_status: "verified"
source_of_truth: false
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources:
  - "README.md"
  - "package.json"
  - "backend/Cargo.toml"
  - "backend/src"
  - "backend/migrations"
  - "frontend/package.json"
  - "frontend/src"
  - "docker-compose.yml"
  - "docker-compose.prod.yml"
  - ".github/workflows"
  - "docs"
  - "okf-bootstrap"
related_documents:
  - "README.md"
  - "index.yaml"
  - "project/overview.md"
  - "project/repository-map.md"
  - "project/glossary.md"
  - "project/navigation-guide.md"
  - "architecture/README.md"
  - "architecture/overview.md"
  - "architecture/boundaries.md"
  - "architecture/components.md"
  - "architecture/dependency-model.md"
  - "architecture/runtime-flows.md"
  - "architecture/integration-points.md"
  - "architecture/architecture-risks.md"
  - "architecture/decisions/README.md"
  - "architecture/decisions/decision-register.md"
  - "backend/README.md"
  - "backend/module-catalog.md"
  - "backend/testing-map.md"
  - "frontend/README.md"
  - "frontend/application-catalog.md"
  - "frontend/feature-catalog.md"
  - "frontend/testing-map.md"
  - "frontend/frontend-risks.md"
uncertainty_markers:
  - "UNKNOWN U-01"
  - "UNKNOWN U-02"
  - "UNKNOWN U-03"
  - "UNKNOWN U-04"
  - "UNKNOWN U-05"
  - "UNKNOWN U-06"
  - "UNKNOWN U-07"
  - "UNKNOWN U-08"
  - "UNKNOWN U-09"
  - "UNKNOWN U-10"
  - "UNKNOWN U-11"
  - "UNKNOWN U-12"
  - "UNKNOWN U-13"
  - "UNKNOWN U-14"
  - "UNKNOWN U-15"
  - "NEEDS_OWNER_CONFIRMATION NOC-01"
  - "NEEDS_OWNER_CONFIRMATION NOC-02"
  - "NEEDS_OWNER_CONFIRMATION NOC-03"
  - "NEEDS_OWNER_CONFIRMATION NOC-04"
  - "NEEDS_OWNER_CONFIRMATION NOC-05"
  - "NEEDS_OWNER_CONFIRMATION NOC-06"
  - "NEEDS_OWNER_CONFIRMATION NOC-07"
  - "NEEDS_OWNER_CONFIRMATION NOC-08"
  - "NEEDS_OWNER_CONFIRMATION NOC-09"
  - "NEEDS_OWNER_CONFIRMATION NOC-10"
  - "NEEDS_OWNER_CONFIRMATION NOC-11"
  - "NEEDS_OWNER_CONFIRMATION NOC-12"
  - "NEEDS_OWNER_CONFIRMATION NOC-13"
  - "NEEDS_OWNER_CONFIRMATION NOC-14"
  - "NEEDS_OWNER_CONFIRMATION NOC-15"
  - "NEEDS_OWNER_CONFIRMATION NOC-16"
  - "NEEDS_OWNER_CONFIRMATION NOC-17"
  - "NEEDS_OWNER_CONFIRMATION NOC-18"
  - "DOCUMENTATION_CODE_CONFLICT DCC-01"
  - "DOCUMENTATION_CODE_CONFLICT DCC-02"
  - "DOCUMENTATION_CODE_CONFLICT DCC-03"
  - "DOCUMENTATION_CODE_CONFLICT DCC-04"
  - "DOCUMENTATION_CODE_CONFLICT DCC-05"
  - "DOCUMENTATION_CODE_CONFLICT DCC-06"
  - "DOCUMENTATION_CODE_CONFLICT DCC-07"
  - "DOCUMENTATION_CODE_CONFLICT DCC-08"
  - "DOCUMENTATION_CODE_CONFLICT DCC-09"
  - "DOCUMENTATION_CODE_CONFLICT DCC-10"
  - "PLANNED_NOT_IMPLEMENTED PNI-01"
  - "PLANNED_NOT_IMPLEMENTED PNI-02"
  - "PLANNED_NOT_IMPLEMENTED PNI-03"
  - "PLANNED_NOT_IMPLEMENTED PNI-04"
  - "PLANNED_NOT_IMPLEMENTED PNI-05"
  - "IMPLEMENTATION_STATUS_UNCLEAR ISU-01"
  - "IMPLEMENTATION_STATUS_UNCLEAR ISU-02"
  - "IMPLEMENTATION_STATUS_UNCLEAR ISU-03"
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR ABU-01"
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR ABU-02"
  - "ARCHITECTURAL_BOUNDARY_UNCLEAR ABU-03"
  - "DEPENDENCY_DIRECTION_UNCLEAR DDU-01"
  - "DEPENDENCY_DIRECTION_UNCLEAR DDU-02"
  - "DEPENDENCY_DIRECTION_UNCLEAR DDU-03"
  - "DOCUMENTATION_CODE_CONFLICT DCC-11"
  - "FEATURE_BOUNDARY_UNCLEAR FBU-01"
  - "COMPONENT_OWNERSHIP_UNCLEAR COU-01"
  - "STATE_OWNERSHIP_UNCLEAR SOU-01"
  - "API_CONTRACT_UNCLEAR ACU-01"
  - "AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01"
  - "DUPLICATED_CONTRACT DC-01"
  - "DEAD_OR_UNUSED_CODE_UNCONFIRMED DU-02"
  - "UI_BEHAVIOR_UNVERIFIED UBU-01"
---

# Source Register

## Phase 6 API Sources

| Source | Authority and use | Phase 6 finding |
| --- | --- | --- |
| `backend/src/routes/mod.rs` | Authoritative router composition and Utoipa root | 168 handlers across public, authenticated, and tenant subtrees; 149 listed in OpenAPI |
| `backend/src/routes/*.rs` | Authoritative registered handler paths, extractors, DTOs, statuses, validation, and route-level policy | 17 significant route modules and 21 endpoint families |
| `backend/src/middleware/auth.rs` | Bearer authentication contract | Standard header plus preview query-token exception |
| `backend/src/middleware/tenant.rs` | Tenant selection, membership, rate, and quota contract | `X-Organization-Id` plus preview query-organization exception |
| `backend/src/services/rbac.rs` | Reusable global and organization capability checks | Role matrix documented in `api/authorization.md` |
| `backend/src/error.rs` | Application error status and body mapping | Nine `AppError` variants; framework responses remain non-uniform |
| `frontend/src/services/api.ts` | Authoritative shared frontend request construction | 141 current request functions, all matching registered method/path pairs |
| `frontend/src/types/api.ts` | Handwritten frontend DTO projection | Duplicated contract; not generated from Rust or OpenAPI |
| `docs/API.md` | Secondary manual documentation | `DOCUMENTATION_CODE_CONFLICT DCC-09`: obsolete billing webhook path |

See [API OpenAPI Consistency](../api/openapi-consistency.md) for the exact 19 missing operations and security-scheme gap.

This register records the material evidence used by the current Phase 1 project documents and Phase 2 architecture documents. It supports the [Project Overview](../project/overview.md), [System Architecture](../architecture/README.md), [Repository Map](../project/repository-map.md), [Project Glossary](../project/glossary.md), [Navigation Guide](../project/navigation-guide.md), [OKF Entry Point](../README.md), and [Machine-Readable Index](../index.yaml).

The Phase 1 baseline entries were checked against commit `49b2c1886168497e99f7086e4941b21616985656`; the Phase 2 architecture-specific entries were checked against commit `17e69e266c558c8568ec65524560d52d7cb89d4c`. Both verifications occurred on `2026-07-17`. The register does not promote narrative documentation to implementation authority.

## Reliability Scale

| Reliability | Meaning |
|---|---|
| PRIMARY | Executable code, migration, manifest, configuration, workflow, or environment contract that directly establishes current repository behavior. |
| SUPPORTING | Narrative, diagram, audit, or generated inventory that explains or summarizes primary evidence. |
| INFERRED | A conclusion derived from multiple primary sources rather than explicitly declared in one source. |
| OUTDATED_OR_CONFLICTING | A source containing useful history or intent but known to conflict with stronger current evidence. |

## Project Manifests and Entry Documents

| Path | Source type | Information derived | Reliability | Verified commit | Notes and conflicts |
|---|---|---|---|---|---|
| `README.md` | Root project introduction | Project identity, high-level scope, quick start, local commands, API and documentation links | SUPPORTING | `49b2c188` | Cross-check operational claims against manifests and Compose files. |
| `package.json` | Root npm manifest | Workspace orchestration, test/build commands, Marketplace CLI entry | PRIMARY | `49b2c188` | `dev` invokes the default Compose file, which currently contains infrastructure services only. |
| `backend/Cargo.toml` | Rust crate manifest | Backend package identity, Rust dependencies, declared crate license | PRIMARY | `49b2c188` | Crate license does not establish a repository-wide license file; U-10 and NOC-16 remain open. |
| `backend/Cargo.lock` | Rust lockfile | Resolved backend dependency graph | PRIMARY | `49b2c188` | Dependency versions are implementation evidence, not a product capability statement. |
| `frontend/package.json` | Frontend npm manifest | React/Vite stack, scripts, runtime and test dependencies | PRIMARY | `49b2c188` | Use with the lockfile and Vite configuration. |
| `frontend/package-lock.json` | Frontend lockfile | Resolved frontend dependency graph | PRIMARY | `49b2c188` | Generated dependency resolution. |
| `AGENTS.md` | Repository agent instructions | Persistent handoff, safety, validation, and mistake-log requirements | PRIMARY | `49b2c188` | Governs repository work, not product runtime. |
| `HANDOFF.md` | Persistent work handoff | Completed work, actual validations, remaining tasks, exact next action | SUPPORTING | `49b2c188` | Repository and Git state take precedence if they differ. |

## Backend Sources

| Path | Source type | Information derived | Reliability | Verified commit | Notes and conflicts |
|---|---|---|---|---|---|
| `backend/src/main.rs` | Backend executable entry | Configuration loading, database setup, shared state, router startup | PRIMARY | `49b2c188` | Start here for runtime composition. |
| `backend/src/lib.rs` | Backend library root | Exported module boundaries | PRIMARY | `49b2c188` | Confirms available application modules. |
| `backend/src/config.rs` | Configuration module | Required environment variables and runtime defaults | PRIMARY | `49b2c188` | Use with `.env.example` and `env.example`. |
| `backend/src/state.rs` | Shared application state | Configuration, PostgreSQL pool, Redis client, and process-local page preview channels | PRIMARY | `debde202` | Confirms the exact four-field `AppState` composition. |
| `backend/src/error.rs` | Error model | Application error categories and HTTP response mapping | PRIMARY | `49b2c188` | More reliable than narrative API error examples; DCC-09 applies. |
| `backend/src/db/mod.rs` | Database integration | Connection, migrations, tenant transaction context | PRIMARY | `49b2c188` | Important for RLS behavior. |
| `backend/src/routes/mod.rs` | Router composition | Route module registration, middleware layering, `/api` and `/api/v1` boundaries | PRIMARY | `49b2c188` | Canonical route-family entry point. |
| `backend/src/routes/auth.rs` | HTTP handlers | Authentication endpoints and response contracts | PRIMARY | `49b2c188` | Pair with auth middleware and JWT/password services. |
| `backend/src/routes/content.rs` | HTTP handlers | Content types, entries, validation and lifecycle operations | PRIMARY | `49b2c188` | Consolidates content-type and entry routes. |
| `backend/src/routes/pages.rs` | HTTP handlers | Pages, versions, page-builder registry and template behavior | PRIMARY | `49b2c188` | Includes component-registry routes. |
| `backend/src/routes/media.rs` | HTTP handlers and storage logic | Media upload, validation, local filesystem paths, variants and deletion | PRIMARY | `49b2c188` | Confirms local storage; object storage/CDN is not implemented. |
| `backend/src/routes/delivery.rs` | Public HTTP handlers | Public Delivery API behavior and current organization selection | PRIMARY | `49b2c188` | Hard-coded `default` organization behavior is U-08 and NOC-01. |
| `backend/src/routes/organizations.rs` | HTTP handlers | Organization and membership operations | PRIMARY | `49b2c188` | Supports tenant and workspace terminology. |
| `backend/src/routes/billing.rs` | HTTP handlers | Billing, plan, quota and provider-facing operations | PRIMARY | `49b2c188` | Provider and production readiness remain subject to owner confirmation. |
| `backend/src/routes/marketplace.rs` | HTTP handlers | Marketplace catalog, creator, submission and installation operations | PRIMARY | `49b2c188` | Use the specialized Marketplace route modules for later phases as well. |
| `backend/src/routes/marketplace_runtime.rs` | HTTP handlers | Permission catalog, runtime authorization and kill-switch operations | PRIMARY | `49b2c188` | Host API authorization is not arbitrary uploaded-code execution; PNI-01. |
| `backend/src/routes/marketplace_finance.rs` | HTTP handlers | Ledger, refund and finance-operation records | PRIMARY | `49b2c188` | Automated payout settlement and full disputes remain PNI-02 and PNI-03. |
| `backend/src/routes/marketplace_analytics.rs` | HTTP handlers | Marketplace analytics ingestion and reporting | PRIMARY | `49b2c188` | Runtime telemetry pipeline remains PNI-05. |
| `backend/src/middleware/auth.rs` | Request middleware | Bearer token validation and authenticated claims | PRIMARY | `49b2c188` | Authentication precedes tenant selection. |
| `backend/src/middleware/tenant.rs` | Request middleware | Organization selection, membership verification and TenantContext | PRIMARY | `49b2c188` | Central tenant-boundary evidence. |
| `backend/src/middleware/security.rs` | Request middleware | Security-related request and response controls | PRIMARY | `49b2c188` | Check configuration before assuming production posture. |
| `backend/src/services/rbac.rs` | Authorization service | Global and organization role checks | PRIMARY | `49b2c188` | Distinguish role scopes from Marketplace permissions. |
| `backend/src/services/rls.rs` | Tenant database service | RLS transaction helpers and organization context | PRIMARY | `49b2c188` | Pair with migration `0009`. |
| `backend/src/services/marketplace_installation.rs` | Domain service | Installation and entitlement transition rules | PRIMARY | `49b2c188` | Specialized Marketplace rule evidence. |
| `backend/src/services/marketplace_package.rs` | Domain service | Artifact/package validation and path rules | PRIMARY | `49b2c188` | Retention and backup ownership remain U-11 and NOC-17. |
| `backend/src/services/marketplace_runtime.rs` | Domain service | Allowlisted host capabilities and runtime policy checks | PRIMARY | `49b2c188` | Does not execute arbitrary uploaded server-side code. |
| `backend/src/plugins/mod.rs` | Built-in plugin abstraction | `CmsPlugin`, hooks, built-in registry and execution | PRIMARY | `49b2c188` | Separate trust boundary from Marketplace artifacts. |
| `backend/src/plugins/seo.rs` | Built-in plugin | Concrete SEO automation plugin | PRIMARY | `49b2c188` | Confirms at least one built-in plugin implementation. |

## Frontend Sources

| Path | Source type | Information derived | Reliability | Verified commit | Notes and conflicts |
|---|---|---|---|---|---|
| `frontend/package.json` | Frontend manifest | One package boundary, runtime/testing dependencies, and scripts | PRIMARY | `7d25e4cb` | React Hook Form, resolvers, and Zod are declared but have no verified source import; DCC-11/DU-02. |
| `frontend/src/main.tsx` | Frontend entry | React root, providers, router startup and styles | PRIMARY | `7d25e4cb` | Canonical UI entry point. |
| `frontend/src/router.tsx` | Route configuration | Public login, protected parent, eager feature routes, and wildcard redirect | PRIMARY | `7d25e4cb` | No lazy route, loader, action, or route error element was found. |
| `frontend/src/components/AppShell.tsx` | Layout component | Navigation, organization/locale controls, health, identity, logout, and outlet | PRIMARY | `7d25e4cb` | Static menu is not filtered by feature permission. |
| `frontend/src/components/RequireAuth.tsx` | Route guard | Token-presence route admission | PRIMARY | `7d25e4cb` | Not a substitute for backend authorization; ABV-01. |
| `frontend/src/components/DynamicForm.tsx` | Shared form component | Field-schema-driven entry controls and value conversion | PRIMARY | `7d25e4cb` | Uses controlled inputs/native required, not React Hook Form or Zod. |
| `frontend/src/hooks/useHealth.ts` | Shared hook | Health/readiness polling and local async state | PRIMARY | `7d25e4cb` | No request abort; Shell and Dashboard can both instantiate it. |
| `frontend/src/stores/useAppStore.ts` | Zustand store | Shared authentication, organization and shell state | PRIMARY | `7d25e4cb` | Coordinates with API module variables and browser storage under SOU-01. |
| `frontend/src/services/api.ts` | API client | HTTP methods, headers, endpoint calls, uploads, and error handling | PRIMARY | `7d25e4cb` | No automatic refresh/retry, cancellation, or runtime schema validation. |
| `frontend/src/types/api.ts` | Type declarations | Frontend view of backend request and response shapes | PRIMARY | `7d25e4cb` | Manual duplicated contract under ACU-01/DC-01/DDU-03. |
| `frontend/src/i18n` | i18n module directory | Locale selection, messages, fallback, direction, and provider behavior | PRIMARY | `7d25e4cb` | Current source is broader than `docs/I18N.md` coverage text; DCC-02. |
| `frontend/src/pages` | Page component directory | Implemented UI surfaces and dominant feature composition boundaries | PRIMARY | `7d25e4cb` | Marketplace, Pages, and Organization have broad responsibility. |
| `frontend/src/pages/PagesPage.tsx` | Route page and editor | Visual Page Builder, local preview, persistence, versions, workflow, template and preview handoff | PRIMARY | `7d25e4cb` | Confirms implemented builder; local and WebSocket previews are distinct. |
| `frontend/src/styles/index.css` | Global stylesheet | Semantic UI classes, responsive breakpoints, RTL behavior, and Tailwind import | PRIMARY | `7d25e4cb` | No formal token/design-system boundary; COU-01. |
| `frontend/vite.config.ts` | Build configuration | React/Tailwind plugins and development server | PRIMARY | `7d25e4cb` | No proxy, alias, explicit route split, or bundle budget. |
| `frontend/vitest.config.ts` and `frontend/src/test/setup.ts` | Test configuration | jsdom and jest-dom test environment | PRIMARY | `7d25e4cb` | Test cases are page-level. |
| `frontend/src/pages/*.test.tsx` | Frontend tests | Dashboard, Pages shell, and selected Marketplace behavior | PRIMARY | `7d25e4cb` | Three files and 14 observed cases; no coverage artifact. |
| `frontend/Dockerfile`, `frontend/Dockerfile.prod`, `frontend/nginx.conf` | Packaging and static-host configuration | Development Vite image, production-like Nginx image, SPA fallback | PRIMARY | `7d25e4cb` | Packaging capability is not deployment proof. |
| `.github/workflows/frontend-ci.yml` | CI workflow | Node 22 install, lint, typecheck, test, and build gates | PRIMARY | `7d25e4cb` | Docker images use Node 24; supported matrix remains unknown. |

## Database Sources

| Path | Source type | Information derived | Reliability | Verified commit | Notes and conflicts |
|---|---|---|---|---|---|
| `backend/migrations` | Ordered SQL migrations | Complete tracked schema evolution and seed behavior | PRIMARY | `49b2c188` | Schema authority over narrative models and simplified diagrams. |
| `backend/migrations/0001_initial_schema.sql` | SQL migration | Initial users, roles and core schema foundation | PRIMARY | `49b2c188` | Later migrations extend or alter the initial model. |
| `backend/migrations/0003_phase_one_core.sql` | SQL migration | Content types, entries and core CMS data | PRIMARY | `49b2c188` | Use with `backend/src/routes/content.rs`. |
| `backend/migrations/0004_phase_two_page_builder.sql` | SQL migration | Pages, versions and component registry | PRIMARY | `49b2c188` | Use with `backend/src/routes/pages.rs`. |
| `backend/migrations/0008_v2_phase_one_organizations.sql` | SQL migration | Organization tenancy and membership | PRIMARY | `49b2c188` | Establishes the primary tenant entity. |
| `backend/migrations/0009_v2_phase_three_rls.sql` | SQL migration | PostgreSQL RLS policies and tenant isolation | PRIMARY | `49b2c188` | Use with RLS service code. |
| `backend/migrations/0010_v2_phase_five_billing_quota.sql` through `0014_v2_phase_nine_beta_release.sql` | SQL migration series | Billing, Stripe records, SaaS operations, hardening and beta-era schema | PRIMARY | `49b2c188` | Phase names do not prove actual release state; ISU-03. |
| `backend/migrations/0015_v3_phase_one_marketplace_foundation.sql` through `0026_v3_phase_thirteen_marketplace_qa_performance.sql` | SQL migration series | Marketplace foundation, review, installation, runtime, finance, feedback and QA schema | PRIMARY | `49b2c188` | Later planned outcomes still require code and operational evidence. |

## API Sources

| Path | Source type | Information derived | Reliability | Verified commit | Notes and conflicts |
|---|---|---|---|---|---|
| `backend/src/routes/mod.rs` | Router registration | Actual route families and middleware boundaries | PRIMARY | `49b2c188` | Strongest single navigation source for HTTP surfaces. |
| `backend/src/routes` | Handler directory | Endpoint paths, methods, validation, ownership and response shapes | PRIMARY | `49b2c188` | Search exact route declarations rather than relying on prose counts. |
| `backend/src/error.rs` | HTTP error mapping | Status and response behavior for application errors | PRIMARY | `49b2c188` | Overrides stale narrative examples. |
| `frontend/src/services/api.ts` | Client integration | Endpoints actually consumed by the UI | PRIMARY | `49b2c188` | Does not enumerate backend-only endpoints. |
| `docs/API.md` | Narrative API guide | Selected endpoint examples and intended usage | OUTDATED_OR_CONFLICTING | `49b2c188` | Incomplete and partly stale compared with router code; DCC-09. |
| `docs/diagrams/08-route-boundaries.mmd` | Architecture diagram | High-level route grouping and trust boundaries | SUPPORTING | `49b2c188` | Verify exact paths and middleware in code. |

## Test Sources

| Path | Source type | Information derived | Reliability | Verified commit | Notes and conflicts |
|---|---|---|---|---|---|
| `backend/src` | Colocated Rust tests | Unit and integration-style module tests embedded beside source | PRIMARY | `49b2c188` | No current `backend/tests` directory. |
| `frontend/src/pages/DashboardPage.test.tsx` | Vitest test file | Dashboard UI behavior | PRIMARY | `49b2c188` | One of three tracked page-level test files. |
| `frontend/src/pages/MarketplacePage.test.tsx` | Vitest test file | Marketplace UI behavior | PRIMARY | `49b2c188` | Covers selected behaviors, not complete Marketplace workflows. |
| `frontend/src/pages/PagesPage.test.tsx` | Vitest test file | Page-builder UI behavior | PRIMARY | `49b2c188` | Covers selected page interactions. |
| `frontend/src/test/setup.ts` | Test setup | Shared Testing Library/Vitest environment | PRIMARY | `49b2c188` | Loaded through Vitest configuration. |
| `.github/workflows/backend-ci.yml` | CI workflow | Backend format, lint and test gates | PRIMARY | `49b2c188` | CI evidence does not prove a particular local run passed. |
| `.github/workflows/frontend-ci.yml` | CI workflow | Frontend install, lint, typecheck, test and build gates | PRIMARY | `49b2c188` | No deployment job was found. |

## Infrastructure and Operations Sources

| Path | Source type | Information derived | Reliability | Verified commit | Notes and conflicts |
|---|---|---|---|---|---|
| `docker-compose.yml` | Development Compose | PostgreSQL, Redis and pgAdmin local infrastructure | PRIMARY | `49b2c188` | Does not currently define both application services. |
| `docker-compose.prod.yml` | Production-like Compose | Container topology, reverse proxy and environment wiring | PRIMARY | `49b2c188` | Configuration is not proof of a production deployment; ISU-03. |
| `backend/Dockerfile` | Development container build | Backend development image behavior | PRIMARY | `49b2c188` | Pair with Compose or explicit run commands. |
| `backend/Dockerfile.prod` | Production-like container build | Backend release build and runtime image | PRIMARY | `49b2c188` | Does not establish hosting platform or release procedure. |
| `frontend/Dockerfile` | Development container build | Frontend development image behavior | PRIMARY | `49b2c188` | Pair with Vite configuration. |
| `frontend/Dockerfile.prod` | Production-like container build | Frontend build and Nginx image | PRIMARY | `49b2c188` | Does not establish actual deployment status. |
| `frontend/nginx.conf` | Nginx configuration | SPA routing, proxying and static delivery behavior | PRIMARY | `49b2c188` | Review together with production-like Compose. |
| `.env.example` | Environment template | Required local variables and example values | PRIMARY | `49b2c188` | Never copy real secrets into OKF documents. |
| `env.example` | Environment template | Alternate tracked environment example | PRIMARY | `49b2c188` | Duplication/ownership is U-09 and NOC-14. |
| `.gitignore` | Ignore policy | Generated, local, secret-bearing and runtime paths | PRIMARY | `49b2c188` | Supports generated-content notes in the Repository Map. |
| `backend/.dockerignore` | Container ignore policy | Backend build-context exclusions | PRIMARY | `49b2c188` | Build-context scope only. |
| `frontend/.dockerignore` | Container ignore policy | Frontend build-context exclusions | PRIMARY | `49b2c188` | Build-context scope only. |
| `scripts/marketplace-cli.mjs` | CLI script | Marketplace package build, validation and artifact output | PRIMARY | `49b2c188` | Output retention is unresolved under U-11 and NOC-17. |
| `scripts/marketplace-phase13-load-smoke.ps1` | Operational script | Marketplace performance smoke workflow | PRIMARY | `49b2c188` | Script presence does not prove a recent run. |
| `scripts/marketplace-phase14-beta-readiness.ps1` | Operational script | Marketplace beta readiness checks | PRIMARY | `49b2c188` | Readiness tooling is not proof of beta completion; ISU-03. |
| `scripts/marketplace-phase15-ga-check.ps1` | Operational script | Marketplace GA checks | PRIMARY | `49b2c188` | GA tooling is not proof of launch; ISU-03. |

## Existing Narrative Documentation

| Path | Source type | Information derived | Reliability | Verified commit | Notes and conflicts |
|---|---|---|---|---|---|
| `docs/ARCHITECTURE.md` | Architecture narrative | Intended components, request flow and deployment view | SUPPORTING | `49b2c188` | Cross-check with current route and infrastructure code. |
| `docs/I18N.md` | Frontend narrative | Intended localization architecture | OUTDATED_OR_CONFLICTING | `49b2c188` | Current module details differ; DCC-02. |
| `docs/V3_MARKETPLACE_SCOPE.md` | Product scope | Official Marketplace scope and explicit non-goals | SUPPORTING | `49b2c188` | Important evidence for PNI-01 and roadmap boundaries. |
| `docs/V3_PRODUCT_TAXONOMY.md` | Product taxonomy | Marketplace product-type vocabulary | SUPPORTING | `49b2c188` | Cross-check accepted values in migrations and services. |
| `docs/V3_MARKETPLACE_DOMAIN_MODEL.md` | Domain narrative | Marketplace entities and relationships | OUTDATED_OR_CONFLICTING | `49b2c188` | Finance and feedback entities described as future are implemented; DCC-03. |
| `docs/V3_MARKETPLACE_MANIFEST_SCHEMA.md` | Schema narrative | Marketplace manifest intent and examples | SUPPORTING | `49b2c188` | Validator code and current schema rules take precedence. |
| `docs/V3_PACKAGE_STORAGE.md` | Storage narrative | Artifact storage and lifecycle intent | SUPPORTING | `49b2c188` | Current local storage and ownership gaps remain U-05, U-11, and NOC-17. |
| `docs/V3_MARKETPLACE_POLICY.md` | Policy narrative | Review, safety and Marketplace governance intent | SUPPORTING | `49b2c188` | Operational enforcement must be confirmed in code. |
| `docs/V3_MARKETPLACE_GAP_LIST.md` | Gap list | Historical Marketplace implementation gaps | OUTDATED_OR_CONFLICTING | `49b2c188` | Some items have since been implemented; current code and Phase Zero findings take precedence. |
| `docs/MARKETPLACE_CREATOR_GUIDE.md` | User guide | Creator workflow and CLI usage | SUPPORTING | `49b2c188` | Verify commands against `scripts/marketplace-cli.mjs`. |
| `docs/V3_MARKETPLACE_OPERATIONS_RUNBOOK.md` | Runbook | Intended Marketplace operations and incident procedures | SUPPORTING | `49b2c188` | External notification and ownership gaps remain open. |
| `docs/V2_OPERATIONS_RUNBOOK.md` | Runbook | SaaS operations and recovery intent | SUPPORTING | `49b2c188` | Automated alert delivery remains ISU-02. |
| `docs/PHASE_ONE.md` through `docs/PHASE_SEVEN.md` | Historical phase documents | Core CMS implementation narrative | SUPPORTING | `49b2c188` | `docs/PHASE_THREE.md` can imply the visual builder is future despite current implementation; DCC-01. |
| `docs/V2_PHASE_ZERO.md` through `docs/V2_PHASE_TEN.md` | Historical phase documents | SaaS evolution narrative | SUPPORTING | `49b2c188` | Do not infer release state from filenames. |
| `docs/V3_PHASE_0_1.md` through `docs/V3_PHASE_FIFTEEN.md` | Historical phase documents | Marketplace evolution narrative | SUPPORTING | `49b2c188` | Do not infer deployment, beta, or GA state from filenames. |

## Mermaid and Diagram Sources

| Path | Source type | Information derived | Reliability | Verified commit | Notes and conflicts |
|---|---|---|---|---|---|
| `docs/diagrams/README.md` | Diagram catalog | Diagram purpose, grouping and navigation | SUPPORTING | `49b2c188` | Catalog does not make every diagram current. |
| `docs/diagrams/TRACEABILITY.md` | Diagram traceability | Claimed source relationships for diagrams | SUPPORTING | `49b2c188` | Confirm evidence paths before reuse. |
| `docs/diagrams/DIAGRAM_CONVENTIONS.md` | Diagram standard | Mermaid conventions and status notation | SUPPORTING | `49b2c188` | Useful for future OKF diagram work. |
| `docs/diagrams/ARCHITECTURE_AUDIT.md` | Diagram-set audit | Historical diagram coverage and implementation-status assessment | OUTDATED_OR_CONFLICTING | `49b2c188` | It states that Marketplace purchases, entitlements, and payouts are absent despite current finance implementation; DCC-04. |
| `docs/diagrams/00-implementation-status-map.mmd` | Mermaid diagram | High-level implemented/planned status | OUTDATED_OR_CONFLICTING | `49b2c188` | Its OpenAPI/API status label needs narrowing; DCC-05. |
| `docs/diagrams/02-system-context.mmd` | Mermaid diagram | Actors, external systems and system boundary | OUTDATED_OR_CONFLICTING | `49b2c188` | Paid purchase is no longer deferred, and public-site routing remains unresolved; DCC-06 and NOC-01. |
| `docs/diagrams/03-identity-and-authorization-boundaries.mmd` | Mermaid diagram | Identity, role and tenant boundaries | SUPPORTING | `49b2c188` | Verify exact role checks in `backend/src/services/rbac.rs`. |
| `docs/diagrams/04-container-architecture.mmd` | Mermaid diagram | Application and infrastructure containers | SUPPORTING | `49b2c188` | Production-like topology is not deployment proof. |
| `docs/diagrams/13-identity-auth-data-model.mmd` through `docs/diagrams/22-security-trust-boundaries.mmd` | Mermaid diagram series | Data entities, tenancy and trust boundaries | SUPPORTING | `49b2c188` | Migrations and code remain authoritative. |
| `docs/diagrams/20-marketplace-package-review-pipeline.mmd` | Mermaid diagram | Package validation and review flow | OUTDATED_OR_CONFLICTING | `49b2c188` | The `purchase_runtime` status text and visual class disagree; DCC-10. |
| `docs/diagrams/30-marketplace-sequences.mmd` | Mermaid diagram | Marketplace request sequences | OUTDATED_OR_CONFLICTING | `49b2c188` | It labels implemented install, purchase, payout, and rating flows as unimplemented; DCC-07. |
| `docs/diagrams/31-observability-and-failure-recovery.mmd` | Mermaid diagram | Observability and recovery intent | SUPPORTING | `49b2c188` | Metrics exporter/APM and automatic alert delivery are not proven. |
| `docs/diagrams/33-marketplace-installation-lifecycle.mmd` | Mermaid diagram | Installation and entitlement lifecycle | OUTDATED_OR_CONFLICTING | `49b2c188` | It treats paid entitlement as future/rejected despite current entitlement gating; DCC-08. |
| `docs/diagrams/34-marketplace-security-runtime.mmd` through `docs/diagrams/42-marketplace-launch-ga.mmd` | Mermaid diagram series | Marketplace runtime, finance, feedback, analytics, QA, beta and launch intent | SUPPORTING | `49b2c188` | Several outcomes remain PNI-01 through PNI-05 or ISU-03. |

## Phase Zero Evidence Reviews

Phase Zero documents are evidence-backed inventories created at the verified commit. They are secondary sources: current code, migrations, manifests, configuration, and Git state still take precedence.

| Path | Source type | Information derived | Reliability | Verified commit | Notes and conflicts |
|---|---|---|---|---|---|
| `okf-bootstrap/00-repository-inventory.md` | Phase Zero inventory | Repository structure, tracked-file distribution and generated paths | SUPPORTING | `49b2c188` | Baseline for the Repository Map. |
| `okf-bootstrap/01-technology-inventory.md` | Phase Zero inventory | Languages, frameworks, libraries and tooling | SUPPORTING | `49b2c188` | Baseline for the technology summary. |
| `okf-bootstrap/02-architecture-observations.md` | Phase Zero analysis | Components, request flows, trust boundaries and operational observations | SUPPORTING | `49b2c188` | Clearly separates observations from unknowns. |
| `okf-bootstrap/03-module-inventory.md` | Phase Zero inventory | Backend and frontend module responsibilities | SUPPORTING | `49b2c188` | Use exact current paths when navigating. |
| `okf-bootstrap/04-documentation-audit.md` | Phase Zero audit | Documentation coverage, staleness and DCC-01 through DCC-10 | SUPPORTING | `49b2c188` | Primary conflict register for Phase 1. |
| `okf-bootstrap/05-mermaid-audit.md` | Phase Zero audit | Diagram catalog, evidence quality and drift | SUPPORTING | `49b2c188` | Use with diagram traceability. |
| `okf-bootstrap/06-database-inventory.md` | Phase Zero inventory | Migration sequence, tables and schema observations | SUPPORTING | `49b2c188` | Migrations remain schema authority. |
| `okf-bootstrap/07-api-inventory.md` | Phase Zero inventory | Route families and API observations | SUPPORTING | `49b2c188` | Route code remains endpoint authority. |
| `okf-bootstrap/08-conventions-inventory.md` | Phase Zero inventory | Naming, errors, tests, configuration and repository conventions | SUPPORTING | `49b2c188` | Useful for future authoring standards. |
| `okf-bootstrap/09-knowledge-gaps.md` | Phase Zero gap register | U-01 through U-15 and related owner dependencies | SUPPORTING | `49b2c188` | Do not close markers without new evidence. |
| `okf-bootstrap/10-proposed-okf-structure.md` | Phase Zero proposal | Proposed long-term OKF information architecture | SUPPORTING | `49b2c188` | Planned paths are not current files. |
| `okf-bootstrap/11-implementation-phases.md` | Phase Zero plan | Proposed phased OKF implementation | SUPPORTING | `49b2c188` | This Phase 1 work must not imply later phases are complete. |
| `okf-bootstrap/12-owner-questions.md` | Phase Zero question register | NOC-01 through NOC-18 | SUPPORTING | `49b2c188` | Requires owner answers or primary evidence. |
| `okf-bootstrap/phase-zero-summary.md` | Phase Zero summary | Consolidated findings, validations and next recommendation | SUPPORTING | `49b2c188` | Entry point to the full Phase Zero evidence set. |

## Phase 2 Architecture-Specific Verification

The following sources were inspected at commit `17e69e266c558c8568ec65524560d52d7cb89d4c` to establish process boundaries, dependency direction, runtime flows, integration semantics, risks, and inferred decisions. Rows supplement the broader Phase 1 inventory; they do not change the source-of-truth priority.

| Path | Source type | Architecture information derived | Reliability | Verified commit | Notes and conflicts |
|---|---|---|---|---|---|
| `backend/src/services/health.rs` | Service implementation | Reverse dependency on route-owned `DependencyCheck` | PRIMARY | `17e69e26` | Registered as DDU-02. |
| `backend/src/services/cache.rs` | Redis adapter | Cache fallback, best-effort invalidation, and prefix scan behavior | PRIMARY | `17e69e26` | Redis criticality differs from rate limiting and readiness. |
| `backend/src/services/email.rs` | Integration adapter | Log, disabled, and webhook email modes and strict failure behavior | PRIMARY | `17e69e26` | Production provider and retries remain unknown. |
| `backend/src/services/stripe_billing.rs` | Integration adapter | Stripe API endpoint, bearer authentication, and provider error mapping | PRIMARY | `17e69e26` | Pair with billing and Marketplace finance routes. |
| `backend/src/routes/webhooks.rs` and `backend/src/services/webhooks.rs` | Route and delivery implementation | Webhook configuration plus signed outbound task, timeout, and one recorded attempt | PRIMARY | `17e69e26` | No durable queue or automatic retry worker was found. |
| `backend/src/services/media_processing.rs` | Processing service | Blocking image work and WebP variant generation | PRIMARY | `17e69e26` | Pair with the already registered media route sources. |
| `backend/src/services/marketplace_adapters.rs` | Marketplace host adapter | Allowlisted host capabilities and adapter policy boundary | PRIMARY | `17e69e26` | In-process policy adapter, not a separate runtime service. |
| `frontend/src/pages/WorkspaceRedirectPage.tsx` | Client navigation page | Organization-slug resolution and workspace redirect behavior | PRIMARY | `17e69e26` | Client navigation is not a backend authorization control. |
| `docs/diagrams/09-request-middleware-pipeline.mmd` | Historical diagram | Prior request and middleware flow representation | SUPPORTING | `17e69e26` | Phase 2 rebuilt the verified request flow from current source. |

## Phase 3 Backend-Specific Verification

The following sources were inspected at commit `debde2021c029d1827abaa38bcc32c682f53f55a` for the Phase 3 module catalog, boundaries, dependency flow, request handling, state, errors, tests, and risks. They supplement the earlier register without replacing source-code authority.

| Path | Backend information derived | Reliability | Verified commit | Notes |
|---|---|---|---|---|
| `backend/Cargo.toml` | One-package boundary, framework/runtime/persistence dependencies, and test dependency | PRIMARY | `debde202` | Supports the modular-monolith and test-framework classification. |
| `backend/src/routes` | Handler ownership, router entry points, DTOs, direct SQL, orchestration, errors, and colocated route tests | PRIMARY | `debde202` | The module catalog consolidates files by significant responsibility rather than treating each file as an independent service. |
| `backend/src/services` | Reusable policy, validation, integrations, Marketplace domain behavior, persistence calls, and service tests | PRIMARY | `debde202` | Service boundaries are heterogeneous; `services/health.rs` active use remains unconfirmed. |
| `backend/src/models` | Partial shared record/model coverage | PRIMARY | `debde202` | Many DTOs and mappings remain route- or service-local. |
| `backend/src/middleware` | Authentication, tenant context, and security cross-cutting contracts | PRIMARY | `debde202` | Claims and tenant context have broad consumers. |
| `backend/src/plugins` | Built-in plugin contract, registry, SEO implementation, and tests | PRIMARY | `debde202` | Separate from Marketplace package runtime policy. |
| `backend/src/error.rs` | Complete shared `AppError` status/category mapping and SQLx conversion | PRIMARY | `debde202` | Some stored technical messages are user-visible; do not assume sanitization. |
| `backend/src/config.rs` | Exact environment contract, defaults, validation, and configuration tests | PRIMARY | `debde202` | Secret names may be documented; secret values must not be copied. |
| `backend/src/state.rs` | Exact application-state composition and preview-channel lifecycle | PRIMARY | `debde202` | Preview channels are process-local and not stored in Redis. |
| `.github/workflows/backend-ci.yml` | Explicit format, lint, and test commands plus CI PostgreSQL/Redis services | PRIMARY | `debde202` | No coverage report or separate end-to-end suite is defined here. |
| `backend/migrations` | Primary persistence, enum, constraint, index, trigger, seed, and RLS evidence | PRIMARY | `70b97242` | Detailed schema navigation is current under `okf/database`. |

## Phase 4 Frontend-Specific Verification

The following evidence was inspected at commit `7d25e4cbc53284a78033478e2681d8e9ebeb2fb1` for the Phase 4 application and feature catalogs, routing, layout/components, state, API integration, access cues, forms, styling, failures, Page Builder, build, tests, and risks:

| Evidence group | Frontend information derived | Reliability | Notes |
|---|---|---|---|
| Manifest, lockfile, Vite, TypeScript, ESLint, Vitest | Package boundary, dependency/tool versions, scripts, strict build, lint/test behavior | PRIMARY | Declared dependencies do not establish source usage. |
| `main.tsx`, `router.tsx`, shared components and hook | Root/provider order, eager routes, protected shell, navigation, shared health/form/status behavior | PRIMARY | Client admission and role cues are not backend authorization. |
| Store, API client, and API types | State persistence, header context, request/error behavior, manual contracts | PRIMARY | SOU-01, ACU-01, DC-01, and DDU-03 apply. |
| All route pages | Thirteen significant feature responsibilities, UI composition, local state, access cues, and failure handling | PRIMARY | Catalog groups by responsibility rather than creating one feature per file. |
| Three page test files | Selected jsdom behavior and major coverage gaps | PRIMARY | Coverage percentage remains unknown. |
| Global CSS, i18n, and font assets | Semantic styling, responsive intent, locale/fallback, and LTR/RTL behavior | PRIMARY | Runtime browser/accessibility quality remains UBU-01. |
| Dockerfiles, Nginx, frontend CI | Build/package capability, SPA fallback, and validation gates | PRIMARY | Actual deployment state remains ISU-03. |
| `docs/I18N.md` and Phase Zero frontend inventories | Earlier intended/current narratives | SUPPORTING_OR_CONFLICTING | DCC-02 and DCC-11 record narrowed conflicts. |

## Phase 5 Database-Specific Verification

Phase 5 inspected all 26 SQL files under `backend/migrations`, database initialization and startup, SQLx queries and transactions in routes/services, shared and local persistence mappings, Compose/CI PostgreSQL configuration, migration seeds, the startup bootstrap, the tenant fixture, and database-related tests at commit `70b972428799304c7defd7e67f95459cd4a3644e`.

| Source | Phase 5 evidence | Reliability | Important limit |
| --- | --- | --- | --- |
| `backend/migrations/0001` through `0026` | 51 tables, 7 enums, extensions, functions, triggers, constraints, indexes, RLS, seeds | Primary | Intended history does not prove deployed state |
| `backend/src/db/mod.rs`; `backend/src/main.rs` | Pool creation, embedded migration runner, startup seed | Primary | Deployment-only URL/pool behavior is unknown |
| `backend/src/routes`; `backend/src/services` | 290 direct SQLx call occurrences, transactions, side effects, tenant/bypass usage | Primary | Source counts are not runtime volume |
| `backend/src/models` | Partial shared row and enum mapping | Primary | Conflicts `MMC-01` and `MMC-02` are retained |
| Compose and CI | PostgreSQL 16 repository environment | Primary configuration | Does not establish production version |
| `docs/V2_PHASE_EIGHT_FIXTURE.sql` | Manual local/staging tenant-isolation fixture | Supporting executable fixture | Not an automated reset harness |
| `okf/database` | Phase 5 navigation and synthesis | Secondary | Not executable schema authority |

`DCC-12` records that the current hardening service enumerates 24 RLS tables while migration history intends 32. Runtime schema remains `SCHEMA_RUNTIME_STATUS_UNKNOWN SRU-01`.

## Conflict Handling

The current OKF documents preserve `DCC-01` through `DCC-10` from the Phase Zero documentation audit, add `DCC-11` for the unsupported React Hook Form/Zod source-usage claim, and add `DCC-12` for database RLS-verification drift. When a registered narrative or diagram conflicts with implementation evidence:

1. Prefer current executable code, migrations, manifests, configuration, workflows, and Git state.
2. Keep the conflicting source registered for historical context.
3. Carry the applicable DCC marker into derived documentation.
4. Do not silently rewrite the conflict as a verified fact.
5. Require owner confirmation where product intent cannot be derived from implementation evidence.

## Related Documents

- [OKF Entry Point](../README.md)
- [Machine-Readable Index](../index.yaml)
- [Project Overview](../project/overview.md)
- [Repository Map](../project/repository-map.md)
- [Project Glossary](../project/glossary.md)
- [Navigation Guide](../project/navigation-guide.md)
- [System Architecture](../architecture/README.md)
- [Architecture Overview](../architecture/overview.md)
- [System Boundaries](../architecture/boundaries.md)
- [Components and Responsibilities](../architecture/components.md)
- [Dependency Model](../architecture/dependency-model.md)
- [Runtime Flows](../architecture/runtime-flows.md)
- [Integration Points](../architecture/integration-points.md)
- [Architecture Risks](../architecture/architecture-risks.md)
- [Architecture Decisions](../architecture/decisions/README.md)
- [Architecture Decision Register](../architecture/decisions/decision-register.md)
- [Backend Documentation](../backend/README.md)
- [Backend Module Catalog](../backend/module-catalog.md)
- [Backend Testing Map](../backend/testing-map.md)
- [Frontend Architecture](../frontend/README.md)
- [Frontend Application Catalog](../frontend/application-catalog.md)
- [Frontend Feature Catalog](../frontend/feature-catalog.md)
- [Frontend Testing Map](../frontend/testing-map.md)
- [Frontend Risks](../frontend/frontend-risks.md)
- [Database Architecture](../database/README.md)
- [Database Schema Catalog](../database/schema-catalog.md)
- [Database Risk Register](../database/database-risks.md)

## Phase 7 Security Sources

| Source area | Authority for Phase 7 |
| --- | --- |
| `backend/src/routes/auth.rs` | Authentication endpoint behavior, token issue/rotation/logout, cookie attributes, bootstrap role mapping |
| `backend/src/middleware/auth.rs`, `tenant.rs`, `security.rs` | Bearer, tenant, and response-header boundaries |
| `backend/src/services/jwt.rs`, `password.rs`, `security.rs` | Token cryptography, password hashing, login limiting, rich-text sanitization |
| `backend/src/services/rbac.rs`, `rls.rs`, `audit.rs` | Role decisions, tenant SQL context/bypass, and audit persistence |
| `backend/migrations/0001`, `0003`, `0007`-`0009`, `0012`, `0020` | Identity, roles, login attempts, organizations, RLS, audit, Marketplace permissions/kill switches |
| `frontend/src/services/api.ts`, `stores/useAppStore.ts`, access/Marketplace components | Browser token storage, request construction, tenant selection, and frontend-only role cues |

The [Security README](../security/README.md) organizes these sources without superseding them. Environment templates were reviewed by variable name; no secret value is reproduced.

## Phase 8 Business Rule and Workflow Evidence

| Source group | Business behavior established |
| --- | --- |
| `backend/src/services/workflow.rs`, content/page routes | Editorial transition matrix, bypass permission, version changes, publication side effects, and restore behavior |
| `backend/src/routes/organizations.rs`, `backend/src/services/organization.rs` | Provisioning, invitations, active membership, last-owner checks, and ownership transfer |
| `backend/src/routes/media.rs`, media services | Upload validation, quota checks, file/database ordering, image variants, and deletion behavior |
| `backend/src/routes/webhooks.rs`, `backend/src/services/webhook.rs` | Event selection, URL/secret validation, HMAC delivery, timeout, and delivery records |
| billing, Stripe, quota, SaaS, and beta routes/services | Subscription state, provider idempotency, quota accounting, operational settings, and beta evidence |
| plugin and Marketplace routes/services plus migrations `0020`-`0025` | Built-in plugin behavior, artifact immutability, review gates, installation permissions, purchase entitlements, and finance limits |
| `backend/migrations`, frontend pages/components/services/types | Persistence invariants, UI-supported actions, validation duplication, and test evidence |

The [Domain Catalog](../domain/domain-catalog.md) and [Business Rule Catalog](../domain/business-rule-catalog.md) organize these sources. They do not supersede code, migrations, or tests.

## Phase 9 Evidence Set

Phase 9 prioritizes backend/src/plugins, plugin and Marketplace route modules, Marketplace service modules, migrations 0006 and 0015 through 0026, MarketplacePage and PagesPage, their tests, marketplace-cli, and current V3 manifest/package/creator documentation. The [Extensibility Catalog](../extensibility/extensibility-catalog.md) maps each verified mechanism to direct evidence.
