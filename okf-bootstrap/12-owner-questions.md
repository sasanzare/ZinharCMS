# Owner Questions

## Usage

These questions cannot be answered reliably from repository code, migrations, tests, or existing documents. They are not blockers for repository-derived Phase One work. Each answer should be recorded with decision date, decision owner, rationale, and affected OKF paths.

## High-Priority Questions

| ID | Question | Why the repository cannot answer it | Affected OKF areas | Can Phase One start without it? |
| --- | --- | --- | --- | --- |
| NEEDS_OWNER_CONFIRMATION NOC-01 | What is the intended production rule for selecting a public organization: the current default slug, request host/custom domain, route parameter, or another mapping? | Code proves only the current default-slug behavior; it does not establish product intent | Architecture, delivery API, multi-tenancy, domains, security | Yes; keep production routing marked draft |
| NEEDS_OWNER_CONFIRMATION NOC-02 | Should production media and Marketplace artifacts use instance-local storage, shared filesystem, object storage, or a CDN, and which assets must require authorization? | The repository implements local paths and public upload serving but has no production storage configuration | Architecture, media, Marketplace, upload security, deployment, backup | Yes; document only reference behavior |
| NEEDS_OWNER_CONFIRMATION NOC-03 | What backup schedule, restore procedure, RPO, RTO, retention period, and restore-test cadence are required for PostgreSQL and stored files? | No platform backup policy or restore evidence is in the repository | Database, deployment, backup/restore, troubleshooting | Yes; do not state recovery guarantees |
| NEEDS_OWNER_CONFIRMATION NOC-04 | Which logging, metrics, tracing, dashboards, alerts, SLOs, and on-call roles are authoritative in production? | Code contains tracing and probes but no collector, metrics backend, dashboards, or ownership configuration | Architecture, observability, incident response, troubleshooting | Yes; keep observability UNKNOWN |
| NEEDS_OWNER_CONFIRMATION NOC-05 | Which privacy, data residency, audit, security-log, artifact, billing, and user-data retention requirements apply? | These are legal/product policies outside executable behavior | Database retention, security, operations, Marketplace finance, deletion | Yes; do not invent retention periods |
| NEEDS_OWNER_CONFIRMATION NOC-06 | What environments, deployment mechanism, promotion gates, rollback method, release cadence, and release authority are intended? | CI stops at quality checks; no production deployment configuration exists | Deployment, development workflow, operations, architecture | Yes; separate local/reference topology from production |
| NEEDS_OWNER_CONFIRMATION NOC-09 | What guarantees are required when a database mutation succeeds but cache invalidation, webhook delivery, file cleanup, email, or another side effect fails? | Code shows current transaction and task boundaries but cannot define desired compensation or retry policy | Architecture, business rules, webhooks, media, Marketplace, operations | Yes; record observed behavior and gaps |
| NEEDS_OWNER_CONFIRMATION NOC-10 | Which Marketplace capabilities are intentionally out of scope or planned: external package code execution, moderation appeal/restoration, artifact cleanup automation, partial refunds, disputes, tax handling, transfers, and payout settlement? | Current code proves the implemented boundary but not roadmap commitments | Marketplace domain, runtime policy, security, finance, operations | Yes; label current and future scope separately |

## Medium-Priority Questions

| ID | Question | Why the repository cannot answer it | Affected OKF areas | Can Phase One start without it? |
| --- | --- | --- | --- | --- |
| NEEDS_OWNER_CONFIRMATION NOC-07 | Which Rust, Node.js, package-manager, database, Redis, browser, and operating-system versions are officially supported? | Docker and CI use different Rust/Node policies and no support matrix exists | Local setup, contribution, deployment, frontend compatibility | Yes |
| NEEDS_OWNER_CONFIRMATION NOC-08 | What API compatibility, versioning, deprecation, and support-window policy should apply to administrative, public, WebSocket, and webhook contracts? | Current prefixes show implementation history, not a policy | API overview, route index, versioning, client integration | Yes; versioning remains draft |
| NEEDS_OWNER_CONFIRMATION NOC-11 | What verification provider/process, renewal behavior, failure handling, and routing effect should organization domains have? | The schema and routes do not establish a complete production verification lifecycle | Organizations, multi-tenancy, delivery, operations | Yes |
| NEEDS_OWNER_CONFIRMATION NOC-12 | What are the intended policies for content-schema evolution, workflow state meaning, page-component compatibility, browser session recovery, accessibility, and frontend component decomposition? | Code provides current mechanics but these long-term product/engineering rules require deliberate choices | CMS modules, page builder, frontend, API, business rules | Yes; document current mechanics only |
| NEEDS_OWNER_CONFIRMATION NOC-13 | Which existing documents are canonical for each domain, who owns them, and when should historical or conflicting documents be corrected, superseded, or archived? | Git history and filenames do not assign durable documentation authority | Project documentation map, references, all OKF maintenance | Yes; use code/migrations as current technical authority |
| NEEDS_OWNER_CONFIRMATION NOC-14 | What branch, commit, review, required-check, test-coverage, browser-E2E, migration-test, and documentation-update policy should contributors follow? | CI and recent commits expose patterns but no contribution contract | Development, testing, contribution workflow | Yes; label patterns as inferred |
| NEEDS_OWNER_CONFIRMATION NOC-15 | Who owns each product module, Marketplace review, support, security response, production operations, and final documentation approval? | No CODEOWNERS, team directory, or escalation roster is present | Module index, operations, incidents, owner decisions | Yes; owner fields remain unknown |
| NEEDS_OWNER_CONFIRMATION NOC-16 | What product and repository license/distribution terms should be authoritative? | Dependency metadata is not a root product license | Project overview, contribution, distribution | Yes; do not infer a repository license |

## Low-Priority Questions

| ID | Question | Why the repository cannot answer it | Affected OKF areas | Can Phase One start without it? |
| --- | --- | --- | --- | --- |
| NEEDS_OWNER_CONFIRMATION NOC-17 | Should ignored Marketplace distribution archives and generated local samples be retained, reproducibly regenerated, or removed after use? | Ignore rules and local artifacts do not define lifecycle intent | Repository map, creator tooling, troubleshooting | Yes |
| NEEDS_OWNER_CONFIRMATION NOC-18 | Which product terms and abbreviations are preferred when code, UI, and historical documents use alternatives? | Usage frequency does not establish owner-approved terminology | Glossary and all user-facing knowledge documents | Yes; mark disputed terms |

## Recommended Answer Order

1. Resolve NOC-01 through NOC-06 before finalizing architecture and operations claims.
2. Resolve NOC-09 and NOC-10 before finalizing failure semantics and Marketplace roadmap boundaries.
3. Resolve NOC-07, NOC-08, and NOC-11 through NOC-16 before final validation of development, API, ownership, and governance documents.
4. Resolve NOC-17 and NOC-18 when repository hygiene and glossary work require them.

## Decision Recording Template

| Field | Required value |
| --- | --- |
| Decision ID | Existing NOC identifier |
| Decision | Direct answer, including exclusions |
| Owner | Accountable person or role |
| Date | ISO date |
| Rationale | Short reason and trade-off |
| Evidence | Policy, platform configuration, ticket, or other authoritative source |
| Affected paths | OKF and existing documents that must change |
| Review trigger | Date, release, architecture change, or policy event |
