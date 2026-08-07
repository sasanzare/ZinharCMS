---
type: Decision
title: Migration and Architecture Decisions
description: Phase 2 decisions that bound the Google OKF v0.2 staging construction and preserve the legacy knowledge base.
status: stable
sources:
  - id: source-phase2-decision
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/docs/okf-migration/PHASE_02_DECISION_RECORD.md
    title: PHASE_02_DECISION_RECORD.md at construction commit
  - id: source-phase2-target
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/docs/okf-migration/PHASE_02_TARGET_ARCHITECTURE.md
    title: PHASE_02_TARGET_ARCHITECTURE.md at construction commit
  - id: source-google-baseline
    resource: https://github.com/sasanzare/ZinharCMS/blob/b58840e9c227ff9d937b482eced5331122291f82/docs/okf-migration/GOOGLE_OKF_V02_BASELINE.md
    title: GOOGLE_OKF_V02_BASELINE.md at construction commit
---

# Phase 2 decisions applied to Phase 3

The target is a Google OKF v0.2 bundle with the official reserved `index.md`
and `log.md` conventions, a root version declaration, approved descriptive
types, source-backed metadata, and bundle-local navigation. The future
canonical target is isolated from the legacy `okf/` tree until a separately
authorized cutover.

Phase 3 therefore constructs only the 19 high-confidence
`CREATE_FROM_VERIFIED_KNOWLEDGE` targets. The other catalog rows are recorded
as merge, regeneration, owner-decision, or historical deferrals rather than
being represented by speculative Concepts. The staging root is
`docs/okf-migration/staging/google-okf-v0.2/`; no repository reference is
redirected to it.

The bundle uses the 19 approved descriptive type values and zero custom
frontmatter extensions. Sources point to the immutable construction commit.
Open owner decisions and unverified operational claims remain explicit in the
Phase 3 status reports.

The current system shape is documented in [system architecture](/architecture/system-architecture.md), and the complete construction matrix is maintained outside the bundle in the Phase 3 migration report.
