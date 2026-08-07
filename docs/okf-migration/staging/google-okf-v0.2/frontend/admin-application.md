---
type: Component
title: Admin Application
description: React/Vite administrative single-page application shell, provider composition, and feature route surface.
status: stable
sources:
  - id: source-main
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/frontend/src/main.tsx
    title: frontend/src/main.tsx at construction commit
  - id: source-router
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/frontend/src/router.tsx
    title: frontend/src/router.tsx at construction commit
  - id: source-package
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/frontend/package.json
    title: frontend/package.json at construction commit
---

# Application shell

The administrative application is a React/Vite single-page application. Its
entry point composes strict-mode rendering, internationalization, session
bootstrap, and the client router before mounting the application into the
browser document.

The router exposes login and authenticated feature areas for dashboard,
content types, entries, media, Marketplace, pages, workflow, organization,
workspace, billing, beta, settings, and a catch-all route. Authentication is
enforced by the client route guard and backed by the server contracts; the
client route list is not a substitute for server authorization.

Build and test tooling, including Vite, TypeScript, ESLint, Vitest, and the
package-manager version boundary, is recorded in [development and testing](/development/development-and-testing.md). Client token and organization
state are described in [routing and state](/frontend/routing-and-state.md).
