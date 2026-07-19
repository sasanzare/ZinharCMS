---
okf_document_id: "delivery-container-builds"
title: "Container Builds"
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
  - "backend/Dockerfile"
  - "backend/Dockerfile.prod"
  - "frontend/Dockerfile"
  - "frontend/Dockerfile.prod"
  - "frontend/nginx.conf"
  - "docker-compose.yml"
  - "docker-compose.prod.yml"
related_documents:
  - "artifact-production.md"
  - "deployment-workflow.md"
  - "../development/prerequisites.md"
  - "../security/secrets-and-configuration.md"
  - "../operations/runtime-topology.md"
related_diagrams:
  - "diagrams/deployment-flow.mmd"
---

# Container Builds

| Image definition | Context/stages | Base images | Runtime user | Port/command | Configuration and storage | Health check | Tags/registry/platform/cache |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `backend/Dockerfile` | Single stage; copies manifest, lock, migrations, source; debug build | `rust:1.87-bookworm` | Root/default | `8080`; `cargo run` | Runtime environment external; source/migrations inside image | None | Not defined; build cache follows Docker layers |
| `backend/Dockerfile.prod` | Rust builder then Debian runtime | `rust:1.87-bookworm`, `debian:bookworm-slim` | System user `cms` UID 10001 | `8080`; `/app/cms-backend` | `/app/uploads` created; Compose mounts volume; migrations copied | None | Not defined; target platform not declared |
| `frontend/Dockerfile` | Single Node development stage | `node:24-alpine` | Root/default | `5173`; Vite dev server | Installs with `npm install`; source copied | None | Not defined |
| `frontend/Dockerfile.prod` | Node builder then Nginx runtime | `node:24-alpine`, `nginx:1.27-alpine` | Image default | `80`; `nginx -g daemon off;` | `VITE_API_URL` build argument; static bundle; SPA fallback | None | Not defined; no registry/push |

## Compose Relationships

Local Compose uses PostgreSQL, Redis, and pgAdmin only. Production-like Compose builds the production backend and frontend images, waits for PostgreSQL and Redis health before starting backend, mounts named database/cache/upload volumes, and exposes backend `8080` and frontend host port `5173` to container port `80`. Frontend depends on backend start order but not a backend health condition.

## Security and Reproducibility

- Production backend runs as non-root; other custom application stages do not declare a non-root user.
- Base images use mutable tags rather than digests.
- Frontend dependency installation uses `npm install`; backend uses the tracked Cargo lock.
- No image scan, SBOM, signature, provenance, registry credentials, push step, target architecture, resource limit, read-only filesystem, capability drop, or application container health check is defined.
- Secret values are injected at runtime/build invocation, not baked by these tracked files; `VITE_API_URL` is intentionally browser-visible.

These images are buildable definitions, not evidence of production hardening or deployment. See [Deployment Workflow](deployment-workflow.md), [Runtime Topology](../operations/runtime-topology.md), and [Secrets](../security/secrets-and-configuration.md).

