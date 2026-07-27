# Phase 2 Outbound, Session, Proxy, and RLS Hardening

## Scope

Phase 2 implements the security work explicitly deferred by the Phase 1
repository audit:

- resolution-aware SSRF protection for tenant-configurable CMS webhooks;
- transactional refresh-token families with rotation and reuse response;
- authoritative access-token freshness after user or global-role changes;
- an explicit trusted-proxy boundary for login rate-limit identity;
- live PostgreSQL RLS, tenant-isolation, context-cleanup, and migration evidence;
- removal of the unused browser-readable refresh-token compatibility path.

This is source and disposable-local-environment evidence. It is not a
penetration test, a deployment review, or evidence about production
configuration.

## Starting Repository State

Work started on `security/security-audit-fixes` from clean commit
`eaf90c43 fix(security): complete phase 1 audit and baseline hardening`. Phase 1
was committed before Phase 2 began. Phase 2 changes remain uncommitted, and no
branch switch, push, history rewrite, staging deployment, or production access
was performed.

The source tree, migrations through `0026`, Phase 1 report, current Git state,
and application call sites were revalidated before implementation. The owner
response required by `SEC-P01-001` remains outside source-code authority.

## Inherited Findings

| ID | Previous rating | Phase 2 disposition |
| --- | --- | --- |
| `SEC-P01-002` | High, Confirmed | Closed in source by the pinned, no-redirect outbound webhook transport and dispatch-time validation. |
| `SEC-P01-005` | Medium, Confirmed | Closed in source by empty-by-default trusted-proxy CIDRs and peer-aware header parsing. |
| `SEC-P01-007` | Medium, Confirmed | Closed in source by access-token authentication versions and current database identity checks. |
| `SEC-P01-008` | Medium, Confirmed | Closed in source by locked transactional token-family rotation and reuse response. |
| `SEC-P01-017` | High, Unverified | Reclassified and closed for the tested source/migration boundary after a passing live non-superuser PostgreSQL matrix. Deployment-specific RLS behavior remains an operational verification requirement. |

`SEC-P01-001` remains an unresolved owner action. `SEC-P01-003` is only
partially mitigated: browser-readable refresh-token storage was removed, but
the access token is still stored in `localStorage`.

## Threat Model

The outbound policy assumes an authenticated tenant webhook administrator may
control a destination URL and may operate DNS or redirect responses. It denies
loopback, private, link-local, multicast, unspecified, reserved, documentation,
benchmark, metadata, and otherwise non-global addresses, including
IPv4-mapped IPv6. It also treats mixed or empty DNS answers as failures.

The session design assumes a refresh token can be copied and replayed,
including concurrently. A successfully rotated predecessor is a one-time
credential; replay is treated as evidence that the family may be compromised.
Raw refresh tokens are browser-cookie values and are never persisted.

The proxy design assumes every forwarding header is attacker-controlled unless
the immediate socket peer is explicitly configured as trusted.

The RLS design assumes application queries can omit an organization predicate.
The database role must therefore be a non-superuser without `BYPASSRLS`; the
tracked application previously violated this assumption in local Compose.

## Outbound Request Inventory

| Path | Destination control | Current classification and control |
| --- | --- | --- |
| `backend/src/services/webhooks.rs` | Tenant webhook administrators store the complete destination URL. | Security-sensitive. Uses the reusable `OutboundHttpClient`, dispatch-time URL and DNS validation, pinned resolved addresses, no redirects or environment proxy, bounded timeouts, and a bounded response body. |
| `backend/src/services/email.rs` | Operator configuration through `EMAIL_WEBHOOK_URL`. | Not tenant-controlled. It still uses a general `reqwest::Client`; the operator must restrict the configured receiver and egress. End-to-end authentication, timeout, and destination behavior remains inherited risk `SEC-P01-020`. |
| `backend/src/services/stripe_billing.rs` | The origin is fixed in source to Stripe's API; application values select provider operations and identifiers, not an arbitrary origin. | Provider client, not a tenant URL fetcher. It is not routed through the tenant webhook SSRF policy. Network egress and timeout behavior remain deployment concerns. |

Inbound Stripe webhooks and database, Redis, and filesystem operations are not
server-side outbound HTTP destinations.

