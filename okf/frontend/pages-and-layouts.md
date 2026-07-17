---
okf_document_id: "frontend-pages-layouts"
title: "Frontend Pages and Layouts"
project: "ZinharCMS"
category: "frontend"
phase: 4
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "7d25e4cbc53284a78033478e2681d8e9ebeb2fb1"
last_verified_date: "2026-07-17"
primary_sources:
  - "frontend/src/router.tsx"
  - "frontend/src/components/AppShell.tsx"
  - "frontend/src/pages"
  - "frontend/src/styles/index.css"
related_documents:
  - "frontend/routing.md"
  - "frontend/component-architecture.md"
  - "frontend/feature-catalog.md"
  - "frontend/loading-errors-and-notifications.md"
related_diagrams:
  - "frontend/diagrams/frontend-routing-flow.mmd"
uncertainty_markers:
  - "COMPONENT_OWNERSHIP_UNCLEAR COU-01"
  - "UI_BEHAVIOR_UNVERIFIED UBU-01"
---

# Frontend Pages and Layouts

## Layout Inventory

| Layout | Source | Applies to | Composition | State dependencies | Status |
|---|---|---|---|---|---|
| Authentication page layout | `AuthPage.tsx` | `/login` | Page-owned authentication card and locale selector | Local form state and i18n | `VERIFIED` |
| Protected application shell | `AppShell.tsx` | Every protected route | Sidebar, top bar, organization switcher, locale, health, user, logout, `Outlet` | Zustand, `useHealth`, API logout, i18n | `VERIFIED` |
| Page-level panel layouts | Individual page modules | One feature route | Semantic sections, grids, tables, cards, forms | Page-local data and request state | `INFERRED_FROM_STRUCTURE` |
| Page Builder layout | `PagesPage.tsx` | `/pages` | Palette, canvas/local preview, property panel, templates, pages, versions | Builder-local state and global session context | `VERIFIED` |

There is no separate layout directory, route-level layout metadata, public-site layout, or feature-owned nested route layout.

## Page Inventory

| Page | Primary user purpose | Main composition | Shared dependencies | Local async ownership |
|---|---|---|---|---|
| `AuthPage` | Login or register | Mode switch, credential/profile fields, locale selector, status | i18n, API, store, `StatusBadge` | Auth request and error |
| `DashboardPage` | Read system and CMS summary | Foundation cards, health/readiness, content and usage metrics | `useHealth`, API, i18n, status | Stats and usage requests |
| `ContentTypesPage` | Manage content models | Filter/list, model editor, field schema controls | API, i18n, status | Model CRUD |
| `EntriesPage` | Manage content records | Type selector, list, generated editor, workflow actions | `DynamicForm`, API, i18n, status | Type and entry CRUD/status |
| `MediaPage` | Manage media | Upload, media list/grid, metadata editor | API, i18n, status | List/upload/update/delete |
| `MarketplacePage` | Operate Marketplace lifecycles | Catalog, install, runtime, creator, finance, review, feedback, analytics sections | API, store, i18n, status | Many parallel and action requests |
| `PagesPage` | Compose and manage pages | Builder shell, templates, page list, version list | dnd-kit, API, store, i18n, status | Builder load/save/version/workflow |
| `WorkflowPage` | Review content/pages | Queue, detail, comments, plugin list | API, i18n, status | Queue/comment/plugin operations |
| `OrganizationPage` | Administer organizations | Details, workspace, domains, limits, audit, alerts, members, invitations | API, store, i18n, status | Control-plane operations |
| `WorkspaceRedirectPage` | Select organization by slug | Transient loading or failure message | API, store, router, i18n | Organization resolution |
| `BillingPage` | Manage plan and usage | Current subscription, usage metrics, plans, provider actions | API, store, i18n, status | Billing load/actions |
| `BetaPage` | Operate beta readiness | Metrics, feedback, blockers, participants, product dashboard | API, store, i18n, status | Beta load/actions |
| `SettingsPage` | Manage account/system/webhooks | Account, health/config, webhook list/editor | API, store, i18n, status | User/probes/webhooks |

## AppShell Composition

The shell has two primary columns: a sidebar and workspace. The workspace contains a top bar and `main.content-area`. The main element uses the active organization ID as its React `key`, forcing the current route subtree to remount after organization changes. This is an intentional current mechanism for discarding route-local tenant data, but no formal state-reset contract or test was found.

The shell also polls health independently of the dashboard. Consequently, visiting the dashboard can create two health/readiness polling owners.

## Page Composition Conventions

`INFERRED_FROM_CODE`:

- `page-stack` is the dominant page root.
- `panel`, `panel-header`, `panel-actions`, metric grids, data tables, status stacks, toolbars, and form grids are global composition conventions.
- Pages define local helper functions and sometimes page-local components rather than importing a feature component tree.
- Loading, error, success, and empty states are usually inline within page composition.
- User-facing labels primarily use the i18n function, but hard-coded English remains in selected page content and builder/template prompts.

These are current implementation patterns, not a formally versioned design-system contract.

## Responsive and Direction Behavior

The global stylesheet contains multiple responsive media queries and RTL selectors. The shell, builder, Marketplace, organization, and table layouts have breakpoint-specific rules. The i18n provider updates root direction.

`UI_BEHAVIOR_UNVERIFIED UBU-01`: Phase 4 did not execute browser screenshots, keyboard walkthroughs, screen-reader checks, or a device matrix, so CSS presence does not establish usable behavior at every viewport or locale.

## Ownership Risks

- Route modules own both data orchestration and complex presentation.
- Global class names allow changes in one stylesheet area to affect unrelated pages.
- Page-local components in `PagesPage` and `MarketplacePage` are not independently tested or reusable boundaries.
- Organization remount behavior depends on the shell key and page effects rather than a centralized tenant-transition protocol.

## Related Documents

- [Routing](routing.md)
- [Component Architecture](component-architecture.md)
- [Loading, Errors, and Notifications](loading-errors-and-notifications.md)
- [Frontend Routing Flow](diagrams/frontend-routing-flow.mmd)

