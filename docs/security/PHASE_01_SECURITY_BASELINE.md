# Phase 1 Security Baseline

Audit date: 2026-07-26  
Branch: `security/security-audit-fixes`  
Audited source revision: `64d780b6` plus the uncommitted Phase 1 changes listed below

This is a source-level security baseline and hardening pass. It is not a
penetration test, a deployed-environment assessment, or a claim that the
application is vulnerability-free or ready for production.

## Scope

The review covered the complete tracked repository and relevant ignored local
configuration:

- Rust/Axum backend code, route composition, middleware, services, tests, SQL,
  migrations, and Docker build;
- React/Vite frontend code, browser state, API client, build configuration,
  tests, and Docker/Nginx build;
- development and production Compose files, environment templates, CI
  workflows, scripts, dependency manifests and lockfiles;
- repository documentation, diagrams, the OKF route/security baseline, and Git
  history for known secret patterns.

The review was static except for local builds, unit/component tests, dependency
metadata checks, Compose rendering, and package-manager advisory queries. No
deployed environment, live payment account, live webhook receiver, live mail
service, or populated PostgreSQL tenant dataset was available.

## Repository State

- The required branch was already checked out and tracks
  `origin/security/security-audit-fixes`.
- Starting `HEAD` was `64d780b6` (`docs(release): define v3.0.0 source-only publication`).
- The working tree was clean at audit start.
- Existing work was preserved. No reset, clean, commit, push, dependency update,
  lockfile rewrite, migration rewrite, or destructive operation was performed.
- The repository and Git state were treated as authoritative where the earlier
  handoff described another branch or phase.
- The working tree is intentionally modified by the Phase 1 changes. See
  `HANDOFF.md` and `git status` for the current checkpoint.

## Architecture Summary

ZinharCMS is a three-part application:

1. A Rust 2024 backend using Axum, SQLx/PostgreSQL, Redis, Argon2, custom
   HMAC-SHA256 JWT access tokens, opaque refresh tokens, and local filesystem
   artifact storage.
2. A React 19/Vite browser application using a centralized API client and a
   Zustand session store.
3. PostgreSQL and Redis data services, with Docker/Compose definitions for local
   development and a source-level production topology.

The backend runs embedded migrations before binding, composes public,
bearer-only, and bearer-plus-tenant route zones, and applies timeouts, CORS,
request IDs, tracing, compression, and security headers. The verified OKF route
snapshot records 168 handler-method endpoints: 17 public, 12 bearer-only, and
139 tenant-protected. The static upload service is outside that count. The
Phase 1 changes do not add or remove handler-method endpoints.

## Trust Boundaries

| Boundary | Trusted side | Untrusted or less-trusted side | Primary controls |
| --- | --- | --- | --- |
| Browser to backend | Backend route and middleware stack | Browser input, local storage, extensions, XSS-capable content | TLS at deployment, CORS, bearer/refresh tokens, validation, security headers |
| Public API to protected API | Auth and tenant middleware | Anonymous callers | JWT verification, route composition |
| Global identity to tenant | Active organization and membership context | Caller-supplied organization identifier | Membership lookup, status checks, RBAC, RLS session context |
| Application to PostgreSQL | SQLx queries and migrations | Request-derived identifiers and filters | Bind parameters, allowlisted dynamic fragments, transactions, forced RLS |
| Application to Redis | Rate limit, quota, cache code | Shared mutable cache/rate state | Namespaced keys, bounded expiries, failure policy |
| Upload clients to filesystem | Media and Marketplace validators | Multipart files, filenames, ZIP metadata and bytes | Size/type/signature checks, generated paths, ZIP policy, checksum verification |
| Application to webhook targets | Outbound webhook dispatcher | Tenant-configured URL and remote responses | Scheme/literal-address validation, HMAC signing; DNS/redirect control is incomplete |
| Stripe to public callback | Signed Stripe event handler | Public request body and headers | HMAC verification, timestamp tolerance, event persistence/idempotency |
| Marketplace package to host | Host application and permission policy | Creator-supplied package metadata/artifact | Manifest/ZIP validation, checksum, review state, explicit permissions, kill switches |
| CI/build to dependencies | Repository workflows and lockfiles | Registries, action tags, image tags | Lockfiles, `npm ci`, lint/tests; several supply-chain controls are deferred |

