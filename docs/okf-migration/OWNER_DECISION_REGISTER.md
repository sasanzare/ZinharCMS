# Owner Questions and Decision Debt Register

This register preserves unresolved questions and policy decisions that cannot
be answered safely from repository code, migrations, tests, configuration, or
legacy documentation. Unknowns remain visible even when they are not suitable
for an authoritative Google OKF concept.

Status vocabulary follows the Phase 1 brief: still unresolved, resolved by
current implementation, obsolete, historical, and requires owner decision.
“Still unresolved” is used for an active question whose answer must come from
an accountable owner; “requires owner decision” is used for an evidence gap
that cannot be closed by more repository inspection.

## Owner decision register

| ID | Decision debt / question | Status | Priority | Why repository evidence is insufficient | Affected future concepts | Required resolution evidence |
| --- | --- | --- | --- | --- | --- | --- |
| NOC-01 | What production rule selects a public organization: current default slug, host/custom domain, route parameter, or another mapping? | still unresolved | High | Current code demonstrates the default organization rule but not product intent. | TenantIsolation; PublicDelivery; API Contract | Owner-approved routing policy and tests/configuration for the selected rule. |
| NOC-02 | Should media and Marketplace artifacts use instance-local storage, shared filesystem, object storage, or CDN, and which assets require authorization? | still unresolved | High | Repository behavior is local-filesystem/reference topology only. | Media; Marketplace; Operations and Deployment | Storage architecture, asset classification, access policy, and deployment configuration. |
| NOC-03 | What backup schedule, restore procedure, RPO, RTO, retention, and restore-test cadence apply to PostgreSQL and stored files? | still unresolved | High | No platform backup policy or restore evidence is tracked. | Database Model; Operations and Deployment | Approved recovery policy plus dated restore-test evidence. |
| NOC-04 | Which logging, metrics, tracing, dashboards, alerts, SLOs, and on-call roles are authoritative in production? | still unresolved | High | Source tracing and readiness probes do not establish a production observability system. | Operations and Deployment; Security Posture | Monitoring configuration, SLOs, alert ownership, and escalation record. |
| NOC-05 | Which privacy, data-residency, audit, security-log, artifact, billing, and user-data retention requirements apply? | still unresolved | High | Legal and product policy are outside the repository. | Security Posture; Database Model; Marketplace | Approved policy, retention schedule, deletion/legal-hold rules, and owner. |
| NOC-06 | What environments, promotion gates, rollback method, release cadence, and release authority are intended? | still unresolved | High | CI contains quality gates but no production deployment contract. | Operations and Deployment; Development and Testing | Environment matrix, promotion workflow, rollback runbook, and accountable owner. |
| NOC-07 | Which Rust, Node, package-manager, database, Redis, browser, and operating-system versions are officially supported? | requires owner decision | Medium | Docker and CI expose implementation versions but not a support policy. | Development and Testing; Frontend; Operations and Deployment | Published support matrix and update policy. |
| NOC-08 | What compatibility, versioning, deprecation, and support-window policy applies to administrative, public, WebSocket, and webhook contracts? | requires owner decision | Medium | Route prefixes and current contracts are history, not a compatibility promise. | API Contract; PublicDelivery | Versioning policy, deprecation rules, and consumer support window. |
| NOC-09 | What guarantees are required when mutation succeeds but cache invalidation, webhook delivery, file cleanup, email, or another side effect fails? | still unresolved | High | Current code shows transaction and task boundaries but cannot choose desired compensation/retry semantics. | Architecture; API Contract; Marketplace; Operations | Decision record with retry, compensation, idempotency, and user-visible error policy. |
| NOC-10 | Which Marketplace capabilities are out of scope or planned: external package execution, appeals/restoration, cleanup automation, refunds, disputes, tax, transfers, and payout settlement? | still unresolved | High | Current code proves implemented boundaries, not roadmap commitments. | Marketplace; Extensibility; Security Posture | Approved scope matrix and security/finance policy. |
| NOC-11 | What provider/process verifies organization domains, how does renewal/failure work, and when does verification affect routing? | requires owner decision | Medium | Schema/routes do not establish a complete production verification lifecycle. | TenantIsolation; PublicDelivery; Operations | Verification lifecycle specification and integration evidence. |
| NOC-12 | What policies govern content schema evolution, workflow meanings, page-component compatibility, browser session recovery, accessibility, and frontend decomposition? | requires owner decision | Medium | Code provides current mechanics but not long-term product/engineering policy. | Content Workflow; Page Builder; Frontend; Authentication | Approved evolution and compatibility policies plus test obligations. |
| NOC-13 | Which document is canonical for each domain, who owns it, and when are historical/conflicting documents corrected, superseded, or archived? | still unresolved | Medium | Git and filenames do not assign durable documentation authority. | Project; Maintenance; Provenance | Owner map, canonical paths, retirement criteria, and review triggers. |
| NOC-14 | What branch, commit, review, required-check, coverage, browser-E2E, migration-test, and documentation-update policy should contributors follow? | requires owner decision | Medium | CI and recent commits expose patterns but no complete contribution contract. | Development and Testing; Maintenance | Published contribution policy and enforced checks. |
| NOC-15 | Who owns each product module, Marketplace review, support, security response, production operations, and documentation approval? | still unresolved | Medium | No CODEOWNERS, team directory, or escalation roster is tracked. | Project; Security; Operations; Maintenance | Accountable owner/role map with escalation contacts. |
| NOC-16 | What product and repository license/distribution terms are authoritative? | resolved by current implementation | Medium | The original bootstrap question was resolved after the historical audit. | Project; Historical Audit Trail | LICENSE and aligned package metadata; retain the original question as historical evidence. |
| NOC-17 | Should ignored Marketplace distribution archives and generated local samples be retained, reproducibly regenerated, or removed? | requires owner decision | Low | Ignore rules and local artifacts do not define lifecycle intent. | Marketplace; Maintenance; Provenance | Artifact retention/regeneration policy. |
| NOC-18 | Which product terms and abbreviations are preferred when code, UI, and historical documents use alternatives? | requires owner decision | Low | Usage frequency does not establish owner-approved terminology. | Project; all typed concepts | Approved glossary and naming decision. |

