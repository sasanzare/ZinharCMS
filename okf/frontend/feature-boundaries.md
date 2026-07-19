---
okf_document_id: "frontend-feature-boundaries"
title: "Frontend Feature Boundaries"
project: "ZinharCMS"
category: "frontend"
phase: 4
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "mixed"
last_verified_commit: "7d25e4cbc53284a78033478e2681d8e9ebeb2fb1"
last_verified_date: "2026-07-17"
primary_sources:
  - "frontend/src/router.tsx"
  - "frontend/src/pages"
  - "frontend/src/components"
  - "frontend/src/hooks"
  - "frontend/src/services/api.ts"
  - "frontend/src/stores/useAppStore.ts"
  - "frontend/src/types/api.ts"
related_documents:
  - "frontend/feature-catalog.md"
  - "frontend/component-architecture.md"
  - "frontend/state-management.md"
  - "frontend/api-client.md"
  - "architecture/boundaries.md"
  - "backend/module-boundaries.md"
related_diagrams:
  - "frontend/diagrams/frontend-application-map.mmd"
  - "frontend/diagrams/frontend-state-flow.mmd"
uncertainty_markers:
  - "FEATURE_BOUNDARY_UNCLEAR FBU-01"
  - "FEATURE_BOUNDARY_UNCLEAR FBU-02"
  - "COMPONENT_OWNERSHIP_UNCLEAR COU-01"
  - "STATE_OWNERSHIP_UNCLEAR SOU-01"
  - "RESPONSIBILITY_OVERLAP FRO-01"
  - "RESPONSIBILITY_OVERLAP FRO-02"
---

# Frontend Feature Boundaries

## Boundary Model

The current frontend has technical folders but no feature directories or package-enforced feature imports. A route page is therefore the most consistent observed feature boundary, while shared components, store state, API groups, types, i18n, and CSS cross those boundaries. `EXPLICIT` means a module has a deliberate exported boundary; `OBSERVED` means current source ownership is reasonably cohesive; `OVERLAPPING` means responsibility or state crosses multiple feature areas.

## Ownership Matrix

| Feature | Routes | Pages and layouts | Components and hooks | State | API and schemas | Tests | Permission cues | Assets and styles | Boundary |
|---|---|---|---|---|---|---|---|---|---|
| FE-FEAT-001 Authentication and Session | `/login`; protected parent | `AuthPage`; `RequireAuth`; `AppShell` exit | `StatusBadge`; i18n | Zustand plus local form and `localStorage` | `api.auth`; auth types | None dedicated | Token-presence guard | Global auth classes | `OVERLAPPING` |
| FE-FEAT-002 Dashboard and Shell | `/` and every protected child | `DashboardPage`; `AppShell` | `useHealth`; `StatusBadge`; `LanguageSelect` | Zustand, health hook, local metrics | health, readiness, content counts, billing usage | Dashboard page test | No menu filtering | Lucide and global shell/dashboard CSS | `OVERLAPPING` |
| FE-FEAT-003 Organizations and Workspaces | `/organization`; `/workspace/:slug` | `OrganizationPage`; `WorkspaceRedirectPage`; shell switcher | Shared status and i18n | Zustand organization context plus local control-plane state | `api.organizations`; organization types | None dedicated | owner/admin and owner-specific controls | Lucide and global organization CSS | `OVERLAPPING` |
| FE-FEAT-004 Content Modeling | `/content-types` | `ContentTypesPage` | Shared status and i18n | Local list/editor state | `api.contentTypes`; field schemas | None dedicated | No route-specific cue | Global content-type CSS | `OBSERVED` |
| FE-FEAT-005 Content Entries | `/entries` | `EntriesPage` | `DynamicForm`; `StatusBadge` | Local entry/form state | content-types and entries; JSON/field types | None dedicated | No route-specific cue | Global entry/form CSS | `OVERLAPPING` |
| FE-FEAT-006 Media Library | `/media` | `MediaPage` | Shared status and i18n | Local file/list/editor state | `api.media`; multipart and media types | None dedicated | No route-specific cue | Global media CSS | `OBSERVED` |
| FE-FEAT-007 Pages and Page Builder | `/pages` | `PagesPage` and page-local builder components | dnd-kit and page-local palette/canvas/preview/props | Large local builder/server state plus global token/org | pages, components, Marketplace adapters; page JSON types | Pages page test | No page-level role cue | Global builder CSS | `OVERLAPPING` |
| FE-FEAT-008 Editorial Workflow | `/workflow` | `WorkflowPage` | Shared status and page-local renderers | Local queue/comments/plugins | entries, pages, comments, plugins | None dedicated | No route-specific cue | Global workflow CSS | `OVERLAPPING` |
| FE-FEAT-009 Billing and Usage | `/billing` | `BillingPage` | Shared status and i18n | Local billing state plus membership | `api.billing`; billing types | None dedicated | owner/admin/billing manager | Global billing CSS | `OBSERVED` |
| FE-FEAT-010 Beta Operations | `/beta` | `BetaPage` | Shared status and page-local helpers | Local beta state plus user/membership | `api.beta`; beta types | None dedicated | member manager and global admin cues | Global beta CSS | `OVERLAPPING` |
| FE-FEAT-011 Marketplace | `/marketplace` | `MarketplacePage` | Many page-local helpers; shared status | Very large local state plus user/membership | Marketplace and adapter groups; large type surface | Marketplace page test | organization and global role cues | Global Marketplace CSS | `OVERLAPPING` |
| FE-FEAT-012 Settings and Webhooks | `/settings` | `SettingsPage` | Shared status and i18n | Local account/probe/webhook state plus session clear | auth, probes, webhooks | None dedicated | No webhook-specific role cue | Global settings CSS | `OVERLAPPING` |
| FE-FEAT-013 Localization and Direction | Root-wide | `I18nProvider`; selectors in auth/shell | Explicit `i18n/index.ts` exports | React context plus `localStorage` | No backend API | None dedicated | Available before and after auth | Dictionaries, Vazirmatn font, RTL CSS | `EXPLICIT` |

