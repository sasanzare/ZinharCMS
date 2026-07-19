---
okf_document_id: "frontend-risks"
title: "Frontend Risks"
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
  - "frontend/src"
  - "frontend/package.json"
  - "frontend/vite.config.ts"
  - "frontend/vitest.config.ts"
  - ".github/workflows/frontend-ci.yml"
related_documents:
  - "frontend/overview.md"
  - "frontend/feature-boundaries.md"
  - "frontend/routing.md"
  - "frontend/state-management.md"
  - "frontend/api-client.md"
  - "frontend/authentication-and-access.md"
  - "frontend/page-builder.md"
  - "frontend/testing-map.md"
  - "architecture/architecture-risks.md"
related_diagrams:
  - "frontend/diagrams/frontend-state-flow.mmd"
  - "frontend/diagrams/frontend-api-flow.mmd"
  - "frontend/diagrams/page-builder-flow.mmd"
uncertainty_markers:
  - "FEATURE_BOUNDARY_UNCLEAR FBU-01"
  - "STATE_OWNERSHIP_UNCLEAR SOU-01"
  - "API_CONTRACT_UNCLEAR ACU-01"
  - "AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01"
  - "DUPLICATED_CONTRACT DC-01"
  - "DEAD_OR_UNUSED_CODE_UNCONFIRMED DU-02"
  - "UI_BEHAVIOR_UNVERIFIED UBU-01"
---

# Frontend Risks

## Scoring

Likelihood and impact use Low, Medium, or High. Severity is the combined documentation priority, not a vulnerability rating. Mitigations are recommended follow-up, not claims of completed work.

## Risk Register

### FE-RISK-001: Route-Page Responsibility Concentration

| Field | Value |
|---|---|
| Description | Marketplace, Organization, and Pages route files combine broad UI composition, domain state, API orchestration, permissions, formatting, and failure handling. |
| Evidence | `MarketplacePage.tsx`; `OrganizationPage.tsx`; `PagesPage.tsx`; feature boundary matrix |
| Affected features | FE-FEAT-003, FE-FEAT-007, FE-FEAT-011 |
| Likelihood | High |
| Impact | High |
| Severity | High |
| Mitigation | Establish feature-owned internal components/hooks with behavior tests while preserving current contracts. |
| Follow-up phase | Phase 10 engineering/testing detail; owner architecture decision as needed |
| Owner confirmation | Needed for intended decomposition policy under NOC-12 |
| Status | `FEATURE_BOUNDARY_UNCLEAR FBU-01`; open |

### FE-RISK-002: Development Credentials in Authentication UI Defaults

| Field | Value |
|---|---|
| Description | Authentication form source initializes visible development credential values. A production-like build can preserve those defaults. Values are intentionally not reproduced here. |
| Evidence | `frontend/src/pages/AuthPage.tsx` |
| Affected features | FE-FEAT-001 |
| Likelihood | High if the same source is built |
| Impact | Medium to High depending on backend bootstrap and environment |
| Severity | High |
| Mitigation | Define environment-safe demo behavior and remove production credential prefill; assess jointly with backend bootstrap. |
| Follow-up phase | Phase 7 security |
| Owner confirmation | Required for demo/development policy |
| Status | Open; current behavior `VERIFIED` |

### FE-RISK-003: Browser-Persisted Session and Preview Query Context

| Field | Value |
|---|---|
| Description | Tokens persist in `localStorage`; Page Builder copies a preview WebSocket URL containing access and organization context in its query. |
| Evidence | `useAppStore.ts`; `api.ts`; `PagesPage.tsx` |
| Affected features | FE-FEAT-001, FE-FEAT-007 |
| Likelihood | High |
| Impact | High if browser or copied URL context is exposed |
| Severity | High |
| Mitigation | Define token storage and preview authorization policy; consider short-lived purpose-bound preview credentials. |
| Follow-up phase | Phase 7 security |
| Owner confirmation | Required for accepted session/preview threat model |
| Status | `STATE_OWNERSHIP_UNCLEAR SOU-01`; open |

### FE-RISK-004: Manual Frontend and Backend Contract Duplication

