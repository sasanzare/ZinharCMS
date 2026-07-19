---
okf_document_id: "api-risks"
title: "API Risks"
project: "ZinharCMS"
category: "api-risk"
phase: 6
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "eed1e0dbdf6d873457d1165158b3c8fbfd6647e1"
last_verified_date: "2026-07-18"
primary_sources:
  - "backend/src/routes"
  - "backend/src/middleware"
  - "backend/src/error.rs"
  - "frontend/src/services/api.ts"
related_documents:
  - "api/openapi-consistency.md"
  - "api/error-contracts.md"
  - "api/api-testing.md"
  - "architecture/architecture-risks.md"
uncertainty_markers:
  - "OPENAPI_IMPLEMENTATION_CONFLICT OIC-01"
  - "ERROR_CONTRACT_UNCLEAR ECU-01"
  - "VERSIONING_BEHAVIOR_UNCLEAR VBU-01"
  - "FRONTEND_BACKEND_CONTRACT_CONFLICT FBCC-01"
---

# API Risks

## Prioritized Register

| ID | Priority | Risk | Evidence and impact | Recommended next validation |
| --- | --- | --- | --- | --- |
| API-RISK-01 | High | OpenAPI omits 19 handlers and all security/tenant requirements | Generated clients and reviewers receive an incomplete contract (`OIC-01`) | Automate route/OpenAPI parity and declare security schemes |
| API-RISK-02 | High | Error responses are not uniform across handlers, middleware, framework, timeout, WebSocket, and static files | Recovery code may misparse or misclassify failures (`ECU-01`) | Real-router error matrix and explicit error policy |
| API-RISK-03 | High | Database error strings can reach clients | Schema or operational detail may be disclosed | Phase 7 redaction and correlation-ID review |
| API-RISK-04 | High | Authorization is distributed across router placement, RBAC calls, ownership rules, and RLS | A change can preserve authentication but weaken capability enforcement | Endpoint authorization matrix plus negative tests |
| API-RISK-05 | High | Frontend types duplicate Rust DTOs manually | Field, enum, nullability, and status drift is possible despite all paths matching (`DC-01`) | Schema fixtures or generated client/types |
| API-RISK-06 | Medium | Unversioned administrative API has no compatibility/deprecation policy | Breaking changes lack an agreed migration path (`VBU-01`) | Architecture decision before a breaking change |
| API-RISK-07 | Medium | Many lists have inconsistent or absent pagination | Large tenants can face performance and UX instability | Inventory cardinality and establish family-specific limits |
| API-RISK-08 | Medium | Public `/uploads` bypasses application authorization and `AppError` | Files are accessible by object path and behavior is not contract-tested | Phase 7 exposure review and static-service tests |
| API-RISK-09 | Medium | Preview sends tokens and tenant IDs in query parameters | URLs may be logged or retained | Constrain lifetime/logging and test upgrade authorization |
| API-RISK-10 | Medium | Multipart bodies are buffered and have layered size limits | Memory pressure and rejection behavior vary by route/framework | Load tests and explicit per-part limits |
| API-RISK-11 | Medium | No comprehensive router-level API suite | Route, middleware, serialization, and side-effect regressions can escape unit tests | Add representative integration harness |
| API-RISK-12 | Low | Manual `docs/API.md` uses obsolete billing webhook path | Human consumers may call the wrong endpoint (`DCC-09`) | Correct the source document in its owning phase/change scope |

## Scope Boundary

This register identifies current contract and verification risk without changing runtime behavior. Security threat modeling, secret/token handling review, CORS/cookie hardening, rate-limit abuse analysis, and remediation design are recommended for Phase 7.

## Marker Discipline

Use a named uncertainty or conflict marker only when the relevant evidence is incomplete or contradictory. Do not convert a risk into a defect claim without a reproducer or code-level proof.

## Phase 7 Security Follow-Up

The recommended security review is complete and documented in [Security Risks](../security/security-risks.md) and [Threat Register](../security/threat-register.md). Highest-priority API-adjacent findings include bootstrap privilege, script-readable access tokens, stale JWT roles, refresh lifecycle, RLS bypass, tenant-isolation assurance, preview query credentials, cookie/deployment posture, and incomplete security regression coverage.
