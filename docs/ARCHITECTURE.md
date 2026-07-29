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
- Redis 7 for Delivery API cache, organization/user rate limits, and hash-keyed
  MFA pre-authentication, Step-Up, attempt-lock, and failure records.
- Local filesystem storage under `UPLOAD_DIR` for CMS media and Marketplace
  package artifacts.

The repository does not implement a separately deployed API gateway, S3-compatible
storage provider, CDN, durable queue, or background worker.

## Backend Boundaries

The root Axum router exposes four distinct boundaries:

1. Public system routes: `/`, `/health`, `/ready`, and `/openapi.json`.
2. Public integration/auth routes: registration, login, cookie-authenticated
   refresh/logout, ticket-authenticated preview WebSocket, Stripe webhook, and
   the Delivery API.
3. Authentication-only routes: current user, organization list/create/invitation
   acceptance, global plugin management, and product-level beta operations.
4. Tenant-aware routes: CMS management, pages, media, webhooks, organization
   workspace operations, billing, beta organization data, and all Marketplace
   routes.

Tenant-aware requests require a valid access token and `X-Organization-Id`.
`tenant_middleware` verifies the current active user, global role, and
authentication version, then verifies an active organization and active
membership, applies rate limits and API quota checks, and inserts
`TenantContext`. Database helpers set PostgreSQL RLS session variables before
tenant-owned queries.

## Identity And Authorization

Global roles and organization membership roles are separate:

- Global roles: `super_admin`, `admin`, `editor`, `author`, `viewer`.
- Organization roles: `owner`, `admin`, `editor`, `author`, `viewer`,
  `billing_manager`.

Frontend route guards and hidden controls are user-experience controls. Backend
middleware and handler/service role checks remain authoritative.

Access tokens are signed HS256 JWTs and are not stored as database entities.
The protected header contains an exact algorithm, type, and key identifier.
Verification selects one configured active or time-bounded previous key by
identifier and never tries every key. Tokens carry an authentication version,
and protected middleware performs one indexed authoritative identity lookup so
deactivation, sensitive identity changes, global-role changes, and logout-all
invalidate existing tokens. Global roles remain separate from organization
membership roles.

Password authentication establishes AAL1. An enabled-MFA account receives only
a short-lived Redis pre-authentication transaction after password success;
TOTP or a one-time recovery code completes AAL2 and normal session issuance.
Access tokens and logical refresh families carry session ID, AAL, AMR,
password-authentication time, optional MFA time, and auth version. Protected
middleware checks that claim context against the active logical family.

TOTP secrets are encrypted in PostgreSQL with a dedicated AES-256-GCM key ring,
fresh nonces, and user/enrollment/version associated data. Accepted time steps
are stored under a row lock to prevent replay. Recovery codes use a fast
SHA-256 lookup plus an Argon2id verifier and atomic used timestamp.

Selected sensitive mutations pass through central Step-Up policy. An AAL2
session obtains a short-lived Redis challenge, proves TOTP or recovery again,
and receives a one-time grant bound to user, session, auth version, and exact
scope. The grant is atomically consumed before the handler runs.

Refresh tokens are random values sent only as `HttpOnly`, `SameSite=Lax`
cookies scoped to `/api/auth`. Only hashes are stored. Each login creates a
token family with an absolute expiry. Rotation locks and updates the family in
one database transaction, creates exactly one linked successor, and marks the
predecessor rotated. Reuse of a rotated token revokes the complete family.
Logout revokes the current family rather than every family for the user.
Authenticated users can list non-expired logical families by opaque public ID,
revoke one owned family, or revoke every family. A privileged bulk revocation
path rechecks the caller's current database role and accepts only
`super_admin`. These mutations use a per-user PostgreSQL advisory transaction
lock so refresh rotation and bulk revocation serialize.

## Data And Tenant Isolation

The final schema is migration-authoritative through migration `0029`.

- Core identity: users with authentication versions, roles, user roles, refresh
  token families and hashed tokens, account-bound hashed security-token
  foundations, encrypted MFA enrollment state, hash-only recovery codes, global
  security audit events, and login attempts.
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
Marketplace installation tables. The application database connection must use
a non-superuser role without `BYPASSRLS`; tracked Compose initialization creates
that separate role for fresh volumes. Existing initialized volumes require an
operator-managed role migration because PostgreSQL init scripts do not rerun.

Global identity and Marketplace catalog/review entities use application
authorization instead. `organization_members` and `organization_invitations`
also use application authorization because membership must be read before
tenant session variables can be established. A global `super_admin` does not
automatically bypass tenant middleware; explicit bypass transactions are
limited to selected platform operations such as verified Stripe webhook
processing.

## Core CMS And Page Builder

The visual Page Builder is implemented, not future work. `PagesPage.tsx` provides
the component palette, drag-and-drop canvas, props editor, local preview, manual
save, and debounced autosave for persisted pages.

The backend validates page JSON against registered component keys, stores complete
page snapshots in `page_versions`, supports restore-to-new-draft behavior, and
publishes process-local WebSocket preview updates. Preview channels are in-memory,
so multiple backend replicas require an explicit shared-broadcast design.

### Browser Session Boundary

