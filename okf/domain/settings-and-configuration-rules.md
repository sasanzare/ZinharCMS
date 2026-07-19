---
okf_document_id: "domain-settings-configuration-rules"
title: "Settings and Configuration Rules"
project: "ZinharCMS"
category: "domain"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/routes/organizations.rs"
  - "backend/src/routes/delivery.rs"
  - "backend/src/routes/webhooks.rs"
  - "backend/src/routes/plugins.rs"
  - "frontend/src/pages/SettingsPage.tsx"
related_documents:
  - "domains/delivery-settings-and-webhooks.md"
  - "../backend/configuration-and-state.md"
  - "../frontend/features/settings-and-webhooks.md"
related_diagrams: []
---

# Settings and Configuration Rules

Infrastructure configuration is documented separately. This file records product-visible or domain-affecting settings only.

| Setting group | Scope/storage/default | Override order and validation | Allowed actor | Runtime effects | Tests/confidence |
| --- | --- | --- | --- | --- | --- |
| Organization settings | Tenant; `organizations.settings`; `{}` | Submitted object replaces stored object; object-only check; no typed keys | Organization admin/owner | Available on organization detail; feature semantics for arbitrary keys not centralized | No tests; High storage, Low key meaning |
| Public settings | Tenant; `public_settings` key/value | Key regex; public delivery folds rows into JSON; database/seed values only in inspected management surface | Current edit actor/API not found | Public settings endpoint, site URL for sitemap/robots | No management tests; `WORKFLOW_UNCLEAR` |
| Navigation | Tenant; `navigation_items`; ordered/parented rows | Nonempty label/URL, locale regex, parent FK | Current edit actor/API not found | Public navigation by locale | No management tests; `WORKFLOW_UNCLEAR` |
| Organization rate limits | Tenant; one row per organization; migration defaults | Positive request/user limits and nonnegative burst; DB and route validation | Organization admin/owner | Tenant middleware Redis limits | Positive-value unit check only; High |
| Webhook subscriptions | Tenant; webhook row; active default true | Name/URL/event/secret validation; submitted `is_active` defaults true on create/update | Webhook manager | Determines publication delivery targets | Unit validators and UI flows; High |
| CMS plugin enabled state | Global; `cms_plugins.is_enabled`; built-ins true | Explicit enable/disable; built-in sync refreshes metadata but does not force enabled on conflict | Global plugin manager | Gates synchronous content plugin hooks across all tenants | SEO tests; scope intent unclear |
| Component definitions | System/global or tenant; component registry | System rows readable; tenant rows keyed by component slug and prop schema | Component manager; system writes protected | Page JSON validation and frontend palette | No route tests; High |
| Marketplace installation permissions | Tenant installation/version | Exact approval then reapproval on permission change | Tenant Marketplace manager | Runtime authorization and host adapters | Strong service/frontend tests; High |
| Marketplace kill switch | Tenant installation runtime status | `ready`/`blocked` with bounded reason | Tenant admin/manager | Blocks runtime operation authorization | Runtime tests; High |
| Frontend language/direction | Browser/frontend state | Supported locale catalog; no backend user-preference persistence found | Current browser user | UI messages and document direction | Frontend behavior not fully tested; `FRONTEND_ONLY_RULE` |

## Defaults and Inheritance

No general setting inheritance engine exists. Defaults come from database column defaults, seed rows, backend configuration, or frontend initial state. Organization settings do not automatically inherit from a global product-settings row. Plan features/limits are a separate billing policy, not organization settings inheritance.

## Visibility and Caching

- Public settings/navigation are unauthenticated delivery data and can be Redis-cached.
- Organization settings, rate limits, webhooks, plugin state, and Marketplace runtime controls are protected management data.
- The Settings frontend page manages profile refresh and webhooks and displays selected environment values; it is not a complete product settings console.

## Reset Behavior

No universal reset-to-default endpoint exists. Replacing organization settings with `{}` is possible through current update behavior. Webhooks and tenant components are deleted rather than reset. Plugin and kill-switch state can be toggled back. Public settings/navigation reset behavior is `WORKFLOW_UNCLEAR`.

