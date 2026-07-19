---
okf_document_id: "operations-logging-tracing"
title: "Logging and Tracing"
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
  - "backend/src/services/webhooks.rs"
  - "backend/src/error.rs"
  - "docker-compose.prod.yml"
  - "frontend/src"
related_documents:
  - "../architecture/integration-points.md"
  - "../delivery/deployment-workflow.md"
  - "../database/database-risks.md"
  - "../security/security-risks.md"
  - "metrics-and-monitoring.md"
  - "troubleshooting.md"
related_diagrams:
  - "diagrams/observability-flow.mmd"
---

# Logging and Tracing

| Concern | Verified status |
| --- | --- |
| Framework | Rust `tracing` and `tracing-subscriber` formatting subscriber |
| Levels/filter | `EnvFilter::from_default_env`; tracked templates/production-like Compose set `RUST_LOG=info` |
| Format | Default formatter; no JSON/structured-output policy configured |
| HTTP spans | `TraceLayer::new_for_http()` |
| Request IDs | Tower sets an `X-Request-Id` UUID and propagates it |
| Correlation IDs | Request ID exists; cross-service/provider correlation contract not found |
| Application events | Startup listen and bootstrap seed info; webhook load/dispatch warnings; other modules emit selected tracing events |
| Database logging | No SQL statement/query logging configuration found |
| Error logging | Startup uses contextual errors; request errors often become HTTP bodies; complete server-side error logging is not uniform |
| Security-sensitive logging | No verified redaction/filtering policy; do not claim secrets or personal data are removed |
| Plugin logging | Built-in plugins have no separate logging channel; Marketplace uploaded code is not executed |
| Frontend logging | No centralized browser telemetry/error-reporting provider; local console behavior is not an operational monitoring contract |
| Destination | Process stdout/stderr as configured by runtime/container; no collector/sink in repository |
| Retention | `LOG_RETENTION_UNKNOWN` |
| Tracing export | No OpenTelemetry/exporter/collector found |
| Tests | No end-to-end log format, redaction, request-ID correlation, collection, or retention test found |

The trace layer provides spans, not distributed tracing. Request IDs can aid manual correlation only if the deployment preserves stdout and response headers. Provider error strings, database errors, readiness messages, email delivery errors, and webhook response snippets may contain operational details; repository-wide redaction evidence is absent.

See [Observability Diagram](diagrams/observability-flow.mmd), [Security Risks](../security/security-risks.md), [Metrics](metrics-and-monitoring.md), and [Troubleshooting](troubleshooting.md).

