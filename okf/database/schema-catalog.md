---
okf_document_id: "database-schema-catalog"
title: "Database Schema Catalog"
project: "ZinharCMS"
category: "database"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
primary_sources: ["backend/migrations", "backend/src/models", "backend/src/routes", "backend/src/services"]
related_documents: ["database/entity-catalog.md", "database/relationships.md", "database/constraints-and-indexes.md"]
related_diagrams: ["database/diagrams/entity-relationship-overview.mmd"]
uncertainty_markers: ["SCHEMA_RUNTIME_STATUS_UNKNOWN SRU-01", "MIGRATION_MODEL_CONFLICT MMC-01", "MIGRATION_MODEL_CONFLICT MMC-02"]
---

# Database Schema Catalog

## Summary

| Object class | Intended final count | Notes |
| --- | ---: | --- |
| Application tables | 51 | Every table has a primary key |
| PostgreSQL enum types | 7 | One enum is extended by a later migration |
| Extensions | 3 | `pgcrypto`, `citext`, `pg_trgm` |
| Active named functions | 14 | Excludes one temporary setup procedure |
| Active triggers | 7 | One Marketplace trigger is redefined later |
| Distinct explicit index names | 109 | Excludes implicit PK/unique indexes |
| Intended effective RLS policies | 118 | 46 direct plus 72 expanded from migration templates |
| Forced-RLS tables | 32 | Final migration intent, not deployed-state evidence |
| Views/materialized views/explicit sequences/active procedures | 0 | None found in migrations |

The combined documented-object metric is 309: 51 tables, 7 enums, 3 extensions, 14 active functions, 7 active triggers, 109 explicit index names, and 118 intended effective policies. This excludes implicit indexes and SQLx's internal migration table. Every migration-defined object has repository status `ACTIVE` or the status noted below, but runtime presence remains `SCHEMA_RUNTIME_STATUS_UNKNOWN SRU-01`.

## Table Catalog

The `Migration(s)` column gives the creation migration followed by notable later alteration groups. `Scope` is architectural intent, not proof of RLS correctness. Query ownership is summarized at module level; see the entity documents for code paths and lifecycle details.

