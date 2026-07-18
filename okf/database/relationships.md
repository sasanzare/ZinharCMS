---
okf_document_id: "database-relationships"
title: "Database Relationships"
project: "ZinharCMS"
category: "database"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
primary_sources: ["backend/migrations", "backend/src/routes", "backend/src/services"]
related_documents: ["database/schema-catalog.md", "database/entity-catalog.md", "database/multi-tenancy.md"]
related_diagrams: ["database/diagrams/entity-relationship-overview.mmd"]
uncertainty_markers: ["RELATIONSHIP_UNCLEAR RLU-01", "RELATIONSHIP_UNCLEAR RLU-02", "CONSTRAINT_COVERAGE_UNCLEAR CCU-01"]
---

# Database Relationships

## Relationship Baseline

The 26 migrations contain 110 textual `REFERENCES` definitions across history. The repeated `marketplace_submissions.submitted_by` `ADD COLUMN IF NOT EXISTS` definition is a no-op because migration 0015 already creates that column, and the direct `marketplace_installations.rollback_version_id` FK is later dropped and replaced by a composite listing/version FK. The resulting migration intent has 108 active foreign-key constraints. The matrix below consolidates them into 55 significant relationship groups; it is a navigation aid, not a replacement for constraint SQL.

## Major Relationship Matrix

