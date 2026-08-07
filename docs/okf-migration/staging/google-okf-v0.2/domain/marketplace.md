---
type: Marketplace Domain
title: Marketplace
description: Implemented Marketplace capability surface, persistence boundary, lifecycle controls, and non-executing runtime adapter.
status: draft
sources:
  - id: source-marketplace-routes
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/routes/marketplace.rs
    title: backend/src/routes/marketplace.rs at construction commit
  - id: source-marketplace-runtime
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/routes/marketplace_runtime.rs
    title: backend/src/routes/marketplace_runtime.rs at construction commit
  - id: source-marketplace-finance
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/routes/marketplace_finance.rs
    title: backend/src/routes/marketplace_finance.rs at construction commit
  - id: source-marketplace-migrations
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/migrations/0015_v3_phase_one_marketplace_foundation.sql
    title: backend/migrations/0015_v3_phase_one_marketplace_foundation.sql at construction commit
  - id: source-marketplace-policy
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/backend/src/services/marketplace_policy.rs
    title: backend/src/services/marketplace_policy.rs at construction commit
---

# Implemented capability surface

The Marketplace route family covers catalog and listing flows, creator and
submission operations, moderation, reviews and abuse handling, installation
lifecycle, runtime status and authorization, analytics, adapters, purchases,
revenue ledger, payout onboarding, balances, and payout requests. The
repository contains Marketplace schema migrations and service support for
these surfaces.

Marketplace runtime authorization checks active installations, ready runtime
state, allowed product type and entry point, declared permissions, payload
size, and kill-switch state. The current runtime adapter returns an explicit
non-executed result; it does not execute arbitrary package code on the server.

This Concept describes implemented route and policy boundaries. The scope of
future Marketplace roadmap work, settlement/refund policy, and operational
ownership are not inferred from route names or documentation strings and remain
open decisions.

The extensibility relationship is described in [extensibility and built-in plugins](/domain/extensibility-and-built-in-plugins.md), and route authorization in [authorization and RBAC](/security/authorization-and-rbac.md).

## Preserved visualization

### marketplace-installation-flow

```mermaid
flowchart LR
    Listing["Marketplace listing"] --> Review["Submission and moderation"]
    Review --> Install["Installation lifecycle"]
    Install --> Runtime["Runtime authorization"]
    Runtime --> Result["Current adapter result: not executed"]
```
