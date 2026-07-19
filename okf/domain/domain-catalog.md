---
okf_document_id: "domain-catalog"
title: "Domain Catalog"
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
  - "overview.md"
  - "business-rule-catalog.md"
related_diagrams:
  - "diagrams/domain-map.mmd"
---

# Domain Catalog

## Summary

| ID | Domain | Implementation | Boundary | Confidence | Document |
| --- | --- | --- | --- | --- | --- |
| `DOM-IDENTITY` | Identity and Access | `IMPLEMENTED` | `OVERLAPPING` | High | [Open](domains/identity-and-access.md) |
| `DOM-TENANT` | Organizations and Membership | `IMPLEMENTED` | `EXPLICIT` | High | [Open](domains/organizations-and-membership.md) |
| `DOM-CONTENT` | Content and Editorial Workflow | `IMPLEMENTED` | `OVERLAPPING` | High | [Open](domains/content-and-editorial.md) |
| `DOM-PAGE` | Pages and Page Builder | `IMPLEMENTED` | `OVERLAPPING` | High | [Open](domains/pages-and-page-builder.md) |
| `DOM-MEDIA` | Media Library | `IMPLEMENTED` | `OBSERVED` | High | [Open](domains/media-library.md) |
| `DOM-DELIVERY` | Delivery, Settings, and Webhooks | `PARTIALLY_IMPLEMENTED` | `OVERLAPPING` | High | [Open](domains/delivery-settings-and-webhooks.md) |
| `DOM-BILLING` | Billing and Quotas | `IMPLEMENTED` | `OVERLAPPING` | High | [Open](domains/billing-and-quotas.md) |
| `DOM-SAAS` | SaaS Operations and Beta | `PARTIALLY_IMPLEMENTED` | `OVERLAPPING` | Medium | [Open](domains/saas-operations-and-beta.md) |
| `DOM-PLUGIN` | Plugins and Components | `PARTIALLY_IMPLEMENTED` | `OVERLAPPING` | High | [Open](domains/plugins-and-components.md) |
| `DOM-MARKETPLACE` | Marketplace | `PARTIALLY_IMPLEMENTED` | `OVERLAPPING` | High | [Open](domains/marketplace.md) |

## `DOM-IDENTITY` — Identity and Access

- Purpose: authenticate users, persist credentials and refresh-token state, and provide global identity/role context.
- Terminology: `users`, `roles`, `user_roles`, `refresh_tokens`, access token, refresh token, global role.
- Backend: `routes/auth.rs`; `middleware/auth.rs`; password, JWT, security, and RBAC services.
- Frontend: `AuthPage`, `RequireAuth`, `useAppStore`, API session helpers.
- API: `/api/auth/*` and `/api/me`.
- Database: identity/global RBAC entity group.
- Access: public login/refresh plus authenticated logout/profile; global role is not tenant membership.
- Workflows/rules/invariants: session establishment, token rotation/revocation, unique case-insensitive email, active-user login.
- Boundary: authentication, bootstrap, global RBAC, and organization membership overlap.

## `DOM-TENANT` — Organizations and Membership

- Purpose: establish tenant identity, active membership, role, ownership, invitations, and tenant administration.
- Terminology: organization, owner, member, invitation, `TenantContext`, organization role.
- Backend: organization routes, tenant middleware, RLS/RBAC/quota/audit/email services.
- Frontend: organization/workspace selection and organization administration.
- API: `/api/organizations*`, `/api/organization-invitations/accept`.
- Database: `organizations`, `organization_members`, `organization_invitations`, domain/rate-limit records.
- Access: owner/admin management; active organization plus active membership required for tenant routes.
- Workflows: [provisioning](workflows/organization-provisioning.md), [membership](workflows/tenant-invitation-and-membership.md), [ownership transfer](workflows/organization-ownership-transfer.md).

## `DOM-CONTENT` — Content and Editorial Workflow

- Purpose: define content schemas, validate dynamic entry data, manage entries, collaboration comments, and editorial states.
- Terminology: content type, field schema, entry, workflow status, comment.
- Backend: content/comments routes; entry validation, workflow, security, plugin, delivery, webhook services.
- Frontend: `ContentTypesPage`, `EntriesPage`, `WorkflowPage`, `DynamicForm`.
- API: `/api/content-types`, `/api/entries/*`, `/api/comments/*`.
- Database: content types/entries and comments.
- Access: organization readers/writers/reviewers/publishers; attribution is not a per-resource ACL.
- Workflows: [entry save](workflows/content-entry-save.md) and [editorial publication](workflows/editorial-publication.md).

## `DOM-PAGE` — Pages and Page Builder

