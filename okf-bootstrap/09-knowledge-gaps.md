# Knowledge Gaps

## Marker Definitions

- UNKNOWN identifies a fact that cannot be verified from the repository or current environment.
- NEEDS_OWNER_CONFIRMATION identifies a product, policy, operational, or architectural decision that the repository cannot make.
- DOCUMENTATION_CODE_CONFLICT identifies a concrete mismatch between current implementation and documentation.

## UNKNOWN Index

| ID | Unknown fact | Why it matters | Best verification source | Priority |
| --- | --- | --- | --- | --- |
| UNKNOWN U-01 | Production hosting topology, ingress, TLS termination, and network boundaries | The Docker Compose topology cannot be represented as production fact | Infrastructure configuration and platform owner | High |
| UNKNOWN U-02 | Applied migration version and schema drift in each deployed environment | Repository migrations do not prove environment state | Deployment database migration table and release records | High |
| UNKNOWN U-03 | Production logging, metrics, tracing collection, dashboards, alerts, and SLO baseline | Code-level tracing and readiness do not define an observability system | Monitoring platform configuration and operations owner | High |
| UNKNOWN U-04 | Backup schedule, restore process, RPO, RTO, retention, and restore-test evidence | Recovery guarantees cannot be inferred | Database/storage platform policies and restore records | High |
| UNKNOWN U-05 | Production media and Marketplace artifact storage and delivery topology | Local filesystem behavior may not scale or survive multiple instances | Infrastructure and storage configuration | High |
| UNKNOWN U-06 | Environment promotion, release cadence, rollback authority, and incident ownership | CI contains quality gates but no deployment workflow | Release process and team ownership records | High |
| UNKNOWN U-07 | Applicable retention, privacy, residency, audit, and legal requirements | Data lifecycle and logging rules depend on external obligations | Product/legal/security policy | High |
| UNKNOWN U-08 | Intended public tenant selection and custom-domain delivery contract | Current public delivery always selects organization slug default | Product architecture decision and routing configuration | High |
| UNKNOWN U-09 | Supported production and contributor Rust, Node, and browser versions | Docker and CI use different Rust/Node version policies | Engineering support policy | Medium |
| UNKNOWN U-10 | Product license and distribution policy | Cargo metadata names licenses, but no root license file establishes repository distribution terms | Legal/product owner | Medium |
| UNKNOWN U-11 | Whether ignored local Marketplace distribution archives must be retained, regenerated, or deleted | Artifact provenance and developer cleanup rules are unclear | Release/creator tooling owner | Low |
| UNKNOWN U-12 | Code ownership, required reviewers, branch protections, and merge policy | Contribution and documentation approval cannot be assigned | Repository settings and engineering owner | Medium |
| UNKNOWN U-13 | Measured production capacity, traffic profile, and scaling thresholds | Phase 13 checks are readiness evidence, not production workload measurements | Production telemetry and capacity plan | Medium |
| UNKNOWN U-14 | Production email delivery provider, retry guarantees, and failure operations | The repository provides logging/webhook bridge behavior, not a confirmed provider topology | Deployment configuration and operations policy | Medium |
| UNKNOWN U-15 | Support and security incident escalation contacts | Runbooks cannot name accountable responders | Organization ownership directory | Medium |

## Domain Gap Register

