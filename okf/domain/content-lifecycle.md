---
okf_document_id: "domain-content-lifecycle"
title: "Content Lifecycle"
project: "ZinharCMS"
category: "domain"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/routes/content.rs"
  - "backend/src/services/workflow.rs"
  - "backend/src/services/entry_validation.rs"
related_documents:
  - "business-rule-catalog.md"
  - "publication-workflow.md"
  - "revisions-and-versioning.md"
related_diagrams:
  - "diagrams/content-lifecycle.mmd"
---

# Content Lifecycle

See the [content lifecycle diagram](diagrams/content-lifecycle.mmd) and [editorial publication workflow](workflows/editorial-publication.md).

## Content Type Lifecycle

| Stage | Actor/access | Validation and persistence | Exit/failure/tests |
| --- | --- | --- | --- |
| Create | Organization content-type manager | Nonempty name, valid slug, parsed field schema; insert tenant row | Returns type; validation/unique/quota-independent failure; no route tests found |
| Edit | Organization content-type manager | Replaces name, slug, and full field schema | Existing entries are not migrated/revalidated in the same operation; `VALIDATION_RULE_UNCLEAR` |
| Delete | Organization content-type manager with `confirm=true` | Hard delete; entry FK cascades | Audit follows; no restore; cascade/cache/webhook effects are not replayed |

## Content Entry Lifecycle

| Stage | Entry condition | Actor/permission | Validation | State/persistence/side effects | Failure and tests |
| --- | --- | --- | --- | --- | --- |
| Create draft | Existing tenant content type and available content capacity | Entry writer | Plugin before-save, rich-text sanitation, dynamic schema validation | Insert `draft`, version 1, author attribution | Validation/quota/DB error; validator/plugin tests, no route test |
| Edit | Existing entry and content type | Entry writer | Same transform/sanitize/validate ordering | Replace `data`, increment version; invalidate cache only when current state is published | No ownership comparison or concurrency token; no route test |
| Submit review | Current `draft` | Entry writer | Shared transition validator | `pending_review`, version increment | Invalid current state rejected; workflow unit test covers permission-independent transition |
| Publish | Current `pending_review`, or `draft` with reviewer bypass | Entry publisher | Shared transition validator; entry data is not revalidated here | `published`, set `published_at`, increment version, invalidate cache, run after-publish plugin, spawn webhooks | State persists before plugin/webhook effects; direct publish transition unit-tested |
| Reject | Current `pending_review` | Workflow reviewer | Shared transition validator | `draft`, clear `published_at`, increment version | No comment/reason required; no route test |
| Unpublish | Current `published` | Entry publisher | Shared transition validator | `draft`, clear timestamp, increment version, invalidate cache, spawn unpublish webhook | Webhook failure is non-fatal; no route test |
| Archive | Current `published` | Workflow reviewer | Shared transition validator | `archived`, clear timestamp, increment version, invalidate cache | No retention policy; no route test |
| Restore state | Current `archived` | Workflow reviewer | Shared transition validator | `draft`, version increment | This is status restoration, not historical content restoration |
| Delete | Any current state | Organization admin/editor | Entity/type/tenant lookup | Hard delete; published cache invalidation; audit | No restore; audit/cache not atomic; no route test |

## Localization and Scheduling

- Content schemas can contain ordinary fields that an owner may use for localized values, but no dedicated localized-entry/version relation was found.
- Public delivery accepts locale for navigation, not for selecting localized content entry variants.
- Scheduled publication, delayed unpublication, or a publication queue was not found: `PLANNED_NOT_IMPLEMENTED` for the observed repository.

## Ownership and Tenant Scope

Every content type and entry is organization-scoped. The API records `created_by`/`author_id`, but write access is organization-role based rather than a general update-own policy. See [Resource Ownership](../security/resource-ownership.md) and [Multi-Tenancy Behavior](multi-tenancy-behavior.md).

## Lifecycle Uncertainties

- `REVISION_BEHAVIOR_UNCLEAR`: entry version is a counter, not a retrievable snapshot history.
- `PUBLICATION_BEHAVIOR_UNCLEAR`: publish does not re-run schema validation; current saved data is assumed valid.
- `DELETION_BEHAVIOR_UNCLEAR`: cascaded type deletion can remove entries without per-entry webhook/audit behavior.
- `INVARIANT_UNVERIFIED`: updating a content schema can make stored entry data inconsistent with the new schema.

