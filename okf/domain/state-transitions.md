---
okf_document_id: "domain-state-transitions"
title: "State Transitions"
project: "ZinharCMS"
category: "domain"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/services/workflow.rs"
  - "backend/src/routes"
  - "backend/migrations"
related_documents:
  - "content-lifecycle.md"
  - "publication-workflow.md"
  - "business-rule-catalog.md"
related_diagrams:
  - "diagrams/content-lifecycle.mmd"
  - "diagrams/publication-workflow.mmd"
  - "diagrams/tenant-membership-workflow.mmd"
---

# State Transitions

## Editorial Transition Matrix

This matrix is enforced by `services/workflow.rs` for content entries and pages.

| From | To | Status | Trigger and authorization | Validation/side effects | Tests |
| --- | --- | --- | --- | --- | --- |
| `draft` | `pending_review` | `VERIFIED_ALLOWED` | Submit endpoint; entry/page writer | Increment entry version; page broadcast | `draft_can_submit_for_review` |
| `draft` | `published` | `VERIFIED_ALLOWED` only with bypass | Publish endpoint; publisher/reviewer capability | Timestamp, cache invalidation, webhook; entry plugin hook | `reviewer_can_publish_draft_directly` |
| `draft` | `archived` | `VERIFIED_REJECTED` | No handler path | Transition validator rejects | No direct test |
| `pending_review` | `published` | `VERIFIED_ALLOWED` | Publisher capability | Timestamp, cache/webhook/plugin effects | Transition path inferred from matrix; no route test |
| `pending_review` | `draft` | `VERIFIED_ALLOWED` | Reviewer reject | Clears publication timestamp | No direct unit test |
| `published` | `archived` | `VERIFIED_ALLOWED` | Reviewer/archive | Clears publication timestamp; cache invalidation | No direct unit test |
| `published` | `draft` | `VERIFIED_ALLOWED` | Publisher/unpublish | Clears timestamp, cache, unpublish webhook | No direct unit test |
| `archived` | `draft` | `VERIFIED_ALLOWED` | Reviewer/restore | Clears timestamp | No direct unit test |
| Any same state | Same state | `VERIFIED_REJECTED` | Transition helper | Validation error | No direct test |
| Any other pair | Any other pair | `VERIFIED_REJECTED` | Transition helper | Validation error | Partial unit coverage |

Content entry transitions increment `version`. Page transitions do not create a `page_versions` snapshot; they broadcast current JSON. Direct SQL updates can select any database enum value and do not invoke the application state machine.

## Other Stateful Entities

