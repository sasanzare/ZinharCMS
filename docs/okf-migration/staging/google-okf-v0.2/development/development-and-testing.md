---
type: Development Guide
title: Development and Testing
description: Repository development commands, package boundaries, and CI checks evidenced by manifests and workflows.
status: draft
sources:
  - id: source-readme
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/README.md
    title: README.md at construction commit
  - id: source-frontend-package
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/frontend/package.json
    title: frontend/package.json at construction commit
  - id: source-backend-ci
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/.github/workflows/backend-ci.yml
    title: .github/workflows/backend-ci.yml at construction commit
  - id: source-frontend-ci
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/.github/workflows/frontend-ci.yml
    title: .github/workflows/frontend-ci.yml at construction commit
---

# Development surface

The repository separates the Rust backend and React/Vite frontend. The
frontend manifest defines development, build, typecheck, test, lint, and
security-sink scripts and constrains the supported Node/npm major versions.
The backend uses Cargo formatting, linting, and tests together with SQLx
migrations embedded from the backend migration directory.

The backend CI workflow provisions PostgreSQL 16 and Redis 7 for its job and
runs format, Clippy with warnings denied, and tests. The frontend workflow
installs dependencies, runs the declared audit, lint, typecheck, test, and
build checks, and verifies the expected version boundary.

These workflow definitions are repository evidence of CI gates, not proof of a
successful deployment or of every external environment. Support-version,
contribution/check, and long-term development ownership decisions remain open.

The runtime and package boundaries are described in [backend runtime](/backend/backend-runtime.md) and [admin application](/frontend/admin-application.md).
