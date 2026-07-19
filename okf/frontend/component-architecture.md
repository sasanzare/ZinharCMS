---
okf_document_id: "frontend-component-architecture"
title: "Frontend Component Architecture"
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
  - "frontend/src/components"
  - "frontend/src/pages"
  - "frontend/src/hooks/useHealth.ts"
  - "frontend/src/i18n"
  - "frontend/src/styles/index.css"
related_documents:
  - "frontend/pages-and-layouts.md"
  - "frontend/feature-boundaries.md"
  - "frontend/forms-and-validation.md"
  - "frontend/styling-and-design-system.md"
related_diagrams:
  - "frontend/diagrams/frontend-application-map.mmd"
uncertainty_markers:
  - "COMPONENT_OWNERSHIP_UNCLEAR COU-01"
  - "INFERRED_FROM_STRUCTURE"
  - "UI_BEHAVIOR_UNVERIFIED UBU-01"
---

# Frontend Component Architecture

## Component Layers

| Layer | Current source | Responsibility | Boundary status |
|---|---|---|---|
| Root composition | `main.tsx` | React root, StrictMode, i18n, router, global CSS | `EXPLICIT` |
| Route composition | `router.tsx` | Map URLs to page elements and protected shell | `EXPLICIT` |
| Shared application components | `components/` | Shell, auth admission, generated form, status badge | `OBSERVED` |
| Shared hook | `hooks/useHealth.ts` | Poll health/readiness | `OBSERVED` |
| Localization components | `i18n/` | Provider, context, hook, selector, messages | `EXPLICIT` |
| Route pages | `pages/` | Feature UI, orchestration, local state, API calls | `OVERLAPPING` for complex pages |
| Page-local components | `PagesPage.tsx`, `MarketplacePage.tsx`, other pages | Feature-specific rendering helpers | `INFERRED_FROM_STRUCTURE` |

## Shared Component Catalog

### `AppShell`

Owns protected application chrome, static navigation, route title, organization selection, locale selection, readiness polling, user-role display, logout, and child outlet. It is both a layout and a cross-cutting integration component.

### `RequireAuth`

Reads the access token from Zustand and chooses `AppShell` or a redirect. It has no token verification, refresh, role, or loading behavior.

### `DynamicForm`

Renders content fields from `FieldSchema`. It supports boolean, long text/rich text, number, datetime, and default text input behavior. It converts numbers with `Number`, represents blank numeric input as `null`, and uses native `required`. It does not expose a validation result, touched state, or structured field error contract.

### `StatusBadge`

Maps a label and tone to a styled status element. Pages use it for loading, success, warning, errors, lifecycle states, and metrics. It is presentation-only and not a notification transport.

## Shared Hook Catalog

`useHealth` calls health and readiness in parallel, stores the last results, polls every 15 seconds by default, and stops its interval on unmount. It prevents updates after cleanup but does not abort in-flight requests. The shell and dashboard can each instantiate it.

## Page-Local Component Examples

`PagesPage.tsx` declares palette, sortable canvas node, canvas, preview node, local preview, and property-control components in the same file. `MarketplacePage.tsx` declares helper renderers and formatting/status functions around a very large route component. Other pages use smaller local helper functions rather than separate component modules.

These components are implemented and used, but file-local visibility should not be interpreted as a deliberately governed internal API.

## Props and Contract Style

- TypeScript types describe props at declaration sites.
- No component schema/runtime prop validation is used.
- Shared components accept narrow callbacks and values; route pages read global state and API modules directly.
- API response types flow directly into many UI components.
- No view-model layer, presenter layer, component registry package, or frontend domain model package was found.

## Composition Direction

The dominant direction is root to router to guard/shell to page, while pages depend inward on shared components, hooks, i18n, store, API, and types. Shared components do not import route pages. `AppShell` is the main exception to a presentation-only component because it calls the API and integrates several state owners.

## Design-System Status

No independent design-system package, Storybook, component documentation site, token file, or visual-regression suite was found. Semantic global CSS classes and the four shared components form an informal UI vocabulary. This is `COMPONENT_OWNERSHIP_UNCLEAR COU-01`, not evidence that no design decisions exist.

## Accessibility Evidence

Source contains semantic `main`, `nav`, `aside`, `header`, labels, tables, buttons, and many icon `aria-hidden` and button `aria-label` attributes. Some compact visible labels use visually hidden CSS. No dedicated accessibility test, automated scanner, focus-management layer, or documented keyboard contract was found. Runtime accessibility remains `UI_BEHAVIOR_UNVERIFIED UBU-01`.

## Change Guidance

- Keep a shared component's state and side effects explicit; `AppShell` already has broad impact.
- Verify all route consumers before changing global semantic class behavior.
- For Page Builder changes, test both nested sortable rendering and preview rendering because they are separate component paths.
- Do not claim a formal component library without ownership, supported APIs, documentation, and test evidence.

## Related Documents

- [Pages and Layouts](pages-and-layouts.md)
- [Feature Boundaries](feature-boundaries.md)
- [Forms and Validation](forms-and-validation.md)
- [Styling and Design System](styling-and-design-system.md)

## Extensible Component Boundary

Page Builder component registration is metadata-driven. System and tenant records plus Marketplace definitions provide keys and props schemas, while host React code remains responsible for rendering. See [Component and Block Registration](../extensibility/component-and-block-registration.md).
