---
okf_document_id: "plugin-isolation-trust"
title: "Isolation and Trust"
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
  - "backend/src/plugins/mod.rs"
  - "backend/src/services/marketplace_validation.rs"
  - "backend/src/services/marketplace_runtime.rs"
  - "backend/src/routes/marketplace_runtime.rs"
related_documents:
  - "../security/trust-boundaries.md"
  - "../security/threat-register.md"
  - "plugin-permissions.md"
related_diagrams:
  - "diagrams/plugin-permission-flow.mmd"
  - "diagrams/extensibility-context.mmd"
---

# Isolation and Trust

## Built-in Plugins

CmsPlugin code executes in the backend process with host memory and process authority. There is no process, container, WASM, interpreter, OS-user, resource-quota, timeout, or capability isolation around callbacks. It must be treated as trusted application code. PLUGIN_ISOLATION_UNVERIFIED would be misleading as an implementation claim: current source shows no isolation boundary.

## Marketplace Packages

Upload validation checks manifest/package structure, paths, checksums, declared assets/entry points, and security findings. Install stores approval snapshots. Runtime policy checks safe entry points, payload size, kill switches, product type, and permissions.

However, current source does not execute uploaded package code. The sandbox policy name and authorization endpoint are policy scaffolding, not evidence of an execution sandbox. Runtime decisions explicitly return not_executed. Host adapters own template import, component metadata synchronization, and hook authorization.

## Trust Conclusions

- Built-ins: trusted and in-process.
- Marketplace artifacts: untrusted input validated and stored.
- Declarative adapter data: parsed by trusted host code under tenant/RBAC/RLS checks.
- Future executable Marketplace plugins: PLANNED_NOT_IMPLEMENTED.
- Deployed artifact storage hardening, malware tooling, signature chain, and runtime containment remain PLUGIN_TRUST_MODEL_UNCLEAR or UNKNOWN.

## Isolation Classification

| Mechanism | Classification | Consequence |
|---|---|---|
| Compiled CmsPlugin | NO_RUNTIME_ISOLATION | Can execute with backend process authority and affect request latency/failure. |
| Marketplace artifact | PARTIALLY_ISOLATED as inert stored input | It is path/checksum/manifest validated but not executed. |
| Marketplace host adapter | Host trust boundary, not plugin sandbox | Trusted host code parses declaration and performs constrained actions. |
| Frontend Marketplace declaration | FRONTEND_BOUNDARY_ONLY for data rendering | No package JavaScript or DOM access is granted. |
| Future arbitrary package executor | PLUGIN_ISOLATION_UNVERIFIED and PLANNED_NOT_IMPLEMENTED | No safety claim is valid. |

## Trust Capability Matrix

| Concern | Built-in plugin | Marketplace artifact/declaration |
|---|---|---|
| Process/WASM/container isolation | None | No executor; no runtime isolation implementation |
| Filesystem access | Process authority in principle; current SEO does not use it | Artifact verification only; future access not granted |
| Network access | Process authority in principle | external_network.request is policy-only and requires approval |
| Database/application-state access | Callback receives shared data/context; host process can access state | Host adapters mediate tenant queries/writes |
| Secrets | No capability boundary prevents trusted code access in principle | No secret manifest/config contract; package code not run |
| Tenant data | Context passed; global enablement | Tenant middleware/RBAC/RLS on host routes |
| Frontend DOM/browser storage | No frontend plugin code | No package JavaScript loader |
| Resource limits/timeouts/crash isolation | None around callback | 64-KiB authorization payload cap; no executor resources |
| Error boundary | Callback Result propagates | Host validation/HTTP error; no third-party crash |
| Signature/integrity | Trusted build chain outside Phase 9 | SHA-256/size/path checks; no package signature field |
| Review | Code review/build process inferred | Automated validation and human Marketplace review |

Before any executable runtime, require an owner-approved threat model covering signatures, provenance, least privilege, process/WASM isolation, filesystem/network/database mediation, quotas, timeouts, crash containment, audit, revocation, and incident response.
