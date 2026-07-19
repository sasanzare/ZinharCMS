---
okf_document_id: "architecture-integration-points"
title: "Integration Points"
project: "ZinharCMS"
category: "architecture"
phase: 2
status: "current"
review_status: "verified"
source_of_truth: false
architecture_status: "mixed"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-17"
primary_sources:
  - "backend/src/config.rs"
  - "backend/src/services/cache.rs"
  - "backend/src/services/email.rs"
  - "backend/src/services/stripe_billing.rs"
  - "backend/src/services/media_processing.rs"
  - "backend/src/routes/webhooks.rs"
  - "backend/src/services/webhooks.rs"
  - "backend/src/routes/billing.rs"
  - "backend/src/routes/media.rs"
  - "backend/src/services/marketplace_runtime.rs"
  - "backend/src/services/marketplace_adapters.rs"
  - "frontend/src/services/api.ts"
  - "frontend/nginx.conf"
  - "docker-compose.yml"
  - "docker-compose.prod.yml"
  - ".github/workflows"
related_documents:
  - "architecture/README.md"
  - "architecture/overview.md"
  - "architecture/boundaries.md"
  - "architecture/components.md"
  - "architecture/dependency-model.md"
  - "architecture/runtime-flows.md"
  - "architecture/architecture-risks.md"
related_diagrams:
  - "architecture/diagrams/system-context.mmd"
  - "architecture/diagrams/container-view.mmd"
uncertainty_markers:
  - "INFERRED_FROM_CONFIGURATION"
  - "UNKNOWN U-01"
  - "UNKNOWN U-02"
  - "UNKNOWN U-03"
  - "UNKNOWN U-04"
  - "UNKNOWN U-05"
  - "NEEDS_OWNER_CONFIRMATION NOC-02"
  - "NEEDS_OWNER_CONFIRMATION NOC-03"
  - "NEEDS_OWNER_CONFIRMATION NOC-04"
  - "PLANNED_NOT_IMPLEMENTED PNI-01"
---

# Integration Points

## Phase 6 Contract Cross-Reference

| Integration | API boundary | Contract documentation |
| --- | --- | --- |
| Stripe inbound | `POST /api/billing/stripe/webhook`, raw bytes and signature headers | [Billing Endpoints](../api/endpoints/billing-subscription-and-usage.md) |
| Stripe outbound | Billing checkout/portal and Marketplace commerce/payout operations | [Marketplace Commerce and Payouts](../api/endpoints/marketplace-commerce-and-payouts.md) |
| CMS webhook destinations | Tenant-managed `/api/webhooks` plus outbound delivery services | [CMS Webhook Endpoints](../api/endpoints/cms-webhooks.md) |
| Redis | Readiness, public-delivery cache, rate/quota/runtime support | [API Request Lifecycle](../api/route-architecture.md) |
| Filesystem/static files | Media and Marketplace multipart uploads; public `/uploads` mount | [Uploads, Downloads, and Streaming](../api/uploads-downloads-and-streaming.md) |
| Frontend administration app | 141 typed JSON request functions | [Frontend Contract Map](../api/frontend-contract-map.md) |
| External/public delivery consumers | Eight public `/api/v1/*` handlers | [Public Delivery Endpoints](../api/endpoints/public-delivery.md) |

Provider secrets, live account state, network policy, and deployed proxy behavior remain environment-dependent. Phase 6 documents repository-visible transport contracts, not live-provider verification.

## Integration Inventory

