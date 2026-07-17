---
okf_document_id: "frontend-configuration-build"
title: "Frontend Configuration and Build"
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
  - "frontend/package-lock.json"
  - "frontend/vite.config.ts"
  - "frontend/vitest.config.ts"
  - "frontend/tsconfig.json"
  - "frontend/tsconfig.app.json"
  - "frontend/tsconfig.node.json"
  - "frontend/eslint.config.js"
  - "frontend/Dockerfile"
  - "frontend/Dockerfile.prod"
  - "frontend/nginx.conf"
  - ".github/workflows/frontend-ci.yml"
related_documents:
  - "frontend/application-catalog.md"
  - "frontend/testing-map.md"
  - "frontend/api-client.md"
  - "frontend/frontend-risks.md"
  - "project/repository-map.md"
related_diagrams:
  - "frontend/diagrams/frontend-application-map.mmd"
uncertainty_markers:
  - "UNKNOWN U-06"
  - "UNKNOWN U-09"
  - "IMPLEMENTATION_STATUS_UNCLEAR ISU-03"
---

# Frontend Configuration and Build

## Package and Scripts

| Script | Command | Purpose |
|---|---|---|
| `dev` | Vite bound to all interfaces | Development server |
| `build` | TypeScript build then Vite build | Production bundle |
| `preview` | Vite preview bound to all interfaces | Local bundle preview |
| `typecheck` | TypeScript project build | Static type validation |
| `test` | Vitest single run | Unit/component tests |
| `lint` | ESLint current package | Static lint checks |

No package script for formatting, coverage, end-to-end tests, Storybook, visual tests, bundle analysis, or deployment was found.

## Build Configuration

Vite registers React and Tailwind plugins. The development server listens on `0.0.0.0:5173`. No development API proxy, path alias, explicit chunk split, bundle-size budget, source-map policy, service worker, PWA plugin, or custom output directory was found.

`VITE_API_URL` is the verified frontend runtime-build variable. It is compiled into the browser bundle and must not contain secrets. The API client supplies a local fallback.

## TypeScript

The root frontend TypeScript project references application and Node/config projects. Application settings include strict mode, ES2022, DOM libraries, bundler module resolution, isolated modules, JSON imports, React JSX, and no emit. The Node/config project targets ES2023. No path aliases are configured.

## Linting

ESLint uses the JavaScript recommended set, TypeScript ESLint recommended sets, React Hooks rules, browser globals, and the React Refresh export rule. Generated `dist` is ignored. The configuration does not establish formatting or accessibility lint rules.

## Containers and Static Hosting

| File | Observed behavior |
|---|---|
| `frontend/Dockerfile` | Node 24 Alpine development image; `npm install`; Vite dev server on 5173 |
| `frontend/Dockerfile.prod` | Node 24 Alpine builder; optional `VITE_API_URL`; `npm install`; build; copy `dist` into Nginx 1.27 Alpine |
| `frontend/nginx.conf` | Static root, SPA history fallback, immutable one-year cache for `/assets/` |

The Nginx image does not proxy the API; browser requests go to the compiled API base URL.

## CI

`.github/workflows/frontend-ci.yml` runs on frontend/workflow changes for pushes and pull requests. It uses Node 22, runs `npm install`, then lint, typecheck, tests, and build in that order.

The repository therefore uses different Node majors in CI and Docker images. `UNKNOWN U-09` covers the unsupported official toolchain matrix. The workflow is a validation pipeline, not deployment evidence.

## Dependency Observations

- Runtime: React, React DOM, React Router, Zustand, dnd-kit, Lucide, Tailwind.
- Declared but without verified source imports: React Hook Form, Hook Form resolvers, and Zod (`DEAD_OR_UNUSED_CODE_UNCONFIRMED DU-02`).
- Testing/build: Vite, Vitest, Testing Library, jsdom, TypeScript, ESLint.
- `package-lock.json` exists and is the npm resolution record; Phase 4 did not audit dependency vulnerabilities or licenses.

## Deployment Status

Production-like images, Nginx behavior, and Compose configuration establish an available packaging path. They do not establish a deployed environment, actual `VITE_API_URL`, CDN/ingress, release promotion, rollback, or live version. Those remain `UNKNOWN U-06` and `IMPLEMENTATION_STATUS_UNCLEAR ISU-03`.

## Related Documents

- [Application Catalog](application-catalog.md)
- [Testing Map](testing-map.md)
- [API Client](api-client.md)
- [Frontend Risks](frontend-risks.md)
- [Repository Map](../project/repository-map.md)

