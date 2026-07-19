---
okf_document_id: "security-testing"
title: "Security Testing"
project: "ZinharCMS"
category: "security-testing"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "mixed"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/middleware/security.rs"
  - "backend/src/services/rbac.rs"
  - "backend/src/services/security.rs"
  - "backend/src/services/hardening.rs"
  - "backend/src/services/marketplace_runtime.rs"
  - "backend/src/services/marketplace_phase_thirteen.rs"
  - "frontend/src/pages/MarketplacePage.test.tsx"
related_documents:
  - "threat-register.md"
  - "security-risks.md"
  - "../backend/testing-map.md"
  - "../api/api-testing.md"
related_diagrams:
  - "diagrams/authorization-decision-flow.mmd"
---

# Security Testing

## Verified Test Areas

| Area | Evidence | What it demonstrates |
| --- | --- | --- |
| Security headers | `middleware/security.rs` unit test | Expected header values are inserted into a header map |
| Rich-text sanitizer | `services/security.rs` unit tests | Script-block removal and attribute stripping examples |
| Organization RBAC | `services/rbac.rs` unit matrix | Selected capability/role allow and deny results |
| Forced RLS contract | `services/hardening.rs` | Expected migrations include forced RLS and context helpers |
| Marketplace manifest/runtime | Marketplace service tests | Permission allowlist, operation mapping, payload/entry-point gates, kill-switch contracts |
| Marketplace security regression | `marketplace_phase_thirteen.rs` static contracts | Selected IDOR, permission-bypass, malicious-package, refund, and review-abuse paths exist |
| Frontend role/permission cues | `MarketplacePage.test.tsx` | Visible role and permission-approval behaviors |

## Missing or Partial Coverage

- No dedicated authentication route integration suite was found for register/login/refresh/logout/cookie rotation.
- No JWT unit suite proves malformed-token cases, clock boundaries, or key rotation.
- No password policy/rehash tests were found.
- No full global RBAC matrix or exhaustive endpoint-by-role suite was found.
- RLS tests are largely static migration assertions rather than live cross-tenant database attempts.
- No deployed CORS/cookie/TLS/header test was performed.
- No complete CSRF, XSS, SSRF, IDOR, token replay, concurrency, or audit-event integration suite was found.
- No DAST, SAST, dependency-vulnerability, secret-scanning, or fuzz result was used in this phase.

## Interpretation

Static contract tests prove that expected strings and code paths exist; they do not prove runtime resistance. Frontend tests prove UX behavior, not backend authorization. Unit sanitizer examples are not a substitute for a mature HTML security test corpus.

`SECURITY_TEST_COVERAGE_UNCLEAR STCU-01` is the umbrella marker for these gaps. Phase 7 documentation validation may run existing safe test suites, but it must not convert unexecuted security tests into passed claims.

## Phase 7 Validation Run

On 2026-07-19, `cargo test` completed successfully with 117 backend unit tests passed and no failures. The targeted `MarketplacePage.test.tsx` Vitest run completed successfully with 12 tests passed and no failures. These results verify the current repository test suites at the documented snapshot; they do not close the missing integration and attack-path coverage above.
