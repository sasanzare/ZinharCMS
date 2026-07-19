---
okf_document_id: "domain-background-processes"
title: "Background Processes"
project: "ZinharCMS"
category: "domain"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/services/webhooks.rs"
  - "backend/src/services/media_processing.rs"
  - "backend/src/routes/pages.rs"
  - "backend/src/services/email.rs"
related_documents:
  - "domain-events.md"
  - "cross-module-workflows.md"
  - "domain-risks.md"
related_diagrams:
  - "diagrams/cross-module-orchestration.mmd"
---

# Background Processes

No durable queue, queue consumer, scheduled job runner, worker deployment, publication scheduler, search indexer, retry scheduler, or general background-job status model was found.

## Observed Asynchronous or Deferred Work

| ID | Trigger/input/owner | Entities and tenant scope | Retry/idempotency | Completion/failure | Tests/confidence |
| --- | --- | --- | --- | --- | --- |
| `PROC-WEBHOOK-001` | Entry/page publication event; webhook service | Active webhook subscriptions and delivery rows; tenant-scoped | No retry or durable idempotency key | `tokio::spawn` sends HTTP and records delivered/failed; process loss can drop work | URL/signature tests; Medium guarantee |
| `PROC-MEDIA-001` | Image upload bytes; media processing service | Files and media variants; tenant directory | No retry/idempotency contract | `spawn_blocking` completes before upload response; failure aborts request after earlier file/write steps may exist | No processing tests; High behavior |
| `PROC-PREVIEW-001` | Page write/transition; pages route | In-memory page channel, process-local | No replay; broadcast is best effort | Sends serialized JSON when a channel exists; send/serialization absence is ignored | No integration tests; High behavior |
| `PROC-EMAIL-001` | Invitation/billing/operations calls; email service | Email delivery records, optionally tenant-scoped | No durable retry worker found | Provider/config result is recorded as sent/failed/skipped in request flow | No live provider tests; Medium |

## Processes Not Found

- Scheduled publication or unpublication.
- Background media optimization beyond synchronous request-owned processing.
- Search indexing.
- Import/export queue.
- Automatic Marketplace update/synchronization.
- Automated creator payout transfer.
- Audit-log post-processing or export.
- Webhook/email retry and dead-letter handling.
- Orphaned media file cleanup.
- Refresh-token expiry cleanup.

These absences are reported as `PLANNED_NOT_IMPLEMENTED` only where current project documentation explicitly defers the capability; otherwise they are simply not found.

## Operational Consequences

The web process owns all observed async work. Process restarts, request cancellation, or network failures can separate committed business state from side effects. `CROSS_MODULE_ORCHESTRATION_UNCLEAR` applies to exactly when operators should consider these workflows complete.

## Extensibility Background Work

No independent plugin worker, package executor, scheduled plugin runtime, or durable hook queue was found. CMS callbacks are synchronous and Marketplace adapter operations are request-driven host actions. See [Plugin Architecture](../extensibility/plugin-architecture.md).

## Operational Lifecycle

Spawned CMS webhooks and in-memory page-preview broadcasts remain inside the web process. Graceful server shutdown is configured, but no drain, replay, durable outbox, retry supervisor, or independent health/metrics surface exists for this work. Process restart can lose in-flight effects. See [Service Lifecycle](../operations/service-lifecycle.md), [Alerts and Incident Signals](../operations/alerts-and-incident-signals.md), and [Operational Risks](../operations/operational-risks.md).