| Category | Missing or ambiguous information | Why it matters | Derivable from code? | Owner decision | Priority |
| --- | --- | --- | --- | --- | --- |
| Architecture | UNKNOWN U-01, U-05, U-13; process boundaries beyond one modular monolith instance | Final context, container, and deployment views need real production boundaries | Local/reference architecture only | NOC-02 and NOC-06 | High |
| Modules | Formal module ownership and public/internal dependency rules | Current 22-module map is observed, not governed | Dependencies are partly extractable; ownership is not | NOC-15 | Medium |
| Database | UNKNOWN U-02 and U-04; deletion/retention policy; enum versus text-state rationale | Schema and recovery docs need operational truth | Schema mechanics yes; policies no | NOC-03, NOC-05, NOC-12 | High |
| APIs | Partial OpenAPI, no deprecation policy, inconsistent list/idempotency conventions | Client safety and compatibility are incomplete | Route inventory yes | NOC-08 | High |
| Authentication | Browser token-expiry recovery and production session policy are not consolidated | User-facing session failure behavior may vary | Current mechanics yes; target behavior partly policy | NOC-12 | Medium |
| Authorization | No single global-role, tenant-role, creator, reviewer, and support-role matrix | Reviewers cannot verify least privilege from one source | Most checks are extractable | NOC-15 | High |
| Business Rules | Content schema evolution, workflow meaning, compensation, refund/dispute/tax boundaries | State and money semantics need explicit authority | Current rules partly extractable | NOC-09, NOC-12 | High |
| Multi-tenancy | UNKNOWN U-08; custom-domain verification and public delivery routing | Tenant isolation is strong internally, but public selection is unclear | Current default behavior yes | NOC-01 and NOC-11 | High |
| Plugins | Built-in plugin lifecycle versus Marketplace adapter roadmap and external execution boundary | The two extension systems can be confused | Current non-execution boundary yes | NOC-10 | Medium |
| Marketplace | Appeal/restoration, artifact cleanup, disputes, partial refunds, automated payouts, package execution roadmap | Current scope is implemented but future contractual boundary is unclear | Current behavior yes | NOC-09 and NOC-10 | High |
| Page Builder | Component governance, schema migration, preview scaling, and large-page decomposition rules | Builder data compatibility and multi-instance preview behavior need policy | Current structure yes | NOC-12 | Medium |
| Frontend | Supported browsers, accessibility target, component architecture, token recovery, DTO generation | Quality and compatibility claims cannot be finalized | Current implementation yes | NOC-07 and NOC-12 | Medium |
| Backend | Durable jobs, transaction/side-effect policy, module ownership, capacity behavior | Failure semantics are dispersed | Current behavior mostly | NOC-09 and NOC-15 | High |
| Testing | No coverage threshold, API integration standard, migration matrix, browser E2E suite, or Mermaid parser gate | Passing current tests does not define complete quality policy | Existing test counts yes | NOC-14 | Medium |
| Security | UNKNOWN U-07 and U-15; key rotation, secret management, incident response, retention, public asset policy | Production security posture extends beyond code | Some controls yes; operational policy no | NOC-02, NOC-05, NOC-15 | High |
| Deployment | UNKNOWN U-01, U-02, U-05, U-06, U-09 | No production deployment contract exists in the repository | Reference containers only | NOC-02, NOC-06, NOC-07 | High |
| Operations | UNKNOWN U-03, U-04, U-06, U-14, U-15 | Runbooks need real systems, ownership, recovery, and escalation | No | NOC-03, NOC-04, NOC-06, NOC-15 | High |
| Observability | UNKNOWN U-03 and U-13; business and security signal ownership | Readiness probes are insufficient for service-level operations | Instrumentation fragments only | NOC-04 | High |
| Contribution | UNKNOWN U-09 and U-12; commit, review, ownership, and documentation update policy | Contributors cannot reliably follow hidden rules | Recent patterns only | NOC-07 and NOC-14 | Medium |
| Troubleshooting | Environment-specific diagnosis, known failure signatures, safe repair, rollback, and escalation | Existing phase/handoff history is not a concise runbook | Some symptoms are extractable | NOC-03, NOC-04, NOC-06, NOC-15 | Medium |

## NEEDS_OWNER_CONFIRMATION Index

Detailed questions and impact are in 12-owner-questions.md.

