---
okf_document_id: "domain-membership-ownership"
title: "Membership and Ownership"
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
  - "backend/src/services/rbac.rs"
  - "backend/migrations/0008_v2_phase_one_organizations.sql"
related_documents:
  - "multi-tenancy-behavior.md"
  - "../security/resource-ownership.md"
  - "../security/roles-and-permissions-catalog.md"
related_diagrams:
  - "diagrams/tenant-membership-workflow.mmd"
---

# Membership and Ownership

## Organization Membership Rules

| Actor | Subject/action | Required access | Persistence/service enforcement | API/frontend/failure/tests | Confidence |
| --- | --- | --- | --- | --- | --- |
| Authenticated user | Create organization/self owner membership | Authentication | Organization+owner+subscription transaction; membership PK | Create organization; Organization UI; duplicate slug/DB failure; no integration test | High |
| Owner/admin | List members | Admin helper includes owner override | Tenant filters/RLS | Current members; Organization UI; no tests | High |
| Admin | Assign non-owner role or change non-owner | Admin | Destination role allowlist; update active membership row | Member patch; invalid role/not found; no tests | High |
| Owner | Assign owner role or modify an owner | Owner | Owner-only branch; last-owner check on downgrade | Member patch; validation on last owner; no concurrency test | Medium-high |
| Owner/admin | Remove non-owner | Admin | Hard delete tenant membership | Member delete; audit after mutation; no tests | High |
| Owner | Remove owner | Owner | Last-owner count check then hard delete | Member delete; validation; no concurrency test | Medium |
| Current member | Leave organization | Any active member | Self membership hard delete; owners use last-owner check | Leave endpoint/UI; no tests | Medium-high |
| Owner/admin | Invite member | Admin; owner required when inviting owner | Valid role/email, plan member capacity, unique pending invite | Invitation endpoint/UI/email; no end-to-end test | High |
| Invitation recipient | Accept invitation | Authentication and email/token match | Membership upsert + invitation accepted in transaction | Public protected accept endpoint; invalid/expired not found; no tests | High |
| Owner/admin | Revoke invitation | Admin | Pending invitation changes to revoked | Invitation delete-style endpoint/UI; no tests | High |

## Ownership Transfer

Only the current organization owner can transfer ownership. The target must be a current active member and cannot be the caller. One transaction demotes the caller to admin, promotes/activates the target as owner, and updates `organizations.owner_id`. See [Organization Ownership Transfer](workflows/organization-ownership-transfer.md).

## Last-Owner Rule

The service counts active owner memberships and rejects removal, downgrade, or leave when the count is one or lower. This is application enforcement only. Because count and later mutation are not consistently locked in one transaction, the invariant is `OWNERSHIP_RULE_UNCLEAR` under concurrent owner changes.

## Duplicate Membership and Invitation Rules

- Membership primary key prevents duplicate organization/user rows.
- Acceptance upserts and reactivates an existing membership, retaining the original `joined_at` when present.
- Partial unique index prevents duplicate pending invitations per organization/email.
- Historical invitations in non-pending states can coexist with a new pending invitation.

## Resource Ownership

| Resource | Ownership field | Current authorization meaning |
| --- | --- | --- |
| Organization | `owner_id` plus owner membership | Explicit owner-only administration and transfer rules |
| Content entry | `author_id` | Attribution; general write access is role-based |
| Page/version | `author_id`, `created_by` | Attribution; page mutation is role-based |
| Media | `uploader_id` | Attribution; media mutation is role-based |
| Comment | `author_id` | Attribution; manager role resolves/deletes |
| Marketplace creator | `user_id` | Explicit creator ownership checks |
| Marketplace review | `author_id` | Customer ownership plus admin moderation |
| Marketplace installation | `organization_id`, `installed_by` | Tenant lifecycle control, not personal ACL |

There is no general repository-wide “owner can edit, others cannot” policy. Legacy role permission strings include update-own terms, but current organization RBAC helpers do not read those arrays.

## Disabled and Deleted Users

Inactive users cannot establish/refresh sessions. Existing tenant membership rows remain until removed or cascaded. User deletion behavior is mixed by foreign key: access records cascade while attribution often becomes null. No general user disable-to-membership-suspension workflow was found.

## Administrative Override

Global `super_admin` overrides global `require_any`, but not tenant membership/RLS. Organization `owner` overrides many organization-role helpers, but does not grant global Marketplace or plugin administration. Explicit bypass workflows must still perform their caller-specific authorization.

