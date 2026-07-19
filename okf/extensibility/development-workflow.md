---
okf_document_id: "extensibility-development-workflow"
title: "Extensibility Development Workflow"
project: "ZinharCMS"
category: "extensibility"
phase: 9
status: "current"
source_of_truth: false
implementation_view: "observed"
extensibility_status: "mixed"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/plugins"
  - "scripts/marketplace-cli.mjs"
  - "docs/MARKETPLACE_CREATOR_GUIDE.md"
  - "docs/V3_MARKETPLACE_MANIFEST_SCHEMA.md"
  - "docs/V3_PACKAGE_STORAGE.md"
related_documents:
  - "plugin-manifest.md"
  - "compatibility-and-versioning.md"
  - "extensibility-testing.md"
related_diagrams:
  - "diagrams/plugin-registration-flow.mmd"
---

# Extensibility Development Workflow

## Built-in Plugin

1. Implement CmsPlugin in backend source.
2. Add the implementation explicitly to builtin_plugins().
3. Add focused callback tests.
4. Build and deploy the backend.
5. Call the plugin-list endpoint to synchronize metadata.
6. Enable the global plugin record through authorized administration.

No generator or plugin-specific CLI was found for this path.

## Marketplace Product

The tracked marketplace-cli supports init, validate, pack, and submit commands. Creator documentation and sample manifests describe the repository-supported package workflow:

1. Initialize or author a package directory and manifest.
2. Validate manifest, permissions, compatibility, paths, assets, adapter declarations, and package findings.
3. Pack the artifact.
4. Submit multipart artifact plus manifest to a creator-owned listing.
5. Complete server validation and human review.
6. Publish an approved catalog version.
7. Install under a tenant with exact permission approval.
8. Use host-owned component/template/hook/runtime endpoints as supported.

Do not invent dynamic loading, signing commands, dependency installation, database migrations, local hot reload, or arbitrary execution. Their status is PLANNED_NOT_IMPLEMENTED or UNKNOWN.

## Supported Workflow

| Concern | Built-in plugin | Marketplace product |
|---|---|---|
| Template/example | SEO Auto is the concrete built-in example | docs/marketplace-samples contains component-pack and integration-plugin manifests |
| SDK/development package | No separate SDK found | scripts/marketplace-cli.mjs and creator documentation |
| Registration | Add implementation to builtin_plugins() | Upload validated package/version; install after approval |
| Local development | Normal backend build/test | CLI init/validate/pack plus repository sample edits |
| Build | Cargo build/test as application code | No package-code build contract beyond creator artifact preparation |
| Test | Rust unit/integration conventions | CLI/server validation and frontend/backend tests |
| Package | Not applicable | marketplace CLI pack command |
| Manifest validation | Not applicable | marketplace CLI validate plus server validator |
| Compatibility | Same-build compiler | Manifest host range and install gates |
| Development install | Deploy/enable global built-in | Normal tenant installation API after review; no special dev bypass documented |
| Debugging/logging | Host tracing; no plugin-specific logger | Validation reports, API errors, audit/review records |
| Publishing | Application deployment | CLI submit or web upload, then review/catalog |
| Upgrade | Application deployment | Explicit version upload, review, install update |
| Deprecation | Source/release process; stale row behavior unclear | Version/listing moderation and explicit rollback rules |

## Inferred Workflow

Using SEO Auto as a pattern for a second built-in is INFERRED_FROM_CODE; no generator guarantees conventions beyond the trait and registry. Local Marketplace iteration against a development server is plausible from CLI/API evidence but no dedicated development-install mode was found.

## Missing Tooling

- No built-in plugin scaffold, SDK, ABI package, dynamic loader, dependency solver, hot reload, plugin debugger, or per-plugin logger.
- No signature-generation/verification command or package migration command.
- No executable backend/frontend package runner or isolation test harness.

## Planned or Unsupported Workflow

Backend-extension and integration-plugin manifest types exist, but executable installation/runtime is PLANNED_NOT_IMPLEMENTED. Do not add undocumented commands or bypass review, compatibility, permission, tenant, or artifact gates.

## Repository Development Integration

Use the [Development Prerequisites](../development/prerequisites.md), [Command Catalog](../development/commands.md), [Testing Workflow](../development/testing-workflow.md), and [Contribution Workflow](../development/contribution-workflow.md). Marketplace CLI validate/pack are local tooling; submit is an authenticated mutation and requires explicit authorization. No package release/signing/sandbox deployment workflow exists.
