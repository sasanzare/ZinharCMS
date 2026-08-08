---
type: Development Guide
title: Documentation Maintenance
description: Repository instructions, evidence review, bundle navigation, handoff continuity, and retirement boundaries for maintained knowledge.
status: draft
sources:
  - id: source-agents
    resource: https://github.com/sasanzare/ZinharCMS/blob/eb050a0010ccd721446f5d2ac4de4863679a9564/AGENTS.md
    title: Repository handoff protocol
  - id: source-handoff
    resource: https://github.com/sasanzare/ZinharCMS/blob/eb050a0010ccd721446f5d2ac4de4863679a9564/HANDOFF.md
    title: Project handoff and migration checkpoints
  - id: source-readme
    resource: https://github.com/sasanzare/ZinharCMS/blob/eb050a0010ccd721446f5d2ac4de4863679a9564/README.md
    title: Contributor-facing repository guidance
  - id: source-okf-policy
    resource: https://github.com/sasanzare/ZinharCMS/blob/eb050a0010ccd721446f5d2ac4de4863679a9564/docs/okf-migration/OKF_TYPE_AND_METADATA_POLICY.md
    title: ZinharCMS OKF type and metadata policy
  - id: source-navigation-policy
    resource: https://github.com/sasanzare/ZinharCMS/blob/eb050a0010ccd721446f5d2ac4de4863679a9564/docs/okf-migration/NAVIGATION_AND_LINKING_POLICY.md
    title: Staging navigation and linking policy
  - id: source-validation-contract
    resource: https://github.com/sasanzare/ZinharCMS/blob/eb050a0010ccd721446f5d2ac4de4863679a9564/docs/okf-migration/OKF_VALIDATION_CONTRACT.md
    title: OKF validation contract
  - id: source-backend-ci
    resource: https://github.com/sasanzare/ZinharCMS/blob/eb050a0010ccd721446f5d2ac4de4863679a9564/.github/workflows/backend-ci.yml
    title: Backend CI workflow
  - id: source-frontend-ci
    resource: https://github.com/sasanzare/ZinharCMS/blob/eb050a0010ccd721446f5d2ac4de4863679a9564/.github/workflows/frontend-ci.yml
    title: Frontend CI workflow
---

# Responsibility

This Concept owns the repository-visible maintenance boundary for current
knowledge and the isolated Google OKF staging bundle. It covers when evidence
must be reviewed, how handoff and navigation are maintained, and how retirement
is kept separate from ordinary documentation edits. The command, test, and CI
surface remains owned by [Development and Testing](/development/development-and-testing.md);
this Concept links to that boundary instead of copying its catalog.

# Current repository mechanisms

`AGENTS.md` defines a persistent handoff protocol: a development session reads
the applicable instructions and `HANDOFF.md`, inspects Git state, preserves
existing work, records actual validation, and leaves one precise next action.
`HANDOFF.md` carries the repository state and recovery context across sessions.
These are tracked repository instructions, not executable checks that block a
commit.

`README.md` is the contributor-facing source boundary. It states that current
source code, configuration, migrations, and tests take precedence when
historical documents conflict, and that generated or dependency directories are
not source of truth. It also records that no repository-wide `CONTRIBUTING.md`,
`CODEOWNERS`, `CODE_OF_CONDUCT.md`, or ownership policy is tracked.

The migration bundle is maintained in
`docs/okf-migration/staging/google-okf-v0.2/` while the repository `okf/` and
`okf-bootstrap/` trees remain legacy material. The staging bundle is not a
canonical replacement and its reports, status matrices, ledgers, and manifests
are migration evidence rather than application behavior.

# Maintenance triggers and responses

| Change or signal | Documentation response | Classification |
| --- | --- | --- |
| Source, configuration, migration, test, script, or CI behavior changes | Re-read affected evidence, correct the owning Concept when its claims change, and review its navigation and sources. | `DOCUMENTED_POLICY` |
| A source and a document disagree | Prefer current executable or configuration evidence; preserve the unresolved boundary or historical explanation instead of guessing. | `DOCUMENTED_POLICY` |
| A staged Concept is added or materially changed | Update its direct index, the root log only for a semantic knowledge change, the phase status/ledger, and the complete staging hash snapshot. | `DOCUMENTED_POLICY` |
| Backend or frontend validation changes | Treat the checks listed by the corresponding workflow as CI evidence; do not infer documentation validation from them. | `CI_ENFORCED` |
| A legacy file might be retired or a canonical owner might change | Record the source disposition and required owner decision; do not delete, rename, redirect, or cut over from a documentation edit. | `OWNER_DECISION` |

The first three responses are maintenance policy recorded in repository
instructions and migration contracts. They are not a general-purpose
documentation bot or a repository-wide CI gate. The two GitHub Actions
workflows enforce their listed backend/frontend checks and release-version
consistency; neither workflow validates the OKF bundle, indexes, links, or
`HANDOFF.md`.

# Index, log, and source maintenance

The staged bundle follows the Google reserved-file model: `index.md` files
provide progressive disclosure, the root index declares `okf_version: "0.2"`,
and the root `log.md` records meaningful knowledge changes newest first. Local
policy requires direct child Concepts to be listed once, internal links to
resolve, and diagrams to remain embedded Mermaid rather than standalone `.mmd`
files. These are staging-quality rules; no tracked generator or automatic
staleness job is present in the repository.

Concept sources use immutable repository links at the construction commit.
Legacy `okf/index.yaml` metadata and legacy document IDs are not copied into
the target frontmatter. Bundle relationships use ordinary Markdown links, and
the migration reports retain the detailed row-level disposition and owner
decision traceability outside the bundle.

# Delivery, operations, and retirement boundary

The repository contains source-release notes, readiness scripts, Dockerfiles,
Compose definitions, health/readiness endpoints, and operational runbook
material. Those artifacts describe or implement bounded repository behavior;
they do not establish a production provider, promotion system, backup/restore
process, observability stack, retention owner, or deployment topology. Such
facts remain outside this Concept until current evidence or an owner decision
exists.

Retirement is a controlled migration action. Historical maintenance reports,
legacy metadata, owner questions, and unresolved operational claims are not
silently converted into current facts. Canonical ownership and retirement
criteria (NOC-13), contributor/review and required-check policy (NOC-14), and
Marketplace artifact retention or regeneration (NOC-17) remain unresolved.
The Concept therefore remains `draft`; it records the current maintenance
mechanisms without assigning people, approving policy, or performing cutover.