## External Entry Points

The externally reachable source-defined surfaces are:

- system discovery and probes: `/`, `/health`, `/ready`, and `/openapi.json`;
- public authentication: registration, login, and refresh;
- public delivery: published content, pages, settings, navigation, sitemap, and
  robots responses;
- the signed Stripe webhook receiver;
- authenticated session, organization bootstrap/invitation, global beta
  administration, and built-in plugin routes;
- tenant-protected content, workflow, media, page builder/WebSocket preview,
  comments, billing, organization, webhook, and Marketplace routes;
- public static media below `/uploads/{organization_uuid}/...`.

The original catch-all upload mount also reached Marketplace package objects
stored under the same filesystem root. Phase 1 restricts the public static mount
to generated media paths and returns 404 for other storage namespaces.

The backend and Vite development server bind all interfaces when launched
directly. Development PostgreSQL, Redis, and pgAdmin Compose ports now bind only
to `127.0.0.1`.

## Authentication Surfaces

- Passwords use Argon2 with OS-generated salts.
- Access tokens are custom HS256 JWTs containing user ID, global role, issue
  time, and expiry. Phase 1 changes verification to HMAC's constant-time
  `verify_slice`.
- Refresh tokens are 32 random bytes, stored as SHA-256 hashes. The normal
  browser path uses an `HttpOnly`, `SameSite=Lax` cookie scoped to `/api/auth`;
  a legacy response/body path remains supported.
- Login throttling uses Redis and an IP selected from forwarding headers or the
  socket address. Forwarding-header trust is not constrained to configured
  proxies.
- Public registration now always creates the non-administrative author role.
- An administrator is provisioned only on an empty users table when both
  bootstrap environment variables are explicitly set and pass validation.
- The ignored local `.env` still contains a JWT placeholder. Its value was not
  copied into this report. The hardened backend will reject it until the
  operator replaces it.

## Authorization and Tenant Boundaries

The route tree separates bearer-only routes from tenant-protected routes.
Tenant middleware verifies the JWT, resolves `X-Organization-Id` (or the preview
query equivalent), loads an active organization and active membership, and
applies tenant rate/quota checks. Global and organization roles are distinct;
organization owner and global super-administrator overrides are implemented in
RBAC helpers and handler-specific checks.

Authorization is partly centralized in middleware and partly distributed
across handlers and service functions. High-impact call sites therefore require
both route-zone review and handler-level role/ownership review. The
authorization-agnostic RLS-bypass transaction helper is especially sensitive;
the reviewed call sites were preceded by administrator, ownership, or signed
provider checks, but live negative-path verification is still required.

## Database and RLS Boundaries

Tenant-owned tables use PostgreSQL row-level security policies and `FORCE ROW
LEVEL SECURITY` through the migration series. Tenant transactions set
`zinhar.organization_id` and `zinhar.user_id`; privileged operations can set
`zinhar.rls_bypass`. SQLx bind parameters are used for request values. The
reviewed dynamic SQL fragments were limited to static query fragments or
allowlisted sort/status choices; no confirmed SQL injection path was found.

Static contract tests passed for forced-RLS coverage and tenant-context policy
helpers. This phase did not start a populated PostgreSQL instance or execute a
cross-tenant positive/negative matrix, so runtime policy behavior, connection
context cleanup under failures, and every bypass call path remain unverified.

## File and Artifact Handling

Media upload handling bounds request size, validates supported MIME types and
file signatures, generates organization/UUID-based storage paths, and produces
image variants in a dedicated subdirectory. Original client filenames are
metadata rather than public path selectors.

