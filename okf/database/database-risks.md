---
okf_document_id: "database-risks"
title: "Database Risk Register"
project: "ZinharCMS"
category: "database"
phase: 5
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "mixed"
last_verified_commit: "70b972428799304c7defd7e67f95459cd4a3644e"
last_verified_date: "2026-07-18"
primary_sources: ["backend/migrations", "backend/src/models", "backend/src/routes", "backend/src/services", "backend/src/services/hardening.rs"]
related_documents: ["database/migrations.md", "database/multi-tenancy.md", "database/transactions-and-consistency.md", "database/database-testing.md"]
uncertainty_markers: ["UNKNOWN U-02", "MIGRATION_MODEL_CONFLICT MMC-01", "MIGRATION_MODEL_CONFLICT MMC-02", "TENANT_ISOLATION_UNVERIFIED TIV-01", "TRANSACTION_BOUNDARY_UNCLEAR TBU-01"]
---

# Database Risk Register

## Prioritized Risks

| Risk ID and title | Description and evidence | Affected entities / modules | Likelihood / impact / severity | Existing mitigation; follow-up; owner confirmation; status |
| --- | --- | --- | --- | --- |
| DB-RISK-001 Runtime schema drift (`U-02`, `SRU-01`) | Migration files do not prove the objects applied in each environment | All entities; platform/operations | Possible / high / `HIGH` | SQLx migration metadata exists; Phase 10 inventory/runbook; owner yes; `OPEN` |
| DB-RISK-002 Tenant-parent incoherence (`TIV-01`, `CCU-01`) | Separate organization and parent FKs do not universally prove same-tenant relationships | Content, pages, media, comments, Marketplace; tenant/domain modules | Possible / high / `HIGH` | RLS, predicates, selected propagation triggers; Phase 7/8 tests and policy decision; owner yes; `OPEN` |
| DB-RISK-003 RLS verification drift (`DCC-12`) | Hardening service lists 24 RLS tables while final migration intent contains 32 | Later Marketplace tenant tables; hardening/runtime | Likely / high / `HIGH` | Migration policies exist; implementation-phase checker/test update after expected scope is approved; owner yes; `OPEN` |
| DB-RISK-004 Page status mapping drift (`MMC-01`) | Shared `PageStatus` omits migration value `pending_review` | Pages/versions; Pages module | Likely when shared enum is used / medium / `HIGH` | Local String rows avoid some paths; Phase 6 contract audit then authorized code fix; owner no for factual drift; `OPEN` |
| DB-RISK-005 Partial shared models (`MMC-02`) | Several shared CMS models omit migration-defined organization columns | Content, pages, components, media; multiple CMS modules | Likely / medium / `MEDIUM` | Selected-column/local projections; classify/align in future backend work; owner yes for intended model role; `OPEN` |
| DB-RISK-006 Media partial writes (`TBU-01`) | Filesystem and DB operations lack one atomic resource transaction | Media/variants; Media module | Possible / high / `HIGH` | Validation and sequential error handling; Phase 8/10 idempotency, cleanup, failure tests; owner yes; `OPEN` |
| DB-RISK-007 Cross-commit audit gap (`TBU-02`) | Organization creation commits domain state before separate audit write | Organizations/membership/subscription/audit | Possible / medium / `MEDIUM` | Domain transaction protects primary rows; Phase 8 audit failure policy; owner yes; `OPEN` |
| DB-RISK-008 Provider checkout boundary (`TBU-03`) | Paid checkout commits pending state before external Stripe creation | Purchases/entitlements; Marketplace finance/Stripe | Possible / high / `HIGH` | Idempotency IDs and failure status; Phase 8/10 reconciliation tests/runbook; owner yes; `OPEN` |
| DB-RISK-009 Non-durable webhook side effect (`TBU-04`) | Spawned CMS webhook has no durable queue/outbox or automatic retry scheduler | Webhooks/deliveries; Delivery | Likely under process loss / high / `HIGH` | Delivery-attempt records and timeout; Phase 8/10 durability decision; owner yes; `OPEN` |
| DB-RISK-010 Fragmented SQL ownership (`PBU-01`, `EOU-01`) | Routes/services and shared audit/provider paths write across module boundaries | All, especially audit/catalog/finance; backend modules | Likely change-risk / medium / `MEDIUM` | Module/entity maps and code search; Phase 12 governance/synchronization; owner yes; `OPEN` |
| DB-RISK-011 Status/constraint contract drift (`CCU-02`) | Checked text states, PostgreSQL enums, and Rust strings/enums have no one generated contract | Workflow, SaaS, Marketplace; multiple modules | Possible / medium / `MEDIUM` | DB checks and application validation; Phase 6 contract inventory plus compatibility tests; owner no for need to test; `OPEN` |
| DB-RISK-012 Index purpose uncertainty (`IPU-01`) | Some indexes overlap uniqueness/other prefixes without workload or plan evidence | Identity, CMS, media, Marketplace; platform | Possible cost / low-to-medium / `LOW` | 109-name inventory; Phase 10 measure before optimization; owner yes before removal; `OPEN` |
| DB-RISK-013 Weak database regression isolation (`TIU-01`, `TCG-01`) | No dedicated DB reset harness or broad real-PostgreSQL integration suite was found | All entities/modules | Likely / high / `HIGH` | CI provisions PostgreSQL and manual fixture exists; Phase 10 build isolated migration/RLS/transaction suite; owner yes; `OPEN` |
| DB-RISK-014 Mixed deletion lifecycle (`SDBU-01`, `DLU-02`) | Hard delete, archive, replacement, and uninstall semantics differ by entity | CMS, settings, installations, audit/history; domain modules | Likely / medium / `MEDIUM` | FK/status constraints and entity docs; Phase 8 lifecycle ownership; owner yes; `OPEN` |
| DB-RISK-015 Retention/privacy unknown (`DLU-01`, `U-07`) | No repository-wide purge, anonymization, legal-hold, or retention policy was found | Identity/security, audit, delivery, billing, beta, abuse | Likely governance gap / high / `HIGH` | None verified; Phase 7/10 owner/legal decision; owner yes; `OPEN` |
| DB-RISK-016 Recovery unknown (`U-04`, `NOC-03`) | Backup, restore, RPO/RTO, and restore-test evidence is absent | Entire PostgreSQL store; operations | Unknown / high / `HIGH` | None verified; Phase 10 recovery runbook and restore test; owner yes; `OPEN` |
| DB-RISK-017 Migration governance unknown (`NOC-14`) | Startup auto-migrates, but production authority, lock budgeting, and rollback ownership are not documented | All schemas; platform/operations | Possible / high / `HIGH` | Ordered SQLx history; Phase 10 deployment/migration runbook; owner yes; `OPEN` |
| DB-RISK-018 Public tenant selection unknown (`U-08`) | Host/domain-to-organization resolution is not proven end-to-end | Settings/navigation/delivery/domains; Delivery/Organizations | Possible / high / `HIGH` | Membership/RLS protect admin paths; Phase 6 API and Phase 7 security verification; owner yes; `OPEN` |

## Non-Findings

No table, column, index, or model is declared dead solely because a direct reference was not found. No index removal, constraint change, schema rewrite, or migration edit is proposed in Phase 5. Migration-defined intent is not presented as deployed fact.

## Owner Decisions Needed

Owners must confirm schema deployment/migration authority, recovery objectives, retention/privacy rules, public tenant selection, cross-resource compensation, database-test isolation, and the support window for application/schema compatibility. Those decisions should precede remediation work.

## Evidence Links

The principal evidence views are [Schema Catalog](schema-catalog.md), [Migrations](migrations.md), [Relationships](relationships.md), [Multi-Tenancy](multi-tenancy.md), [Transactions and Consistency](transactions-and-consistency.md), [Persistence Mapping](persistence-mapping.md), and [Database Testing](database-testing.md).
