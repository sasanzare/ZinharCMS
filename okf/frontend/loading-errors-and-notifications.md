---
okf_document_id: "frontend-loading-errors-notifications"
title: "Frontend Loading, Errors, and Notifications"
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
  - "frontend/src/components/StatusBadge.tsx"
  - "frontend/src/hooks/useHealth.ts"
  - "frontend/src/services/api.ts"
  - "frontend/src/pages"
  - "frontend/src/router.tsx"
related_documents:
  - "frontend/state-management.md"
  - "frontend/api-client.md"
  - "frontend/forms-and-validation.md"
  - "frontend/testing-map.md"
  - "frontend/frontend-risks.md"
related_diagrams:
  - "frontend/diagrams/frontend-api-flow.mmd"
uncertainty_markers:
  - "UI_BEHAVIOR_UNVERIFIED UBU-01"
  - "AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01"
  - "API_CONTRACT_UNCLEAR ACU-01"
---

# Frontend Loading, Errors, and Notifications

## Async Presentation Model

Each route page generally owns boolean loading/action flags, a string error, an optional success message, and domain data. `StatusBadge` presents many of these values. Marketplace and other large pages sometimes maintain separate loading flags for independent sections.

No global loading bar, suspense fallback, route loader, error boundary, toast queue, notification center, offline banner, or monitoring SDK was found.

## Loading Patterns

| Pattern | Examples | Behavior |
|---|---|---|
| Initial page load flag | Content Types, Entries, Media, Pages | Render page chrome with loading text/badge and later data or empty state |
| Refresh flag | Dashboard hooks, Billing, Beta, Organization | Disable refresh or show neutral badge during request |
| Action flag | Billing, Beta, Marketplace, Organization | Disable one or many mutation controls while an action runs |
| Section-specific flag | Marketplace catalog/installations/actions | Independent areas can display their own progress |
| Polling | `useHealth` in Shell and Dashboard | Repeat health/readiness every 15 seconds by default |
| Autosave status | Pages | Display pending/saved/failed state around delayed save |

There is no common loading-state type or shared guarantee for retaining previous data during refresh.

## Error Flow

1. The API client parses a non-success response.
2. It throws `ApiError(status, message)`.
3. Page code catches the error and usually chooses `ApiError.message` or a translated fallback.
4. A danger-tone `StatusBadge` renders the string.

Selected local parsing failures, clipboard operations, and browser prompts follow page-specific behavior. Errors are not normalized by feature, classified as retryable, attached to fields, or reported centrally.

## Success and Notification Flow

Success feedback normally uses a page-local message rendered as a success-tone status badge. Some flows update data without a persistent message. Browser-native `confirm` and `prompt` are used for selected destructive, Marketplace, and template-import actions. Clipboard success is sometimes represented by a page-local message.

`StatusBadge` is inline status presentation. It is not a toast, live-region manager, durable notification, cross-route event, or background job tracker.

## Empty States

Pages use table rows, `empty-copy`, `empty-state`, or builder placeholders when collections are empty. Empty and failed states are separate only where page logic explicitly distinguishes them. No shared empty-state component or recovery-action contract exists.

## Unhandled Boundaries

- Render exceptions have no application Error Boundary.
- Router errors have no `errorElement`.
- Unhandled promise failures outside local `try/catch` have no UI channel.
- A global `401` does not automatically clear the session.
- Request cancellation and timeout behavior are not client-controlled.
- No offline/reconnect state was found.
- No client telemetry or source-map reporting path was found.

## Accessibility Evidence

Inline badges expose visible text, but no shared `role="alert"`, `aria-live` strategy, focus transfer, or notification announcement policy was verified. Browser-native dialogs have platform behavior but are not styled or feature-testable in the same way as application components. Runtime accessibility remains `UI_BEHAVIOR_UNVERIFIED UBU-01`.

## Change Guidance

- Preserve the distinction between initial load, refresh, action, empty, and failure states.
- If centralizing errors, retain HTTP status and safe user message without exposing internal details.
- Define session-expiry behavior together with API and router ownership.
- Add accessibility behavior and tests before calling a new toast or alert system complete.
- Do not claim retries, offline support, or monitoring unless implementation and execution evidence exist.

## Related Documents

- [State Management](state-management.md)
- [API Client](api-client.md)
- [Forms and Validation](forms-and-validation.md)
- [Testing Map](testing-map.md)
- [Frontend API Flow](diagrams/frontend-api-flow.mmd)