Marketplace packages are stored under a separate namespace beneath the same
upload root. Upload and install paths enforce object-key shape, canonical path
containment, package size, SHA-256 checksum, ZIP central-directory limits,
manifest consistency, and review/install state. The reviewed code does not
extract or execute Marketplace package code.

Phase 1 closes the public static-route crossover between generated media and
Marketplace package objects. It does not introduce authenticated Marketplace
artifact download endpoints; that should be designed in a later Marketplace
hardening phase if customers require direct downloads.

## Webhook and Payment Surfaces

Tenant owners/admins can create CMS webhooks, choose event types, and supply a
destination URL and signing secret. Deliveries use an HMAC signature and record
attempt/response information. Literal localhost and private IP destinations are
rejected, but hostnames are not resolved and pinned before the request, and a
default Reqwest client may follow redirects. This leaves a confirmed SSRF path
for an authorized webhook manager to indirect or redirected private addresses.

The Stripe callback is public by design. It validates the timestamped signature
with constant-time HMAC verification, applies a 300-second tolerance, and uses
database event identity/order checks. The outbound Stripe API URL is fixed and
uses the configured provider credential. No live Stripe end-to-end test was
performed.

The operator-configured email webhook is a separate outbound channel. Its
deployment endpoint, authentication expectation, and network egress policy
could not be verified from a live environment.

## Marketplace Surfaces

The reviewed Marketplace flow separates creator submission, automated package
validation, global review/publication, tenant purchase/entitlement,
installation/update/rollback, declared runtime permissions, kill switches,
finance, feedback, moderation, and analytics.

Package bytes are treated as data. The current runtime and adapter layers
authorize declared host operations but do not load or execute arbitrary package
code. Permission changes require re-approval, and blocked/inactive
installations fail runtime authorization. Residual risk concentrates in the
large distributed authorization surface, external payment state, artifact
delivery design, and future changes that might introduce executable extension
code.

## Frontend Security Boundaries

React escaping is used and no `dangerouslySetInnerHTML` use was found. The
custom rich-text sanitizer removes selected dangerous blocks and attributes,
but it is not a full browser parser or established sanitization library.

Access tokens, legacy refresh tokens, user data, and active organization state
are persisted in `localStorage`. Any same-origin script execution or sufficiently
privileged browser extension can read those values. Route guards check local
session state for navigation convenience; backend middleware remains the
security boundary.

Preview WebSocket compatibility permits access tokens in query parameters,
which can leak through browser history, telemetry, reverse-proxy logs, or copied
URLs. Phase 1 removed deterministic login form defaults and replaced the
security-sensitive `Math.random` webhook-secret fallback with Web Crypto or
backend generation.

## Infrastructure and CI Surfaces

The production backend image runs as an unprivileged user. PostgreSQL and Redis
have no production host port mapping, while backend and frontend ports remain
published for an external ingress layer. TLS termination, HSTS at the edge,
firewall rules, secrets injection, backup encryption, image admission, and
runtime security options are deployment responsibilities not demonstrated by
this repository.

Production Compose now defaults `COOKIE_SECURE` to true and accepts the optional
bootstrap pair. Development data-service ports are loopback-only. The Compose
files render successfully; the development file still declares an obsolete
Compose `version` field, which is an operational warning rather than a security
failure.

Backend CI runs formatting, Clippy with warnings denied, and tests. Frontend CI
runs install, audit, lint, typecheck, tests, and build. Missing controls include
Rust advisory/license scanning, repository secret scanning, dedicated static
security analysis, action commit-SHA pinning, container scanning/signing, and a
live migration/RLS test matrix.

## Dependency and Tooling Baseline

