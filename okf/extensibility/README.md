---
okf_document_id: "extensibility-readme"
title: "Plugins, Marketplace, and Extensibility"
project: "ZinharCMS"
category: "extensibility"
phase: 9
status: "current"
source_of_truth: false
implementation_view: "observed"
extensibility_status: "mixed"
last_verified_commit: "56d733985fdd7aa3f25ee6981b88cf29c52f65c9"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/plugins"
  - "backend/src/routes/plugins.rs"
  - "backend/src/routes/marketplace.rs"
  - "backend/src/routes/marketplace_adapters.rs"
  - "backend/src/routes/marketplace_runtime.rs"
  - "backend/src/services"
  - "backend/migrations"
  - "frontend/src/pages/MarketplacePage.tsx"
  - "frontend/src/pages/PagesPage.tsx"
related_documents:
  - "../backend/module-catalog.md"
  - "../frontend/page-builder.md"
  - "../database/entity-catalog.md"
  - "../api/endpoint-catalog.md"
  - "../security/trust-boundaries.md"
  - "../domain/page-builder-rules.md"
related_diagrams:
  - "diagrams/extensibility-context.mmd"
  - "diagrams/plugin-registration-flow.mmd"
  - "diagrams/plugin-lifecycle.mmd"
  - "diagrams/component-registration.mmd"
  - "diagrams/plugin-permission-flow.mmd"
  - "diagrams/plugin-data-ownership.mmd"
  - "diagrams/marketplace-installation-flow.mmd"
---

# Plugins, Marketplace, and Extensibility

## Purpose and Authority

This Phase 9 section describes extension behavior observed in the repository. Current source code, migrations, manifests, registered routes, and tests remain authoritative. Historical V3 documents provide intent only where current implementation agrees.

## Verified Classification

ZinharCMS is a mixed, partially implemented extensibility platform:

- **STATIC_EXTENSION_REGISTRATION** and **BUILT_IN_EXTENSIONS_ONLY** describe the executable CMS plugin mechanism.
- **COMPONENT_REGISTRY_ONLY** describes the Page Builder registry; records are schemas and metadata, not executable modules.
- **MARKETPLACE_INTEGRATED** describes implemented creator, review, catalog, installation, finance, feedback, analytics, permission, and host-adapter flows.
- **PARTIALLY_IMPLEMENTED** applies because uploaded Marketplace package code is not dynamically loaded or executed.
- PLUGIN_SYSTEM_NOT_FOUND is false, but DYNAMIC_PLUGIN_LOADING is not supported by inspected code.

## Reading Order

1. [Overview](overview.md)
2. [Terminology](terminology.md)
3. [Extensibility Catalog](extensibility-catalog.md)
4. [Plugin Architecture](plugin-architecture.md)
5. [Extension Points](extension-points.md)
6. [Plugin Manifest](plugin-manifest.md)
7. [Discovery and Registration](plugin-discovery-and-registration.md)
8. [Plugin Lifecycle](plugin-lifecycle.md)
9. [Permissions](plugin-permissions.md), [Scope](tenant-and-global-scope.md), and [Isolation](isolation-and-trust.md)
10. [Marketplace Architecture](marketplace-architecture.md) and [Workflows](marketplace-workflows.md)
11. [Testing](extensibility-testing.md) and [Risks](extensibility-risks.md)

## Primary Documents

- [Overview](overview.md)
- [Terminology](terminology.md)
- [Extensibility Catalog](extensibility-catalog.md)
- [Plugin Architecture](plugin-architecture.md)
- [Extension Points](extension-points.md)
- [Plugin Manifest](plugin-manifest.md)
- [Plugin Discovery and Registration](plugin-discovery-and-registration.md)
- [Plugin Lifecycle](plugin-lifecycle.md)
- [Installation and Removal](installation-and-removal.md)
- [Activation and Deactivation](activation-and-deactivation.md)
- [Plugin Configuration](plugin-configuration.md)
- [Plugin Permissions](plugin-permissions.md)
- [Tenant and Global Scope](tenant-and-global-scope.md)
- [Backend Extension Points](backend-extension-points.md)
- [Frontend Extension Points](frontend-extension-points.md)
- [Component and Block Registration](component-and-block-registration.md)
- [Hooks and Events](hooks-and-events.md)
- [Plugin Data and Migrations](plugin-data-and-migrations.md)
- [Compatibility and Versioning](compatibility-and-versioning.md)
- [Isolation and Trust](isolation-and-trust.md)
- [Marketplace Architecture](marketplace-architecture.md)
- [Marketplace Workflows](marketplace-workflows.md)
- [Development Workflow](development-workflow.md)
- [Extensibility Testing](extensibility-testing.md)
- [Extensibility Risks](extensibility-risks.md)

