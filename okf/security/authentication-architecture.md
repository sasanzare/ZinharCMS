---
okf_document_id: "security-authentication-architecture"
title: "Authentication Architecture"
project: "ZinharCMS"
category: "security"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "partially_verified"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/routes/auth.rs"
  - "backend/src/middleware/auth.rs"
  - "backend/src/services/jwt.rs"
  - "backend/src/services/password.rs"
  - "backend/migrations/0001_initial_schema.sql"
related_documents:
  - "README.md"
  - "authentication-flows.md"
  - "session-and-token-lifecycle.md"
  - "../api/authentication.md"
related_diagrams:
  - "diagrams/authentication-flow.mmd"
---

# Authentication Architecture

## Components

| Component | Responsibility |
| --- | --- |
| `routes/auth.rs` | Register, login, refresh, logout, session inventory/revocation, current user, cookie construction |
| `services/password.rs` | Argon2 password hash and verification |
| `services/jwt.rs` | Key-identified access-token signing/verification and opaque refresh-token generation/hash |
| `services/sessions.rs` | Refresh-family inventory, rotation, individual/bulk revocation, and concurrency locks |
| `services/mfa.rs` | Standard TOTP profile and AES-256-GCM secret encryption |
| `services/mfa_accounts.rs` | Pending enrollment, replay state, recovery codes, disable, and key rotation |
| `services/mfa_challenges.rs` | Redis pre-authentication, Step-Up, rate limits, attempt locks, and one-time grants |
| `services/security_tokens.rs` | Internal purpose/user/binding-bound recovery-token foundation |
| `middleware/step_up.rs` | Central method/path-to-scope policy for sensitive mutations |
| `middleware/auth.rs` | Bearer extraction and claim insertion for authentication-only routes |
| `middleware/tenant.rs` | Bearer verification plus organization membership for tenant routes |
| `users`, `roles`, `user_roles` | Account and global-role persistence |
| `refresh_token_families`, `refresh_tokens` | Logical sessions, hashed refresh tokens, expiry, rotation, and revocation |
| `security_tokens` | Hashed single-use recovery and verification token foundation |
| `user_mfa`, `mfa_recovery_codes` | Encrypted TOTP state, accepted-step replay marker, and hash-only recovery credentials |
| `login_attempts` | Successful and failed login-attempt records |

## Authentication Factors and Identities

Email plus password establishes AAL1. Users may enroll a standard TOTP factor;
password plus TOTP or a one-time recovery code establishes AAL2. Selected
sensitive mutations additionally require a short-lived one-time Step-Up grant
bound to the current session and exact scope. Email is normalized to lowercase
for registration and login; the database uses `CITEXT`.

The repository still has no federation, passkey, public password-reset,
email-verification, administrative MFA-reset, or service-account flow. The
internal hash-only security-token foundation remains available for future
recovery and verification product flows.

## Access Token Contract

Access tokens are application-built compact JWTs using HS256 and a strict
`JWT_KEY_RING`. The header must contain exact `alg`, `typ`, and `kid` values;
verification selects one active or unexpired previous key by `kid` and rejects
unknown, retired, or legacy no-`kid` tokens. Claims are `sub`, `role`, `ver`,
`iat`, `exp`, `sid`, `aal`, `amr`, `auth_time`, and optional `mfa_time`.
Verification checks bounded structure, signature, time, lifetime, and
assurance semantics, then protected middleware reloads active-user, global-role,
authentication-version, and logical-session context.

Global role, active state, and `auth_version` are authoritative database state.
Role/security-sensitive identity changes invalidate prior access tokens through
the version check. Preview handshakes and open sockets apply the same freshness
boundary.

## Refresh Credential Contract

Refresh tokens are opaque random values. Only their SHA-256 representation is
persisted in transactional session families. Browser refresh/logout accept the
credential only from the `HttpOnly` cookie; request-body refresh tokens are
unsupported. Rotation is one-time, and reuse compromises/revokes the family.

## Router Placement

- Public/cookie boundary: module status, registration, login, MFA login
  completion, refresh, and logout.
- Bearer protected: current user, session inventory, owned revocation,
  logout-all, MFA enrollment/management, Step-Up, and super-admin bulk
  revocation.
- Tenant protected: CMS and organization-scoped operations; tenant middleware also verifies the bearer token.
- Preview handshake: exact Origin plus a short-lived one-time ticket; query
  parameters are rejected.

## Uncertainties

- `AUTHENTICATION_FLOW_UNCLEAR AFU-01`: TOTP MFA, recovery codes, and Step-Up
  are implemented; account verification, public password reset,
  administrative MFA recovery, and identity-provider flows remain undefined.
- `TOKEN_LIFECYCLE_UNCLEAR TLU-01`: refresh families, reuse detection,
  individual/all-session revocation, and bounded JWT key rotation are
  implemented. MFA-backed Step-Up now covers the selected sensitive-action
  matrix; stronger device binding and deployment rotation ownership remain open.
- `DOCUMENTATION_CODE_CONFLICT DCC-09` remains an API documentation conflict unrelated to authentication; no new authentication documentation/code conflict was confirmed.
