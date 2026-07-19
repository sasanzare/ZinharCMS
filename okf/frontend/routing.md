---
okf_document_id: "frontend-routing"
title: "Frontend Routing"
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
  - "frontend/src/main.tsx"
  - "frontend/src/router.tsx"
  - "frontend/src/components/RequireAuth.tsx"
  - "frontend/src/components/AppShell.tsx"
  - "frontend/src/pages/WorkspaceRedirectPage.tsx"
  - "frontend/nginx.conf"
related_documents:
  - "frontend/overview.md"
  - "frontend/pages-and-layouts.md"
  - "frontend/authentication-and-access.md"
  - "frontend/feature-catalog.md"
  - "architecture/runtime-flows.md"
related_diagrams:
  - "frontend/diagrams/frontend-routing-flow.mmd"
uncertainty_markers:
  - "ROUTING_BEHAVIOR_UNCLEAR RBU-01"
  - "AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01"
  - "UI_BEHAVIOR_UNVERIFIED UBU-01"
---

# Frontend Routing

## Router Model

`VERIFIED`: `router.tsx` constructs one React Router browser router. Route elements are imported eagerly. The route tree has one public login page and one protected parent whose element is `RequireAuth`. When a token exists, `RequireAuth` renders `AppShell`; the shell renders the active child through `Outlet`.

No route loader, action, `errorElement`, lazy route module, `React.lazy`, `Suspense`, or per-route metadata/permission declaration was found.

## Route Inventory

| Pattern | Route name | Feature | Page | Layout | Auth gate | Visible permission cue | Loading mode | Status | Evidence |
|---|---|---|---|---|---|---|---|---|---|
| `/login` | Authentication | FE-FEAT-001 | `AuthPage` | Standalone auth layout inside page | None | Public login/register forms | Eager | `VERIFIED` | `router.tsx`; `AuthPage.tsx` |
| `/` | Dashboard | FE-FEAT-002 | `DashboardPage` | `AppShell` | Token presence | None | Eager | `VERIFIED` | `router.tsx`; `DashboardPage.tsx` |
| `/content-types` | Content Types | FE-FEAT-004 | `ContentTypesPage` | `AppShell` | Token presence | None at route/menu level | Eager | `VERIFIED` | `router.tsx`; `ContentTypesPage.tsx` |
| `/entries` | Entries | FE-FEAT-005 | `EntriesPage` | `AppShell` | Token presence | None at route/menu level | Eager | `VERIFIED` | `router.tsx`; `EntriesPage.tsx` |
| `/media` | Media | FE-FEAT-006 | `MediaPage` | `AppShell` | Token presence | None at route/menu level | Eager | `VERIFIED` | `router.tsx`; `MediaPage.tsx` |
| `/marketplace` | Marketplace | FE-FEAT-011 | `MarketplacePage` | `AppShell` | Token presence | In-page role and lifecycle gates | Eager | `VERIFIED` | `router.tsx`; `MarketplacePage.tsx` |
| `/pages` | Pages | FE-FEAT-007 | `PagesPage` | `AppShell` | Token presence | None at route/menu level | Eager | `VERIFIED` | `router.tsx`; `PagesPage.tsx` |
| `/workflow` | Workflow | FE-FEAT-008 | `WorkflowPage` | `AppShell` | Token presence | None at route/menu level | Eager | `VERIFIED` | `router.tsx`; `WorkflowPage.tsx` |
| `/organization` | Organization | FE-FEAT-003 | `OrganizationPage` | `AppShell` | Token presence | In-page membership role gates | Eager | `VERIFIED` | `router.tsx`; `OrganizationPage.tsx` |
| `/workspace/:slug` | Workspace Redirect | FE-FEAT-003 | `WorkspaceRedirectPage` | `AppShell` | Token presence | Membership list determines resolution | Eager | `VERIFIED` | `router.tsx`; `WorkspaceRedirectPage.tsx` |
| `/billing` | Billing | FE-FEAT-009 | `BillingPage` | `AppShell` | Token presence | owner/admin/billing-manager actions | Eager | `VERIFIED` | `router.tsx`; `BillingPage.tsx` |
| `/beta` | Beta | FE-FEAT-010 | `BetaPage` | `AppShell` | Token presence | membership and global-role sections | Eager | `VERIFIED` | `router.tsx`; `BetaPage.tsx` |
| `/settings` | Settings | FE-FEAT-012 | `SettingsPage` | `AppShell` | Token presence | None at route/menu level | Eager | `VERIFIED` | `router.tsx`; `SettingsPage.tsx` |
| `*` | Protected fallback | FE-FEAT-002 | `Navigate` | Protected parent | Token presence | None | Eager redirect to `/` | `VERIFIED` | `router.tsx` |

## Navigation Model

`AppShell` owns a static `navItems` array. Every authenticated user sees links for dashboard, content types, entries, media, Marketplace, pages, workflow, organization, billing, beta, and settings. The active link comes from `NavLink`; the page title comes from an exact pathname-to-label map.

The title map does not have a dynamic `/workspace/:slug` key, so the dashboard title is the fallback while that redirect page is active. This is current behavior, not an intended title policy.

## Authentication Admission

`RequireAuth` reads only the Zustand `accessToken`. A truthy token renders the shell; otherwise the router redirects to `/login` with replacement. It does not validate expiration, load the current user, refresh tokens, or declare route-specific roles. A backend `401` is displayed by individual page behavior rather than handled by a global routing interceptor.

This makes the guard a client experience mechanism, not a security control. See [Authentication and Access](authentication-and-access.md).

## Workspace Redirect

`WorkspaceRedirectPage` reads the `slug` path parameter, consults the store organization list, can load organizations when needed, activates the matched organization, and navigates to the dashboard. Its purpose is context selection; it does not provide a nested workspace route tree. Most normal feature URLs do not contain the organization slug, and the selected organization is carried in store/header state.

## Browser and Hosting Behavior

The production-like Nginx configuration falls back to `index.html` for non-asset paths, which supports browser-history routes on that image. `ROUTING_BEHAVIOR_UNCLEAR RBU-01`: repository configuration does not prove that every actual ingress or static host uses equivalent fallback behavior.

## Loading, Errors, and Not Found

- Route modules load with the initial bundle; there is no route-level loading UI.
- Pages own their own data loading and errors after render.
- There is no route error boundary or dedicated not-found page.
- An unknown protected child silently redirects to the dashboard.
- A direct unknown URL without a stored token first passes through the protected parent and redirects to login.

## Routing Risks

- Static menu visibility can expose inaccessible destinations even when backend actions are correctly denied.
- Eager route imports include large route modules in one bundle unless Vite performs internal optimization; no explicit feature split exists.
- No central expired-session redirect behavior was found.
- Wildcard redirect loses the requested path and provides no diagnostic or not-found explanation.
- Dynamic workspace context is not encoded in most URLs, so copied feature URLs depend on browser-stored organization state.

## Related Documents

- [Pages and Layouts](pages-and-layouts.md)
- [Authentication and Access](authentication-and-access.md)
- [Frontend Routing Flow](diagrams/frontend-routing-flow.mmd)
- [Architecture Runtime Flows](../architecture/runtime-flows.md)
- [Frontend Security Boundaries](../security/frontend-security-boundaries.md)

## Security Boundary

All application-shell child routes share the same local token-presence guard; per-route role admission is not implemented in the router. Hidden or disabled page actions are UX controls, not authorization. See `FRONTEND_ONLY_SECURITY_CHECK FOSC-01` and the backend [Authorization Architecture](../security/authorization-architecture.md).
