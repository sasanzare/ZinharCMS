---
okf_document_id: "project-glossary"
title: "Project Glossary"
project: "ZinharCMS"
category: "project"
phase: 1
status: "current"
review_status: "verified"
source_of_truth: false
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
primary_sources:
  - "README.md"
  - "backend/migrations"
  - "backend/src"
  - "frontend/src"
  - "docs/ARCHITECTURE.md"
  - "docs/V3_MARKETPLACE_SCOPE.md"
  - "docs/V3_PRODUCT_TAXONOMY.md"
related_documents:
  - "README.md"
  - "index.yaml"
  - "project/overview.md"
  - "project/repository-map.md"
  - "project/navigation-guide.md"
  - "references/source-register.md"
  - "frontend/README.md"
  - "frontend/application-catalog.md"
  - "frontend/feature-catalog.md"
  - "frontend/state-management.md"
uncertainty_markers:
  - "UNKNOWN U-04"
  - "UNKNOWN U-05"
  - "NEEDS_OWNER_CONFIRMATION NOC-01"
  - "NEEDS_OWNER_CONFIRMATION NOC-07"
  - "DOCUMENTATION_CODE_CONFLICT DCC-01"
  - "DOCUMENTATION_CODE_CONFLICT DCC-03"
  - "DOCUMENTATION_CODE_CONFLICT DCC-04"
  - "DOCUMENTATION_CODE_CONFLICT DCC-11"
  - "FEATURE_BOUNDARY_UNCLEAR FBU-01"
  - "STATE_OWNERSHIP_UNCLEAR SOU-01"
  - "API_CONTRACT_UNCLEAR ACU-01"
  - "INFERRED_FROM_CODE"
  - "AMBIGUOUS"
  - "PLANNED_NOT_IMPLEMENTED"
  - "PLANNED_NOT_IMPLEMENTED PNI-01"
---

# Project Glossary

This glossary defines the vocabulary used by the Phase 1 OKF documents. It is descriptive, not authoritative: executable code, migrations, and configuration remain the primary evidence. Status values distinguish direct evidence from interpretation and unresolved terminology.

