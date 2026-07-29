---
okf_document_id: "security-secrets-configuration"
title: "Secrets and Security Configuration"
project: "ZinharCMS"
category: "security-configuration"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "mixed"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/config.rs"
  - ".gitignore"
  - ".env.example"
  - "env.example"
  - "docker-compose.yml"
  - "docker-compose.prod.yml"
related_documents:
  - "password-and-credential-handling.md"
  - "browser-and-http-security.md"
  - "../backend/configuration-and-state.md"
related_diagrams:
  - "diagrams/trust-boundaries.mmd"
---

# Secrets and Security Configuration

## Configuration Sources

The backend optionally loads a local dotenv file and then reads environment variables. Active `.env` and `.env.*` files are ignored except the tracked `.env.example`; `env.example` is also tracked. Frontend API configuration is build-time `VITE_API_URL` with a local default.

## Security-Relevant Variables

| Category | Variable names | Observed behavior |
| --- | --- | --- |
| Persistence | `DATABASE_URL`, `REDIS_URL` | Database is required; Redis has a local default |
| Token | `JWT_KEY_RING`, `JWT_ACCESS_EXPIRY`, `JWT_REFRESH_EXPIRY` | JSON key ring required; exactly one active HS256 key; unique `kid`; every secret at least 32 bytes; lifetimes configurable |
| MFA | `MFA_ENCRYPTION_KEY_RING`, `MFA_ISSUER`, `MFA_ENROLLMENT_TTL_SECONDS`, `MFA_PRE_AUTH_TTL_SECONDS`, `MFA_STEP_UP_TTL_SECONDS`, `MFA_RATE_LIMIT_MAX_ATTEMPTS` | Separate AES-256-GCM key ring required; bounded active/previous lifecycle and MFA timing/attempt policy |
| Browser | `CORS_ORIGIN`, `COOKIE_SECURE` | Single CORS origin; cookie Secure flag configurable |
| Login limiting | `LOGIN_RATE_LIMIT_MAX_FAILURES`, `LOGIN_RATE_LIMIT_WINDOW_SECONDS` | Configurable failed-attempt threshold/window |
| Tenant limiting | `ORG_RATE_LIMIT_PER_MINUTE`, `ORG_USER_RATE_LIMIT_PER_MINUTE`, `ORG_RATE_LIMIT_BURST` | Redis-backed organization/user limits |
| Files | `UPLOAD_DIR`, `MAX_UPLOAD_SIZE` | Storage path and request/upload limit inputs |
| Billing/provider | `STRIPE_SECRET_KEY`, `STRIPE_WEBHOOK_SECRET`, price and redirect variables | Optional provider credentials and URLs |
| Email | `EMAIL_PROVIDER`, `EMAIL_FROM`, `EMAIL_WEBHOOK_URL`, `EMAIL_FAILURE_MODE` | Provider mode, sender, optional webhook, failure policy |
| Retention | `SECURITY_CLEANUP_BATCH_SIZE`, `EXPIRED_SESSION_RETENTION_DAYS`, `REVOKED_SESSION_RETENTION_DAYS`, `COMPROMISED_SESSION_RETENTION_DAYS`, `SECURITY_TOKEN_RETENTION_DAYS`, `SECURITY_AUDIT_RETENTION_DAYS`, `LOGIN_ATTEMPT_RETENTION_DAYS` | Bounded cleanup batch and minimum-validated retention windows |
| Runtime | `APP_BASE_URL`, `PORT`, `RUST_LOG`, `VITE_API_URL` | URL, listening, logging, and client API base |

No secret values are included in this document.

## Validation and Failure

The backend rejects a missing or malformed key ring, duplicate/invalid key
identifiers, unsupported algorithms, weak or placeholder key material, more
than one active key, an unbounded previous key, and invalid typed values.
Previous keys must have a verification cutoff bounded by the configured access
token lifetime. Several high-impact settings have development-oriented
defaults. Optional provider credentials can disable or change provider behavior
depending on the calling service.

The MFA encryption ring is independent from JWT signing keys. It requires
exactly one active 32-byte AES-256-GCM key. Previous decrypt-only keys require a
future cutoff no more than seven days away; retired keys cannot decrypt.
Duplicate key identifiers/material, placeholders, wrong length, unsupported
algorithm, or ambiguous active status prevent startup.

## Unverified Secret Lifecycle

`SECRET_HANDLING_UNVERIFIED SHU-01`: the repository now supports a bounded
active/previous JWT rotation window, strict key identifiers, and payload
redaction for stored invitation deliveries. Phase 6 adds an equivalent bounded
MFA encryption-key lifecycle and lazy re-encryption. No repository evidence proves a
production secret manager, encryption at rest, access policy, rotation
ownership, incident revocation, or comprehensive log redaction. Tracked
examples and ignored local files are hygiene controls, not production secret
governance.

## Exposure Finding

`POTENTIAL_SECRET_EXPOSURE PSE-01` covers deterministic development bootstrap credentials in source and UI defaults. It does not assert that a production secret is committed; tracked environment templates were reviewed by key name without reproducing values.

## Operational Secret Injection

Production-like Compose requires selected variables and passes other names/defaults, but no secret manager, encrypted configuration store, CI secret reference, access policy, rotation, revocation, backup, or recovery process is tracked. `VITE_API_URL` is a public build input and must never contain secrets. See [Environment Configuration](../operations/environment-configuration.md), [Environment Promotion](../delivery/environment-promotion.md), and [Operational Risks](../operations/operational-risks.md).