| Integration and scope | Purpose | Source and configuration | Mechanism and authentication | Failure behavior | Local status | Production status | Evidence and documentation status |
|---|---|---|---|---|---|---|---|
| PostgreSQL — internal infrastructure | Durable system of record, migrations, RLS | SQLx call sites, `backend/migrations`, `backend/src/config.rs`; `DATABASE_URL` | PostgreSQL protocol; connection string plus tenant session context on RLS-aware paths | Startup migration, readiness, or request failure depending on call | Verified in default Compose | Production-like Compose only; actual service/backup unknown | Primary code/config evidence; detailed reference deferred to Phase 5 |
| Redis — internal infrastructure | Delivery cache, tenant rate limiting, readiness | Cache/rate services and readiness code; `REDIS_URL` | Redis protocol; no separate app credential behavior documented beyond URL | Cache fail-open; rate-limit and readiness fail-closed | Verified in default Compose | Production-like Compose only; capacity/HA unknown | Primary code/config evidence; operations detail deferred to Phase 10 |
| Upload filesystem — internal storage | Media and Marketplace package bytes plus static reads | Media/Marketplace routes, config, static route; `UPLOAD_DIR` | Local filesystem I/O; application ownership checks on writes | Request failure and possible database/file partial success | Verified local path behavior | Production-like volume only; shared/durable/private topology unknown | Primary code/config evidence; NOC-02 and later data/security phases |
| React SPA contract — internal application boundary | Administration UI communication | `frontend/src/services/api.ts`, `types/api.ts`, store; API base configuration | HTTP/JSON, multipart, WebSocket; bearer JWT, organization header, browser credentials | Non-success response parsed as client error; no automatic refresh retry verified | Verified development application | Built frontend/Nginx reference only; actual routing unknown | Primary client/server evidence; DDU-03 and Phases 4/6 |
| Stripe API — external | Checkout, subscription, refund, and Marketplace finance operations | `backend/src/services/stripe_billing.rs`, billing/Marketplace routes, backend config | HTTPS to Stripe v1; bearer secret key | Provider/network error fails affected operation | Code path available when configured | Live account and secret wiring unknown | Repository-visible contracts are documented in Phase 6; live provider behavior remains unverified |
| Stripe webhook — external inbound | Apply provider billing events | Public billing route and backend config | HTTP callback; HMAC signature with webhook secret | Invalid signature rejected; handler/data errors return failure | Available when local webhook configuration exists | Live endpoint and delivery state unknown | Primary route evidence; DCC-09 protects path accuracy |
| Email webhook — external | Deliver application email through configured target | `backend/src/services/email.rs`; email mode/URL/strict config | HTTP/HTTPS JSON; no provider-specific authorization found | Log/disabled/webhook mode; strict failure can affect origin operation | Log/disabled behavior verified; webhook configuration-dependent | Provider, retries, deliverability, and ownership unknown | Primary service evidence; U-14/NOC-04 |
| CMS webhooks — external outbound | Notify customer-configured endpoints after selected events | `backend/src/routes/webhooks.rs`; `backend/src/services/webhooks.rs`; database webhook configuration | HTTP/HTTPS JSON with `X-CMS-Signature` HMAC | One spawned attempt with timeout and delivered/failed record; no durable retry | Implemented when subscription exists | Target/network operations and guarantee unknown | Primary route/service evidence; ABU-03/NOC-09 |
| Built-in plugin interface — internal | Run trusted content hooks | `backend/src/plugins`, content routes, plugin configuration/registry | Direct Rust trait/function calls; enabled registry and tenant request context | Shares backend request/process failure lifecycle | Implemented | Same deployment as backend; no isolation | Primary code evidence; detailed extension reference deferred to Phase 9 |
| Marketplace host adapters — internal extension boundary | Expose allowlisted CMS capabilities under Marketplace policy | Runtime/adapters services and Marketplace routes; manifest/installation policy | Direct Rust calls and HTTP route results; installation, entitlement, and host policy context | Validation or adapter error; uploaded package code is not executed | Implemented policy path | Same deployment as backend; future execution remains PNI-01 | Primary code plus explicit scope; Phase 9 |
| Nginx SPA server — internal deployment edge | Serve built UI assets and route fallback | `frontend/nginx.conf`; frontend production Dockerfile | Browser HTTP; no backend API proxy in tracked config | Static server failure prevents SPA load | Not required for Vite development | Production-like image only; ingress/TLS unknown | Primary config evidence; actual deployment U-01 |
| CI workflows — repository integration | Backend and frontend quality gates | `.github/workflows/backend-ci.yml`; `frontend-ci.yml` | GitHub Actions jobs and package/Cargo commands | Failed job blocks only according to external branch policy, which is unknown | Not a runtime dependency | No deployment workflow found | Primary workflow evidence; deployment automation remains unknown |

## PostgreSQL

PostgreSQL is the application system of record. Embedded migrations run during backend startup. Route and service modules use the SQLx pool directly or obtain RLS-scoped connections and transactions. Readiness actively checks the database. The production backup owner, recovery objective, high-availability arrangement, and managed-service topology are `UNKNOWN U-03` and require `NOC-03`.

## Redis

Redis supports multiple semantics:

- public delivery caching uses get-or-set behavior and falls back to PostgreSQL when cache access fails;
- cache invalidation is best effort, and prefix invalidation uses Redis `KEYS`;
- tenant rate limiting treats Redis failure as service unavailable;
- readiness requires a successful Redis check.

The different failure policies are implemented behavior, not one uniform Redis availability contract. Capacity, eviction, cluster topology, and production ownership are unknown.

## Filesystem Storage

