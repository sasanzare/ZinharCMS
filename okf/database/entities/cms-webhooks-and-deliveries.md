---
okf_document_id: "database-entity-cms-webhooks-deliveries"
title: "CMS Webhooks and Deliveries"
project: "ZinharCMS"
category: "database-entity"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
entity_id: "DB-ENT-009"
entity_name: "CMS Webhooks and Deliveries"
entity_domain: "Outbound Delivery"
schema_objects: ["webhooks", "webhook_deliveries"]
owning_module: "Delivery and Webhooks"
tenant_scope: "tenant"
implementation_status: "IMPLEMENTED"
confidence: "high"
primary_sources: ["backend/migrations/0005_phase_five_delivery_api.sql", "backend/migrations/0008_v2_phase_one_organizations.sql", "backend/src/routes/delivery.rs", "backend/src/services/webhooks.rs"]
related_documents: ["database/transactions-and-consistency.md", "database/multi-tenancy.md", "backend/modules/cms-webhooks.md"]
related_diagrams: ["database/diagrams/entity-relationship-overview.mmd"]
uncertainty_markers: ["TRANSACTION_BOUNDARY_UNCLEAR TBU-04", "DATA_LIFECYCLE_UNCLEAR DLU-01"]
---

# CMS Webhooks and Deliveries

## Entity Identity

`DB-ENT-009` represents tenant webhook subscriptions and outbound delivery-attempt records.

## Purpose

Verified purpose: configure event destinations/secrets and record dispatch attempts/results. Durable queue semantics are not implemented or inferred.

## Storage Structure

| Field group | Type/null/default/constraint | Meaning and evidence |
| --- | --- | --- |
| webhook ID/organization | UUID/FKs | Tenant subscription |
| URL, secret, events, active | Text/array-or-JSON/boolean | Destination and event selection |
| delivery ID/webhook/organization | UUID/FKs | Attempt identity and tenant |
| event/payload/status/response | Text/JSONB/text | Delivery request/result |
| attempt/timing fields | Numeric/time | Delivery history |

## Identifiers

Both tables use UUID PKs. Webhook URL is not treated as a globally unique identity. Delivery IDs identify attempts.

## Relationships

Webhooks belong to organizations and have many deliveries. A trigger derives delivery organization context. Content/pages can produce events logically.

## Ownership

Delivery/webhook services own configuration and attempts; domain routes produce events.

## Tenant Isolation

Both tables are forced RLS. Secrets/payloads require tenant context and privileged operational handling.

## Lifecycle

Subscriptions can be activated/deactivated/deleted; deliveries form history. Retention and retry ceilings are not governed.

## Constraints and Indexes

Organization/webhook FKs, status/check fields, and event/time lookup indexes support subscription and history queries.

## Persistence Mapping

Direct SQLx is combined with an in-process spawned dispatch path. Attempt persistence occurs around external delivery rather than through a durable outbox.

## Security and Privacy

Secrets, URLs, payloads, response bodies, and headers may be sensitive. Documentation must never include actual secret values.

## Known Risks and Unknowns

Process loss can interrupt spawned work (`TBU-04`); retry, retention, redaction, and destination-governance policies are unclear.

## Related Documents

See [Transactions and Consistency](../transactions-and-consistency.md), [Lifecycle and Auditing](../lifecycle-and-auditing.md), and [Database Risks](../database-risks.md).

Return to the [Entity Catalog](../entity-catalog.md) or continue with the owning [CMS Webhooks module](../../backend/modules/cms-webhooks.md).
