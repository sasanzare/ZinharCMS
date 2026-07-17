---
okf_document_id: "project-overview"
title: "Project Overview"
project: "ZinharCMS"
category: "project"
phase: 1
status: "current"
review_status: "verified"
source_of_truth: false
last_verified_commit: "49b2c1886168497e99f7086e4941b21616985656"
last_verified_date: "2026-07-17"
primary_sources:
  - "README.md"
  - "backend/Cargo.toml"
  - "frontend/package.json"
  - "backend/src/main.rs"
  - "backend/src/routes/mod.rs"
  - "backend/migrations"
  - "docs/ARCHITECTURE.md"
  - "docs/V3_MARKETPLACE_SCOPE.md"
  - "okf-bootstrap/phase-zero-summary.md"
related_documents:
  - "README.md"
  - "index.yaml"
  - "project/repository-map.md"
  - "project/glossary.md"
  - "project/navigation-guide.md"
  - "references/source-register.md"
uncertainty_markers:
  - "UNKNOWN U-01"
  - "UNKNOWN U-03"
  - "UNKNOWN U-04"
  - "UNKNOWN U-05"
  - "UNKNOWN U-06"
  - "UNKNOWN U-08"
  - "UNKNOWN U-09"
  - "UNKNOWN U-10"
  - "UNKNOWN U-12"
  - "UNKNOWN U-14"
  - "UNKNOWN U-15"
  - "NEEDS_OWNER_CONFIRMATION NOC-10"
  - "NEEDS_OWNER_CONFIRMATION NOC-15"
  - "NEEDS_OWNER_CONFIRMATION NOC-16"
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
---

# Project Overview

## 1. Project Identity

| Field | Verified description |
| --- | --- |
| Project name | ZinharCMS |
| Repository name | ZinharCMS |
| Repository URL | https://github.com/sasanzare/ZinharCMS |
| Project type | API-first headless content management system with a separate administration SPA |
| High-level category | Multi-tenant SaaS CMS with page building, delivery APIs, operations controls, and a governed product Marketplace |
| Current repository status | Source, migrations, tests, UI, scripts, and documentation implement the original CMS phases, V2 SaaS phases, and V3 Marketplace phases through Phase 15. This is repository implementation evidence, not proof of production deployment or launch. |
| Project-level license | UNKNOWN U-10 and NEEDS_OWNER_CONFIRMATION NOC-16. backend/Cargo.toml declares MIT OR Apache-2.0 for the Rust crate, but no root license file establishes terms for the whole repository. |
| Primary maintainership | UNKNOWN U-12 and NEEDS_OWNER_CONFIRMATION NOC-15. No CODEOWNERS file, maintainer roster, or contribution guide assigns ownership. |

## 2. Project Purpose

ZinharCMS addresses the need to define, manage, review, publish, and deliver structured content and page layouts while separating editorial administration from consuming websites or applications. Its current code also adds organization isolation, subscription and quota controls, operational administration, and a reviewed Marketplace lifecycle.

### Implemented Capabilities

- Identity, session, global-role, organization membership, and tenant-context handling.
- Content-type schemas, dynamic entries, workflow transitions, media, pages, page versions, a visual page builder, comments, settings, navigation, delivery APIs, and CMS webhooks.
- Organization administration, PostgreSQL row-level security, plans, quotas, Stripe subscription integration, audit/email records, beta feedback, and readiness tooling.
- Built-in host plugins.
- Marketplace creator/listing/version/submission, validation, review, catalog, installation, runtime policy, kill switches, host-owned adapters, purchases, entitlements, ledger, payout eligibility/request records, reviews, abuse reporting, analytics, creator tooling, and readiness evidence.

### Partially Implemented Capabilities

- Public delivery is implemented, but the intended production organization/domain selection contract is UNKNOWN U-08.
- Local media and Marketplace artifact storage is implemented, but production storage and delivery topology is UNKNOWN U-05.
- Webhook dispatch and page preview work in the application process; no durable worker or multi-instance coordination is present.
- Localization infrastructure supports English and Persian with RTL behavior, but current coverage documentation conflicts with code and complete UI coverage is not verified.
- SaaS alert definitions exist, but automatic evaluation and delivery are IMPLEMENTATION_STATUS_UNCLEAR ISU-02.
- Marketplace finance supports current records and full-refund effects, but automated transfers, partial refunds, and dispute automation are not implemented.

