---
type: Architecture
title: Integrations and Side Effects
description: Current cache, webhook, email, file, outbound-request, and transaction side-effect boundaries of ZinharCMS.
status: draft
sources:
  - id: source-routes
    resource: https://github.com/sasanzare/ZinharCMS/blob/e37e94e2e6960a2547f33bf1ebb4225f818b3a4b/backend/src/routes/mod.rs
    title: backend/src/routes/mod.rs at Phase 4 source head
  - id: source-cache
    resource: https://github.com/sasanzare/ZinharCMS/blob/e37e94e2e6960a2547f33bf1ebb4225f818b3a4b/backend/src/services/cache.rs
    title: backend/src/services/cache.rs at Phase 4 source head
  - id: source-delivery
    resource: https://github.com/sasanzare/ZinharCMS/blob/e37e94e2e6960a2547f33bf1ebb4225f818b3a4b/backend/src/routes/delivery.rs
    title: backend/src/routes/delivery.rs at Phase 4 source head
  - id: source-webhooks
    resource: https://github.com/sasanzare/ZinharCMS/blob/e37e94e2e6960a2547f33bf1ebb4225f818b3a4b/backend/src/services/webhooks.rs
    title: backend/src/services/webhooks.rs at Phase 4 source head
  - id: source-email
    resource: https://github.com/sasanzare/ZinharCMS/blob/e37e94e2e6960a2547f33bf1ebb4225f818b3a4b/backend/src/services/email.rs
    title: backend/src/services/email.rs at Phase 4 source head
  - id: source-outbound-http
    resource: https://github.com/sasanzare/ZinharCMS/blob/e37e94e2e6960a2547f33bf1ebb4225f818b3a4b/backend/src/services/outbound_http.rs
    title: backend/src/services/outbound_http.rs at Phase 4 source head
  - id: source-file-cleanup
    resource: https://github.com/sasanzare/ZinharCMS/blob/e37e94e2e6960a2547f33bf1ebb4225f818b3a4b/backend/src/services/file_cleanup.rs
    title: backend/src/services/file_cleanup.rs at Phase 4 source head
  - id: source-config
    resource: https://github.com/sasanzare/ZinharCMS/blob/e37e94e2e6960a2547f33bf1ebb4225f818b3a4b/backend/src/config.rs
    title: backend/src/config.rs at Phase 4 source head
---

# Responsibility

This Concept consolidates the current side-effect boundaries that sit around
the route, service, persistence, and public-delivery paths. It describes
repository behavior, not a guarantee that every effect is globally atomic or
that an external provider is available.

## Cache and public delivery

Public delivery reads use Redis-backed read-through caching with a default
five-minute TTL. A Redis connection failure or cache miss falls back to the
repository fetch, and cache writes are best effort. Published content and
pages invalidate the corresponding organization-scoped delivery keys after
mutations. Cache availability is therefore not presented as a durable
source-of-truth boundary.

## Webhooks

Entry and page publish/unpublish paths can trigger the supported webhook
events. The service loads active organization-scoped subscriptions, signs the
JSON payload, and dispatches each delivery in a Tokio task through the shared
outbound HTTP client. Delivery attempts are recorded with status, response
metadata, or an error. The repository does not establish a durable queue,
retry schedule, compensation action, or user-visible failure policy; NOC-09
remains unresolved and no such guarantee is inferred here.

## Email

Invitation and billing notifications create an organization-scoped delivery
record before selecting the configured `disabled`, `webhook`, or log-style
provider behavior. The delivery record is updated with the result, and strict
email failure mode can surface a provider failure to the caller. A worker,
retry policy, or external provider SLA is not established by the current
source.

## Files and outbound requests

Media cleanup is represented by durable database jobs that can be processed in
bounded batches, with retry and failed states around local filesystem removal.
The repository exposes a configured upload directory; it does not establish
object storage, CDN, shared-filesystem durability, or a production storage
topology. Those storage and authorization questions remain NOC-02.

Webhook delivery uses a shared outbound client that restricts schemes and
unsafe destinations, resolves and rejects forbidden private or metadata
addresses, disables redirects, and applies bounded connection, total-request,
and response-size limits. These are request-safety controls, not evidence of
an external integration platform.

## Transaction and audit boundaries

Tenant and organization operations use SQLx connections or transactions and
record audit events in the affected paths. Some route handlers commit database
work before broadcasting preview updates, invalidating delivery cache, or
triggering webhooks. The code therefore exposes explicit local transaction
boundaries but does not define one universal atomic boundary across database,
Redis, filesystem, email, and webhook effects.

## Relationships

The overall process composition is described in [system architecture](/architecture/system-architecture.md), and request classes are described in [runtime and request boundaries](/architecture/runtime-and-request-boundaries.md). Persistence and configuration are consolidated in [persistence, services, and configuration](/backend/persistence-services-and-configuration.md), while backend module boundaries are described in [module boundaries](/backend/module-boundaries.md). Public API and tenant/security contracts remain in [API contract overview](/api/api-contract-overview.md) and [tenant isolation](/security/tenant-isolation.md).

## Open decision dependencies

NOC-02 covers storage architecture and asset authorization. NOC-09 covers
failure guarantees for cache invalidation, webhooks, email, file cleanup, and
related effects. The current implementation is recorded without choosing
those owner policies.
