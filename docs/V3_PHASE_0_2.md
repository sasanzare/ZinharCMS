# V3 Phase 0.2 V2 Marketplace Readiness Audit

Phase 0.2 audits the V2 SaaS foundation before V3 Marketplace domain modeling starts. The goal is to make every critical dependency on organizations, billing, RBAC, and audit logs explicit, then turn remaining work into a concrete gap list for plugin installation, creator payments, and Marketplace permissions.

## Objective

Confirm that V2 provides enough stable infrastructure to begin V3 Marketplace design without hidden assumptions about tenant ownership, billing state, organization roles, or auditability.

## Delivered Scope

- Added a V2 readiness audit for Marketplace integration surfaces.
- Added a V2 dependency matrix for organizations, billing, RBAC, audit logs, RLS, and operational support.
- Added a Marketplace gap list covering plugin install, creator payment, and permission work.
- Added backend static tests that keep the phase 0.2 audit outputs present and non-ambiguous.

## Audit Result

V2 is ready to support V3 phase 1.1 domain modeling with known gaps. The critical V2 dependency boundaries are clear:

- Organizations provide tenant ownership and active membership context.
- Billing provides organization subscriptions, plan limits, Stripe lifecycle hooks, and usage counters.
- RBAC provides organization roles and reusable role-check helpers.
- Audit logs provide organization-scoped event recording.
- RLS provides a defensive tenant-isolation layer for organization-owned tables.

No critical V2 dependency remains ambiguous for starting V3. The remaining work belongs to V3 domain model, manifest, registry, install, payment, and permission phases.

## Acceptance Criteria

- Organizations, billing, RBAC, and audit log integration points are documented.
- Plugin install gaps are listed with ownership and target phases.
- Creator payment gaps are listed with ownership and target phases.
- Permission model gaps are listed with ownership and target phases.
- No critical V2 dependency remains ambiguous before phase 1.1 starts.

## Related Documents

- `docs/V3_V2_MARKETPLACE_READINESS_AUDIT.md`
- `docs/V3_MARKETPLACE_V2_DEPENDENCY_MATRIX.md`
- `docs/V3_MARKETPLACE_GAP_LIST.md`