### Planned Capabilities Not Implemented

- PLANNED_NOT_IMPLEMENTED PNI-01: arbitrary Marketplace package server-side execution without an official sandbox remains blocked/deferred.
- PLANNED_NOT_IMPLEMENTED PNI-02: automated Marketplace payout transfer execution and settlement remain future work.
- PLANNED_NOT_IMPLEMENTED PNI-03: partial-refund and dispute automation remain future work.
- PLANNED_NOT_IMPLEMENTED PNI-04: external delivery for critical Marketplace abuse notifications remains future work.
- PLANNED_NOT_IMPLEMENTED PNI-05: Marketplace runtime execution telemetry, warehouse export, and anomaly alerting remain future work.

### Unclear Capabilities

- IMPLEMENTATION_STATUS_UNCLEAR ISU-01: organization domain records and routes exist, but a complete verification-to-public-routing lifecycle is not established.
- IMPLEMENTATION_STATUS_UNCLEAR ISU-03: beta/GA documents and readiness scripts exist, but actual deployment or launch state cannot be established from this repository.
- Production monitoring, recovery, release promotion, support ownership, and infrastructure boundaries are outside current repository evidence.

## 3. Intended Users

| User group | Repository evidence | Supported activity |
| --- | --- | --- |
| Content authors, editors, administrators, and viewers | Global and organization roles in backend/src/services/rbac.rs; CMS pages in frontend/src/pages | Create, review, publish, read, and administer content according to role |
| Organization owners, administrators, and billing managers | Organization routes, billing routes, OrganizationPage, BillingPage | Manage members, invitations, organization settings, plans, usage, and organization controls |
| Page and site builders | PagesPage, component registry routes, page migrations | Compose page JSON from registered components, preview it, and manage versions/workflow |
| Delivery API consumers and site/application developers | backend/src/routes/delivery.rs and docs/API.md | Read published entries, pages, public settings, navigation, sitemap, and robots output |
| Marketplace creators and integration developers | Marketplace creator/upload routes, scripts/marketplace-cli.mjs, docs/MARKETPLACE_CREATOR_GUIDE.md | Prepare, validate, package, submit, and monitor reviewed Marketplace products |
| Marketplace reviewers and global administrators | Marketplace review/moderation/runtime/analytics routes and MarketplacePage | Review submissions, moderate products and feedback, manage global controls, and inspect aggregate health |
| Platform operators | Docker/Compose files, readiness endpoints, scripts, V2/V3 runbooks | Run repository-provided environments, inspect readiness, and execute documented release/readiness checks |
| Repository contributors and AI coding agents | Manifests, CI workflows, tests, AGENTS.md, this OKF | Navigate, change, test, and document the codebase under repository rules |

The repository does not contain a separate public website application. End-user presentation beyond the administration SPA is expected to consume the Delivery API or be supplied externally.

## 4. Major Capabilities

