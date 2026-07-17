---
okf_document_id: "frontend-feature-billing-usage"
title: "Billing and Usage"
project: "ZinharCMS"
category: "frontend-feature"
phase: 4
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "7d25e4cbc53284a78033478e2681d8e9ebeb2fb1"
last_verified_date: "2026-07-17"
feature_id: "FE-FEAT-009"
feature_name: "Billing and Usage"
feature_paths:
  - "frontend/src/pages/BillingPage.tsx"
  - "frontend/src/services/api.ts"
  - "frontend/src/types/api.ts"
boundary_status: "OBSERVED"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "frontend/src/pages/BillingPage.tsx"
  - "frontend/src/services/api.ts"
  - "frontend/src/types/api.ts"
related_documents:
  - "frontend/feature-catalog.md"
  - "frontend/authentication-and-access.md"
  - "frontend/features/organizations-and-workspaces.md"
  - "backend/modules/billing-quotas.md"
related_diagrams:
  - "frontend/diagrams/frontend-api-flow.mmd"
uncertainty_markers:
  - "AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01"
  - "API_CONTRACT_UNCLEAR ACU-01"
  - "IMPLEMENTATION_STATUS_UNCLEAR ISU-03"
---

# Billing and Usage

## Feature Identity

| Field | Value |
|---|---|
| Feature ID | `FE-FEAT-009` |
| Application | `FE-APP-001` |
| Implementation | `IMPLEMENTED` |
| Boundary | `OBSERVED` |
| Confidence | High |
| Route | `/billing` |

## Responsibility

Displays subscription and usage, lists plans, supports plan changes, starts hosted checkout, opens the billing portal, and requests usage rebuild.

## Owned Source Areas

- Route page: `frontend/src/pages/BillingPage.tsx`.
- Billing API methods and manual plan/usage contracts in the central service/type files.

## Entry Points

- Protected `/billing` route and sidebar link.
- Plan cards, checkout/change-plan actions, portal action, usage rebuild, and refresh.
- Dashboard consumes billing usage summaries separately.

## Internal Structure

The page formats money, bytes, metric limits, and plan state; loads plans and current usage in parallel; and dispatches billing mutations or browser navigation based on the selected action.

## State

Local plan list, usage response, load/action flags, error, and success message. The active membership comes from Zustand and determines enabled management actions.

## Backend Interactions

Uses plans, usage, change-plan, checkout, portal, and rebuild methods. Checkout and portal responses provide external URLs assigned to browser location. The frontend does not handle provider credentials or callbacks.

## Access Control

Owner, admin, and billing-manager memberships can enable billing management buttons. Reading the page is not role-filtered by the router. Backend billing authorization remains authoritative (`ABV-01`).

## UI Composition

Subscription summary, metric cards, usage/limit display, plan cards, role-sensitive buttons, status stack, and provider-navigation icons inside AppShell.

## Loading and Error Behavior

Plans and usage share a load. Mutations share an action flag. Checkout/portal set browser location on success; their action state remains relevant only on failure. Errors and success messages render inline. No return-from-provider route state is handled here.

## Tests

No dedicated Billing page test was found. Role gating, usage formatting, unlimited limits, provider navigation, plan lifecycle, and failure behavior are uncovered by the current frontend suite.

## Known Risks and Unknowns

- UI role gating can drift from backend enforcement.
- Actual provider/deployment readiness remains `ISU-03`.
- Manual plan/usage types carry `ACU-01`.
- Browser navigation depends on safe URLs returned by the backend; detailed security is deferred.

## Related Documents

- [Authentication and Access](../authentication-and-access.md)
- [Organizations and Workspaces](organizations-and-workspaces.md)
- [Backend Billing and Quotas](../../backend/modules/billing-quotas.md)
- [Frontend API Flow](../diagrams/frontend-api-flow.mmd)

