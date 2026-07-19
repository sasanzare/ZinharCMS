---
okf_document_id: "database-multi-tenancy"
title: "Database Multi-Tenancy"
project: "ZinharCMS"
category: "database"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources: ["backend/migrations/0008_v2_phase_one_organizations.sql", "backend/migrations/0009_v2_phase_three_rls.sql", "backend/migrations", "backend/src/middleware/tenant.rs", "backend/src/services/rls.rs", "backend/src/services/hardening.rs"]
related_documents: ["database/relationships.md", "database/constraints-and-indexes.md", "architecture/boundaries.md"]
related_diagrams: ["database/diagrams/tenant-isolation.mmd"]
uncertainty_markers: ["TENANT_ISOLATION_UNVERIFIED TIV-01", "CONSTRAINT_COVERAGE_UNCLEAR CCU-01", "DOCUMENTATION_CODE_CONFLICT DCC-12", "UNKNOWN U-08"]
---

# Database Multi-Tenancy

## Tenancy Model

`organizations` is the tenant root. ZinharCMS uses one shared database and shared namespace. Tenant rows carry `organization_id`; request middleware verifies an active organization and active membership; queries commonly add explicit organization predicates; and 32 tables use forced RLS. These layers are complementary. The presence of `organization_id` alone is not proof of isolation.

## Database Session Context

RLS helpers use these custom PostgreSQL settings:

- `zinhar.organization_id`
- `zinhar.user_id`
- `zinhar.rls_bypass`

`app_current_organization_id`, `app_current_user_id`, `app_rls_bypass_enabled`, and matching predicate functions read these values. A tenant transaction sets transaction-local context. The connection-level tenant helper sets session context and marks the connection `close_on_drop`, preventing a context-bearing connection from returning to the general pool. Privileged bypass transactions deliberately disable tenant filtering for global/admin/catalog/provider flows and require narrow review.

## Forced-RLS Inventory

The final migration intent forces RLS on 32 tables:

`audit_logs`, `beta_feedback`, `beta_ga_blockers`, `beta_participants`, `billing_events`, `comments`, `component_registry`, `content_entries`, `content_types`, `email_deliveries`, `marketplace_abuse_reports`, `marketplace_entitlements`, `marketplace_installations`, `marketplace_kill_switches`, `marketplace_plugin_hooks`, `marketplace_product_reviews`, `marketplace_purchases`, `marketplace_revenue_ledger`, `marketplace_template_imports`, `media`, `media_variants`, `navigation_items`, `organization_domains`, `organization_rate_limits`, `organization_subscriptions`, `page_versions`, `pages`, `public_settings`, `saas_alert_rules`, `usage_counters`, `webhook_deliveries`, and `webhooks`.

Migration source contains 46 direct policy definitions plus templates that expand to 72 effective policies: 44 in 0009, 16 in 0012, and 12 in 0014. The intended total is 118 effective policies. This is a source-derived metric, not a runtime catalog query.

`component_registry` has mixed semantics: system rows are globally selectable while tenant custom rows are organization-scoped. Global identity, membership, organization, plan, Marketplace catalog/creator, payout, and selected operational tables rely on application or privileged-path authorization rather than uniform tenant RLS.

## Entity Scope and Isolation Matrix

