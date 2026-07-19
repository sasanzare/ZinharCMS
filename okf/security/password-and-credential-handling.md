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

`hash_password` uses `Argon2::default()` with a fresh OS-random salt and stores the encoded password hash. Verification parses the stored hash and uses the Argon2 verifier. Plaintext user passwords are not written to the user table by the inspected route code.

The repository does not pin explicit Argon2 memory, iteration, or parallelism parameters in application code, and no password rehash-on-login policy was found. The effective defaults therefore depend on the crate version locked by the build.

## Password Policy

Registration enforces only a minimum length of eight characters. No maximum length, breached-password screening, composition rule, password history, expiration, password change, recovery, or MFA policy was found. `INPUT_VALIDATION_UNCLEAR IVU-01` and `NEEDS_OWNER_CONFIRMATION` apply to the intended account-security policy.

## Other Credentials

- Access tokens are bearer credentials signed with `JWT_SECRET`.
- Refresh tokens are random bearer credentials; only hashes are stored server-side.
- Organization invitation tokens are random values stored as hashes.
- Stripe and provider secrets are configuration values, not user credentials.
- Webhook secret handling is documented separately from account passwords.

## Development Credential Finding

`POTENTIAL_SECRET_EXPOSURE PSE-01`: startup and login-page source contain deterministic development bootstrap credentials. This can create an unsafe deployment if production starts with an empty user database or the UI defaults are not removed. The credential value is intentionally not copied into OKF.

## Logging and Error Behavior

Login records email, IP, success state, and timestamp in `login_attempts`. Authentication errors do not echo the password. Generic internal-error conversion can expose technical text in other paths; see [Backend Error Handling](../backend/error-handling.md) and [Security Risks](security-risks.md).