| Field | Value |
|---|---|
| Description | TypeScript response/request types and handwritten API methods can drift from Rust routes and serializers without compile-time or runtime detection. |
| Evidence | `frontend/src/types/api.ts`; `frontend/src/services/api.ts`; backend route/models |
| Affected features | All API-backed features |
| Likelihood | High |
| Impact | High |
| Severity | High |
| Mitigation | Establish generated/shared contracts or representative contract tests and runtime validation at selected boundaries. |
| Follow-up phase | Phase 6 API |
| Owner confirmation | Needed for contract authority and compatibility policy |
| Status | `API_CONTRACT_UNCLEAR ACU-01`; `DUPLICATED_CONTRACT DC-01`; open |

### FE-RISK-005: No Global Session-Expiry or Retry Policy

| Field | Value |
|---|---|
| Description | A refresh method exists, but no automatic refresh/retry, scheduled renewal, global `401` clearing, or access-denied route behavior was found. |
| Evidence | `api.ts`; `RequireAuth.tsx`; page error handlers |
| Affected features | All protected features |
| Likelihood | High during expired sessions |
| Impact | Medium to High |
| Severity | High |
| Mitigation | Define one safe session-expiry state machine with replay limits and router ownership. |
| Follow-up phase | Phase 7 security and Phase 10 testing |
| Owner confirmation | Required for session recovery policy under NOC-12 |
| Status | Open |

### FE-RISK-006: Browser Role Cues Can Drift from Backend Enforcement

| Field | Value |
|---|---|
| Description | Static route/menu access and page-specific role checks are distributed and do not prove backend authorization equivalence. |
| Evidence | `AppShell.tsx`; Billing, Beta, Marketplace, and Organization pages |
| Affected features | All protected features, especially FE-FEAT-003, FE-FEAT-009, FE-FEAT-010, FE-FEAT-011 |
| Likelihood | Medium |
| Impact | High for misleading or unsafe assumptions |
| Severity | High |
| Mitigation | Build a route/action permission matrix against backend enforcement and keep browser checks informational. |
| Follow-up phase | Phase 7 security |
| Owner confirmation | Needed for intended UI visibility policy |
| Status | `AUTHORIZATION_BEHAVIOR_UNVERIFIED ABV-01`; open |

### FE-RISK-007: Fragmented Form and Validation Behavior

| Field | Value |
|---|---|
| Description | Controlled forms, native constraints, JSON parsing, and server errors are implemented independently; declared form/schema libraries are not used. |
| Evidence | Page forms; `DynamicForm.tsx`; `package.json`; no matching source imports |
| Affected features | Most mutation features |
| Likelihood | High |
| Impact | Medium |
| Severity | Medium |
| Mitigation | Define a shared error/validation contract before adopting or removing libraries; test dynamic conversions. |
| Follow-up phase | Phase 10 development/testing |
| Owner confirmation | Needed for frontend form convention under NOC-12 |
| Status | `DOCUMENTATION_CODE_CONFLICT DCC-11`; `DEAD_OR_UNUSED_CODE_UNCONFIRMED DU-02`; open |

### FE-RISK-008: Fragmented Loading and Error Boundaries

| Field | Value |
|---|---|
| Description | Pages repeat loading/error/message logic; no route Error Boundary, global session failure, toast system, cancellation, or monitoring boundary exists. |
| Evidence | `router.tsx`; `api.ts`; page-local states; absence of Error Boundary/monitoring imports |
| Affected features | All features |
| Likelihood | High |
| Impact | Medium |
| Severity | Medium |
| Mitigation | Define shared failure taxonomy and recovery UI while retaining feature context and safe messages. |
| Follow-up phase | Phase 10 development/operations/testing |
| Owner confirmation | Needed for UX and observability policy |
| Status | Open |

### FE-RISK-009: Page-Local Server State Has No Cache or Cancellation Contract

| Field | Value |
|---|---|
| Description | Independent effects and loaders can duplicate work, race during transitions, and become stale without tenant-keyed cache/invalidation rules. |
| Evidence | Pages and `useHealth.ts`; no query-cache or AbortController usage |
| Affected features | All API-backed features |
| Likelihood | Medium |
| Impact | Medium |
| Severity | Medium |
| Mitigation | Define request identity, cancellation, tenant keying, invalidation, and stale-data policy before centralizing server state. |
| Follow-up phase | Phase 10 development/testing |
| Owner confirmation | Needed for state evolution policy under NOC-12 |
| Status | `STATE_OWNERSHIP_UNCLEAR SOU-02`; open |

