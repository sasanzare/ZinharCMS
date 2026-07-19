---
okf_document_id: "security-risks"
title: "Security Risks"
project: "ZinharCMS"
category: "security-risks"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "mixed"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/main.rs"
  - "backend/src/routes/auth.rs"
  - "backend/src/services/rbac.rs"
  - "backend/src/services/rls.rs"
  - "frontend/src/stores/useAppStore.ts"
  - "backend/migrations"
related_documents:
  - "threat-register.md"
  - "security-testing.md"
  - "../architecture/architecture-risks.md"
  - "../api/api-risks.md"
related_diagrams:
  - "diagrams/trust-boundaries.mmd"
---

# Security Risks

## Prioritized Register

| ID | Risk | Evidence | Priority | Recommended treatment direction |
| --- | --- | --- | --- | --- |
| SEC-01 | Deterministic development administrator can survive an empty production bootstrap | Startup and login-page defaults | Critical | Replace with explicit installation/bootstrap ceremony and production fail-closed mode |
| SEC-02 | Script-readable access token amplifies XSS impact | Frontend `localStorage` | High | Evaluate in-memory or hardened cookie/session design with CSRF model |
| SEC-03 | Public registration grants first user global super-admin | Registration count logic | Critical | Require explicit installation state, one-time secret, or operator provisioning |
| SEC-04 | Global role in JWT can remain effective after role/activity change | Claims-only middleware | High | Add token version/session validation or shorter/revocable authorization state |
| SEC-05 | Refresh lifecycle lacks family/reuse detection and atomic rotation | Auth refresh flow | High | Model token families, reuse response, transactional rotation, logout-all, cleanup |
| SEC-06 | RLS bypass helper is authorization-agnostic | `begin_bypass_transaction` callers | High | Centralize narrowly named privileged operations and test preconditions/audit |
| SEC-07 | Three permission vocabularies can drift | RBAC helpers, role arrays, Marketplace catalog | High | Define one authoritative mapping or explicitly retire legacy arrays |
| SEC-08 | Tenant isolation assurance is primarily static | Middleware, migrations, static tests | High | Add live cross-tenant integration matrix with real RLS role and all major resources |
| SEC-09 | Authentication/security event audit is incomplete | Audit/login-attempt call map | Medium | Define required event taxonomy, retention, redaction, export, and denial coverage |
| SEC-10 | Proxy trust can weaken IP-based login limiting | first valid `X-Forwarded-For` | Medium | Configure trusted proxies and derive client IP only through trusted hops |
| SEC-11 | Preview query tokens may leak | Preview query compatibility | Medium | Use short-lived scoped preview credentials and mandatory log redaction |
| SEC-12 | Cookie/TLS/HSTS behavior depends on unverified deployment | Configurable Secure, no app HSTS | High | Make production Secure fail-closed and verify ingress TLS/header policy |
| SEC-13 | Custom rich-text sanitizer has limited coverage | `services/security.rs` | Medium | Use a maintained sanitizer and context-specific output tests |
| SEC-14 | Public static upload behavior is delegated and under-tested | `/uploads` `ServeDir` | Medium | Verify cache, type, download, range, indexing, and untrusted-content origin policy |
| SEC-15 | No complete security regression suite | Test inventory | High | Add auth, RBAC, IDOR, RLS, CSRF, XSS, SSRF, replay, concurrency, and header integration suites |

## Required Uncertainty Markers

| Marker | Meaning in this snapshot |
| --- | --- |
| `AUTHENTICATION_FLOW_UNCLEAR AFU-01` | Account verification, recovery, MFA, and bootstrap intent are unclear |
| `SESSION_LIFECYCLE_UNCLEAR SLU-01` | Frontend expiry and invalid-session behavior are incomplete |
| `TOKEN_LIFECYCLE_UNCLEAR TLU-01` | Token family, replay, key rotation, bulk revocation, cleanup, and concurrency are unclear |
| `COOKIE_SECURITY_UNVERIFIED CSU-01` | Deployed Secure/TLS behavior was not verified |
| `AUTHORIZATION_ENFORCEMENT_UNCLEAR AEU-01` | Distributed enforcement lacks an exhaustive route-action proof |
| `RBAC_MAPPING_UNCLEAR RMU-01` | Stored role permission arrays are not runtime grants |
| `PERMISSION_SEMANTICS_UNCLEAR PSU-01` | Legacy, named-capability, and Marketplace permission vocabularies differ |
| `TENANT_ACCESS_UNVERIFIED TAV-01` | Live cross-tenant/RLS behavior was not executed |
| `RESOURCE_OWNERSHIP_UNVERIFIED ROU-01` | No complete ownership/action matrix exists |
| `ADMINISTRATIVE_BYPASS_UNCLEAR ABY-01` | Bypass authorization is caller-distributed |
| `FRONTEND_ONLY_SECURITY_CHECK FOSC-01` | Frontend role/route checks are UX only |
| `SECRET_HANDLING_UNVERIFIED SHU-01` | Secret storage, access, rotation, and redaction are unverified |
| `INPUT_VALIDATION_UNCLEAR IVU-01` | Validation coverage and consistency are incomplete |
| `AUDIT_COVERAGE_UNCLEAR ACU-01` | Required security events and retention are undefined |
| `SECURITY_HEADER_STATUS_UNCLEAR SHSU-01` | Deployed headers/TLS/proxy behavior are unverified |
| `RATE_LIMITING_STATUS_UNCLEAR RLSU-01` | Trusted-proxy and live limiter behavior are unverified |
| `SECURITY_TEST_COVERAGE_UNCLEAR STCU-01` | Runtime security test coverage is incomplete |
| `POTENTIAL_SECRET_EXPOSURE PSE-01` | Deterministic development credentials exist in source/UI |

## Treatment Boundary

These are documentation findings, not implemented fixes. Owners, acceptance, deadlines, and product priorities require explicit confirmation.
