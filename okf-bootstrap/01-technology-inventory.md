# Technology Inventory

## Confidence Scale

- HIGH: version and use are directly declared in a manifest or image tag.
- MEDIUM: use is direct, but the deployed version or production choice is environment-dependent.
- LOW: evidence is indirect.

## Runtime and Application Technologies

| Technology | Version or constraint | Location | Role | Evidence | Confidence |
| --- | --- | --- | --- | --- | --- |
| Rust | Edition 2024; Docker image 1.87; CI uses stable | backend | Backend language and build | backend/Cargo.toml, backend/Dockerfile*, backend-ci.yml | HIGH |
| Axum | 0.8 | backend | HTTP routing, extractors, middleware, multipart, WebSocket | backend/Cargo.toml, backend/src/routes | HIGH |
| Tokio | 1.45 | backend | Async runtime, filesystem, signals, spawned webhook tasks | backend/Cargo.toml, backend/src/main.rs, services/webhooks.rs | HIGH |
| Tower HTTP | 0.6 | backend | CORS, compression, timeout, request IDs, trace, static files | backend/Cargo.toml, backend/src/main.rs, routes/mod.rs | HIGH |
| SQLx | 0.8 | backend | PostgreSQL pool, migrations, queries, row mapping | backend/Cargo.toml, backend/src/db, migrations | HIGH |
| PostgreSQL | 16-alpine | persistence | System of record, constraints, indexes, forced RLS | docker-compose*.yml, migrations | HIGH |
| Redis | 7-alpine; Rust client 0.28 | backend/infrastructure | Delivery cache, request rate limits, readiness | docker-compose*.yml, services/cache.rs, services/rate_limit.rs | HIGH |
| React | 19.1 | frontend | Admin SPA | frontend/package.json | HIGH |
| React DOM | 19.1 | frontend | Browser rendering | frontend/package.json | HIGH |
| React Router DOM | 7.6 | frontend | Client routing and protected routes | frontend/package.json, frontend/src/router.tsx | HIGH |
| TypeScript | 5.8.x | frontend | Strict frontend language and type checking | frontend/package.json, tsconfig.app.json | HIGH |
| Vite | 6.3 | frontend | Development server and production build | frontend/package.json, vite.config.ts | HIGH |
| Zustand | 5.0 | frontend | Session, organization, and shell state | frontend/package.json, useAppStore.ts | HIGH |
| React Hook Form | 7.56 | frontend | Form state | frontend/package.json and page components | HIGH |
| Zod | 3.25 | frontend | Form/schema validation | frontend/package.json and page components | HIGH |
| dnd-kit | 6.3/10.0 | frontend | Page-builder drag and drop | frontend/package.json, PagesPage.tsx | HIGH |
| Tailwind CSS | 4.1 | frontend | CSS tooling through Vite | frontend/package.json, vite.config.ts | HIGH |
| Lucide React | 0.511 | frontend | UI icons | frontend/package.json, components/pages | HIGH |

## Security and Integration Technologies

| Technology | Version | Location | Role | Evidence | Confidence |
| --- | --- | --- | --- | --- | --- |
| Argon2 | 0.5 | backend | Argon2id password hashing and verification | Cargo.toml, services/password.rs | HIGH |
| HMAC + SHA-256 | hmac 0.12, sha2 0.10 | backend | HS256 access tokens, refresh-token hashing, webhook signatures | services/jwt.rs, services/webhooks.rs | HIGH |
| Utoipa | 4 | backend | Generated OpenAPI schemas and paths | Cargo.toml, routes/mod.rs and route attributes | HIGH |
| Reqwest | 0.12 with rustls | backend | Stripe calls, email webhook provider, outbound CMS webhooks | Cargo.toml, stripe_billing.rs, email.rs, webhooks.rs | HIGH |
| Image | 0.25 | backend | JPEG, PNG, and WebP processing | Cargo.toml, media_processing.rs | HIGH |
| UUID | 1.16 with UUIDv7 | backend/database | Primary identifiers | Cargo.toml, migrations, models | HIGH |
| Chrono | 0.4 | backend | Timestamp/date serialization and rules | Cargo.toml, routes/services | HIGH |
| Stripe HTTP API | No SDK; endpoint/config driven | backend | Subscription checkout, Marketplace checkout, webhooks, refunds | stripe_billing.rs, routes/billing.rs, env templates | MEDIUM |
| Email log/webhook bridge | Repository implementation | backend | Invitation and billing notification delivery recording | services/email.rs, config.rs | HIGH |

