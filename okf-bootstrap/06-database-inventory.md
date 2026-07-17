# Database Inventory

## Database Platform and Migration Strategy

The application uses PostgreSQL 16 in the repository-provided Docker Compose environment and SQLx 0.8 from the Rust backend. Backend startup runs the embedded SQLx migrations before serving requests. The repository contains 26 forward-only migration files, numbered 0001 through 0026. No down migrations are present.

Migration files are the source of truth for intended schema state. The schema version and drift of every deployed environment are UNKNOWN.

## Schema Metrics

| Item | Observed count | Notes |
| --- | ---: | --- |
| Migration files | 26 | 0001 through 0026 |
| Created tables | 51 | Includes global, tenant, operations, and Marketplace tables |
| Explicit PostgreSQL enum types | 7 | Additional state values are stored as text or constrained values |
| CREATE INDEX occurrences | 110 | Includes regular and unique indexes across migrations |
| CREATE POLICY source occurrences | 58 | 46 direct statements plus 12 dynamic SQL template lines; this is not a count of distinct effective policies |
| Trigger creations | 8 | Primarily timestamp and integrity behavior |
| Function creations | 15 | Includes RLS helpers, trigger functions, and migration utilities |
| Tables with forced RLS in migration intent | 32 | Tenant-owned or partially tenant-owned tables |

## Table Inventory by Domain

| Domain | Tables | Ownership and major relationships |
| --- | --- | --- |
| Identity and global access | users, roles, user_roles, refresh_tokens, login_attempts | Users own refresh tokens and role assignments; login attempts support throttling |
| Core CMS | content_types, content_entries, pages, page_versions, component_registry, media, media_variants, comments, cms_plugins, webhooks, webhook_deliveries, public_settings, navigation_items | Content and delivery entities are organization-scoped; entries depend on content types; page versions depend on pages; media variants depend on media |
| Organizations and SaaS operations | organizations, organization_members, organization_invitations, organization_domains, organization_rate_limits, organization_subscriptions, usage_counters, billing_events, audit_logs, email_deliveries, saas_alert_rules, beta_participants, beta_feedback, beta_ga_blockers, plans | Organizations are the primary tenant boundary; plans are global; operational records associate with organizations where applicable |
| Marketplace | marketplace_creators, marketplace_listings, marketplace_versions, marketplace_submissions, marketplace_installations, marketplace_review_events, marketplace_permission_catalog, marketplace_kill_switches, marketplace_template_imports, marketplace_plugin_hooks, marketplace_purchases, marketplace_entitlements, marketplace_revenue_ledger, marketplace_payout_accounts, marketplace_payouts, marketplace_product_reviews, marketplace_abuse_reports, marketplace_internal_notifications | Creator/listing/version/submission form the publication chain; installations and finance are organization-owned; feedback and moderation connect customers, listings, and global administration |

## Explicit Enum Types

| Type | Domain | Role |
| --- | --- | --- |
| content_status | Content | Entry workflow state |
| page_status | Pages | Page workflow state |
| organization_status | Tenancy | Organization lifecycle |
| organization_member_role | Authorization | Tenant member role |
| organization_member_status | Authorization | Tenant membership state |
| organization_invitation_status | Organizations | Invitation lifecycle |
| organization_subscription_status | Billing | Subscription lifecycle |

Marketplace and several operations domains use text state columns plus application validation instead of PostgreSQL enum types. The rationale for choosing enum versus text per domain is not documented.

## Relationship and Ownership Observations

- organizations is the main tenant root. Most CMS, billing, installation, entitlement, ledger, feedback, and operations rows carry organization_id directly or inherit scope through an owning entity.
- users is a global identity root. Organization membership maps users to tenant roles.
- content_entries belongs to content_types and an organization. Dynamic entry data is stored in JSONB and checked by application validation.
- page_versions belongs to pages; media_variants belongs to media; webhook_deliveries belongs to webhooks.
- Marketplace publication flows from creator to listing to version and submission. Review events record decisions separately.
- Marketplace installations connect an organization to a published listing/version. Paid installation gates depend on active entitlements.
- Purchases can create entitlements and revenue ledger entries. Refund handling reverses ledger effects and can revoke entitlements.
- Comments use an application-validated polymorphic target rather than a database foreign key to every supported entity type.

