---
okf_document_id: "security-permission-marketplace-runtime"
title: "Marketplace Runtime Capabilities Permission Group"
project: "ZinharCMS"
category: "security-permission-group"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "partially_verified"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
permission_group_id: "marketplace-runtime-capabilities"
permission_group_name: "Marketplace Runtime Capabilities"
resource_domain: "Marketplace package-to-host operations"
permission_scope: "approved installation within an organization"
implementation_status: "verified"
primary_sources:
  - "backend/src/services/marketplace_manifest.rs"
  - "backend/src/services/marketplace_installation.rs"
  - "backend/src/services/marketplace_runtime.rs"
  - "backend/src/routes/marketplace_runtime.rs"
  - "backend/migrations/0020_v3_phase_seven_permission_sandbox_kill_switch.sql"
related_documents:
  - "../roles-and-permissions-catalog.md"
  - "../input-validation.md"
  - "../threat-register.md"
  - "../../api/endpoints/marketplace-runtime-security.md"
related_diagrams:
  - "../diagrams/authorization-decision-flow.mmd"
  - "../diagrams/tenant-access-control.mmd"
---

# Marketplace Runtime Capabilities Permission Group

## Included Permissions

`content.read`, `content.write`, `page.read`, `page.write`, `media.read`, `media.write`, `webhook.send`, `settings.read`, and `external_network.request`.

## Resource Domain and Operations

The catalog maps permissions to bounded host operations such as content/page/media access, component rendering, integration invocation, settings read, webhook send, and allowlisted external-network request. Product types and risk levels further constrain catalog entries.

## Scope and Roles

Permissions belong to a specific organization installation, not directly to a user role. Organization owner/admin must approve the complete requested snapshot at installation and on permission-expanding updates. Runtime calls also require an active tenant user and eligible installation.

## Backend Enforcement

Manifest validation restricts requested keys. Installation stores the approved snapshot and approval actor/time. Runtime authorization verifies permission-to-operation mapping, safe entry point/payload, installation/runtime state, and global/organization kill switches. It returns an authorization decision; it does not execute arbitrary package code in the observed implementation.

## Frontend Checks and API

Marketplace UI displays catalog risks and requires every requested permission checkbox before enabling install/update. Backend approval validation is authoritative. See the Marketplace runtime security and installation endpoint families.

## Database Implications

The permission catalog is global; approved snapshots and runtime state are installation-scoped; kill switches support global and organization scope with RLS rules for organization mutations.

## Tests and Unclear Semantics

Manifest, installation, runtime, and static security tests cover selected mappings and gates. `PERMISSION_SEMANTICS_UNCLEAR PSU-01` warns that these dot-delimited keys do not map automatically to CMS RBAC roles or legacy permission arrays.
