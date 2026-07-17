---
okf_document_id: "backend-module-authentication"
title: "Authentication"
project: "ZinharCMS"
category: "backend-module"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
module_id: "BE-MOD-002"
module_name: "Authentication"
module_paths:
  - "backend/src/routes/auth.rs"
  - "backend/src/middleware/auth.rs"
  - "backend/src/services/jwt.rs"
  - "backend/src/services/password.rs"
  - "backend/src/services/security.rs"
  - "backend/src/models/user.rs"
module_type: "Domain and HTTP module"
boundary_status: "OBSERVED"
primary_sources:
  - "backend/src/routes/auth.rs"
  - "backend/src/middleware/auth.rs"
  - "backend/src/services/jwt.rs"
  - "backend/src/services/password.rs"
  - "backend/src/services/security.rs"
  - "backend/src/models/user.rs"
related_documents:
  - "backend/README.md"
  - "backend/module-catalog.md"
  - "backend/module-boundaries.md"
  - "backend/dependency-map.md"
  - "backend/testing-map.md"
  - "backend/backend-risks.md"
  - "architecture/components.md"
  - "architecture/boundaries.md"
  - "architecture/dependency-model.md"
related_diagrams:
  - "backend/diagrams/backend-module-map.mmd"
  - "backend/diagrams/backend-dependency-flow.mmd"
uncertainty_markers:
  - "MODULE_OWNERSHIP_UNCLEAR MOU-01"
  - "DEPENDENCY_DIRECTION_UNCLEAR DDU-01"
---

# Authentication

## Module Identity

| Field | Value |
|---|---|
| Module ID | `BE-MOD-002` |
| Module type | Domain and HTTP module |
| Implementation status | `IMPLEMENTED` |
| Boundary status | `OBSERVED` |
| Confidence | High |
| Source paths | `backend/src/routes/auth.rs`; `backend/src/middleware/auth.rs`; `backend/src/services/jwt.rs`; `backend/src/services/password.rs`; `backend/src/services/security.rs`; `backend/src/models/user.rs` |

## Responsibility

Verified responsibility: Registers users, authenticates credentials, issues and rotates access/refresh tokens, logs out sessions, loads current-user context, and verifies bearer claims for protected requests.

Shared or inferred responsibility: Default organization membership creation overlaps with Organizations and Tenant Authorization. Login rate tracking uses security helpers and shared persistence.

No additional business intent is inferred beyond current code, tests, migrations, and registered documentation.

## Owned Source Areas

- `backend/src/routes/auth.rs`
- `backend/src/middleware/auth.rs`
- `backend/src/services/jwt.rs`
- `backend/src/services/password.rs`
- `backend/src/services/security.rs`
- `backend/src/models/user.rs`

Ownership means the paths are primary implementation evidence for this documentation module. It does not imply exclusive table, type, or infrastructure ownership where other modules access the same data or service.

## Entry Points

`auth::public_router`, `auth::protected_router`, `auth_middleware`, JWT encode/decode helpers, and password hash/verify helpers.

## Internal Structure

Route-local DTOs and SQL implement session flows; middleware extracts bearer claims; services implement crypto and login hardening.

## Public and Internal Interfaces

Public and protected auth routers, `Claims`, auth response DTOs, refresh cookie behavior, and JWT/password service functions.

The intended long-term public/internal visibility contract is not separately governed unless the interface is enforced by router composition, Rust visibility, or a trait.

## Dependencies

- Internal backend dependencies: `AppState`, PostgreSQL, Config, Argon2, HMAC/SHA-256, RBAC constants, organization membership tables, and security helpers.
- Shared infrastructure: `AppState`, `AppError`, SQLx/PostgreSQL, and cross-cutting services when listed above.
- Persistence: Users, roles, refresh tokens, login-attempt state, and organization membership summaries.
- External libraries or services: only those named in the verified responsibility and dependency statements above.

## Consumers

Root router, tenant middleware, organization routes, RBAC services, webhooks, and every protected/tenant feature.

## Data Concepts

Users, roles, refresh tokens, login-attempt state, and organization membership summaries.

This is a structural ownership view, not a table or column reference. Detailed schema documentation is deferred to Phase 5.

## Request and Processing Flows

Credentials to password verification and token issuance; bearer token to `Claims`; refresh token to rotation; logout to revocation.

Detailed endpoint contracts are deferred to Phase 6.

## Error Behavior

Credential/token failures map to unauthorized or validation errors; persistence errors convert through `AppError`.

## Configuration

JWT expiry and secret, secure-cookie flag, and login-rate thresholds. Secret values are not documented.

Secret values and local environment contents are intentionally excluded.

## Tests

Security service tests cover selected login protections. No dedicated auth route integration-test file or JWT/password unit-test block was found.

Coverage percentages are `UNKNOWN` because no coverage report was used.

## Known Risks and Unknowns

The intended ownership of `Claims` is unclear because services import a middleware-owned type. Historical rationale for token persistence policy is UNKNOWN.

Relevant markers: `MODULE_OWNERSHIP_UNCLEAR MOU-01`, `DEPENDENCY_DIRECTION_UNCLEAR DDU-01`.

## Related Documents

- [Backend Module Catalog](../module-catalog.md)
- [Module Boundaries](../module-boundaries.md)
- [Dependency Map](../dependency-map.md)
- [Testing Map](../testing-map.md)
- [Backend Risks](../backend-risks.md)
- [Architecture Components](../../architecture/components.md)
- [Architecture Boundaries](../../architecture/boundaries.md)
- [Architecture Dependency Model](../../architecture/dependency-model.md)
- [Backend Module Map](../diagrams/backend-module-map.mmd)
- [Backend Dependency Flow](../diagrams/backend-dependency-flow.mmd)
