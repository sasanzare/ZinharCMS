# V3 Phase 7 Marketplace Security Runtime

Phase 7 adds the security boundary between the Phase 6 installation registry
and any future component or integration runtime. It is deliberately a policy
and containment phase: uploaded Marketplace code is not executed by the API.

## 7.1 Permission Model

Migration `0020_v3_phase_seven_permission_sandbox_kill_switch.sql` seeds an
explicit permission catalog for `content.read`, `content.write`, `page.read`,
`page.write`, `media.read`, `media.write`, `webhook.send`, `settings.read`, and
`external_network.request`. Each row declares category, risk level, supported
product types, and allowlisted runtime operations.

Phase 6 install approval remains the immutable organization owner/admin snapshot.
Phase 7 adds the host-runtime contract that must check that snapshot before any
future product action.

## 7.2 Sandbox MVP

The sandbox MVP is an allowlisted host API policy service, not an arbitrary code
runner. `marketplace_runtime.rs` enforces:

- active installation and `runtime_status = ready`;
- known runtime operation and compatible product type;
- declared manifest entry point with an artifact-local safe path;
- approved permission for the requested operation;
- JSON object payloads bounded to 64 KiB;
- no direct URL, path traversal, database, or arbitrary server-side execution.

`POST /api/marketplace/installations/{installation_id}/runtime/authorize` returns
an auditable allow/deny decision and always reports `execution = not_executed`.
Component rendering is the first allowlisted operation for Component Packs and
Design Templates. Integration operations remain policy-gated until later host
adapters provide concrete runtime behavior.

## 7.3 Kill Switch

`marketplace_kill_switches` supports active global and organization scopes. An
organization owner/admin can activate or lift an organization switch. A global
`admin`/`super_admin` can activate or lift the global switch.

Activation marks non-uninstalled installations as `runtime_status = blocked`,
retains the reason and timestamp, and records an organization-scoped audit event.
Runtime authorization, new installation, and re-enable gates refuse to proceed
while a relevant switch is active. Lifting an organization switch restores ready
runtime state only when no global switch remains; lifting the global switch keeps
organizations with their own active switch blocked.

## API

- `GET /api/marketplace/permissions`
- `GET /api/marketplace/runtime/status`
- `POST /api/marketplace/installations/{installation_id}/runtime/authorize`
- `POST /api/marketplace/kill-switches/organization`
- `POST /api/marketplace/kill-switches/global`
- `POST /api/marketplace/kill-switches/{kill_switch_id}/lift`

All routes are tenant-aware. Global kill-switch mutations additionally require a
global `admin` or `super_admin` claim and use a narrowly scoped bypass transaction
for cross-organization state changes. Organization mutations use forced-RLS
tenant transactions.

## Acceptance

- [x] Permissions are cataloged with risk/category/product/runtime metadata.
- [x] Runtime operations are allowlisted and permission-bound.
- [x] Unsafe entry points, oversized payloads, inactive installations, and blocked runtimes are denied.
- [x] Runtime policy never executes uploaded package code.
- [x] Global and organization kill switches block new install/re-enable/runtime authorization.
- [x] Kill-switch activation/lift and runtime decisions are auditable.
- [x] Phase 6 paid products, backend extensions, purchases, payouts, and arbitrary runtime execution remain outside this phase.