| Check | Result |
| --- | --- |
| Rust toolchain | `cargo 1.96.0`, `rustc 1.96.0` |
| Rust locked metadata | `cargo metadata --locked --offline --no-deps` passed |
| Rust advisory status | Not established: `cargo-audit` and `cargo-deny` were not installed |
| Frontend production dependency tree | `npm ls --omit=dev --depth=0` passed |
| Frontend production audit | 0 reported vulnerabilities |
| Frontend full lockfile audit | 0 reported vulnerabilities |
| Frontend runtime | Node `v24.17.0`, npm `11.17.0` |
| Additional scanners | `cargo-geiger`, `actionlint`, `gitleaks`, `trufflehog`, and `semgrep` were unavailable |
| Targeted source scans | No Rust `unsafe` blocks, process execution, dangerous React HTML API, or disabled TLS verification pattern found |
| Selected provider-token/private-key patterns | No match in the current tracked tree or Git history |
| Legacy deterministic bootstrap markers | Removed from runtime/UI/docs; one negative config test retains a rejected placeholder marker; five historical commits match |

Package-manager audit output is time- and registry-dependent. The results above
describe this lockfile and the checks run on 2026-07-26, not future dependency
state.

## Confirmed Findings

| ID | Severity | Confidence | Finding and impact | Affected files / evidence | Status |
| --- | --- | --- | --- | --- | --- |
| SEC-P01-001 | Critical | Confirmed | Empty-database startup created a deterministic privileged account, and the first public registration could receive global super-administrator. An unauthenticated caller could take over a fresh or reset installation. | `backend/src/main.rs`, `backend/src/routes/auth.rs`, `backend/src/services/rbac.rs`, `frontend/src/pages/AuthPage.tsx`, environment/docs history | Mitigated in the working tree. Existing deployments must independently identify, disable/rotate, and investigate any account created by the old path. Git history is unchanged. |
| SEC-P01-002 | High | Confirmed | Tenant webhook URL validation blocks literal private targets but does not pin DNS resolution or revalidate redirects, allowing an authorized webhook manager to cause server-side requests to internal destinations. | `backend/src/services/webhooks.rs`, `backend/src/routes/webhooks.rs` | Deferred to a dedicated outbound-request/SSRF phase. |
| SEC-P01-003 | High | Confirmed | Browser access tokens and legacy refresh-token state are persisted in `localStorage`, so same-origin script execution can exfiltrate them. | `frontend/src/services/api.ts`, `frontend/src/stores/useAppStore.ts` | Deferred to authentication/frontend hardening. |
| SEC-P01-004 | Medium | Confirmed | The public `ServeDir` covered the entire shared upload root, including Marketplace package objects. A discovered artifact URL bypassed entitlement checks. | `backend/src/routes/mod.rs`, `backend/src/services/marketplace_package.rs` | Fixed: only generated media path shapes reach the static service; router tests cover allow/deny behavior. |
| SEC-P01-005 | Medium | Confirmed | Login throttling trusts the first forwarding-header address without a configured trusted-proxy boundary, allowing direct clients or misconfigured proxies to spoof limiter identity. | `backend/src/routes/auth.rs`, `backend/src/services/security.rs` | Deferred to proxy-aware rate-limit hardening. |
| SEC-P01-006 | Medium | Confirmed | The production Compose service did not pass `COOKIE_SECURE`, while application configuration defaults it to false for local HTTP. Production refresh cookies could therefore be emitted without `Secure`. | `backend/src/config.rs`, `docker-compose.prod.yml` | Fixed for the tracked production Compose path by defaulting it to true; non-Compose deployments must set it explicitly. |
| SEC-P01-007 | Medium | Confirmed | Access-token global role claims remain authoritative until expiry; deactivation/role changes do not invalidate already-issued tokens at verification time. | `backend/src/services/jwt.rs`, auth middleware and global-role handlers | Deferred to token-version/revocation design. |
| SEC-P01-008 | Medium | Confirmed | Refresh rotation selects and revokes in separate database operations without row locking or a token-family reuse response. Concurrent reuse can issue multiple successor sessions. | `backend/src/routes/auth.rs`, refresh-token migration/model | Deferred to transactional refresh-family hardening. |
| SEC-P01-009 | Medium | Confirmed | Page-preview compatibility accepts bearer tokens and organization context in the WebSocket query string, increasing exposure through URL logs/history/telemetry. | `backend/src/middleware/tenant.rs`, `backend/src/routes/pages.rs`, frontend page-builder code | Deferred to short-lived preview tickets or header/cookie authentication. |
| SEC-P01-010 | Low | Confirmed | Generic internal errors and readiness dependency failures returned raw technical details to clients. | `backend/src/error.rs`, `backend/src/routes/mod.rs` | Fixed with generic client messages; internal details remain available to server-side logging/callers. |
| SEC-P01-011 | Low | Confirmed | A known tracked JWT placeholder met the previous length-only validation and could be reused unchanged. | `backend/src/config.rs`, environment templates, backend CI | Fixed with placeholder rejection, explicit templates, and tests. |
| SEC-P01-012 | Low | Confirmed | Development PostgreSQL, Redis, and pgAdmin ports were published on all host interfaces with development credentials. | `docker-compose.yml` | Fixed by binding all three mappings to `127.0.0.1`. |
| SEC-P01-013 | Low | Confirmed | The frontend webhook-secret helper fell back to `Math.random`, which is not cryptographically secure. | `frontend/src/pages/SettingsPage.tsx` | Fixed with Web Crypto or empty input so the backend OS CSPRNG generates the secret. |
| SEC-P01-014 | Low | Confirmed | Custom JWT verification compared encoded signatures with ordinary string equality. | `backend/src/services/jwt.rs` | Fixed with HMAC constant-time verification and a tampering regression test. |
| SEC-P01-015 | Medium | Confirmed | CI lacks Rust advisory, secret, SAST, container, and live tenant-isolation gates, leaving important regressions and dependency risk outside automated enforcement. | `.github/workflows/backend-ci.yml`, `.github/workflows/frontend-ci.yml` | Deferred; frontend npm audit is already present. |
| SEC-P01-016 | Low | Confirmed | Generated OpenAPI omits 19 of 168 handlers and declares neither bearer security nor the tenant header, so generated consumers cannot infer security requirements. Runtime middleware is not bypassed. | `okf/api/openapi-consistency.md`, `backend/src/routes/mod.rs` | Deferred to API contract hardening. |