## SSRF Protection Design

`backend/src/services/outbound_http.rs` owns the tenant webhook transport and is
created once in `AppState`.

The design:

- parses with `reqwest::Url` and allows only `http` and `https`;
- rejects missing hosts and embedded usernames or passwords;
- normalizes DNS hostnames to lower case without a trailing dot;
- validates literal IPv4 and IPv6 destinations before dispatch;
- resolves hostnames for each dispatch and rejects the complete result if any
  candidate is forbidden or if resolution is empty or fails;
- supplies the validated address set directly to reqwest's resolver used for
  the real connection, preserving the URL hostname for `Host` and TLS SNI;
- disables redirects and environment proxy discovery;
- disables idle connection reuse for this client so every dispatch is resolved
  and revalidated;
- uses HTTP/1, a 3-second connection timeout, and a 10-second total timeout;
- reads at most 64 KiB from the response, including chunked responses;
- exposes only generic blocked, failed, or oversized-response errors.

Webhook HMAC-SHA256 signatures and the existing event/header contract are
preserved. Creation-time URL checks are retained as early feedback, but
dispatch-time validation is authoritative.

The `ipnet` `2.12` dependency is declared directly because runtime
configuration now needs maintained, typed IPv4 and IPv6 CIDR parsing for the
trusted-proxy boundary. The same version was already present transitively in
the lockfile.

## Session and Refresh-Token Design

Migration `0027_security_phase_two_sessions.sql` adds:

- `users.auth_version`, constrained to a positive value;
- `refresh_token_families` with user, absolute expiry, revoke, compromise, and
  creation timestamps;
- family, predecessor, successor, and rotation fields on `refresh_tokens`;
- foreign keys, uniqueness constraints, and active-family lookup indexes;
- database triggers that increment authentication versions after global-role
  changes and sensitive user changes.

Login and registration create one family and one random refresh token. Only the
cryptographic token hash is stored. The raw value is issued after the database
transaction succeeds and only in the `zinhar_refresh_token` HttpOnly,
`SameSite=Lax`, `/api/auth` cookie.

Refresh hashes the presented cookie, locks the token and family state with
`FOR UPDATE`, verifies the current active user and global role, inserts exactly
one successor, links both rows, marks the predecessor rotated, and commits
before issuing a new cookie. A successor cannot extend the family's original
absolute expiry.

Logout revokes the one family identified by the current cookie. It is not a
global logout and does not revoke unrelated families for the same user. The
cookie is cleared even when no valid family can be revoked.

## Token Reuse Response

Reusing a token that already has `rotated_at` or a successor is treated as
family compromise. The rotation transaction marks the family compromised and
revoked and revokes every token in that family. A successor from that family is
then rejected.

Concurrent rotation is serialized by row locking. Exactly one request can
create the successor. The other observes the rotated predecessor, receives a
generic unauthorized result, and triggers the family compromise response.

Unknown, expired, revoked, inactive-user, reused, and compromised credentials
share a generic unauthorized public outcome. Successor insertion failure rolls
back the predecessor mutation, so a database error cannot leave a partially
rotated family.

## Access-Token Revocation Design

Access-token claims now include `ver`. Both authentication-only and
tenant-aware middleware verify the signature and then load the authoritative
active user, current global role, and current authentication version from
PostgreSQL. The claim is accepted only when all three match.

Database triggers increment `auth_version` for:

- activation or deactivation;
- password-hash changes;
- email changes;
- global role insert, update, or delete.

Reactivation increments the version again, so it cannot restore an obsolete
token. Current global role is distinct from organization membership role;
tenant middleware still performs the existing active organization and active
membership checks and stores the organization role in `TenantContext`.

The expected performance effect is one indexed user/global-role database query
for each protected request. This gives immediate source-level freshness but
adds database latency and availability coupling to authentication.

## Trusted Proxy Policy

`TRUSTED_PROXY_CIDRS` accepts a comma-separated list of IPv4 and IPv6 CIDRs.
The default is empty, and malformed values fail configuration startup.

The socket peer is always the initial client identity. Forwarding headers are
considered only if that peer is inside a configured trusted CIDR. Header
precedence is:

