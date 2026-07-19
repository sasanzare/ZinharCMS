---
okf_document_id: "operations-service-lifecycle"
title: "Service Lifecycle"
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
  - "backend/src/db/mod.rs"
  - "backend/src/routes/mod.rs"
  - "frontend/Dockerfile.prod"
  - "frontend/nginx.conf"
  - "docker-compose.prod.yml"
related_documents:
  - "runtime-topology.md"
  - "health-and-readiness.md"
  - "../delivery/database-deployment.md"
  - "../architecture/runtime-flows.md"
  - "../security/secrets-and-configuration.md"
  - "troubleshooting.md"
related_diagrams:
  - "diagrams/health-check-flow.mmd"
---

# Service Lifecycle

## Backend Initialization

1. Load a nearby `.env` file when available.
2. Initialize `tracing_subscriber::fmt` with `EnvFilter::from_default_env`.
3. Parse and validate environment configuration.
4. Parse `DATABASE_URL` and create a lazy PostgreSQL pool with a maximum of 10 connections.
5. Connect as needed and run embedded SQLx migrations.
6. Seed bootstrap identity/membership data only when the users table is empty.
7. Construct a Redis client and `AppState`, including in-memory page-preview broadcast channels.
8. Register public, authenticated, tenant-protected, static upload, middleware, timeout, compression, CORS, request-ID, and trace layers.
9. Bind `0.0.0.0:<PORT>` and begin serving.

Configuration, migration, seed, Redis URL parsing, bind, or server failure returns an error and exits startup. Redis reachability itself is checked by `/ready`, not by client construction.

## Background Work

CMS webhook deliveries are spawned with `tokio::spawn` inside the web process. Preview channels are in-memory. No durable queue, independent worker lifecycle, retry supervisor, or shutdown drain contract was found. Process termination can lose in-flight spawned work (`PARTIALLY_DEFINED`).

## Health Transition

`/health` becomes reachable after bind and reports process/application version without dependency checks. `/ready` queries PostgreSQL and pings Redis on every request. Compose does not use backend readiness as a container health check.

## Shutdown and Restart

The backend listens for Ctrl+C everywhere and SIGTERM on Unix, then passes a graceful shutdown future to Axum. No explicit database-pool close, Redis close, webhook drain, or preview broadcast cleanup is coded; resource owners drop with process shutdown. Restart policy, maximum grace period, and orchestrator behavior are not defined.

## Frontend Lifecycle

The development frontend uses Vite. The production-like frontend starts Nginx in the foreground and serves a prebuilt SPA with fallback. No custom initialization, health check, graceful-shutdown hook, runtime configuration reload, or application state migration exists in frontend source.

See [Health and Readiness](health-and-readiness.md), [Database Deployment](../delivery/database-deployment.md), [Runtime Flows](../architecture/runtime-flows.md), and [Troubleshooting](troubleshooting.md).