| Capability | High-level description | Primary evidence paths | Status | Related future OKF phase |
| --- | --- | --- | --- | ---: |
| Identity and access | Registration, login, token rotation/logout, global roles, organization membership, and tenant context | backend/src/routes/auth.rs; backend/src/middleware; backend/src/services/jwt.rs; backend/src/services/rbac.rs | IMPLEMENTED | 7 |
| Structured content and workflow | Content-type schemas, entries, validation, editorial states, and comments | backend/src/routes/content.rs; backend/src/services/entry_validation.rs; backend/src/services/workflow.rs | IMPLEMENTED | 3 and 8 |
| Media library | Validated upload, metadata, variants, local byte storage, and public URLs | backend/src/routes/media.rs; backend/src/services/media_processing.rs | IMPLEMENTED with storage/transaction limitations | 3, 7, and 10 |
| Pages and visual builder | Page JSON, component registry, versions, workflow, drag/drop builder, and live preview | backend/src/routes/pages.rs; frontend/src/pages/PagesPage.tsx | IMPLEMENTED | 3, 4, and 8 |
| Public delivery and SEO | Published content/pages/settings/navigation, Redis cache, sitemap, and robots | backend/src/routes/delivery.rs; backend/src/services/cache.rs | IMPLEMENTED with UNKNOWN U-08 tenant-routing intent | 2, 6, and 8 |
| Webhooks and built-in plugins | Signed CMS webhooks and host-owned in-process CMS plugin hooks | backend/src/routes/webhooks.rs; backend/src/services/webhooks.rs; backend/src/plugins | IMPLEMENTED with non-durable dispatch | 3, 8, and 9 |
| Multi-tenant SaaS | Organizations, members, invitations, RLS, plans, quotas, billing, audit/email records, beta operations | migrations 0008-0014; organization/billing/beta routes | IMPLEMENTED with operational gaps | 2, 5, 7, 8, and 10 |
| Administration SPA | Protected routes and pages for CMS, organizations, billing, beta, workflow, and Marketplace | frontend/src/router.tsx; frontend/src/pages; frontend/src/stores/useAppStore.ts | IMPLEMENTED; localization coverage is partial | 4 |
| Marketplace publication | Creator onboarding, listings, package upload, validation, review, moderation, and catalog | migrations 0015-0018; backend/src/routes/marketplace.rs; Marketplace services | IMPLEMENTED | 9 |
| Marketplace installation and runtime policy | Tenant installation lifecycle, permissions, kill switches, components, template import, and hook authorization | migrations 0019-0021; Marketplace installation/runtime/adapter routes and services | IMPLEMENTED host-owned boundary; external code execution not implemented | 7 and 9 |
| Marketplace finance and feedback | Purchases, entitlements, ledger, payout eligibility/request, reviews, abuse reports, and notifications | migrations 0022-0025; finance/feedback routes and services | IMPLEMENTED with planned finance/notification limits | 8 and 9 |
| Marketplace analytics and readiness | Creator/admin aggregates, creator CLI and samples, QA/load checks, beta and GA evidence scripts and runbooks | analytics services; scripts; docs/V3_PHASE_ELEVEN.md through V3_PHASE_FIFTEEN.md | IMPLEMENTED as repository tooling; actual launch state unclear | 9 and 10 |

## 5. System Boundaries

### Inside ZinharCMS

- React administration SPA and its browser-side state/API client.
- One Rust/Axum backend process containing routes, middleware, policies, SQL, integrations, and built-in plugins.
- PostgreSQL schema and SQLx migrations.
- Redis cache and rate-limit integration.
- Repository-defined local filesystem handling for media and Marketplace artifacts.
- Host-owned Marketplace components, template imports, hooks, and policy decisions.

### External Dependencies and Actors

- PostgreSQL and Redis runtime services.
- Stripe HTTP APIs and signed Stripe webhook events when billing/paid Marketplace features are configured.
- Optional outbound email webhook provider and customer-configured CMS webhook destinations.
- Browsers and external applications consuming the administration or Delivery APIs.
- Nginx in the repository-provided frontend production image.

### Deployment-Specific Boundaries

docker-compose.yml defines local PostgreSQL, Redis, and pgAdmin. docker-compose.prod.yml defines a production-like assembly containing backend, frontend, database, cache, and local volumes. Neither file proves the actual production topology.

- UNKNOWN U-01: production ingress, TLS, networking, scaling, and hosting.
- UNKNOWN U-05: production media/artifact storage and delivery.
- UNKNOWN U-14: production email provider and retry behavior.
- UNKNOWN U-03: monitoring, alerting, dashboards, and SLOs.

## 6. High-Level Technology Summary

