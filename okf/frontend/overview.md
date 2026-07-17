---
okf_document_id: "frontend-overview"
title: "Frontend Overview"
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
  - "frontend/package.json"
  - "frontend/src/main.tsx"
  - "frontend/src/router.tsx"
  - "frontend/src/components/AppShell.tsx"
  - "frontend/src/services/api.ts"
  - "frontend/src/stores/useAppStore.ts"
  - "frontend/src/styles/index.css"
related_documents:
  - "frontend/README.md"
  - "frontend/application-catalog.md"
  - "frontend/feature-catalog.md"
  - "frontend/routing.md"
  - "frontend/state-management.md"
  - "frontend/api-client.md"
  - "frontend/frontend-risks.md"
  - "architecture/overview.md"
related_diagrams:
  - "frontend/diagrams/frontend-application-map.mmd"
uncertainty_markers:
  - "INFERRED_FROM_STRUCTURE"
  - "IMPLEMENTATION_STATUS_UNCLEAR ISU-03"
  - "UI_BEHAVIOR_UNVERIFIED UBU-01"
---

# Frontend Overview

## Classification

`VERIFIED`: ZinharCMS has one browser application under `frontend/`. It is an authenticated management SPA built with React 19, React Router, TypeScript, Vite, Zustand, native `fetch`, a first-party i18n context, global CSS, Tailwind CSS tooling, Lucide icons, and dnd-kit for Page Builder interaction.

`INFERRED_FROM_STRUCTURE`: The frontend is organized by technical role (`pages`, `components`, `services`, `stores`, `i18n`, `types`) rather than by feature-owned directories. Route pages therefore act as the dominant feature composition and state boundaries.

## Runtime Shape

1. `main.tsx` mounts one React root in `StrictMode`.
2. `I18nProvider` establishes locale, translation, and document direction state.
3. `RouterProvider` activates the browser router.
4. `/login` renders outside the protected shell.
5. Every other known page sits below `RequireAuth` and `AppShell`.
6. Pages call the central API object and keep most server responses in component-local state.
7. The Zustand store holds session, organization, and sidebar state; the i18n context separately holds locale.
8. The API client adds bearer and organization headers and calls the Rust backend.

## Application Responsibilities

The SPA provides management surfaces for authentication, dashboard health and usage, organizations and workspaces, content types, entries, media, pages and visual composition, editorial workflow, billing, beta operations, Marketplace lifecycles, settings, webhooks, and localization.

It does not implement a verified public delivery renderer. The Page Builder preview visible inside `PagesPage` renders a local interpretation of page JSON, while a preview WebSocket URL is copied for an external preview client. Public delivery and backend preview ownership are described in [Backend Modules](../backend/module-catalog.md).

## Architectural Characteristics

| Concern | Current implementation | Status |
|---|---|---|
| Application boundary | One Vite SPA and one production Nginx static image | `VERIFIED` |
| Routing | One eager route table; protected shell; wildcard redirect | `VERIFIED` |
| Feature organization | Route pages plus shared technical folders | `INFERRED_FROM_STRUCTURE` |
| Global client state | One Zustand store for session, organization, and shell | `VERIFIED` |
| Server state | Page-local `useState`/`useEffect`; no query cache found | `VERIFIED` |
| API integration | One typed-looking but manually declared `fetch` client | `VERIFIED` |
| Forms | Controlled React state and native constraints | `VERIFIED` |
| Styling | Tailwind import plus a large shared semantic-class stylesheet | `VERIFIED` |
| Localization | English and Persian dictionaries with LTR/RTL switching | `VERIFIED` |
| Page Builder | dnd-kit palette/canvas/property editor/local preview/version flow | `VERIFIED` |
| Testing | Three page-level Vitest/Testing Library files | `VERIFIED` |
| Deployment state | Images and Compose are present; actual environment is not proven | `IMPLEMENTATION_STATUS_UNCLEAR ISU-03` |
| Browser behavior | Responsive CSS exists; cross-browser, accessibility, and visual execution were not run | `UI_BEHAVIOR_UNVERIFIED UBU-01` |

## Major Findings

- All route modules are imported eagerly; no `React.lazy`, route loader, route action, or route error element was found.
- The shell exposes all navigation links to authenticated users. Several pages conditionally hide or disable actions by role, but the frontend is not an authorization boundary.
- Most page components own fetching, mutation state, loading, errors, and success messages directly. Marketplace, Organizations, and Pages are especially concentrated responsibility hubs.
- Browser API contracts are manually duplicated in `types/api.ts`; no generated client or runtime response validation was found.
- `react-hook-form`, `@hookform/resolvers`, and `zod` are declared dependencies but have no verified source import. Phase Zero convention claims are corrected in [Forms and Validation](forms-and-validation.md) under `DOCUMENTATION_CODE_CONFLICT DCC-11`.
- The first-party i18n implementation is broader than the coverage statement in `docs/I18N.md`, resolving the current-state question while retaining `DOCUMENTATION_CODE_CONFLICT DCC-02` for the older narrative.
- No Error Boundary, toast/notification subsystem, client monitoring SDK, end-to-end test suite, visual test suite, or Storybook configuration was found.

## Architectural Boundary

The SPA is an untrusted browser client. It can improve navigation and prevent accidental actions, but identity, tenant membership, role, input, and resource checks must remain enforced by the backend. See [Authentication and Access](authentication-and-access.md), [Architecture Boundaries](../architecture/boundaries.md), and the Phase 3 [Tenant Authorization module](../backend/modules/tenant-authorization.md).

## Related Documents

- [Application Catalog](application-catalog.md)
- [Feature Catalog](feature-catalog.md)
- [Routing](routing.md)
- [State Management](state-management.md)
- [API Client](api-client.md)
- [Frontend Risks](frontend-risks.md)
- [Frontend Application Map](diagrams/frontend-application-map.mmd)