1. `Forwarded`;
2. `X-Forwarded-For`;
3. `X-Real-IP`.

For a valid chain, parsing proceeds from the nearest hop toward the original
client, removes only explicitly trusted proxy addresses, and selects the first
remaining untrusted address. IPv4, bracketed IPv6, quoted values, and optional
ports are normalized to an `IpAddr` string. A malformed selected header falls
back to the socket peer rather than accepting another attacker-controlled
identity.

Operators must configure only proxies that actually sanitize and replace the
selected forwarding header. An empty setting is correct for direct
deployments.

## RLS and Tenant-Isolation Inventory

The live catalog reported:

- migration version: `27`;
- tenant-owned tables with an `organization_id` column: 34;
- RLS-enabled tables: 32;
- forced-RLS tables: 32;
- policies: 118;
- policy operations: 32 `INSERT`, 25 `DELETE`, 32 `SELECT`, and 29 `UPDATE`.

The two tenant-keyed tables without RLS are
`organization_invitations` and `organization_members`. They are intentional
identity/control-plane tables: application middleware must read membership
before it can establish tenant session variables. Their application predicates
and authorization checks therefore remain security-critical.

All 32 RLS-enabled tables were also forced. Catalog queries inspected policy
operations and the current migrations define their use of
`zinhar.organization_id`, `zinhar.user_id`, and `zinhar.rls_bypass`.

The application contains 17 `begin_bypass_transaction` call sites:

- beta global operations require an administrative global role;
- Marketplace catalog/read paths are authenticated and constrain results to
  published or current-tenant data;
- moderation, reporting, and global analytics paths require global admin;
- creator analytics and finance paths perform creator ownership checks before
  reading protected domain data;
- runtime global kill-switch paths require global admin;
- Stripe processing verifies the provider signature before bypass.

No call site was found that used bypassed tenant data before its corresponding
global-admin, provider-signature, or ownership boundary.

## Live RLS Test Matrix

The matrix used a uniquely named Docker Compose project, PostgreSQL 16 Alpine,
tmpfs database storage, generated test-only credentials, and a dedicated
non-superuser application role with `NOBYPASSRLS`. No normal developer volume,
remote database, or real user/provider data was used.

Fresh migrations and representative fixtures created two organizations, two
owners, one normal member, tenant content, public settings, and webhooks. The
20 recorded assertions covered:

- own-tenant visibility and cross-tenant read denial for both organizations;
- cross-tenant update, delete, and insert denial;
- allowed same-tenant insert;
- missing and invalid organization context;
- current-user context and normal bypass=false state;
- pool reuse after commit;
- failed-transaction rollback cleanup;
- bypass success where explicitly intended;
- bypass cleanup after commit and rollback;
- forced-RLS and non-superuser catalog assertions.

Result: 20 of 20 matrix cases passed after the database-role correction. All
32 RLS-enabled tables were forced and all 118 policies were inventoried. The
test is representative CRUD plus complete catalog inventory; it does not claim
that every handler or every possible row state was dynamically exercised.

The live session suite also passed three database-backed tests for family
lifecycle and concurrency, invalid session states, and access-claim freshness.
The migration upgrade test applied migrations through `0026`, inserted a
legacy session, applied `0027`, verified its explicit revoked-family backfill,
and removed the uniquely named upgrade database.

## Migration Strategy

Migration `0027` is forward-only and preserves users and refresh-token audit
rows. Existing refresh tokens cannot be safely assigned a shared family from
the legacy schema. Each legacy token is therefore placed in its own family and
both the token and family are explicitly revoked. This intentionally signs out
existing browser sessions instead of ambiguously treating an old token as a
new family root.

Fresh and upgrade paths were executed on disposable PostgreSQL. New indexes
support hash lookup, family lookup, active-family cleanup, and user-family
revocation. Raw token values are not introduced by the migration.

## Confirmed Phase 2 Findings

### SEC-P02-001

- **Severity:** High
- **Confidence:** Confirmed
- **Type:** Confirmed configuration vulnerability
- **Affected files:** previous `docker-compose.yml`,
  `docker-compose.prod.yml`; runtime consequence in PostgreSQL RLS
