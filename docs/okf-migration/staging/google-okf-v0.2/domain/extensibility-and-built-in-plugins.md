---
type: Component
title: Extensibility and Built-in Plugins
description: In-process plugin hooks, built-in SEO behavior, registry routes, and the boundary to Marketplace adapters.
status: draft
sources:
  - id: source-plugin-trait
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/plugins/mod.rs
    title: backend/src/plugins/mod.rs at construction commit
  - id: source-seo-plugin
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/plugins/seo.rs
    title: backend/src/plugins/seo.rs at construction commit
  - id: source-plugin-routes
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/routes/plugins.rs
    title: backend/src/routes/plugins.rs at construction commit
  - id: source-marketplace-runtime
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/services/marketplace_runtime.rs
    title: backend/src/services/marketplace_runtime.rs at construction commit
---

# Plugin boundary

The backend defines a `CmsPlugin` trait with entry-save and publish hooks. The
built-in registry includes an in-process SEO plugin that can derive a slug from
an entry title when the slug is empty. Plugin registry routes synchronize the
built-in set and allow authorized enable/disable operations.

Built-in plugins execute within the trusted Rust process. Marketplace packages
are handled through validated adapters and a constrained runtime authorization
surface; the current adapter does not execute arbitrary package code. This
distinction is an implementation boundary, not a claim that all future
extension policy is settled.

Plugin ownership, Marketplace roadmap boundaries, and the long-term policy for
third-party execution remain open migration decisions. Content integration is
described in [content and editorial workflow](content-and-editorial-workflow.md), and the wider Marketplace surface in [Marketplace](marketplace.md).

## Preserved visualization

### plugin-data-ownership

```mermaid
flowchart LR
    Entry["Content entry operation"] --> Hooks["CmsPlugin hooks"]
    Hooks --> Seo["Built-in SEO plugin"]
    Seo --> Slug["Derive slug when empty"]
    Marketplace["Marketplace package"] --> Adapter["Validated adapter boundary"]
    Adapter --> NonExec["No arbitrary package execution"]
```