### FE-RISK-010: Eager Route Bundle and No Budget

| Field | Value |
|---|---|
| Description | All route pages, including large Marketplace and Organization modules, are imported eagerly; no explicit split or bundle-size gate was found. |
| Evidence | `router.tsx`; Vite config; frontend scripts/CI |
| Affected features | FE-APP-001; all routes |
| Likelihood | High |
| Impact | Medium, dependent on measured clients and bundle output |
| Severity | Medium |
| Mitigation | Measure output and runtime navigation before introducing route splits; add an agreed budget if justified. |
| Follow-up phase | Phase 10 development/performance |
| Owner confirmation | Needed for target device/performance expectations |
| Status | Open; measured impact `UNKNOWN` |

### FE-RISK-011: Informal Global Styling Boundary

| Field | Value |
|---|---|
| Description | One large global stylesheet, literal visual values, and no component/token governance create broad regression scope. |
| Evidence | `styles/index.css`; shared semantic classes; no design-system package or Storybook |
| Affected features | All UI features |
| Likelihood | Medium |
| Impact | Medium |
| Severity | Medium |
| Mitigation | Define style ownership and tokens incrementally with representative visual and RTL tests. |
| Follow-up phase | Phase 10 development/testing |
| Owner confirmation | Required for component/design policy under NOC-12 |
| Status | `COMPONENT_OWNERSHIP_UNCLEAR COU-01`; open |

### FE-RISK-012: Sparse Frontend Test Coverage

| Field | Value |
|---|---|
| Description | Only Dashboard, Pages, and Marketplace have tests; critical shared state, routing, API, forms, i18n, and most features lack dedicated coverage. |
| Evidence | Three `.test.tsx` files; Vitest configuration; no coverage report |
| Affected features | All features |
| Likelihood | High |
| Impact | High |
| Severity | High |
| Mitigation | Prioritize session, tenant switching, contract, builder persistence, and role-sensitive mutation tests; add real-browser coverage for critical flows. |
| Follow-up phase | Phase 10 testing |
| Owner confirmation | Needed for required quality gates |
| Status | Open; coverage percentage `UNKNOWN` |

### FE-RISK-013: Runtime Accessibility, RTL, and Responsive Quality Unverified

| Field | Value |
|---|---|
| Description | Semantic and responsive source evidence exists, but no browser, keyboard, assistive-technology, screenshot, or accessibility test evidence was executed. |
| Evidence | JSX semantics; global media/RTL CSS; absent dedicated test configuration |
| Affected features | All UI features, especially Pages and Marketplace |
| Likelihood | Medium |
| Impact | Medium to High depending on user requirements |
| Severity | Medium |
| Mitigation | Define supported browsers/devices and accessibility target; run representative LTR/RTL and keyboard checks. |
| Follow-up phase | Phase 10 testing and operations |
| Owner confirmation | Required under NOC-07 and NOC-12 |
| Status | `UI_BEHAVIOR_UNVERIFIED UBU-01`; open |

## Priority Summary

| Priority | Risks |
|---|---|
| High | FE-RISK-001 through FE-RISK-006; FE-RISK-012 |
| Medium | FE-RISK-007 through FE-RISK-011; FE-RISK-013 |

## Related Documents

- [Feature Boundaries](feature-boundaries.md)
- [Authentication and Access](authentication-and-access.md)
- [API Client](api-client.md)
- [Page Builder](page-builder.md)
- [Testing Map](testing-map.md)
- [Architecture Risks](../architecture/architecture-risks.md)
- [Security Risks](../security/security-risks.md)

## Phase 7 Security Correlation

Script-readable access tokens, stale local roles, query-string preview credentials, frontend-only route/action checks, and missing automatic session-expiry handling are tracked in [Frontend Security Boundaries](../security/frontend-security-boundaries.md) and security risks `SEC-02`, `SEC-04`, and `SEC-11`.