- Purpose: persist structured page documents, component definitions, preview state, version snapshots, and page publication.
- Terminology: `page_json`, layout node, component registry, page version, preview channel.
- Backend: pages route, workflow, delivery, webhook, quota, RLS/RBAC.
- Frontend: `PagesPage`, dnd-kit builder, property controls, local preview, autosave.
- API: `/api/pages*`, `/api/component-registry*`, `/api/preview/{page_id}`.
- Database: pages, page versions, component registry.
- Access: tenant page/component capabilities; system components are readable but tenant mutation is restricted.
- Workflows: [save/version](workflows/page-builder-save-and-version.md), [version restore](workflows/page-version-restoration.md), [publication](workflows/editorial-publication.md).

## `DOM-MEDIA` — Media Library

- Purpose: upload validated files, store metadata, create image variants, update descriptive data, and remove DB/files.
- Terminology: media, variant, uploader, MIME type, upload quota.
- Backend: media route and media processing service.
- Frontend: `MediaPage`.
- API: `/api/media*` and static `/uploads` delivery.
- Database: media and media variants.
- Access: tenant media writer/manager; uploader attribution is not general update-own enforcement.
- Workflow: [media upload and processing](workflows/media-upload-and-processing.md).

## `DOM-DELIVERY` — Delivery, Settings, and Webhooks

- Purpose: expose published CMS data, cache delivery responses, store public settings/navigation, and dispatch signed lifecycle webhooks.
- Terminology: public delivery, cache key, public setting, navigation item, webhook subscription, delivery attempt.
- Backend: delivery route, cache service, webhook route/service.
- Frontend: webhook controls in `SettingsPage`; no complete public-settings/navigation editor was found.
- API: `/api/v1/*`, `/sitemap.xml`, `/robots.txt`, `/api/webhooks*`.
- Database: public settings, navigation items, webhooks, webhook deliveries.
- Status: read and webhook management are implemented; product setting management is partial.
- Workflow: [publication webhook delivery](workflows/publication-webhook-delivery.md).

## `DOM-BILLING` — Billing and Quotas

- Purpose: manage plans, subscriptions, usage limits, Stripe checkout/portal/callback processing, and capacity gates.
- Terminology: plan, subscription, usage counter, billing event, quota, provider event.
- Backend: billing route; quota and Stripe services.
- Frontend: `BillingPage` and organization usage/admin surfaces.
- API: `/api/billing*` and Stripe webhook callback.
- Database: plans, organization subscriptions, usage counters, billing events.
- Access: organization billing manager for mutations; signed public provider callback.
- Workflow: [billing subscription](workflows/billing-subscription.md).

## `DOM-SAAS` — SaaS Operations and Beta

- Purpose: manage organization domains/rate limits, expose audit/email/alert operational records, and track beta feedback/readiness.
- Terminology: organization domain, rate limit, audit log, email delivery, alert rule, beta participant, feedback, GA blocker.
- Backend: organization/beta routes; audit, email, quota, readiness services.
- Frontend: `OrganizationPage`, `BetaPage`, dashboard indicators.
- API: organization operations plus `/api/beta*` and global beta administration.
- Database: operations/audit/email/alerts and beta entity groups.
- Status: CRUD/readiness views are implemented; automated alert evaluation and verified domain routing are unclear.
- Workflow: [beta feedback and readiness](workflows/beta-feedback-and-readiness.md).

## `DOM-PLUGIN` — Plugins and Components

- Purpose: register/enable built-in CMS hooks and supply system/tenant component definitions to Page Builder.
- Terminology: `CmsPlugin`, hook, `cms_plugins`, component registry, `component_key`.
- Backend: plugin routes and trait implementation; pages component registry; Marketplace host adapters.
- Frontend: Workflow plugin toggles and Page Builder palette.
- API: `/api/plugins*`, `/api/component-registry*`, Marketplace component/hook adapter endpoints.
- Database: CMS plugin and component registry tables plus Marketplace adapter records.
- Status: built-in plugin execution and host-owned adapters are implemented; arbitrary uploaded code execution is `PLANNED_NOT_IMPLEMENTED`.

## `DOM-MARKETPLACE` — Marketplace

- Purpose: govern creator/catalog submission, package validation/review, publication, installation/runtime adapters, finance, feedback, analytics, and readiness evidence.
- Terminology: creator, listing, version, submission, installation, permission, kill switch, purchase, entitlement, review, abuse report.
- Backend: Marketplace route families and service family.
- Frontend: `MarketplacePage`.
- API: `/api/marketplace*` route groups.
- Database: Marketplace entity groups `DB-ENT-013` through `DB-ENT-018`.
- Access: global administration, creator ownership, tenant roles, entitlements, manifest permissions, and runtime policy.
- Workflows: [product publication](workflows/marketplace-product-publication.md), [installation](workflows/marketplace-installation-lifecycle.md), [purchase](workflows/marketplace-purchase-and-entitlement.md).
- Status: host-owned safe capabilities are implemented; arbitrary package execution and automated payout transfer are not.