Media routes create organization-scoped paths, write originals, and generate WebP variants. Marketplace package operations also write and read artifacts beneath the configured upload directory. Axum serves that directory through `/uploads`.

No S3-compatible object-store adapter or CDN integration was found. Shared-volume behavior, encryption, private-versus-public object policy, backup, malware scanning, retention, and multi-instance consistency remain `U-02`, `U-05`, and `NOC-02`.

## Frontend Contract

The SPA communicates through `frontend/src/services/api.ts`. The client adds bearer and organization headers from browser-managed state and uses credentials on requests. `frontend/src/types/api.ts` manually mirrors the expected contract. There is no generated SDK or compile-time shared schema.

The WebSocket preview URL carries access token and organization information in its query parameters. Operational logging and intermediary handling of those URLs must be considered in the security phase.

## Stripe

The Stripe adapter calls `https://api.stripe.com/v1` with the configured secret. Billing and Marketplace finance flows use it for checkout, subscription, and refund-related operations. The public Stripe webhook handler validates the provider signature using the configured webhook secret before applying events.

The production-like Compose file does not itself establish complete Stripe secret injection or a live Stripe environment. Provider configuration in an actual deployment remains inferred from configuration capability, not verified deployment evidence.

## Email

Email delivery supports log, disabled, and webhook modes. Delivery attempts are recorded in `email_deliveries`. Webhook mode posts JSON to the configured target; no provider-specific authorization header or durable queue was found. Strict behavior can make an email failure fail the originating application operation.

Provider identity, deliverability controls, retry policy, and operational ownership remain `UNKNOWN U-04` and `NOC-04`.

## CMS Webhooks

Customer-configured webhooks accept only HTTP or HTTPS schemes and reject obvious private-network destinations. Outbound payloads carry an HMAC signature. The webhook service dispatches in an in-process spawned task with a request timeout and records delivered or failed status.

The URL check is not a complete proof against every DNS rebinding or network-routing scenario. Security validation belongs to Phase 7. Delivery has no durable queue or automatic retry loop, and the post-mutation guarantee remains an owner decision.

## Built-In Plugins

Built-in plugins are Rust implementations registered inside the backend process. The SEO plugin is an observed implementation. Content handlers invoke hooks around selected content operations. This is an internal extension point, not process isolation.

## Marketplace Runtime and Host Adapters

Marketplace services validate package manifests, installation and entitlement state, and allowlisted host operations. Host adapters expose selected CMS capabilities under policy checks. The runtime reports execution as `not_executed`; the backend does not run arbitrary uploaded server-side package code. Any future execution sandbox is outside current architecture and remains `PNI-01`.

## Observability Boundary

The backend emits structured tracing to its configured subscriber and the container configuration exposes standard process logs. No verified metrics exporter, APM collector, distributed tracing backend, or automatic alert delivery integration was found. Production observability ownership remains unknown.

## Container Runtime and Delivery Automation

Dockerfiles and Compose files define development and production-like build/runtime shapes. GitHub Actions workflows run backend and frontend quality gates. No verified production deployment, registry publication, environment promotion, rollback, or container-orchestrator integration was found; these remain U-01, U-06, NOC-06, and ISU-03.

No external authentication provider, search service, object-store API, message broker, monitoring backend, or automatic alert-delivery provider is verified by current repository evidence.

## Security Integration Notes

No external identity provider, MFA provider, secret manager, security-event exporter, or deployed WAF is verified. Stripe signature verification, email/webhook delivery, Redis limiting, filesystem uploads, and Marketplace host-operation authorization cross trust boundaries documented in [Security Trust Boundaries](../security/trust-boundaries.md), [Secrets and Configuration](../security/secrets-and-configuration.md), and [Threat Register](../security/threat-register.md).

## Marketplace Integration Classification

Marketplace package upload, Stripe-facing commerce, filesystem artifacts, host adapters, and tenant component synchronization are verified integration points. No dynamic plugin loader or uploaded-code execution path was found. See [Marketplace Architecture](../extensibility/marketplace-architecture.md) and [Isolation and Trust](../extensibility/isolation-and-trust.md).

## Operational Integration Status

PostgreSQL, Redis, local files, Stripe, optional email webhook delivery, tenant CMS webhooks, Nginx, and build-time package registries are the verified operational dependencies. No object-storage provider, queue, search service, identity provider, container registry, metrics backend, alert manager, backup service, or deployment provider is verified. See [External Dependencies](../operations/external-dependencies.md), [Logging and Tracing](../operations/logging-and-tracing.md), and [Metrics and Monitoring](../operations/metrics-and-monitoring.md).
