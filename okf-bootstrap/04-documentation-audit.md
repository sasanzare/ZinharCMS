# Documentation Audit

## Scope and Method

The tracked repository contains:

- 66 Markdown files: 63 under docs plus README.md, AGENTS.md, and HANDOFF.md;
- 43 Mermaid files, audited individually in 05-mermaid-audit.md;
- 9 additional files under docs: 2 SQL fixtures, 5 JSON sample files, and 2 text sample assets.

Code, migrations, route composition, and tests were treated as more authoritative than prose. Git last-change dates are freshness signals, not proof of correctness.

Status values in this audit are CURRENT, PARTIALLY_CURRENT, OUTDATED, DUPLICATED, INCOMPLETE, and UNKNOWN.

## Root Documentation

| Path | Title/topic and audience | Status and code alignment | Freshness signal | Preserve/update/OKF/duplication disposition |
| --- | --- | --- | --- | --- |
| README.md | Product scope and quick start; all developers | CURRENT; matches implementation through V3 Phase 15 | Updated 2026-07-12 | Preserve; link from OKF project overview; do not duplicate phase-by-phase history |
| AGENTS.md | Persistent handoff protocol; maintainers and agents | CURRENT; repository workflow authority | Updated 2026-07-10 | Preserve and link from development workflow |
| HANDOFF.md | Cumulative recovery state; maintainers and agents | PARTIALLY_CURRENT; top checkpoint is current, historical overrides deliberately retain stale claims | Cumulative file with older sections | Preserve as operational state; OKF should link only, never ingest it as architecture truth |

## Canonical Cross-Cutting Documents

| Path | Title/topic and audience | Status and code alignment | Freshness signal | Preserve/update/OKF/duplication disposition |
| --- | --- | --- | --- | --- |
| docs/API.md | Manual API reference; API consumers/developers | PARTIALLY_CURRENT; route groups are current, but one paragraph names /api/billing/webhook instead of /api/billing/stripe/webhook | Updated through Phase 15 | Preserve and correct later; link from OKF API docs; deduplicate endpoint prose against generated OpenAPI |
| docs/ARCHITECTURE.md | Current architecture; developers/operators | CURRENT; matches modular monolith, tenancy, storage, and deferred boundaries | Updated through Phase 15 | Preserve; use as a secondary source and link from observed architecture |
| docs/I18N.md | Frontend localization guidance; frontend developers | OUTDATED; runtime rules remain valid, but coverage says only auth/shell/dashboard while translations now span more pages and remain incomplete | Last changed 2026-06-21 | Preserve and update in Phase 4; link from OKF frontend/i18n |

## Core Phase Records

Phase records are retained as historical, phase-scoped evidence. Statements such as “not implemented in this phase” are not current project claims unless the document presents itself as a current architecture source.

| Path | Topic/audience | Status | Code match and disposition |
| --- | --- | --- | --- |
| docs/PHASE_ZERO.md | Foundation; developers | CURRENT | Accurate phase record; preserve, link from project history |
| docs/PHASE_ONE.md | Auth/content/media; developers | CURRENT | Accurate phase record; preserve, link from module references |
| docs/PHASE_TWO.md | Page engine/preview; developers | CURRENT | Accurate phase record; preserve |
| docs/PHASE_THREE.md | Initial React admin; frontend developers | PARTIALLY_CURRENT | Accurate for Phase Three, but “visual builder remains Phase Four” is stale when read as current; preserve as history and add explicit supersession link later |
| docs/PHASE_FOUR.md | Visual page builder; developers | CURRENT | Matches PagesPage implementation; preserve |
| docs/PHASE_FIVE.md | Delivery/cache/webhooks; developers | CURRENT | Matches routes/services with documented retry limitations; preserve |
| docs/PHASE_SIX.md | Workflow/comments/plugins; developers | CURRENT | Matches built-in plugin scope; preserve |
| docs/PHASE_SEVEN.md | Security hardening; developers/security reviewers | CURRENT | Matches login throttling, headers, sanitization, and MIME/SSRF checks; preserve |

## V2 Phase and Operations Records

