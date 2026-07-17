# Conventions Inventory

## Classification

- EXPLICIT_CONVENTION means the repository states or mechanically enforces the rule.
- INFERRED_CONVENTION means the pattern is consistent in current code but is not declared as a durable project rule.
- UNKNOWN means the repository does not provide enough evidence.

## Repository and Naming Conventions

| Area | Classification | Observed convention | Evidence and gaps |
| --- | --- | --- | --- |
| Rust files and modules | INFERRED_CONVENTION | snake_case files/modules, PascalCase types, snake_case functions | Consistent backend source; no local Rust style guide beyond formatter/linter tooling |
| TypeScript/React files | INFERRED_CONVENTION | PascalCase page/component files, camelCase functions and values, type names in PascalCase | Consistent frontend source; some infrastructure files use lowercase names |
| SQL | INFERRED_CONVENTION | snake_case objects, plural table names, UUID primary keys, numbered descriptive migration files | Consistent across migrations; enum versus text-state policy is not explicit |
| HTTP paths | INFERRED_CONVENTION | Lowercase plural resources, nested actions when domain-specific, /api and /api/v1 prefixes | Route composition; no explicit versioning policy |
| Environment variables | INFERRED_CONVENTION | Upper snake case, loaded centrally by backend configuration | .env.example and config.rs |
| Documentation files | INFERRED_CONVENTION | Upper snake case for phase documents, numbered kebab-case Mermaid files | Existing docs; final OKF naming is not yet established |
| Git branches | INFERRED_CONVENTION | Feature or documentation prefixes are used | Current branch is docs/okf-phase-zero; no branch policy file |

## Backend Conventions

| Area | Classification | Observed convention | Evidence and gaps |
| --- | --- | --- | --- |
| Runtime structure | EXPLICIT_CONVENTION in architecture docs | Modular monolith with route, service, model/config, middleware, plugin, and migration layers | docs/ARCHITECTURE.md and source layout |
| Route composition | INFERRED_CONVENTION | Domain routers merged into public, authenticated, or tenant stacks | backend/src/routes/mod.rs |
| Shared state | INFERRED_CONVENTION | AppState holds database, Redis, configuration, registries, and preview broadcast state | backend/src/state.rs |
| Errors | EXPLICIT_CONVENTION in code | AppError maps domain failures to status codes and an error/message JSON envelope | backend/src/error.rs |
| Validation | INFERRED_CONVENTION | Extractor checks plus dedicated validation/workflow services and SQL constraints | Entry, media, Marketplace, billing, and feedback services |
| Logging | INFERRED_CONVENTION | tracing macros and request tracing middleware | Runtime and route source; log schema and redaction policy are not documented |
| Configuration | INFERRED_CONVENTION | Environment-driven typed configuration with startup validation | backend/src/config.rs and .env.example |
| Database access | INFERRED_CONVENTION | SQLx compile-time/runtime queries, explicit transactions for multi-row mutations, RLS-scoped connections for tenant data | Services and routes; transaction rules are not consolidated |
| Time and IDs | INFERRED_CONVENTION | UTC timestamps and UUID identifiers | Models and migrations |
| Background work | INFERRED_CONVENTION | Short-lived Tokio tasks for webhook delivery; in-process broadcast for preview | No durable job framework |

## Authentication, Authorization, and Tenancy Conventions

| Area | Classification | Observed convention | Evidence and gaps |
| --- | --- | --- | --- |
| Authentication | EXPLICIT_CONVENTION in code/docs | Argon2 passwords, short-lived bearer access JWT, rotating hashed refresh tokens, HttpOnly SameSite=Lax refresh cookie | Auth routes, services, and API docs |
| Tenant selection | EXPLICIT_CONVENTION in code/docs | X-Organization-Id selects the tenant after identity and membership validation | Tenant middleware and V2 tenant docs |
| Tenant database context | EXPLICIT_CONVENTION in code | PostgreSQL local settings carry organization, user, and bypass context | rls.rs and RLS migrations |
| Authorization | INFERRED_CONVENTION | Global roles, tenant member roles, creator ownership, and domain gates are combined per route | RBAC services and handlers; no single final matrix |
| Bypass | EXPLICIT_CONVENTION in code | Global administrative or controlled service paths may use explicit RLS bypass helpers | rls.rs and relevant routes |
| Public delivery tenant | UNKNOWN | Current implementation resolves organization slug default | Owner intent is required before this becomes a documented convention |

## API Conventions

| Area | Classification | Observed convention | Evidence and gaps |
| --- | --- | --- | --- |
| JSON errors | EXPLICIT_CONVENTION in code | error and message fields with domain-to-status mapping | AppError |
| Success DTOs | INFERRED_CONVENTION | Typed JSON structures, domain-specific list envelopes, occasional no-content responses | Route handlers and frontend client |
| Pagination | INFERRED_CONVENTION | Query-based pagination with domain-specific names and response metadata | Catalog, queues, and lists; not uniform |
| Uploads | INFERRED_CONVENTION | Multipart request, service-level validation, filesystem persistence, row metadata | Media and Marketplace routes |
| Idempotency | INFERRED_CONVENTION | Provider event IDs and selected business uniqueness constraints prevent repeats | Stripe and finance code; no general client contract |
| OpenAPI | EXPLICIT_CONVENTION in code but incomplete | Utoipa annotations and a generated /openapi.json document | Some real routes are not annotated |
| API versioning | UNKNOWN | Public delivery uses /api/v1 while administrative APIs use /api | No deprecation or compatibility policy |