| # | From | To | Cardinality / optionality | Enforcement and lifecycle note |
| ---: | --- | --- | --- | --- |
| 1 | `user_roles.user_id` | `users.id` | many-to-one | Composite join PK; delete cascades |
| 2 | `user_roles.role_id` | `roles.id` | many-to-one | Global RBAC join; delete cascades |
| 3 | `refresh_tokens.user_id` | `users.id` | many-to-one | Token ownership; delete cascades |
| 4 | `organization_members.organization_id` | `organizations.id` | many-to-one | Composite membership key; delete cascades |
| 5 | `organization_members.user_id` | `users.id` | many-to-one | User may join multiple organizations; delete cascades |
| 6 | `organization_invitations.organization_id` | `organizations.id` | many-to-one | Tenant invitation; delete cascades |
| 7 | invitation actor columns | `users.id` | optional many-to-one | `invited_by` deletes set null |
| 8 | `content_types.organization_id` | `organizations.id` | many-to-one | Non-null tenant FK; delete cascades |
| 9 | `content_entries.organization_id` | `organizations.id` | many-to-one | Non-null tenant FK and forced RLS |
| 10 | `content_entries.type_id` | `content_types.id` | many-to-one | Non-null schema parent; delete cascades |
| 11 | `pages.organization_id` | `organizations.id` | many-to-one | Non-null tenant FK; delete cascades |
| 12 | `page_versions.organization_id` | `organizations.id` | many-to-one | Non-null tenant FK and forced RLS |
| 13 | `page_versions.page_id` | `pages.id` | many-to-one | Non-null snapshot parent; delete cascades |
| 14 | `component_registry.organization_id` | `organizations.id` | optional many-to-one | Null denotes system component; delete cascades for tenant rows |
| 15 | `media.organization_id` | `organizations.id` | many-to-one | Non-null tenant FK; delete cascades |
| 16 | `media_variants.organization_id` | `organizations.id` | many-to-one | Non-null derived tenant FK; delete cascades |
| 17 | `media_variants.media_id` | `media.id` | many-to-one | Non-null parent; delete cascades |
| 18 | `comments.organization_id` | `organizations.id` | many-to-one | Non-null tenant FK and forced RLS |
| 19 | `comments.author_id` / `resolved_by` | `users.id` | optional many-to-one | User deletion sets actor references null |
| 20 | `webhooks.organization_id` | `organizations.id` | many-to-one | Non-null tenant FK; delete cascades |
| 21 | `webhook_deliveries.organization_id` | `organizations.id` | many-to-one | Non-null derived tenant FK; delete cascades |
| 22 | `webhook_deliveries.webhook_id` | `webhooks.id` | many-to-one | Non-null attempt parent; delete cascades |
| 23 | `public_settings.organization_id` | `organizations.id` | many-to-one | Organization/key composite PK; delete cascades |
| 24 | `navigation_items.organization_id` | `organizations.id` | many-to-one | Non-null tenant FK; delete cascades |
| 25 | `navigation_items.parent_id` | `navigation_items.id` | optional self-reference | Tree parent; delete cascades |
| 26 | `organization_subscriptions.organization_id` | `organizations.id` | one-to-one current row | Organization ID is PK; delete cascades |
| 27 | `organization_subscriptions.plan_id` | `plans.id` | many-to-one | Non-null plan; default delete action |
| 28 | `usage_counters.organization_id` | `organizations.id` | many-to-one | Non-null tenant/period/metric row; delete cascades |
| 29 | `billing_events.organization_id` | `organizations.id` | optional many-to-one | Organization deletion sets null |
| 30 | billing provider/subscription identifiers | external/provider state | optional logical | No local subscription FK |
| 31 | `organization_domains.organization_id` | `organizations.id` | many-to-one | Non-null; delete cascades |
| 32 | `organization_rate_limits.organization_id` | `organizations.id` | one-to-one | Organization ID is PK; delete cascades |
| 33 | `audit_logs.organization_id` | `organizations.id` | many-to-one | Non-null tenant stream; delete cascades |
| 34 | `audit_logs.actor_id` | `users.id` | optional many-to-one | User deletion sets actor null |
| 35 | `email_deliveries.organization_id` | `organizations.id` | optional many-to-one | Organization deletion sets null |
| 36 | `saas_alert_rules.organization_id` | `organizations.id` | many-to-one | Non-null tenant rule; delete cascades |
| 37 | beta participant/feedback/blocker organization FKs | `organizations.id` | one-to-one/many-to-one | Non-null; delete cascades |
| 38 | `marketplace_creators.user_id` | `users.id` | one-to-one | Non-null unique FK; delete cascades |
| 39 | `marketplace_listings.creator_id` | `marketplace_creators.id` | many-to-one | Non-null; delete cascades |
| 40 | `marketplace_versions.listing_id` | `marketplace_listings.id` | many-to-one | Non-null; delete cascades |
| 41 | `marketplace_submissions.version_id` | `marketplace_versions.id` | many-to-one | Non-null; delete cascades |
| 42 | `marketplace_review_events.submission_id` | `marketplace_submissions.id` | optional many-to-one | Submission deletion sets null |
| 43 | `marketplace_installations.organization_id` | `organizations.id` | many-to-one | Non-null forced-RLS tenant FK; delete cascades |
| 44 | installation listing/version FKs | listings and versions | many-to-one | Listing cascades, version restricts, composite pair enforced |
| 45 | `marketplace_kill_switches.organization_id` | `organizations.id` | optional many-to-one | Null for global scope; organization delete cascades |
| 46 | template-import organization/installation/page FKs | organization/install/page | many-to-one | Cascades for organization/page; installation restricts |
| 47 | plugin-hook organization/installation FKs | organization/install | many-to-one | Both non-null; deletes cascade |
| 48 | purchase organization/buyer/listing/version FKs | organization/user/catalog | many-to-one | Buyer optional/set null; catalog restricts; composite pair enforced |
| 49 | entitlement organization/purchase/listing/version FKs | organization/purchase/catalog | many-to-one | Organization cascades; purchase/catalog restrict; composite pair enforced |
| 50 | ledger organization/purchase/creator/listing FKs | tenant/finance/catalog | many-to-one | Organization cascades; other parents restrict |
| 51 | `marketplace_payout_accounts.creator_id` | `marketplace_creators.id` | one-to-one | Non-null unique FK; creator delete cascades |
| 52 | `marketplace_payouts.creator_id` | `marketplace_creators.id` | many-to-one | Non-null; creator delete restricts |
| 53 | product-review organization/listing/version/author FKs | tenant/catalog/user | many-to-one | Version optional/set null; listing/org cascade; author restricts |
| 54 | abuse-report organization/listing/version/reporter FKs | tenant/catalog/user | many-to-one | Version optional/set null; listing/org cascade; reporter restricts |
| 55 | `marketplace_internal_notifications.abuse_report_id` | `marketplace_abuse_reports.id` | one-to-one | Non-null unique FK; report delete cascades |

