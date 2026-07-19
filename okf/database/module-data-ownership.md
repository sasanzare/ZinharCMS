---
okf_document_id: "database-module-data-ownership"
title: "Module Data Ownership"
project: "ZinharCMS"
category: "database"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "mixed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
primary_sources: ["backend/src/routes", "backend/src/services", "backend/src/middleware", "backend/migrations"]
related_documents: ["database/persistence-mapping.md", "backend/module-catalog.md", "backend/module-boundaries.md"]
related_diagrams: ["database/diagrams/module-data-ownership.mmd"]
uncertainty_markers: ["ENTITY_OWNERSHIP_UNCLEAR EOU-01", "PERSISTENCE_BOUNDARY_UNCLEAR PBU-01", "MODULE_OWNERSHIP_UNCLEAR MOU-05"]
---

# Module Data Ownership

## Ownership Rule

Ownership here means the module with the strongest verified write/lifecycle responsibility. It does not imply exclusive SQL access. Routes and services query SQLx directly, cross-cutting services write audit or billing state, and migrations remain the schema authority.

| Backend module | Owned entities/tables | Reads | Writes | Migration group | Direct SQL | Shared/cross-module access | Status | Confidence |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Auth | Identity and RBAC tables | Identity, role, token, membership context | Users, grants, tokens, attempts | 0001, 0002, 0007 | Yes | Organizations and Marketplace read users | `OBSERVED`, shared | High |
| Organizations | Organizations, members, invitations, domains | Users, plans, subscriptions, audit | Tenant root, membership, invitation, domain | 0008, 0012 | Yes | Billing initializes/reads subscription; middleware reads membership | `OBSERVED`, shared | High |
| Content/workflow | Content types, entries, comments | Users, tenant, settings/webhooks | Content schemas, entries, comments | 0001, 0003, 0006, 0008, 0009 | Yes | Delivery invalidation/webhook and audit writes | `OBSERVED` | High |
| Pages/builder | Pages, versions, component registry | Users, Marketplace installation/adapters | Pages, snapshots, components | 0001, 0004, 0006, 0008, 0009, 0021 | Yes | Preview, delivery, plugins, Marketplace adapters | `OBSERVED`, shared | High |
| Media | Media and variants | Users and tenant | Media metadata and variants | 0001, 0003, 0008, 0009 | Yes | Filesystem processing/public serving | `OBSERVED` | High |
| Plugins | CMS plugins; component registry participation | Components, pages | Plugin enable/configuration | 0004, 0006 | Yes | Pages and Marketplace adapters | `SHARED` | Medium |
| Delivery/settings | Webhooks/deliveries, settings, navigation | Content/pages, tenant | Subscriptions, attempts, public configuration | 0005, 0008, 0009 | Yes | Content/pages produce events and invalidation | `OBSERVED`, shared | High |
| Billing/SaaS | Plans, subscriptions, usage, billing events | Organizations and provider IDs | Subscription, usage, provider events | 0010, 0011 | Yes | Organizations and middleware read/write commercial state | `SHARED` | High |
| SaaS operations/audit | Rate limits, audit, email, alerts | Organization/user/domain entities | Operational configuration/history | 0012, 0013 | Yes | Most domain modules write audit; middleware reads limits | `ENTITY_OWNERSHIP_UNCLEAR` | Medium |
| Beta/release | Beta participants, feedback, blockers | Organizations/users | Beta/readiness records | 0014 | Yes | Admin and readiness services | `OBSERVED` | High |
| Marketplace creator/catalog/review | Creators, listings, versions, submissions, review events | Users and validation output | Catalog and moderation lifecycle | 0015–0018 | Yes | Runtime, finance, feedback consume catalog | `SHARED` | High |
| Marketplace installation/runtime | Installations, permission catalog, kill switches, imports, hooks | Catalog, entitlements, pages, users | Installation and runtime policy | 0019–0021 | Yes | Pages and finance cross the boundary | `SHARED` | High |
| Marketplace finance | Purchases, entitlements, ledger, payout accounts/payouts | Organizations, users, catalog, creators | Commercial and provider state | 0022, 0023 | Yes | Installation reads entitlements; Stripe writes purchase/event state | `SHARED` | High |
| Marketplace trust/feedback | Product reviews, abuse reports, internal notifications | Organizations, users, catalog | Feedback and moderation state | 0024–0026 | Yes | Analytics and audit consume/write related records | `SHARED` | High |

## Read Ownership

Read access is intentionally broader than lifecycle ownership. Identity and organization context are read across most authenticated modules; content/pages feed delivery; plans/subscriptions/usage feed quota checks; Marketplace catalog is read by installation, finance, feedback, and public catalog flows; audit and analytics consume events from many writers.

## Write Ownership

The owning module in the matrix performs the dominant create/update/delete or status-transition behavior. Cross-module writes include organization bootstrap to subscription state, Content/Pages to audit and delivery behavior, Stripe processing to billing or Marketplace finance tables, and Marketplace installation/trust workflows to audit records.

## Migration Ownership

Migration filenames follow product phases rather than enforced module ownership. The matrix maps each migration group to the strongest current module responsibility, but repository governance does not identify a formal schema-change owner (`NEEDS_OWNER_CONFIRMATION NOC-14`).

## Cross-Module Access

Direct SQL appears in routes and services on both sides of several domain boundaries. Pages/Marketplace adapters, Content/Delivery invalidation, shared audit, subscription/quota reads, entitlement/install checks, and Stripe-facing billing/finance are the main verified cross-module edges.

## Shared Data

The most broadly shared data is identity, organizations/memberships, audit logs, plans/subscriptions, component registry, Marketplace catalog, entitlements, and provider-linked financial state. Shared does not mean unrestricted; global roles, tenant membership, RLS, catalog state, and privileged transactions still constrain access.

## Shared and Global Tables

`users`, `roles`, `organizations`, `plans`, Marketplace creator/catalog tables, payout tables, and internal notifications are not uniformly tenant-RLS protected. Their callers must enforce global role, membership, creator, catalog-publication, or privileged-service rules. The database does not provide one universal ownership gate.

Audit records are written by many domains through shared helpers. `organization_subscriptions` and Marketplace entitlements are consulted outside their nominal write modules. `component_registry` deliberately combines global system rows with optional tenant rows.

## Change Routing

Before changing a table:

1. Identify its owning entity document and all query sites.
2. Check cross-module readers here, the [Backend Module Catalog](../backend/module-catalog.md), [Backend Module Boundaries](../backend/module-boundaries.md), and [Backend Dependency Map](../backend/dependency-map.md).
3. Review RLS/session-context requirements.
4. Review provider, filesystem, cache, webhook, and audit side effects.
5. Add a forward migration rather than modifying applied history.
6. Update ownership documentation when a module gains write authority.

`PERSISTENCE_BOUNDARY_UNCLEAR PBU-01` remains because no mandatory repository layer centralizes these responsibilities.

## Plugin and Marketplace Ownership

Built-in plugin code is binary-owned and its registry is global. Marketplace catalog data is global, installation/runtime/adapter data is organization-owned, and component_registry can be system-global or tenant-owned. No generic plugin-owned schema or migration namespace exists. See [Tenant and Global Scope](../extensibility/tenant-and-global-scope.md).