## Build, Test, and Delivery Technologies

| Technology | Version | Location | Role | Evidence | Confidence |
| --- | --- | --- | --- | --- | --- |
| Cargo | Rust toolchain dependent | root/backend | Backend build, format, lint, and tests | package.json, CI | HIGH |
| npm | Node image/toolchain dependent | root/frontend | Frontend dependency and script runner | package.json files, Dockerfiles, CI | HIGH |
| Node.js | Docker development 24; CI 22 | frontend | Vite and Marketplace CLI runtime | Dockerfile*, frontend-ci.yml | HIGH |
| Vitest | 3.1 | frontend | Unit/component tests in jsdom | frontend/package.json, vitest.config.ts | HIGH |
| Testing Library | React 16.3, jest-dom 6.6 | frontend | UI behavior assertions | package.json, test/setup.ts | HIGH |
| ESLint | 9.27 | frontend | TypeScript/React linting | frontend/package.json, eslint.config.js | HIGH |
| rustfmt and Clippy | CI stable toolchain | backend | Formatting and lint gates | backend-ci.yml | HIGH |
| Docker Compose | File format 3.9 | root | Local and production-like orchestration | docker-compose*.yml | HIGH |
| Docker | Rust, Node, Debian, Nginx images | backend/frontend | Development and production-like images | Dockerfiles | HIGH |
| Nginx | 1.27-alpine | frontend production image | Static SPA hosting; no API reverse proxy | frontend/Dockerfile.prod, nginx.conf | HIGH |
| GitHub Actions | checkout v4, setup-node v4, rust-cache v2 | .github/workflows | Backend and frontend CI | workflow files | HIGH |
| PowerShell | Environment-dependent | scripts | Readiness and load-smoke automation | scripts/*.ps1 | MEDIUM |
| Node CLI | Dependency-free repository script | scripts | Marketplace validate, pack, and submit workflow | scripts/marketplace-cli.mjs | HIGH |

## Explicitly Absent or Deferred

| Capability | Finding | Evidence confidence |
| --- | --- | --- |
| ORM with generated entities | Not present; SQLx queries are handwritten | HIGH |
| Durable queue/worker | Not found; webhook work can use process-local Tokio tasks | HIGH |
| Search service | Not found; Marketplace search is PostgreSQL query logic | HIGH |
| S3/object-storage provider | Not found; uploads and Marketplace artifacts use local filesystem paths | HIGH |
| CDN | Not found | HIGH |
| Metrics exporter/collector | Not found | HIGH |
| Central tracing/log vendor | Not found | HIGH |
| Deployment pipeline | Not found; CI builds/tests only | HIGH |
| Arbitrary uploaded package execution | Deliberately not implemented | HIGH |
| Automated Marketplace payout transfers | Deferred; onboarding, eligibility, and ledger exist | HIGH |

## Version and Environment Risks

- Node 24 is used in Docker development/production builds while CI uses Node 22.
- Rust Docker images pin 1.87 while CI tracks stable.
- pgAdmin uses the floating latest tag.
- Frontend dependencies use caret ranges and package-lock.json is the exact npm resolution source.
- The actual production PostgreSQL, Redis, Node, and Rust versions are UNKNOWN because no deployed environment metadata is stored in the repository.

