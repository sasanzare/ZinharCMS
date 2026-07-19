---
okf_document_id: "plugin-lifecycle"
title: "Plugin Lifecycle"
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
  - "backend/src/routes/plugins.rs"
  - "backend/src/services/marketplace_installation.rs"
  - "backend/src/routes/marketplace.rs"
  - "backend/migrations/0019_v3_phase_six_installation_lifecycle.sql"
related_documents:
  - "installation-and-removal.md"
  - "activation-and-deactivation.md"
  - "compatibility-and-versioning.md"
related_diagrams:
  - "diagrams/plugin-lifecycle.mmd"
  - "diagrams/marketplace-installation-flow.mmd"
---

# Plugin Lifecycle

## Built-in CMS Plugin Lifecycle

| State or action | Precondition | Result | Persistence |
|---|---|---|---|
| Compiled | Included in builtin_plugins() | Available to the backend process | Binary/source |
| Synchronized | Plugin list requested | Metadata upserted | Global cms_plugins |
| Enabled | Plugin-manager action | is_enabled becomes true | Global row |
| Disabled | Plugin-manager action | is_enabled becomes false | Global row |
| Updated | Metadata/settings PATCH | Selected fields stored | Global row |
| Removed from source | Deployment change | Executable absent | Stale row cleanup not found |

The CMS mechanism has no install/uninstall API and no explicit migration callback.

## Marketplace Installation Lifecycle

| Current state | Allowed actions | Result |
|---|---|---|
| active | disable, uninstall, update, rollback | disabled, uninstalled, or version change |
| disabled | enable, uninstall, update, rollback | active, uninstalled, or version change |
| blocked | uninstall | uninstalled |
| uninstalled | none | Terminal |

Updates require a newer compatible version and changelog confirmation. Permission changes require reapproval. Rollback requires a stored safe target and acceptable permission snapshot. Versions remain pinned; no automatic update path was found.

PLUGIN_LIFECYCLE_UNCLEAR applies to compiled-plugin removal and metadata cleanup. PLUGIN_UPDATE_BEHAVIOR_UNCLEAR is resolved for Marketplace installations but remains unclear for built-in code changes.

## Verified State Detail

| Mechanism/state | Entry condition | Exit condition | Allowed transitions | Required permission | Scope | Database change | Runtime change | Side effects | Failure behavior | Tests |
|---|---|---|---|---|---|---|---|---|---|---|
| Built-in available/compiled | Source included and build succeeds | New deployment without source | synchronize metadata | Build/deploy authority outside app | Global | None | Code becomes available in process | None | Compile/start failure | Same-build unit tests |
| Built-in registered | Plugin-list sync upserts key | Source removal does not delete row | enable, disable, metadata/settings update | Global plugin reader triggers sync; manager mutates | Global | cms_plugins insert/update | No callback eligibility change unless enable state differs | Metadata refresh | API/DB error | No route integration test |
| Built-in enabled | Manager enables row | Disable or absent compiled implementation | disable, update | Global plugin manager | Global | is_enabled true | Callback becomes eligible when compiled key matches | Future requests invoke hook | Error returned; no partial callback isolation | No management-route test |
| Built-in disabled | Manager disables row | Enable | enable, update | Global plugin manager | Global | is_enabled false | Callback skipped | Data/settings retained | Error returned | No management-route test |
| Marketplace active | Successful install or enable | Disable, block, uninstall, update, rollback | disable, uninstall, update, rollback | Tenant installer; approver when permissions change | Tenant | Status/version/snapshots/timestamps | Host adapters eligible if runtime ready | Audit; possible registry/import use later | Transaction rollback on DB error | Lifecycle helper tests |
| Marketplace disabled | Disable from active | Enable, uninstall, update, rollback | enable, uninstall, update, rollback | Tenant installer | Tenant | Status/timestamps | Adapters ineligible | Data retained; audit | Conflict on invalid transition | Lifecycle helper tests |
| Marketplace blocked | Emergency control | Uninstall or authorized kill-switch recovery changes runtime overlay | uninstall | Privileged Marketplace control for block; installer for uninstall | Tenant or global overlay | runtime_status/block fields; lifecycle status can be blocked | Authorization/install/re-enable denied | Audit | Denial with reason | Runtime/lifecycle tests |
| Marketplace uninstalled | Soft uninstall from active/disabled/blocked | No exit | None | Tenant installer | Tenant | status and uninstalled_at retained | Adapters ineligible | Organization data preserved; audit | Terminal conflict | Lifecycle helper test |
| Marketplace updating | No persisted state; request-local transaction | Commit or rollback | active/disabled with new version | Tenant installer and possible permission approver | Tenant | Version, rollback target, snapshot, timestamps | New declarations become current after commit | Audit | Transaction rollback; old version remains on failure | Pure/helper tests |
| Marketplace incompatible | Eligibility decision, not persisted lifecycle state | Compatible version/host/plan | Install/update denied | None | Tenant decision | None | None | User receives conflict/report | Validation error | Compatibility tests |

No generic failed plugin state was found. Callback failure returns an error but does not persist a failed status.
