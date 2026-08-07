---
type: Component
title: Persistence, Services, and Configuration
description: Current PostgreSQL, Redis, application-state, environment-configuration, and backend service-composition boundaries.
status: draft
sources:
  - id: source-main
    resource: https://github.com/sasanzare/ZinharCMS/blob/e37e94e2e6960a2547f33bf1ebb4225f818b3a4b/backend/src/main.rs
    title: backend/src/main.rs at Phase 4 source head
  - id: source-state
    resource: https://github.com/sasanzare/ZinharCMS/blob/e37e94e2e6960a2547f33bf1ebb4225f818b3a4b/backend/src/state.rs
    title: backend/src/state.rs at Phase 4 source head
  - id: source-db
    resource: https://github.com/sasanzare/ZinharCMS/blob/e37e94e2e6960a2547f33bf1ebb4225f818b3a4b/backend/src/db/mod.rs
    title: backend/src/db/mod.rs at Phase 4 source head
  - id: source-config
    resource: https://github.com/sasanzare/ZinharCMS/blob/e37e94e2e6960a2547f33bf1ebb4225f818b3a4b/backend/src/config.rs
    title: backend/src/config.rs at Phase 4 source head
  - id: source-services
    resource: https://github.com/sasanzare/ZinharCMS/blob/e37e94e2e6960a2547f33bf1ebb4225f818b3a4b/backend/src/services/mod.rs
    title: backend/src/services/mod.rs at Phase 4 source head
  - id: source-cargo
    resource: https://github.com/sasanzare/ZinharCMS/blob/e37e94e2e6960a2547f33bf1ebb4225f818b3a4b/backend/Cargo.toml
    title: backend/Cargo.toml at Phase 4 source head
---

# Runtime composition

Startup loads environment-backed configuration, initializes tracing, creates
a lazy PostgreSQL pool, runs the embedded SQLx migrations, performs the
configured bootstrap step when applicable, creates the Redis client, builds
the shared `AppState`, and serves the composed Axum router. The runtime
composition is one backend process; these initialization steps are not a
claim about a deployed environment.

## Shared state

`AppState` holds the shared configuration, PostgreSQL pool, Redis client,
outbound HTTP client, and page-preview broadcast channels. Route handlers,
middleware, and services receive this state through the Axum application
boundary. The service registry includes authentication, sessions, MFA,
authorization, workflow, cache, email, webhook, file, Marketplace, and other
domain-support modules; it does not imply that each module is an independently
owned or deployed service.

## Persistence

PostgreSQL access uses SQLx. The configured pool is bounded to ten connections
in the current database module, and the repository embeds and runs the
repository migration directory at startup. Tenant-scoped connections and
transactions are used by affected route and service paths. Redis is used as a
cache and for selected short-lived runtime state; it is not presented as the
authoritative persistence layer for content.

## Configuration surface

`Config` validates environment-backed database and Redis URLs, JWT and MFA key
rings, upload limits and directory, CORS and cookie settings, preview and
rate-limit settings, billing and email provider settings, organization limits,
cleanup/retention settings, application URL, trusted proxy behavior, and the
listen port. The Concept records the configuration surface and validation
boundary without copying environment values or claiming production defaults.

## Storage boundary

The current file boundary is the configured local upload directory, coordinated
with database media records and cleanup jobs. Shared/object storage, CDN
behavior, backup durability, and asset authorization are not established by
the repository evidence. NOC-02 remains the decision dependency for those
questions.

## Relationships

The process-level view is in [backend runtime](/backend/backend-runtime.md),
and route, middleware, service, and plugin structure is in [module boundaries](/backend/module-boundaries.md).
Cross-cutting cache, file, email, webhook, and transaction effects are in [integrations and side effects](/architecture/integrations-and-side-effects.md). The external API family boundary is in [API contract overview](/api/api-contract-overview.md), and tenant access controls are in [tenant isolation](/security/tenant-isolation.md).

## Open decision dependency

NOC-02 remains open for storage architecture, asset authorization, and the
deployment-level durability model. No production storage or backup topology
is inferred from the local configuration and migration sources.
