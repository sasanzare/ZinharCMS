---
okf_document_id: "delivery-risks"
title: "Delivery Risks"
project: "ZinharCMS"
category: "delivery"
phase: 10
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-19"
primary_sources:
  - ".github/workflows"
  - "docker-compose.prod.yml"
  - "backend/src/main.rs"
  - "docs/V2_OPERATIONS_RUNBOOK.md"
  - "docs/V3_MARKETPLACE_OPERATIONS_RUNBOOK.md"
related_documents:
  - "ci-architecture.md"
  - "release-process.md"
  - "deployment-workflow.md"
  - "rollback-and-recovery.md"
  - "../security/security-risks.md"
  - "../operations/operational-risks.md"
related_diagrams:
  - "diagrams/ci-pipeline.mmd"
  - "diagrams/deployment-flow.mmd"
---

# Delivery Risks

| Risk ID | Title | Evidence | Impact | Likelihood | Severity | Existing mitigation | Recommended follow-up | Owner confirmation required | Status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| DEL-R01 | No formal release/CD pipeline | Only two CI workflows | Manual, inconsistent releases | High | HIGH | Readiness scripts and notes | Define release trigger, approvals, identity, and publication | Yes | `RELEASE_PROCESS_UNCLEAR` |
| DEL-R02 | Production target/topology unknown | Compose only; no deploy workflow | Cannot verify security, capacity, TLS, or rollout | High | CRITICAL | Production-like reference config | Name target and encode deployment contract | Yes | `PRODUCTION_TOPOLOGY_UNKNOWN` |
| DEL-R03 | CI does not cover repository-wide changes | Path filters only backend/frontend | Root/docs/ops drift can merge unchecked | High | HIGH | Manual review | Add scoped docs/config CI gates | Yes | `VERIFIED` risk |
| DEL-R04 | Mutable action/image references | Major/stable tags, image tags without digests | Supply-chain reproducibility risk | Medium | HIGH | Lock files for app dependencies | Approve pinning/update policy | Yes | `INFERRED_FROM_WORKFLOWS` |
| DEL-R05 | No artifact publication/integrity/retention | CI uploads nothing; no registry/signing | Rollback and provenance unavailable | High | HIGH | Local builds | Define registry, immutable IDs, retention, SBOM/signing | Yes | `ROLLBACK_BEHAVIOR_UNCLEAR` |
| DEL-R06 | Migrations run in app startup | `main.rs` before bind | Multi-replica contention and incompatible rollout risk | Medium | HIGH | SQLx ordered history; startup fails closed | Decide one-shot migration vs startup policy | Yes | `INFERRED_FROM_CODE` |
| DEL-R07 | Database rollback depends on nonexistent backup process | Runbooks vs no backup tooling | Data-loss/recovery failure | High | CRITICAL | Warnings in runbooks | Implement and test backup/restore before relying on rollback | Yes | `DOCUMENTATION_CODE_CONFLICT` |
| DEL-R08 | Application health is not wired into Compose | Only DB/Redis health checks | Traffic/start order may accept unready apps | Medium | HIGH | `/health` and `/ready` endpoints exist | Add target-specific health gating in implementation phase | Yes | `HEALTH_CHECK_STATUS_UNCLEAR` |
| DEL-R09 | Secrets source/rotation unknown | Runtime variable names only | Deployment security and recovery uncertain | High | HIGH | Required variable validation and ignored `.env` | Select secret manager and rotation/access process | Yes | `SECRET_INJECTION_UNCLEAR` |
| DEL-R10 | No rollout/restart/notification policy | Compose lacks restart; workflows lack deploy notifications | Unbounded outage/manual response | Medium | HIGH | Manual runbooks | Define rollout, rollback, notification, and owner | Yes | `DEPLOYMENT_TARGET_UNCLEAR` |
| DEL-R11 | Version and release identity are weakly governed | Three `0.1.0` declarations; no tags | Artifact/source mismatch | Medium | MEDIUM | Git commit history | Define version authority and automated consistency | Yes | `RELEASE_PROCESS_UNCLEAR` |
| DEL-R12 | Frontend install not immutable | `npm install` in CI/build images | Dependency resolution drift | Medium | MEDIUM | Lockfile tracked | Decide `npm ci`/lock enforcement in a separate implementation change | Yes | `INFERRED_FROM_WORKFLOWS` |

See [Operational Risks](../operations/operational-risks.md), [Security Risks](../security/security-risks.md), [CI Architecture](ci-architecture.md), and [Deployment Workflow](deployment-workflow.md).

