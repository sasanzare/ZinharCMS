---
okf_document_id: "operations-readme"
title: "Operations"
project: "ZinharCMS"
category: "operations"
phase: 10
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "131c4f30583affc7a07dbcabaaa45b42c490dc27"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/main.rs"
  - "backend/src/config.rs"
  - "backend/src/routes/mod.rs"
  - "docker-compose.yml"
  - "docker-compose.prod.yml"
related_documents:
  - "../architecture/overview.md"
  - "../delivery/README.md"
  - "../database/README.md"
  - "../security/README.md"
  - "../maintenance/README.md"
related_diagrams:
  - "diagrams/runtime-topology.mmd"
  - "diagrams/observability-flow.mmd"
  - "diagrams/health-check-flow.mmd"
  - "diagrams/backup-restore-flow.mmd"
---

# Operations

## Scope and Authority

This section documents repository-evidenced runtime topology, lifecycle, configuration, dependencies, health/readiness, logging/tracing, monitoring gaps, incident signals, database operations, recovery status, troubleshooting, runbooks, and risks. It does not certify a live environment.

Repository documentation may not fully represent production. The tracked Compose configuration is production-like, while hosting, ingress, TLS, replicas, regions, traffic management, secret management, observability providers, backup systems, and operational ownership remain unknown.

## Runtime Limitations

- One Rust/Axum web process owns APIs, migrations, bootstrap seed, in-memory preview channels, and spawned webhook tasks.
- PostgreSQL is the system of record; Redis supports readiness, cache, and rate-limiting behavior.
- Media and Marketplace artifacts use a local filesystem/upload volume.
- The frontend production image is static Nginx with SPA fallback.
- No durable worker/queue, application metrics exporter, alert manager, backup automation, restore tooling, or formal disaster-recovery plan is implemented.

## Reading Order

1. [Runtime Topology](runtime-topology.md)
2. [Service Lifecycle](service-lifecycle.md)
3. [Environment Configuration](environment-configuration.md)
4. [External Dependencies](external-dependencies.md)
5. [Health and Readiness](health-and-readiness.md)
6. [Logging and Tracing](logging-and-tracing.md)
7. [Metrics and Monitoring](metrics-and-monitoring.md)
8. [Alerts and Incident Signals](alerts-and-incident-signals.md)
9. [Database Operations](database-operations.md)
10. [Backup and Restore](backup-and-restore.md)
11. [Disaster Recovery](disaster-recovery.md)
12. [Troubleshooting](troubleshooting.md)
13. [Runbook Catalog](runbook-catalog.md)
14. [Operational Risks](operational-risks.md)

Diagrams: [Runtime Topology](diagrams/runtime-topology.mmd), [Observability Flow](diagrams/observability-flow.mmd), [Health Check Flow](diagrams/health-check-flow.mmd), and [Backup/Restore Status](diagrams/backup-restore-flow.mmd).

Related context: [Architecture](../architecture/overview.md), [Delivery](../delivery/README.md), [Database](../database/README.md), and [Security](../security/README.md).

