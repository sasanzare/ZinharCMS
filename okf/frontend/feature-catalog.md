---
okf_document_id: "frontend-feature-catalog"
title: "Frontend Feature Catalog"
project: "ZinharCMS"
category: "frontend"
phase: 4
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources:
  - "frontend/src/router.tsx"
  - "frontend/src/pages"
  - "frontend/src/components"
  - "frontend/src/hooks"
  - "frontend/src/i18n"
  - "frontend/src/services/api.ts"
  - "frontend/src/stores/useAppStore.ts"
  - "frontend/src/types/api.ts"
related_documents:
  - "frontend/README.md"
  - "frontend/application-catalog.md"
  - "frontend/feature-boundaries.md"
  - "frontend/routing.md"
  - "frontend/testing-map.md"
  - "backend/module-catalog.md"
related_diagrams:
  - "frontend/diagrams/frontend-application-map.mmd"
uncertainty_markers:
  - "FEATURE_BOUNDARY_UNCLEAR FBU-01"
  - "FEATURE_BOUNDARY_UNCLEAR FBU-02"
  - "COMPONENT_OWNERSHIP_UNCLEAR COU-01"
  - "RESPONSIBILITY_OVERLAP FRO-01"
  - "RESPONSIBILITY_OVERLAP FRO-02"
---

# Frontend Feature Catalog

## Selection Rule

Phase 4 selects a feature when current frontend code provides a meaningful user workflow, route-level surface, or cross-cutting runtime responsibility. Small helpers remain part of an owning feature or shared platform concern. This catalog intentionally does not create one feature per source file.

## Summary

| ID | Feature | Primary entry | Responsibility | Implementation | Boundary | Confidence |
|---|---|---|---|---|---|---|
| FE-FEAT-001 | [Authentication and Session](features/authentication-and-session.md) | `/login`; `RequireAuth` | Login, registration, logout, session restoration, route admission | `IMPLEMENTED` | `OVERLAPPING` | High |
| FE-FEAT-002 | [Dashboard and Application Shell](features/dashboard-and-application-shell.md) | `/`; `AppShell` | Navigation, shared chrome, health, summary metrics | `IMPLEMENTED` | `OVERLAPPING` | High |
| FE-FEAT-003 | [Organizations and Workspaces](features/organizations-and-workspaces.md) | `/organization`; `/workspace/:slug` | Organization selection and control-plane UI | `IMPLEMENTED` | `OVERLAPPING` | High |
| FE-FEAT-004 | [Content Modeling](features/content-modeling.md) | `/content-types` | Content-type and field-schema management | `IMPLEMENTED` | `OBSERVED` | High |
| FE-FEAT-005 | [Content Entries](features/content-entries.md) | `/entries` | Entry listing, editing, status changes, deletion | `IMPLEMENTED` | `OVERLAPPING` | High |
| FE-FEAT-006 | [Media Library](features/media-library.md) | `/media` | Media listing, upload, metadata update, deletion | `IMPLEMENTED` | `OBSERVED` | High |
| FE-FEAT-007 | [Pages and Page Builder](features/pages-and-page-builder.md) | `/pages` | Page CRUD, composition, preview, versions, workflow | `IMPLEMENTED` | `OVERLAPPING` | High |
| FE-FEAT-008 | [Editorial Workflow and Collaboration](features/editorial-workflow-and-collaboration.md) | `/workflow` | Review queues, publication decisions, comments, plugin toggles | `IMPLEMENTED` | `OVERLAPPING` | High |
| FE-FEAT-009 | [Billing and Usage](features/billing-and-usage.md) | `/billing` | Plans, usage, checkout, portal, usage rebuild | `IMPLEMENTED` | `OBSERVED` | High |
| FE-FEAT-010 | [Beta Operations](features/beta-operations.md) | `/beta` | Feedback, blockers, participant and readiness dashboards | `IMPLEMENTED` | `OVERLAPPING` | High |
| FE-FEAT-011 | [Marketplace](features/marketplace.md) | `/marketplace` | Catalog, install, commerce, creator, review, runtime, analytics | `IMPLEMENTED` | `OVERLAPPING` | High |
| FE-FEAT-012 | [Settings and Webhooks](features/settings-and-webhooks.md) | `/settings` | Current-user display, probes, logout, webhook management | `IMPLEMENTED` | `OVERLAPPING` | High |
| FE-FEAT-013 | [Localization and Direction](features/localization-and-direction.md) | Root provider and language selector | Locale selection, translation, fallback, LTR/RTL | `IMPLEMENTED` | `EXPLICIT` | High |

