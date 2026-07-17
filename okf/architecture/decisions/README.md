---
okf_document_id: "architecture-decisions-readme"
title: "Architecture Decisions"
project: "ZinharCMS"
category: "architecture"
phase: 2
status: "current"
review_status: "verified"
source_of_truth: false
architecture_status: "inferred"
last_verified_commit: "17e69e266c558c8568ec65524560d52d7cb89d4c"
last_verified_date: "2026-07-17"
primary_sources:
  - "backend/src/main.rs"
  - "backend/src/routes/mod.rs"
  - "backend/src/state.rs"
  - "frontend/src/main.tsx"
  - "frontend/src/services/api.ts"
  - "docker-compose.prod.yml"
  - "docs/V3_MARKETPLACE_SCOPE.md"
related_documents:
  - "architecture/README.md"
  - "architecture/overview.md"
  - "architecture/dependency-model.md"
  - "architecture/architecture-risks.md"
  - "architecture/decisions/decision-register.md"
related_diagrams:
  - "architecture/diagrams/system-context.mmd"
  - "architecture/diagrams/container-view.mmd"
  - "architecture/diagrams/dependency-direction.mmd"
uncertainty_markers:
  - "INFERRED_FROM_CODE"
  - "INFERRED_FROM_STRUCTURE"
  - "INFERRED_FROM_CONFIGURATION"
  - "UNKNOWN"
  - "PROPOSED_NOT_IMPLEMENTED"
---

# Architecture Decisions

## Purpose

An architectural decision is a consequential choice about system structure, runtime boundaries, dependency direction, persistence, integration, deployment shape, or another cross-cutting constraint. This directory records choices that can be reconstructed from current implementation, configuration, and governing documentation.

Decisions are discovered by tracing repeated implementation patterns, composition roots, manifests, executable configuration, migrations, imports, runtime calls, and explicit scope statements. The repository does not contain a complete historical ADR set, so most entries in the register are inferred decisions, not claims about original intent or rationale.

## Register

Use the [Architecture Decision Register](decision-register.md) for the current decision inventory, evidence, consequences, confidence, and review triggers.

## Explicit and Inferred Decisions

An `EXPLICIT` decision is stated by a governing source and agrees with current implementation. An `INFERRED_FROM_CODE` or `INFERRED_FROM_CONFIGURATION` decision describes the structure that the current repository implements without claiming why it was selected. `DOCUMENTATION_ONLY` identifies a documented choice without sufficient implementation confirmation. `PROPOSED_NOT_IMPLEMENTED` identifies a target choice that current implementation does not yet realize.

Inferred decisions need owner confirmation before their historical rationale, permanence, or preferred future direction is treated as established. The implementation itself can be observed with high confidence while its intent remains unknown.

## Status Rules

| Status | Meaning |
|---|---|
| `ACCEPTED` | An explicit governing source and current implementation agree |
| `OBSERVED` | Current code or executable configuration implements the choice |
| `PROPOSED` | A target choice is recorded but is not yet the current architecture |
| `SUPERSEDED` | A later accepted or observed choice replaced the entry |
| `DEPRECATED` | The choice remains visible but is scheduled for removal |
| `UNCLEAR` | Available evidence cannot establish the current choice |

`OBSERVED` does not mean the choice was formally approved. For an inferred decision, the register states `Decision rationale: UNKNOWN` unless an explicit governing source supplies it. Phase 2 found no architecture proposal that needed a `PROPOSED_NOT_IMPLEMENTED` decision entry.

## Adding Future ADRs

Add a standalone ADR when a future architecture choice is intentionally proposed or accepted. Give it a stable ID, context, status, decision type, observed or proposed decision, evidence, consequences, documented alternatives, confidence, owner confirmation, related components, and links. Add it to the register without rewriting the historical meaning of earlier entries.

When a decision is replaced, mark the previous record `SUPERSEDED`, link the replacement, and preserve both records. Do not delete the earlier decision or silently convert a proposal into an observed fact.

## Change Policy

When architecture changes:

1. verify the implementation and configuration at the new commit;
2. update the existing register entry or add a new unique ID;
3. distinguish implementation evidence from rationale and owner intent;
4. link affected risks, documents, and diagrams;
5. do not mark an inferred decision as explicitly accepted without evidence;
6. preserve superseded entries for traceability.

This register documents the current system. It does not authorize application changes.