| Table | Migration(s) | Domain / owner | Purpose and scope | Model/query mapping | Entity document |
| --- | --- | --- | --- | --- | --- |
| `users` | 0001; 0008, 0012 | Identity / auth | Global user identity; security-sensitive | shared user/auth queries | identity-and-global-rbac |
| `roles` | 0001; seed updates | Access / auth | Global named role and permission arrays | role models/auth | identity-and-global-rbac |
| `user_roles` | 0001 | Access / auth | User-to-role join | auth queries | identity-and-global-rbac |
| `refresh_tokens` | 0001; 0007 | Identity / auth | Refresh-token lineage and revocation | auth queries | identity-and-global-rbac |
| `login_attempts` | 0007 | Security / auth | Login security history | auth queries | identity-and-global-rbac |
| `organizations` | 0008; 0012 | Tenancy / organizations | Tenant root and organization lifecycle | organization rows/queries | organizations-and-membership |
| `organization_members` | 0008 | Tenancy / organizations | Tenant membership and role | organization rows/queries | organizations-and-membership |
| `organization_invitations` | 0008 | Tenancy / organizations | Invitation lifecycle | organization rows/queries | organizations-and-membership |
| `content_types` | 0001; 0008, 0009 | CMS / content | Tenant content schema definitions; forced RLS | content models/direct SQL | content-types-and-entries |
| `content_entries` | 0001; 0003, 0006, 0008, 0009 | CMS / content | Versioned publishable content; forced RLS | content models/direct SQL | content-types-and-entries |
| `pages` | 0001; 0004, 0006, 0008, 0009 | CMS / pages | Tenant pages and builder document; forced RLS | page models/direct SQL | pages-and-versions |
| `page_versions` | 0001; 0008, 0009 | CMS / pages | Immutable-style page snapshots; forced RLS | page-local row types | pages-and-versions |
| `component_registry` | 0001; 0004, 0008, 0009 | CMS / pages/plugins | System and tenant component definitions; forced RLS | shared partial model/pages | component-and-plugin-registry |
| `media` | 0001; 0008, 0009 | Media / media | Tenant media metadata; forced RLS | media models/routes | media-and-variants |
| `media_variants` | 0001; 0003, 0008, 0009 | Media / media | Derived media metadata; forced RLS | media models/routes | media-and-variants |
| `comments` | 0006; 0008, 0009 | Editorial / comments | Polymorphic content/page comments; forced RLS | route-local SQL | editorial-comments |
| `cms_plugins` | 0006 | Plugins / plugins | Built-in CMS plugin registry | plugin queries | component-and-plugin-registry |
| `webhooks` | 0005; 0008, 0009 | Delivery / webhooks | Tenant outbound webhook subscriptions; forced RLS | delivery queries | cms-webhooks-and-deliveries |
| `webhook_deliveries` | 0005; 0008, 0009 | Delivery / webhooks | Delivery-attempt history; forced RLS | delivery queries | cms-webhooks-and-deliveries |
| `public_settings` | 0005; 0008, 0009 | Delivery / settings | Tenant public key/value settings; forced RLS | delivery/settings SQL | public-settings-and-navigation |
| `navigation_items` | 0005; 0008, 0009 | Delivery / settings | Tenant ordered navigation tree; forced RLS | delivery/settings SQL | public-settings-and-navigation |
| `plans` | 0010; 0011 | Billing / billing | Global subscription plan catalog | billing queries | plans-subscriptions-and-usage |
| `organization_subscriptions` | 0010; 0011 | Billing / billing | Current tenant plan and provider linkage; forced RLS | billing queries | plans-subscriptions-and-usage |
| `usage_counters` | 0010 | Billing / billing | Tenant metric-period counters; forced RLS | billing queries | plans-subscriptions-and-usage |
| `billing_events` | 0011 | Billing / billing | Provider event idempotency/history; forced RLS | Stripe service | plans-subscriptions-and-usage |
| `organization_domains` | 0012 | SaaS / organizations | Tenant domain records; forced RLS | organization/SaaS queries | saas-operations-and-audit |
| `organization_rate_limits` | 0012 | SaaS / operations | Tenant rate-limit configuration; forced RLS | middleware/admin queries | saas-operations-and-audit |
| `audit_logs` | 0012 | SaaS / audit | Tenant audit history; forced RLS | audit service | saas-operations-and-audit |
| `email_deliveries` | 0012 | SaaS / operations | Email delivery state/history; forced RLS | organization/SaaS services | saas-operations-and-audit |
| `saas_alert_rules` | 0012 | SaaS / operations | Tenant alert definitions; forced RLS | SaaS admin queries | saas-operations-and-audit |
| `beta_participants` | 0014 | Release / beta | Tenant beta participation state; forced RLS | beta routes/services | beta-release-records |
| `beta_feedback` | 0014 | Release / beta | Tenant beta feedback; forced RLS | beta routes/services | beta-release-records |
| `beta_ga_blockers` | 0014 | Release / beta | Tenant launch blocker records; forced RLS | beta routes/services | beta-release-records |
| `marketplace_creators` | 0015; 0016 | Marketplace / creator | Global creator profile and review state | Marketplace route-local rows | marketplace-creators |
| `marketplace_listings` | 0015; 0016, 0018 | Marketplace / catalog | Global catalog listing and moderation state | Marketplace routes/services | marketplace-catalog-and-review-pipeline |
| `marketplace_versions` | 0015; 0016, 0017, 0018 | Marketplace / catalog | Package version, artifact, validation state | Marketplace routes/services | marketplace-catalog-and-review-pipeline |
| `marketplace_submissions` | 0016; 0017, 0018 | Marketplace / review | Version submission lifecycle | review services | marketplace-catalog-and-review-pipeline |
| `marketplace_review_events` | 0018 | Marketplace / review | Append-style moderation history | review services | marketplace-catalog-and-review-pipeline |
| `marketplace_installations` | 0019; 0020, 0021 | Marketplace / installation | Tenant installation lifecycle; forced RLS | installation services | marketplace-installations-and-runtime-adapters |
| `marketplace_permission_catalog` | 0020 | Marketplace / runtime | Global permission definitions | Marketplace policy services | marketplace-installations-and-runtime-adapters |
| `marketplace_kill_switches` | 0020 | Marketplace / runtime | Global or organization runtime block controls; forced RLS | kill-switch services | marketplace-installations-and-runtime-adapters |
| `marketplace_template_imports` | 0021 | Marketplace / adapters | Tenant template import mapping; forced RLS | adapter services | marketplace-installations-and-runtime-adapters |
| `marketplace_plugin_hooks` | 0021 | Marketplace / adapters | Tenant plugin hook registration; forced RLS | adapter services | marketplace-installations-and-runtime-adapters |
| `marketplace_purchases` | 0022; 0023 | Marketplace / finance | Tenant purchase/provider lifecycle; forced RLS | finance services | marketplace-purchases-and-entitlements |
| `marketplace_entitlements` | 0022; 0023 | Marketplace / finance | Tenant catalog access grant; forced RLS | finance/install services | marketplace-purchases-and-entitlements |
| `marketplace_revenue_ledger` | 0022; 0023 | Marketplace / finance | Tenant append-only revenue ledger; forced RLS | finance services | marketplace-ledger-and-payouts |
| `marketplace_payout_accounts` | 0022 | Marketplace / finance | Creator provider payout account | finance services | marketplace-ledger-and-payouts |
| `marketplace_payouts` | 0022; 0023 | Marketplace / finance | Creator payout batch/state | finance services | marketplace-ledger-and-payouts |
| `marketplace_product_reviews` | 0024 | Marketplace / feedback | Tenant/listing review; forced RLS | review services | marketplace-reviews-and-abuse |
| `marketplace_abuse_reports` | 0024 | Marketplace / trust | Tenant abuse report lifecycle; forced RLS | abuse services | marketplace-reviews-and-abuse |
| `marketplace_internal_notifications` | 0025 | Marketplace / trust | Internal moderation notification | abuse/operations services | marketplace-reviews-and-abuse |

