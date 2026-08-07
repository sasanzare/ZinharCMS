---
type: API Contract
title: Marketplace and Extension Contracts
description: Observed Marketplace, runtime-adapter, and built-in plugin route families with their access and execution boundaries.
status: draft
sources:
  - id: source-route-composition
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/src/routes/mod.rs
    title: backend/src/routes/mod.rs at Phase 5 source HEAD
  - id: source-marketplace-routes
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/src/routes/marketplace.rs
    title: backend/src/routes/marketplace.rs at Phase 5 source HEAD
  - id: source-runtime-routes
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/src/routes/marketplace_runtime.rs
    title: backend/src/routes/marketplace_runtime.rs at Phase 5 source HEAD
  - id: source-adapter-routes
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/src/routes/marketplace_adapters.rs
    title: backend/src/routes/marketplace_adapters.rs at Phase 5 source HEAD
  - id: source-plugin-routes
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/src/routes/plugins.rs
    title: backend/src/routes/plugins.rs at Phase 5 source HEAD
  - id: source-rbac
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/src/services/rbac.rs
    title: backend/src/services/rbac.rs at Phase 5 source HEAD
---

# Route-family boundary

The tenant-protected router composes Marketplace catalog, reviews, abuse
reports, installations, finance, analytics, runtime, and adapter families.
The authenticated protected router separately composes the built-in plugin
family. The route module registers these handlers and selected schemas in the
served OpenAPI document; this evidence supports a family-level contract, not
an exhaustive compatibility or deprecation contract.

The current families are:

* catalog, creator, listing submission, review, moderation, and abuse flows;
* organization installation, update, rollback, enable, disable, and removal;
* runtime permissions, status, authorization, and organization/global kill
  switch operations;
* component-pack, template, and hook adapters;
* purchases, revenue ledger, payout onboarding, balances, and payout requests;
* creator and administrator analytics; and
* built-in plugin listing, inspection, update, enable, and disable operations.

# Access decisions

Tenant routes require authenticated organization context. Organization admin
checks protect installation, Marketplace permission approval, kill-switch,
webhook, and billing-management operations where the current route calls the
corresponding RBAC helper. Global admin checks protect moderation, review
queues, abuse resolution, payout verification, and administrator analytics.
Creator operations also resolve the current creator owner before changing a
creator-owned listing or payout surface. The exact role matrix remains in
[authorization and RBAC](/security/authorization-and-rbac.md).

# Extension and execution boundary

Built-in plugins are registered and controlled in-process. Marketplace
artifacts are validated, reviewed, installed, and passed through adapter or
runtime authorization gates. The current runtime result explicitly reports
that the operation was not executed; this route contract does not authorize
arbitrary package code execution or external network execution.

Finance routes and Marketplace tables provide implementation mechanisms, but
they do not prove provider settlement, refunds, disputes, tax handling, or
automatic transfer guarantees. Runtime and package controls are consolidated
in [Marketplace runtime and safety boundary](/domain/marketplace-runtime-and-safety-boundary.md), and storage/quarantine controls are described in [storage and file security](/security/storage-and-file-security.md).

## Open decision dependencies

* NOC-08: route annotations and `/openapi.json` do not establish a declared
  versioning, compatibility, or deprecation policy.
* NOC-10 and NOC-17: Marketplace scope, settlement, disputes, external
  execution, cleanup, and archive-retention policy require owner evidence.

