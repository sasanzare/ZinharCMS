---
okf_document_id: "backend-configuration-and-state"
title: "Backend Configuration and Application State"
project: "ZinharCMS"
category: "backend"
phase: 3
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "debde2021c029d1827abaa38bcc32c682f53f55a"
last_verified_date: "2026-07-17"
primary_sources:
  - "backend/src/config.rs"
  - "backend/src/state.rs"
  - "backend/src/main.rs"
  - ".env.example"
  - "docker-compose.yml"
related_documents:
  - "backend/overview.md"
  - "backend/request-handling.md"
  - "backend/persistence-access.md"
  - "backend/shared-infrastructure.md"
  - "architecture/components.md"
related_diagrams:
  - "backend/diagrams/application-state-composition.mmd"
uncertainty_markers:
  - "INFERRED_FROM_CONFIGURATION"
  - "IMPLEMENTATION_STATUS_UNCLEAR"
  - "UNKNOWN"
---

# Backend Configuration and Application State

## Configuration Loading

`backend/src/main.rs` optionally loads a local `.env` file and then calls `Config::from_env`. Configuration is captured once at startup and stored in `Arc<Config>` inside `AppState`; no runtime reload mechanism was found.

Required variables are `DATABASE_URL` and `JWT_SECRET`. The JWT secret must be at least 32 characters. Optional values and defaults are parsed by explicit helper functions. Invalid numeric, boolean, port, origin, URL parsing at consumers, or weak-secret values can stop startup.

## Environment Variable Catalog

| Area | Variables | Required/default behavior |
|---|---|---|
| Persistence | `DATABASE_URL`, `REDIS_URL` | Database URL required; Redis defaults to local Redis |
| Tokens | `JWT_SECRET`, `JWT_ACCESS_EXPIRY`, `JWT_REFRESH_EXPIRY` | Secret required and length-checked; expiries default in source |
| Media | `UPLOAD_DIR`, `MAX_UPLOAD_SIZE` | Local directory and maximum size have source defaults |
| HTTP | `CORS_ORIGIN`, `COOKIE_SECURE`, `PORT` | Local-development-oriented defaults are present |
| Login protection | `LOGIN_RATE_LIMIT_MAX_FAILURES`, `LOGIN_RATE_LIMIT_WINDOW_SECONDS` | Integer defaults are present |
| CMS billing | `STRIPE_SECRET_KEY`, `STRIPE_WEBHOOK_SECRET`, `STRIPE_SUCCESS_URL`, `STRIPE_CANCEL_URL`, `STRIPE_PORTAL_RETURN_URL`, `STRIPE_PRO_PRICE_ID`, `STRIPE_ENTERPRISE_PRICE_ID` | Credentials and price IDs are optional; browser return URLs have defaults |
| Application links | `APP_BASE_URL` | Defaults to the local frontend URL |
| Email | `EMAIL_PROVIDER`, `EMAIL_FROM`, `EMAIL_WEBHOOK_URL`, `EMAIL_FAILURE_MODE` | Log-oriented defaults; webhook URL optional |
| Tenant limits | `ORG_RATE_LIMIT_PER_MINUTE`, `ORG_USER_RATE_LIMIT_PER_MINUTE`, `ORG_RATE_LIMIT_BURST` | Integer defaults are present |

Secret values are deliberately not reproduced in OKF. `.env.example`, container configuration, CI, and deployment configuration must be checked together before changing environment contracts.

## Validation and Optional Features

Parsing accepts common boolean spellings and rejects invalid values. Empty optional strings become `None`. Stripe and email provider capabilities inspect optional configuration at their call sites; the application can start without all Stripe or email-webhook values, but the corresponding operation may be unavailable or use fallback behavior.

No general feature-flag framework was found. Configuration-dependent branches and database lifecycle/status fields act as feature availability controls in specific modules.

## Application State Composition

`AppState` has exactly four fields at the verification commit:

| Field | Type/shape | Lifecycle | Main consumers |
|---|---|---|---|
| `config` | `Arc<Config>` | Built once at startup, cloned by reference | Handlers, middleware, provider/configured behavior |
| `db` | SQLx `PgPool` | Lazy pool built at startup, shared by clone | Routes, services, health/readiness |
| `redis` | Redis `Client` | Client built at startup, connections acquired by operations | Delivery cache, rate/security counters |
| `page_preview_channels` | `Arc<RwLock<HashMap<Uuid, broadcast::Sender<String>>>>` | Empty on process start, mutated in memory | Page preview publishers and WebSocket subscribers |

Services are not stored as long-lived objects in `AppState`; modules generally call service functions and pass state fragments.

## Startup State Transitions

1. Load environment and initialize tracing.
2. Parse and validate `Config`.
3. prepare the lazy PostgreSQL pool.
4. Run embedded migrations.
5. Seed a default administrator and default-organization relationship when no users exist.
6. Create the Redis client.
7. Build `AppState`, router groups, and global HTTP layers.
8. Bind the configured port and serve until the shutdown signal.

The bootstrap source contains fixed initial credential material used only when the user table is empty. Values are not duplicated here; production provisioning and rotation expectations need owner confirmation.

## Development and Deployment Configuration

Source defaults favor local development. Repository container files and CI supply service endpoints and test values, while deployment-specific secrets and overrides are external. OKF cannot verify the values or topology of any running environment. `INFERRED_FROM_CONFIGURATION` applies whenever a deployment conclusion is drawn only from templates.

## State Risks

- Preview channels are process-local and disappear on restart; multi-replica fan-out is not implemented by this state shape.
- Runtime configuration changes require process restart.
- Every handler receiving `AppState` can access all shared infrastructure fields.
- Redis client creation does not prove Redis is reachable until an operation connects.
- The fixed database pool maximum may not match production concurrency requirements.
- Bootstrap credentials and seeding behavior require explicit production operational control.

## Related Documentation

See the [state composition diagram](diagrams/application-state-composition.mmd), [Bootstrap and Runtime module](modules/bootstrap-runtime.md), [Persistence Access](persistence-access.md), and [Backend Risks](backend-risks.md).