| Path | Topic/audience | Status | Code match and disposition |
| --- | --- | --- | --- |
| docs/V2_PHASE_ZERO.md | V1 assessment and V2 design; architects | PARTIALLY_CURRENT | Valuable design/history, but later V2/V3 code supersedes planning statements; preserve and link selectively |
| docs/V2_PHASE_ONE.md | Organization schema; backend developers | CURRENT | Matches migration 0008; preserve |
| docs/V2_PHASE_TWO.md | Tenant API context; full-stack developers | CURRENT | Matches middleware/frontend organization context; preserve |
| docs/V2_PHASE_THREE.md | PostgreSQL RLS; backend/security | CURRENT | Matches migration 0009 and rls.rs; preserve |
| docs/V2_PHASE_FOUR.md | Organization UI/member management; frontend/backend | CURRENT | Matches current routes and UI; preserve |
| docs/V2_PHASE_FIVE.md | Plans, quotas, usage; backend/product | CURRENT | Matches migration 0010 and quota service; preserve |
| docs/V2_PHASE_SIX.md | Stripe billing; backend/operators | CURRENT | Matches subscription integration; preserve |
| docs/V2_PHASE_SEVEN.md | SaaS operations; operators | PARTIALLY_CURRENT | Stored alert definitions exist, but evaluation/delivery remains absent; preserve with limitation |
| docs/V2_PHASE_EIGHT.md | Security QA/hardening; security/developers | CURRENT | Matches hardening tests and migration 0013; preserve |
| docs/V2_PHASE_NINE.md | Beta release; product/operators | CURRENT | Matches beta tables/routes/UI; preserve |
| docs/V2_PHASE_TEN.md | V2 GA; operators/product | CURRENT | Correctly represents readiness as docs/scripts/checks; preserve |
| docs/V2_ADMIN_GUIDE.md | Organization administration; operators | CURRENT | Matches current admin routes/UI; preserve and link from operations |
| docs/V2_BILLING_GUIDE.md | Billing administration; operators | CURRENT | Matches plan/usage/Stripe behavior; preserve |
| docs/V2_MIGRATION_GUIDE.md | V1-to-V2 migration; operators | CURRENT | Historical operational guide; preserve, do not copy into current schema overview |
| docs/V2_OPERATIONS_RUNBOOK.md | V2 operations/incidents; operators | PARTIALLY_CURRENT | Useful manual procedures; repository does not implement all backup/monitoring assumptions | Preserve and link; owner decisions required for production procedures |
| docs/V2_RELEASE_NOTES.md | V2 release summary; users/operators | CURRENT | Historical release record; preserve, reference only |

## V3 Phase Records

| Path | Topic/audience | Status | Code match and disposition |
| --- | --- | --- | --- |
| docs/V3_PHASE_0_1.md | Scope/policy lock | CURRENT | Accurate phase record; preserve |
| docs/V3_PHASE_0_2.md | V2 Marketplace readiness | CURRENT | Accurate baseline record; preserve |
| docs/V3_PHASE_ONE.md | Marketplace domain/base migration/storage | CURRENT | Phase-scoped non-goals are explicit and later phases are referenced; preserve |
| docs/V3_PHASE_TWO.md | Creator onboarding/submission | CURRENT | Matches routes/UI/service; preserve |
| docs/V3_PHASE_THREE.md | Package validation/security/compatibility | CURRENT | Matches persisted reports and validation service; preserve |
| docs/V3_PHASE_FOUR.md | Review/moderation | CURRENT | Matches global-admin review workflow; preserve |
| docs/V3_PHASE_FIVE.md | Catalog/search/detail | CURRENT | “No install in Phase Five” is historical phase scope; preserve |
| docs/V3_PHASE_SIX.md | Installation lifecycle | CURRENT | Phase-scoped deferred boundaries are explicit; preserve |
| docs/V3_PHASE_SEVEN.md | Permission/runtime policy/kill switch | CURRENT | Correctly states policy-only execution boundary; preserve |
| docs/V3_PHASE_EIGHT.md | Host-owned runtime adapters | CURRENT | Matches adapter routes/services; preserve |
| docs/V3_PHASE_NINE.md | Marketplace finance | CURRENT | Matches migrations 0022-0023 and finance/Stripe routes; preserve |
| docs/V3_PHASE_TEN.md | Customer feedback and abuse | CURRENT | Matches migrations 0024-0025 and moderation routes; preserve |
| docs/V3_PHASE_ELEVEN.md | Creator/admin analytics | CURRENT | Matches aggregate queries and UI; preserve |
| docs/V3_PHASE_TWELVE.md | Creator CLI and samples | CURRENT | Matches script and sample packages; preserve |
| docs/V3_PHASE_THIRTEEN.md | Security QA/performance | CURRENT | Matches migration 0026, tests, cache policy, load script; preserve |
| docs/V3_PHASE_FOURTEEN.md | Creator/customer beta readiness | CURRENT | Matches evidence script/static contracts; preserve |
| docs/V3_PHASE_FIFTEEN.md | Launch readiness/GA | CURRENT | Matches runbook, release notes, script, static contracts; preserve |

