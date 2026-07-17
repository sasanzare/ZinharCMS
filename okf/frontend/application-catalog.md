---
okf_document_id: "frontend-application-catalog"
title: "Frontend Application Catalog"
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
  - "frontend/src/main.tsx"
  - "frontend/src/router.tsx"
  - "frontend/Dockerfile"
  - "frontend/Dockerfile.prod"
  - "frontend/nginx.conf"
related_documents:
  - "frontend/README.md"
  - "frontend/overview.md"
  - "frontend/feature-catalog.md"
  - "frontend/configuration-and-build.md"
related_diagrams:
  - "frontend/diagrams/frontend-application-map.mmd"
uncertainty_markers:
  - "INFERRED_FROM_STRUCTURE"
  - "IMPLEMENTATION_STATUS_UNCLEAR ISU-03"
---

# Frontend Application Catalog

## Selection Rule

An application must have a distinct startup, build, runtime, or deployment boundary. Directories, pages, libraries, and route groups are not cataloged as applications merely because they have separate responsibilities.

## Summary

| ID | Application | Path | Runtime | Audience | Build and deployment | Status | Confidence |
|---|---|---|---|---|---|---|---|
| FE-APP-001 | ZinharCMS Management SPA | `frontend/` | Browser SPA | Authenticated CMS, SaaS, and Marketplace users | Vite build; Nginx production image; Vite development image | `IMPLEMENTED`; deployed status `IMPLEMENTATION_STATUS_UNCLEAR ISU-03` | High |

## FE-APP-001: ZinharCMS Management SPA

| Field | Value |
|---|---|
| Application ID | `FE-APP-001` |
| Name | ZinharCMS Management SPA |
| Primary source root | `frontend/` |
| Startup entry point | `frontend/src/main.tsx` |
| Route entry point | `frontend/src/router.tsx` |
| Package manifest | `frontend/package.json` |
| Framework and language | React 19 and strict TypeScript |
| Bundler and development server | Vite 6 |
| Runtime | User browser |
| Primary backend | ZinharCMS Rust API through `VITE_API_URL` |
| Development container | `frontend/Dockerfile`, Node 24 Alpine, Vite port 5173 |
| Production-like container | `frontend/Dockerfile.prod`, Vite build copied to Nginx 1.27 Alpine |
| SPA fallback | `frontend/nginx.conf` uses `try_files` to return `index.html` |
| Deployment proof | Not present; repository files establish capability, not live deployment |

### Responsibilities

- Compose the authenticated management shell and route pages.
- Acquire and persist browser session and active-organization context.
- Call backend management APIs and display results.
- Provide feature workflows and visual Page Builder interaction.
- Localize UI copy and switch document direction.

### Excluded Application Candidates

| Candidate | Why it is not a separate application |
|---|---|
| `frontend/src/pages` | Route-level components compiled into FE-APP-001 |
| `frontend/src/components` | Shared and page-supporting components compiled into FE-APP-001 |
| `frontend/src/i18n` | Cross-cutting runtime library inside FE-APP-001 |
| Page Builder | A feature implemented inside `PagesPage.tsx`, not a separate entry point or bundle |
| Marketplace | A route and feature inside the same SPA |
| Public delivery | Backend data-delivery behavior; no separate public frontend application was found |
| `frontend/dist` | Generated build output, not source or an independently owned application |

### Entry and Exit Boundaries

The browser enters through `index.html`, loads the Vite bundle, and mounts `main.tsx`. External exits are backend HTTP calls, the copied backend WebSocket preview URL, browser navigation to billing-provider URLs returned by the backend, clipboard operations, and browser storage. Detailed endpoint contracts are deferred to Phase 6.

### Known Unknowns

- Supported production browsers and devices are `UNKNOWN` under the broader toolchain owner question.
- The actual deployed origin, CDN, ingress, release mechanism, and rollback state are not established.
- No independent public-site application is present; intended public rendering ownership requires owner confirmation if such an application is expected.

## Related Documents

- [Frontend Overview](overview.md)
- [Feature Catalog](feature-catalog.md)
- [Configuration and Build](configuration-and-build.md)
- [Frontend Application Map](diagrams/frontend-application-map.mmd)

