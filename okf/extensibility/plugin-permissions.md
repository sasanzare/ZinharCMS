---
okf_document_id: "plugin-permissions"
title: "Plugin Permissions"
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
  - "backend/src/services/marketplace_manifest.rs"
  - "backend/src/services/marketplace_runtime.rs"
  - "backend/migrations/0020_v3_phase_seven_permission_sandbox_kill_switch.sql"
  - "backend/src/services/rbac.rs"
related_documents:
  - "../security/roles-and-permissions-catalog.md"
  - "../security/permissions/marketplace-runtime-capabilities.md"
  - "isolation-and-trust.md"
  - "tenant-and-global-scope.md"
related_diagrams:
  - "diagrams/plugin-permission-flow.mmd"
---

# Plugin Permissions

## Built-in Plugins

Built-in callbacks run inside the backend process and receive application data/context directly. No capability token or per-plugin permission check surrounds execution. Global RBAC protects management routes, but does not sandbox plugin behavior. PLUGIN_PERMISSION_UNCLEAR applies to future built-ins; current effective authority is the host process.

## Marketplace Permission Catalog

| Permission | Host capability |
|---|---|
| content.read | Read organization content |
| content.write | Create or update organization content |
| page.read | Read organization pages |
| page.write | Create or update organization pages |
| media.read | Read organization media |
| media.write | Create or update organization media |
| webhook.send | Request a host-managed webhook |
| settings.read | Read permitted settings |
| external_network.request | Request allowlisted outbound network behavior |

Install requires the exact manifest permission array. Update requires fresh approval when permissions change. Runtime authorization maps each operation to one permission, compatible product types, and a manifest entry-point key. It also checks active status, ready runtime, safe path, kill switches, and payload size.

The response says execution: not_executed. It proves a policy decision, not actual data access or third-party code execution. See [Security Permissions](../security/permissions/marketplace-runtime-capabilities.md).

## Capability Enforcement Matrix

| Capability ID | Description | Request source | Grant mechanism | Enforcement path | Scope | Default | Denial behavior | Tests | Confidence |
|---|---|---|---|---|---|---|---|---|---|
| content.read | Read tenant content through a future host operation | Manifest permissions | Exact install/update approval snapshot | marketplace_runtime operation mapping | Tenant | Denied unless declared and approved | Authorization denied; not executed | Allowed/denial policy tests | High |
| content.write | Create/update tenant content | Manifest | Exact approval | Runtime policy | Tenant | Denied | Denied; no host execution found | Escalation test | High |
| page.read | Read tenant pages/component render context | Manifest | Exact approval | Runtime policy and host adapter context | Tenant | Denied | Denied | Allowed component test | High |
| page.write | Create/update pages | Manifest | Exact approval | Runtime policy; template import separately requires page writer | Tenant | Denied | Denied/host RBAC error | Policy and template tests | High |
| media.read | Read tenant media | Manifest | Exact approval | Runtime operation mapping | Tenant | Denied | Denied | Policy mapping tests | High |
| media.write | Create/update tenant media | Manifest | Exact approval | Runtime operation mapping | Tenant | Denied | Denied | Policy mapping tests | High |
| webhook.send | Request host-managed webhook behavior | Manifest | Exact approval | Runtime operation mapping | Tenant | Denied | Denied; execution absent | Policy mapping tests | High |
| settings.read | Read permitted settings | Manifest | Exact approval | Runtime operation mapping | Tenant | Denied | Denied; execution absent | Policy mapping tests | High |
| external_network.request | Request allowlisted outbound network behavior | Manifest | Exact approval | Runtime operation mapping | Tenant | Denied | Denied; no generic executor found | Policy/path tests | High |
| built_in.host_process | Effective authority of compiled CmsPlugin | Source inclusion | Trusted build/deploy, not manifest grant | No per-plugin runtime check | Process/global with tenant data passed | Available to compiled code | Only code review/build controls; callback error propagates | SEO behavior only | High |

Installer, configurator, and activator roles are defined by CMS/organization RBAC and are separate from the nine manifest capabilities. No plugin filesystem capability, route-registration capability, event-subscription grant, UI-registration grant, or administrative bypass capability is exposed as a third-party permission.
