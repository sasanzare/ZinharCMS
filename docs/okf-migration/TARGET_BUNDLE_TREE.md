# ZinharCMS Target Google OKF v0.2 Bundle Tree

**DESIGN ONLY - NOT YET IMPLEMENTED**

This is the logical tree for the future canonical `/okf/` bundle. None of the
paths below is created by Phase 2. The current `/okf/` and `/okf-bootstrap/`
trees remain legacy preservation material.

## Proposed tree

```text
okf/
├── index.md                                      # Reserved root index; okf_version: "0.2"
├── log.md                                        # Reserved root semantic update log
├── project/
│   ├── index.md
│   ├── project-overview.md
│   ├── terminology-and-glossary.md
│   ├── repository-and-evidence-map.md
│   ├── documentation-governance.md
│   └── source-lineage.md
├── architecture/
│   ├── index.md
│   ├── system-architecture.md
│   ├── runtime-and-request-boundaries.md
│   ├── integrations-and-side-effects.md
│   └── architecture-decision-records.md
├── backend/
│   ├── index.md
│   ├── backend-runtime.md
│   ├── module-boundaries.md
│   ├── persistence-services-and-configuration.md
│   └── backend-validation.md
├── frontend/
│   ├── index.md
│   ├── admin-application.md
│   ├── routing-and-state.md
│   ├── feature-boundaries.md
│   └── frontend-quality-and-testing.md
├── api/
│   ├── index.md
│   ├── api-contract-overview.md
│   ├── route-surface.md
│   ├── authentication-and-session-contract.md
│   ├── public-delivery-and-webhooks-contract.md
│   └── marketplace-and-extension-contracts.md
├── database/
│   ├── index.md
│   ├── schema-and-migrations.md
│   ├── entities-and-relationships.md
│   ├── tenant-data-policy.md
│   └── data-lifecycle-and-retention.md
├── security/
│   ├── index.md
│   ├── authentication-and-sessions.md
│   ├── authorization-and-rbac.md
│   ├── tenant-isolation.md
│   ├── preview-security.md
│   ├── storage-and-file-security.md
│   └── security-posture-and-risks.md
├── domain/
│   ├── index.md
│   ├── content-and-editorial-workflow.md
│   ├── page-builder-and-preview.md
│   ├── media-and-file-storage.md
│   ├── public-delivery.md
│   ├── billing-and-quotas.md
│   ├── marketplace.md
│   ├── extensibility-and-built-in-plugins.md
│   └── marketplace-runtime-and-safety-boundary.md
├── operations/
│   ├── index.md
│   ├── local-and-reference-topology.md
│   ├── ci-and-release-gates.md
│   ├── deployment-and-recovery.md
│   └── observability-and-support.md
├── development/
│   ├── index.md
│   ├── development-and-testing.md
│   ├── contribution-and-change-validation.md
│   └── documentation-maintenance.md
├── decisions/
│   ├── index.md
│   ├── owner-decision-debt.md
│   └── migration-and-architecture-decisions.md
└── history/
    ├── index.md
    ├── phase-0-baseline.md
    ├── phase-1-preservation.md
    ├── bootstrap-audit-record.md
    ├── legacy-conflict-and-completion-record.md
    └── legacy-structure-and-sequencing.md
```

There is intentionally no `references/` or `diagrams/` directory. External
evidence is represented by official `sources` entries using immutable source
URLs, and Mermaid is embedded in Concept bodies. The official `references/`
convention is optional; omitting it avoids copying source files into the
bundle. The future validator must reject legacy `index.yaml`, legacy metadata,
and standalone `.mmd` files inside this target boundary as project-policy
violations, while the Google conformance check remains limited to official
requirements.

## Directory responsibilities

| Directory | Purpose and included knowledge | Excluded knowledge | Phase 1 groups | Expected Concepts |
| --- | --- | --- | --- | ---: |
| `/okf/` | Bundle identity, progressive disclosure, and semantic update history | Source code, migrations, legacy registry, migration diary | All | 54 plus 2 reserved files |
| `project/` | Product identity, terminology, repository/evidence map, documentation authority, and source lineage | Runtime implementation details and full historical audit reports | 001, 011, 013 | 5 |
| `architecture/` | System boundaries, request flow, integrations, side-effect limits, and architecture decisions | Endpoint catalogs, schema inventories, owner NOC register | 002, 004, 006, 011 | 4 |
| `backend/` | Rust runtime, module boundaries, service/persistence/configuration composition, backend validation | Frontend features, external API contract catalog, production deployment claims | 003, 004, 005, 011 | 4 |
| `frontend/` | React/Vite administration app, route/state integration, feature boundaries, frontend quality evidence | Backend authority, browser support policy not approved by owner | 008, 011 | 4 |
| `api/` | Reachable route families, API contract summaries, generated route surface, public/auth/Marketplace contracts | Internal service implementation and undocumented compatibility promises | 004, 006, 007, 009 | 5 |
| `database/` | Migration-defined schema, entity relationships, tenant data policy, and retention decision boundary | Applied production schema, backups, RPO/RTO, or legal retention guarantees without evidence | 005, 006, 007, 009, 011 | 4 |
| `security/` | Authentication/session, RBAC, tenant isolation, preview, file security, and security posture | Owner-approved incident/on-call policy and unverified external security services | 005, 006, 007, 009, 010 | 6 |
| `domain/` | Content, pages, media, delivery, billing, Marketplace, and extension capability workflows | Route inventory, raw database schema, unresolved product roadmap as fact | 003, 004, 005, 006, 007, 008, 009 | 8 |
| `operations/` | Local/reference topology, CI/release gates, and owner-dependent deployment/recovery/observability | Claims that local Compose is production, or unsupported backup/monitoring guarantees | 010, 011 | 4 |
| `development/` | Development/testing commands, contribution/change checks, and documentation maintenance | Durable ownership policy until NOC-13 through NOC-15 are resolved | 001, 003, 008, 011 | 3 |
| `decisions/` | Open owner decision debt and accepted migration/architecture decisions | Unresolved decisions presented as current implementation facts | 001, 002, 006, 007, 009, 010, 011, 012 | 2 |
| `history/` | Phase 0/1, bootstrap, conflict, completion, and prior-structure history | Current implementation authority and active lifecycle policy | 002, 011, 012 | 5 |

## Tree controls

- Every non-reserved `.md` file in the future tree is a Concept and has
  parseable frontmatter with a non-empty `type`.
- Every domain `index.md` is reserved and has no frontmatter. Only the root
  index may declare `okf_version: "0.2"`.
- Every Concept is listed once by its direct parent index. The root index lists
  each domain in the order shown above.
- No Concept uses a legacy `okf_document_id`, `category`, `phase`, relation
  array, or other custom extension key.
- No `.mmd`, `.yaml`, source-code, migration, or test file is part of the
  bundle. Those remain external evidence and are referenced by pinned URLs.
- The directory count is 13 including `/okf/`; the Concept count is 54; the
  reserved-file count is 14 (13 indexes and one log).

See [TARGET_CONCEPT_CATALOG.md](TARGET_CONCEPT_CATALOG.md) for the one-to-one
Concept path list and
[NAVIGATION_AND_LINKING_POLICY.md](NAVIGATION_AND_LINKING_POLICY.md) for
index and link behavior.