| Technology | Role | Verified version or constraint | Evidence |
| --- | --- | --- | --- |
| Rust | Backend language | Edition 2024; Docker pins 1.87; CI uses stable | backend/Cargo.toml; backend/Dockerfile; backend CI |
| Axum | HTTP routing, extractors, middleware, multipart, WebSocket | 0.8 | backend/Cargo.toml; backend/src/routes |
| Tokio | Async runtime and process-local tasks/broadcasts | 1.45 | backend/Cargo.toml; backend/src/main.rs; backend/src/state.rs |
| SQLx | PostgreSQL pool, migrations, and handwritten queries | 0.8 | backend/Cargo.toml; backend/src/db; backend/migrations |
| PostgreSQL | System of record, constraints, indexes, and RLS | 16-alpine in repository Compose/CI | docker-compose files; migrations |
| Redis | Delivery cache, rate limiting, and readiness dependency | 7-alpine; Rust client 0.28 | Compose files; backend/Cargo.toml; cache/rate-limit services |
| React | Administration SPA | 19.1 | frontend/package.json |
| TypeScript | Frontend language and contract typing | 5.8.x | frontend/package.json; TypeScript configs |
| Vite | Frontend development/build tooling | 6.3 | frontend/package.json; frontend/vite.config.ts |
| React Router and Zustand | Browser routing and application/session/organization state | 7.6 and 5.0 | frontend/package.json; router.tsx; useAppStore.ts |
| Docker Compose and Nginx | Reference orchestration and static SPA serving | Compose 3.9 files; Nginx 1.27 image | docker-compose files; frontend/Dockerfile.prod |
| GitHub Actions | Backend/frontend quality gates | Workflow action versions are declared in YAML | .github/workflows |

The complete technology evidence remains in [Phase Zero Technology Inventory](../../okf-bootstrap/01-technology-inventory.md).

## 7. Repository Components

| Component | Purpose |
| --- | --- |
| backend | Rust API crate, migrations, runtime configuration, Docker images, and colocated tests |
| frontend | React SPA, routes/pages/components/state/API types, styles, tests, and Docker/Nginx files |
| docs | Existing API, architecture, phase, operations, Marketplace, sample, and diagram documentation |
| scripts | Marketplace CLI and PowerShell readiness/load/GA checks |
| .github/workflows | Backend and frontend CI quality gates |
| docker-compose.yml | Local PostgreSQL, Redis, and pgAdmin services |
| docker-compose.prod.yml | Production-like container assembly; not proof of the deployed environment |
| .env.example and env.example | Non-secret environment variable templates; .env is ignored and may contain secrets |
| okf-bootstrap | Phase Zero evidence and planning reports |
| okf | Incremental knowledge/navigation layer created by the current OKF phases |

See the [Repository Map](repository-map.md) for path-level navigation.

## 8. Development Lifecycle

Only commands declared by README files, manifests, CI workflows, scripts, or Dockerfiles are listed.

| Activity | Working directory | Verified command or mechanism | Evidence |
| --- | --- | --- | --- |
| Prepare local environment | Repository root | Copy-Item .env.example .env | README.md |
| Start local infrastructure | Repository root | npm run dev:infra | package.json; starts PostgreSQL, Redis, and pgAdmin |
| Start default Compose definition | Repository root | npm run dev | package.json; current default Compose file contains infrastructure services only |
| Run backend | Repository root | npm run dev:backend | package.json |
| Run backend directly | backend | cargo run | README.md |
| Install frontend dependencies | frontend | npm install | README.md and frontend CI |
| Run frontend | Repository root | npm run dev:frontend | package.json |
| Run frontend directly | frontend | npm run dev | frontend/package.json |
| Test backend | Repository root | npm run test:backend | package.json |
| Format backend | backend | cargo fmt --check | backend CI |
| Lint backend | backend | cargo clippy --all-targets --all-features -- -D warnings | backend CI; current handoff records legacy warnings, so do not assume this gate is green without running it |
| Test frontend | Repository root | npm run test:frontend | package.json |
| Lint/typecheck frontend | frontend | npm run lint; npm run typecheck | frontend/package.json and CI |
| Build frontend | Repository root | npm run build:frontend | package.json |
| Build backend production image binary | backend Docker build | cargo build --release | backend/Dockerfile.prod |
| Package Marketplace sample/product | Repository root | npm run marketplace -- validate PATH; npm run marketplace -- pack PATH --force | package.json; Marketplace creator guide |
| Deployment | Not established | docker-compose.prod.yml defines a production-like assembly, but no production deploy command or deployment workflow is defined | UNKNOWN U-01 and U-06 |

## 9. Documentation Landscape

