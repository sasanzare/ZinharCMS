---
okf_document_id: "frontend-feature-settings-webhooks"
title: "Settings and Webhooks"
project: "ZinharCMS"
category: "frontend-feature"
phase: 4
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "7d25e4cbc53284a78033478e2681d8e9ebeb2fb1"
last_verified_date: "2026-07-17"
feature_id: "FE-FEAT-012"
feature_name: "Settings and Webhooks"
feature_paths:
  - "frontend/src/pages/SettingsPage.tsx"
  - "frontend/src/services/api.ts"
  - "frontend/src/types/api.ts"
boundary_status: "OVERLAPPING"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "frontend/src/pages/SettingsPage.tsx"
  - "frontend/src/services/api.ts"
  - "frontend/src/types/api.ts"
related_documents:
  - "frontend/feature-catalog.md"
  - "frontend/features/authentication-and-session.md"
  - "frontend/features/dashboard-and-application-shell.md"
  - "backend/modules/cms-webhooks.md"
related_diagrams:
  - "frontend/diagrams/frontend-api-flow.mmd"
uncertainty_markers:
  - "AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01"
  - "RESPONSIBILITY_OVERLAP FRO-02"
  - "API_CONTRACT_UNCLEAR ACU-01"
---

# Settings and Webhooks

## Feature Identity

| Field | Value |
|---|---|
| Feature ID | `FE-FEAT-012` |
| Application | `FE-APP-001` |
| Implementation | `IMPLEMENTED` |
| Boundary | `OVERLAPPING` |
| Confidence | High |
| Route | `/settings` |

## Responsibility

Displays current-user information, supports logout, shows health/readiness and selected client configuration values, and provides tenant CMS webhook list/create/update/delete/test workflows.

## Owned Source Areas

- Route page: `frontend/src/pages/SettingsPage.tsx`.
- Auth/system/webhook client methods and relevant manual contracts in central API/type files.

Account, health, configuration, and webhooks are adjacent settings surfaces rather than one backend domain.

## Entry Points

- Protected `/settings` route and sidebar link.
- Current-user refresh and logout.
- Health/readiness refresh.
- Webhook create/edit/toggle/delete/test actions.

## Internal Structure

The page defines supported webhook event values and a webhook draft, then loads current user, probes, and webhooks. It composes account, system/configuration, and webhook sections and calls session clear on logout.

## State

Local current user, health/readiness, webhooks, editing selection/draft, load/webhook flags, error, and message. Global session clear comes from Zustand.

## Backend Interactions

Uses current-user/logout, health/readiness, and webhook list/create/update/delete/test methods. It does not own webhook delivery execution or durable retry behavior.

## Access Control

The route is token-gated. No webhook-specific role visibility was found in the page. Backend tenant and webhook authorization remains authoritative (`ABV-01`).

## UI Composition

Account/read-only identity fields, system status cards, static configuration fields, webhook table/editor, event selections, status badges, and action buttons inside AppShell.

## Loading and Error Behavior

User and webhooks have local loading flows; errors and success messages are shared at page level. Webhook deletion uses confirmation and test results are shown through page feedback. No delivery polling or background notification exists.

## Tests

No dedicated Settings, current-user, probe, logout, or webhook frontend test was found.

## Known Risks and Unknowns

- Settings overlaps session, system health, and CMS webhook domains.
- UI role behavior for webhook management is not explicitly declared.
- Static displayed configuration is not proof of actual backend/deployment configuration.
- Manual webhook contracts carry `ACU-01`.

## Related Documents

- [Authentication and Session](authentication-and-session.md)
- [Dashboard and Application Shell](dashboard-and-application-shell.md)
- [API Client](../api-client.md)
- [Backend CMS Webhooks](../../backend/modules/cms-webhooks.md)