- **Supporting evidence:** the first live matrix connected with the Compose
  bootstrap role. PostgreSQL reported it as a superuser, and cross-tenant rows
  were visible despite forced RLS. PostgreSQL superusers bypass RLS.
- **Realistic impact:** if the backend connects as that role, an omitted or
  defective application tenant predicate can expose or modify another
  organization's data despite the RLS policies.
- **Remediation status:** fixed in tracked Compose source for fresh volumes.
  `docker/postgres-init-app-user.sh` creates or alters a dedicated
  `NOSUPERUSER NOBYPASSRLS` application role, makes it database/schema owner,
  and removes public schema creation. The backend connection remains the
  application role. Existing initialized volumes require explicit owner
  migration or safe recreation because PostgreSQL init scripts do not rerun.
- **Regression-test evidence:** the complete live matrix asserts
  `rolsuper = false`; after the correction it passed 20 of 20 cases with
  32 of 32 RLS tables forced.

No additional Critical, Medium, Low, or Informational Phase 2 vulnerability
finding was confirmed. Operational limitations and deferred inherited risks
are listed separately rather than inflated into vulnerability findings.

## Phase 1 Findings Closed

- `SEC-P01-002`: closed by dispatch-time resolution validation, actual
  connection pinning, redirect disablement, no environment proxy, timeouts,
  bounded response reading, generic errors, and deterministic regression tests.
- `SEC-P01-005`: closed by the empty-by-default trusted CIDR boundary,
  socket-peer authority, safe chain selection, and spoofing regressions.
- `SEC-P01-007`: closed by token authentication versions, database-triggered
  invalidation, authoritative active-state/global-role checks, and live tests.
- `SEC-P01-008`: closed by locked atomic family rotation, exactly-one-successor
  constraints, reuse compromise response, rollback, and concurrent tests.
- `SEC-P01-017`: reclassified from High/Unverified and closed for the executed
  source/migration boundary. The live non-superuser matrix found the Compose
  role vulnerability, it was corrected, and the full matrix then passed.

These closures do not close `SEC-P01-001`, the access-token portion of
`SEC-P01-003`, or deployment-specific risks.

## Changes Applied

Created:

- `backend/migrations/0027_security_phase_two_sessions.sql`;
- `backend/src/services/outbound_http.rs`;
- `backend/src/services/sessions.rs`;
- `backend/tests/security_phase2_rls.rs`;
- `backend/tests/docker-compose.phase2.yml`;
- `docker/postgres-init-app-user.sh`;
- `frontend/src/services/api.test.ts`;
- this Phase 2 report.

Modified application areas include configuration and state construction, auth
and tenant middleware, auth routes, JWT/session/security/webhook services,
Compose and environment templates, frontend API/session consumers, API and
architecture documentation, and `HANDOFF.md`.

The frontend removes the obsolete `zinhar.refresh_token` key at startup,
stops accepting or returning refresh tokens in JSON, and calls refresh/logout
with the HttpOnly cookie only. The access-token `localStorage` path is
deliberately unchanged in this phase.

## Validation Results

Passed:

- `cargo fmt --manifest-path backend/Cargo.toml -- --check`;
- `cargo clippy --manifest-path backend/Cargo.toml --all-targets --all-features -- -D warnings`;
- `cargo test --manifest-path backend/Cargo.toml --all-features`: 150 backend
  unit tests passed, plus the conditionally configured integration-test
  harness and documentation targets;
- 12 deterministic outbound-client tests, including IP/DNS denial, pinned
  connection use, redirects, response limit, timeout, dispatch revalidation,
  and error redaction;
- 3 webhook service tests;
- trusted-proxy, forwarding-chain, normalization, and invalid-CIDR tests;
- JWT authentication-version and signature tests;
- auth JSON/cookie/logout contract tests;
- 2 frontend API compatibility regressions;
- a separately configured disposable-PostgreSQL run of 3 database-backed
  session/access tests;
- a separately configured live RLS/migration upgrade run with 20 of 20 matrix
  cases passed;
- `npm --prefix frontend run lint`;
- `npm --prefix frontend run typecheck`;
- `npm --prefix frontend test`: 5 files and 17 tests passed on the final run;
- `npm --prefix frontend run build`; Vite emitted only the existing large-chunk
  advisory;
