---
type: Component
title: Frontend Feature Boundaries
description: Current React/Vite administrative feature areas, shared client boundaries, and organization-aware application flow.
status: draft
sources:
  - id: source-main
    resource: https://github.com/sasanzare/ZinharCMS/blob/e37e94e2e6960a2547f33bf1ebb4225f818b3a4b/frontend/src/main.tsx
    title: frontend/src/main.tsx at Phase 4 source head
  - id: source-router
    resource: https://github.com/sasanzare/ZinharCMS/blob/e37e94e2e6960a2547f33bf1ebb4225f818b3a4b/frontend/src/router.tsx
    title: frontend/src/router.tsx at Phase 4 source head
  - id: source-shell
    resource: https://github.com/sasanzare/ZinharCMS/blob/e37e94e2e6960a2547f33bf1ebb4225f818b3a4b/frontend/src/components/AppShell.tsx
    title: frontend/src/components/AppShell.tsx at Phase 4 source head
  - id: source-api
    resource: https://github.com/sasanzare/ZinharCMS/blob/e37e94e2e6960a2547f33bf1ebb4225f818b3a4b/frontend/src/services/api.ts
    title: frontend/src/services/api.ts at Phase 4 source head
  - id: source-store
    resource: https://github.com/sasanzare/ZinharCMS/blob/e37e94e2e6960a2547f33bf1ebb4225f818b3a4b/frontend/src/stores/useAppStore.ts
    title: frontend/src/stores/useAppStore.ts at Phase 4 source head
  - id: source-i18n
    resource: https://github.com/sasanzare/ZinharCMS/blob/e37e94e2e6960a2547f33bf1ebb4225f818b3a4b/frontend/src/i18n/I18nProvider.tsx
    title: frontend/src/i18n/I18nProvider.tsx at Phase 4 source head
  - id: source-package
    resource: https://github.com/sasanzare/ZinharCMS/blob/e37e94e2e6960a2547f33bf1ebb4225f818b3a4b/frontend/package.json
    title: frontend/package.json at Phase 4 source head
---

# Shared application boundary

The administrative frontend is a React/Vite single-page application. The
entry point composes strict-mode rendering, internationalization, session
bootstrap, and the client router. `RequireAuth` guards the authenticated
route tree, while `AppShell` owns the shared navigation, organization switcher,
language direction, health state, and logout affordances.

## Feature areas

The route and API surfaces form the current feature boundary rather than a
set of independently deployed frontend applications. Content modeling,
entries, workflow, comments, and built-in plugin interactions make up the
editorial area. Pages, page-builder components, preview, media, and related
adapters form the page and asset area. Marketplace, billing, beta operations,
organization/workspace administration, settings, and the dashboard are
separate page-level areas within the same authenticated shell.

The router exposes these areas through the current page components, while the
shared API module also exposes the corresponding content, page, media,
Marketplace, billing, organization, plugin, comment, and webhook calls. The
feature grouping is a current source-derived map; it is not a claim about
future package or team ownership.

## Network and session boundary

Feature pages call the centralized API service. Its request wrapper builds the
API origin, attaches the volatile bearer token and active organization header,
supports step-up headers and JSON/FormData/blob responses, and handles the
stable invalid-access-token refresh and replay path. The server remains the
authority for authentication and authorization.

The Zustand application store keeps the active access token in process memory.
It stores browser-safe projections for user, organization, and active-
organization state, while session bootstrap and logout coordinate with the
API boundary. This Concept does not duplicate the detailed session contract in
[routing and state](/frontend/routing-and-state.md) or [authentication and session contract](/api/authentication-and-session-contract.md).

## Presentation and localization boundary

Shared components provide the shell, authentication guard, dynamic forms,
status display, rich-text safety, session bootstrap, and step-up interaction.
The internationalization provider selects supported locales, updates document
language and direction, and persists the locale projection in browser storage.
The repository does not establish an owner-approved frontend decomposition,
accessibility policy, or browser-compatibility matrix.

## Relationships

The overall shell and route surface are described in [admin application](/frontend/admin-application.md), and token, organization, and refresh behavior are in [routing and state](/frontend/routing-and-state.md). Backend persistence and service composition are in [persistence, services, and configuration](/backend/persistence-services-and-configuration.md), while API families are in [API contract overview](/api/api-contract-overview.md). Editorial and page-builder behavior remains in [content and editorial workflow](/domain/content-and-editorial-workflow.md) and [page builder and preview](/domain/page-builder-and-preview.md).

## Open decision dependencies

NOC-12 covers schema evolution, workflow compatibility, page compatibility,
browser/session policy, accessibility, and frontend decomposition questions.
NOC-18 covers preferred terminology and abbreviations. The current feature
boundaries are recorded without resolving either owner decision.
