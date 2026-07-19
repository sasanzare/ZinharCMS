---
okf_document_id: "delivery-artifact-production"
title: "Artifact Production"
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
  - "backend/Dockerfile.prod"
  - "frontend/Dockerfile.prod"
  - "frontend/package.json"
  - "scripts/marketplace-cli.mjs"
  - ".github/workflows"
related_documents:
  - "container-builds.md"
  - "release-process.md"
  - "../development/build-and-quality.md"
  - "../database/migrations.md"
  - "../operations/runtime-topology.md"
related_diagrams:
  - "diagrams/release-flow.mmd"
---

# Artifact Production

| Artifact | Source | Build command | Environment | Destination | Versioning | Retention | Consumer | Integrity verification | Confidence |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Debug backend binary | Backend crate | `cargo build` | Rust toolchain | `backend/target/debug` | Crate `0.1.0`; file path not versioned | Local Cargo behavior; ignored | Developer | Cargo compilation only | `VERIFIED` |
| Release backend binary | Backend crate | `cargo build --release` | Rust 1.87 builder in production Dockerfile | Builder `target/release/cms-backend`; copied into image | Crate `0.1.0`; image tag undefined | Not defined | Backend runtime image | Docker build/Cargo; no checksum/signing | `VERIFIED` build, retention unknown |
| Frontend bundle | Frontend source | `npm run build` | Node/npm; Node 24 in production builder | `frontend/dist`; copied to Nginx image | Frontend `0.1.0`; filenames may be hashed by Vite | CI output is ephemeral; local ignored | Nginx/static host | Build success; no published checksum | `VERIFIED` |
| Backend image | Production backend Dockerfile | Compose/Docker build | Docker | Local image store unless caller tags it | No image tag policy | Unknown | Container runtime | No signing/SBOM/provenance | `VERIFIED` definition; destination unknown |
| Frontend image | Production frontend Dockerfile | Compose/Docker build | Docker | Local image store unless caller tags it | No image tag policy | Unknown | Container runtime | No signing/SBOM/provenance | `VERIFIED` definition; destination unknown |
| Embedded migrations | `backend/migrations` | Compiled by SQLx macro into backend binary; also copied into images | Cargo/Docker | Binary plus image migration directory | Ordered filenames | Source history/image retention | Backend startup migrator | SQLx migration checksums/history | `VERIFIED` |
| OpenAPI document | Backend `ApiDoc` | Runtime `GET /openapi.json` | Running backend | HTTP response only | Backend crate version; no persisted artifact | None defined | API consumers | Known route/security omissions | `PARTIALLY_DEFINED` |
| Marketplace package ZIP | Creator package directory | `npm run marketplace -- pack ...` | Node | `marketplace-dist` by default | Manifest semantic version in filename | Ignored; policy unknown | Marketplace submission | CLI reports SHA-256 | `VERIFIED`; not an app release artifact |
| Documentation package | Markdown/Mermaid source | No packaging command | N/A | Git tree | Commit history | Git retention | Humans/agents | Phase 10 validation only | `NOT_IMPLEMENTED` as a packaged artifact |
| CI test/build reports | CI job output | Workflow commands | GitHub runner | Logs only | Workflow run | Provider settings unknown | Reviewers | Job result | `VERIFIED`; no uploaded artifacts |

No application source archive workflow, container registry destination, release checksum file, signature, SBOM, provenance attestation, or artifact retention policy was found.

See [Container Builds](container-builds.md), [Release Process](release-process.md), [Migrations](../database/migrations.md), and [Runtime Topology](../operations/runtime-topology.md).

