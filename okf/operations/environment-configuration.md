---
okf_document_id: "operations-environment-configuration"
title: "Environment Configuration"
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
  - "backend/src/config.rs"
  - ".env.example"
  - "env.example"
  - "docker-compose.prod.yml"
  - "frontend/vite.config.ts"
related_documents:
  - "../backend/configuration-and-state.md"
  - "../frontend/configuration-and-build.md"
  - "../security/secrets-and-configuration.md"
  - "../delivery/environment-promotion.md"
  - "external-dependencies.md"
  - "troubleshooting.md"
related_diagrams: []
---

# Environment Configuration

Defaults below appear only when explicit in `Config::from_env` or tracked container configuration. Secret values are intentionally omitted.

| Group | Variable | Purpose | Required/default | Scope | Secret sensitivity | Source | Validation/failure |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Database | `DATABASE_URL` | PostgreSQL connection | Required | Backend | Secret connection material | `config.rs` | Missing/invalid URL prevents startup or migration |
| Cache | `REDIS_URL` | Redis connection | Optional; `redis://localhost:6379` | Backend | Usually sensitive by deployment | `config.rs` | Invalid URL prevents client creation; unreachable makes readiness fail |
| Authentication | `JWT_SECRET` | HMAC token signing | Required; no default | Backend | **Secret** | `config.rs` | Missing or fewer than 32 characters prevents startup |
| Authentication | `JWT_ACCESS_EXPIRY` | Access-token lifetime seconds | `3600` | Backend | Non-secret | `config.rs` | Invalid integer prevents startup |
| Authentication | `JWT_REFRESH_EXPIRY` | Refresh-token lifetime seconds | `604800` | Backend | Non-secret | `config.rs` | Invalid integer prevents startup |
| Authentication | `COOKIE_SECURE` | Secure refresh-cookie flag | `false` | Backend/browser boundary | Non-secret; security-critical | `config.rs` | Invalid boolean prevents startup |
| Authentication | `LOGIN_RATE_LIMIT_MAX_FAILURES` | Login failure threshold | `5` | Backend | Non-secret | `config.rs` | Invalid integer prevents startup |
| Authentication | `LOGIN_RATE_LIMIT_WINDOW_SECONDS` | Login limiter window | `900` | Backend | Non-secret | `config.rs` | Invalid integer prevents startup |
| Storage | `UPLOAD_DIR` | Media/package filesystem root | `./uploads` | Backend | Path may reveal topology | `config.rs` | Filesystem errors occur at operation time |
| Storage | `MAX_UPLOAD_SIZE` | Request/upload size limit | `52428800` | Backend | Non-secret | `config.rs` | Invalid integer prevents startup |
| Application | `CORS_ORIGIN` | Single allowed browser origin | `http://localhost:5173` | Backend HTTP | Non-secret; security-critical | `config.rs` | Invalid header value prevents startup |
| Application | `PORT` | Backend listen port | `8080` | Backend | Non-secret | `config.rs` | Invalid port or bind failure prevents startup |
| Application | `APP_BASE_URL` | Absolute links, including invitations | `http://localhost:5173` | Backend/email | Public origin | `config.rs` | Used without a dedicated URL validator |
| Logging | `RUST_LOG` | Tracing filter | Subscriber default behavior; template/Compose use `info` | Backend | Non-secret; can increase data exposure | `main.rs`, templates | Invalid filter behavior belongs to tracing subscriber |
| Email | `EMAIL_PROVIDER` | `log`, `webhook`, or disabled/default branch | `log` | Backend | Non-secret | `config.rs`, `email.rs` | Unknown values follow log/default branch |
| Email | `EMAIL_FROM` | Sender identity | Local synthetic sender | Backend | Potential personal/config data | `config.rs` | No dedicated validation found |
| Email | `EMAIL_WEBHOOK_URL` | Outbound provider endpoint | Optional | Backend | Potential secret endpoint | `config.rs` | Missing while webhook provider produces failed delivery |
| Email | `EMAIL_FAILURE_MODE` | Strict versus logged failure | `log` | Backend | Non-secret | `config.rs`, `email.rs` | Strict failed provider returns service unavailable |
| Billing | `STRIPE_SECRET_KEY` | Stripe API authentication | Optional | Backend | **Secret** | `config.rs` | Feature calls fail/unavailable when absent |
| Billing | `STRIPE_WEBHOOK_SECRET` | Verify provider callbacks | Optional | Backend | **Secret** | `config.rs` | Signed webhook processing unavailable/invalid when absent |
| Billing | `STRIPE_PRO_PRICE_ID` | Pro plan provider price | Optional | Backend | Identifier, not secret | `config.rs` | Plan checkout depends on configured ID |
| Billing | `STRIPE_ENTERPRISE_PRICE_ID` | Enterprise plan provider price | Optional | Backend | Identifier, not secret | `config.rs` | Plan checkout depends on configured ID |
| Billing | `STRIPE_SUCCESS_URL` | Checkout redirect | Local billing success URL | Backend/provider | Public URL | `config.rs` | No dedicated URL validation shown |
| Billing | `STRIPE_CANCEL_URL` | Checkout cancel redirect | Local billing cancelled URL | Backend/provider | Public URL | `config.rs` | No dedicated URL validation shown |
| Billing | `STRIPE_PORTAL_RETURN_URL` | Customer portal return | Local billing URL | Backend/provider | Public URL | `config.rs` | No dedicated URL validation shown |
| Application | `ORG_RATE_LIMIT_PER_MINUTE` | Organization request rate | `600` | Backend | Non-secret | `config.rs` | Invalid integer prevents startup |
| Application | `ORG_USER_RATE_LIMIT_PER_MINUTE` | Per-user organization rate | `120` | Backend | Non-secret | `config.rs` | Invalid integer prevents startup |
| Application | `ORG_RATE_LIMIT_BURST` | Organization burst allowance | `120` | Backend | Non-secret | `config.rs` | Invalid integer prevents startup |
| Frontend | `VITE_API_URL` | Browser API base URL | Required build arg in production Dockerfile; frontend code has its own handling | Build/browser | Public by design | Dockerfile/frontend source | Missing production build arg fails Compose expansion |
| Deployment | `POSTGRES_DB` | PostgreSQL database name | `cms_prod` in production-like Compose | Container | Non-secret | Compose | Used by image initialization/health |
| Deployment | `POSTGRES_USER` | PostgreSQL user | `cms_user` in production-like Compose | Container | Identifier | Compose | Used by image initialization/health |
| Deployment | `POSTGRES_PASSWORD` | PostgreSQL password | Required in production-like Compose | Container | **Secret** | Compose | Compose expansion fails when missing |

No environment variables were found for a search engine, durable queue, application metrics exporter, monitoring vendor, backup destination, restore policy, plugin execution sandbox, or deployment provider. Production secret injection, access, rotation, and recovery remain `SECRET_INJECTION_UNCLEAR`.

See [Backend Configuration](../backend/configuration-and-state.md), [Frontend Build](../frontend/configuration-and-build.md), [Security Configuration](../security/secrets-and-configuration.md), and [Troubleshooting](troubleshooting.md).

