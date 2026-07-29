# ZinharCMS

[![Backend CI](https://github.com/sasanzare/ZinharCMS/actions/workflows/backend-ci.yml/badge.svg)](https://github.com/sasanzare/ZinharCMS/actions/workflows/backend-ci.yml)
[![Frontend CI](https://github.com/sasanzare/ZinharCMS/actions/workflows/frontend-ci.yml/badge.svg)](https://github.com/sasanzare/ZinharCMS/actions/workflows/frontend-ci.yml)

ZinharCMS is a multi-tenant headless CMS and SaaS administration platform built
with Rust, Axum, React, PostgreSQL, and Redis. It combines structured content,
a visual page builder, public delivery APIs, organization management, billing,
and a reviewed-product Marketplace in one repository.

The repository implementation includes the original CMS phases, the V2 SaaS
track, and V3 Marketplace phases 0.1 through 15. The planned `v3.0.0`
publication is a GitHub source-code release: it publishes the tracked source
and documentation but does not deploy or enable a production environment.
Production General Availability remains a separate operational decision.

## Release Status

| Track | Repository status |
| --- | --- |
| Core CMS | Implemented through the original Phase Seven scope |
| V2 SaaS | Implemented through the Phase Ten GA-readiness scope |
| V3 Marketplace | Implementation phases 0.1 through 15 complete |
| V3 source release | Application version `3.0.0`; GitHub source tag `v3.0.0` |
| V3 production deployment | Not part of the source release; requires separate target-environment validation and owner sign-off |
| Application version | `3.0.0` across the root, backend, and frontend manifests |
| OKF knowledge base | Complete with documented open owner and operational questions |

V3 source-release scope, deployment criteria, known limitations, support
expectations, and rollback conditions are defined in the
[Phase 15 guide](docs/V3_PHASE_FIFTEEN.md),
[release notes](docs/V3_MARKETPLACE_RELEASE_NOTES.md), and
[operations runbook](docs/V3_MARKETPLACE_OPERATIONS_RUNBOOK.md).

## Capabilities

### Core CMS

- Authentication, refresh-token sessions, global RBAC, and security hardening.
- Content-type schemas, validated content entries, editorial workflow, and
  collaboration comments.
- Media uploads, metadata, image variants, and organization-scoped ownership.
- Visual page builder, component registry, page versions, restoration, and live
  preview streaming.
- Public delivery APIs, SEO endpoints, Redis-backed caching, and signed
  webhooks.
- Built-in plugin registration and English/Persian admin localization with RTL
  support.

### Multi-Tenant SaaS

- Organizations, memberships, invitations, workspace URLs, and custom-domain
  records.
- Tenant-aware middleware and PostgreSQL forced row-level security.
- Subscription plans, usage quotas, Stripe subscription hooks, and billing
  administration.
- Audit logs, email-delivery records, SaaS alerts, beta feedback, dashboards,
  and GA blocker tracking.

### V3 Marketplace

- Creator profiles, listings, package versions, submissions, and immutable
  artifact metadata.
- Manifest, ZIP structure, compatibility, permission, and security validation.
- Reviewer queues, approval decisions, moderation, takedown, and emergency
  blocking.
- Tenant-aware catalog, search, listing details, installation lifecycle,
  version pinning, update, and rollback.
- Permission snapshots, host-owned adapters, runtime authorization policy, and
  organization/global kill switches.
- Free and paid one-time purchases, entitlements, full-refund revocation,
  revenue ledger entries, and payout-provider onboarding.
- Customer reviews, abuse reports, internal critical notifications, creator
  analytics, and global administration analytics.
- Creator CLI commands, sample packages, security QA, performance baselines,
  beta-readiness checks, and GA-readiness tooling.

## Architecture

| Area | Implementation |
| --- | --- |
| Backend | Rust 2024, Axum 0.8, Tokio, SQLx; modular monolith |
| Frontend | React 19.2, TypeScript, Vite 7, React Router 8, Zustand |
| Database | PostgreSQL 16 with embedded SQLx migrations and forced RLS |
| Cache and limits | Redis 7 for delivery caching and rate-limit counters |
| Storage | Local filesystem for CMS uploads and Marketplace artifacts |
| Authentication | Argon2id passwords, TOTP MFA, one-time recovery codes, Step-Up, HMAC-SHA256 access tokens, hashed refresh tokens |
| Local infrastructure | Docker Compose for PostgreSQL, Redis, and pgAdmin |
| Production-like images | Rust backend image and Nginx-hosted frontend image |
| CI | Separate GitHub Actions workflows for backend and frontend validation |

ZinharCMS is not implemented as independently deployed microservices. The
backend owns API routing, business services, persistence access, migrations,
and host-controlled extension adapters.

See the [architecture guide](docs/ARCHITECTURE.md), the
[architecture diagrams](docs/diagrams/README.md), and the
[OKF architecture section](okf/architecture/README.md) for verified boundaries
and evidence.

## Prerequisites

- Git.
- Rust 1.96 with `cargo`, `rustfmt`, and `clippy`.
- Node.js 24.x and npm 11.x. CI and both frontend container builds use Node.js
  24.
- Docker with Docker Compose for the repository-provided local infrastructure.
- PowerShell only when running the supplied release and readiness scripts.

See [Development Prerequisites](okf/development/prerequisites.md) for the
evidence-backed version and environment matrix.

## Quick Start

Clone the repository and create a local environment file:

```powershell
git clone https://github.com/sasanzare/ZinharCMS.git
Set-Location ZinharCMS
Copy-Item .env.example .env
```

Review `.env` before starting services. Replace the `JWT_KEY_RING` example with
a JSON key ring containing one active HS256 key with a unique identifier and a
unique random secret of at least 32 bytes. Also provision a separate
`MFA_ENCRYPTION_KEY_RING` with exactly one active AES-256-GCM key; never reuse
JWT key material. On a new empty database, set both
`BOOTSTRAP_ADMIN_EMAIL` and `BOOTSTRAP_ADMIN_PASSWORD` to provision the first
administrator, then remove both values after the successful first startup.
Public registration never grants an administrative role. The template is for
local development; do not reuse development credentials or secret values in a
deployed environment.

Install frontend dependencies:

```powershell
Set-Location frontend
npm ci
Set-Location ..
```

Start PostgreSQL, Redis, and pgAdmin:

```powershell
npm run dev:infra
```

Start the backend from the repository root in a separate terminal:

```powershell
npm run dev:backend
```

Backend startup connects to PostgreSQL and Redis, applies embedded migrations,
and provisions an administrator only when the user table is empty and both
explicit bootstrap variables are configured.

Start the frontend from the repository root in another terminal:

```powershell
npm run dev:frontend
```

Local endpoints:

| Service | URL |
| --- | --- |
| Admin UI | `http://localhost:5173` |
| API | `http://localhost:8080` |
| Liveness | `http://localhost:8080/health` |
| Readiness | `http://localhost:8080/ready` |
| pgAdmin | `http://localhost:5050` |
| PostgreSQL | `localhost:5432` |
| Redis | `localhost:6379` |

Verify the backend:

```powershell
Invoke-RestMethod http://localhost:8080/health
Invoke-RestMethod http://localhost:8080/ready
```

Stop the backend and frontend with `Ctrl+C`. Stop local infrastructure without
removing its named volumes:

```powershell
docker compose stop
```

> The root `npm run dev` command runs the local Compose file. That file contains
> infrastructure services only; it does not start the backend or frontend
> processes.

For failure indicators, reset cautions, and environment details, use the
[Local Environment Guide](okf/development/local-environment.md) and
[Troubleshooting Guide](okf/operations/troubleshooting.md).

## Validation

Start required infrastructure before environment-dependent backend tests.

Release version consistency:

```powershell
npm run check:version
```

Backend tests:

```powershell
npm run test:backend
```

Frontend quality gates:

```powershell
npm run audit:frontend
npm --prefix frontend run lint
npm --prefix frontend run typecheck
npm run test:frontend
npm --prefix frontend run security:sinks
npm run build:frontend
```

The backend CI additionally enforces:

```powershell
cargo fmt --manifest-path backend/Cargo.toml --check
cargo clippy --manifest-path backend/Cargo.toml --all-targets --all-features -- -D warnings
```

See the [Command Catalog](okf/development/commands.md),
[Testing Workflow](okf/development/testing-workflow.md), and
[CI Architecture](okf/delivery/ci-architecture.md) for command provenance and
known limitations.

## Marketplace Creator Tooling

Validate and package the tracked Component Pack sample:

```powershell
npm run marketplace -- validate docs/marketplace-samples/component-pack
npm run marketplace -- pack docs/marketplace-samples/component-pack --force
```

The CLI can also submit a package to an authenticated Marketplace API. Submission
is an external mutation and requires an approved creator/listing context. See
the [Marketplace Creator Guide](docs/MARKETPLACE_CREATOR_GUIDE.md).

## V3 Release Readiness

Run the local/static Phase 15 readiness report:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/marketplace-phase15-ga-check.ps1 -ReportOnly -SkipFrontendBuild
```

The GitHub source release requires consistent `3.0.0` version metadata,
successful applicable CI checks, GPLv3 metadata, reviewed release notes, a clean
pushed `main` commit, and explicit owner approval. It publishes repository
source archives only; it does not publish container images, binaries, or a
hosted service.

A future production deployment additionally requires a safe target environment,
explicit authorization, organization and authentication context, reviewed beta
evidence, assigned support/release/rollback owners, healthy `/health` and
`/ready` responses, and resolution or approval of reported exceptions.

The repository contains production-like container build definitions, but it
does not define a production provider, deployment workflow, environment
promotion system, automatic backup/restore process, or application
metrics/alerting integration. Do not infer those guarantees from
`docker-compose.prod.yml`.

The production-like frontend requires exact `CSP_API_ORIGIN` and
`CSP_WEBSOCKET_ORIGIN` values in addition to the built `VITE_API_URL`. Keep
those values aligned with backend CORS and Preview WebSocket Origin
configuration. The Nginx entrypoint expands the tracked security-header
template; bypassing that entrypoint bypasses the configured production CSP.
See the [Phase 4 security report](docs/security/PHASE_04_CSP_TRUSTED_TYPES_RICH_TEXT_HARDENING.md).

## Marketplace Runtime Boundaries

- Uploaded Marketplace package code is never executed.
- Component Packs, Design Templates, and public Plugin Hooks use host-owned
  adapters and policy checks.
- Only reviewed, approved, compatible, and safe product versions are eligible
  for catalog/install flows.
- Automated payout transfers are not implemented.
- Partial Marketplace refunds are not supported.
- External critical-report notification delivery is deferred; persisted
  internal notifications and moderation queues remain authoritative.
- Runtime execution telemetry, warehouse export, and anomaly alerting are not
  implemented.

These boundaries are part of the release contract. See the
[Marketplace Scope](docs/V3_MARKETPLACE_SCOPE.md),
[Product Taxonomy](docs/V3_PRODUCT_TAXONOMY.md), and
[Marketplace Policy](docs/V3_MARKETPLACE_POLICY.md).

## Documentation

| Topic | Document |
| --- | --- |
| API routes and contracts | [API Guide](docs/API.md) |
| System architecture | [Architecture Guide](docs/ARCHITECTURE.md) |
| Localization and RTL | [Internationalization Guide](docs/I18N.md) |
| V3 scope | [Marketplace Scope](docs/V3_MARKETPLACE_SCOPE.md) |
| V3 domain model | [Marketplace Domain Model](docs/V3_MARKETPLACE_DOMAIN_MODEL.md) |
| Package manifest | [Manifest Schema](docs/V3_MARKETPLACE_MANIFEST_SCHEMA.md) |
| Package storage | [Package Storage](docs/V3_PACKAGE_STORAGE.md) |
| Creator workflow | [Marketplace Creator Guide](docs/MARKETPLACE_CREATOR_GUIDE.md) |
| Review and moderation | [Marketplace Policy](docs/V3_MARKETPLACE_POLICY.md) |
| GA criteria | [Phase 15](docs/V3_PHASE_FIFTEEN.md) |
| Release notes | [V3 Marketplace Release Notes](docs/V3_MARKETPLACE_RELEASE_NOTES.md) |
| Operations | [Marketplace Operations Runbook](docs/V3_MARKETPLACE_OPERATIONS_RUNBOOK.md) |
| Complete knowledge base | [Open Knowledge Format](okf/README.md) |
| Repository navigation | [OKF Navigation Guide](okf/project/navigation-guide.md) |
| Final documentation status | [OKF Completion Report](okf/maintenance/final-completion-report.md) |

Historical implementation records remain under `docs/`. Current source code,
configuration, migrations, and tests take precedence when historical phase
documents conflict with implementation.

## Repository Layout

| Path | Purpose |
| --- | --- |
| `backend/` | Rust API, services, middleware, migrations, tests, and container builds |
| `frontend/` | React administration application, tests, and frontend container build |
| `docs/` | Product, API, architecture, Marketplace, release, and runbook documents |
| `docs/diagrams/` | Evidence-linked Mermaid architecture diagrams |
| `docs/marketplace-samples/` | Creator tooling sample packages |
| `scripts/` | Marketplace CLI, smoke checks, and release-readiness scripts |
| `okf/` | Structured, indexed, evidence-based repository knowledge |
| `okf-bootstrap/` | Original OKF inventory and planning analysis |
| `.github/workflows/` | Backend and frontend CI definitions |

Generated and dependency directories such as `backend/target`,
`frontend/node_modules`, `frontend/dist`, and `marketplace-dist` are not source
of truth.

## License

ZinharCMS is licensed under the GNU General Public License version 3 only
(`GPL-3.0-only`). See [LICENSE](LICENSE) for the complete license terms.

## Security and Project Governance

- Never commit `.env` files, credentials, tokens, private keys, or production
  data.
- Review [Security Architecture](okf/security/README.md) and
  [Secrets and Configuration](okf/security/secrets-and-configuration.md) before
  changing authentication, authorization, tenant isolation, or secret handling.
- No repository-wide `SECURITY.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, or
  ownership policy is currently tracked.
- Do not infer a vulnerability disclosure channel until the project owner
  publishes a security policy.
