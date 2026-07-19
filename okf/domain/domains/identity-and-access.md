---
okf_document_id: "domain-identity-access"
title: "Identity and Access Domain"
project: "ZinharCMS"
category: "domain-detail"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
domain_id: "DOM-IDENTITY"
domain_name: "Identity and Access"
domain_status: "IMPLEMENTED"
boundary_status: "OVERLAPPING"
primary_sources:
  - "backend/src/routes/auth.rs"
  - "backend/src/middleware/auth.rs"
  - "backend/src/services/jwt.rs"
  - "backend/src/services/password.rs"
  - "backend/migrations/0001_initial_schema.sql"
related_documents:
  - "../domain-catalog.md"
  - "../../security/authentication-architecture.md"
  - "../../security/rbac-model.md"
related_diagrams: []
---

# Identity and Access Domain

## Domain Identity

- Domain ID: `DOM-IDENTITY`
- Terminology: user, credential, access token, refresh token, global role, claims, session.
- Implementation: `IMPLEMENTED`; boundary `OVERLAPPING`; confidence High.

## Responsibility

- Verified: register/bootstrap users, authenticate credentials, issue/refresh/revoke tokens, expose current user, and load global role claims.
- Inferred: the authenticated user is the root actor reused by every other domain.
- Shared: organization membership turns identity into tenant access; global RBAC governs plugins/Marketplace administration.
- Unclear: account verification, recovery, MFA, session-family policy, and production bootstrap intent.

## Core Entities

`users`, `roles`, `user_roles`, `refresh_tokens`, and `login_attempts`.

## Core Services

Auth route, authentication middleware, JWT, password, security, and RBAC services.

## API Surface

See [Authentication Endpoints](../../api/endpoints/authentication-and-session.md): registration/login, refresh, logout, and current user.

## Frontend Surface

`AuthPage`, `RequireAuth`, `useAppStore`, API token/cookie helpers, AppShell logout/profile state.

## Actors

Unauthenticated visitor, authenticated user, inactive user, and global role holders loaded from persistence.

## Business Rules

`BR-IDENTITY-001` through `BR-IDENTITY-003` in the [Business Rule Catalog](../business-rule-catalog.md).

## Invariants

Case-insensitive unique email, unique refresh-token hash, and valid user/role assignments. See `INV-IDENTITY-001` through `INV-IDENTITY-003`.

## State and Lifecycle

User activation is boolean. Refresh tokens have expiry and optional revocation time. Access tokens expire by claim. The frontend has explicit login/logout/refresh calls but no complete automatic expiry/retry state machine.

## Access Rules

Public authentication routes establish identity. Protected routes require valid bearer claims. Global role does not replace active organization membership. See [Authentication Architecture](../../security/authentication-architecture.md), [RBAC](../../security/rbac-model.md), and [Tenant Access](../../security/tenant-access-control.md).

## Validation Rules

Credential/email/password validation occurs in auth/password services and request handlers. Passwords are Argon2-hashed. Refresh tokens are opaque and hashed before persistence.

## Workflows

Session workflows are documented in Phase 7. Identity is a precondition for every workflow in [Cross-Module Workflows](../cross-module-workflows.md).

## Side Effects

User/token writes, login-attempt records, refresh cookie changes, token issuance, and selected security logs. No background session cleanup was found.

## Tests

Password/JWT/security helpers and authentication contracts have focused coverage, but recovery, replay, concurrent refresh, inactive-token revocation, and browser end-to-end tests are gaps.

## Risks and Unknowns

Token-family/reuse behavior, key rotation, deterministic development bootstrap credentials, frontend session expiry, and active-token behavior after user disable remain documented Phase 7 risks.

Return to the [Domain Catalog](../domain-catalog.md).

