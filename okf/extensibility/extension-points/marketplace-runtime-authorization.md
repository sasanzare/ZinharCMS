---
okf_document_id: "extension-point-marketplace-runtime-authorization"
title: "Marketplace Runtime Authorization Extension Point"
project: "ZinharCMS"
category: "extensibility"
phase: 9
status: "current"
source_of_truth: false
implementation_view: "observed"
extensibility_status: "partially_verified"
last_verified_commit: "56d733985fdd7aa3f25ee6981b88cf29c52f65c9"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/services/marketplace_runtime.rs"
  - "backend/src/routes/marketplace_runtime.rs"
  - "backend/migrations/0020_v3_phase_seven_permission_sandbox_kill_switch.sql"
extension_point_id: "EP-006"
extension_point_name: "Marketplace Runtime Authorization"
extension_point_category: "host_capability_policy"
registration_type: "static_operation_catalog_and_manifest"
implementation_status: "policy_implemented_execution_not_implemented"
related_documents:
  - "../extension-points.md"
  - "../plugin-permissions.md"
  - "../isolation-and-trust.md"
  - "../../security/permissions/marketplace-runtime-capabilities.md"
related_diagrams:
  - "../diagrams/plugin-permission-flow.mmd"
---

# Marketplace Runtime Authorization Extension Point

## Identity

EP-006 is the policy boundary for a fixed catalog of host operations and nine manifest permissions.

| Identity field | Value |
|---|---|
| Extension-point ID | EP-006 |
| Name | Marketplace Runtime Authorization |
| Category | Host capability policy |
| Source paths | marketplace_runtime route/service; migration 0020 |
| Implementation status | POLICY_IMPLEMENTED_EXECUTION_NOT_IMPLEMENTED |
| Confidence | High |

## Purpose

Decide whether an active Marketplace installation may request a declared host capability.

## Contract

The request identifies an installation, operation, entry point, and JSON payload. Policy maps the operation to required permission, allowed product types, and manifest entry-point key. Payload JSON is capped at 64 KiB.

## Registration

Operation definitions are compiled into marketplace_runtime.rs. Installation permissions and entry points come from the approved manifest/snapshot.

Host developers register unique static operation definitions; creators declare matching entry-point keys and permissions; tenant approvers grant exact snapshots. Registration is mixed static policy plus dynamic metadata. No dependency injection or arbitrary operation naming exists, and each request evaluates one operation without an ordering contract.

## Execution

Authorization validates state, runtime readiness, operation, product type, safe artifact-local path, exact approved permission, and payload size. Response execution is not_executed. No operation runner is called.

The request is synchronous and shares host/tenant database context. Denial returns a bounded reason. No retry, third-party side effect, shared package memory, or isolation boundary is involved because execution is absent.

## Security

Global/organization kill switches and blocked installation runtime state can deny the request. The policy is a control boundary, not a sandbox.

## Compatibility

The sandbox-policy/adapter contract version is 2026-07. No external runner protocol exists.

## Tests

Unit tests cover allowed component access, permission escalation, unknown operations, blocked state, unsafe paths, oversized payloads, and bounded reasons.

## Risks and Unknowns

PLUGIN_ISOLATION_UNVERIFIED and PLUGIN_TRUST_MODEL_UNCLEAR apply to any future executor. See [Extension Points](../extension-points.md).