## Verified Plugin

- [SEO Auto Generator](plugins/seo-auto.md)

## Verified Extension-Point Documents

- [CMS Entry Before-Save Hook](extension-points/cms-entry-before-save.md)
- [CMS Entry After-Publish Hook](extension-points/cms-entry-after-publish.md)
- [Page Builder Component Registry](extension-points/page-builder-component-registry.md)
- [Marketplace Template Adapter](extension-points/marketplace-template-adapter.md)
- [Marketplace Public Hook Adapter](extension-points/marketplace-public-hook-adapter.md)
- [Marketplace Runtime Authorization Boundary](extension-points/marketplace-runtime-authorization.md)

## Marketplace Area Documents

- [Creator, Listing, and Version Management](marketplace/creator-listing-version.md)
- [Package Validation and Review](marketplace/package-validation-review.md)
- [Installation, Update, and Rollback](marketplace/installation-update-rollback.md)
- [Runtime Permissions and Host Adapters](marketplace/runtime-permissions-adapters.md)
- [Commerce and Entitlements](marketplace/commerce-entitlements.md)
- [Feedback, Moderation, and Analytics](marketplace/feedback-moderation-analytics.md)

## Diagram Navigation

- [Extensibility Context](diagrams/extensibility-context.mmd)
- [Plugin Registration Flow](diagrams/plugin-registration-flow.mmd)
- [Plugin Lifecycle](diagrams/plugin-lifecycle.mmd)
- [Component Registration](diagrams/component-registration.mmd)
- [Plugin Permission Flow](diagrams/plugin-permission-flow.mmd)
- [Plugin Data Ownership](diagrams/plugin-data-ownership.mmd)
- [Marketplace Installation Flow](diagrams/marketplace-installation-flow.mmd)

## Terminology Boundaries

A plugin is a compiled CmsPlugin implementation. An extension point is a verified host contract. A component or block is Page Builder registry metadata. A module is ordinary application source unless it implements an extension contract. An integration uses a host-owned boundary such as webhooks, Stripe, or Marketplace adapters. Documentation describes these mechanisms but does not install, register, activate, or execute them.

## For AI Coding Agents

1. Start with [Overview](overview.md).
2. Read [Terminology](terminology.md).
3. Locate the relevant mechanism in the [Extensibility Catalog](extensibility-catalog.md).
4. Read its dedicated extension-point document.
5. Review [Plugin Permissions](plugin-permissions.md) and [Isolation and Trust](isolation-and-trust.md).
6. Review [Compatibility and Versioning](compatibility-and-versioning.md) before changing an interface.
7. Review [Plugin Data and Migrations](plugin-data-and-migrations.md) before adding persistence.
8. Review [Tenant and Global Scope](tenant-and-global-scope.md) before changing availability.
9. Verify registration against current runtime source code.
10. Never assume a planned Marketplace capability is implemented.
11. Never load or execute untrusted plugin code.
12. Update related OKF documents when an extension contract changes.

## Important Limitation

The executable CMS plugin surface is trusted, compiled Rust code. Marketplace packages are validated and persisted, while inspected runtime endpoints return authorization decisions and host-owned adapters interpret declarative data. PLUGIN_LOADING_UNCLEAR is resolved for current source as **no dynamic loader found**; a deployed runtime was not inspected. PLUGIN_ISOLATION_UNVERIFIED, PLUGIN_TRUST_MODEL_UNCLEAR, and MARKETPLACE_BEHAVIOR_UNCLEAR remain relevant wherever deployment behavior or future package execution is assumed.

This documentation does not certify any third-party plugin as safe, trusted, compatible, or isolated.
