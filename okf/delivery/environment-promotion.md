---
okf_document_id: "delivery-environment-promotion"
title: "Environment Promotion"
project: "ZinharCMS"
category: "delivery"
phase: 10
status: "current"
review_status: "mixed"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-19"
primary_sources:
  - "docker-compose.yml"
  - "docker-compose.prod.yml"
  - ".github/workflows/backend-ci.yml"
  - ".github/workflows/frontend-ci.yml"
  - ".env.example"
related_documents:
  - "deployment-workflow.md"
  - "release-process.md"
  - "../development/local-environment.md"
  - "../database/migrations.md"
  - "../security/secrets-and-configuration.md"
  - "../operations/runtime-topology.md"
related_diagrams:
  - "diagrams/deployment-flow.mmd"
---

# Environment Promotion

| Environment | Purpose | Configuration | Artifact source | Database behavior | Secret source | Trigger/approval | Observability | Data sensitivity | Confidence |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Local | Developer infrastructure and separately run apps | `.env` from templates; local Compose | Source tree/local builds | Persistent PostgreSQL volume; startup migrations | Developer-managed ignored `.env` | Manual | stdout, health/readiness | Synthetic/development expected | `VERIFIED` configuration |
| CI backend | Format/lint/test | Inline workflow environment | Checked-out source and Cargo cache | PostgreSQL service; test command, no explicit deployment | Synthetic inline values | Matching push/PR | Job logs | Synthetic | `VERIFIED` |
| CI frontend | Lint/typecheck/test/build | Workflow Node 22 | Checked-out source/npm install | None | No secret reference | Matching push/PR | Job logs | Synthetic | `VERIFIED` |
| Production-like Compose | Reference runtime assembly | Required/default Compose variables | Local source builds through production Dockerfiles | Persistent volume; startup migrations | Caller environment; mechanism unknown | Manual command not defined | Container logs and app endpoints only | Potentially sensitive | `INFERRED_FROM_CONFIGURATION`; not deployed evidence |
| Development environment | No separate tracked remote environment | N/A | N/A | N/A | N/A | N/A | N/A | Unknown | `DEPLOYMENT_TARGET_UNCLEAR` |
| Staging | Mentioned by runbooks, no tracked environment definition | N/A | N/A | Runbook intent only | N/A | N/A | N/A | Unknown | `DEPLOYMENT_TARGET_UNCLEAR` |
| Production | Mentioned by docs/Compose filename, live topology absent | N/A | N/A | Applied migration state unknown | `SECRET_INJECTION_UNCLEAR` | `NEEDS_OWNER_CONFIRMATION` | `OBSERVABILITY_STATUS_UNCLEAR` | Production-sensitive | `PRODUCTION_TOPOLOGY_UNKNOWN` |

No promotion source/target relationship, environment protection, approval gate, artifact identity propagation, configuration-diff check, or data-classification policy is implemented in tracked workflows. See [Deployment Workflow](deployment-workflow.md), [Runtime Topology](../operations/runtime-topology.md), and [Secrets](../security/secrets-and-configuration.md).