## Detailed Entries

### FE-FEAT-001: Authentication and Session

- Primary paths: `pages/AuthPage.tsx`, `components/RequireAuth.tsx`, `stores/useAppStore.ts`, and auth methods in `services/api.ts`.
- Responsibility and entry: public `/login`; registration/login submission; authenticated-route admission; shell logout; browser session restoration.
- UI and layout: `AuthPage` renders outside `AppShell`; protected pages render through `RequireAuth` and `AppShell`.
- State: controlled auth form state plus global token, user, organization, and active-organization state persisted in `localStorage`.
- Backend interaction and schema: auth login/register/refresh/logout and current-user methods; manual auth types in `types/api.ts`.
- Access: token presence controls route admission. This is advisory and carries `AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01` for Phase 4.
- Tests and assets: no dedicated auth test; shared localization and global styles; no auth-specific asset boundary.
- Relationships: Organizations, Shell, API Client, and Localization.
- Status: `IMPLEMENTED`, `OVERLAPPING`, high confidence.

### FE-FEAT-002: Dashboard and Application Shell

- Primary paths: `components/AppShell.tsx`, `pages/DashboardPage.tsx`, `hooks/useHealth.ts`, and `components/StatusBadge.tsx`.
- Responsibility and entry: `/` dashboard plus chrome shared by all protected routes.
- UI and layout: persistent sidebar, top bar, organization switcher, locale selector, readiness badge, user chip, logout, and route outlet.
- State: global sidebar/session/organization state; periodic health hook; dashboard-local summary and usage state.
- Backend interaction and schema: health/readiness, content counts, and billing usage through manual API types.
- Access: any client-authenticated user sees all shell navigation links; no route-specific menu filtering was found.
- Tests and assets: `DashboardPage.test.tsx`; Lucide icons; global CSS.
- Relationships: every protected feature, Organizations, Authentication, Localization.
- Status: `IMPLEMENTED`, `OVERLAPPING`, high confidence; `FEATURE_BOUNDARY_UNCLEAR FBU-02`.

### FE-FEAT-003: Organizations and Workspaces

- Primary paths: `pages/OrganizationPage.tsx`, `pages/WorkspaceRedirectPage.tsx`, `components/AppShell.tsx`, and organization state in `stores/useAppStore.ts`.
- Responsibility and entry: `/organization` administration and `/workspace/:slug` context selection.
- UI and layout: AppShell plus organization detail, members, invitations, domains, limits, audit, email, alerts, create/accept flows.
- State: global organization list/active ID plus extensive page-local request, draft, and feedback state.
- Backend interaction and schema: organization membership, workspace, domain, operational, and invitation methods; manual organization types.
- Access: owner/admin management cues and owner-specific actions are visible in page logic; backend enforcement remains authoritative.
- Tests and assets: no dedicated frontend test; Lucide icons and shared styles.
- Relationships: Authentication, Shell, Billing, Beta, Marketplace, and every tenant API call.
- Status: `IMPLEMENTED`, `OVERLAPPING`, high confidence.

### FE-FEAT-004: Content Modeling

- Primary path: `pages/ContentTypesPage.tsx`; content-type API methods and `FieldSchema` types.
- Responsibility and entry: `/content-types`; list, filter, create, update, and delete models and field definitions.
- UI and layout: AppShell; page-local table, editor, and field controls using shared status presentation.
- State: page-local list, selection, drafts, loading, and error state.
- Backend interaction and schema: content-type methods; manually declared content and field shapes.
- Access: protected route with no route-level role visibility; action authorization is not proven by the frontend.
- Tests and assets: no dedicated frontend test; no feature-specific assets.
- Relationships: Content Entries and DynamicForm.
- Status: `IMPLEMENTED`, `OBSERVED`, high confidence.

### FE-FEAT-005: Content Entries

- Primary paths: `pages/EntriesPage.tsx` and `components/DynamicForm.tsx`.
- Responsibility and entry: `/entries`; choose a content type, list entries, edit generated fields, change status, and delete.
- UI and layout: AppShell; generated form plus list/editor panels.
- State: page-local selected type, entries, draft content, request state, and feedback.
- Backend interaction and schema: content-type reads and entry CRUD/workflow methods; `FieldSchema` and JSON types.
- Access: protected route; no independent route permission; backend status rules remain authoritative.
- Tests and assets: no dedicated frontend test; shared global CSS.
- Relationships: Content Modeling, Editorial Workflow, StatusBadge, API Client.
- Status: `IMPLEMENTED`, `OVERLAPPING`, high confidence.

