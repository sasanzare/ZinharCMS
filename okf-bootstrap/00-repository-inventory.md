# Repository Inventory

## Inventory Basis

- Repository: ZinharCMS
- Branch inspected: docs/okf-phase-zero
- Commit inspected: 61ed3b38
- Inspection date: 2026-07-17
- Source of truth order: runtime code and migrations, configuration, tests, then documentation
- Scope: tracked repository content plus the names of ignored runtime/generated directories
- Excluded from detailed enumeration: backend/target, frontend/node_modules, frontend/dist, root target, uploads, marketplace-dist archives, logs, lockfile contents, and temporary files

The repository was clean before Phase Zero started. The only intentional changes in this phase are the required analytical reports and the mandatory HANDOFF.md update.

## Tracked Inventory

| Measure | Count | Evidence |
| --- | ---: | --- |
| Tracked files | 287 | git ls-files |
| Backend tracked files | 104 | Git path grouping |
| Frontend tracked files | 51 | Git path grouping |
| Documentation-tree tracked files | 115 | Git path grouping |
| Rust source files | 73 | File-extension grouping |
| SQL files | 28 | 26 migrations plus 2 documentation fixtures |
| Markdown files | 66 | Root and docs tree |
| Mermaid files | 43 | docs/diagrams |
| TypeScript/TSX files | 38 | Frontend source and configuration |
| PowerShell scripts | 5 | scripts |

## Important Repository Tree

    ZinharCMS/
    +-- .github/
    |   +-- workflows/
    +-- backend/
    |   +-- migrations/
    |   +-- src/
    |   |   +-- db/
    |   |   +-- middleware/
    |   |   +-- models/
    |   |   +-- plugins/
    |   |   +-- routes/
    |   |   +-- services/
    |   +-- Cargo.toml
    |   +-- Dockerfile
    |   +-- Dockerfile.prod
    +-- docs/
    |   +-- diagrams/
    |   +-- marketplace-samples/
    |   +-- API.md
    |   +-- ARCHITECTURE.md
    |   +-- I18N.md
    |   +-- PHASE_*.md
    |   +-- V2_*.md
    |   +-- V3_*.md
    +-- frontend/
    |   +-- public/
    |   +-- src/
    |   |   +-- components/
    |   |   +-- hooks/
    |   |   +-- i18n/
    |   |   +-- pages/
    |   |   +-- services/
    |   |   +-- stores/
    |   |   +-- styles/
    |   |   +-- test/
    |   |   +-- types/
    |   +-- package.json
    |   +-- Dockerfile
    |   +-- Dockerfile.prod
    |   +-- nginx.conf
    +-- scripts/
    +-- .env.example
    +-- env.example
    +-- docker-compose.yml
    +-- docker-compose.prod.yml
    +-- AGENTS.md
    +-- HANDOFF.md
    +-- package.json
    +-- README.md

## Directory Responsibilities

| Path | Purpose and contents | System layer | Runtime/tooling/docs/deployment | Independent OKF document |
| --- | --- | --- | --- | --- |
| .github/workflows | Backend and frontend CI definitions | Delivery pipeline | Tooling | Yes, under development and testing |
| backend/migrations | Forward SQLx migrations 0001 through 0026 | Persistence | Runtime schema | Yes, database migration catalog |
| backend/src/routes | Axum route declarations, DTOs, handlers, and many SQL queries | API/application | Runtime | Yes, API domain inventory |
| backend/src/services | Cross-route policies, validation, security, RLS, billing, Marketplace rules, and integrations | Application/domain/infrastructure | Runtime and static contract tests | Yes, backend service map |
| backend/src/middleware | Bearer authentication, tenant selection, quota/rate limit checks, and security headers | API boundary | Runtime | Yes, request lifecycle and security |
| backend/src/models | A small set of SQLx data structs | Data mapping | Runtime, but not the complete domain model | Link from database and backend docs |
| backend/src/plugins | Built-in CmsPlugin trait, registry, and SEO plugin | Extensibility | Runtime | Yes, plugins and extensibility |
| frontend/src/pages | Route-level React admin screens | Presentation | Runtime | Yes, frontend route/page map |
| frontend/src/services | Browser API client and token/organization header behavior | Frontend integration | Runtime | Link from API and frontend docs |
| frontend/src/stores | Zustand application/session/organization state | Frontend state | Runtime | Link from frontend architecture |
| frontend/src/i18n | Locale metadata, provider, messages, and RTL behavior | Frontend cross-cutting | Runtime | Yes, localization |
| frontend/src/components | Shared shell, auth gate, form renderer, and status badge | Presentation | Runtime | Usually link from frontend architecture |
| frontend/src/types | Shared browser-side API contract types | Frontend contract | Runtime/build | Link from API coverage |
| docs | Phase records, API/architecture guides, operations material, fixtures, and Marketplace samples | Knowledge base | Documentation/test support | Preserve and index; do not duplicate wholesale |
| docs/diagrams | Mermaid set plus evidence, traceability, conventions, and ambiguity records | Visual knowledge base | Documentation | Preserve; link from future OKF diagrams index |
| scripts | GA/readiness/load checks and Marketplace creator CLI | Operations/tooling | Tooling | Yes, operations and development commands |
| marketplace-dist | Ignored local package archives | Generated artifact output | Tooling output | No; document generation behavior only |
| backend/uploads and root uploads | Ignored local file storage | Persistence infrastructure | Runtime data | No file inventory; document lifecycle and risk |
| backend/target, frontend/node_modules, frontend/dist | Compiler, dependency, and build output | Generated | Tooling output | No |

## Root Files

| File | Responsibility | OKF treatment |
| --- | --- | --- |
| README.md | Current product scope and local startup | Link from OKF project overview; avoid copying phase history |
| AGENTS.md | Repository handoff and safety protocol | Link from development workflow |
| HANDOFF.md | Cumulative recovery state | Reference current checkpoint only; do not treat historical overrides as product truth |
| backend/Cargo.toml | Rust crate metadata and dependencies | Primary technology evidence |
| frontend/package.json | Frontend dependencies and scripts | Primary technology evidence |
| package.json | Cross-project developer commands | Link from development quick start |
| docker-compose.yml | Local PostgreSQL, Redis, and pgAdmin | Primary local-runtime evidence |
| docker-compose.prod.yml | Production-like PostgreSQL, Redis, backend, frontend, and volumes | Primary deployment evidence, not proof of a production platform |
| .env.example | Most complete non-secret environment template | Primary configuration reference |
| env.example | Smaller legacy template | Keep only if a compatibility reason is confirmed; otherwise consolidate later |
| .gitignore | Generated/runtime exclusions | Evidence for repository boundaries |

## Generated and Local-Only State

- marketplace-dist contains two ignored ZIP files generated by the Marketplace CLI.
- backend/target, frontend/node_modules, and frontend/dist exist locally but are not source-of-truth.
- .env is ignored and was not inspected because it can contain secrets.
- uploads and logs are ignored runtime artifacts.
- .agents is an empty local directory and has no tracked project knowledge.

## Repository-Level Observations

- The system is one Rust API crate plus one React SPA, not a multi-repository or shared-package workspace.
- Domain code is organized primarily by route and service files rather than one directory per bounded context.
- SQL is embedded directly in route/service files; there is no separate repository abstraction layer.
- The documentation set is unusually extensive and should be indexed and corrected rather than copied into OKF.
- The final okf/ directory must not be created during Phase Zero.

