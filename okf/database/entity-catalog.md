---
okf_document_id: "database-entity-catalog"
title: "Database Entity Catalog"
project: "ZinharCMS"
category: "database"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "mixed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources: ["backend/migrations", "backend/src/models", "backend/src/routes", "backend/src/services"]
related_documents: ["database/schema-catalog.md", "database/relationships.md", "database/module-data-ownership.md"]
related_diagrams: ["database/diagrams/database-domain-map.mmd"]
uncertainty_markers: ["ENTITY_OWNERSHIP_UNCLEAR EOU-01", "SOFT_DELETE_BEHAVIOR_UNCLEAR SDBU-01"]
---

# Database Entity Catalog

## Phase 6 API Cross-Reference

Every Phase 6 endpoint-family document links to its primary entity documentation, and [API Backend Module Map](../api/backend-module-map.md) provides the complete route-to-module-to-persistence map. API access zone and organization role do not replace database isolation: tenant handlers should use `TenantContext` and the RLS helpers documented in [Multi-Tenancy](multi-tenancy.md), while public, authentication, callback, and global-administration paths require explicit non-tenant justification.

The API catalog does not claim one endpoint maps to only one table. Marketplace, billing, workflow, auditing, webhook, and media operations commonly cross aggregate and side-effect boundaries; the endpoint-family and backend module documents identify those review paths.

## Catalog

The catalog groups 51 technical tables into 18 significant domain entities. `Tenant` means organization-scoped storage or behavior; `Mixed` includes both system/global and tenant rows; `Global` is not isolated by tenant RLS. All entries are implemented in migration history with high repository confidence and runtime status unknown.

| ID | Entity | Objects and model mapping | Owner and responsibility | Tenant / relationships | Lifecycle, audit, and security | Document |
| --- | --- | --- | --- | --- | --- | --- |
| DB-ENT-001 | Identity and Global RBAC | `users`, `roles`, `user_roles`, `refresh_tokens`, `login_attempts`; shared auth/user types | Auth; identity, credentials, tokens, global roles | Global; user-role and token ownership | Revocation/history; highly sensitive; hard deletion behavior unclear | [Open](entities/identity-and-global-rbac.md) |
| DB-ENT-002 | Organizations and Membership | organizations, members, invitations; organization row types | Organizations; tenant root and access membership | Mixed; organization-user-plan graph | Status-driven lifecycle and audit calls; sensitive | [Open](entities/organizations-and-membership.md) |
| DB-ENT-003 | Content Types and Entries | `content_types`, `content_entries`; shared partial models and local rows | Content; schemas and publishable records | Tenant; type-to-entry | Publication status and entry version; audit indirect | [Open](entities/content-types-and-entries.md) |
| DB-ENT-004 | Pages and Versions | `pages`, `page_versions`; partial shared page models and local rows | Pages; builder document and snapshots | Tenant; page-to-versions | Publish/archive plus snapshot history | [Open](entities/pages-and-versions.md) |
| DB-ENT-005 | Component and Plugin Registry | `component_registry`, `cms_plugins`; partial component model | Pages/plugins; component and plugin metadata | Mixed | Registry state; code/runtime security implications | [Open](entities/component-and-plugin-registry.md) |
| DB-ENT-006 | Media and Variants | `media`, `media_variants`; shared partial media models | Media; file metadata and derived assets | Tenant; media-to-variants | Mixed file/DB lifecycle; sensitive paths | [Open](entities/media-and-variants.md) |
| DB-ENT-007 | Editorial Comments | `comments`; route-local mapping | Comments/workflow; editorial discussion | Tenant; polymorphic target and author/resolver links | Resolution/edit timestamps; target FK unclear | [Open](entities/editorial-comments.md) |
| DB-ENT-008 | Public Settings and Navigation | `public_settings`, `navigation_items`; local query rows | Delivery/settings; public configuration | Tenant; navigation self-tree | Hard replacement/deletion patterns; public impact | [Open](entities/public-settings-and-navigation.md) |
| DB-ENT-009 | CMS Webhooks and Deliveries | `webhooks`, `webhook_deliveries`; delivery rows | Delivery/webhooks; subscriptions and attempts | Tenant; webhook-to-deliveries | Delivery history; secrets and payloads sensitive | [Open](entities/cms-webhooks-and-deliveries.md) |
| DB-ENT-010 | Plans, Subscriptions, and Usage | plans, subscriptions, counters, billing events | Billing; plan, metering, provider events | Mixed; organization-to-plan | Subscription status, period counters, idempotent events | [Open](entities/plans-subscriptions-and-usage.md) |
| DB-ENT-011 | SaaS Operations and Audit | domains, rate limits, audit, email, alerts | Organizations/operations/shared services | Tenant | Mostly operational history/configuration; retention unknown | [Open](entities/saas-operations-and-audit.md) |
| DB-ENT-012 | Beta Release Records | participants, feedback, blockers | Beta/release services | Tenant | Status and readiness history; no uniform soft delete | [Open](entities/beta-release-records.md) |
| DB-ENT-013 | Marketplace Creators | `marketplace_creators`; route-local rows | Marketplace creator flows | Global; required unique user linkage | Approval/provider lifecycle; PII/finance sensitivity | [Open](entities/marketplace-creators.md) |
| DB-ENT-014 | Marketplace Catalog and Review Pipeline | listings, versions, submissions, review events | Marketplace catalog/review | Global catalog graph | Moderation states, immutable artifacts, review history | [Open](entities/marketplace-catalog-and-review-pipeline.md) |
| DB-ENT-015 | Marketplace Installations and Runtime Adapters | installations, permissions, switches, imports, hooks | Marketplace installation/runtime | Mixed; tenant installation graph | Soft uninstall, runtime policy state, audit calls | [Open](entities/marketplace-installations-and-runtime-adapters.md) |
| DB-ENT-016 | Marketplace Purchases and Entitlements | purchases, entitlements | Marketplace finance/install | Tenant plus global catalog relations | Provider/payment lifecycle; security-sensitive | [Open](entities/marketplace-purchases-and-entitlements.md) |
| DB-ENT-017 | Marketplace Ledger and Payouts | ledger, payout accounts, payouts | Marketplace finance | Mixed tenant revenue and global creator payout | Append-only ledger; provider state; highly sensitive | [Open](entities/marketplace-ledger-and-payouts.md) |
| DB-ENT-018 | Marketplace Reviews and Abuse | reviews, abuse reports, internal notifications | Marketplace feedback/trust | Mixed tenant reports and internal operations | Moderation lifecycle and notification history | [Open](entities/marketplace-reviews-and-abuse.md) |

## Behavioral Coverage

- Soft deletion is not a uniform platform convention. Marketplace installation status behaves as an uninstall lifecycle, while many records are hard-deleted or archived by status (`SDBU-01`, `DLU-02`).
- Page versions, audit logs, billing events, review events, webhook deliveries, login attempts, and the Marketplace revenue ledger preserve different forms of history; they do not form one universal audit system.
- Publication is explicit for content entries and pages. Marketplace approval/publication is a separate lifecycle.
- Shared Rust models cover only part of the schema. Marketplace persistence shapes are primarily local to routes/services (`MMC-02`, `BRU-02`).

## Phase 8 Domain Cross-Reference

The 18 persistence entity groups support ten significant behavioral domains; the mapping is recorded in the [Domain Catalog](../domain/domain-catalog.md). [Domain Invariants](../domain/invariants.md) identifies 31 conditions and states which are database-enforced, application-supported, or unverified. A table or foreign key is persistence evidence, not by itself a complete business workflow.
