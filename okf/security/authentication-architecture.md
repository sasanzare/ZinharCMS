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
| `services/security_tokens.rs` | Internal purpose/user/binding-bound recovery-token foundation |
| `middleware/auth.rs` | Bearer extraction and claim insertion for authentication-only routes |
| `middleware/tenant.rs` | Bearer verification plus organization membership for tenant routes |
| `users`, `roles`, `user_roles` | Account and global-role persistence |
| `refresh_token_families`, `refresh_tokens` | Logical sessions, hashed refresh tokens, expiry, rotation, and revocation |
| `security_tokens` | Hashed single-use recovery and verification token foundation |
| `login_attempts` | Successful and failed login-attempt records |

## Authentication Factors and Identities

The implemented interactive factor is email plus password. Email is normalized
to lowercase for registration and login; the database uses `CITEXT`. Logical
device/session families can be listed and revoked, but the repository still has
no verified MFA, federation, passkey, public password-reset,
email-verification, account-recovery, or service-account flow. An internal
hashed, single-use token foundation exists for future recovery and verification
flows. Product behavior remains `AUTHENTICATION_FLOW_UNCLEAR AFU-01` and
`NEEDS_OWNER_CONFIRMATION`.

## Access Token Contract

Access tokens are application-built compact JWTs using HS256 and a strict
`JWT_KEY_RING`. The header must contain exact `alg`, `typ`, and `kid` values;
verification selects one active or unexpired previous key by `kid` and rejects
unknown, retired, or legacy no-`kid` tokens. Claims are `sub`, `role`, `ver`,
`iat`, and `exp`. Verification checks bounded structure, signature, time, and
lifetime, then protected middleware reloads active-user, global-role, and
authentication-version state.

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

- Public/cookie boundary: module status, registration, login, refresh, logout.
- Bearer protected: current user, session inventory, owned revocation,
  logout-all, and super-admin bulk revocation.
- Tenant protected: CMS and organization-scoped operations; tenant middleware also verifies the bearer token.
- Preview handshake: exact Origin plus a short-lived one-time ticket; query
  parameters are rejected.

## Uncertainties

- `AUTHENTICATION_FLOW_UNCLEAR AFU-01`: the token foundation does not make
  account verification, password reset, recovery, MFA, or identity-provider
  product flows implemented.
- `TOKEN_LIFECYCLE_UNCLEAR TLU-01`: refresh families, reuse detection,
  individual/all-session revocation, and bounded JWT key rotation are
  implemented. Recent reauthentication, stronger device binding, and deployment
  rotation ownership remain open.
- `DOCUMENTATION_CODE_CONFLICT DCC-09` remains an API documentation conflict unrelated to authentication; no new authentication documentation/code conflict was confirmed.
