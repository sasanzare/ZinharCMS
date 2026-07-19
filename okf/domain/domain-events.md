---
okf_document_id: "domain-events"
title: "Domain and Application Events"
project: "ZinharCMS"
category: "domain"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/services/webhooks.rs"
  - "backend/src/services/audit.rs"
  - "backend/src/plugins"
  - "backend/src/routes/pages.rs"
  - "backend/src/services/stripe_billing.rs"
related_documents:
  - "background-processes.md"
  - "publication-workflow.md"
  - "../security/audit-and-security-events.md"
related_diagrams:
  - "diagrams/cross-module-orchestration.mmd"
---

# Domain and Application Events

ZinharCMS has feature-specific event-like mechanisms, not a general domain event bus. Ordinary function calls are not labeled as events unless they implement an explicit hook, callback, broadcast, audit record, or webhook contract.

| ID | Event / producer / trigger | Payload and consumers | Transaction/delivery/retry/idempotency | Tenant/tests/confidence |
| --- | --- | --- | --- | --- |
| `EVT-CMS-001` | `entry.publish`; content route after publish | Entry envelope; matching active outbound webhooks | After state write; spawned HTTP; no retry/idempotency | Tenant; webhook tests only; High contract/Medium delivery |
| `EVT-CMS-002` | `entry.unpublish`; content route | Entry envelope; outbound webhooks | Same as above | Tenant; High/Medium |
| `EVT-CMS-003` | `page.publish`; pages route | Page envelope; outbound webhooks | Same as above | Tenant; High/Medium |
| `EVT-CMS-004` | `page.unpublish`; pages route | Page envelope; outbound webhooks | Same as above | Tenant; High/Medium |
| `EVT-PLUGIN-001` | `entry.before_save`; content create/update | Mutable `EntryData`; enabled compiled plugins | Synchronous before validation/write; failure aborts save; no retry | Context includes tenant/user; SEO tests; High |
| `EVT-PLUGIN-002` | `entry.after_publish`; content publish | Read-only entry data; enabled compiled plugins | Synchronous after committed state; failure returns error; no retry | Tenant/user context; no active plugin implementation/test for after-publish | Medium |
| `EVT-PREVIEW-001` | Page JSON broadcast; page write/transition | Serialized page JSON; in-process WebSocket subscribers | Process-local, best effort, no replay | Page ID channel; no integration tests; High behavior |
| `EVT-AUDIT-001` | Named audit actions from organization/content/page/media/billing/beta/Marketplace handlers | Actor, action, entity type/id, metadata; audit log readers | Some in same transaction, others after commit; no retry/tamper evidence | Usually tenant; no completeness suite; Medium |
| `EVT-STRIPE-001` | Signed Stripe webhook callback | Provider JSON event; billing/Marketplace finance consumers | Persisted unique provider event inside bypass transaction; idempotent; provider retry external | Mixed global/tenant; signature/order tests; High |
| `EVT-MARKET-REVIEW-001` | Marketplace administrative review decision | Previous/next status and messages; persisted review events | Transaction relationship varies by route/service; immutable history row | Global catalog with actor; static/service tests; Medium-high |
| `EVT-MARKET-CRITICAL-001` | Critical abuse report | Internal notification payload for admin role | Persisted internal record; external delivery not implemented | Mixed tenant/global; static/frontend tests; Medium |

## Audit Events Are Not Workflow Messages

Audit records describe selected completed actions but are not consumed to drive state transitions. They are not an outbox, retry queue, or source of truth for aggregate reconstruction. Coverage is incomplete, especially authentication denials and CMS publication.

## Webhook Delivery Contract

Each outbound webhook uses `X-CMS-Event`, `X-CMS-Signature`, `X-Organization-Id`, JSON content, and a ten-second request timeout. A delivery record stores success/failure status and bounded response/error text. No retry, ordering, exactly-once, or durable-at-least-once guarantee exists.

## Event Unknowns

- `CROSS_MODULE_ORCHESTRATION_UNCLEAR`: audit and side-effect transaction relationships vary.
- `WORKFLOW_UNCLEAR`: no contract states whether consumers may depend on webhook order.
- `PLANNED_NOT_IMPLEMENTED`: external delivery for critical Marketplace notifications and arbitrary plugin execution are absent.