## Shared Ownership

| Shared concern | Current owner | Consumers | Boundary issue |
|---|---|---|---|
| Route composition | `router.tsx` | All pages | Route existence and feature ownership are centralized but not metadata-driven. |
| Auth and organization persistence | `useAppStore.ts`, `api.ts`, browser storage | Guard, shell, every authenticated request | `STATE_OWNERSHIP_UNCLEAR SOU-01`: store state, module variables, and persistence are coordinated by imperative setters. |
| API transport | `services/api.ts` | Every data feature | Central transport is cohesive, but one file also owns all domain methods. |
| Browser contracts | `types/api.ts` | API and pages | Manual cross-application duplication; no feature-local schema authority. |
| Shared components | `components/` | Most features | `COMPONENT_OWNERSHIP_UNCLEAR COU-01`: four shared components exist, but no formal UI-library ownership or contract. |
| Loading and messages | Individual pages plus `StatusBadge` | Every data feature | Behavior is repeated; no centralized async or notification boundary. |
| Styling | `styles/index.css` | Entire application | Semantic classes are globally addressable; ownership is convention-based. |
| Localization | `i18n/` | Entire application | Explicit module boundary, but message ownership is one global dictionary. |

## Overlap Register

### FBU-01: Marketplace Feature Concentration

`MarketplacePage.tsx` owns customer catalog and installs, paid purchase, feedback and abuse reporting, creator onboarding, listing/package submission, platform review, runtime kill switches, finance, and analytics. Backend documentation separates these concerns into multiple modules. The frontend boundary is therefore operationally real but too broad to identify one stable domain owner.

### FBU-02: Dashboard and Shell Boundary

`AppShell` owns layout, navigation, organization switching, locale switching, health polling, user identity display, and logout. Dashboard separately polls health and calls multiple domain APIs. These are shared platform responsibilities, not a single domain feature.

### FRO-01: Pages, Workflow, and Marketplace Adapters

The Pages feature consumes page workflow operations and Marketplace-provided components/templates. Editorial Workflow also mutates pages. Source ownership does not isolate these flows.

### FRO-02: Session, Organization, Health, and Localization in Shell

The application shell integrates four cross-cutting contexts. A change to shell composition can affect all protected features even when the feature route itself is unchanged.

## Boundary Rules for Changes

These are review rules derived from current evidence, not claims of automated enforcement:

- Map a new route to one primary feature and record every shared concern it consumes.
- Keep browser authorization cues descriptive; do not move backend authorization responsibility into route visibility.
- Route HTTP calls through the central API boundary unless a documented exception is introduced.
- Update manual client types together with backend contract changes until a shared or generated contract exists.
- Treat `MarketplacePage.tsx`, `OrganizationPage.tsx`, and `PagesPage.tsx` as high-impact change areas.
- Do not present a directory move as a completed boundary improvement unless imports, state ownership, tests, and public interfaces support it.

## Related Documents

- [Feature Catalog](feature-catalog.md)
- [Component Architecture](component-architecture.md)
- [State Management](state-management.md)
- [API Client](api-client.md)
- [System Boundaries](../architecture/boundaries.md)
- [Backend Module Boundaries](../backend/module-boundaries.md)

## Phase 9 Boundary Note

The frontend owns rendering and API orchestration; Marketplace manifests provide data, not React modules. Declarative public hook names do not establish a general navigation/widget/form injector in inspected frontend code. See [Frontend Extension Points](../extensibility/frontend-extension-points.md).
