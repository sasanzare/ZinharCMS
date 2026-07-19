---
okf_document_id: "maintenance-readme"
title: "OKF Maintenance"
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
  - "AGENTS.md"
  - "okf/index.yaml"
  - "okf/README.md"
  - "okf-bootstrap/phase-zero-summary.md"
related_documents:
  - "okf-update-policy.md"
  - "documentation-ownership.md"
  - "change-impact-matrix.md"
  - "review-checklist.md"
  - "staleness-detection.md"
  - "validation-report.md"
  - "cross-phase-conflicts.md"
  - "unresolved-owner-questions.md"
  - "final-completion-report.md"
related_diagrams: []
---

# OKF Maintenance

## Purpose

OKF must evolve with executable code, migrations, configuration, tests, scripts, and operational evidence. A stale knowledge base can make developers and AI agents act on incorrect boundaries, so implementation changes should normally include affected OKF updates in the same pull request.

## Maintenance Model

- Section maintainers update evidence-based details; ownership classes are mapped in [Documentation Ownership](documentation-ownership.md).
- Review OKF whenever architecture, modules, routes, contracts, migrations, security, business rules, plugins, build/CI, environment, deployment, or operations change.
- AI coding agents must inspect evidence, preserve unknowns/conflicts, update index metadata/relations, run the [Review Checklist](review-checklist.md), and report unavailable validation.
- When documentation and implementation conflict, executable evidence wins for current behavior and the conflict remains in [Cross-Phase Conflicts](cross-phase-conflicts.md) until the source narrative is corrected or an owner decides otherwise.
- Detect drift through [Staleness Detection](staleness-detection.md); mark unresolved drift rather than guessing.
- Consolidate product/policy decisions in [Owner Questions](unresolved-owner-questions.md), then update all affected documents when answered.
- Add future documents with unique English paths/IDs, YAML metadata, source evidence, relations, index registration, and validation.

## Reading Order

1. [OKF Update Policy](okf-update-policy.md)
2. [Documentation Ownership](documentation-ownership.md)
3. [Change Impact Matrix](change-impact-matrix.md)
4. [Review Checklist](review-checklist.md)
5. [Staleness Detection](staleness-detection.md)
6. [Validation Report](validation-report.md)
7. [Cross-Phase Conflicts](cross-phase-conflicts.md)
8. [Unresolved Owner Questions](unresolved-owner-questions.md)
9. [Final Completion Report](final-completion-report.md)

## Current Final Status

The final status is `OKF_COMPLETE_WITH_OPEN_QUESTIONS`. Structural validation has no known broken links, invalid evidence paths, duplicate IDs, or index mismatches. Meaningful product and production questions remain, and Mermaid parser/render validation remains unavailable because no approved parser exists in the repository or environment.

## OKF Section Navigation

[Project](../project/overview.md) · [Architecture](../architecture/README.md) · [Backend](../backend/README.md) · [Frontend](../frontend/README.md) · [Database](../database/README.md) · [API](../api/README.md) · [Security](../security/README.md) · [Domain](../domain/README.md) · [Extensibility](../extensibility/README.md) · [Development](../development/README.md) · [Delivery](../delivery/README.md) · [Operations](../operations/README.md) · [Maintenance](README.md) · [References](../references/source-register.md)

