---
okf_document_id: "frontend-styling-design-system"
title: "Frontend Styling and Design System"
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
  - "frontend/src/styles/index.css"
  - "frontend/src/main.tsx"
  - "frontend/vite.config.ts"
  - "frontend/package.json"
  - "frontend/public/fonts"
  - "frontend/src/components"
  - "frontend/src/pages"
related_documents:
  - "frontend/component-architecture.md"
  - "frontend/pages-and-layouts.md"
  - "frontend/features/localization-and-direction.md"
  - "frontend/frontend-risks.md"
related_diagrams:
  - "frontend/diagrams/frontend-application-map.mmd"
uncertainty_markers:
  - "COMPONENT_OWNERSHIP_UNCLEAR COU-01"
  - "INFERRED_FROM_CODE"
  - "UI_BEHAVIOR_UNVERIFIED UBU-01"
---

# Frontend Styling and Design System

## Styling Stack

`VERIFIED`: `main.tsx` imports one global stylesheet. `index.css` imports Tailwind CSS, while Vite registers the Tailwind plugin. Application JSX predominantly uses semantic class names whose rules live in the global stylesheet. No separate Tailwind configuration file, CSS Module, CSS-in-JS runtime, Sass pipeline, or styled-component system was found.

## Current Visual Vocabulary

| Area | Examples | Ownership |
|---|---|---|
| Foundation | root colors, typography, box sizing, form font inheritance | Global stylesheet |
| Direction | root RTL font, table/palette/review alignment, direction-specific Marketplace rules | Global stylesheet plus i18n provider |
| Application chrome | app shell, sidebar, navigation, top bar, content area | `AppShell` markup plus global stylesheet |
| Controls | primary/secondary/icon/danger buttons, inputs, selects, checkboxes, file picker | Global stylesheet |
| Feedback | status badges, status stacks, empty copy/states, builder error | Shared component plus global stylesheet |
| Content layout | panels, headers, actions, grids, metrics, tables, toolbars | Global stylesheet and page markup |
| Feature layouts | auth, dashboard, Marketplace, builder, organization, billing, beta, workflow | Global stylesheet sections |
| Responsive behavior | multiple breakpoints from 760 to 1320 pixels | Global stylesheet |

## Tokens and Theming

Colors, radii, spacing, shadows, typography, and breakpoints are repeated as CSS literal values. No named design-token file, CSS custom-property token system, theme interface, or component variant registry was found. There is no verified dark mode or `prefers-color-scheme` behavior.

Tailwind's presence provides utility-generation tooling, but Phase 4 did not find application source using a governed utility-class convention. Current architecture should therefore be described as global semantic CSS with Tailwind available, not as a utility-first design system.

## Typography and Direction

The default stack starts with Inter/system fonts. RTL mode uses the bundled `VazirmatnZinhar` font declared with `@font-face`. The i18n provider sets document `dir` and `lang`; CSS selectors adapt selected alignments. New direction-sensitive styling should be verified in both directions because several rules still use feature-specific overrides.

## Icons and Media

Lucide React supplies application icons. Decorative icons normally use `aria-hidden`; icon-only buttons commonly provide translated `aria-label` text. The frontend has a bundled font asset but no independent illustration/icon asset catalog or design asset pipeline.

## Responsive Evidence

The stylesheet contains responsive rules for shell, grids, Marketplace, builder, organization, and other feature layouts. Source proves responsive intent, not runtime quality. No screenshot suite, visual regression baseline, documented device matrix, or browser execution evidence was used in Phase 4, so usability remains `UI_BEHAVIOR_UNVERIFIED UBU-01`.

## Design-System Status

`COMPONENT_OWNERSHIP_UNCLEAR COU-01`: Shared classes and four common components create reuse, but no package boundary, ownership policy, supported component API, versioning, Storybook, token contract, or visual test suite establishes a formal design system.

## Change Risks

- Global selectors can change multiple features without an import-level dependency signal.
- Very large feature-specific stylesheet sections track very large page modules.
- Literal visual values make consistency and theme changes hard to audit.
- Tailwind utilities and semantic CSS could diverge if both are expanded without a stated convention.
- Responsive and RTL changes lack automated visual verification.

## Related Documents

- [Component Architecture](component-architecture.md)
- [Pages and Layouts](pages-and-layouts.md)
- [Localization and Direction](features/localization-and-direction.md)
- [Frontend Risks](frontend-risks.md)

