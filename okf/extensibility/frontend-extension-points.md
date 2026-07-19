---
okf_document_id: "frontend-extension-points"
title: "Frontend Extension Points"
project: "ZinharCMS"
category: "extensibility"
phase: 9
status: "current"
source_of_truth: false
implementation_view: "observed"
extensibility_status: "partially_verified"
last_verified_commit: "56d733985fdd7aa3f25ee6981b88cf29c52f65c9"
last_verified_date: "2026-07-19"
primary_sources:
  - "frontend/src/pages/PagesPage.tsx"
  - "frontend/src/pages/MarketplacePage.tsx"
  - "frontend/src/services/api.ts"
  - "backend/src/routes/marketplace_adapters.rs"
related_documents:
  - "../frontend/component-architecture.md"
  - "../frontend/page-builder.md"
  - "component-and-block-registration.md"
related_diagrams:
  - "diagrams/component-registration.mmd"
---

# Frontend Extension Points

The verified frontend extension behavior is data-driven:

- PagesPage loads standard component-registry records and Marketplace component responses, then combines them for the builder.
- It loads active Marketplace design-template installations and calls host preview/import endpoints.
- MarketplacePage exposes catalog, creator/review, installation, runtime, finance, feedback, and analytics flows through the typed API client.

These are host-rendered React screens. No runtime JavaScript module loader, remote module federation, dynamic script insertion, plugin route registration, plugin menu injection, or third-party React component execution was found.

Ordinary React imports and component composition are not extension points. Declarative marketplace hooks named sidebar.item or dashboard.widget are returned by backend APIs, but inspected frontend code does not establish a general renderer/injector for them. COMPONENT_REGISTRATION_UNCLEAR therefore applies beyond the Page Builder catalog, and MARKETPLACE_BEHAVIOR_UNCLEAR applies to general UI hook placement.

## Frontend Surface Matrix

| Surface | Registry | Source path | Contract | Registration | Rendering location | State/API access | Permission checks | Scope | Error boundary | Lazy loading | Tests | Confidence |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Page Builder component catalog | Backend component registry plus Marketplace component response | frontend/src/pages/PagesPage.tsx | Component key/name/category/props schema | Host API data load | Builder palette/editor | Page-local state and typed API | Backend RBAC/RLS; frontend action gating | System or tenant | Page-level error/notification behavior | No package-module loading | PagesPage test | High |
| Marketplace design template | Active installation list and adapter preview/import | PagesPage.tsx | Template preview/import DTOs | Host Marketplace install | Template selection and page result | Typed API and local modal/form state | Backend page writer/install/runtime gates | Tenant | Page notifications; no third-party boundary | No | PagesPage test | High |
| Marketplace management UI | Fixed React page/routes | MarketplacePage.tsx | Typed Marketplace APIs | Static React/router registration | Marketplace page | Store/context/API client | Frontend role cues plus backend authority | Tenant/global by action | Host page error messages | Static application chunks only | Twelve tests | High |
| Public sidebar/menu/widget/form hook | No verified frontend registry | No consumer found | Backend declaration exists only | None found | None verified | None | Backend authorization only | Tenant declaration | None | None | No renderer test | Low; HOOK_DELIVERY_UNCLEAR |
| Plugin routes/pages/navigation/settings pages/tables/actions/modals/themes/editors/toolbars/localization/notifications | No third-party registry found | Static frontend source | Ordinary React imports | Build-time host code | Fixed host UI | Host application | Host route/action checks | Application-defined | Host boundaries | Vite application loading, not plugins | Existing frontend tests | High confidence absent as plugin surface |

No third-party code receives direct Zustand state, API client credentials, DOM access, browser storage access, or a React error boundary because no frontend code loader exists.
