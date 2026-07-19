---
okf_document_id: "maintenance-review-checklist"
title: "OKF Review Checklist"
project: "ZinharCMS"
category: "maintenance"
phase: 10
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "prescriptive"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-19"
primary_sources:
  - "okf/index.yaml"
  - "okf/README.md"
  - "AGENTS.md"
related_documents:
  - "README.md"
  - "okf-update-policy.md"
  - "change-impact-matrix.md"
  - "staleness-detection.md"
  - "validation-report.md"
related_diagrams: []
---

# OKF Review Checklist

## Evidence and Scope

- [ ] Every behavioral claim has current code, configuration, migration, test, script, or appropriately ranked documentation evidence.
- [ ] No inference is labeled `VERIFIED`; uncertainty and owner decisions remain explicit.
- [ ] Source paths exist and were resolved from the repository rather than memory.
- [ ] Only authorized files changed; unrelated working-tree changes were preserved.
- [ ] Generated/dependency directories are absent from OKF.

## Structure and Metadata

- [ ] New Markdown has valid YAML frontmatter and a unique `okf_document_id`.
- [ ] Title, category, phase, status, implementation view, verification commit/date, and source-of-truth fields match the index.
- [ ] Every document and Mermaid file has exactly one index entry.
- [ ] No stale or invented commit hash is present.
- [ ] Filenames and directories follow English lower-kebab-case conventions, except established `README.md`.

## Links, Sources, and Diagrams

- [ ] All relative Markdown links, metadata relations, diagram links, and index paths resolve.
- [ ] Every source/evidence path exists.
- [ ] Every diagram is linked from Markdown, uses English labels, preserves unknowns, and passes available structural/parser checks.
- [ ] Parser/render validation is reported as unavailable if no Mermaid parser exists.

## Consistency

- [ ] Glossary terms, entity/module/feature/route/role/permission/domain/plugin/environment names match source identifiers.
- [ ] Planned/partial/unknown behavior is not presented as implemented or deployed.
- [ ] Cross-phase conflicts and unresolved owner questions are updated.
- [ ] API compatibility, database migration, security, tenancy, data lifecycle, and test references are reviewed when affected.
- [ ] Change-impact matrix consumers were considered.

## Security and Final Validation

- [ ] No password, token, private key, real connection string, private hostname, production data, or personal data was copied.
- [ ] Potential source exposure is recorded by path/category only.
- [ ] No unsupported production guarantee, SLO, backup, restore, rollback, or topology claim was added.
- [ ] YAML/frontmatter, IDs, index parity, links, paths, terminology, status, secrets, Mermaid, `git diff --check`, and `git status --short` results are recorded honestly.

## OKF Section Navigation

[Project](../project/overview.md) · [Architecture](../architecture/README.md) · [Backend](../backend/README.md) · [Frontend](../frontend/README.md) · [Database](../database/README.md) · [API](../api/README.md) · [Security](../security/README.md) · [Domain](../domain/README.md) · [Extensibility](../extensibility/README.md) · [Development](../development/README.md) · [Delivery](../delivery/README.md) · [Operations](../operations/README.md) · [Maintenance](README.md) · [References](../references/source-register.md)

