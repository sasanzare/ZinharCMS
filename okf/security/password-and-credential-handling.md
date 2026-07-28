---
okf_document_id: "security-password-credential-handling"
title: "Password and Credential Handling"
project: "ZinharCMS"
category: "security"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "mixed"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/services/password.rs"
  - "backend/src/routes/auth.rs"
  - "backend/src/main.rs"
  - "frontend/src/pages/AuthPage.tsx"
  - "backend/migrations/0001_initial_schema.sql"
related_documents:
  - "authentication-architecture.md"
  - "secrets-and-configuration.md"
  - "security-risks.md"
related_diagrams:
  - "diagrams/authentication-flow.mmd"
---

# Password and Credential Handling

## Password Storage

`hash_password` uses Argon2id version 19 with explicit parameters
(`m=19456 KiB`, `t=2`, `p=1`, 32-byte output), a fresh OS-random salt, and the
standard encoded hash. Verification parses the stored hash and uses the Argon2
verifier. Plaintext user passwords are not written to the user table by the
inspected route code. No password rehash-on-login policy was found.

## Password Policy

Registration enforces a minimum length of eight characters. Hashing and
verification reject passwords over 1,024 UTF-8 bytes and embedded NUL
characters before expensive processing. No breached-password screening,
composition rule, password history, expiration, password change, public
recovery route, or MFA policy was found. `INPUT_VALIDATION_UNCLEAR IVU-01` and
`NEEDS_OWNER_CONFIRMATION` still apply to the broader account-security policy.

## Other Credentials

- Access tokens are bearer credentials signed by the active key in `JWT_KEY_RING`.
- Refresh tokens are random bearer credentials; only hashes are stored server-side.
- Organization invitation tokens are random values stored as hashes and cleared
  after acceptance, revocation, or expiry.
- The internal recovery/verification foundation stores only hashes of random,
  purpose-bound, user-bound, optionally binding-bound, single-use tokens. No
  public password-reset or email-verification route is implemented.
- Stripe and provider secrets are configuration values, not user credentials.
- Webhook secret handling is documented separately from account passwords.

## Development Credential Finding

`POTENTIAL_SECRET_EXPOSURE PSE-01`: startup and login-page source contain deterministic development bootstrap credentials. This can create an unsafe deployment if production starts with an empty user database or the UI defaults are not removed. The credential value is intentionally not copied into OKF.

## Logging and Error Behavior

Login records email, IP, success state, and timestamp in `login_attempts`. Authentication errors do not echo the password. Generic internal-error conversion can expose technical text in other paths; see [Backend Error Handling](../backend/error-handling.md) and [Security Risks](security-risks.md).
