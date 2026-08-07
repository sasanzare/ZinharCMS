---
type: Architecture
title: System Architecture
description: Current repository-backed composition of the ZinharCMS frontend, backend, data services, and file boundary.
status: stable
sources:
  - id: source-backend-main
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/main.rs
    title: backend/src/main.rs at construction commit
  - id: source-backend-lib
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/lib.rs
    title: backend/src/lib.rs at construction commit
  - id: source-frontend-main
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/frontend/src/main.tsx
    title: frontend/src/main.tsx at construction commit
  - id: source-compose
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/docker-compose.yml
    title: docker-compose.yml at construction commit
---

# Current composition

ZinharCMS is a modular monolith. The browser loads a React/Vite administrative
application, while one Rust/Axum/Tokio backend process composes the route,
middleware, service, plugin, database, cache, and file-handling boundaries.
The repository does not establish independent deployable microservices.

The backend initializes configuration, logging, the PostgreSQL pool, embedded
SQLx migrations, Redis, shared application state, CORS/security middleware,
compression, request identifiers, and a bounded request timeout before it
serves the Axum router. The frontend mounts the provider and session bootstrap
layers and then delegates route selection to the client router.

## Preserved visualizations

The following two Phase 1-preserved visuals are embedded here because their
repository-level meaning remains accurate for this bounded composition.

### system-context

```mermaid
flowchart LR
    AdminBrowser["Administrative browser"] --> WebApp["React/Vite administrative app"]
    WebApp --> Api["Rust/Axum/Tokio backend"]
    PublicClient["Public client"] --> Api
    Api --> Postgres[("PostgreSQL")]
    Api --> Redis[("Redis")]
    Api --> Files[("Configured file boundary")]
```

### container-view

```mermaid
flowchart TB
    Browser["Browser"] --> Spa["React/Vite SPA"]
    Spa --> Router["Axum route tree"]
    Router --> Middleware["Auth, tenant, rate, quota, and security middleware"]
    Middleware --> Services["Rust services and in-process plugins"]
    Services --> Db[("PostgreSQL via SQLx")]
    Services --> Cache[("Redis")]
    Services --> FileBoundary["Configured file handling"]
```

These diagrams intentionally omit deployment topology and external providers;
the repository evidence is not sufficient to claim those details.

The request-path split is described in [runtime and request boundaries](/architecture/runtime-and-request-boundaries.md), and the process/module view is expanded in [backend runtime](/backend/backend-runtime.md).
