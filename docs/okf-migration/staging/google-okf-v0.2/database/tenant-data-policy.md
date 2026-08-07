---
type: Database Policy
title: Tenant Data Policy
description: Source-backed PostgreSQL session context, row-level security, and application tenant-boundary controls.
status: draft
sources:
  - id: source-rls-service
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/src/services/rls.rs
    title: backend/src/services/rls.rs at Phase 5 source HEAD
  - id: source-rls-migration
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/migrations/0009_v2_phase_three_rls.sql
    title: backend/migrations/0009_v2_phase_three_rls.sql at Phase 5 source HEAD
  - id: source-billing-rls
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/migrations/0010_v2_phase_five_billing_quota.sql
    title: backend/migrations/0010_v2_phase_five_billing_quota.sql at Phase 5 source HEAD
  - id: source-marketplace-installation-rls
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/migrations/0015_v3_phase_one_marketplace_foundation.sql
    title: backend/migrations/0015_v3_phase_one_marketplace_foundation.sql at Phase 5 source HEAD
  - id: source-marketplace-runtime-rls
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/migrations/0020_v3_phase_seven_permission_sandbox_kill_switch.sql
    title: backend/migrations/0020_v3_phase_seven_permission_sandbox_kill_switch.sql at Phase 5 source HEAD
  - id: source-marketplace-adapter-rls
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/migrations/0021_v3_phase_eight_runtime_adapters.sql
    title: backend/migrations/0021_v3_phase_eight_runtime_adapters.sql at Phase 5 source HEAD
  - id: source-marketplace-finance-rls
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/migrations/0022_v3_phase_nine_marketplace_finance.sql
    title: backend/migrations/0022_v3_phase_nine_marketplace_finance.sql at Phase 5 source HEAD
  - id: source-feedback-rls
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/migrations/0024_v3_phase_ten_ratings_abuse.sql
    title: backend/migrations/0024_v3_phase_ten_ratings_abuse.sql at Phase 5 source HEAD
  - id: source-file-rls
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/migrations/0030_security_phase_seven_file_storage.sql
    title: backend/migrations/0030_security_phase_seven_file_storage.sql at Phase 5 source HEAD
  - id: source-tenant-middleware
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/src/middleware/tenant.rs
    title: backend/src/middleware/tenant.rs at Phase 5 source HEAD
  - id: source-rls-test
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/tests/security_phase2_rls.rs
    title: backend/tests/security_phase2_rls.rs at Phase 5 source HEAD
---

# Session context and policies

Tenant connections receive `zinhar.organization_id`, an optional
`zinhar.user_id`, and `zinhar.rls_bypass=false`. Tenant transactions set the
same values transaction-locally. The explicit bypass transaction clears the
organization and user settings and sets the bypass flag only for a deliberate
maintenance/test path; this is not evidence of a normal request mode.

The base RLS migration forces row-level security and creates operation policies
for `content_types`, `content_entries`, `pages`, `page_versions`, `media`,
`media_variants`, `comments`, `webhooks`, `webhook_deliveries`,
`public_settings`, and `navigation_items`. `component_registry` has a separate
policy: system rows may be selected, while writes require a non-system row in
the current organization. The tenant predicate is organization equality unless
the explicit bypass setting is enabled.

Later migrations add the same organization policy pattern to billing
subscriptions and usage counters; Marketplace installations, organization
kill switches, template imports and plugin hooks, purchases, entitlements,
revenue ledger rows, product reviews, and abuse reports; and file cleanup
jobs. Some Marketplace catalog and creator records are not organization rows
and are governed by their route/service ownership checks instead of being
described here as tenant tables.

# Application-layer boundary

Tenant middleware requires `X-Organization-Id`, validates an access identity,
loads an active organization membership, inserts the organization and role
context, then applies rate and quota checks. Tenant handlers and services also
repeat explicit organization predicates in the source-backed delivery,
webhook, media, Marketplace, and quota queries. This is a layered control
model; the evidence does not justify claiming that every table or every live
deployment is protected identically.

The migration files are source chronology and SQLx input. They do not prove
which migration version is applied in a deployed database. The repository test
matrix contains cross-tenant visibility, update, delete, insert, context
cleanup, and non-superuser assertions, but a live test result is not claimed by
this documentation-only phase.

Tenant request placement is described in [tenant isolation](/security/tenant-isolation.md), and API usage is described in [public delivery and webhooks contract](/api/public-delivery-and-webhooks-contract.md). Billing and Marketplace data consumers are linked from [billing and quotas](/domain/billing-and-quotas.md) and [Marketplace](/domain/marketplace.md).

## Open decision dependencies

* NOC-01 and NOC-11: public host, custom-domain, and domain-verification
  routing are not established by the current default-organization delivery
  path.
* NOC-03: applied schema, backup, restore, and migration-drift evidence are
  outside source-only knowledge.
* NOC-05: privacy, retention, residency, deletion, legal hold, and audit
  ownership remain open policy decisions.

## Constructed visualization

### tenant-data-policy-flow

```mermaid
flowchart LR
    Request["Tenant request"] --> Membership["Active membership lookup"]
    Membership --> Context["Organization and user session settings"]
    Context --> Connection["Tenant connection or transaction"]
    Connection --> Predicates["Explicit organization predicates"]
    Connection --> RLS["Forced PostgreSQL RLS policies"]
    Predicates --> Rows[("Organization-scoped rows")]
    RLS --> Rows
```
