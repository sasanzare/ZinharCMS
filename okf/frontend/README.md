---
okf_document_id: "frontend-readme"
title: "Frontend Architecture"
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
  - "frontend/src/components"
  - "frontend/src/pages"
  - "frontend/src/services/api.ts"
  - "frontend/src/stores/useAppStore.ts"
  - "frontend/src/i18n"
related_documents:
  - "frontend/overview.md"
  - "frontend/application-catalog.md"
  - "frontend/feature-catalog.md"
  - "frontend/feature-boundaries.md"
  - "frontend/routing.md"
  - "frontend/state-management.md"
  - "frontend/api-client.md"
  - "frontend/page-builder.md"
  - "frontend/testing-map.md"
  - "frontend/frontend-risks.md"
  - "architecture/components.md"
  - "backend/module-catalog.md"
related_diagrams:
  - "frontend/diagrams/frontend-application-map.mmd"
  - "frontend/diagrams/frontend-routing-flow.mmd"
  - "frontend/diagrams/frontend-state-flow.mmd"
  - "frontend/diagrams/frontend-api-flow.mmd"
  - "frontend/diagrams/page-builder-flow.mmd"
uncertainty_markers:
  - "DOCUMENTATION_CODE_CONFLICT DCC-02"
  - "DOCUMENTATION_CODE_CONFLICT DCC-11"
  - "FEATURE_BOUNDARY_UNCLEAR FBU-01"
  - "STATE_OWNERSHIP_UNCLEAR SOU-01"
  - "API_CONTRACT_UNCLEAR ACU-01"
  - "AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01"
  - "UI_BEHAVIOR_UNVERIFIED UBU-01"
---

# Frontend Architecture

## Purpose

This section documents the current React management frontend as implemented. It maps the application, features, routes, layouts, components, state, API integration, access cues, forms, styling, loading and failure behavior, Page Builder, configuration, tests, and risks. It does not prescribe a redesign and does not treat browser-side checks as a security boundary.

## Verified Scope

The repository contains one frontend application: a React 19, TypeScript, Vite single-page management application under `frontend/`. No separate public-site application, design-system package, editor package, shared frontend workspace, Storybook application, or frontend monorepo package boundary was found.

The Phase 4 evidence snapshot is commit `7d25e4cbc53284a78033478e2681d8e9ebeb2fb1`. Current source and executable configuration outrank this documentation if they later diverge.

## Reading Order

1. [Overview](overview.md) for the runtime shape and major findings.
2. [Application Catalog](application-catalog.md) for the application boundary.
3. [Feature Catalog](feature-catalog.md) and [Feature Boundaries](feature-boundaries.md) for ownership.
4. [Routing](routing.md), [Pages and Layouts](pages-and-layouts.md), and [Component Architecture](component-architecture.md) for UI composition.
5. [State Management](state-management.md), [API Client](api-client.md), and [Authentication and Access](authentication-and-access.md) for runtime data flow.
6. [Forms and Validation](forms-and-validation.md), [Styling and Design System](styling-and-design-system.md), and [Loading, Errors, and Notifications](loading-errors-and-notifications.md) for shared UI behavior.
7. [Page Builder](page-builder.md) for the largest editor workflow.
8. [Configuration and Build](configuration-and-build.md), [Testing Map](testing-map.md), and [Frontend Risks](frontend-risks.md) for delivery and follow-up.

## Feature Documents

The [feature catalog](feature-catalog.md) selects 13 significant verified features. Each has an owning document under [`features/`](features/) with identity, source ownership, entry points, state, backend interaction, access behavior, UI composition, tests, and uncertainty.

## Evidence Rules

- `VERIFIED` means the statement is directly supported by current source, manifest, configuration, or tests.
- `INFERRED_FROM_CODE` and `INFERRED_FROM_STRUCTURE` identify conclusions not governed by an explicit policy.
- Missing code is not automatically future work. A capability is `PLANNED_NOT_IMPLEMENTED` only when planning evidence exists.
- Frontend role checks describe visibility and interaction only. Backend enforcement remains authoritative.
- Detailed endpoint, database, and security references are deferred to Phases 5 through 7 and are linked rather than duplicated here.

## Related System Views

- [Architecture Components](../architecture/components.md)
- [Architecture Boundaries](../architecture/boundaries.md)
- [Architecture Dependency Model](../architecture/dependency-model.md)
- [Architecture Runtime Flows](../architecture/runtime-flows.md)
- [Backend Module Catalog](../backend/module-catalog.md)
- [Backend Testing Map](../backend/testing-map.md)

