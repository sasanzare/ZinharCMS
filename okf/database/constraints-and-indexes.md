---
okf_document_id: "database-constraints-indexes"
title: "Database Constraints and Indexes"
project: "ZinharCMS"
category: "database"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
primary_sources: ["backend/migrations"]
related_documents: ["database/schema-catalog.md", "database/relationships.md", "database/multi-tenancy.md"]
related_diagrams: ["database/diagrams/entity-relationship-overview.mmd"]
uncertainty_markers: ["CONSTRAINT_COVERAGE_UNCLEAR CCU-01", "CONSTRAINT_COVERAGE_UNCLEAR CCU-02", "INDEX_PURPOSE_UNCLEAR IPU-01"]
---

# Database Constraints and Indexes

## Integrity Baseline

All 51 application tables have primary keys. Migration history intends 108 active foreign-key constraints and 109 distinct explicitly named indexes. PostgreSQL also creates implicit indexes for primary keys and unique constraints; those are intentionally excluded from the explicit-index count.

## Identifier Patterns

- UUID primary keys are the dominant pattern.
- Composite primary keys: `user_roles(user_id, role_id)`, `organization_members(organization_id, user_id)`, and `public_settings(organization_id, key)`.
- Organization-keyed single-row primary keys: `organization_subscriptions.organization_id`, `organization_rate_limits.organization_id`, and `beta_participants.organization_id`.
- `marketplace_permission_catalog.permission_key` is a text primary key.
- Tenant-qualified uniqueness includes content type/page slugs, usage periods/metrics, SaaS alert keys, Marketplace hooks, reviews, active installations, entitlements, and kill switches.

## Constraint Families

| Family | Verified examples | Design consequence |
| --- | --- | --- |
| Primary and unique | identity email, tenant slugs, provider event IDs, purchase receipts | Rejects duplicates at the database boundary |
| Foreign key | organization, user, parent, listing/version, purchase/entitlement graphs | Establishes referential integrity with per-constraint delete behavior |
| Enum | content/page and organization lifecycle types | Application mappings must track DB values |
| Check | constrained text statuses, JSON/array shape, numeric/range rules | Many Marketplace lifecycles use text plus checks |
| Partial unique index | active install/entitlement/kill-switch and provider identifiers | Uniqueness depends on row state or non-null values |
| Trigger invariant | tenant propagation, version artifact immutability, append-only ledger | Writes may fail beyond visible column constraints |
| RLS policy | tenant reads/writes and mixed component behavior | Session context is part of effective authorization |

## Constraints

Constraint names and definitions below are exact migration identifiers. Business rationale is not invented; “purpose” describes the enforced technical condition. Confidence is high from migration SQL.

| Name | Table / columns | Type and purpose | Tenant-isolation role | Evidence |
| --- | --- | --- | --- | --- |
| `content_types_organization_slug_unique` | `content_types(organization_id, slug)` | Composite unique tenant slug | Prevents cross-tenant slug collision from becoming a global restriction | 0008 |
| `pages_organization_slug_unique` | `pages(organization_id, slug)` | Composite unique tenant slug | Tenant-qualified page identity | 0008 |
| `component_registry_custom_org_required` | `component_registry(is_system, organization_id)` | Check: system row or non-null tenant | Separates global system and tenant component forms | 0008 |
| `comments_entity_type` | `comments.entity_type` | Check: `entry` or `page` | Limits polymorphic target vocabulary but does not prove target tenant | 0006 |
| `marketplace_installations_listing_version_fk` | `marketplace_installations(listing_id, version_id)` | Composite FK to `marketplace_versions(listing_id, id)` | Prevents cross-listing version selection; organization coherence remains separate | 0015, replaced 0019 |
| `marketplace_purchases_receipt_unique` | `marketplace_purchases.receipt_number` | Unique receipt identity | Tenant rows cannot duplicate a receipt globally | 0022 |
| `marketplace_purchases_listing_version_fk` | purchase listing/version | Composite catalog FK | Prevents mismatched listing/version purchase | 0022 |
| `marketplace_entitlements_purchase_unique` | `marketplace_entitlements.purchase_id` | Unique one entitlement per purchase | Does not by itself prove same organization | 0022 |
| `marketplace_ledger_amounts_valid` | ledger amount/commission columns | Financial check | Applies to tenant ledger rows | 0022 |
| `marketplace_permission_catalog_key_format` | permission key | Format check | Global permission catalog integrity | 0020 |
| `marketplace_product_reviews_rating_range` | `rating` | Check 1 through 5 | Tenant review value integrity | 0024 |
| `marketplace_abuse_reports_severity_supported` | `severity` | Checked severity vocabulary | Tenant trust workflow state | 0024 |