### FE-FEAT-006: Media Library

- Primary path: `pages/MediaPage.tsx`; media API methods and types.
- Responsibility and entry: `/media`; list, upload, edit metadata, and delete media.
- UI and layout: AppShell; upload/editor controls and media grid/list.
- State: page-local files, records, drafts, loading, upload, and error state.
- Backend interaction and schema: multipart upload plus media CRUD; manually declared response types.
- Access: protected route with no route-level permission visibility.
- Tests and assets: no dedicated frontend test; selected browser `File` objects are transient and uploaded media is backend-owned.
- Relationships: Pages and Marketplace can consume media concepts, but no frontend shared media picker boundary was found.
- Status: `IMPLEMENTED`, `OBSERVED`, high confidence.

### FE-FEAT-007: Pages and Page Builder

- Primary path: `pages/PagesPage.tsx`; dnd-kit; page/component/Marketplace-adapter API methods and types.
- Responsibility and entry: `/pages`; page CRUD, JSON composition, palette, canvas, property editor, local preview, autosave, versions, restore, workflow, Marketplace template import, and preview URL copy.
- UI and layout: AppShell; a three-column builder nested in the page route plus page/version/template panels.
- State: extensive page-local builder and server state; access token and active organization read from the global store.
- Backend interaction and schema: pages, component registry, preview URL, Marketplace components/templates, versions, and workflow methods.
- Access: protected route; no page-level role gate was found; backend controls mutations and preview authorization.
- Tests and assets: `PagesPage.test.tsx` verifies the shell with mocked API behavior; no drag/drop, autosave, version, or preview integration test.
- Relationships: Marketplace, Editorial Workflow, Authentication, and backend Pages/Preview.
- Status: `IMPLEMENTED`, `OVERLAPPING`, high confidence; `RESPONSIBILITY_OVERLAP FRO-01`.

### FE-FEAT-008: Editorial Workflow and Collaboration

- Primary path: `pages/WorkflowPage.tsx`; entry, page, comment, and plugin API groups.
- Responsibility and entry: `/workflow`; aggregate pending items, publish/reject, comment and resolve, and toggle built-in plugins.
- UI and layout: AppShell; review list, detail/actions, comment area, and plugin panel.
- State: page-local queue, selected item, comments, filters, plugin records, loading, and errors.
- Backend interaction and schema: content types, entries, pages, comments, plugins; manual polymorphic resource types.
- Access: protected route without a route-specific role gate; visible action buttons do not prove authorization.
- Tests and assets: no dedicated frontend test.
- Relationships: Content Entries, Pages, and backend Content/Comments/Plugins modules.
- Status: `IMPLEMENTED`, `OVERLAPPING`, high confidence.

### FE-FEAT-009: Billing and Usage

- Primary path: `pages/BillingPage.tsx`; billing API methods and types.
- Responsibility and entry: `/billing`; plans, subscription/usage display, plan changes, provider checkout/portal navigation, and usage rebuild.
- UI and layout: AppShell; summary, usage metrics, plan cards, and action buttons.
- State: page-local plans, usage, request/action state, and messages; active membership from global store.
- Backend interaction and schema: plans, usage, plan change, checkout, portal, rebuild.
- Access: UI enables management for owner, admin, and billing manager memberships; backend enforcement is authoritative.
- Tests and assets: no dedicated frontend test.
- Relationships: Organizations, Dashboard, and external billing URLs returned by the backend.
- Status: `IMPLEMENTED`, `OBSERVED`, high confidence.

### FE-FEAT-010: Beta Operations

- Primary path: `pages/BetaPage.tsx`; beta API methods and types.
- Responsibility and entry: `/beta`; organization feedback/dashboard, GA blockers, participant records, and global product dashboard.
- UI and layout: AppShell; metrics, feedback forms/queues, blocker controls, participant controls, and dashboards.
- State: page-local domain data, drafts, request/action state; user and membership from global store.
- Backend interaction and schema: beta dashboard, feedback, blockers, participants, product dashboard.
- Access: owner/admin/editor memberships manage organization beta records; global admin/super-admin sees product dashboard.
- Tests and assets: no dedicated frontend test.
- Relationships: Organizations, global roles, release readiness.
- Status: `IMPLEMENTED`, `OVERLAPPING`, high confidence; actual launch state remains `IMPLEMENTATION_STATUS_UNCLEAR ISU-03`.

