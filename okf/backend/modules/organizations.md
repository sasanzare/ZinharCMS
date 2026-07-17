---
okf_document_id: "backend-module-organizations"
title: "Organizations and SaaS Operations"
project: "ZinharCMS"
category: "backend-module"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
module_id: "BE-MOD-004"
module_name: "Organizations and SaaS Operations"
module_paths:
  - "backend/src/routes/organizations.rs"
  - "backend/src/models/organization.rs"
  - "backend/src/services/audit.rs"
  - "backend/src/services/email.rs"
module_type: "Domain and HTTP module"
boundary_status: "OVERLAPPING"
primary_sources:
  - "backend/src/routes/organizations.rs"
  - "backend/src/models/organization.rs"
  - "backend/src/services/audit.rs"
  - "backend/src/services/email.rs"
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
  - "RESPONSIBILITY_OVERLAP RO-02"
  - "IMPLEMENTATION_STATUS_UNCLEAR ISU-01"
  - "NEEDS_OWNER_CONFIRMATION"
---

# Organizations and SaaS Operations

## Module Identity

| Field | Value |
|---|---|
| Module ID | `BE-MOD-004` |
| Module type | Domain and HTTP module |
| Implementation status | `IMPLEMENTED` |
| Boundary status | `OVERLAPPING` |
| Confidence | High |
| Source paths | `backend/src/routes/organizations.rs`; `backend/src/models/organization.rs`; `backend/src/services/audit.rs`; `backend/src/services/email.rs` |

## Responsibility

Verified responsibility: Manages organizations, members, invitations, workspace access, domains, rate settings, audit views, email deliveries, SaaS alert-rule views, ownership transfer, and organization exit.

Shared or inferred responsibility: Membership and role behavior overlaps Authentication and Tenant Authorization; audit/email are cross-cutting services used by other modules.

No additional business intent is inferred beyond current code, tests, migrations, and registered documentation.

## Owned Source Areas

- `backend/src/routes/organizations.rs`
- `backend/src/models/organization.rs`
- `backend/src/services/audit.rs`
- `backend/src/services/email.rs`

Ownership means the paths are primary implementation evidence for this documentation module. It does not imply exclusive table, type, or infrastructure ownership where other modules access the same data or service.

## Entry Points

`organizations::protected_router`, `organizations::tenant_router`, route handlers, audit recording, and email delivery functions.

## Internal Structure

One large route module contains DTOs, validation, SQL, transactions, and helper loaders. Audit and email services provide reusable side effects.

## Public and Internal Interfaces

Authenticated/tenant organization routers, organization response DTOs, membership response reuse from auth, audit and email service functions.

The intended long-term public/internal visibility contract is not separately governed unless the interface is enforced by router composition, Rust visibility, or a trait.

## Dependencies

- Internal backend dependencies: Authentication, Tenant Authorization/RLS/RBAC, billing quotas, JWT, PostgreSQL, email webhook provider, and `AppState`.
- Shared infrastructure: `AppState`, `AppError`, SQLx/PostgreSQL, and cross-cutting services when listed above.
- Persistence: Organizations, memberships, invitations, domains, rate settings, audit logs, email deliveries, and alert rules.
- External libraries or services: only those named in the verified responsibility and dependency statements above.

## Consumers

SPA organization surfaces, authentication bootstrap, billing/beta/Marketplace tenant behavior, and administrative workflows.

## Data Concepts

Organizations, memberships, invitations, domains, rate settings, audit logs, email deliveries, and alert rules.

This is a structural ownership view, not a table or column reference. Detailed schema documentation is deferred to Phase 5.

## Request and Processing Flows

Authenticated creation/listing; tenant membership/domain/settings operations; invitation acceptance; ownership transfer/leave transactions.

Detailed endpoint contracts are deferred to Phase 6.

## Error Behavior

Handler validation and permission failures map through `AppError`; email failure behavior depends on configured mode.

## Configuration

Email provider/from/webhook/failure mode, app base URL, and shared tenant limits.

Secret values and local environment contents are intentionally excluded.

## Tests

No colocated `#[test]` block was found in `routes/organizations.rs`; email has unit tests and related behavior is indirectly covered by other service tests.

Coverage percentages are `UNKNOWN` because no coverage report was used.

## Known Risks and Unknowns

Domain verification and alert evaluation/delivery are not fully established. Shared membership DTO and data ownership are responsibility overlaps.

Relevant markers: `RESPONSIBILITY_OVERLAP RO-02`, `IMPLEMENTATION_STATUS_UNCLEAR ISU-01`, `NEEDS_OWNER_CONFIRMATION`.

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