For the broader context, see [Project Identity](overview.md#1-project-identity), [Major Capabilities](overview.md#4-major-capabilities), and [System Boundaries](overview.md#5-system-boundaries).

| Term | Definition | Project context | Evidence | Related terms | Status |
|---|---|---|---|---|---|
| ZinharCMS | The repository's named software project: an API-first, headless, multi-tenant CMS with SaaS and Marketplace capabilities. | Names the codebase and product described by the root documentation and package manifests. | `README.md`; `package.json`; `backend/Cargo.toml` | CMS, Marketplace, Delivery API | VERIFIED |
| Organization | The primary persisted tenant and ownership boundary. | Organization IDs scope content, members, roles, billing, quotas, and Marketplace installations. | `backend/migrations/0008_v2_phase_one_organizations.sql`; `backend/src/middleware/tenant.rs` | Tenant, Organization Member, TenantContext | VERIFIED |
| Tenant | The security and data-isolation interpretation of an organization. | Middleware, ownership checks, and PostgreSQL RLS use the organization as the tenant boundary. | `backend/src/middleware/tenant.rs`; `backend/src/services/rls.rs`; `backend/migrations/0009_v2_phase_three_rls.sql` | Organization, RLS, TenantContext | VERIFIED |
| TenantContext | Request-scoped backend context containing the resolved organization and authenticated membership information. | Created by tenant middleware and consumed by organization-scoped handlers. | `backend/src/middleware/tenant.rs`; `backend/src/routes/mod.rs` | Tenant, Organization Member | VERIFIED |
| Workspace | A frontend routing and presentation label for selecting an organization context. | `/workspace/:slug` resolves a membership and redirects to the dashboard; normal feature URLs use the active organization from browser state rather than a nested workspace route tree. No separate workspace persistence model was found. | `frontend/src/pages/WorkspaceRedirectPage.tsx`; `frontend/src/router.tsx`; `frontend/src/stores/useAppStore.ts` | Organization, Tenant | INFERRED_FROM_CODE |
| Site | A possible public-delivery target or custom-domain concept, but not a confirmed first-class entity. | Public routing intent is documented while the complete domain-to-tenant lifecycle remains unresolved. | `backend/src/routes/delivery.rs`; `docs/diagrams/02-system-context.mmd`; `okf-bootstrap/09-knowledge-gaps.md` | Delivery API, Organization | AMBIGUOUS |
| Project | A generic name for the repository or implementation effort, not a confirmed persisted business entity. | No project table, route family, or domain model was identified. | `README.md`; `backend/migrations` | ZinharCMS, Workspace | AMBIGUOUS |
| User | An authenticated account represented in the identity model. | Users authenticate and obtain organization membership and role context. | `backend/migrations/0001_initial_schema.sql`; `backend/src/middleware/auth.rs` | Organization Member, Global Role | VERIFIED |
| Organization Member | The association between a user and an organization. | Membership is checked during tenant resolution and can carry organization-scoped roles. | `backend/migrations/0008_v2_phase_one_organizations.sql`; `backend/src/middleware/tenant.rs` | User, Organization Role | VERIFIED |
| Global Role | A system-wide role independent of an organization membership. | Used by authorization logic for global administrative capabilities. | `backend/src/services/rbac.rs`; `backend/migrations/0001_initial_schema.sql` | Organization Role, Marketplace Permission | VERIFIED |
| Organization Role | A role attached to organization membership and evaluated within a tenant. | Supports organization-scoped authorization. | `backend/src/services/rbac.rs`; `backend/migrations/0008_v2_phase_one_organizations.sql` | Organization Member, Global Role | VERIFIED |
| Marketplace Permission | A declared permission associated with a Marketplace package or installation. | Permissions are part of validation and installation policy; they do not authorize arbitrary server-side code execution. | `backend/migrations/0020_v3_phase_seven_permission_sandbox_kill_switch.sql`; `backend/src/routes/marketplace_runtime.rs`; `backend/src/services/marketplace_runtime.rs` | Installation, Kill Switch, Backend Extension | VERIFIED |
| Content Type | A schema describing structured content fields. | Content types define the structure used by entries. | `backend/migrations/0003_phase_one_core.sql`; `backend/src/routes/content.rs` | Entry, Component | VERIFIED |
| Entry | A structured content record associated with a content type and organization. | Entry routes support lifecycle actions including publication-related operations. | `backend/migrations/0003_phase_one_core.sql`; `backend/src/routes/content.rs` | Content Type, Draft, Published | VERIFIED |
| Draft | An editable, non-published lifecycle state for content or pages. | Draft state participates in content and page workflows. | `backend/src/routes/content.rs`; `backend/src/routes/pages.rs`; `docs/diagrams/23-core-cms-state-machines.mmd` | Published, Page Version | VERIFIED |
| Published | A lifecycle state indicating content is eligible for delivery. | Publication is represented in handlers and state transitions; exact delivery eligibility must still be verified per resource. | `backend/src/routes/content.rs`; `backend/src/routes/pages.rs`; `backend/src/routes/delivery.rs` | Draft, Delivery API | VERIFIED |
| Page | A page-builder resource owned by an organization. | Pages contain versioned structured data used by page-builder workflows. | `backend/migrations/0004_phase_two_page_builder.sql`; `backend/src/routes/pages.rs` | Page Version, Component, Block | VERIFIED |
| Page Version | A persisted version of page data. | Supports page history and page lifecycle operations. | `backend/migrations/0004_phase_two_page_builder.sql`; `backend/src/routes/pages.rs` | Page, Revision | VERIFIED |
| Revision | A general documentation and UI term for historical content; usually a page version in current implementation. | No separate revision domain entity was confirmed across all resource types. | `frontend/src/pages/PagesPage.tsx`; `backend/src/routes/pages.rs` | Page Version, Draft | AMBIGUOUS |
| Component | A reusable page-builder definition or instance represented in structured page data. | Component definitions and registry behavior support page composition. | `backend/migrations/0004_phase_two_page_builder.sql`; `frontend/src/pages/PagesPage.tsx` | Component Registry, Block, Page | VERIFIED |
| Block | A UI-level label for a node or unit in page-builder data. | No standalone database block entity was identified. | `frontend/src/pages/PagesPage.tsx`; `backend/migrations/0004_phase_two_page_builder.sql` | Component, Page | INFERRED_FROM_CODE |
| Component Registry | The catalog used to resolve available page-builder components. | Registry behavior is exposed by backend routes and used by the page-builder UI. | `backend/src/routes/pages.rs`; `frontend/src/pages/PagesPage.tsx` | Component, Page | VERIFIED |
| Media Asset | Uploaded media metadata and its storage reference. | Current runtime storage is local filesystem-based; object storage or CDN delivery is not implemented. | `backend/src/routes/media.rs`; `.env.example`; `env.example` | Delivery API | VERIFIED |
| Plugin | A built-in CMS plugin implementing the host's plugin abstraction. | Built-in plugins are distinct from uploaded Marketplace packages and are registered by the backend. | `backend/src/plugins/mod.rs`; `backend/src/plugins/seo.rs`; `backend/src/routes/plugins.rs` | Marketplace, Extension | VERIFIED |
| Marketplace | The product area for creator submissions, listings, versions, review, installation, finance records, ratings, and abuse operations. | Implemented through migrations, backend routes and services, and frontend pages. | `backend/migrations/0015_v3_phase_one_marketplace_foundation.sql`; `backend/src/routes/marketplace.rs`; `frontend/src/pages/MarketplacePage.tsx` | Creator, Listing, Installation | VERIFIED |
| Creator | A Marketplace participant who owns or manages submitted packages and listings. | Creator records and workflows are distinct from ordinary Marketplace browsing. | `backend/migrations/0016_v3_phase_two_creator_submission.sql`; `backend/src/routes/marketplace.rs` | Submission, Listing, Package | VERIFIED |
| Listing | The Marketplace-facing catalog record for an extension offering. | Listings can have versions and participate in moderation and publication workflows. | `backend/migrations/0015_v3_phase_one_marketplace_foundation.sql`; `backend/src/routes/marketplace.rs` | Package, Version, Submission | VERIFIED |
| Package | An uploaded Marketplace artifact and its declared metadata. | Packages are validated and installed through host-controlled workflows; uploaded code is not executed by the server. | `backend/src/routes/marketplace.rs`; `backend/src/services/marketplace_package.rs`; `backend/src/services/marketplace_runtime.rs`; `docs/V3_MARKETPLACE_SCOPE.md` | Version, Submission, Installation | VERIFIED |
| Version | A versioned Marketplace release associated with a listing or package. | Versions carry artifacts and state used by review and installation workflows. | `backend/migrations/0016_v3_phase_two_creator_submission.sql`; `backend/src/routes/marketplace.rs`; `backend/src/services/marketplace_package.rs` | Listing, Package, Submission | VERIFIED |
| Submission | A creator request to validate and review a Marketplace version. | Submissions pass through validation and moderation states. | `backend/migrations/0017_v3_phase_three_validation_pipeline.sql`; `backend/migrations/0018_v3_phase_four_review_moderation.sql`; `backend/src/routes/marketplace.rs`; `backend/src/services/marketplace_submission.rs` | Version, Creator, Listing | VERIFIED |
| Installation | The organization-scoped record and lifecycle for installing a Marketplace version. | Installations are governed by entitlements, permissions, adapters, and kill-switch controls. | `backend/migrations/0019_v3_phase_six_installation_lifecycle.sql`; `backend/src/routes/marketplace.rs`; `backend/src/services/marketplace_installation.rs` | Entitlement, Marketplace Permission, Kill Switch | VERIFIED |
| Entitlement | A record that permits an organization to use a Marketplace offering. | Entitlements participate in installation and commerce checks. | `backend/migrations/0019_v3_phase_six_installation_lifecycle.sql`; `backend/src/routes/marketplace.rs`; `backend/src/services/marketplace_installation.rs` | Installation, Listing | VERIFIED |
| Kill Switch | A host-controlled mechanism for disabling Marketplace capabilities or installations. | It is part of Marketplace operational safety and permission enforcement. | `backend/migrations/0020_v3_phase_seven_permission_sandbox_kill_switch.sql`; `backend/src/routes/marketplace_runtime.rs`; `backend/src/services/marketplace_runtime.rs` | Installation, Marketplace Permission | VERIFIED |
| Locale | A supported language or regional selection used by the frontend i18n system and localized content behavior. | The frontend contains explicit locale, message, direction, and selection modules. | `frontend/src/i18n/locales.ts`; `frontend/src/i18n/messages.ts`; `frontend/src/i18n/I18nProvider.tsx` | Workspace, Content | VERIFIED |
| Delivery API | The public, versioned route family under `/api/v1` used to deliver published content. | It is distinct from the administrative `/api` route family. Current organization selection includes unresolved hard-coded behavior. | `backend/src/routes/mod.rs`; `backend/src/routes/delivery.rs`; `docs/API.md` | Published, Site, Media Asset | VERIFIED |
| RLS | PostgreSQL Row-Level Security used as a database-level tenant-isolation control. | Request code establishes tenant context used by RLS policies. | `backend/migrations/0009_v2_phase_three_rls.sql`; `backend/src/services/rls.rs`; `backend/src/db/mod.rs` | Tenant, Organization | VERIFIED |
| Extension | A broad product term covering Marketplace offerings and possible integration types. | Existing taxonomy and implementation use overlapping concepts; it is not interchangeable with the built-in CMS plugin abstraction. | `docs/V3_PRODUCT_TAXONOMY.md`; `docs/V3_MARKETPLACE_SCOPE.md`; `backend/src/plugins/mod.rs` | Plugin, Package, Backend Extension | AMBIGUOUS |
| Backend Extension | A proposed Marketplace extension category that would require safe server-side execution. | Official scope defers arbitrary uploaded server-side code; the current runtime is an allowlisted host API rather than arbitrary uploaded-code execution. | `docs/V3_MARKETPLACE_SCOPE.md`; `backend/src/routes/marketplace_runtime.rs`; `backend/src/services/marketplace_runtime.rs`; `okf-bootstrap/09-knowledge-gaps.md` | Extension, Marketplace Permission, Package | PLANNED_NOT_IMPLEMENTED |
| Frontend Application | A separately started, built, or deployed browser application. | The repository has one verified frontend application, FE-APP-001; pages and source directories are not separate applications. | `frontend/package.json`; `frontend/src/main.tsx`; `frontend/Dockerfile.prod`; `okf/frontend/application-catalog.md` | SPA, Route Page | VERIFIED |
| SPA | A single-page application that uses browser-side routing after loading one frontend entry document. | The ZinharCMS management frontend uses React Router and an Nginx history fallback. | `frontend/src/main.tsx`; `frontend/src/router.tsx`; `frontend/nginx.conf` | Frontend Application, Route Page | VERIFIED |
| Route Page | A React component selected directly by the frontend route table and used as the dominant current feature composition boundary. | Route pages generally own presentation, API orchestration, server data, drafts, loading, and errors. | `frontend/src/router.tsx`; `frontend/src/pages`; `okf/frontend/feature-boundaries.md` | AppShell, Frontend Feature | INFERRED_FROM_STRUCTURE |
| Frontend Feature | A significant user workflow or cross-cutting browser responsibility selected by the Phase 4 feature catalog. | Thirteen features map routes, source ownership, state, API usage, access cues, and tests without implying separate packages. | `okf/frontend/feature-catalog.md`; `okf/frontend/features` | Route Page, Frontend Application | VERIFIED |
| AppShell | The shared protected React layout that renders navigation, organization and locale controls, readiness, user status, logout, and the active route outlet. | It integrates several cross-cutting concerns and is not a backend authorization boundary. | `frontend/src/components/AppShell.tsx`; `frontend/src/components/RequireAuth.tsx` | Route Page, Workspace, Client State | VERIFIED |
| Client State | Browser-owned reactive or persistent data used to render and construct requests. | Zustand owns session/organization/shell state, i18n context owns locale, and route pages own feature-local state. | `frontend/src/stores/useAppStore.ts`; `frontend/src/i18n/I18nProvider.tsx`; `frontend/src/pages` | Server State, API Client | VERIFIED |
| Server State | Backend-owned data copied into page-local frontend state after API calls. | No shared query cache was found; pages load and refresh their own server responses. | `frontend/src/pages`; `frontend/src/hooks/useHealth.ts`; `okf/frontend/state-management.md` | Client State, API Client | INFERRED_FROM_CODE |
| API Client | The central browser module that constructs HTTP requests, adds session/organization context, parses JSON, and throws `ApiError`. | Its TypeScript types manually duplicate backend contracts under ACU-01/DC-01. | `frontend/src/services/api.ts`; `frontend/src/types/api.ts` | Client State, Server State | VERIFIED |
| Local Preview | The Page Builder's in-page React rendering of the current page JSON. | It is distinct from the copied backend WebSocket preview URL and does not prove public-renderer parity. | `frontend/src/pages/PagesPage.tsx`; `okf/frontend/page-builder.md` | Page, Component Registry | VERIFIED |
| Page Builder | The frontend feature that composes page JSON from registered component definitions through a palette, sortable canvas, property editor, preview, and persistence workflow. | It is implemented inside `PagesPage`, not as a separate frontend application or package. | `frontend/src/pages/PagesPage.tsx`; `frontend/src/pages/PagesPage.test.tsx`; `okf/frontend/page-builder.md` | Page, Component Registry, Local Preview | VERIFIED |

## Phase 5 Database Terms

| Term | Definition | Clarification | Evidence | Related terms | Status |
| --- | --- | --- | --- | --- | --- |
| Tenant Transaction | A SQLx transaction with transaction-local Zinhar organization, user, and bypass settings used by PostgreSQL RLS. | It is distinct from a pooled connection with session-level context. | `backend/src/services/rls.rs`; `okf/database/multi-tenancy.md` | Organization, RLS, Bypass Transaction | VERIFIED |
| Bypass Transaction | A privileged SQLx transaction that sets `zinhar.rls_bypass` for narrowly scoped global, admin, catalog, or provider work. | It is a security-sensitive exception, not a default persistence path. | `backend/src/services/rls.rs`; Marketplace and billing services | Tenant Transaction, RLS | VERIFIED |
| Migration-Defined Schema | The intended PostgreSQL object state produced by applying all tracked migrations in order. | It does not prove the state of a deployed database. | `backend/migrations`; `okf/database/migrations.md` | Runtime Schema, SQLx | VERIFIED |
| Runtime Schema | The objects and definitions actually present in a specific database environment. | Its current state is `SCHEMA_RUNTIME_STATUS_UNKNOWN SRU-01` until catalog and migration metadata are inspected. | `okf/database/schema-catalog.md`; `UNKNOWN U-02` | Migration-Defined Schema | UNKNOWN |
| Entity Group | A Phase 5 documentation aggregate that can map to one table or several tightly related persistence tables. | It is a navigation construct, not a generated ORM entity. | `okf/database/entity-catalog.md` | Schema Object, Backend Module | VERIFIED |

## Usage Rules

- Prefer `Organization` for the persisted business boundary and `Tenant` when discussing isolation or request context.
- Do not treat `Workspace`, `Site`, or `Project` as confirmed database entities without new primary evidence.
- Do not use `Plugin` and `Marketplace Extension` interchangeably; built-in CMS plugins and uploaded Marketplace artifacts have different trust boundaries.
- Preserve a term's status when reusing it in another OKF document. Promote an ambiguous or inferred term only after recording primary evidence and review.

## Related Documents

- [OKF Entry Point](../README.md)
- [Machine-Readable Index](../index.yaml)
- [Project Overview](overview.md)
- [Repository Map](repository-map.md)
- [Navigation Guide](navigation-guide.md)
- [Source Register](../references/source-register.md)
- [Frontend Architecture](../frontend/README.md)
- [Frontend Application Catalog](../frontend/application-catalog.md)
- [Frontend Feature Catalog](../frontend/feature-catalog.md)
- [Database Architecture](../database/README.md)
- [Database Entity Catalog](../database/entity-catalog.md)