| Entity | State type / initial / allowed values | Verified transition triggers | Authorization and side effects | Status/confidence |
| --- | --- | --- | --- | --- |
| Organization | enum; initial `active`; `active`, `suspended`, `deleted` | Current tenant update only touches active rows; no complete status mutation route found | Active status required by tenant middleware | `STATE_TRANSITION_UNCLEAR`; High values, Low graph |
| Organization member | enum; create/accept `active`; `active`, `invited`, `suspended` | Invitation acceptance activates/upserts; role update does not change status; removal hard deletes | Owner/admin rules; audit after mutation | `STATE_TRANSITION_UNCLEAR`; Medium |
| Organization invitation | enum; `pending`; `accepted`, `revoked`, `expired` | Accept, revoke, and expiry update pending rows | Email on create; audit on create/accept/revoke | `VERIFIED_ALLOWED`; High |
| Organization domain | text; `pending`; `pending`, `verified`, `rejected` | Create inserts pending; status-management flow not found | Admin create/delete; audit in transaction | `STATE_TRANSITION_UNCLEAR`; Medium |
| Subscription | enum; `active`; `trialing`, `active`, `past_due`, `canceled`, `incomplete` | Manual plan change sets active; Stripe maps provider status; deleted provider subscription maps canceled | Billing role/provider signature; audit/email | `VERIFIED_ALLOWED` for mapped writes; Medium graph |
| Billing event | text; `processing`; `processed`, `failed`, `ignored` | Signed webhook processing transaction | Unique provider event; audit for processed/ignored | `VERIFIED_ALLOWED`; High |
| Email delivery | text; `pending`; `sent`, `failed`, `skipped` | Email service records provider/config outcome | Operational record, no retry queue | `VERIFIED_ALLOWED`; Medium |
| Beta participant | text; `candidate`; seven allowed values | Global admin upsert can assign any allowed value | Bypass transaction | `STATE_TRANSITION_UNCLEAR`; High values, Low graph |
| Beta feedback | text; `open`; `triaged`, `planned`, `fixed`, `closed` | Editor/admin update can assign any allowed value | Audit in transaction | `STATE_TRANSITION_UNCLEAR`; High values, Low graph |
| GA blocker | text; `open`; `in_progress`, `blocked`, `resolved`, `deferred` | Editor/admin update can assign any allowed value | Audit in transaction | `STATE_TRANSITION_UNCLEAR`; High values, Low graph |
| CMS plugin | boolean `is_enabled`; initial true | Enable/disable/update | Global plugin manager; affects synchronous hooks | `VERIFIED_ALLOWED`; High |
| Marketplace creator | text; default `pending` after migration 0016; `pending`, `approved`, `suspended`, `rejected` | Creator application/admin decision routes | Ownership/admin checks | `VERIFIED_ALLOWED` paths; Medium complete graph |
| Marketplace listing | text; `draft`; seven allowed values | Submission/review/moderation/archive paths | Creator owner/admin rules | `VERIFIED_ALLOWED` paths; Medium complete graph |
| Marketplace version | text; `draft`; seven allowed values plus validation status | Upload/submit/validate/review/deprecate/block | Artifact immutability trigger, validation/review events | `VERIFIED_ALLOWED` paths; High protected rules |
| Marketplace submission | text; `queued`; seven review values | Validation and review decisions | Global admin review; persisted reports/events | `VERIFIED_ALLOWED` paths; Medium complete graph |
| Marketplace installation | text; `active`; `disabled`, `uninstalled`, `rollback_pending`, `blocked` | Install/disable/enable/uninstall/update/rollback/policy block | Tenant role, permission approval, audit | `VERIFIED_ALLOWED` service tests; High |
| Marketplace runtime | text; `ready`; `blocked` | Organization kill switch and policy state | Tenant admin; authorization denies blocked | `VERIFIED_ALLOWED`; High |
| Marketplace purchase | text; `pending`; `completed`, `failed`, `refunded`, `canceled` | Checkout/provider callback/refund | Billing signature/transaction; entitlement/ledger | `VERIFIED_ALLOWED` selected paths; Medium full graph |
| Marketplace entitlement | text; `active`; `revoked`, `expired` | Completion grants; refund revokes | Provider transaction | `VERIFIED_ALLOWED` grant/revoke; `expired` transition unclear |
| Marketplace payout account | text; `not_configured`; `pending`, `verified`, `restricted` | Creator onboarding/admin verification | Creator ownership/admin | `VERIFIED_ALLOWED` selected paths; Medium |
| Marketplace payout | text; `pending`; `eligible`, `paid`, `failed`, `reversed` | Eligibility/manual records; automatic transfer absent | Global/creator finance controls | `PLANNED_NOT_IMPLEMENTED` for automated payment; graph unclear |
| Marketplace product review | text; `pending`; `published`, `rejected` | Customer create/admin moderate | Eligibility/ownership/admin | `VERIFIED_ALLOWED` selected paths; Medium |
| Marketplace abuse report | text; `open`; `investigating`, `resolved`, `dismissed` | Member report/admin resolution | Critical internal notification | `VERIFIED_ALLOWED` selected paths; Medium |
| Marketplace internal notification | text; `unread`; `acknowledged` | Critical report creation and admin acknowledgement | Admin recipient | `VERIFIED_ALLOWED` selected paths; Medium |

## Per-Entity Transition Interpretation

The compact catalog above applies these evidence rules to every row:

- The state column identifies the persistence type, initial state when verified, and allowed stored values. Text allowlists and PostgreSQL enums reject invalid destination values at application or database boundaries.
- The trigger column lists observed allowed transitions. A row marked `STATE_TRANSITION_UNCLEAR` has verified values but no complete allowed-from/to graph; destination validation must not be read as transition validation.
- The authorization column also records state-dependent validation and observable side effects. Missing side-effect text means no dedicated event, audit, notification, or external effect was found for that transition.
- Each observed transition changes the named entity row unless the row explicitly describes hard deletion. Invitation acceptance, ownership transfer, billing-event processing, and selected beta/Marketplace operations use transactions described in their workflow documents.
- Invalid destination values fail validation or database checks; unauthorized actors fail before mutation; stale/missing records return route-specific not-found or conflict errors. External email, provider, webhook, and audit failures follow the owning workflow's transaction boundary.
- Unless a row or its workflow names a test, no transition-specific test was found. The final column therefore reports confidence in the observed values and paths, not complete transition coverage.

## Transition Gaps

- The application has one explicit state machine only for content/page editorial status. Most other domains validate destination values without a central allowed-from/allowed-to matrix.
- Organization `suspended`/`deleted`, member `invited`/`suspended`, entitlement `expired`, and several payout states have no complete incoming/outgoing route sequence in inspected code.
- No publication scheduling state or scheduled transition worker exists.
- Frontend status-action mapping for entries/pages exposes one “next” action per status but does not define authority; backend transitions remain decisive.
- The shared Rust `PageStatus` model omits `pending_review`; this is `DOCUMENTATION_CODE_CONFLICT`/model drift even though current handlers use strings and `WorkflowStatus`.
