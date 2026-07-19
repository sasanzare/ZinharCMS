---
okf_document_id: "extensibility-activation-deactivation"
title: "Activation and Deactivation"
project: "ZinharCMS"
category: "extensibility"
phase: 9
status: "current"
source_of_truth: false
implementation_view: "observed"
extensibility_status: "verified"
last_verified_commit: "56d733985fdd7aa3f25ee6981b88cf29c52f65c9"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/routes/plugins.rs"
  - "backend/src/plugins/mod.rs"
  - "backend/src/routes/marketplace.rs"
  - "backend/src/routes/marketplace_runtime.rs"
related_documents:
  - "plugin-lifecycle.md"
  - "plugin-permissions.md"
  - "tenant-and-global-scope.md"
related_diagrams:
  - "diagrams/plugin-lifecycle.mmd"
---

# Activation and Deactivation

Built-in enable/disable writes the global cms_plugins.is_enabled flag. Hook runners query enabled keys for every invocation. The action requires the global plugin-manager permission. Registry synchronization does not reset enablement during metadata updates.

Marketplace enable is allowed only from disabled; disable only from active. Enable rechecks the active kill-switch boundary. An emergency block uses separate runtime_status blocked state and can prevent new install, re-enable, or runtime authorization.

Activation does not start package code. It permits host-owned adapter visibility and runtime-policy decisions for eligible installations. Deactivation does not uninstall or delete data.

| Mechanism | Activation scope | Execution consequence |
|---|---|---|
| Built-in plugin | Global | Compiled hook callbacks become eligible |
| Marketplace installation | Organization | Declarative adapters become eligible |
| Marketplace runtime | Global/organization kill-switch overlays | Authorization may be ready or blocked |

## Enablement Behavior Matrix

| Concern | Built-in plugin | Marketplace installation |
|---|---|---|
| Installed | Means compiled/deployed; no application install state | Persisted organization installation |
| Registered | Metadata key synchronized from compiled registry | Version/permission record plus declarative definitions read on demand |
| Enabled | Global is_enabled true | Equivalent lifecycle eligibility is status active |
| Active | No separate state | Explicit active status plus runtime_status ready and no kill switch |
| Activation scope | Global | Tenant |
| Deactivation scope | Global | Tenant; global/tenant kill switch can overlay |
| Runtime registry update | Runners read enabled keys per invocation | Adapters query current active-ready installations |
| Route availability | Static routes remain registered | Static routes remain registered |
| UI availability | Admin UI reflects state; no dynamic UI route injection | Marketplace/Page Builder host UI filters installation state |
| Event registration | Static trait vector unchanged | Declarative hooks become discoverable; not executed |
| Settings availability | Global settings retained enabled or disabled | Manifest/snapshots retained |
| Data retention | Registry row/settings retained | Installation and organization data retained |
| Failure behavior | API error or callback error; no failed state | Invalid transition conflict; transaction rollback |
| Restart requirement | New code requires rebuild/redeploy; flag change does not | State change does not require restart |
| Tests | SEO callback only; route gap | Lifecycle/runtime helper tests |
