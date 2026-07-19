---
okf_document_id: "security-audit-events"
title: "Audit and Security Events"
project: "ZinharCMS"
category: "security-audit"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "partially_verified"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/services/audit.rs"
  - "backend/src/services/security.rs"
  - "backend/src/routes"
  - "backend/migrations/0007_phase_seven_security.sql"
  - "backend/migrations/0012_v2_phase_seven_saas_ops.sql"
related_documents:
  - "administrative-access.md"
  - "threat-register.md"
  - "../database/lifecycle-and-auditing.md"
related_diagrams:
  - "diagrams/authorization-decision-flow.mmd"
---

# Audit and Security Events

## Audit Stores

`audit_logs` stores organization, optional actor, action, entity type, optional entity ID, JSON metadata, and timestamp. It is tenant-scoped and protected by forced RLS. `login_attempts` separately stores email, IP, success state, and attempt time and is not part of tenant audit logs.

## Observed Event Coverage

Audit calls exist for selected organization/member/invitation/domain/rate-limit actions, billing changes and usage rebuilds, beta operations, content-type and entry deletions, media deletion, page/component changes, Marketplace creator/catalog/review/installation/runtime/kill-switch/finance/adapter actions, and related operations.

Marketplace runtime authorization attempts record both allowed and rejected decisions. Provider events and domain-specific review-event tables add separate operational histories.

## Reader Access

Organization audit-log endpoints require organization admin-level access, with owner override through the RBAC helper. RLS constrains returned organization rows. Global platform histories and domain-specific event tables have separate authorization paths.

## Gaps

No tenant audit event was found for registration, login, refresh, logout, failed bearer verification, generic RBAC denial, global-role assignment, JWT-secret changes, or every RLS bypass entry. Login-attempt records capture authentication outcomes but not refresh/logout and have no documented retention policy.

`AUDIT_COVERAGE_UNCLEAR ACU-01`: there is no central security-event taxonomy, required-event matrix, tamper-evidence control, export/SIEM contract, retention schedule, or proof that every privileged mutation records an event.

## Sensitive Data Guidance

Audit metadata is flexible JSON. New writers should avoid secrets, raw tokens, passwords, private certificates, and unnecessary personal/provider data. This is a documentation expectation; no central metadata redactor was found.
