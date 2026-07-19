---
okf_document_id: "extensibility-installation-removal"
title: "Installation and Removal"
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
  - "backend/src/routes/marketplace.rs"
  - "backend/src/services/marketplace_installation.rs"
  - "backend/migrations/0019_v3_phase_six_installation_lifecycle.sql"
related_documents:
  - "plugin-lifecycle.md"
  - "plugin-data-and-migrations.md"
  - "marketplace/installation-update-rollback.md"
related_diagrams:
  - "diagrams/marketplace-installation-flow.mmd"
---

# Installation and Removal

Built-in CMS plugins are installed only by adding source registration, compiling, and deploying the backend. No runtime install, upload, or uninstall endpoint exists for CmsPlugin.

Marketplace install is a transactional tenant operation. It checks installer authorization, kill switches, plan compatibility, listing/version/review state, product type, entitlement for paid products, exact permission approval, and stored artifact size/path/checksum. It then creates an active, version-pinned installation and audit record.

The inspected base install gate admits component_pack and design_template. Although the manifest validator recognizes integration_plugin and backend_extension, new installation of those types is rejected by the current MVP gate.

Marketplace removal is a soft transition to uninstalled. The record and audit history remain; cleanup_policy is preserve_organization_data. No organization pages, media, settings, registry records, or package artifacts are deleted by this transition.

PLUGIN_REMOVAL_BEHAVIOR_UNCLEAR applies to stale Marketplace-derived component rows and uploaded artifact retention. Application uninstall semantics are otherwise VERIFIED.

## Operation Matrix

| Operation | Actor | Permission | Preconditions | Steps | Transaction boundary | Side effects | Failure behavior | Recovery | Audit event | Confidence |
|---|---|---|---|---|---|---|---|---|---|---|
| Add built-in plugin | Backend developer/deployer | Repository and deployment authority | Implement trait and register source | Code, test, compile, deploy, metadata sync, enable | Build/deployment outside application DB transaction | New trusted callback code | Build/start failure or callback error | Redeploy previous binary; DB row may remain | No application install audit | High for source path |
| Marketplace install | Organization owner/admin installer; approver as required | Marketplace installer and exact permission approval | Active tenant; eligible approved version; product type; plan; entitlement if paid; no kill switch; artifact valid | Load candidate, validate, approve, verify, insert active pinned installation, audit, commit | One tenant DB transaction after filesystem verification | Installation and audit records | Request error and DB rollback; artifact remains stored | Correct inputs/state and retry | marketplace.installation.install | High |
| Marketplace update | Tenant installer; permission approver if changed | Installer plus approver when needed | Active/disabled; newer compatible approved version; changelog confirmed; entitlement; artifact valid | Lock/load, validate, snapshot rollback, change version/permissions, audit, commit | One tenant transaction | Version and permission snapshot change | Rollback preserves current version | Retry or explicit rollback after successful update | marketplace.installation.update | High |
| Marketplace rollback | Tenant installer | Installer | Active/disabled; retained same-listing target; compatible artifact; acceptable permission snapshot | Validate target, swap current/rollback versions and snapshots, audit, commit | One tenant transaction | Version reversal | Rollback on error | Fix target/state or remain on current | marketplace.installation.rollback | High |
| Marketplace uninstall | Tenant installer | Installer | active, disabled, or blocked | Validate transition, set uninstalled and timestamps, audit | Tenant transaction in status helper | Removed from active lists; data retained | Conflict/DB error leaves prior state | Retry | marketplace.installation.uninstall | High |

## Installation and Cleanup Findings

| Concern | Verified finding |
|---|---|
| Package retrieval | Uses previously stored artifact path; no remote runtime retrieval during install. |
| Manifest/compatibility/permission review | Enforced before installation. |
| File extraction | No install-time executable extraction or loading path found. |
| Code registration | Not performed for Marketplace artifacts. |
| Migrations | Application migrations only; package migrations unsupported. |
| Assets | Declared assets are validated and template assets are mapped by host code. |
| Configuration | Installation snapshot and manifest metadata; no generic config setup. |
| Activation | Initial Marketplace status is active; built-ins require separate enablement. |
| Removal cleanup | Organization data is explicitly preserved; artifact, derived registry, and config cleanup remain unclear. |
| Global installation | Built-in deployment is global; Marketplace installation is tenant-specific. |
