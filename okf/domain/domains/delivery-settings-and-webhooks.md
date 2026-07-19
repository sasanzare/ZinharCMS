---
okf_document_id: "domain-delivery-settings-webhooks"
title: "Delivery, Settings, and Webhooks Domain"
project: "ZinharCMS"
category: "domain-detail"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
domain_id: "DOM-DELIVERY"
domain_name: "Delivery, Settings, and Webhooks"
domain_status: "PARTIALLY_IMPLEMENTED"
boundary_status: "OVERLAPPING"
primary_sources:
  - "backend/src/routes/delivery.rs"
  - "backend/src/routes/webhooks.rs"
  - "backend/src/services/cache.rs"
  - "backend/src/services/webhooks.rs"
  - "backend/migrations/0005_phase_five_delivery_api.sql"
related_documents:
  - "../domain-catalog.md"
  - "../publication-workflow.md"
  - "../settings-and-configuration-rules.md"
related_diagrams:
  - "../diagrams/publication-workflow.mmd"
---

# Delivery, Settings, and Webhooks Domain

## Domain Identity

- Domain ID: `DOM-DELIVERY`
- Terminology: public delivery, published resource, cache, public setting, navigation item, webhook, delivery attempt.
- Implementation: `PARTIALLY_IMPLEMENTED`; boundary `OVERLAPPING`; confidence High.

## Responsibility

- Verified: deliver published entries/pages/settings/navigation, generate sitemap/robots, cache results, manage webhook subscriptions, and send signed publication notifications.
- Inferred: public settings/navigation configure the public site represented by the default organization.
- Shared: publication producers in Content/Pages; Redis/cache; tenant resolution; static media serving.
- Unclear: multi-tenant public routing, settings/navigation management ownership, webhook delivery guarantee, and cache consistency SLO.

## Core Entities

`public_settings`, `navigation_items`, `webhooks`, `webhook_deliveries`, content/pages, and Redis cache entries.

## Core Services

Delivery route, cache service, webhook route/service, content/pages invalidation callers, RLS helpers.

## API Surface

Unauthenticated `/api/v1` entries/pages/settings/navigation, sitemap/robots, and protected webhook CRUD/test/delivery history. See [Public Delivery](../../api/endpoints/public-delivery.md) and [CMS Webhooks](../../api/endpoints/cms-webhooks.md).

## Frontend Surface

Webhook management in `SettingsPage`; public delivery consumers are external. No current complete editor for public settings/navigation was found.

## Actors

Public consumer, organization webhook manager, content/page publisher, and external webhook receiver.

## Business Rules

`BR-DELIVERY-001` through `BR-DELIVERY-005`.

## Invariants

Published-only delivery, supported webhook events, delivery status allowlist, setting-key format, and navigation locale format.

## State and Lifecycle

Webhook enabled state is boolean; deliveries are terminal `delivered`/`failed`. Public settings/navigation have no application lifecycle beyond stored rows. Cache has TTL/key lifecycle but no business state entity.

## Access Rules

Public reads are unauthenticated. Webhook management uses tenant membership and organization RBAC. Webhook payloads include tenant identity but do not expose the stored secret.

## Validation Rules

Delivery pagination/sort/filter/locale, webhook URL/event/secret, setting-key, and navigation field constraints.

## Workflows

[Editorial Publication](../workflows/editorial-publication.md) and [Publication Webhook Delivery](../workflows/publication-webhook-delivery.md).

## Side Effects

Redis cache fill/invalidation, outbound HTTP, HMAC signature, delivery history, and logs. No durable retry/outbox.

## Tests

Webhook URL/event/signature and delivery filter/XML helpers are tested. Published-only selection, cache invalidation, public tenant selection, actual HTTP delivery, process loss, and settings/navigation workflows are not.

## Risks and Unknowns

Fixed `default` public tenant, route-owned invalidation dependency, best-effort cache and webhook delivery, possible DNS rebinding gap, and missing management workflow.

Return to the [Domain Catalog](../domain-catalog.md).