## Persistent V3 Marketplace Documents

| Path | Topic/audience | Status | Code match and disposition |
| --- | --- | --- | --- |
| docs/V3_MARKETPLACE_SCOPE.md | Initial and complete V3 scope; product/engineering | CURRENT | Separates complete lifecycle from initial beta scope; preserve as product-scope authority and link from OKF |
| docs/V3_PRODUCT_TAXONOMY.md | Product types; product/reviewers | CURRENT | Matches manifest product types and unsupported execution gates; preserve |
| docs/V3_MARKETPLACE_POLICY.md | Review/moderation/final policy; creators/reviewers/operators | CURRENT | Matches implemented moderation and launch policy; preserve |
| docs/V3_MARKETPLACE_DOMAIN_MODEL.md | Marketplace entities; architects/developers | OUTDATED | Still describes purchase, entitlement, ledger, review/report, and payout records as future despite migrations 0022-0025; preserve for history but rewrite/link from OKF database and Marketplace docs |
| docs/V3_MARKETPLACE_MANIFEST_SCHEMA.md | Manifest contract; creators/developers | CURRENT | Matches marketplace_manifest.rs; preserve as canonical contract |
| docs/V3_PACKAGE_STORAGE.md | Object key/checksum/size contract; backend/creators | CURRENT | Matches local filesystem implementation despite object-key terminology; preserve |
| docs/V3_MARKETPLACE_V2_DEPENDENCY_MATRIX.md | V2 dependency baseline; architects | PARTIALLY_CURRENT | Useful readiness baseline; later phases resolve several gaps | Preserve as historical reference, not current status |
| docs/V3_V2_MARKETPLACE_READINESS_AUDIT.md | Readiness audit; architects/product | PARTIALLY_CURRENT | Baseline findings are superseded by completed phases | Preserve, link as historical input |
| docs/V3_MARKETPLACE_GAP_LIST.md | Phase-by-phase gaps/resolutions; product/engineering | CURRENT | Explicitly records resolved and deferred items through Phase 15; preserve and link |
| docs/MARKETPLACE_CREATOR_GUIDE.md | CLI/package/submit guide; creators | CURRENT | Matches CLI and upload contract; preserve as canonical user guide |
| docs/V3_MARKETPLACE_OPERATIONS_RUNBOOK.md | Incidents/support/rollback; operators | CURRENT | Matches existing control surfaces and known manual boundaries; preserve |
| docs/V3_MARKETPLACE_RELEASE_NOTES.md | V3 GA scope/limitations; users/operators | CURRENT | Matches delivered and deferred scope; preserve |

## Diagram Support Documents

| Path | Topic/audience | Status | Code match and disposition |
| --- | --- | --- | --- |
| docs/diagrams/README.md | Diagram catalog; all readers | CURRENT | Catalog includes 00-42; preserve and link from OKF diagram index |
| docs/diagrams/DIAGRAM_CONVENTIONS.md | Evidence/status/syntax rules; diagram authors | CURRENT | Explicit convention authority; preserve and reuse |
| docs/diagrams/TRACEABILITY.md | Diagram-to-code matrix; developers/reviewers | PARTIALLY_CURRENT | Includes all diagrams, but inspection date and some baseline descriptions predate later phases | Preserve; synchronize during Phase 12 |
| docs/diagrams/FILE_EVIDENCE_INDEX.md | Domain-to-file index; developers | PARTIALLY_CURRENT | Broadly useful; several “not found” or older Marketplace notes need final revalidation | Preserve and link; avoid duplicating its entire matrix |
| docs/diagrams/REPOSITORY_INVENTORY.md | Architecture evidence inventory; diagram authors | PARTIALLY_CURRENT | Updated through Phase 15, but some per-document descriptions retain pre-Phase-9 wording | Preserve; use as secondary inventory |
| docs/diagrams/ARCHITECTURE_AUDIT.md | Implementation matrix and RLS analysis; architects | PARTIALLY_CURRENT | Later Phase 15 rows are current, but the Stripe row still says Marketplace finance is absent | Preserve; correct before treating as canonical |
| docs/diagrams/AMBIGUITIES.md | Cumulative ambiguity register; architects/owners | PARTIALLY_CURRENT | Historical checkpoints and superseded decisions are retained; final review resolves some older entries | Preserve; future OKF should expose active vs superseded status clearly |

