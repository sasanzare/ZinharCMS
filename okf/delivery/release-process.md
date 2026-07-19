---
okf_document_id: "delivery-release-process"
title: "Release Process"
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
  - "package.json"
  - "backend/Cargo.toml"
  - "frontend/package.json"
  - "docs/V2_RELEASE_NOTES.md"
  - "docs/V3_MARKETPLACE_RELEASE_NOTES.md"
  - "scripts/v2-ga-check.ps1"
  - "scripts/marketplace-phase15-ga-check.ps1"
related_documents:
  - "artifact-production.md"
  - "deployment-workflow.md"
  - "rollback-and-recovery.md"
  - "../development/build-and-quality.md"
  - "../operations/runbook-catalog.md"
related_diagrams:
  - "diagrams/release-flow.mmd"
---

# Release Process

## Current Status

`RELEASE_PROCESS_UNCLEAR`: the repository contains milestone release notes, readiness scripts, production Dockerfiles, and a production-like Compose file, but no formal release workflow, tag convention, changelog automation, approval gate, signing process, registry publication, package publication, or release-channel definition.

| Concern | Evidence-based status |
| --- | --- |
| Version source | Root package, backend crate, and frontend package each declare `0.1.0`; no automated consistency check |
| Tag format | No Git tag and no documented tag policy found |
| Trigger | No release trigger/workflow; readiness scripts are manually invoked |
| Changelog | No root changelog found |
| Release notes | V2 and V3 Marketplace milestone notes exist |
| Artifact generation | Cargo/Vite/Docker builds exist; CI does not publish them |
| Container publishing | No registry, login, tag, push, or provenance workflow |
| Package publishing | Marketplace CLI can submit product packages to an API; this is not application release publication |
| Migration compatibility | Ordered forward migrations apply during backend startup; no formal expand/contract or downgrade policy |
| Approval | Runbooks require owners conceptually, but no tracked approval mechanism or named owner |
| Signing/checksums | No application release signing/checksum process; Marketplace package CLI reports SHA-256 for its separate package artifacts |
| Channels/pre-releases | Not defined |
| Deprecation | No repository-wide API/plugin/database deprecation policy |
| Rollback | Manual runbook narrative; artifact retention and database restore evidence are absent |

## Best-Supported Manual Gate

The strongest repository-derived release candidate sequence is: run component CI-equivalent checks, use the relevant GA/readiness script, review migrations and operational unknowns, build intended artifacts, and obtain explicit owner approval. That is a documented synthesis, not evidence that a production release has occurred.

See [Artifact Production](artifact-production.md), [Rollback and Recovery](rollback-and-recovery.md), [Release Flow](diagrams/release-flow.mmd), and [Runbook Catalog](../operations/runbook-catalog.md).

