---
type: Domain Model
title: Billing and Quotas
description: Implemented plan, subscription, usage, and request/media/content quota mechanisms with provider-boundary caveats.
status: draft
sources:
  - id: source-billing-routes
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/src/routes/billing.rs
    title: backend/src/routes/billing.rs at Phase 5 source HEAD
  - id: source-quota-service
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/src/services/quota.rs
    title: backend/src/services/quota.rs at Phase 5 source HEAD
  - id: source-billing-migration
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/migrations/0010_v2_phase_five_billing_quota.sql
    title: backend/migrations/0010_v2_phase_five_billing_quota.sql at Phase 5 source HEAD
  - id: source-saas-migration
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/migrations/0012_v2_phase_seven_saas_ops.sql
    title: backend/migrations/0012_v2_phase_seven_saas_ops.sql at Phase 5 source HEAD
  - id: source-rbac
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/src/services/rbac.rs
    title: backend/src/services/rbac.rs at Phase 5 source HEAD
---

# Implemented capability

Plans expose member, content-record, media-byte, and API-request limits along
with plan metadata and feature values. Organization subscriptions and monthly
usage counters are organization-keyed and have RLS policies. A default free
subscription is inserted when a tenant first needs plan state; manual plan
changes update the organization subscription. The route family also exposes
subscription, usage, rebuild, checkout, portal, and Stripe webhook surfaces.

Tenant middleware checks and records API-request usage except on billing paths.
Content and media writes call capacity checks. The media transaction path locks
the organization row before checking the media limit, and usage counters use
organization/period/metric uniqueness with conflict updates. Usage summaries
can rebuild content, member, and media counts from current tenant data before
returning the metric view.

Organization billing-manager authorization is implemented in the RBAC helpers
for billing administration. The presence of Stripe route/service integration
does not establish provider availability, settlement finality, refund/tax
policy, webhook retry guarantees, or compensation behavior.

The data boundary is described in [tenant data policy](/database/tenant-data-policy.md), and the broader request surface is in [API contract overview](/api/api-contract-overview.md).

## Open decision dependencies

* NOC-09: provider failure, retry, idempotency, compensation, refund, and
  user-visible failure rules are not assigned by the current source.
* Deployment and applied-migration state are intentionally outside this
  source-derived domain Concept.