All table entries are `ACTIVE` in intended migration history with high repository confidence and `RUNTIME_STATUS_UNKNOWN` in deployed environments.

## Types and Extensions

| Object | Created/modified | Purpose | Status |
| --- | --- | --- | --- |
| `content_status` | 0001 | Content publication state | Active intent; runtime unknown |
| `page_status` | 0001; adds `pending_review` in 0006 | Page publication/workflow state | Active intent; `MMC-01` model conflict |
| `organization_status` | 0008 | Organization lifecycle | Active intent |
| `organization_member_role` | 0008 | Membership role | Active intent |
| `organization_member_status` | 0008 | Membership state | Active intent |
| `organization_invitation_status` | 0008 | Invitation state | Active intent |
| `organization_subscription_status` | 0010 | Subscription state | Active intent |
| `pgcrypto` | migrations | UUID/cryptographic database capability | Active intent |
| `citext` | migrations | Case-insensitive text capability | Active intent |
| `pg_trgm` | migrations | Trigram matching/index capability | Active intent |

Other status columns commonly use constrained text rather than custom types. Application enums must therefore not be assumed complete without checking both constraints and queries.

## Functions and Triggers

Active function names are `app_default_organization_id`, five child-tenant setter functions, `app_current_organization_id`, `app_current_user_id`, `app_rls_bypass_enabled`, `app_rls_tenant_matches`, `app_rls_component_select`, `app_rls_component_write`, `marketplace_prevent_version_artifact_mutation`, and `marketplace_revenue_ledger_append_only`. The five setters serve content entries, page versions, media variants, comments, and webhook deliveries. The matching active triggers are `trg_content_entries_set_organization_id`, `trg_page_versions_set_organization_id`, `trg_media_variants_set_organization_id`, `trg_comments_set_organization_id`, and `trg_webhook_deliveries_set_organization_id`, plus `trg_marketplace_versions_immutable` and `trg_marketplace_revenue_ledger_append_only`.

`app_enable_tenant_rls` is a temporary migration-time procedure that is dropped in its migration and is not an active stored procedure. No views, materialized views, explicit sequences, or persistent stored procedures were found.

## Namespace and Runtime Caveat

No named application schema is created and SQL generally uses unqualified object names. Default PostgreSQL namespace resolution is therefore inferred from configuration and SQL. Migration history describes intended schema, not evidence of a particular deployed database; verify `_sqlx_migrations` and catalog state before operational work.
