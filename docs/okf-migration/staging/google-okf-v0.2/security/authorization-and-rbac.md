---
type: Authorization Policy
title: Authorization and RBAC
description: Observed role, permission, step-up, organization-membership, and route authorization checks.
status: draft
sources:
  - id: source-auth-middleware
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/middleware/auth.rs
    title: backend/src/middleware/auth.rs at construction commit
  - id: source-tenant-middleware
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/middleware/tenant.rs
    title: backend/src/middleware/tenant.rs at construction commit
  - id: source-rbac
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/services/rbac.rs
    title: backend/src/services/rbac.rs at construction commit
  - id: source-content
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/routes/content.rs
    title: backend/src/routes/content.rs at construction commit
---

# Observed authorization decisions

Route handlers and middleware combine authenticated claims, organization
membership, role/permission checks, and selected step-up requirements. The
tenant middleware resolves the active organization from the request header and
membership before inserting a tenant context for downstream handlers.

Content and related feature routes apply explicit permission checks around
tenant-scoped operations. Administrative plugin and Marketplace operations
also use role or permission gates in their route/service layers. The precise
owner of module, operations, security, and documentation policy remains an
open decision; this Concept records the checks visible in source rather than
assigning ownership or asserting a complete policy catalog.

Authentication and session prerequisites are described in [authentication and sessions](authentication-and-sessions.md), while row-level tenant enforcement is described in [tenant isolation](tenant-isolation.md).

## Preserved visualizations

### authorization-decision-flow

```mermaid
sequenceDiagram
    participant Request as Request
    participant Auth as Auth middleware
    participant Tenant as Tenant middleware
    participant Route as Route handler
    Request->>Auth: Verify access claims
    Auth->>Tenant: Continue authenticated request
    Tenant->>Route: Provide organization context
    alt permission and step-up checks pass
        Route-->>Request: Perform operation
    else a check fails
        Route-->>Request: Authorization error
    end
```

### rbac-model

```mermaid
flowchart LR
    User["Authenticated user"] --> Membership["Organization membership"]
    Membership --> Role["Role and claims"]
    Role --> Permission["Required permission"]
    Permission --> Route["Protected route operation"]
```
