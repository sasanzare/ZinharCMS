---
okf_document_id: "security-overview"
title: "Security Overview"
project: "ZinharCMS"
category: "security"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "mixed"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/main.rs"
  - "backend/src/routes/mod.rs"
  - "backend/src/routes/auth.rs"
  - "backend/src/middleware"
  - "backend/src/services"
  - "backend/migrations"
  - "frontend/src"
related_documents:
  - "README.md"
  - "trust-boundaries.md"
  - "security-risks.md"
related_diagrams:
  - "diagrams/trust-boundaries.mmd"
---

# Security Overview

## Security Model

ZinharCMS uses layered controls:

1. Public, bearer-authenticated, and tenant-protected Axum router subtrees establish the initial access boundary.
2. HMAC-signed access tokens establish user identity and a global role.
3. Tenant middleware requires an active organization and membership, supplies the organization role, and applies organization/user rate and plan-quota checks.
4. Handler-level RBAC helpers authorize operations by hard-coded global or organization role sets.
5. Ownership and lifecycle checks add resource-specific constraints in selected Marketplace, organization, workflow, and feedback paths.
6. PostgreSQL forced RLS protects tenant tables when queries use tenant-aware connections or transactions.
7. Marketplace runtime authorization adds an approved-permission snapshot, operation mapping, runtime state, and kill-switch checks.

## Verified Controls

| Area | Observed control | Status |
| --- | --- | --- |
| Passwords | Argon2 default hashing with a random salt | `VERIFIED` |
| Access tokens | HS256 signature, `sub`, global `role`, `iat`, and `exp` validation | `VERIFIED` |
| Refresh tokens | 32 random bytes; SHA-256 hash persisted; rotation and individual revocation | `VERIFIED` |
| Login abuse | Failed-attempt window by derived client IP | `VERIFIED`, deployment trust is unclear |
| Tenant entry | Active organization plus active membership | `VERIFIED` |
| Authorization | Named global and organization RBAC helper checks | `VERIFIED`, coverage is distributed |
| Tenant persistence | Forced RLS on documented tenant tables with explicit context | `VERIFIED` in migrations/code, not live deployment |
| Browser response | CSP, frame, MIME-sniffing, referrer, and permissions-policy headers | `VERIFIED` in application middleware |
| CORS | One configured origin, credential support, explicit methods and headers | `VERIFIED` in application bootstrap |
| Input protection | Typed extractors, targeted validation, rich-text sanitization, upload/package/webhook validators | `partially_verified` |
| Audit | Organization-scoped audit logs for selected mutations plus login-attempt records | `partially_verified` |

## Important Findings

- Global and organization roles are separate namespaces even where names overlap.
- A global `super_admin` override applies to global RBAC helpers, but it does not bypass tenant middleware or grant organization membership.
- Organization `owner` overrides organization RBAC helper allowlists.
- Database `roles.permissions` arrays are not read by runtime authorization; current enforcement uses role names in `rbac.rs`. This is `RBAC_MAPPING_UNCLEAR RMU-01` and `PERMISSION_SEMANTICS_UNCLEAR PSU-01`.
- The browser stores access tokens and organization state in `localStorage`; the backend sends refresh tokens only as an `HttpOnly` cookie, although the frontend retains legacy refresh-token storage support.
- `POTENTIAL_SECRET_EXPOSURE PSE-01`: deterministic development credentials are embedded in startup/UI source. No secret value is reproduced in OKF.

## Assurance Limits

No deployed configuration, reverse proxy, TLS termination, secret manager, runtime database catalog, or production logs were inspected. No scanner, exploit, or penetration test was run. Security status is therefore `mixed`, not certified.