### FE-FEAT-011: Marketplace

- Primary path: `pages/MarketplacePage.tsx`; Marketplace and host-adapter API groups; large Marketplace type section.
- Responsibility and entry: `/marketplace`; catalog, details, install/update/rollback/uninstall, purchase, feedback/abuse, creator/listings/packages, review/moderation, runtime controls, finance, analytics, and readiness surfaces.
- UI and layout: AppShell; many sections and page-local helper renderers inside one route component.
- State: extensive page-local catalog, creator, installation, feedback, review, runtime, finance, analytics, draft, and request state; role context from global store.
- Backend interaction and schema: the largest API group, file upload, and Marketplace host adapters; manual contracts.
- Access: organization owner/admin manages installations; global admin/super-admin handles platform review and global runtime controls; backend enforcement remains authoritative.
- Tests and assets: `MarketplacePage.test.tsx` covers selected permission, commerce, lifecycle, runtime, feedback, and analytics behavior with mocks.
- Relationships: Pages/Page Builder, Organizations, Billing/Finance, Authentication, and numerous backend Marketplace modules.
- Status: `IMPLEMENTED`, `OVERLAPPING`, high confidence; `FEATURE_BOUNDARY_UNCLEAR FBU-01`.

### FE-FEAT-012: Settings and Webhooks

- Primary path: `pages/SettingsPage.tsx`; auth/system/webhook API methods.
- Responsibility and entry: `/settings`; current-user information, logout, health/readiness display, static client configuration display, and webhook CRUD/test operations.
- UI and layout: AppShell; account, system, configuration, and webhook panels.
- State: page-local user, probe, webhook, draft, loading, and feedback state; session clearing through global store.
- Backend interaction and schema: current user, logout, health/readiness, and webhooks.
- Access: protected route; webhook action visibility has no independent frontend role gate.
- Tests and assets: no dedicated frontend test.
- Relationships: Authentication, Dashboard health, CMS Webhooks.
- Status: `IMPLEMENTED`, `OVERLAPPING`, high confidence.

### FE-FEAT-013: Localization and Direction

- Primary paths: `i18n/`, `main.tsx`, language selectors in `AuthPage` and `AppShell`, and direction-aware global styles.
- Responsibility and entry: application-root provider; locale detection, persistence, typed message lookup, interpolation, fallback, and document `lang`/`dir` updates.
- UI and layout: shared `LanguageSelect`; affects every rendered page and RTL-sensitive CSS.
- State: React context with `localStorage` persistence; separate from the Zustand store.
- Backend interaction and schema: none; localization is client-owned.
- Access: public and authenticated users can select locale.
- Tests and assets: no dedicated i18n test; bundled Vazirmatn font for RTL presentation.
- Relationships: every UI feature and shared styling.
- Status: `IMPLEMENTED`, `EXPLICIT`, high confidence; older coverage narrative remains `DOCUMENTATION_CODE_CONFLICT DCC-02`.

## Coverage Summary

- Applications mapped: 1.
- Significant frontend features mapped: 13.
- Protected route pages mapped: 12 including the index dashboard and workspace redirect.
- Public route pages mapped: 1.
- Dedicated page test files found: 3.
- Shared source folders without independent feature status: `components`, `hooks`, `services`, `stores`, `styles`, `types`, and test setup.

## Related Documents

- [Feature Boundaries](feature-boundaries.md)
- [Routing](routing.md)
- [Testing Map](testing-map.md)
- [Backend Module Catalog](../backend/module-catalog.md)
- [Frontend Application Map](diagrams/frontend-application-map.mmd)

## Phase 8 Domain Cross-Reference

Frontend features support, but do not authoritatively enforce, the business rules in the [Domain Catalog](../domain/domain-catalog.md). Content, pages, organization members, media, billing, SaaS/beta, plugins, and Marketplace screens expose actions and repeat selected validation. Backend authorization, database constraints, provider state, and side effects remain authoritative. Use the [Business Rule Catalog](../domain/business-rule-catalog.md) to distinguish UI cues from enforced outcomes.

## Extensibility Feature Mapping

MarketplacePage is a fixed host UI for Marketplace workflows. PagesPage combines the component registry with Marketplace component responses and invokes template preview/import APIs. Neither surface dynamically loads package JavaScript. See [Frontend Extension Points](../extensibility/frontend-extension-points.md).
