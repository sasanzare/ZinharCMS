---
okf_document_id: "frontend-testing-map"
title: "Frontend Testing Map"
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
  - "frontend/vitest.config.ts"
  - "frontend/src/test/setup.ts"
  - "frontend/src/pages/DashboardPage.test.tsx"
  - "frontend/src/pages/PagesPage.test.tsx"
  - "frontend/src/pages/MarketplacePage.test.tsx"
  - ".github/workflows/frontend-ci.yml"
related_documents:
  - "frontend/feature-catalog.md"
  - "frontend/page-builder.md"
  - "frontend/loading-errors-and-notifications.md"
  - "frontend/frontend-risks.md"
  - "backend/testing-map.md"
related_diagrams:
  - "frontend/diagrams/frontend-application-map.mmd"
uncertainty_markers:
  - "UI_BEHAVIOR_UNVERIFIED UBU-01"
  - "UNKNOWN"
---

# Frontend Testing Map

## Test Stack

`VERIFIED`: Vitest runs in jsdom with `@testing-library/react` and `@testing-library/jest-dom`. `src/test/setup.ts` installs jest-dom matchers. CI runs lint, typecheck, the test suite, and the production build.

No frontend coverage report, threshold, end-to-end framework, real-browser suite, accessibility scanner, visual regression suite, contract test harness, Storybook tests, or performance test was found.

## Test File Inventory

| Test file | Main coverage | Dependency strategy | Important omissions |
|---|---|---|---|
| `DashboardPage.test.tsx` | Renders foundation technology cards | Mocks `useHealth` | Dashboard API stats/usage, loading, errors, polling, localization |
| `PagesPage.test.tsx` | Renders builder shell, system and Marketplace components, empty canvas, props editor | Mocks API module | Drag/drop, save/autosave, errors, versions, workflow, templates, preview, clipboard |
| `MarketplacePage.test.tsx` | Permission confirmation, organization-role gating, paid checkout, duplicate/blocked installs, update approval, kill switch, reviews/reports, moderation, creator/admin analytics | Mocks API and store state | Real transport/contracts, routing, all creator/submission/finance paths, accessibility, browser integration |

The Marketplace test file contains 12 observed test cases. Dashboard and Pages contain one case each, for 14 test cases total at the evidence snapshot.

## Feature Coverage Matrix

| Feature | Dedicated test | Observed confidence |
|---|---|---|
| Authentication and Session | No | Low frontend behavior coverage |
| Dashboard and Shell | Dashboard only; shell not directly tested | Partial |
| Organizations and Workspaces | No | Low |
| Content Modeling | No | Low |
| Content Entries and DynamicForm | No | Low |
| Media Library | No | Low |
| Pages and Page Builder | One shell-render test | Partial shell, low interaction |
| Editorial Workflow and Collaboration | No | Low |
| Billing and Usage | No | Low |
| Beta Operations | No | Low |
| Marketplace | Twelve mocked behavior cases | Strongest frontend area, still integration-limited |
| Settings and Webhooks | No | Low |
| Localization and Direction | No | Low |

## Shared Concern Coverage

| Concern | Evidence |
|---|---|
| Router and wildcard behavior | No dedicated test |
| `RequireAuth` | No dedicated test |
| Zustand persistence/actions | No dedicated test |
| API headers/errors/refresh | No dedicated test |
| `DynamicForm` conversion | No dedicated test |
| `useHealth` polling/cleanup | Mocked by Dashboard test, not tested directly |
| i18n detection/fallback/direction | No dedicated test |
| RTL/responsive styling | No runtime or visual test |
| Accessibility | No dedicated automated or manual evidence |
| Nginx history fallback | Configuration inspection only |

## Mock Boundary

Current page tests replace the API module and, for Marketplace behavior, manipulate store state. They validate selected UI logic without a running backend. This is useful component evidence but does not verify that manual TypeScript contracts match Rust responses, that headers are attached, or that authorization succeeds.

## Coverage Interpretation

Coverage percentages are `UNKNOWN` because no generated coverage artifact was used. File and test counts must not be converted into line/branch coverage claims. Passing jsdom tests would not by itself verify browser layout, drag sensors, clipboard, navigation, provider redirects, WebSockets, or Nginx behavior.

## Recommended Follow-Up Areas

These are risk-driven test priorities, not implementation started by Phase 4:

1. Session persistence, expiry failure, logout, and route admission.
2. Organization switching and stale tenant-state prevention.
3. API header/error/empty-body and contract behavior.
4. Dynamic form value conversion and validation.
5. Page Builder drag/drop, autosave, versions, workflow, template import, and preview handoff.
6. Organization, billing, beta, workflow, and webhook mutation permissions and errors.
7. i18n fallback, document direction, representative RTL layouts, accessibility, and real-browser routing.

## Validation Commands

The repository-defined frontend commands are `npm run lint`, `npm run typecheck`, `npm test`, and `npm run build` from `frontend/`. Exact Phase 4 execution results are reported in the completion summary and must not be inferred from CI configuration.

## Related Documents

- [Feature Catalog](feature-catalog.md)
- [Page Builder](page-builder.md)
- [Frontend Risks](frontend-risks.md)
- [Backend Testing Map](../backend/testing-map.md)