The refresh credential is an HttpOnly, SameSite=Lax cookie scoped to
`/api/auth`; production configuration sets Secure. The browser never persists
an access token. It starts in an `unknown` state, performs a cookie refresh, and
renders protected routes only after reaching `authenticated`. Failed bootstrap
clears cached user/organization projections and reaches `unauthenticated`.

One in-tab promise serializes refresh. Tabs use Web Locks as the primary
critical section and BroadcastChannel for transient session/logout delivery.
A bounded BroadcastChannel election is the fallback when Web Locks are absent;
if neither coordination primitive exists, refresh fails closed. Storage events
are not an authentication channel. The API attaches bearer credentials only to
the configured API origin and retries one request once only for the stable
`access_token_invalid` code.

Cookie-authenticated refresh/logout endpoints compare browser Origin with the
explicit CORS origin. Missing Origin is reserved for non-browser clients;
`null`, malformed, duplicate, or untrusted values are denied.

### Preview WebSocket Boundary

Tenant-authenticated preview readers request an ephemeral Redis ticket for one
page. The ticket contains 256 random bits, has a default 30-second and maximum
60-second lifetime, is rate-limited, and is stored only under a SHA-256-derived
key. Redis `GETDEL` makes consumption atomic and single-use. Redis failure denies
issuance and connection.

The browser opens `/api/preview/{page_id}` without query parameters and offers
`zinhar.preview.v1` plus `zinhar.ticket.<opaque-ticket>` in
`Sec-WebSocket-Protocol`. The server requires one exact allowed Origin, rejects
missing/unknown/duplicate protocols, consumes the ticket, validates its
audience/user/organization/page/version/time scope, and selects only
`zinhar.preview.v1`.

The handshake and each 30–60-second revalidation load current user activity,
authentication version, organization/member activity, preview permission, and
page access. A failed check closes the connection with a generic policy reason.
Frontend reconnects use bounded backoff and obtain a new ticket every time;
logout or definitive policy/protocol rejection stops reconnecting.

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
Tenant-configurable destinations pass through one reusable outbound client.
Each delivery reparses and resolves the destination, denies non-global address
candidates, pins the approved DNS result to the real connection, preserves the
hostname for HTTP and TLS, disables redirects and environment proxies, applies
connect and total timeouts, and limits the response body. Delivery attempts are
stored, but no durable retry queue or worker exists.

## Billing And SaaS Operations

Plans, organization subscriptions, quota counters, Stripe checkout/customer
portal, signed Stripe webhooks, idempotent event storage, and timestamp-based event
ordering are implemented for organization billing.

Audit logs and email-delivery records are persisted. Email supports `log`,
`disabled`, and generic HTTP `webhook` modes; no specific email vendor is built in.
SaaS alert definitions are seeded and listable, but there is no evaluator,
scheduler, or alert destination runtime.

Login rate-limit identity uses the socket peer by default.
`TRUSTED_PROXY_CIDRS` is empty unless explicitly configured. Only a trusted
immediate peer enables parsing of `Forwarded`, `X-Forwarded-For`, or
`X-Real-IP`; the chain is walked from the nearest hop and only configured
trusted proxies are removed.

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
against the existing version upload API. Phase 13 adds Marketplace security QA
contracts for IDOR, permission bypass, malicious package, refund abuse, and
review abuse paths, plus catalog/search/listing/install performance indexes,
private catalog cache headers, and a local latency baseline script. Phase 14
uses the existing beta participant/feedback/blocker model and Marketplace
analytics/lifecycle/finance/report APIs as a read-only evidence layer for
Private Creator Beta and Customer Beta readiness. Phase 15 packages those
existing controls into Launch Readiness and General Availability operations:
runbook, final policy, support workflow, rollback and incident checklist,
release notes, public docs, monitoring dashboard, and support plan. Uploaded
package code is not executed:
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

## Browser Content Security Boundary

Phase 4 establishes a two-layer rich-content boundary. The backend uses Ammonia
to sanitize declared entry and Page Builder rich text before storage and again
for historical, preview, version, template, and delivery paths. The frontend
uses DOMPurify to create a branded `SanitizedRichHtml` value; only
`SafeRichText` can pass that value to the single approved HTML sink. Flat Page
Builder schemas and legacy JSON Schema `properties` use one normalized property
classification.

The production frontend image serves a strict CSP from an Nginx entrypoint
template. Operators supply exact API and Preview WebSocket origins. Production
requires Trusted Types for script sinks and permits only the
`zinhar-rich-content` and `dompurify` policies. Vite development serves an
explicit development CSP; production preview serves the production policy for
local validation. The backend owns a separate deny-all CSP appropriate for JSON
responses.

Public content remains a headless JSON API; this repository does not include a
public HTML renderer. Downstream renderers remain responsible for their own
typed sink and CSP even though declared rich text is sanitized by the API.
Unsupported Page Builder custom style objects are cleared because the
repository has no safe CSS grammar or isolated style renderer.

## Detailed Evidence

The complete diagram set and source traceability are available in:

- `docs/diagrams/README.md`
- `docs/diagrams/ARCHITECTURE_AUDIT.md`
- `docs/diagrams/TRACEABILITY.md`
- `docs/diagrams/32-end-to-end-traceability.mmd`
- `docs/security/PHASE_04_CSP_TRUSTED_TYPES_RICH_TEXT_HARDENING.md`
