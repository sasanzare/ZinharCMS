---
okf_document_id: "security-resource-ownership"
title: "Resource Ownership"
project: "ZinharCMS"
category: "security-authorization"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "mixed"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/routes/content.rs"
  - "backend/src/routes/pages.rs"
  - "backend/src/routes/comments.rs"
  - "backend/src/routes/organizations.rs"
  - "backend/src/routes/marketplace.rs"
  - "backend/src/routes/marketplace_finance.rs"
  - "backend/src/routes/marketplace_analytics.rs"
related_documents:
  - "authorization-architecture.md"
  - "tenant-access-control.md"
  - "roles-and-permissions-catalog.md"
related_diagrams:
  - "diagrams/authorization-decision-flow.mmd"
---

# Resource Ownership

## Ownership Types

| Resource | Stored relationship | Observed authorization meaning |
| --- | --- | --- |
| Organization | `owner_id` and owner membership role | Ownership transfer/leave/removal constraints and organization override |
| Marketplace creator | `user_id` | Creator-profile, listing, analytics, payout, and submission ownership checks |
| Customer product review | `author_id` plus organization | Update/eligibility and moderation distinctions |
| Content entry | `author_id` | Attribution and workflow context; general write permission is role-based |
| Page | `author_id` | Attribution and version creator context; general write permission is role-based |
| Comment | `author_id` | Attribution; create/manage operations are primarily role-based |
| Media | `uploader_id` | Attribution; media mutation is role-based |
| Marketplace installation | `organization_id`, `installed_by` | Organization-scoped lifecycle control, not personal ownership |

## Explicit Owner Enforcement

Marketplace creator operations call helpers that compare the creator record with the authenticated user. Creator analytics and payout operations repeat owner checks even when an RLS bypass transaction is required to query global Marketplace tables. Organization ownership changes require explicit membership and lifecycle rules.

## Attribution Is Not a General ACL

Fields such as entry/page `author_id`, media `uploader_id`, and `created_by` do not establish a repository-wide update-own-only rule. Named organization RBAC helpers authorize many mutations regardless of creator identity. Legacy database role permission strings include concepts such as own-entry updates, but those strings are not used by current runtime RBAC.

## Polymorphic Subjects

Comments and audit logs identify target resources with `entity_type` and `entity_id` without conventional target foreign keys. Comment handlers validate supported target types and tenant presence, but ownership semantics remain distributed.

## Uncertainty

`RESOURCE_OWNERSHIP_UNVERIFIED ROU-01`: no complete resource-action-owner matrix or end-to-end IDOR suite was found. Marketplace static security contracts cover selected ownership paths, but content, page, media, comments, and operational resources do not share one ownership policy engine.