Non-null constraints are widespread on identities, tenant FKs, status fields, and required payloads. No exclusion constraint was found. The complete constraint surface remains in the migrations; this table selects constraints with architectural consequences.

## Indexes

| Name | Table / columns or expression | Unique / partial / type | Creation | Verified purpose |
| --- | --- | --- | --- | --- |
| `idx_content_entries_org_type_status` | content entries: organization, type, status | Non-unique B-tree | 0008 | Tenant type/workflow filtering |
| `idx_pages_org_status` | pages: organization, status | Non-unique B-tree | 0008 | Tenant workflow filtering |
| `idx_page_versions_org_page` | versions: organization, page, version descending | Non-unique B-tree | 0008 | Tenant page history |
| `idx_navigation_items_org_public_locale` | `navigation_items(organization_id, locale, position)` | Non-unique partial B-tree where `is_public = TRUE` | 0008 | Tenant public navigation retrieval |
| `idx_marketplace_installations_active_listing` | `marketplace_installations(organization_id, listing_id)` | Unique partial where `status <> 'uninstalled'` | 0015 | One active installation per tenant/listing |
| `idx_marketplace_purchases_provider_checkout` | provider/checkout ID | Unique partial where checkout ID is non-null | 0022 | Provider checkout idempotency |
| `idx_marketplace_entitlements_active_listing` | organization/listing | Unique partial where status is active | 0022 | One active tenant entitlement per listing |
| `idx_marketplace_ledger_provider_event` | `marketplace_revenue_ledger(purchase_id, entry_type, provider_event_id)` | Unique partial where `provider_event_id IS NOT NULL` | 0023 | Ledger provider idempotency |
| `idx_marketplace_product_reviews_org_listing` | organization/listing | Unique B-tree | 0024 | One tenant review per listing |
| `idx_marketplace_abuse_reports_queue` | status, severity, created time | Non-unique B-tree | 0024 | Moderation queue ordering |
| `idx_marketplace_listings_title_trgm` | listing title expression | Non-unique GIN/trigram | 0026 | Catalog text search support |
| `idx_marketplace_purchases_existing_checkout` | `marketplace_purchases(organization_id, listing_id, version_id, status, created_at DESC)` | Non-unique partial where status is pending/completed | 0026 | Existing-checkout lookup path |

The inventory reviewed all 109 distinct explicit index names. Index type is B-tree unless SQL explicitly selects GIN/trigram or another method. Predicate and expression details must be copied from the exact migration before DDL work.

## Significant Index Purposes

The index set supports identity lookup, tenant scoping, ordered content/page/media access, webhook delivery history, billing provider idempotency, SaaS operations, Marketplace catalog search/filtering, review queues, active installation and entitlement lookup, ledger/payout processing, abuse workflows, and QA/performance query paths. `pg_trgm` enables trigram-oriented search support where used.

Potentially duplicate-looking pairs exist, including a user email uniqueness structure plus `idx_users_email`, tenant uniqueness plus organization-leading indexes for content types/pages, media variant uniqueness plus a media-leading index, and component-key uniqueness plus `idx_component_registry_key`. No execution-plan or workload evidence justifies removal. Their purpose is `INDEX_PURPOSE_UNCLEAR IPU-01`; optimization is outside Phase 5.

## Coverage Gaps and Cautions

Separate organization and parent foreign keys do not always prove same-tenant parentage (`CCU-01`). Trigger-derived tenant columns cover selected children but do not establish a universal composite-tenant invariant. Polymorphic comment and audit targets are not conventional FKs. Text status constraints and application enums may drift (`CCU-02`), as shown by shared page-model drift. JSONB and array checks validate selected structure but do not make embedded identifiers relational.

Before changing validation, search migrations for later constraint replacement, all query writers, SQLx mappings, RLS policies, and triggers. Do not duplicate a database invariant in code without deciding which layer owns error semantics and compatibility.

## Related Entity Documents

See [Organizations and Membership](entities/organizations-and-membership.md), [Content Types and Entries](entities/content-types-and-entries.md), [Pages and Versions](entities/pages-and-versions.md), [Marketplace Installations and Runtime Adapters](entities/marketplace-installations-and-runtime-adapters.md), [Marketplace Purchases and Entitlements](entities/marketplace-purchases-and-entitlements.md), and [Marketplace Reviews and Abuse](entities/marketplace-reviews-and-abuse.md).
