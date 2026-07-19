---
okf_document_id: "workflow-publication-webhook-delivery"
title: "Publication Webhook Delivery Workflow"
project: "ZinharCMS"
category: "domain-workflow"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
workflow_id: "WF-WEBHOOK"
workflow_name: "Publication Webhook Delivery"
workflow_domain: "DOM-DELIVERY"
implementation_status: "IMPLEMENTED"
primary_sources:
  - "backend/src/services/webhooks.rs"
  - "backend/src/routes/webhooks.rs"
  - "backend/src/routes/content.rs"
  - "backend/src/routes/pages.rs"
related_documents:
  - "../cross-module-workflows.md"
  - "../domain-events.md"
  - "../background-processes.md"
related_diagrams:
  - "../diagrams/publication-workflow.mmd"
---

# Publication Webhook Delivery Workflow

## Workflow Identity

- ID/name/domain: `WF-WEBHOOK`, Publication Webhook Delivery, `DOM-DELIVERY`.
- Trigger/actor: successful entry/page publish or unpublish by tenant publisher.
- Status/confidence: `IMPLEMENTED`; High request construction, Medium delivery guarantee.

## Preconditions

Committed publication transition, supported event, active tenant webhook whose event list contains it, valid stored URL/secret.

## Main Flow

1. Publication handler builds an event envelope containing tenant and entity data.
2. Webhook service opens organization-scoped DB connection.
3. Load active subscriptions matching event.
4. Spawn one in-process Tokio task per subscription.
5. Serialize payload and sign with HMAC-SHA256.
6. POST JSON with event, signature, organization, and content-type headers, with ten-second timeout.
7. Classify HTTP success as delivered; other HTTP/network outcomes as failed.
8. Bound response/error text and insert delivery-attempt row.

## Alternative Flows

Webhook manager can invoke a test delivery using the first subscribed event or fallback `page.publish`; this call awaits dispatch and returns failure.

## Failure Flows

Trigger cannot open/load DB: warning and no task. Spawned task/network/record failure: warning; publication remains committed. Process termination can lose unstarted/in-flight work. No retry.

## State Changes

Webhook remains active/inactive. Each completed attempt records terminal `delivered` or `failed`.

## Data Changes

One `webhook_deliveries` row per completed dispatch attempt.

## Transaction Boundaries

Publication state, task creation, external HTTP, and delivery logging are separate. No outbox transaction.

## Side Effects

Outbound HTTP and warning logs.

## Completion Criteria

For async publication trigger, handler completion does not wait for delivery. For test endpoint, dispatch and attempt recording must complete.

## Tests

Safe URL, event allowlist, and stable signature tests. No real HTTP, timeout, process-loss, tenant, ordering, retry, or delivery-row integration test.

## Unknowns and Risks

At-most-once-like process behavior, no retry/idempotency key/order guarantee, DNS resolution policy, and payload privacy/retention.

Return to [Cross-Module Workflows](../cross-module-workflows.md).

