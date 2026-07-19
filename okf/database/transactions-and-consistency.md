---
okf_document_id: "database-transactions-consistency"
title: "Database Transactions and Consistency"
project: "ZinharCMS"
category: "database"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources: ["backend/src/routes", "backend/src/services", "backend/src/middleware/tenant.rs"]
related_documents: ["database/multi-tenancy.md", "database/lifecycle-and-auditing.md", "database/database-risks.md"]
related_diagrams: ["database/diagrams/tenant-isolation.mmd"]
uncertainty_markers: ["TRANSACTION_BOUNDARY_UNCLEAR TBU-01", "TRANSACTION_BOUNDARY_UNCLEAR TBU-02", "TRANSACTION_BOUNDARY_UNCLEAR TBU-03", "TRANSACTION_BOUNDARY_UNCLEAR TBU-04", "NEEDS_OWNER_CONFIRMATION NOC-09"]
---

# Database Transactions and Consistency

## Transaction Model

SQLx transactions are opened explicitly for multi-statement workflows. Tenant helpers set transaction-local PostgreSQL context; bypass transactions support privileged global/catalog/provider operations. No application-wide isolation level override was found, so a particular deployed isolation level must not be asserted beyond PostgreSQL/connection defaults.

Verified transaction entry points include startup seeding, registration, organization lifecycle operations, page create/update/restore, beta operations, Marketplace catalog/review/installation/runtime/finance workflows, Stripe webhook processing, and RLS context helpers.

## Significant Transaction Flow Matrix

| Flow and entry point | Tables/entities | Start and main operations | Commit condition / rollback | External side effects | Risk and evidence |
| --- | --- | --- | --- | --- | --- |
| Page create/update/restore; pages routes | pages, page versions | Begin tenant transaction; mutate page and insert snapshot | Commit after both writes; SQLx error/drop rolls back | Broadcast and delivery invalidation after commit | Database pair is atomic; later side effects are not. Pages route source |
| Marketplace install/update/rollback; installation service/routes | installations, audit logs | Verify artifact, begin tenant transaction, mutate lifecycle and write audit | Commit after database writes; SQLx error rolls back | Artifact/filesystem verification can precede transaction | Files are outside DB atomicity. Installation source |
| Critical abuse report; feedback service | abuse reports, audit logs, internal notifications | Begin organization transaction; insert report/audit/notification/audit | Commit only after all four writes; error rolls back | External critical delivery not established | Internal records atomic; external escalation `PNI-04`. Feedback source |
| Free Marketplace checkout; finance service | purchases, entitlements, revenue ledger, audit | Begin tenant transaction; create completed free purchase, grant, post ledger, audit | Commit after all records; SQLx error rolls back | None required for payment | Atomic database flow. Marketplace finance source |
| Paid checkout; finance route/service | purchases | Begin and commit pending purchase before provider request | Purchase commit precedes provider; later failure update records failure | Stripe checkout creation | Distributed boundary `TBU-03`; retry/reconciliation needed |
| Stripe webhook; Stripe billing service | billing events, purchases, entitlements, ledger, audit | Begin bypass transaction; insert idempotent event, lock purchase `FOR UPDATE`, apply finance changes | Success commits processed state; processing error records failed event and commits that state | Provider webhook acknowledgement follows | Concurrency/idempotency sensitive. Stripe billing source |
| Organization creation; organizations route | organizations, members, subscriptions, audit | Begin transaction for tenant/membership/subscription | Domain transaction commits together; audit uses a later transaction | Email/other follow-up can be separate | Audit gap `TBU-02`. Organizations route source |
| Media upload/variants; media route/processing | media, media variants plus files | File writes and row inserts occur as a multi-resource sequence | No single transaction spans all filesystem/DB work | Filesystem writes and image processing | Partial/orphan risk `TBU-01`. Media source |
| CMS webhook dispatch; webhook service | webhooks, webhook deliveries | Select subscription; spawn task; send; persist attempt | No transaction/outbox spans event and delivery | Outbound HTTP request | Process-loss/retry risk `TBU-04`. Webhook service source |

Other explicit transactions appear in registration; domain/rate-limit/invitation/ownership operations; beta participant, feedback, and blocker flows; Marketplace catalog, submission, review, moderation, analytics, template import, policy, payout, balance, and kill-switch flows; and tenant/bypass helpers. Exact start/commit points remain in the referenced route/service code.

Finance uses `FOR UPDATE` for concurrency-sensitive purchase processing. Provider/event uniqueness and receipt constraints provide idempotency layers. When Stripe-event processing fails, the code records and commits failed event state before returning an error; retry behavior must account for that state.

## Cross-Boundary Consistency

Paid checkout persists a pending purchase before the external Stripe call, then marks failure if provider creation fails. This is an intentional distributed boundary but has no database-only atomic guarantee (`TBU-03`). General compensation ownership remains `NEEDS_OWNER_CONFIRMATION NOC-09`.

Organization creation commits organization, membership, and subscription state before a separate audit transaction; an audit failure can leave the domain write committed (`TBU-02`). Media writes cross filesystem and database operations, including derived files/rows, without one atomic resource transaction (`TBU-01`). CMS webhook delivery is spawned in-process and its attempt is recorded after dispatch; there is no durable queue/outbox guarantee (`TBU-04`).

## Concurrency and Versioning

Page snapshots preserve revision history within page-write transactions. `content_entries.version` is a stored data version, but no platform-wide optimistic-lock contract was found. Marketplace version artifacts become immutable in specified lifecycle states through a trigger. Revenue ledger rows reject updates/deletes through a trigger. Database constraints, row locks, RLS, and explicit transactions collectively enforce consistency; application validation alone is not the full contract.

## Failure Guidance

Future changes must classify each side effect as database, filesystem, cache, process-local broadcast, webhook/email, or payment-provider work. Define commit order, idempotency key, retry owner, observable failure state, and compensation before altering a cross-boundary flow. Do not move side effects into a transaction without considering lock duration and external failure.

## Phase 8 Workflow Transactions

[Cross-Module Workflows](../domain/cross-module-workflows.md) identifies atomic database sections and post-commit effects for provisioning, membership, ownership transfer, entry/page publication, page snapshots, media processing, webhooks, billing callbacks, and Marketplace commerce. Media files and database rows are not one transaction; webhook dispatch is process-local; provider and database work relies on idempotency and ordering rather than distributed atomicity. See [Domain Risks](../domain/domain-risks.md).
