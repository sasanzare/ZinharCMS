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
last_verified_commit: "4396b556a6e722adbdd818db9fb19074c46ee3fb"
last_verified_date: "2026-07-26"
primary_sources:
  - "package.json"
  - "backend/Cargo.toml"
  - "frontend/package.json"
  - "scripts/check-version-consistency.mjs"
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

The owner selected a GitHub source-code-only release for `v3.0.0`. This scope
publishes the Git tag, GitHub Release entry, and GitHub-generated source
archives; it does not publish binaries or container images and does not perform
or claim a production deployment.

`RELEASE_PROCESS_PARTIALLY_DEFINED`: the repository still has no automated
release workflow, changelog automation, signing process, checksum publication,
registry publication, package publication, or formal release-channel policy.

| Concern | Evidence-based status |
| --- | --- |
| Version source | Root package, backend crate, frontend package, lockfiles, Marketplace runtime, and dashboard fallback declare `3.0.0`; `npm run check:version` enforces consistency |
| Tag format | Public `v1.0.0` and `v2.0.0` tags establish `vMAJOR.MINOR.PATCH`; the approved V3 source tag is `v3.0.0` |
| Trigger | No release trigger/workflow; readiness scripts are manually invoked |
| Changelog | No root changelog found |
| Release notes | V2 notes and V3 source-release notes exist |
| V3 release scope | GitHub tag, Release entry, and GitHub-generated source archives only; production deployment is separate |
| Artifact generation | Cargo/Vite/Docker builds exist; CI does not publish them |
| Container publishing | No registry, login, tag, push, or provenance workflow |
| Package publishing | Marketplace CLI can submit product packages to an API; this is not application release publication |
| Migration compatibility | Ordered forward migrations apply during backend startup; no formal expand/contract or downgrade policy |
| Approval | The owner selected the source-only scope; final tag/Release publication still requires explicit approval |
| Signing/checksums | No application release signing/checksum process; Marketplace package CLI reports SHA-256 for its separate package artifacts |
| Channels/pre-releases | Not defined |
| Deprecation | No repository-wide API/plugin/database deprecation policy |
| Rollback | Manual runbook narrative; artifact retention and database restore evidence are absent |

## V3 Source Release Gate

The approved manual source-release sequence is:

1. Verify all version sources report `3.0.0`.
2. Require the applicable backend and frontend CI checks to pass.
3. Verify the GPLv3 license and source-release notes.
4. Require a clean, pushed `main` commit and confirm `v3.0.0` is absent.
5. Obtain explicit owner approval.
6. Create annotated tag `v3.0.0`, push it, and publish a non-prerelease GitHub
   Release using `docs/V3_MARKETPLACE_RELEASE_NOTES.md`.
7. Verify the tag target and GitHub-generated ZIP/tar source archives.

The Phase 15 target-environment readiness script, deployment evidence, support
owners, rollback owners, and monitoring checks remain separate prerequisites
for any future production General Availability claim.

See [Artifact Production](artifact-production.md), [Rollback and Recovery](rollback-and-recovery.md), [Release Flow](diagrams/release-flow.mmd), and [Runbook Catalog](../operations/runbook-catalog.md).

