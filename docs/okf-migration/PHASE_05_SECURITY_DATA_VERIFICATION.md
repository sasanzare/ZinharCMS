# Phase 5 Security and Data Verification

**Date:** 2026-08-07
**Source HEAD:** `6f51612c5716c7d9c3365553811053fd24a03947`
**Scope:** source and test-evidence reconciliation for the eight constructed Phase 5 Concepts

This ledger distinguishes source verification, test-source inspection, and
live deployment evidence. `VERIFIED_SOURCE` means the behavior is directly
present in the pinned source. `PARTIAL_SOURCE` means the check covered a
bounded sample or a known subset. `NOT_EXECUTED` means the repository contains
test code but this documentation-only phase did not run it. `OPEN_OWNER_EVIDENCE`
means a deployment, legal, or operational decision is still required.

## RLS coverage inventory

The following tables are explicitly named by the source migrations as forced
RLS tables or policy targets. This is a migration-source inventory, not a live
database inspection.

### Base migration 0009

`content_types`, `content_entries`, `pages`, `page_versions`, `media`,
`media_variants`, `comments`, `webhooks`, `webhook_deliveries`,
`public_settings`, `navigation_items`, and `component_registry`.

The first eleven tables use the tenant-match policy for select/insert/update/
delete. `component_registry` uses a special select policy that permits system
rows and a write policy that requires a non-system row in the current
organization.

### Later policy additions

| Migration | Explicit policy tables | Observed policy shape |
| --- | --- | --- |
| `0010_v2_phase_five_billing_quota.sql` | `organization_subscriptions`, `usage_counters` | Forced RLS; tenant-match select/insert/update/delete policies |
| `0015_v3_phase_one_marketplace_foundation.sql` | `marketplace_installations` | Forced RLS; tenant-match select/insert/update/delete policies |
| `0020_v3_phase_seven_permission_sandbox_kill_switch.sql` | `marketplace_kill_switches` | Forced RLS; global rows can be selected, organization rows require tenant match; writes are organization-scoped |
| `0021_v3_phase_eight_runtime_adapters.sql` | `marketplace_template_imports`, `marketplace_plugin_hooks` | Forced RLS; tenant-match select/insert/update policies as defined by the migration |
| `0022_v3_phase_nine_marketplace_finance.sql` | `marketplace_purchases`, `marketplace_entitlements`, `marketplace_revenue_ledger` | Forced RLS; tenant-match select/insert policies, with update policy on purchases and no universal delete claim |
| `0024_v3_phase_ten_ratings_abuse.sql` | `marketplace_product_reviews`, `marketplace_abuse_reports` | Forced RLS; tenant-match select/insert/update policies |
| `0030_security_phase_seven_file_storage.sql` | `file_cleanup_jobs` | Forced RLS; tenant-match select/insert/update/delete policies |

The source inventory therefore names 24 tables with explicit RLS setup across
the inspected migrations. It does not claim that all organization-related
tables in the full schema are in this list.

## Verification ledger