## Significant Relationship Constraint Detail

All explicit FKs use PostgreSQL's default `NO ACTION` update behavior because no migration specifies `ON UPDATE`. The owning side is the table containing the FK. Evidence is the final migration definition; confidence is high unless a row is marked logical.

| Source and FK column(s) | Referenced key | Nullability | Delete / update | Status | Tenant implication | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| `user_roles.user_id` | `users.id` | Non-null | CASCADE / NO ACTION | `ENFORCED_BY_FOREIGN_KEY` | Global join | 0001 |
| `user_roles.role_id` | `roles.id` | Non-null | CASCADE / NO ACTION | `ENFORCED_BY_FOREIGN_KEY` | Global join | 0001 |
| `organization_members.(organization_id,user_id)` | `organizations.id`, `users.id` | Non-null | CASCADE / NO ACTION | `ENFORCED_BY_FOREIGN_KEY` | Membership control plane | 0008 |
| `content_entries.(organization_id,type_id)` | `organizations.id`, `content_types.id` | Non-null | CASCADE / NO ACTION | `ENFORCED_BY_FOREIGN_KEY` | Separate FKs do not prove same tenant (`CCU-01`) | 0001, 0008 |
| `page_versions.(organization_id,page_id)` | `organizations.id`, `pages.id` | Non-null | CASCADE / NO ACTION | `ENFORCED_BY_FOREIGN_KEY` | Separate FKs plus propagation trigger | 0001, 0008 |
| `component_registry.organization_id` | `organizations.id` | Nullable | CASCADE / NO ACTION | `ENFORCED_BY_FOREIGN_KEY` | Null is system scope | 0008 |
| `media_variants.(organization_id,media_id)` | `organizations.id`, `media.id` | Non-null | CASCADE / NO ACTION | `ENFORCED_BY_FOREIGN_KEY` | Separate FKs plus propagation trigger | 0001, 0008 |
| `comments.organization_id` | `organizations.id` | Non-null | CASCADE / NO ACTION | `ENFORCED_BY_FOREIGN_KEY` | Forced RLS | 0008 |
| `comments.(entity_type,entity_id)` | Entry or page logical ID | Non-null | Application behavior | `RELATIONSHIP_UNCLEAR` | Target tenant validation is not a conventional FK | 0006; comment queries |
| `navigation_items.parent_id` | `navigation_items.id` | Nullable | CASCADE / NO ACTION | `ENFORCED_BY_FOREIGN_KEY` | No composite same-tenant parent FK | 0005 |
| `organization_subscriptions.(organization_id,plan_id)` | `organizations.id`, `plans.id` | Non-null | CASCADE/default / NO ACTION | `ENFORCED_BY_FOREIGN_KEY` | Tenant subscription to global plan | 0010 |
| `billing_events.organization_id` | `organizations.id` | Nullable | SET NULL / NO ACTION | `ENFORCED_BY_FOREIGN_KEY` | Historical provider event can outlive tenant | 0011 |
| `audit_logs.(organization_id,actor_id)` | `organizations.id`, `users.id` | Non-null/nullable | CASCADE/SET NULL / NO ACTION | `ENFORCED_BY_FOREIGN_KEY` | Tenant stream with optional actor | 0012 |
| `audit_logs.(entity_type,entity_id)` | Logical heterogeneous subject | Text/UUID pair | Application behavior | `RELATIONSHIP_UNCLEAR` | Subject tenant is not FK-proven | 0012; audit service |
| `marketplace_creators.user_id` | `users.id` | Non-null unique | CASCADE / NO ACTION | `ENFORCED_BY_FOREIGN_KEY` | Global creator identity | 0015 |
| `marketplace_versions.listing_id` | `marketplace_listings.id` | Non-null | CASCADE / NO ACTION | `ENFORCED_BY_FOREIGN_KEY` | Global catalog | 0015 |
| `marketplace_installations.(listing_id,version_id)` | `marketplace_versions.(listing_id,id)` | Non-null | Default / NO ACTION | `ENFORCED_BY_FOREIGN_KEY` | Prevents cross-listing version selection | 0015, 0019 |
| `marketplace_template_imports.(organization_id,installation_id,page_id)` | Organization/install/page PKs | Non-null | CASCADE/RESTRICT/CASCADE / NO ACTION | `ENFORCED_BY_FOREIGN_KEY` | Same-tenant coherence still needs tests | 0021 |
| `marketplace_purchases.(listing_id,version_id)` | `marketplace_versions.(listing_id,id)` | Non-null | Default / NO ACTION | `ENFORCED_BY_FOREIGN_KEY` | Tenant purchase to global catalog | 0022 |
| `marketplace_entitlements.purchase_id` | `marketplace_purchases.id` | Non-null unique | RESTRICT / NO ACTION | `ENFORCED_BY_FOREIGN_KEY` | Tenant/purchase coherence requires tests | 0022 |
| `marketplace_revenue_ledger.purchase_id` | `marketplace_purchases.id` | Non-null | RESTRICT / NO ACTION | `ENFORCED_BY_FOREIGN_KEY` | Append-only tenant finance row | 0022, 0023 |
| `marketplace_product_reviews.(organization_id,listing_id,version_id,author_id)` | Organization/catalog/user PKs | Version nullable; others non-null | CASCADE/CASCADE/SET NULL/RESTRICT / NO ACTION | `ENFORCED_BY_FOREIGN_KEY` | Tenant review references global catalog | 0024 |
| `marketplace_abuse_reports.(organization_id,listing_id,version_id,reporter_id)` | Organization/catalog/user PKs | Version nullable; others non-null | CASCADE/CASCADE/SET NULL/RESTRICT / NO ACTION | `ENFORCED_BY_FOREIGN_KEY` | Tenant report references global catalog | 0024 |
| `marketplace_internal_notifications.abuse_report_id` | `marketplace_abuse_reports.id` | Non-null unique | CASCADE / NO ACTION | `ENFORCED_BY_FOREIGN_KEY` | Global operational child of tenant report | 0025 |

