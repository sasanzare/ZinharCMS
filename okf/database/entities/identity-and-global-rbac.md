---
okf_document_id: "database-entity-identity-global-rbac"
title: "Identity and Global RBAC"
project: "ZinharCMS"
category: "database-entity"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
entity_id: "DB-ENT-001"
entity_name: "Identity and Global RBAC"
entity_domain: "Identity and Access"
schema_objects: ["users", "roles", "user_roles", "refresh_tokens", "login_attempts"]
owning_module: "Auth"
tenant_scope: "global"
implementation_status: "IMPLEMENTED"
confidence: "high"
primary_sources: ["backend/migrations/0001_initial_schema.sql", "backend/migrations/0007_phase_seven_security.sql", "backend/src/routes/auth.rs", "backend/src/models"]
related_documents: ["database/entity-catalog.md", "database/multi-tenancy.md", "backend/modules/authentication.md"]
related_diagrams: ["database/diagrams/entity-relationship-overview.mmd"]
uncertainty_markers: ["DATA_LIFECYCLE_UNCLEAR DLU-01"]
---

# Identity and Global RBAC

## Entity Identity

`DB-ENT-001` covers global identities, role definitions and grants, refresh-token state, and login-attempt history. Shared auth/user models and auth-local rows map the aggregate. Implementation and repository confidence are high; runtime schema state is unknown.

## Purpose

Verified purpose: authenticate users, store global role permissions, issue/revoke refresh tokens, and record security-relevant login attempts. No tenant-specific RBAC purpose is inferred; organization membership is separate. Retention and identity-deletion policy are unclear. No planned behavior is represented here.

## Storage Structure

| Field group | Type/null/default/constraint | Meaning and evidence |
| --- | --- | --- |
| `users.id`, email, password hash | UUID identity plus constrained identity/credential fields | Auth identity; migrations and auth queries |
| user status/profile timestamps | Scalar/temporal fields | Account state and lifecycle |
| `roles.id`, name, permissions | UUID/text/array-like permission data | Seeded global roles |
| token hash/expiry/revocation | Security-sensitive text/time fields | Refresh-token rotation and invalidation |
| attempt identity/outcome/time | Text/network/outcome/time fields | Login security history |

## Identifiers

UUID primary keys identify users, roles, tokens, and attempts. `user_roles(user_id, role_id)` is a composite identifier. Email is a unique login identifier; token material is not a public identifier.

## Relationships

Users have many roles through `user_roles`, many refresh tokens, memberships, and domain records elsewhere. Login attempts store email rather than a user FK. FK delete behavior is defined in the migrations and must be checked before identity deletion.

## Ownership

Auth owns writes. Authentication/tenant middleware, Organizations, audit, and Marketplace creator flows read identity state.

## Tenant Isolation

These tables are global and are not in the forced-RLS inventory. Tenant authorization comes from organization membership after identity authentication.

## Lifecycle

Tokens expire or are revoked; login attempts form history; users and roles have timestamps/status. Retention and hard-delete behavior are not governed (`DLU-01`).

## Constraints and Indexes

Unique email, role naming, join PKs, token lookup/revocation indexes, and login-attempt lookup indexes support integrity and security flows.

## Persistence Mapping

Auth routes/services use direct SQLx plus shared user/auth types. Startup bootstrap writes `users` only when the table is empty.

## Security and Privacy

Passwords, token hashes, email addresses, and network/device attempt data are sensitive. Never expose seed credentials, hashes, or token values.

## Known Risks and Unknowns

Bootstrap ownership, credential rotation, identity retention, and role-change governance require owner confirmation.

## Related Documents

See [Multi-Tenancy](../multi-tenancy.md), [Seeds and Fixtures](../seeds-and-fixtures.md), and [Database Risks](../database-risks.md).

Return to the [Entity Catalog](../entity-catalog.md) or continue with the owning [Authentication module](../../backend/modules/authentication.md).
