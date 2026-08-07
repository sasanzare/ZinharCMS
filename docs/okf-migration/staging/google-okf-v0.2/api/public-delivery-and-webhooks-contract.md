---
type: API Contract
title: Public Delivery and Webhooks Contract
description: Observed public delivery and organization-scoped webhook boundaries, cache behavior, and contract limits.
status: draft
sources:
  - id: source-route-composition
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/src/routes/mod.rs
    title: backend/src/routes/mod.rs at Phase 5 source HEAD
  - id: source-delivery-routes
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/src/routes/delivery.rs
    title: backend/src/routes/delivery.rs at Phase 5 source HEAD
  - id: source-webhook-routes
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/src/routes/webhooks.rs
    title: backend/src/routes/webhooks.rs at Phase 5 source HEAD
  - id: source-webhook-service
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/src/services/webhooks.rs
    title: backend/src/services/webhooks.rs at Phase 5 source HEAD
  - id: source-cache-service
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/src/services/cache.rs
    title: backend/src/services/cache.rs at Phase 5 source HEAD
---

# Observed delivery contract

The root router exposes public delivery routes without the authenticated
tenant middleware. The current public surface includes published content
lists and details, published pages, public settings, navigation, sitemap, and
robots responses. The handlers select the active organization whose slug is
`default`; the repository does not establish host-based or custom-domain
organization selection.

Delivery queries use an organization-scoped database connection and repeat the
organization predicate in the query. Content and pages are restricted to
published state. Public settings and navigation require their public flags,
and page rendering resolves system or organization component schemas. Input
slugs, locale, sorting, filtering, and pagination are normalized before query
construction; list sizes are bounded by the handler.

The cache keys include the selected organization identifier and the relevant
delivery surface. Content, page, sitemap, and robots invalidation functions
are called by the current route/service boundary. These mechanics do not
establish a global freshness, durability, or invalidation guarantee.

# Observed webhook contract

Webhook administration is mounted inside the tenant-protected router. A
request requires `X-Organization-Id`, active organization membership, and the
organization webhook-manager role. List, create, read, update, delete,
delivery-history, and test operations are exposed as one organization-scoped
family. Webhook events are limited to the supported entry and page publish or
unpublish values, and URLs pass the outbound HTTP safety validator.

Dispatch loads only active subscriptions for the current organization,
generates an HMAC signature, sends the JSON payload through the outbound HTTP
boundary, and records success or failure in `webhook_deliveries`. Dispatch is
spawned asynchronously and the observed implementation records one attempt;
it does not prove retry, idempotency, compensation, delivery ordering, or
provider-level durability.

The broader route-family inventory is in [API contract overview](/api/api-contract-overview.md). Tenant request context and database enforcement are
described by [tenant isolation](/security/tenant-isolation.md) and [tenant data policy](/database/tenant-data-policy.md). Public organization selection is also the domain boundary in [public delivery](/domain/public-delivery.md).

## Open decision dependencies

* NOC-01 and NOC-11: intended host, custom-domain, and domain-verification
  routing are not implemented evidence in this source snapshot.
* NOC-08: versioning, compatibility windows, deprecation, and a complete
  public contract remain unassigned; the served OpenAPI document is not treated
  as a complete compatibility guarantee.
* NOC-09: retry, compensation, idempotency, and user-visible side-effect
  policy are not inferred from asynchronous dispatch.

The public-delivery visualization is owned by [public delivery](/domain/public-delivery.md) so the API and domain Concepts do not duplicate the same flow.