| Location | Role and authority |
| --- | --- |
| ../../README.md | Current scope summary and quick start; source and manifests outrank it |
| ../../docs/ARCHITECTURE.md | Current high-level architecture; verify against runtime code and deployment configuration |
| ../../docs/API.md | Broad manual API guide; route composition and handlers are authoritative |
| ../../docs/I18N.md | Localization guidance; DOCUMENTATION_CODE_CONFLICT DCC-02 affects its coverage statement |
| ../../docs/PHASE_*.md | Historical records for original CMS phases |
| ../../docs/V2_*.md | V2 SaaS phase records, guides, release notes, and runbook |
| ../../docs/V3_*.md | V3 Marketplace scope, policy, domain, phase, guide, and operations records |
| ../../docs/diagrams | 43 standalone Mermaid sources plus conventions, evidence, and traceability records |
| ../../backend/src/routes/mod.rs | Reachable route composition and generated OpenAPI registration |
| ../../backend/migrations | Intended schema authority |
| ../../okf-bootstrap | Phase Zero audit and planning evidence |
| ../README.md and ../index.yaml | OKF navigation and machine-readable document registry |

Known conflicts are recorded, not corrected, in this phase:

- DOCUMENTATION_CODE_CONFLICT DCC-01: Phase Three can imply that the visual builder is still future.
- DOCUMENTATION_CODE_CONFLICT DCC-02: I18N.md understates current translation coverage while coverage remains incomplete.
- DOCUMENTATION_CODE_CONFLICT DCC-03: the Marketplace domain model describes implemented finance/feedback entities as future.
- DOCUMENTATION_CODE_CONFLICT DCC-04: the architecture audit says Marketplace finance is absent.
- DOCUMENTATION_CODE_CONFLICT DCC-05 through DCC-08 and DCC-10: several Mermaid status/lifecycle descriptions are stale.
- DOCUMENTATION_CODE_CONFLICT DCC-09: API.md contains one incorrect billing webhook path; the router uses /api/billing/stripe/webhook.

No existing documentation was changed during Phase 1.

## 10. Known Limitations and Unknowns

| Marker | High-level issue |
| --- | --- |
| UNKNOWN U-01 | Actual production hosting, ingress, TLS, networking, and scaling topology |
| UNKNOWN U-03 | Production logging/metrics/tracing collection, dashboards, alerting, and SLO baseline |
| UNKNOWN U-04 | Backup/restore process, RPO, RTO, retention, and restore-test evidence |
| UNKNOWN U-05 | Production media and Marketplace artifact storage/delivery topology |
| UNKNOWN U-06 | Environment promotion, release cadence, rollback authority, and incident ownership |
| UNKNOWN U-08 | Intended public organization/custom-domain delivery selection contract |
| UNKNOWN U-09 | Officially supported Rust, Node.js, browser, and contributor toolchain versions |
| UNKNOWN U-10 | Project-level license and distribution policy |
| UNKNOWN U-12 | Code ownership, required reviewers, branch protection, and merge policy |
| UNKNOWN U-14 | Production email provider and retry/failure behavior |
| UNKNOWN U-15 | Support and security incident escalation contacts |
| NEEDS_OWNER_CONFIRMATION NOC-10 | Marketplace execution, appeal, cleanup, and finance roadmap boundaries |
| IMPLEMENTATION_STATUS_UNCLEAR ISU-01 | Complete domain-verification-to-public-routing behavior |
| IMPLEMENTATION_STATUS_UNCLEAR ISU-02 | Automatic SaaS alert evaluation and delivery |
| IMPLEMENTATION_STATUS_UNCLEAR ISU-03 | Actual deployment, beta completion, or GA launch status |

See [index.yaml](../index.yaml) and [Phase Zero Knowledge Gaps](../../okf-bootstrap/09-knowledge-gaps.md) for the full marker register.

## 11. Related Documents

- [OKF Entry Point](../README.md)
- [Machine-Readable Index](../index.yaml)
- [Repository Map](repository-map.md)
- [Project Glossary](glossary.md)
- [Navigation Guide](navigation-guide.md)
- [Phase 1 Source Register](../references/source-register.md)
- [Repository README](../../README.md)
- [Existing Architecture](../../docs/ARCHITECTURE.md)
- [Existing API Guide](../../docs/API.md)
- [Marketplace Scope](../../docs/V3_MARKETPLACE_SCOPE.md)
- [Diagram Catalog](../../docs/diagrams/README.md)
