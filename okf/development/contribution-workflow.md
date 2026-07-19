---
okf_document_id: "development-contribution-workflow"
title: "Contribution Workflow"
project: "ZinharCMS"
category: "development"
phase: 10
status: "current"
review_status: "mixed"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-19"
primary_sources:
  - "AGENTS.md"
  - ".github/workflows/backend-ci.yml"
  - ".github/workflows/frontend-ci.yml"
  - ".gitignore"
  - "backend/migrations"
related_documents:
  - "development-workflow.md"
  - "build-and-quality.md"
  - "../maintenance/review-checklist.md"
  - "../maintenance/documentation-ownership.md"
  - "../security/security-risks.md"
related_diagrams: []
---

# Contribution Workflow

No `CONTRIBUTING.md`, `CODEOWNERS`, issue template, pull-request template, root security policy, approval count, merge strategy, branch protection, or required-check declaration is tracked. Governance beyond the observable items below remains `NEEDS_OWNER_CONFIRMATION NOC-14/NOC-15`.

| Topic | Verified or inferred practice | Classification |
| --- | --- | --- |
| Branch naming | Current OKF work uses `docs/okf-phase-zero`; no general repository branch-naming policy was found | `INFERRED_CONVENTION` |
| Commit subjects | Recent history uses conventional-style scopes such as `docs(okf):` and `feat(marketplace):` | `INFERRED_CONVENTION`; no commit hook/linter |
| Pull requests | Backend/frontend workflows run on PRs when their path filters match | `EXPLICIT_CONVENTION` |
| Review expectations | No tracked reviewer or approval policy | `NEEDS_OWNER_CONFIRMATION` |
| Required checks | Workflow jobs exist, but external branch-protection requirements are not observable | `CI_BEHAVIOR_UNCLEAR` |
| Documentation | `AGENTS.md` requires handoff maintenance; this OKF policy requires same-change documentation review | `EXPLICIT_CONVENTION` for agents; owner adoption unconfirmed |
| Testing | Run the smallest relevant check and repository CI-equivalent gates for changed areas | `EXPLICIT_CONVENTION` from workflows and project protocol |
| Migrations | Add ordered SQL, review tenant/RLS and compatibility, never assume rollback | `INFERRED_CONVENTION` from migrations/startup |
| Backward compatibility | No formal API, schema, plugin, or browser support window | `NEEDS_OWNER_CONFIRMATION NOC-08` |
| Security review | Security-sensitive changes require review, but no named reviewer/process is tracked | `NEEDS_OWNER_CONFIRMATION` |
| Release notes | V2/V3 release notes exist for major milestones; no universal requirement | `INFERRED_CONVENTION` |
| Issues/PR templates | Not found | `NOT_IMPLEMENTED` |

## Evidence-Based Preparation Checklist

1. Keep changes scoped and preserve unrelated work.
2. Update tests and the affected OKF documents when behavior changes.
3. For migrations, record forward behavior, compatibility, failure, and recovery uncertainty.
4. Run format/lint/type-check/test/build commands appropriate to changed paths.
5. Review security, tenancy, API compatibility, and secret exposure where relevant.
6. Inspect status, diff, and whitespace before requesting review.
7. State which checks ran, failed, or were unavailable.
8. Do not stage, commit, publish, deploy, or mutate production without explicit authorization.

See [Development Workflow](development-workflow.md), [Review Checklist](../maintenance/review-checklist.md), and [Documentation Ownership](../maintenance/documentation-ownership.md).
