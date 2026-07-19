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
| `routes/auth.rs` | Register, login, refresh, logout, current user, cookie construction |
| `services/password.rs` | Argon2 password hash and verification |
| `services/jwt.rs` | Access-token signing/verification and opaque refresh-token generation/hash |
| `middleware/auth.rs` | Bearer extraction and claim insertion for authentication-only routes |
| `middleware/tenant.rs` | Bearer verification plus organization membership for tenant routes |
| `users`, `roles`, `user_roles` | Account and global-role persistence |
| `refresh_tokens` | Hashed refresh tokens, expiry, and revocation timestamp |
| `login_attempts` | Successful and failed login-attempt records |

## Authentication Factors and Identities

The implemented interactive factor is email plus password. Email is normalized to lowercase for registration and login; the database uses `CITEXT`. The repository has no verified MFA, federation, passkey, password-reset, email-verification, account-recovery, device-session, or service-account flow. Their product status is `AUTHENTICATION_FLOW_UNCLEAR AFU-01` and `NEEDS_OWNER_CONFIRMATION`.

## Access Token Contract

Access tokens are application-built compact JWTs using HS256 and one `JWT_SECRET`. Claims are `sub`, `role`, `iat`, and `exp`. Verification checks three token parts, recomputes the HMAC, deserializes claims, and rejects an expired token. No issuer, audience, JWT ID, key ID, nonce, or token-version claim is present.

The global role is captured at issuance. A role change or account deactivation is not reloaded on every access-token request; account activity is checked during login/refresh and `/me`, while generic middleware validates only the token. This creates a bounded stale-authorization window until token expiry.

## Refresh Credential Contract

Refresh tokens are opaque random values. Only their SHA-256 representation is persisted. The token is accepted from the request JSON body first and otherwise from the refresh cookie. A valid refresh revokes the prior database row before issuing another token pair.

## Router Placement

- Public: module status, registration, login, refresh.
- Bearer protected: logout and current user.
- Tenant protected: CMS and organization-scoped operations; tenant middleware also verifies the bearer token.
- Preview compatibility: access token and organization ID may be in the query string.

## Uncertainties

- `AUTHENTICATION_FLOW_UNCLEAR AFU-01`: missing account verification, recovery, MFA, and identity-provider policy cannot be interpreted as an intentional final design.
- `TOKEN_LIFECYCLE_UNCLEAR TLU-01`: no refresh-token family, reuse detection, all-session revocation, key rotation, or token binding was found.
- `DOCUMENTATION_CODE_CONFLICT DCC-09` remains an API documentation conflict unrelated to authentication; no new authentication documentation/code conflict was confirmed.
