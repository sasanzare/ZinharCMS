---
okf_document_id: "frontend-feature-localization-direction"
title: "Localization and Direction"
project: "ZinharCMS"
category: "frontend-feature"
phase: 4
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "7d25e4cbc53284a78033478e2681d8e9ebeb2fb1"
last_verified_date: "2026-07-17"
feature_id: "FE-FEAT-013"
feature_name: "Localization and Direction"
feature_paths:
  - "frontend/src/i18n"
  - "frontend/src/main.tsx"
  - "frontend/src/styles/index.css"
  - "frontend/public/fonts"
boundary_status: "EXPLICIT"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "frontend/src/i18n/I18nProvider.tsx"
  - "frontend/src/i18n/locales.ts"
  - "frontend/src/i18n/messages.ts"
  - "frontend/src/i18n/LanguageSelect.tsx"
  - "frontend/src/styles/index.css"
related_documents:
  - "frontend/feature-catalog.md"
  - "frontend/styling-and-design-system.md"
  - "frontend/component-architecture.md"
  - "frontend/testing-map.md"
related_diagrams:
  - "frontend/diagrams/frontend-application-map.mmd"
  - "frontend/diagrams/frontend-state-flow.mmd"
uncertainty_markers:
  - "DOCUMENTATION_CODE_CONFLICT DCC-02"
  - "UI_BEHAVIOR_UNVERIFIED UBU-01"
---

# Localization and Direction

## Feature Identity

| Field | Value |
|---|---|
| Feature ID | `FE-FEAT-013` |
| Application | `FE-APP-001` |
| Implementation | `IMPLEMENTED` |
| Boundary | `EXPLICIT` |
| Confidence | High |
| Entry | Root provider; language selectors in auth and shell |

## Responsibility

Detects, normalizes, persists, and changes locale; supplies typed message lookup and interpolation; falls back to English; and synchronizes document language and direction for LTR/RTL presentation.

## Owned Source Areas

- `frontend/src/i18n/context.ts`
- `frontend/src/i18n/I18nProvider.tsx`
- `frontend/src/i18n/index.ts`
- `frontend/src/i18n/labels.ts`
- `frontend/src/i18n/LanguageSelect.tsx`
- `frontend/src/i18n/locales.ts`
- `frontend/src/i18n/messages.ts`
- `frontend/src/i18n/useI18n.ts`
- Direction-sensitive rules and bundled font in styling/assets.

## Entry Points

- `I18nProvider` wrapping `RouterProvider` in `main.tsx`.
- `LanguageSelect` on the public authentication page and protected shell.
- `useI18n` calls throughout components and pages.

## Internal Structure

Locale definitions establish English and Persian/RTL. Initial locale comes from storage, browser language lists, browser language, or default. The provider memoizes locale/direction/translation operations and writes `lang`, `dir`, and storage after changes. Message lookup falls back to English and then the key.

## State

Locale lives in React context and persists in `localStorage`. Direction is derived. It is intentionally separate from Zustand session/organization state and available before authentication.

## Backend Interactions

None. UI messages and selection are frontend-owned. User-generated content and backend enum values are not translated by the provider automatically.

## Access Control

Locale selection is available publicly and after authentication. No role or organization restriction applies.

## UI Composition

`LanguageSelect` uses a label, Lucide language icon, and native select. The provider affects all pages. Global CSS swaps the root font in RTL and contains direction-aware table, builder, review, and Marketplace rules. The bundled font asset supports RTL presentation.

## Loading and Error Behavior

Initialization is synchronous. Invalid stored locales are ignored through normalization. Missing messages fall back to English or the key. No runtime missing-key report, lazy dictionary load, or localization monitoring exists.

## Tests

No dedicated test was found for locale detection, persistence, fallback, interpolation, document attributes, selectors, RTL CSS, or message completeness. Some page tests render English default text.

## Known Risks and Unknowns

- `docs/I18N.md` understates current translated-page coverage (`DCC-02`), while hard-coded strings still exist.
- Dictionary completeness is type-oriented but no runtime/reporting test was found.
- RTL and responsive behavior remains `UBU-01` without browser evidence.
- Locale-specific date/number formatting is mostly page/native behavior rather than one i18n formatter boundary.

## Related Documents

- [Styling and Design System](../styling-and-design-system.md)
- [Component Architecture](../component-architecture.md)
- [Testing Map](../testing-map.md)
- [Frontend State Flow](../diagrams/frontend-state-flow.mmd)

