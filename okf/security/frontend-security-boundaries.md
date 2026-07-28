---
okf_document_id: "security-frontend-boundaries"
title: "Frontend Security Boundaries"
project: "ZinharCMS"
category: "security-frontend"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "mixed"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
primary_sources:
  - "frontend/src/components/RequireAuth.tsx"
  - "frontend/src/components/SafeRichText.tsx"
  - "frontend/src/components/AppShell.tsx"
  - "frontend/src/pages/AuthPage.tsx"
  - "frontend/src/pages/MarketplacePage.tsx"
  - "frontend/src/security/richContent.ts"
  - "frontend/securityHeaders.ts"
  - "frontend/src/services/api.ts"
  - "frontend/src/stores/useAppStore.ts"
related_documents:
  - "authentication-architecture.md"
  - "session-and-token-lifecycle.md"
  - "../frontend/authentication-and-access.md"
related_diagrams:
  - "diagrams/trust-boundaries.mmd"
---

# Frontend Security Boundaries

## Client Responsibilities

The React client collects credentials, persists the local session projection, selects an organization, attaches bearer and tenant headers, includes cookies, renders API errors, and conditionally exposes controls based on stored roles and lifecycle state.

## Browser Authentication Storage

The access token exists only in volatile module/store memory. Legacy
access/refresh keys are deleted and ignored. User projection, organization
memberships, selected organization, and locale may use `localStorage`; any
same-origin script can read those non-secret projections and the currently live
in-memory access token. The refresh credential remains in an `HttpOnly` cookie
and is never returned to frontend JavaScript.

## Route Guard

`RequireAuth` checks only whether an access-token string exists. It does not validate signature, expiry, user activity, or server session state. Every child route is otherwise registered for every locally authenticated user.

## Role-Based UI Cues

Marketplace, beta, organization, billing, and other pages compute role flags to hide, disable, or skip calls. For example, Marketplace install and organization kill-switch controls require local organization owner/admin state; global review and global kill-switch controls use local global admin/super-admin state.

All such logic is `FRONTEND_ONLY_SECURITY_CHECK FOSC-01`. It is useful for UX but must be mirrored by backend middleware, RBAC, ownership, lifecycle, and RLS checks.

## Request Construction

Only requests marked `auth: true` receive the access token and selected organization header. The helper sends cookies for all API requests through `credentials: include`. There is no automatic refresh/retry, global 401 session clear, CSRF token, or client-side token-expiry scheduler.

## Frontend Validation

HTML input constraints and local enable/disable conditions provide early feedback. They are bypassable and are not security controls. Backend validation remains authoritative.

## Rich-Content Rendering

React text interpolation remains the default. Declared Page Builder rich text is
sanitized by DOMPurify and converted to a branded `SanitizedRichHtml` value.
`SafeRichText` is the only approved HTML-capable sink and its prop type rejects
arbitrary strings. An AST policy check rejects runtime `innerHTML`,
`outerHTML`, `insertAdjacentHTML`, `document.write`, `eval`, `Function`,
`srcDoc`, and `dangerouslySetInnerHTML` outside that component.

Flat Page Builder schemas and legacy JSON Schema `properties` are normalized.
Preview WebSocket messages are shape-checked and pass through the same renderer.
Marketplace support, screenshot, checkout, and billing destinations use the
central URL policy rather than feature-local direct navigation.

## CSP and Trusted Types

The production frontend policy uses exact API and WebSocket origins, no
`unsafe-eval`, no inline scripts, no wildcard script source, and no frame/object
surface. Production requires Trusted Types and permits exactly
`zinhar-rich-content` and `dompurify`. Development permits inline styles only
for Vite style injection and does not enforce Trusted Types.

## Risk Notes

- XSS could expose the access token and role/session projection.
- Stale local roles can expose controls that the backend must reject.
- Downstream headless clients can create an unsafe sink if they ignore the API
  rich-content contract.
- Remote HTTPS Marketplace images can disclose client IP and request timing.
- `SESSION_LIFECYCLE_UNCLEAR SLU-01` covers expiry, cross-tab logout, and invalid-session behavior.
