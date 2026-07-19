---
okf_document_id: "domain-overview"
title: "Domain Overview"
project: "ZinharCMS"
category: "domain"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/routes"
  - "backend/src/services"
  - "backend/migrations"
related_documents:
  - "README.md"
  - "domain-catalog.md"
  - "business-rule-catalog.md"
related_diagrams:
  - "diagrams/domain-map.mmd"
---

# Domain Overview

## Domain Summary

ZinharCMS implements ten significant observed domains:

1. Identity and access: users, credentials, refresh tokens, global roles, and session establishment.
2. Organizations and membership: tenant creation, active membership, invitations, organization roles, quotas, leave, and ownership transfer.
3. Content and editorial workflow: content schemas, entries, dynamic validation, collaboration comments, and publication states.
4. Pages and Page Builder: page documents, registered components, nested layout validation, preview, snapshots, and page workflow.
5. Media library: tenant-scoped upload metadata, content-derived MIME validation, image variants, and filesystem deletion.
6. Delivery, settings, and webhooks: published content delivery, Redis caching, public settings/navigation, sitemap/robots output, and signed publication webhooks.
7. Billing and quotas: plan catalog, current tenant subscription, usage counters, Stripe checkout/callbacks, and feature capacity checks.
8. SaaS operations and beta: custom domains, rate limits, audit/email records, alert definitions, beta participants, feedback, blockers, and readiness views.
9. Plugins and components: built-in CMS plugin hooks, global plugin enablement, system/tenant component definitions, and Page Builder component use.
10. Marketplace: creator, listing, version, validation, review, installation, runtime permission, adapter, purchase, entitlement, feedback, analytics, and readiness lifecycles.

Localization is implemented as frontend presentation state and public navigation locale data, but no independent localization aggregate or localized content-entry persistence model was found. It is therefore documented as a cross-cutting behavior rather than a standalone domain.

## Domain Architecture

Business logic is distributed. Route handlers frequently combine request validation, RBAC, tenant checks, SQL, lifecycle branching, cache invalidation, audit calls, email, webhook triggers, or provider calls. Reusable services own workflow transitions, entry validation, security sanitization, quotas, Stripe handling, webhook dispatch, media processing, and most Marketplace policies. Migrations enforce enums, checks, foreign keys, uniqueness, triggers, RLS, and selected immutability rules.

The architecture is therefore mixed rather than a strict domain layer:

- API handlers are authoritative for many orchestration sequences.
- Services provide reusable rules where the same behavior is needed by several handlers.
- PostgreSQL supplies final shape and relationship constraints but does not encode every allowed state transition.
- Frontend pages mirror workflows and validation for usability, while backend and database enforcement remain authoritative.
- Transactions are operation-specific; no universal unit-of-work boundary exists.

## Main Lifecycle Concepts

- Content entries and pages use `draft`, `pending_review`, `published`, and `archived` with the shared `WorkflowStatus` transition policy.
- Content entry `version` increments on data updates and workflow transitions; no entry snapshot table exists.
- Page writes create append-only numbered `page_versions` snapshots in the same transaction as create/update/restore.
- Organizations and memberships require `active` status to establish `TenantContext`.
- Invitations progress among `pending`, `accepted`, `revoked`, and `expired` through explicit handlers and expiry maintenance.
- Subscriptions, billing events, beta records, plugin enabled state, and Marketplace objects have independent lifecycle vocabularies.
- Deletion policy is mixed: several CMS resources are hard deleted, content/pages can also be archived, and Marketplace installations use soft lifecycle states.

## Cross-Domain Relationships

- Every tenant CMS resource depends on organization membership, tenant middleware, organization RBAC, and RLS.
- Content saves invoke built-in plugin hooks; publication invokes delivery-cache invalidation and webhook dispatch.
- Page writes depend on the component registry and publish preview updates; Marketplace component packs and templates extend Page Builder inputs through host-owned adapters.
- Media and content/page counts are limited by the current billing plan.
- Billing provider callbacks update subscriptions and Marketplace purchase/entitlement records.
- SaaS operations consume audit, email, quota, webhook, and billing data for administrative and beta readiness views.

## Main Unknowns

- `BUSINESS_RULE_UNVERIFIED`: repository code establishes current thresholds and transitions but usually not their product rationale.
- `STATE_TRANSITION_UNCLEAR`: most text-status domains validate allowed values without one exhaustive transition policy.
- `INVARIANT_UNVERIFIED`: same-tenant coherence is not represented by composite foreign keys for every parent/child pair.
- `CROSS_MODULE_ORCHESTRATION_UNCLEAR`: audit, cache, email, files, webhooks, and external provider effects are not uniformly atomic with domain writes.
- `DOCUMENTATION_CODE_CONFLICT`: the shared `PageStatus` Rust model omits `pending_review`, although migration and handlers support it.
- `WORKFLOW_UNCLEAR`: public settings/navigation have delivery reads and seeded storage, but no complete authenticated management workflow was found.
- `PLANNED_NOT_IMPLEMENTED`: durable background workers, automatic payout execution, arbitrary Marketplace package execution, and publication scheduling were not found.