- local and production Compose configuration rendering with placeholder-only
  process environment values;
- `git diff --check`;
- exact Phase 2 report-heading comparison;
- changed-source and Markdown scan with no Persian characters found;
- sensitive-pattern review: the only connection-pattern matches were explicit
  placeholder templates in the two environment examples;
- final Docker project-label inspection: no Phase 2 container, network, or
  volume remained.

## Failed or Unavailable Checks

Expected test-first failures occurred before implementation: new Rust tests
initially did not compile against missing outbound, trusted-proxy, session, and
access-version APIs, and the two frontend compatibility regressions initially
showed the legacy refresh key and request body. These passed after the
implementation.

The first live RLS execution failed because the application database role was a
Compose-created PostgreSQL superuser and could read cross-tenant data. This
failure produced `SEC-P02-001`; the role boundary was corrected and the complete
matrix was rerun successfully.

The first final frontend suite run had one pre-existing Marketplace test exceed
its five-second timeout while other tests were running concurrently. The same
test passed in a focused rerun, and the final full suite passed all 17 tests.
TypeScript validation also rejected a newly added partial `Response` test
double; the intentional test-double boundary was corrected and both typecheck
and the focused API tests passed.

Rust dependency advisory scanning was not run because no pinned advisory tool
is installed. No production ingress, firewall, TLS, secret injection, backup,
or live log configuration was available or authorized. No real email, Stripe,
webhook, metadata, arbitrary public-network, staging, or production request was
sent.

## Compatibility Impact

- Migration `0027` intentionally revokes every pre-Phase-2 refresh session;
  users must authenticate again after deployment.
- Access tokens issued before `ver` existed are rejected and users must
  authenticate again.
- Refresh and logout no longer accept `refresh_token` JSON and authentication
  responses no longer return it. Current repository consumers use the cookie.
- Each protected request now performs an indexed authoritative identity query.
- `TRUSTED_PROXY_CIDRS` defaults to empty. Deployments behind a reverse proxy
  must explicitly set the real trusted proxy CIDRs or login throttling will use
  the proxy peer address.
- Production Compose now requires separate bootstrap and application database
  credentials. Existing volumes do not execute the new init script
  automatically and require an owner-managed role migration or safe recreation.
- Webhook redirects are no longer followed and response bodies over 64 KiB are
  recorded as failed deliveries.

## Residual Risks

- `SEC-P01-001` requires deployment inventory, account response, credential
  rotation, and log review by the owner.
- Browser access tokens remain in `localStorage`; `SEC-P01-003` is not closed.
- Preview WebSocket authentication still supports query-string access tokens.
- The operator-configured email webhook and fixed Stripe provider client are
  not governed by the tenant webhook client's complete timeout/egress policy.
- `organization_members` and `organization_invitations` intentionally rely on
  application authorization rather than RLS.
- Database-backed auth freshness adds a database dependency to every protected
  request; load behavior was not benchmarked.
- Existing deployment database roles, actual ingress headers, egress policy,
  TLS, HSTS, secrets, backups, logs, and container restrictions were not
  inspected.
- The live matrix is representative rather than exhaustive over every handler,
  role transition, policy expression, and tenant table row state.

## Deferred Areas

The following remain for later phases:

- browser access-token storage redesign and CSRF-aware cookie/session design;
- short-lived preview WebSocket tickets or another query-token replacement;
- OpenAPI security schemes and complete handler coverage;
- rich-text sanitizer/browser mutation testing;
- pinned Rust and JavaScript dependency advisory enforcement in CI;
- operator email webhook authentication and egress contract;
- deployment, ingress, TLS, firewall, backup, logging, and runtime-container
  hardening;
- broader authorization and bypass-handler integration coverage.

## Recommended Next Phase

First complete the owner actions for `SEC-P01-001` and migrate every existing
deployment to a verified non-superuser, `NOBYPASSRLS` application database role.

Then begin Phase 3 as a dedicated browser authentication phase: remove the
access token from browser-readable persistent storage, replace preview
query-string tokens, define the CSRF/session boundary, and update frontend,
WebSocket, and API security contracts with regression tests. Do not combine
that work with unrelated Marketplace expansion.
