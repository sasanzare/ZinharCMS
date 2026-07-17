---
okf_document_id: "backend-module-tenant-authorization"
title: "Tenant Authorization and RLS"
project: "ZinharCMS"
category: "backend-module"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
module_id: "BE-MOD-003"
module_name: "Tenant Authorization and RLS"
module_paths:
  - "backend/src/middleware/tenant.rs"
  - "backend/src/services/rbac.rs"
  - "backend/src/services/rls.rs"
  - "backend/src/services/rate_limit.rs"
  - "backend/src/services/quota.rs"
  - "backend/migrations/0008_v2_phase_one_organizations.sql"
  - "backend/migrations/0009_v2_phase_three_rls.sql"
module_type: "Shared domain and persistence module"
boundary_status: "OVERLAPPING"
primary_sources:
  - "backend/src/middleware/tenant.rs"
  - "backend/src/services/rbac.rs"
  - "backend/src/services/rls.rs"
  - "backend/src/services/rate_limit.rs"
  - "backend/src/services/quota.rs"
  - "backend/migrations/0008_v2_phase_one_organizations.sql"
  - "backend/migrations/0009_v2_phase_three_rls.sql"
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
  - "MODULE_BOUNDARY_UNCLEAR MBU-01"
  - "MODULE_OWNERSHIP_UNCLEAR MOU-02"
  - "RESPONSIBILITY_OVERLAP RO-01"
  - "BUSINESS_RULE_UNVERIFIED BRU-01"
---

# Tenant Authorization and RLS

## Module Identity

| Field | Value |
|---|---|
| Module ID | `BE-MOD-003` |
| Module type | Shared domain and persistence module |
| Implementation status | `IMPLEMENTED` |
| Boundary status | `OVERLAPPING` |
| Confidence | High |
| Source paths | `backend/src/middleware/tenant.rs`; `backend/src/services/rbac.rs`; `backend/src/services/rls.rs`; `backend/src/services/rate_limit.rs`; `backend/src/services/quota.rs`; `backend/migrations/0008_v2_phase_one_organizations.sql`; `backend/migrations/0009_v2_phase_three_rls.sql` |

## Responsibility

Verified responsibility: Resolves organization context, verifies active membership, applies organization/user rate and plan quota gates, evaluates role permissions, and provides RLS-scoped database access.

Shared or inferred responsibility: Tenant enforcement is distributed across middleware, handler checks, SQL predicates, RBAC helpers, and RLS helpers.

No additional business intent is inferred beyond current code, tests, migrations, and registered documentation.

## Owned Source Areas

- `backend/src/middleware/tenant.rs`
- `backend/src/services/rbac.rs`
- `backend/src/services/rls.rs`
- `backend/src/services/rate_limit.rs`
- `backend/src/services/quota.rs`
- `backend/migrations/0008_v2_phase_one_organizations.sql`
- `backend/migrations/0009_v2_phase_three_rls.sql`

Ownership means the paths are primary implementation evidence for this documentation module. It does not imply exclusive table, type, or infrastructure ownership where other modules access the same data or service.

## Entry Points

`tenant_middleware`, `TenantContext`, RBAC permission functions/constants, tenant connection/transaction helpers, quota checks, and rate-limit checks.

## Internal Structure

Middleware orchestrates JWT verification and membership; services own reusable permission, RLS, quota, and Redis rate logic.

## Public and Internal Interfaces

Request extensions carrying `Claims` and `TenantContext`; RLS connection/transaction APIs; RBAC and quota/rate service functions.

The intended long-term public/internal visibility contract is not separately governed unless the interface is enforced by router composition, Rust visibility, or a trait.

## Dependencies

- Internal backend dependencies: Authentication, Organizations, Config/AppState, PostgreSQL, Redis, billing plan data, and feature handlers.
- Shared infrastructure: `AppState`, `AppError`, SQLx/PostgreSQL, and cross-cutting services when listed above.
- Persistence: Organization memberships, roles, plan limits, usage counters, rate settings/counters, and PostgreSQL session context.
- External libraries or services: only those named in the verified responsibility and dependency statements above.

## Consumers

All tenant-protected routers plus billing, Marketplace, email, audit, and organization services.

## Data Concepts

Organization memberships, roles, plan limits, usage counters, rate settings/counters, and PostgreSQL session context.

This is a structural ownership view, not a table or column reference. Detailed schema documentation is deferred to Phase 5.

## Request and Processing Flows

Bearer and organization ID to membership; rate/quota checks to `TenantContext`; context to handler and RLS-scoped SQL.

Detailed endpoint contracts are deferred to Phase 6.

## Error Behavior

Invalid/missing context becomes unauthorized/forbidden/bad request; Redis rate failures can become service unavailable.

## Configuration

Organization and per-user rate limits plus burst values; database and Redis connectivity.

Secret values and local environment contents are intentionally excluded.

## Tests

RBAC, quota, and rate-limit modules contain unit/static tests. Complete route-by-route isolation coverage is not established in Phase 3.

Coverage percentages are `UNKNOWN` because no coverage report was used.

## Known Risks and Unknowns

The boundary is cross-cutting rather than package-enforced. Query-level coverage and intent behind limits remain later security/database work.

Relevant markers: `MODULE_BOUNDARY_UNCLEAR MBU-01`, `MODULE_OWNERSHIP_UNCLEAR MOU-02`, `RESPONSIBILITY_OVERLAP RO-01`, `BUSINESS_RULE_UNVERIFIED BRU-01`.

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