| ID | Area | Verification claim | Evidence | Result | Limitation or next evidence |
| --- | --- | --- | --- | --- | --- |
| P5-VERIFY-001 | RLS helper | `app_rls_tenant_matches` compares the row organization with the session organization, with an explicit bypass branch. | `backend/migrations/0009_v2_phase_three_rls.sql` | VERIFIED_SOURCE | Bypass authorization and production caller inventory remain separate. |
| P5-VERIFY-002 | RLS base coverage | Base content, page, media, comment, webhook, public-settings, navigation, and component tables receive forced RLS/policies. | Migration `0009`; `services/rls.rs` | VERIFIED_SOURCE | Applied database state was not inspected. |
| P5-VERIFY-003 | RLS later coverage | Billing, Marketplace organization data, and file-cleanup tables add explicit policy migrations. | Migrations `0010`, `0015`, `0020`, `0021`, `0022`, `0024`, `0030` | VERIFIED_SOURCE | Policy operation sets vary; no blanket table claim is made. |
| P5-VERIFY-004 | Session context | Normal connections set organization/user context and `rls_bypass=false`; tenant transactions set the same values transaction-locally. | `backend/src/services/rls.rs` | VERIFIED_SOURCE | Pool/deployment configuration is not verified here. |
| P5-VERIFY-005 | Tenant membership | `X-Organization-Id`, access claims, active organization membership, role, rate limit, and quota checks are enforced in tenant middleware. | `backend/src/middleware/tenant.rs` | VERIFIED_SOURCE | This is source evidence, not a live request result. |
| P5-VERIFY-006 | Query defense | Inspected delivery, webhook, media, Marketplace, and quota paths repeat organization predicates in addition to connection context. | Route/service source at Phase 5 HEAD | PARTIAL_SOURCE | The sample is not an exhaustive query audit; the future schema/entity target remains deferred. |
| P5-VERIFY-007 | Live RLS matrix | The repository contains cross-tenant visibility/mutation/context-cleanup assertions and non-superuser checks. | `backend/tests/security_phase2_rls.rs` | NOT_EXECUTED | No `PHASE2_TEST_DATABASE_URL` live run was performed in Phase 5. |
| P5-VERIFY-008 | File constraints | File keys, filenames, MIME/content detection, PDF structure, symlink/reparse components, checksums, and lifecycle combinations are guarded. | `services/file_security.rs`; migration `0030`; Phase 7 file tests | VERIFIED_SOURCE | External malware scanning and storage durability remain unresolved. |
| P5-VERIFY-009 | Media delivery | Public media requires verified active public media/image rows; restricted downloads use tenant routes and organization predicates; cleanup is queued. | `backend/src/routes/media.rs`; `services/file_cleanup.rs` | VERIFIED_SOURCE | Production filesystem/CDN behavior is not established. |
| P5-VERIFY-010 | Authentication hardening | Access claims are validated against sessions; MFA/assurance and scope-bound Step-Up are required for selected sensitive paths. | `middleware/auth.rs`, `middleware/step_up.rs`, session/MFA services, migration tests | VERIFIED_SOURCE | Deployment secret/key rotation and edge configuration are outside this source pass. |
| P5-VERIFY-011 | Authorization | Global, organization, billing, webhook, Marketplace installer/approver, kill-switch, and creator-owner checks appear in the inspected helpers/routes. | `services/rbac.rs`; Marketplace/billing/webhook routes | VERIFIED_SOURCE | Complete policy ownership and support-role catalog remain open. |
| P5-VERIFY-012 | Billing/quota | Plan limits, usage counters, request checks, content/media checks, and organization-row locking are implemented in the quota path. | `services/quota.rs`; billing migration/routes | VERIFIED_SOURCE | Provider failure, compensation, refunds, tax, and retry policy are not evidenced. |
| P5-VERIFY-013 | Marketplace safety | Manifest/archive/checksum/permission gates, runtime allowlists, kill switches, and `execution: not_executed` are source-backed. | Marketplace validation/package/runtime services, routes, migrations, tests | VERIFIED_SOURCE | This does not prove a complete sandbox or external execution policy. |
| P5-VERIFY-014 | Deployment posture | Production ingress, TLS, secrets, external malware, monitoring, alerting, SLOs, on-call, and applied schema parity are not established by this repository snapshot. | Source and owner-decision register review | OPEN_OWNER_EVIDENCE | Requires platform/deployment evidence; do not promote placeholders. |
| P5-VERIFY-015 | Data governance | Privacy, retention, residency, deletion, legal hold, audit retention authority, and Marketplace archive retention are not fully assigned. | NOC-05, NOC-17; source lifecycle code | OPEN_OWNER_EVIDENCE | Requires owner/legal policy and affected-Concept updates. |

## Integrity conclusions

* The database Concept states exactly which source migrations were inspected
  and avoids a claim that all tenant data is protected by one universal policy.
* The security Concepts separate implemented mechanisms from production,
  legal, provider, and ownership gaps.
* The public-delivery Concepts distinguish the repository-defined default
  organization from intended custom-domain routing.
* The Marketplace Concepts distinguish validation/authorization from package
  execution and payment settlement.
* No live database, Redis, HTTP, payment, webhook receiver, object-storage, or
  monitoring check is represented as passed by this ledger.
