# Architecture

ZinharCMS is an API-first headless CMS implemented as a modular monolith. The
React administration application is deployed separately from one Rust/Axum
backend process, but backend route and service modules are logical boundaries,
not independently deployed microservices.

## Runtime Containers

- React 19/Vite administration application. The production image serves the SPA
  through Nginx.
- Rust/Axum backend. It composes public, authenticated, and tenant-aware routes in
  one process.
- PostgreSQL 16 primary database accessed through SQLx.
- Redis 7 for Delivery API cache and organization/user rate-limit counters.
- Local filesystem storage under `UPLOAD_DIR` for CMS media and Marketplace
  package artifacts.

The repository does not implement a separately deployed API gateway, S3-compatible
storage provider, CDN, durable queue, or background worker.

## Backend Boundaries

The root Axum router exposes four distinct boundaries:

1. Public system routes: `/`, `/health`, `/ready`, and `/openapi.json`.
2. Public integration/auth routes: registration, login, refresh, Stripe webhook,
   and the Delivery API.
3. Authentication-only routes: current user, organization list/create/invitation
   acceptance, global plugin management, and product-level beta operations.
4. Tenant-aware routes: CMS management, pages, media, webhooks, organization
   workspace operations, billing, beta organization data, and all Marketplace
   routes.

Tenant-aware requests require a valid access token and `X-Organization-Id`.
`tenant_middleware` verifies an active organization and active membership, applies
rate limits and API quota checks, and inserts `TenantContext`. Database helpers set
PostgreSQL RLS session variables before tenant-owned queries.

## Identity And Authorization

Global roles and organization membership roles are separate:

- Global roles: `super_admin`, `admin`, `editor`, `author`, `viewer`.
- Organization roles: `owner`, `admin`, `editor`, `author`, `viewer`,
  `billing_manager`.

Frontend route guards and hidden controls are user-experience controls. Backend
middleware and handler/service role checks remain authoritative.

Access tokens are signed JWTs and are not stored as database entities. Refresh
tokens are random values stored as hashes in `refresh_tokens` and sent to browsers
as `HttpOnly`, `SameSite=Lax` cookies scoped to `/api/auth`.

## Data And Tenant Isolation

The final schema is migration-authoritative through migration `0025`.

- Core identity: users, roles, user roles, refresh tokens, login attempts.
- Core CMS: content types, entries, pages, page versions, components, media,
  settings, navigation, comments, plugins, and webhooks.
- Organizations: memberships, invitations, domains, rate limits, subscriptions,
  usage counters, audit logs, email deliveries, alert definitions, beta feedback,
  and GA blockers.
- Marketplace: creators, listings, versions/package metadata, submissions, review
  events, tenant-owned installation records, purchases, entitlements, revenue
  ledger entries, payout accounts, payout records, customer reviews, abuse
  reports, and critical-report internal notifications.

Forced PostgreSQL RLS protects tenant-owned CMS, billing, operations, beta, and
Marketplace installation tables. Global identity and Marketplace catalog/review
entities use application authorization instead. A global `super_admin` does not
automatically bypass tenant middleware; explicit bypass transactions are limited
to selected platform operations such as Stripe webhook processing.

## Core CMS And Page Builder

The visual Page Builder is implemented, not future work. `PagesPage.tsx` provides
the component palette, drag-and-drop canvas, props editor, local preview, manual
save, and debounced autosave for persisted pages.

The backend validates page JSON against registered component keys, stores complete
page snapshots in `page_versions`, supports restore-to-new-draft behavior, and
publishes process-local WebSocket preview updates. Preview channels are in-memory,
so multiple backend replicas require an explicit shared-broadcast design.

Entries and pages share workflow states but route actions have distinct side
effects. Publishing may invalidate Redis cache, run built-in plugin hooks, and
dispatch signed webhooks after the primary database mutation.

## Delivery, Media, And Webhooks

The public Delivery API reads published content from the active organization whose
slug is `default`. Redis values use a 300-second TTL; Redis cache failures fall back
to PostgreSQL. Rate-limit Redis failures do not use that fallback.

Media metadata is tenant-owned, while file bytes are served by the public
`/uploads` static route when a URL is known. Image uploads generate WebP variants.
Filesystem and relational writes are not one atomic transaction, so partial media
or artifact cleanup remains an operational decision.

CMS webhooks use HMAC-SHA256 signatures and transient `tokio::spawn` dispatch.
Delivery attempts are stored, but no durable retry queue or worker exists.

## Billing And SaaS Operations

Plans, organization subscriptions, quota counters, Stripe checkout/customer
portal, signed Stripe webhooks, idempotent event storage, and timestamp-based event
ordering are implemented for organization billing.

Audit logs and email-delivery records are persisted. Email supports `log`,
`disabled`, and generic HTTP `webhook` modes; no specific email vendor is built in.
SaaS alert definitions are seeded and listable, but there is no evaluator,
scheduler, or alert destination runtime.

GA readiness is represented by documentation, static Rust tests, and
`scripts/v2-ga-check.ps1`; it is not a runtime product service.

## Marketplace

Implemented Marketplace behavior includes creator requests and verification,
listing metadata/submission, package upload to local storage, manifest/static/
security/compatibility validation, persisted reports, global-admin review and
moderation, and a tenant-aware compatible catalog.

The catalog is product-facing but not anonymous: every `/api/marketplace/*` route
is currently mounted behind tenant middleware. Phase 6 implements organization-
owned install, enable, disable, soft-uninstall, pinned update, and safe rollback
state transitions for free Component Packs and Design Templates. Phase 7 adds the
permission catalog, allowlisted sandbox host API decisions, runtime status
blocking, and global/organization kill switches. Phase 8 connects safe manifest
declarations to the organization component registry, template import pipeline,
and public plugin-hook contracts. Phase 9 adds separate Marketplace purchases,
paid entitlements, auditable revenue splits/refund reversals, and payout account
verification. Phase 10 adds ownership-gated customer ratings/reviews, global-admin
review moderation, abuse-report intake and an actionable moderation queue, plus a
persisted internal notification for every critical report. Phase 11 adds
creator-owned product analytics and global-admin Marketplace health analytics
over persisted install, purchase, revenue, review, report, package, submission,
and review-event records. Phase 12 adds a creator-side CLI, creator guide, and
sample packages for local manifest validation, packaging, and upload submission
against the existing version upload API. Uploaded package code is not executed:
installation and adapter authorization remain host-owned policy state protected
by compatibility, permission approval, artifact integrity, audit, and forced-RLS
gates. Partial-refund workflows, automated payout transfers, external
notification delivery, runtime error telemetry, and subscription-style
Marketplace add-ons remain planned only.

## Observability And Recovery

- `TraceLayer` and formatted `tracing` output provide process-local request logs.
- request IDs are generated and propagated as `x-request-id`.
- `/health` reports liveness; `/ready` checks PostgreSQL and Redis.
- startup migration or seed failure prevents the listener from binding.
- Ctrl+C and Unix SIGTERM trigger Axum graceful shutdown.

No monitoring vendor, metrics exporter, durable retry worker, automatic backup,
TLS termination, or public reverse proxy is configured by this repository.
Operational gaps and owner decisions are recorded in
`docs/diagrams/AMBIGUITIES.md`.

## Detailed Evidence

The complete diagram set and source traceability are available in:

- `docs/diagrams/README.md`
- `docs/diagrams/ARCHITECTURE_AUDIT.md`
- `docs/diagrams/TRACEABILITY.md`
- `docs/diagrams/32-end-to-end-traceability.mmd`
