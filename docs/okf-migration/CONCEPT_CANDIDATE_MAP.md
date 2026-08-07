# Preliminary Google OKF Concept Candidate Map

This is planning evidence for Phase 2. It is not the final Google OKF bundle
architecture. No candidate below has been created inside legacy okf/, and no
directory layout or final concept count is approved by this map.

Candidate dispositions describe the likely future treatment of the knowledge,
not a Phase 1 file operation:

- MIGRATE: distinct current knowledge can seed a concept after source review.
- MERGE: several legacy views should feed a smaller concept set.
- REGENERATE: the topic is valuable but should be rebuilt from current source,
  migrations, tests, configuration, or owner evidence.
- PRESERVE_HISTORICAL: planning, audit, or decision history should remain
  accessible without becoming current-state authority.

| Proposed Concept | Proposed Google OKF type | Source legacy knowledge | Authoritative evidence to use | Candidate disposition | Confidence | Merge groups | Unresolved questions |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Project | Project | okf/project/overview.md; okf/README.md; okf/project/glossary.md | README.md; LICENSE; Phase 1 corpus and claim ledger | MERGE | High | MERGE-GROUP-001 | Canonical product terminology and maintainer ownership |
| Architecture | Architecture | okf/architecture/*; okf-bootstrap/02-architecture-observations.md | backend/src/main.rs; backend/src/routes/mod.rs; frontend/src/main.tsx; docker-compose files | MERGE | High | MERGE-GROUP-002 | Production boundaries and actual ingress |
| Backend | Backend Component | okf/backend/*; okf/api/backend-module-map.md | backend/src; backend/Cargo.toml; backend/src/lib.rs | MERGE | High | MERGE-GROUP-003; MERGE-GROUP-004 | Formal module ownership and extraction policy |
| Frontend | Frontend Application | okf/frontend/* | frontend/package.json; frontend/src; frontend/nginx.conf | MERGE | High | MERGE-GROUP-008 | Supported browsers, accessibility target, and DTO generation policy |
| API Contract | API Contract | okf/api/overview.md; okf/api/endpoints/*; okf/api/groups/* | backend/src/routes; backend/src/routes/mod.rs; frontend/src/services/api.ts | REGENERATE | High | MERGE-GROUP-004 | Versioning, compatibility windows, deprecation, and OpenAPI completeness |
| Authentication and Sessions | Authentication | okf/api/authentication.md; okf/security/authentication-*; okf/security/session-token-lifecycle.md | backend/src/routes/auth.rs; backend/src/services/jwt.rs; backend/src/services/sessions.rs; migrations 0027-0029 | MERGE | High | MERGE-GROUP-006 | Recovery, provider, bootstrap, and session-support policy |
| Authorization and RBAC | Authorization Model | okf/security/authorization-*; okf/security/rbac-model.md; roles/*; permissions/* | backend/src/services/rbac.rs; backend/src/middleware/auth.rs; backend/src/middleware/tenant.rs; backend/src/routes | MERGE | High | MERGE-GROUP-006 | Global/tenant/support role ownership and final permission matrix |
| Tenant Isolation | Security Boundary | okf/database/multi-tenancy.md; okf/security/tenant-access-control.md; okf/api/tenant-context.md | backend/src/middleware/tenant.rs; backend/src/services/rls.rs; migration 0009; RLS tests | MERGE | High | MERGE-GROUP-005; MERGE-GROUP-006; MERGE-GROUP-007 | Live schema/RLS state and public tenant routing |
| Database Model | Database Model | okf/database/*; database/entities/*; okf-bootstrap/06-database-inventory.md | backend/migrations; backend/src/db; backend/src/models; current tests | REGENERATE | High | MERGE-GROUP-005 | Applied migration version, drift, retention, and production backup |
| Content and Editorial Workflow | Domain Workflow | okf/domain/content-lifecycle.md; domain/workflows/*; database/entities/content-types-and-entries.md | backend/src/routes/content.rs; backend/src/services/workflow.rs; migrations 0003 and 0006 | MERGE | High | MERGE-GROUP-007 | Schema-evolution and workflow policy |
| Page Builder and Preview | Domain Workflow | okf/domain/page-builder-rules.md; domain/workflows/page-builder-*; api/endpoints/pages-workflow-versions-and-preview.md | backend/src/routes/pages.rs; frontend/src/pages; backend/src/services/preview_tickets.rs | MERGE | High | MERGE-GROUP-004; MERGE-GROUP-007; MERGE-GROUP-008 | Large-page compatibility and preview scaling |
| Media and File Storage | Resource Boundary | okf/domain/workflows/media-upload-and-processing.md; okf/api/uploads-downloads-and-streaming.md; okf/security/overview.md | backend/src/routes/media.rs; backend/src/services/media_processing.rs; backend/src/services/file_security.rs; migration 0030 | REGENERATE | High | MERGE-GROUP-004; MERGE-GROUP-007; MERGE-GROUP-006 | Public/private asset policy, shared storage, CDN, malware scanning |
| Public Delivery | Delivery Contract | okf/api/endpoints/public-delivery.md; okf/domain/domains/delivery-settings-and-webhooks.md | backend/src/routes/delivery.rs; backend/src/services/cache.rs; migrations 0005 and relevant settings tables | MIGRATE | High | MERGE-GROUP-004; MERGE-GROUP-007 | Intended host/custom-domain tenant selection and cache guarantees |
| Billing and Quotas | Billing Policy | okf/api/groups/billing-and-quota.md; okf/domain/domains/billing-and-quotas.md; database/entities/plans-subscriptions-and-usage.md | backend/src/routes/billing.rs; backend/src/services/quota.rs; migrations 0010-0012 | MERGE | High | MERGE-GROUP-004; MERGE-GROUP-005; MERGE-GROUP-007 | Compensation, provider failure, refunds, tax, and retention |
| Marketplace | Marketplace Domain | okf/domain/domains/marketplace.md; okf/extensibility/marketplace/*; api/endpoints/marketplace-*; database/entities/marketplace-* | backend/src/routes/marketplace*.rs; backend/src/services/marketplace_*.rs; migrations 0015-0026 | MERGE | High | MERGE-GROUP-004; MERGE-GROUP-005; MERGE-GROUP-007; MERGE-GROUP-009 | Payout settlement, disputes, appeals, cleanup, and external execution scope |
| Extensibility and Built-in Plugins | Extension Model | okf/extensibility/plugin-*; extension-points/*; okf/extensibility/hooks-and-events.md | backend/src/plugins; backend/src/routes/plugins.rs; backend/src/services/marketplace_runtime.rs | MERGE | High | MERGE-GROUP-009 | Trusted in-process plugin policy versus Marketplace adapters |
| Operations and Deployment | Operational Guide | okf/operations/*; okf/delivery/*; okf-bootstrap/01-technology-inventory.md | docker-compose files; Dockerfiles; backend/src/config.rs; CI workflows | REGENERATE | Medium | MERGE-GROUP-010 | Production topology, promotion, rollback, storage, backups, and ownership |
| Development and Testing | Development Guide | okf/development/*; backend/testing-map.md; frontend/testing-map.md | package manifests; CI workflows; backend/tests; frontend tests | MERGE | High | MERGE-GROUP-003; MERGE-GROUP-008; MERGE-GROUP-011 | Support matrix, coverage policy, E2E scope, and review requirements |
| Security Posture | Security Model | okf/security/*; okf/security/diagrams/*; Phase 7 changes | backend/src/middleware; backend/src/services; security migrations/tests; current config | MERGE | High | MERGE-GROUP-006 | Live deployment controls, retention, incident contacts, and external security services |
| Provenance and Source Lineage | Provenance Record | okf/references/source-register.md; primary_sources and verification fields | Phase 1 evidence manifest; Git history; current source paths; Google OKF sources semantics | MIGRATE | High | MERGE-GROUP-013 | Actor identity, commit retention, freshness policy, and source authority |
| Maintenance and Documentation Governance | Governance Guide | okf/maintenance/*; project/navigation-guide.md; documentation-ownership.md | HANDOFF.md; current Git state; owner decisions | MERGE | Medium | MERGE-GROUP-001; MERGE-GROUP-011 | Canonical owners, retirement policy, and update triggers |
| Decision Debt and Owner Register | Decision Register | okf-bootstrap/09-knowledge-gaps.md; okf-bootstrap/12-owner-questions.md; maintenance/unresolved-owner-questions.md | docs/okf-migration/OWNER_DECISION_REGISTER.md; owner/platform evidence when supplied | PRESERVE_HISTORICAL | High | MERGE-GROUP-012; MERGE-GROUP-011 | 17 open NOC decisions remain |
| Historical Audit Trail | Historical Record | all okf-bootstrap/*; maintenance/final-completion-report.md; cross-phase-conflicts.md | Git history; Phase 0 and Phase 1 manifests; original bootstrap files | PRESERVE_HISTORICAL | High | MERGE-GROUP-012 | Retention and navigation policy for historical records |

## Candidate boundary rules

1. A candidate is not a commitment to a final filename, directory, or
   one-to-one source mapping.
2. Current-state concepts must use source, migration, test, configuration, and
   owner evidence according to the claim ledger; a legacy document cannot
   upgrade its own trust level.
3. Risk, uncertainty, and owner questions should be linked or recorded as
   explicit knowledge rather than silently promoted to implementation facts.
4. Generated catalogs such as exhaustive endpoint, schema, and source lists
   should be regenerated in the future and carry Google OKF provenance.