## Tenant Isolation and RLS

Migrations express forced row-level security for 32 tenant-owned or partially tenant-owned tables. Runtime helpers set PostgreSQL session-local settings for organization, user, and bypass context. Tenant middleware verifies identity and membership before handlers acquire organization-scoped connections.

The backend hardening test constant currently names 24 RLS tables and omits eight later Marketplace tables that migrations protect:

- marketplace_kill_switches;
- marketplace_template_imports;
- marketplace_plugin_hooks;
- marketplace_purchases;
- marketplace_entitlements;
- marketplace_revenue_ledger;
- marketplace_product_reviews;
- marketplace_abuse_reports.

This is a test-coverage gap, not evidence that the eight tables lack RLS. The migrations are authoritative for the observed protection.

## Indexes and Constraints

- Primary keys use UUIDs in most business tables.
- Foreign keys express most ownership chains and commonly use cascade or restricted deletion according to the domain.
- Unique indexes enforce identities such as slugs, membership combinations, and idempotency/provider keys.
- Composite indexes support tenant filtering, workflow queues, catalog queries, finance lookup, and moderation priority.
- JSONB supports content schemas/data, reports, manifests, evidence, and other flexible payloads.
- Check constraints and application validation coexist. The complete business-rule mapping is dispersed and should be consolidated in later OKF phases.

## Delete, Archive, and History Behavior

There is no uniform soft-delete convention.

- Most deletion routes perform hard deletion subject to foreign keys and service checks.
- Marketplace installation uninstall is a soft lifecycle transition.
- Content and pages provide archive-like workflow states rather than generic deleted_at columns.
- Page versions, audit logs, review events, billing events, webhook deliveries, ledger rows, and internal notifications provide domain-specific history.
- created_at and updated_at columns are common but not universal.

Retention and purge requirements are NEEDS_OWNER_CONFIRMATION because they cannot be derived from migrations alone.

## Seed and Bootstrap Behavior

Backend startup applies migrations and invokes seed behavior for baseline roles and related bootstrap data. Repository samples and development configuration support local setup. Production seed ownership, repeatability policy, and controlled data migration procedure are not documented as a single operational contract.

## Database Risks and Ambiguities

| Priority | Finding | Impact | Later OKF action |
| --- | --- | --- | --- |
| High | Deployed schema version and drift are UNKNOWN | Documentation may describe migrations not applied in an environment | Record environment verification procedure in operations docs |
| High | Backup, restore, RPO, RTO, and retention are NEEDS_OWNER_CONFIRMATION | Recovery guarantees cannot be stated | Resolve owner policy before final operations runbook |
| High | RLS hardening constant omits eight protected Marketplace tables | Future regression could escape the current static list | Expand security test inventory during implementation, outside Phase Zero |
| Medium | No down migrations | Rollback requires forward repair or database restore | Document release and recovery strategy |
| Medium | JSONB schema evolution is not consolidated | Content and Marketplace payload compatibility can drift | Document versioning and migration rules |
| Medium | Enum versus text-state policy is inconsistent | State validation is split between database and code | Record per-domain state authorities |
| Medium | Hard delete and lifecycle archive rules are domain-specific | Retention and restoration behavior may surprise operators | Produce a deletion and retention matrix |

## Recommended Final Database Documentation

The final OKF database set should include:

1. a generated schema index sourced from migrations;
2. a tenant-ownership and RLS matrix covering all 51 tables;
3. domain ER diagrams rather than one oversized diagram;
4. state and constraint catalogs with database versus application ownership;
5. migration, rollback, backup, and restore procedures;
6. retention and deletion behavior by domain;
7. a documented verification command for deployed migration state.