## Logical and Polymorphic Relationships

`comments(entity_type, entity_id)` targets content entries or pages without a conventional foreign key. Validation and authorization are application/trigger concerns, so target integrity is `RELATIONSHIP_UNCLEAR RLU-01`. `audit_logs(entity_type, entity_id)` records heterogeneous subjects without a foreign key; historical target existence is `RELATIONSHIP_UNCLEAR RLU-02`.

JSONB documents contain additional logical references, particularly builder content, Marketplace manifests, permissions, provider metadata, and audit payloads. Those values are not equivalent to FK-enforced relationships.

## Cascades and Tenant Coherence

Deletion actions differ by constraint and must be inspected in the relevant migration before modification. Parent and tenant links frequently coexist on a child row, but separate FKs do not prove that both parents belong to the same organization. Setter triggers populate selected child `organization_id` values in some cases; an explicitly supplied value may not receive equivalent cross-parent validation. This is `CONSTRAINT_COVERAGE_UNCLEAR CCU-01` and `TENANT_ISOLATION_UNVERIFIED TIV-01`.

## Related Documents

Use the [Entity Catalog](entity-catalog.md) and its individual entity documents for field-level context. The [Entity Relationship Overview](diagrams/entity-relationship-overview.mmd) visualizes the significant graph; [Multi-Tenancy](multi-tenancy.md) explains organization-context enforcement.
