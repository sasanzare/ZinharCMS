---
okf_document_id: "frontend-forms-validation"
title: "Frontend Forms and Validation"
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
  - "frontend/src/components/DynamicForm.tsx"
  - "frontend/src/pages/AuthPage.tsx"
  - "frontend/src/pages/ContentTypesPage.tsx"
  - "frontend/src/pages/EntriesPage.tsx"
  - "frontend/src/pages/PagesPage.tsx"
  - "frontend/src/pages/MarketplacePage.tsx"
related_documents:
  - "frontend/component-architecture.md"
  - "frontend/loading-errors-and-notifications.md"
  - "frontend/feature-catalog.md"
  - "frontend/frontend-risks.md"
related_diagrams:
  - "frontend/diagrams/frontend-state-flow.mmd"
uncertainty_markers:
  - "DOCUMENTATION_CODE_CONFLICT DCC-11"
  - "DEAD_OR_UNUSED_CODE_UNCONFIRMED DU-02"
  - "API_CONTRACT_UNCLEAR ACU-01"
---

# Frontend Forms and Validation

## Current Form Model

`VERIFIED`: Forms use controlled React state, event handlers, native HTML constraints, and page-specific pre-submit checks. No import of `react-hook-form`, `@hookform/resolvers`, or `zod` was found under `frontend/src` at the Phase 4 evidence snapshot.

This corrects current-state claims in Phase Zero technology and convention inventories that attributed page form state and schema validation to React Hook Form and Zod. The older evidence is retained as `DOCUMENTATION_CODE_CONFLICT DCC-11`; the installed dependencies are `DEAD_OR_UNUSED_CODE_UNCONFIRMED DU-02`, not declared dead code.

## Form Inventory

| Form area | State owner | Validation observed | Submission/error behavior |
|---|---|---|---|
| Login and registration | `AuthPage` local state | Native required, email input, password minimum length, mode-specific fields | API call; one inline error badge |
| Content-type editor | `ContentTypesPage` local state | Required inputs and page-specific field draft checks | API create/update; page error badge |
| Dynamic entry editor | `EntriesPage` plus `DynamicForm` | Field-schema `required`; native number/datetime types; simple value conversion | Entry API; page error badge |
| Media upload/metadata | `MediaPage` local state | Browser file selection and page checks | Multipart API; loading/error/message state |
| Page metadata and properties | `PagesPage` local draft | Native required for title/slug and required prop definitions; ad hoc JSON/array parsing | Save/autosave; builder error badge |
| Workflow comments | `WorkflowPage` local state | Trimmed body required by disabled submit | Comment API; page error |
| Organization controls | `OrganizationPage` local drafts | Required/disabled controls and page-specific guards | Many API actions; shared page error/message |
| Billing actions | No editable schema form | Role/lifecycle button gates | API action or browser navigation |
| Beta feedback/blockers/participants | `BetaPage` local drafts | Required fields and action gates | API action; shared page error/message |
| Marketplace workflows | `MarketplacePage` many local drafts | Length checks, JSON parsing, lifecycle/permission confirmations, selected file checks | API or multipart calls; shared page error/message |
| Settings and webhooks | `SettingsPage` local drafts | Required fields, selected events, page checks | Webhook API; shared page error/message |

## `DynamicForm`

`DynamicForm` renders fields from backend-provided `FieldSchema` and sends a new JSON object on every change.

- Boolean values use checkboxes.
- `longtext` and `richtext` use textareas.
- Number values use native number inputs and are converted with `Number`; blank becomes `null`.
- Datetime fields use `datetime-local`.
- Other types render as text.
- Object values are rendered with `JSON.stringify`, but general object editing is not parsed back into structured JSON by this component.
- `required` is the only field-schema validation property applied.

The component has no error prop, touched/dirty contract, cross-field validation, async validation, or backend field-error mapping.

## Page Builder Property Validation

`PropControl` maps component property definitions to boolean, select, textarea, number, email, or text inputs. Array/JSON values are edited as text and parsed by page-local logic. The builder normalizes page JSON before save and the backend performs authoritative validation. The UI does not surface a structured path-to-error map.

## Native and Ad Hoc Constraints

Current source uses selected `required`, `minLength`, `maxLength`, input types, disabled states, trimmed string checks, confirmations, prompts, and JSON parse guards. These constraints are distributed across pages. No reusable validation message model or single schema-to-form convention exists.

## Server Validation

Backend errors reach the client as a single `ApiError.message` in normal page flows. No verified mapping converts backend validation details to individual controls. A failed submission usually preserves current local draft state, but this is page behavior rather than a shared guarantee.

## Accessibility and UX Limits

Many controls are wrapped in labels, and native constraints provide browser behavior. However, no shared `aria-describedby`, field-error ID, focus-on-first-error, summary, dirty-navigation warning, or form accessibility test was found. Runtime behavior is not certified by source inspection alone.

## Change Guidance

- Do not describe React Hook Form or Zod as current runtime conventions until imports and usage exist.
- Keep backend validation authoritative even if client schemas are added.
- Define how structured backend errors map to fields before adding a second incompatible form pattern.
- Preserve unknown user data when changing dynamic field conversion.
- Add tests around JSON parsing, numeric blanks, required dynamic fields, and unsuccessful submissions when changing shared form behavior.

## Related Documents

- [Component Architecture](component-architecture.md)
- [Loading, Errors, and Notifications](loading-errors-and-notifications.md)
- [Frontend Risks](frontend-risks.md)
- [Frontend State Flow](diagrams/frontend-state-flow.mmd)