## Non-Markdown Documentation and Samples

| Path(s) | Purpose | Status | Disposition |
| --- | --- | --- | --- |
| docs/sample-data.sql | Default-organization demo content | CURRENT | Preserve as a fixture; link from development setup |
| docs/V2_PHASE_EIGHT_FIXTURE.sql | Tenant-isolation hardening fixture | CURRENT | Preserve as a test fixture; link from testing/RLS docs |
| docs/marketplace-samples/component-pack/manifest.json | Example component-pack manifest | CURRENT | Preserve and link from creator guide |
| docs/marketplace-samples/component-pack/components/index.json | Component index | CURRENT | Preserve |
| docs/marketplace-samples/component-pack/components/hero-banner.json | Example component definition | CURRENT | Preserve |
| docs/marketplace-samples/component-pack/assets/preview.txt | Placeholder sample asset | CURRENT | Preserve as sample content |
| docs/marketplace-samples/integration-plugin/manifest.json | Example integration manifest | CURRENT | Preserve |
| docs/marketplace-samples/integration-plugin/hooks/index.json | Example hook index | CURRENT | Preserve |
| docs/marketplace-samples/integration-plugin/assets/readme.txt | Placeholder sample asset | CURRENT | Preserve |

## Documentation-Code Conflicts

| ID | Conflict | Code evidence | Recommended later action |
| --- | --- | --- | --- |
| DCC-01 | docs/PHASE_THREE.md can imply the visual builder is still future | PagesPage.tsx and PHASE_FOUR.md implement it | Add an explicit superseded-by note |
| DCC-02 | docs/I18N.md understates current translation coverage | frontend/src/i18n and translated page usage | Rewrite coverage section while keeping known hard-coded labels |
| DCC-03 | docs/V3_MARKETPLACE_DOMAIN_MODEL.md describes finance/feedback entities as future | migrations 0022-0025 and Marketplace routes | Update current physical model and keep phase history separate |
| DCC-04 | docs/diagrams/ARCHITECTURE_AUDIT.md says Marketplace purchases, entitlements, and payouts are absent | migration 0022 and finance routes/services | Update the Stripe/Marketplace finance row |
| DCC-05 | docs/diagrams/00-implementation-status-map.mmd says both API.md and OpenAPI are stale | API.md covers current route groups; OpenAPI is partial for older Marketplace/catalog handlers | Narrow the label to generated OpenAPI coverage |
| DCC-06 | docs/diagrams/02-system-context.mmd says paid Marketplace purchase is deferred | Phase 9 implements free/paid one-time checkout and entitlements | Update Marketplace API label |
| DCC-07 | docs/diagrams/30-marketplace-sequences.mmd says install, purchase, payout, and rating flows are not implemented | Phases 6, 9, and 10 implement those flows | Rewrite or split the stale sequence |
| DCC-08 | docs/diagrams/33-marketplace-installation-lifecycle.mmd treats paid entitlement as future/rejected | Phase 9 gates paid lifecycle by active entitlement | Update install-gate branches |
| DCC-09 | docs/API.md names /api/billing/webhook in one Marketplace paragraph | Router path is /api/billing/stripe/webhook | Correct the path |
| DCC-10 | docs/diagrams/20-marketplace-package-review-pipeline.mmd labels purchase_runtime as implemented/partial but assigns the planned visual class | Node text and current Phase 9/10 implementation | Correct the class assignment |

## Duplication and Consolidation Findings

- Phase documents intentionally duplicate summaries in README.md; OKF should link to phase records and maintain one current capability summary.
- API route lists appear in code annotations, docs/API.md, frontend/src/services/api.ts, and diagrams. Future OKF should generate or validate a coverage index rather than copy all four.
- Architecture facts appear in ARCHITECTURE.md and seven diagram support documents. OKF should use a short observed architecture document plus links to the evidence matrices.
- HANDOFF.md duplicates completed-work history and must remain outside canonical product knowledge.
- V3 readiness audits and dependency matrices are historical inputs; V3_MARKETPLACE_GAP_LIST.md is the more current status ledger.
- No document should be deleted in Phase Zero.