## Frontend Conventions

| Area | Classification | Observed convention | Evidence and gaps |
| --- | --- | --- | --- |
| Routing | INFERRED_CONVENTION | React Router route map with authenticated shell and page-level screens | frontend/src/router.tsx |
| State | INFERRED_CONVENTION | Zustand stores for authentication and organization context; local component state elsewhere | frontend/src/stores |
| API access | INFERRED_CONVENTION | Central typed fetch client, bearer header, organization header, credentials included | frontend/src/services/api.ts |
| Forms | INFERRED_CONVENTION | React Hook Form and Zod where complex validation is needed | package manifest and page source |
| Styling | EXPLICIT_CONVENTION through tooling | Tailwind CSS utilities and shared global styles | Tailwind/Vite setup and source |
| Icons | INFERRED_CONVENTION | Lucide React | package manifest and components |
| Localization | EXPLICIT_CONVENTION in docs/runtime | English and Persian dictionaries, language selection, RTL/LTR direction | i18n runtime and docs/I18N.md; coverage documentation is stale |
| Component size | UNKNOWN as policy | Several feature pages are large, especially builder and Marketplace surfaces | No documented decomposition threshold |

## Testing and Quality Conventions

| Area | Classification | Observed convention | Evidence and gaps |
| --- | --- | --- | --- |
| Rust formatting/lint/test | EXPLICIT_CONVENTION in CI | cargo fmt check, Clippy with warnings denied, and cargo test | .github/workflows/ci.yml |
| Frontend quality | EXPLICIT_CONVENTION in CI | npm clean install, lint, TypeScript check, Vitest, production build | .github/workflows/ci.yml and package scripts |
| Backend test placement | INFERRED_CONVENTION | Unit/contract/static tests colocated with modules | 117 test attributes across 38 backend source files |
| Frontend test placement | INFERRED_CONVENTION | Vitest and Testing Library tests beside relevant frontend source | 3 test files with 14 test cases |
| Integration environment | INFERRED_CONVENTION | Docker Compose PostgreSQL and Redis plus safe fixture cleanup for live smoke | Handoff history and scripts; not a standard automated suite |
| Coverage target | UNKNOWN | No repository coverage threshold or coverage report workflow | CI does not publish coverage |

## Migration, Release, and Operations Conventions

| Area | Classification | Observed convention | Evidence and gaps |
| --- | --- | --- | --- |
| Migrations | EXPLICIT_CONVENTION in code | Sequential forward SQLx migrations run at backend startup | backend/migrations and main.rs |
| Rollback | UNKNOWN | No down migrations or documented universal rollback workflow | Forward repair and restore responsibilities need owner confirmation |
| Containers | EXPLICIT_CONVENTION for local/reference environment | Dockerfiles and Docker Compose define backend, frontend, PostgreSQL, Redis, Nginx, and pgAdmin | Production equivalence is UNKNOWN |
| CI | EXPLICIT_CONVENTION | GitHub Actions performs backend and frontend quality checks | No deployment job exists |
| Deployment | UNKNOWN | No production pipeline, environment promotion model, or infrastructure-as-code is present | Owner confirmation required |
| Observability | UNKNOWN | Tracing and readiness exist, but no metrics exporter, collector, APM, dashboards, or SLO source exists | Owner confirmation required |
| Storage | INFERRED_CONVENTION for local setup only | Media and Marketplace artifacts use local filesystem paths | Production shared/object storage decision is UNKNOWN |

## Documentation and Contribution Conventions

| Area | Classification | Observed convention | Evidence and gaps |
| --- | --- | --- | --- |
| Phase records | INFERRED_CONVENTION | Immutable or mostly historical phase documents plus cumulative current docs | Many phase files and README |
| Diagrams | INFERRED_CONVENTION | Standalone numbered Mermaid files linked from TRACEABILITY.md | 43 source files; no renderer validation in CI |
| Persistent handoff | EXPLICIT_CONVENTION | HANDOFF.md is updated at milestones and before stopping | AGENTS.md |
| Lessons learned | EXPLICIT_CONVENTION | Recurring mistakes are recorded in the external mistakes log | User-provided project instructions |
| Commits | INFERRED_CONVENTION | Recent history commonly uses Conventional Commit style prefixes | No explicit commit policy |
| Review ownership | UNKNOWN | No CODEOWNERS or contribution guide defines reviewers | Owner confirmation required |
| Documentation authority | INFERRED_CONVENTION | Code and migrations supersede stale planning prose | Phase Zero method; a formal ownership map is absent |

## Toolchain Drift

The repository contains version differences that should be resolved as support policy rather than silently normalized:

- the backend Dockerfile pins Rust 1.87 while CI uses the stable channel;
- the frontend Dockerfile uses Node 24 while CI uses Node 22;
- PostgreSQL and Redis are major-version pinned in Docker Compose;
- pgAdmin uses a floating latest tag.

The intended supported production and contributor toolchain versions are NEEDS_OWNER_CONFIRMATION.

## Recommended Convention Documents

Later OKF phases should create concise authoritative documents for code structure, naming, errors, API behavior, tenancy/RLS, state transitions, migrations, testing, release operations, documentation ownership, and contribution workflow. Inferred patterns should not be promoted to mandatory rules without repository enforcement or owner confirmation.