| ID | Decision area | Priority |
| --- | --- | --- |
| NEEDS_OWNER_CONFIRMATION NOC-01 | Public delivery tenant and custom-domain routing | High |
| NEEDS_OWNER_CONFIRMATION NOC-02 | Production media and Marketplace artifact storage/security | High |
| NEEDS_OWNER_CONFIRMATION NOC-03 | Backup, restore, RPO, RTO, and retention | High |
| NEEDS_OWNER_CONFIRMATION NOC-04 | Observability, SLOs, alerting, and on-call ownership | High |
| NEEDS_OWNER_CONFIRMATION NOC-05 | Privacy, legal, audit, and data-retention requirements | High |
| NEEDS_OWNER_CONFIRMATION NOC-06 | Deployment environments, promotion, rollback, and release ownership | High |
| NEEDS_OWNER_CONFIRMATION NOC-07 | Supported toolchain, browser, and contributor versions | Medium |
| NEEDS_OWNER_CONFIRMATION NOC-08 | API versioning, compatibility, and deprecation | Medium |
| NEEDS_OWNER_CONFIRMATION NOC-09 | Transaction, retry, compensation, and failure guarantees | High |
| NEEDS_OWNER_CONFIRMATION NOC-10 | Marketplace execution, moderation appeal, cleanup, and finance roadmap boundaries | High |
| NEEDS_OWNER_CONFIRMATION NOC-11 | Organization-domain verification lifecycle | Medium |
| NEEDS_OWNER_CONFIRMATION NOC-12 | Content schema, workflow, builder, session, and frontend evolution policy | Medium |
| NEEDS_OWNER_CONFIRMATION NOC-13 | Canonical documentation ownership and retirement policy | Medium |
| NEEDS_OWNER_CONFIRMATION NOC-14 | Contribution, branch, review, testing, and coverage policy | Medium |
| NEEDS_OWNER_CONFIRMATION NOC-15 | Module, reviewer, support, and incident ownership | Medium |
| NEEDS_OWNER_CONFIRMATION NOC-16 | Product license and distribution policy | Medium |
| NEEDS_OWNER_CONFIRMATION NOC-17 | Local generated/archive artifact retention policy | Low |
| NEEDS_OWNER_CONFIRMATION NOC-18 | Preferred glossary and product terminology | Low |

## DOCUMENTATION_CODE_CONFLICT Index

| ID | Conflict | Corrective destination | Priority |
| --- | --- | --- | --- |
| DOCUMENTATION_CODE_CONFLICT DCC-01 | docs/PHASE_THREE.md can imply the visual builder is still future, while Phase Four and PagesPage implement it | Add explicit historical supersession link | Medium |
| DOCUMENTATION_CODE_CONFLICT DCC-02 | docs/I18N.md understates current translation coverage while remaining incomplete | Re-audit and rewrite frontend localization coverage | Medium |
| DOCUMENTATION_CODE_CONFLICT DCC-03 | docs/V3_MARKETPLACE_DOMAIN_MODEL.md describes finance and feedback entities as future although migrations 0022 through 0025 implement them | Update current-state domain model | High |
| DOCUMENTATION_CODE_CONFLICT DCC-04 | docs/diagrams/ARCHITECTURE_AUDIT.md says Marketplace Stripe finance is absent although Phase 9 implements it | Refresh audit evidence table | High |
| DOCUMENTATION_CODE_CONFLICT DCC-05 | Diagram 00 describes both API.md and OpenAPI as stale, although the manual API reference is current except one typo and generated OpenAPI is partial | Separate manual accuracy from generated coverage | Medium |
| DOCUMENTATION_CODE_CONFLICT DCC-06 | Diagram 02 says paid Marketplace purchases are deferred, while Phase 9 implements them | Update system context | High |
| DOCUMENTATION_CODE_CONFLICT DCC-07 | Diagram 30 says install, purchase, payout, and rating are unimplemented, while Phases 6, 9, and 10 implement them | Rewrite as history or current sequences | High |
| DOCUMENTATION_CODE_CONFLICT DCC-08 | Diagram 33 treats paid entitlements as future/rejected, while Phase 9 gates paid installation with active entitlements | Update installation lifecycle | High |
| DOCUMENTATION_CODE_CONFLICT DCC-09 | docs/API.md names /api/billing/webhook in one paragraph; actual route is /api/billing/stripe/webhook | Correct manual API path | Medium |
| DOCUMENTATION_CODE_CONFLICT DCC-10 | Diagram 20 marks purchase_runtime as planned despite labeling it implemented or partial | Correct Mermaid class assignment | Medium |

## Phase-Zero Blocking Assessment

No knowledge gap blocks completion of Phase Zero because this phase records evidence and uncertainty rather than making owner decisions. Phase One can begin with repository-derived overview and glossary work. High-priority owner decisions should be resolved before the corresponding architecture, security, deployment, and operations documents are declared final.
