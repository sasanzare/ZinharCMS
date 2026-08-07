---
type: Security Control
title: Tenant Isolation
description: Organization context, membership checks, quota controls, and PostgreSQL row-level isolation observed in the repository.
status: draft
sources:
  - id: source-tenant-middleware
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/middleware/tenant.rs
    title: backend/src/middleware/tenant.rs at construction commit
  - id: source-rls-service
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/services/rls.rs
    title: backend/src/services/rls.rs at construction commit
  - id: source-rls-migration
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/migrations/0009_v2_phase_three_rls.sql
    title: backend/migrations/0009_v2_phase_three_rls.sql at construction commit
  - id: source-rls-tests
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/tests/security_phase2_rls.rs
    title: backend/tests/security_phase2_rls.rs at construction commit
---

# Tenant request context

Tenant-protected requests require an organization UUID in
`X-Organization-Id`. Middleware verifies active membership, resolves the
organization context and role, applies configured rate and quota checks, and
inserts the tenant context for downstream handlers. Selected operations may
also require a step-up scope.

The RLS service sets organization and user context on tenant connections and
transactions. The migration defines helper functions, forces row-level
security on protected tables, and adds policies; application queries also use
explicit organization predicates in the observed routes and services. The
repository includes a security test matrix for cross-tenant access and insert
behavior.

These files establish implementation and test evidence, not proof that a
particular production database has applied every migration or that the live
deployment is configured identically. Public organization routing and the
durable ownership of this policy remain open decisions.

Authorization prerequisites are in [authorization and RBAC](/security/authorization-and-rbac.md), and the request-layer placement is in [runtime and request boundaries](/architecture/runtime-and-request-boundaries.md).

## Preserved visualization

### tenant-membership-workflow

```mermaid
sequenceDiagram
    participant Request as Request
    participant Middleware as Tenant middleware
    participant Membership as Organization membership
    participant Handler as Tenant handler
    Request->>Middleware: Read X-Organization-Id
    Middleware->>Membership: Verify active membership
    alt membership is valid
        Membership-->>Middleware: Organization and role
        Middleware->>Handler: Insert tenant context
        Handler-->>Request: Continue tenant-scoped operation
    else membership is missing or inactive
        Membership-->>Middleware: Reject
        Middleware-->>Request: Authorization error
    end
```

### tenant-access-control

```mermaid
flowchart LR
    Request["Tenant request"] --> Context["Tenant context"]
    Context --> Query["Tenant connection / transaction"]
    Query --> Session["PostgreSQL session settings"]
    Session --> Policy["Forced RLS policies"]
    Policy --> Rows[("Organization-scoped rows")]
```
