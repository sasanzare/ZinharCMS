---
type: Component
title: Backend Runtime
description: Rust/Axum/Tokio process initialization, shared state, database access, and response boundary.
status: stable
sources:
  - id: source-main
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/main.rs
    title: backend/src/main.rs at construction commit
  - id: source-state
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/state.rs
    title: backend/src/state.rs at construction commit
  - id: source-db
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/db/mod.rs
    title: backend/src/db/mod.rs at construction commit
  - id: source-error
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/error.rs
    title: backend/src/error.rs at construction commit
---

# Process boundary

The backend is a single Rust process using Tokio and Axum. Startup loads
environment configuration, initializes tracing, creates the PostgreSQL pool,
runs embedded SQLx migrations, optionally performs the configured bootstrap
step, constructs a Redis client and shared `AppState`, and serves the composed
router on the configured address and port.

`AppState` contains shared configuration, the PostgreSQL pool, Redis client,
outbound HTTP client, and page-preview channel state. Database access is
provided through SQLx; the repository configures a bounded pool and embeds the
migration directory in the binary.

The response boundary maps application errors to JSON error responses and
keeps internal error detail out of client-facing messages. The runtime applies
CORS, security headers, compression, request identifiers, and a 30-second
request timeout in the application assembly observed at construction.

Route composition and request classes are described in [runtime and request boundaries](/architecture/runtime-and-request-boundaries.md), while module ownership is described in [module boundaries](/backend/module-boundaries.md).