| Entity group | Classification | Tenant column / relationship | Query and write enforcement | Tenant-qualified constraint note | Test evidence / isolation confidence |
| --- | --- | --- | --- | --- | --- |
| Identity and Global RBAC | `GLOBAL` | None | Auth/global role logic; no tenant RLS | Not applicable | No DB integration harness; `APPLICATION_ENFORCED` |
| Organizations and Membership | `TENANT_RELATED` | Organization is root; membership composite key | Membership/status checks in middleware/services | `(organization_id, user_id)` membership PK | Manual/static evidence; `APPLICATION_ENFORCED` |
| Content Types and Entries | `TENANT_SCOPED` | `organization_id` | Explicit predicates, tenant transaction, forced RLS, entry propagation trigger | Type/page slugs are tenant-qualified; parent coherence `CCU-01` | Manual fixture is partial; `PARTIALLY_ENFORCED` |
| Pages and Versions | `TENANT_SCOPED` | `organization_id` | Explicit predicates, forced RLS, version propagation trigger | Page slug tenant-qualified; version parent coherence unproven | No broad runtime suite; `PARTIALLY_ENFORCED` |
| Component and Plugin Registry | `MIXED_SCOPE` | Nullable component `organization_id`; plugins global | Mixed system/tenant RLS helper policies; plugin app authorization | Custom component requires organization | Static policy evidence; `PARTIALLY_ENFORCED` |
| Media and Variants | `TENANT_SCOPED` | `organization_id` | Explicit filters, forced RLS, variant propagation trigger | Separate variant parent/tenant FKs | No file/DB isolation suite; `PARTIALLY_ENFORCED` |
| Editorial Comments | `TENANT_SCOPED` | `organization_id` plus logical target | Target authorization, forced RLS, propagation trigger | Polymorphic target is not FK-enforced | No cross-target tenant suite; `TENANT_ISOLATION_UNVERIFIED` |
| Public Settings and Navigation | `TENANT_SCOPED` | `organization_id` | Forced RLS and delivery queries | Setting PK and navigation indexes are tenant-qualified | Public tenant selection unknown; `PARTIALLY_ENFORCED` |
| CMS Webhooks and Deliveries | `TENANT_SCOPED` | `organization_id` | Forced RLS, tenant queries, delivery propagation trigger | Delivery parent/tenant FKs are separate | No durable/cross-tenant suite; `PARTIALLY_ENFORCED` |
| Plans, Subscriptions, and Usage | `MIXED_SCOPE` | Plans global; other rows use organization | Forced RLS on subscription/usage/events; provider bypass paths | Usage uniqueness is tenant-period-metric | Provider/RLS tests incomplete; `PARTIALLY_ENFORCED` |
| SaaS Operations and Audit | `TENANT_SCOPED` | `organization_id`, nullable for selected history | Forced RLS and admin/tenant services | Domain/alert uniqueness is tenant-aware where defined | No comprehensive suite; `PARTIALLY_ENFORCED` |
| Beta Release Records | `TENANT_SCOPED` | `organization_id` | Tenant/bypass transactions and forced RLS | Participant organization is PK | Static policy evidence only; `TENANT_ISOLATION_UNVERIFIED` |
| Marketplace Creators | `GLOBAL` | None | Creator/admin authorization and bypass paths | User/slug uniqueness is global | No dedicated DB suite; `APPLICATION_ENFORCED` |
| Marketplace Catalog and Review Pipeline | `GLOBAL` | None | Catalog state and privileged creator/reviewer paths | Listing/version relationships are global | No dedicated DB suite; `APPLICATION_ENFORCED` |
| Marketplace Installations and Runtime Adapters | `MIXED_SCOPE` | Tenant rows use `organization_id`; permission catalog global; kill switch can be global | Forced RLS on tenant/runtime rows, tenant transactions, bypass for global policy | Active install/hook/switch uniqueness is scoped | Hardening list drift `DCC-12`; `PARTIALLY_ENFORCED` |
| Marketplace Purchases and Entitlements | `TENANT_SCOPED` | `organization_id` | Tenant/bypass finance transactions and forced RLS | Active entitlement is organization/listing-qualified | Same-tenant graph tests absent; `TENANT_ISOLATION_UNVERIFIED` |
| Marketplace Ledger and Payouts | `MIXED_SCOPE` | Ledger tenant; payout account/payout creator-global | Ledger forced RLS; finance bypass/application authorization for payouts | Ledger provider/purchase idempotency | No finance DB concurrency suite; `PARTIALLY_ENFORCED` |
| Marketplace Reviews and Abuse | `MIXED_SCOPE` | Reviews/reports tenant; internal notifications global operational | Forced RLS on reviews/reports; privileged notifications | Review uniqueness is organization/listing | Hardening list drift and no broad suite; `PARTIALLY_ENFORCED` |

`DATABASE_ENFORCED` is not assigned to an entire entity group because Phase 5 did not execute isolation tests and application/global paths remain part of the contract. Individual constraints and RLS policies are database-enforced in migration intent.

## Tenant Propagation and Coherence

Triggers derive `organization_id` for content entries, page versions, media variants, comments, and webhook deliveries when appropriate. However, several child tables retain separate tenant and parent foreign keys rather than a composite same-tenant relationship. Explicitly supplied tenant values may not be validated against every parent. Same-tenant coherence is therefore `TENANT_ISOLATION_UNVERIFIED TIV-01` and `CONSTRAINT_COVERAGE_UNCLEAR CCU-01` until database tests prove each write path.

## Verification Conflict

`backend/src/services/hardening.rs` lists 24 tenant-RLS tables and migrations only through 0015, while migration history intends 32 forced-RLS tables. The omitted later tables are `marketplace_kill_switches`, `marketplace_template_imports`, `marketplace_plugin_hooks`, `marketplace_purchases`, `marketplace_entitlements`, `marketplace_revenue_ledger`, `marketplace_product_reviews`, and `marketplace_abuse_reports`. This is `DOCUMENTATION_CODE_CONFLICT DCC-12`: the verification helper is incomplete relative to current migrations, not proof that runtime RLS is absent.

## Safe Change Checklist

1. Establish whether the table is global, tenant-only, or mixed.
2. Verify membership/role checks and explicit organization predicates.
3. Review all RLS policies, including `WITH CHECK` behavior.
4. Verify transaction/session context and bypass callers.
5. Prove parent-child tenant coherence.
6. Test cross-tenant reads, inserts, updates, deletes, and pooled-connection reuse.
7. Confirm public tenant selection separately (`UNKNOWN U-08`).

## Phase 7 Access-Control Interpretation

[Tenant Access Control](../security/tenant-access-control.md) confirms that normal tenant requests require an active organization and active membership before role checks and tenant SQL context. Global `super_admin` does not bypass this middleware. Explicit `zinhar.rls_bypass` transactions are a separate privileged backend path. Live cross-tenant verification remains `TENANT_ACCESS_UNVERIFIED TAV-01`.

## Phase 8 Tenant Workflows

[Multi-Tenancy Behavior](../domain/multi-tenancy-behavior.md) and [Membership and Ownership](../domain/membership-and-ownership.md) connect tenant middleware/RLS to provisioning, invitations, membership changes, last-owner checks, and ownership transfer. Provisioning and transfer use database transactions; last-owner checks are application-level and not verified under concurrency. Public delivery currently selects an active organization with slug `default`, so host/domain tenant routing remains `TENANT_BEHAVIOR_UNCLEAR DTBU-01`.
