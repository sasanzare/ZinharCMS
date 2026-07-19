---
okf_document_id: "extensibility-risks"
title: "Extensibility Risks"
project: "ZinharCMS"
category: "extensibility"
phase: 9
status: "current"
source_of_truth: false
implementation_view: "observed"
extensibility_status: "mixed"
last_verified_commit: "56d733985fdd7aa3f25ee6981b88cf29c52f65c9"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/plugins/mod.rs"
  - "backend/src/routes/plugins.rs"
  - "backend/src/services/marketplace_runtime.rs"
  - "backend/src/routes/marketplace_adapters.rs"
  - "backend/migrations"
related_documents:
  - "plugin-architecture.md"
  - "plugin-permissions.md"
  - "plugin-data-and-migrations.md"
  - "compatibility-and-versioning.md"
  - "isolation-and-trust.md"
  - "extensibility-testing.md"
related_diagrams:
  - "diagrams/extensibility-context.mmd"
  - "diagrams/plugin-permission-flow.mmd"
---

# Extensibility Risks

## Canonical Risk Register

| Risk ID | Title | Description | Evidence | Affected plugins | Affected extension points | Affected tenants | Likelihood | Impact | Severity | Existing mitigation | Mitigation confidence | Recommended follow-up | Owner confirmation required | Status |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| EXR-01 | Trusted callback failure or misuse | Built-in callbacks execute with backend-process authority and can fail requests. | backend/src/plugins/mod.rs | SEO Auto and future built-ins | EP-001, EP-002 | All tenants while globally enabled | Possible | Process/request/data mutation | HIGH | Trusted build, code review, Result propagation | Medium | Define trust policy, timeout/failure strategy, and integration tests | Yes: Backend and Security | VERIFIED_WEAKNESS |
| EXR-02 | Runtime policy mistaken for sandbox | Authorization may be presented as executable isolation although responses say not_executed. | marketplace_runtime.rs and runtime routes | Future Marketplace plugins | EP-005, EP-006 | Marketplace tenants | Possible | Unsafe product expectations and future design error | HIGH | Explicit not_executed response and host adapters | High for current non-execution | Preserve wording and design isolation before any executor | Yes: Product and Security | DOCUMENTATION_CONFLICT_RISK |
| EXR-03 | Tenant isolation lacks live proof | RLS/RBAC/context are structural evidence without a full live cross-tenant suite. | Tenant routes, RLS migrations, testing gap | Marketplace products | EP-003 through EP-006 | All tenants | Possible | Cross-tenant data exposure if a path regresses | HIGH | Tenant middleware, RBAC, forced RLS | Medium | Add real two-tenant route/database matrix including bypass callers | Yes: Backend, Database, Security | MISSING_EVIDENCE |
| EXR-04 | Built-in least authority absent | CmsPlugin has no per-plugin capability boundary. | CmsPlugin callback contract | SEO Auto and future built-ins | EP-001, EP-002 | All tenants | Likely for any new built-in | Excess data/process access | HIGH | Trusted-source boundary | Medium | Require code-owner review and design a narrower interface before expansion | Yes: Backend and Security | VERIFIED_WEAKNESS |
| EXR-05 | Stale built-in registry rows | Sync upserts current built-ins but does not delete absent keys. | routes/plugins.rs | Removed/renamed built-ins | Built-in registration | Global operations | Possible | Confusing or ineffective admin state | MEDIUM | Stable unique keys and metadata upsert | High | Define deprecation and removal reconciliation | Yes: Backend/Product | VERIFIED_WEAKNESS |
| EXR-06 | Derived component collision or staleness | Marketplace definitions share component_registry and cleanup behavior is incomplete. | marketplace_adapters.rs; component registry migrations | Component-pack declarations | EP-003 | Installing tenants | Possible | Wrong palette/schema or unusable pages | HIGH | Namespaced keys, unique key, installation FK | Medium | Specify ownership/upsert/disable/uninstall/update cleanup and test it | Yes: Pages/Marketplace | SUSPECTED_WEAKNESS |
| EXR-07 | Plugin compatibility policy absent | Compiled trait and renderer contracts lack stability/deprecation policy. | CmsPlugin trait and unversioned component schema | SEO Auto and future built-ins/components | EP-001 through EP-003 | All tenants | Possible | Upgrade breakage | MEDIUM | Same-repository compilation; pinned Marketplace versions | Medium | Define interface and renderer support policy | Yes: Backend/Frontend/Product | MISSING_EVIDENCE |
| EXR-08 | Stored settings may be ineffective or unsafe | cms_plugins.settings is persisted but not passed to current callbacks and has no secret model. | routes/plugins.rs; CmsPlugin signature | Built-ins | Plugin configuration surface | All tenants/global admins | Likely for attempted use | Misconfiguration or secret exposure | MEDIUM | JSON object validation and restricted management | Low | Remove unused settings or add typed validated non-secret contract | Yes: Backend/Product/Security | VERIFIED_WEAKNESS |
| EXR-09 | Future package execution lacks isolation prerequisites | Current validation is not a signature chain or execution sandbox. | validation/runtime services; no loader | Future integration/backend extensions | Future executor; EP-006 policy | Future installing tenants | Possible if roadmap adds execution | Host compromise | CRITICAL | Current non-execution, checksum/path checks, review, kill switches | High for current safety boundary | Require provenance, signing, isolation, mediation, quotas, audit, and response controls first | Yes: Security/Architecture | PLANNED_BEHAVIOR |
| EXR-10 | Public hooks have no verified delivery | Definitions can be listed/authorized but not rendered or invoked. | marketplace_adapters route/service; frontend search | Integration-plugin declarations | EP-005 | Installing tenants | Likely | Feature appears active without effect | MEDIUM | not_executed response and strict public-type allowlist | High | Document limitation and implement host-owned consumers selectively | Yes: Product/Frontend/Marketplace | VERIFIED_WEAKNESS |
| EXR-11 | Uninstall retention incomplete | Soft uninstall preserves organization data, while artifact and derived-record retention are unclear. | installation service and migrations | All Marketplace products | EP-003 through EP-005 | Installing tenants | Likely over time | Storage growth/stale references | MEDIUM | Auditable soft state and preserve-data policy | High | Define retention and cleanup without deleting tenant-authored content | Yes: Marketplace/Operations/Data | MISSING_EVIDENCE |
| EXR-12 | Cross-layer test coverage gap | Pure/unit/frontend tests do not prove complete DB/RLS/provider/concurrency flows. | Extensibility Testing matrix | SEO Auto and Marketplace products | All verified points | All tenants | Likely | Undetected lifecycle/security regressions | HIGH | 79 Marketplace tests, 2 SEO tests, frontend tests | Medium | Build creator-to-review-to-install-to-adapter real-stack suite | Yes: QA/Backend/Frontend | VERIFIED_WEAKNESS |

Status classes deliberately separate VERIFIED_WEAKNESS, SUSPECTED_WEAKNESS, MISSING_EVIDENCE, PLANNED_BEHAVIOR, and DOCUMENTATION_CONFLICT_RISK. EXR-09 does not classify the currently unimplemented executor as a present vulnerability.
