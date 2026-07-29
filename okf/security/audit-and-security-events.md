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

`audit_logs` stores organization, optional actor, action, entity type, optional
entity ID, JSON metadata, and timestamp. It is tenant-scoped and protected by
forced RLS. `security_audit_events` stores selected global identity/session
security events with actor, target, controlled metadata, and timestamp.
`login_attempts` separately stores email, IP, success state, and attempt time.

## Observed Event Coverage

Audit calls exist for selected organization/member/invitation/domain/rate-limit actions, billing changes and usage rebuilds, beta operations, content-type and entry deletions, media deletion, page/component changes, Marketplace creator/catalog/review/installation/runtime/kill-switch/finance/adapter actions, and related operations.

Marketplace runtime authorization attempts record both allowed and rejected decisions. Provider events and domain-specific review-event tables add separate operational histories.

Global security events cover individual and bulk session revocation,
logout-all, recovery/verification token issuance/consumption/reuse/revocation,
invitation acceptance, MFA enrollment/enable/disable, MFA-completed login,
recovery-code replacement, completed Step-Up, and bounded cleanup counts. The writer rejects metadata
field names associated with credentials, hashes, passwords, secrets, and
tokens.

## Reader Access

Organization audit-log endpoints require organization admin-level access, with owner override through the RBAC helper. RLS constrains returned organization rows. Global platform histories and domain-specific event tables have separate authorization paths.

## Gaps

No tenant/global security audit event is emitted for every registration, AAL1
login, failed MFA proof, refresh, failed bearer verification, generic RBAC denial, global-role
assignment, JWT key-ring change, or RLS bypass entry. Login-attempt records
capture authentication outcomes. Default retention is now explicit for global
security events and login attempts, but cleanup requires an external scheduler.

`AUDIT_COVERAGE_UNCLEAR ACU-01`: a controlled security-event writer and
retention defaults now exist, but there is no complete required-event matrix,
tamper-evidence control, export/SIEM contract, scheduler ownership, or proof that
every privileged mutation records an event.

## Sensitive Data Guidance

Tenant audit metadata remains flexible JSON. The new global security-event
writer validates controlled top-level fields, but it is not a recursive general
redactor. No writer may include raw tokens, hashes, passwords, authorization or
cookie values, private certificates, secrets, or unnecessary personal/provider
data.
