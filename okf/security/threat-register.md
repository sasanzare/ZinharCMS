---
okf_document_id: "security-threat-register"
title: "Threat Register"
project: "ZinharCMS"
category: "security-threats"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "mixed"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/main.rs"
  - "backend/src/routes"
  - "backend/src/middleware"
  - "backend/src/services"
  - "backend/migrations"
  - "frontend/src"
related_documents:
  - "trust-boundaries.md"
  - "security-risks.md"
  - "security-testing.md"
related_diagrams:
  - "diagrams/trust-boundaries.mmd"
  - "diagrams/authorization-decision-flow.mmd"
---

# Threat Register

## Threats

| ID | Threat | Assets/boundary | Existing controls | Residual status |
| --- | --- | --- | --- | --- |
| THR-01 | Credential stuffing or brute force | Public login, accounts | Generic error, Argon2, failed-IP window | Medium; proxy trust and distributed attacks remain |
| THR-02 | Bootstrap account takeover | Empty database/startup | Password hashing, first-user logic | High; deterministic development credentials and public first registration |
| THR-03 | Access-token theft through XSS | Browser volatile memory | No persistence, CSP, React escaping, selected sanitization | High impact remains while a live token is script-readable |
| THR-04 | Refresh-token replay | HttpOnly cookie, refresh families | Random token, hash storage, transactional rotation, family reuse response | Residual availability impact if browser coordination fails |
| THR-05 | Stale authorization after role/activity change | Access token | Current user/global role/auth-version database check | Residual organization-role freshness follows request/preview revalidation boundaries |
| THR-06 | Cross-tenant data access/IDOR | Tenant header, handlers, PostgreSQL | Active membership, RBAC, tenant SQL context, forced RLS | High impact; live exhaustive verification absent |
| THR-07 | Privilege escalation through role ambiguity | Global/org namespaces, legacy permissions | Separate claims/context and helper families | Medium; overlapping names and unused permission arrays |
| THR-08 | Administrative bypass misuse | RLS bypass helper | Explicit opt-in and caller checks | High impact; helper itself is not authorization-aware |
| THR-09 | CSRF on cookie-backed auth action | Refresh/logout | SameSite=Lax, exact Origin when present, explicit credentialed CORS | Residual deployment-origin/cookie configuration risk |
| THR-10 | Preview credential disclosure or replay | Ticket protocol/Redis | Credential-free URL, short TTL, hash-only store, exact Origin, atomic consume | Residual protocol-header logging/configuration risk |
| THR-11 | Stored or reflected XSS | Rich text, page JSON, Marketplace data | CSP, React escaping, targeted sanitizer | Medium; sanitizer/rendering coverage is incomplete |
| THR-12 | Malicious file/package or path traversal | Upload/package/filesystem | Body limit, signature/MIME and package/path validation | Medium; public serving/deployed scanner posture unverified |
| THR-13 | SSRF or unsafe outbound request | Webhooks, provider/webhook URLs, Marketplace runtime | URL restrictions, signed webhooks, allowlisted runtime operations | High impact; egress network enforcement unverified |
| THR-14 | Marketplace permission bypass | Package-to-host boundary | Manifest allowlist, approval snapshot, operation mapping, runtime/kill-switch state | Medium; no arbitrary package execution in observed phase |
| THR-15 | Secret/configuration disclosure | Environment, logs, source | Ignored local env files, required JWT length, optional provider values | High impact; secret lifecycle and redaction unverified |
| THR-16 | Audit evasion/tampering | Audit and login-attempt stores | RLS, selected transaction-coupled events | Medium; coverage, retention, and tamper evidence incomplete |
| THR-17 | Security-header/proxy drift | Application to ingress/browser | Header middleware and unit test, single-origin CORS | Medium; deployment not inspected |
| THR-18 | Resource exhaustion | Public/tenant API, uploads, Redis/database | Timeout, tenant body limit, login/org rate limits, quotas | Medium; public endpoint and distributed limits incomplete |

## Ownership and Review

No formal security owner, severity SLA, risk-acceptance authority, or review cadence was found. Assigning owners and treatment dates requires `NEEDS_OWNER_CONFIRMATION`; this register intentionally does not invent them.

## Phase 9 Threat Mapping

Extension-specific threats include trusted callback misuse, deceptive sandbox claims, permission escalation during update, unsafe artifact paths, oversized payloads, stale derived records, and future execution without isolation. Current controls and gaps are consolidated in [Extensibility Risks](../extensibility/extensibility-risks.md).