## Unknown facts carried forward

These are the active evidence-gap labels from okf-bootstrap/09-knowledge-gaps.md.
They are not silently treated as decisions:

| Unknown | Current classification | Link to owner debt |
| --- | --- | --- |
| U-01 production hosting, ingress, TLS, and network boundaries | requires owner decision | NOC-06 |
| U-02 applied migration version and deployed schema drift | requires owner decision | NOC-03; NOC-06 |
| U-03 production logs, metrics, tracing, dashboards, alerts, and SLOs | requires owner decision | NOC-04 |
| U-04 backup schedule, restore, RPO, RTO, retention, and tests | requires owner decision | NOC-03 |
| U-05 production media and Marketplace artifact storage/delivery | requires owner decision | NOC-02 |
| U-06 environment promotion, release cadence, rollback authority, and incidents | requires owner decision | NOC-06 |
| U-07 privacy, residency, audit, and legal requirements | requires owner decision | NOC-05 |
| U-08 public tenant selection and custom-domain delivery | requires owner decision | NOC-01; NOC-11 |
| U-09 supported Rust, Node, browser, and contributor versions | requires owner decision | NOC-07 |
| U-10 product license/distribution | resolved by current implementation | NOC-16 |
| U-11 local Marketplace archive retention | requires owner decision | NOC-17 |
| U-12 code ownership and merge policy | requires owner decision | NOC-13; NOC-14; NOC-15 |
| U-13 measured production capacity and scaling thresholds | requires owner decision | NOC-04; NOC-06 |
| U-14 production email provider, retries, and failure operations | requires owner decision | NOC-04; NOC-09 |
| U-15 support and security escalation contacts | requires owner decision | NOC-15 |

## Completion rule

The open owner-decision count is 17: every NOC except NOC-16. Phase 2 may
design candidate boundaries around these questions, but it must not convert
them into stable current-state claims or guessed Google OKF metadata.
