---
type: Project
title: Project Overview
description: Bounded overview of the ZinharCMS product scope and implemented repository release boundary.
status: draft
sources:
  - id: source-readme
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/README.md
    title: README.md at construction commit
  - id: source-license
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/LICENSE
    title: LICENSE at construction commit
---

# Project scope

ZinharCMS is a multi-tenant content-management and page-building application
with an administrative web application, tenant-scoped editorial workflows,
public delivery routes, preview support, extensibility hooks, and a
Marketplace capability surface. The repository identifies the current release
as 3.0.0.

## Implemented boundary observed in the repository

The application is implemented as a React/Vite frontend and a Rust/Axum/Tokio
backend backed by PostgreSQL and Redis, with local/reference file handling in
the repository configuration. The backend includes authentication, tenant
context, content, pages, media, delivery, plugins, and Marketplace route
families. The frontend provides the administrative route shell and client API
integration.

## Evidence limits

This Concept records repository-visible scope only. It does not assert a
production deployment, provider configuration, backup policy, observability
ownership, or an owner-approved terminology glossary. Those questions remain
outside the Phase 3 foundation.

The runtime composition is described in [system architecture](/architecture/system-architecture.md), and current delivery and security boundaries are described in [runtime and request boundaries](/architecture/runtime-and-request-boundaries.md) and [authentication and sessions](/security/authentication-and-sessions.md).