## Unverified Risks

| ID | Potential severity | Confidence | Risk and missing evidence | Required verification |
| --- | --- | --- | --- | --- |
| SEC-P01-017 | High | Unverified | Cross-tenant isolation and every RLS-bypass call path were reviewed statically but not executed against representative tenant data. | Run migrations on PostgreSQL and a positive/negative matrix for each tenant table, role, connection reuse path, and bypass caller. |
| SEC-P01-018 | High | Unverified | Rust dependency advisories could not be assessed because no Rust advisory scanner was installed and network-backed installation was outside this phase. | Add a pinned `cargo-audit` or `cargo-deny` CI job and triage the locked graph. |
| SEC-P01-019 | Medium | Unverified | TLS termination, HSTS, firewall/egress policy, real secret injection, backups, log redaction, and runtime container restrictions are not demonstrated by source configuration. | Review the actual deployment/ingress/cloud configuration and rendered runtime environment without exporting secret values. |
| SEC-P01-020 | Medium | Unverified | The operator-configured email webhook's authentication, timeout, destination trust, and egress behavior were not exercised end to end. | Test against a controlled receiver and define an outbound network/authentication contract. |
| SEC-P01-021 | Medium | Unverified | The custom rich-text sanitizer passed focused tests, but broad browser parser differentials and mutation-based XSS cases were not tested. | Add a maintained sanitizer or a browser-based malicious corpus and CSP validation. |

No real committed secret was confirmed by the available pattern/history scans.
That statement is limited by scanner availability and cannot exclude every
credential format.

## Deferred Areas

