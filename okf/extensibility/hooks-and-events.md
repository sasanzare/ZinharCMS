---
okf_document_id: "hooks-and-events"
title: "Hooks and Events"
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
  - "backend/src/plugins/mod.rs"
  - "backend/src/routes/content.rs"
  - "backend/src/services/marketplace_adapters.rs"
  - "backend/src/routes/marketplace_adapters.rs"
  - "backend/src/services/audit.rs"
  - "backend/src/services/webhooks.rs"
related_documents:
  - "../domain/domain-events.md"
  - "../domain/background-processes.md"
  - "extension-points.md"
related_diagrams:
  - "diagrams/plugin-registration-flow.mmd"
---

# Hooks and Events

| Mechanism | Delivery model | Extensibility status |
|---|---|---|
| CMS entry hooks | Synchronous in-process callback; error propagates | VERIFIED executable built-in hook |
| Marketplace public hooks | Manifest definition listed/authorized by host | PARTIALLY_IMPLEMENTED; no package execution |
| Audit records | Transactional or adjacent persistence of internal events | Internal evidence, not a plugin hook |
| CMS webhooks | Host-managed outbound delivery | Public integration, separate from CmsPlugin |
| Spawned tasks | In-process background work | Internal orchestration, not extension registration |
| WebSocket preview | Host transport for Page Builder | Internal UI integration |

The four allowlisted Marketplace public-hook types are sidebar.item, dashboard.widget, form.field, and webhook.adapter. Authorization returns execution: not_executed.

HOOK_DELIVERY_UNCLEAR applies to Marketplace hook rendering or invocation after authorization. Audit actions and webhook events must not be treated as CmsPlugin lifecycle events merely because names resemble events.

## Hook Contract Matrix

| ID | Name | Producer | Trigger | Payload | Registration | Consumers | Ordering | Sync/async | Failure | Retry | Tenant context | Permissions | Transaction relationship | Tests | Confidence |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| HOOK-CMS-01 | entry.before_save | Content create/update | Before entry persistence | Mutable EntryData, PluginContext | Static CmsPlugin | Enabled compiled plugins; SEO Auto | builtin_plugins() vector order | Synchronous | Error aborts request | None | user_id/org_id passed | No callback capability; global enable management | Before host persistence | SEO unit tests | High |
| HOOK-CMS-02 | entry.after_publish | Content publish | After publication orchestration point | Immutable EntryData, PluginContext | Static CmsPlugin | No verified subscriber | Vector order | Synchronous | Error propagates | None | user_id/org_id passed | No callback capability | Exact commit/side-effect ordering needs integration proof | No subscriber test | Medium |
| HOOK-MP-01 | sidebar.item | Active integration manifest | Host list/authorize request | Key, label, config, context | Manifest declaration | No frontend renderer found | No delivery order | Authorization request only | Denied result | None | Tenant installation | Runtime snapshot/capability | No package callback or transaction | Adapter extraction tests | Medium |
| HOOK-MP-02 | dashboard.widget | Active integration manifest | Host list/authorize request | Same declaration shape | Manifest declaration | No renderer found | None | Authorization only | Denied result | None | Tenant | Runtime policy | No execution | Adapter tests | Medium |
| HOOK-MP-03 | form.field | Active integration manifest | Host list/authorize request | Same declaration shape | Manifest declaration | No renderer found | None | Authorization only | Denied result | None | Tenant | Runtime policy | No execution | Adapter tests | Medium |
| HOOK-MP-04 | webhook.adapter | Active integration manifest | Host list/authorize request | Same declaration shape | Manifest declaration | No executor found | None | Authorization only | Denied result | None | Tenant | Runtime policy including webhook.send where mapped | No outbound delivery established | Adapter tests | Medium |

## Non-Extension Event Classification

| Mechanism | Classification | Reason |
|---|---|---|
| Audit actions | Audit events | Persist evidence; no subscriber registration |
| CMS webhooks | Supported outbound integration | Host-managed subscriptions/delivery, separate from CmsPlugin |
| Spawned Tokio work | Internal background orchestration | No extension registration or durable queue |
| Preview WebSocket | Frontend transport event | Host UI protocol, not plugin event bus |
| Ordinary callbacks/service calls | Internal functions | Static implementation details |

No general queue-message, frontend-event, or event-subscriber extension registry was found.