- SSRF-safe webhook delivery: resolver policy, redirect disabling or
  revalidation, IP-range enforcement after resolution, egress allowlisting,
  bounded timeouts/body capture, and DNS-rebinding tests.
- Session redesign: transactional refresh rotation, token families/reuse
  detection, role/session versioning, revocation, and reduced browser-readable
  token storage.
- Live database migration and tenant/RLS authorization testing.
- Authenticated/entitled Marketplace artifact delivery and storage separation.
- Proxy trust configuration and rate-limit identity.
- OpenAPI security declarations and route/OpenAPI parity automation.
- CSP/Trusted Types and rich-text/browser XSS testing.
- CI supply-chain controls, action/image pinning, Rust advisory scans, secret
  scans, SAST, container scans/signing, and deployment policy.
- Actual ingress, TLS, secret manager, monitoring, backup, and incident-response
  configuration.

## Changes Applied

- Removed deterministic privileged seeding and first-registration privilege
  escalation; added explicit validated bootstrap configuration.
- Removed deterministic login form values and added a frontend regression test.
- Rejected tracked-style secret placeholders and removed `Debug` from the
  secret-bearing configuration object.
- Changed custom JWT verification to constant-time HMAC verification.
- Hid internal/readiness error details from public responses.
- Restricted public static uploads to generated media paths and added policy
  plus router-level tests.
- Replaced the frontend non-cryptographic webhook-secret fallback.
- Bound local Compose data/admin services to loopback.
- Defaulted production Compose refresh cookies to `Secure` and passed optional
  bootstrap values.
- Updated active setup documentation, historical phase notes, diagrams,
  environment templates, CI test configuration, `HANDOFF.md`, and the external
  lessons log without recording secret values.

## Validation Results

Passed:

- `cargo fmt --manifest-path backend/Cargo.toml -- --check`
- `cargo clippy --manifest-path backend/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo test --manifest-path backend/Cargo.toml --all-features` — 124 passed
- focused config, registration-role, JWT-tampering, error-redaction, upload-path,
  upload-router, and AuthPage tests
- `npm --prefix frontend run lint`
- `npm --prefix frontend run typecheck`
- `npm --prefix frontend test` — 15 passed
- `npm --prefix frontend run build` — passed with the existing large-chunk warning
- production and development npm advisory scans — 0 reported vulnerabilities
- `cargo metadata --manifest-path backend/Cargo.toml --locked --offline --no-deps --format-version 1`
- development and production `docker compose ... config --quiet`
- rendered local port check — PostgreSQL, Redis, and pgAdmin resolve to
  `127.0.0.1`
- `git diff --check`
- sanitized current-tree and history secret-pattern checks

Not run:

- Rust advisory/license scan: `cargo-audit`/`cargo-deny` unavailable.
- Dedicated secret/SAST/action/container scanners: unavailable.
- Live PostgreSQL migrations, RLS/tenant matrix, Redis integration, browser E2E,
  payment, webhook, email, and deployed-edge tests: required services and
  deployment context were not started in this source-only phase.
- Root npm audit: no root npm lockfile; frontend audits cover the npm project.

An initial focused Rust run found a missing `StatusCode` import introduced by
the upload middleware change. It was corrected before the final matrix, and all
final Rust checks passed.

## Recommended Next Phase

First, treat SEC-P01-001 as an owner-side credential incident check: inventory
every deployed environment, disable or rotate any administrator created by the
old deterministic bootstrap path, review relevant audit/login history, replace
local/deployed placeholders, and decide whether Git-history rewriting is
warranted. Do not place replacement credentials in repository history.

Then execute Phase 2 as a combined outbound-request and session-security phase:
close SEC-P01-002 with an SSRF-safe HTTP client and negative tests; make refresh
rotation transactional with token-family reuse detection; define trusted-proxy
handling; and run a live RLS/tenant authorization matrix before expanding the
Marketplace or plugin runtime.
