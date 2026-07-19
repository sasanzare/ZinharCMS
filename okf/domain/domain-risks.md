---
okf_document_id: "domain-risks"
title: "Domain Risk Register"
project: "ZinharCMS"
category: "domain"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/routes"
  - "backend/src/services"
  - "backend/migrations"
  - "frontend/src"
related_documents:
  - "business-rule-catalog.md"
  - "invariants.md"
  - "business-rule-testing.md"
  - "../architecture/architecture-risks.md"
related_diagrams:
  - "diagrams/cross-module-orchestration.mmd"
---

# Domain Risk Register

| ID | Title/type | Description and evidence | Domains/workflows/entities | Likelihood / impact / severity | Existing mitigation | Follow-up / owner confirmation / status |
| --- | --- | --- | --- | --- | --- | --- |
| `DR-001` | Distributed business logic / verified architecture | Rules and SQL span routes, services, middleware, migrations, and UI. Evidence: content/pages/organization handlers and service catalog. | All; especially publish, tenant, Marketplace | High / High / `HIGH` | Focused services and OKF maps | Define change-review checklist or consolidate per rule; Yes; Open |
| `DR-002` | Side effects after commit / verified inconsistency | Publish state can commit before plugin/webhook/cache effects; content plugin error can fail response after publication. | Content/Page publication | Medium / High / `HIGH` | Best-effort logging/delivery records | Decide transaction/outbox/compensation semantics; Yes; Open |
| `DR-003` | Media DB/files non-atomic / verified inconsistency | Files, media row, variants, deletion, and audit do not share one transaction; file delete errors ignored. | Media upload/delete | Medium / High / `HIGH` | Tenant directories and defensive path removal | Add compensation/orphan reconciliation tests/design; Yes; Open |
| `DR-004` | Last-owner race / suspected inconsistency | Count check is separate from mutation and not protected by a verified lock. | Tenant ownership/member operations | Low-Medium / High / `HIGH` | Application count guard | Add transactional lock/constraint strategy and concurrency test; Yes; Open |
| `DR-005` | Schema changes can invalidate entries / missing enforcement | Content type update replaces schema without migrating/revalidating stored entries. | Content modeling/save/publish | Medium / High / `HIGH` | New writes validate current schema | Define schema-evolution policy and negative tests; Yes; Open |
| `DR-006` | Frontend/backend validation drift / verified inconsistency | Frontend exposes a subset of field types and does not reproduce all dynamic rules. | Content/Page validation | High / Medium / `MEDIUM` | Backend authoritative validation | Generate/share schemas or document deliberate subset; Yes; Open |
| `DR-007` | Page prop semantics incomplete / missing evidence | Tree validator checks component registration and shapes, not values against `props_schema`. | Page Builder save/template import | Medium / Medium / `MEDIUM` | Frontend property controls | Define server prop validation or explicitly accept opaque props; Yes; Open |
| `DR-008` | Page snapshot concurrency / suspected inconsistency | `MAX(version)+1` with no optimistic token can conflict under concurrent writes; no conflict UI. | Page save/version restore | Medium / Medium / `MEDIUM` | Unique constraint and transaction prevent duplicate persistence | Add locking/optimistic concurrency and tests; Yes; Open |
| `DR-009` | Non-durable webhook delivery / verified architecture | Spawned tasks have no queue, retry, outbox, or process-loss recovery. | Publication webhook workflow | Medium / High / `HIGH` | Delivery attempt rows and timeouts | Establish delivery guarantee and retry/idempotency policy; Yes; Open |
| `DR-010` | Cache consistency best effort / missing guarantee | Invalidations do not fail mutations and direct DB updates bypass them. | Public delivery/publication | Medium / Medium / `MEDIUM` | Bounded cache keys/TTL behavior | Add cache-consistency tests and operational purge; Yes; Open |
| `DR-011` | State transition enforcement uneven / verified inconsistency | Editorial has a state machine; beta, operations, billing, and Marketplace statuses often validate only destination values. | Multiple stateful domains | High / Medium / `MEDIUM` | DB allowlists and route-specific guards | Catalog allowed-from/to matrices and enforce/test critical ones; Yes; Open |
| `DR-012` | Shared Page model drift / documentation-code conflict | `models::PageStatus` omits `pending_review` supported by DB and routes. | Page/editorial/API | High / Medium / `MEDIUM` | Route responses use strings/WorkflowStatus | Align model or remove misleading type in implementation phase; No; Open |
| `DR-013` | Tenant parent-child coherence / missing evidence | Separate organization and parent FKs do not universally prove same-tenant relationship. | Tenant data graph | Low-Medium / Critical / `HIGH` | RLS, triggers, tenant filters | Add composite constraints/live isolation tests where needed; Yes; Open |
| `DR-014` | Public tenant routing ambiguity / documentation conflict | Delivery always resolves active `default`; custom-domain records do not establish full routing workflow. | Delivery/tenant/domain settings | High / High / `HIGH` | Fixed predictable default lookup | Confirm public multi-tenant product contract; Yes; Open |
| `DR-015` | Audit coverage inconsistent / missing evidence | Some destructive/admin writes audit, while publication and comment/plugin changes lack equivalent records. | All regulated workflows | Medium / Medium / `MEDIUM` | Shared audit service | Define audit-required action matrix and transaction policy; Yes; Open |
| `DR-016` | Core workflow test gaps / missing evidence | Organization, content, page, media, webhook, tenant, and transaction paths lack end-to-end coverage. | Core CMS workflows | High / High / `HIGH` | Pure unit tests and DB constraints | Add negative auth/tenant/transition/rollback suites; No; Open |
| `DR-017` | Global plugin enablement scope / owner ambiguity | A built-in plugin enabled flag is global and affects all tenant content saves. | Plugin/content save | Medium / Medium / `MEDIUM` | Global plugin manager authorization | Confirm whether tenant override is required; Yes; Open |
| `DR-018` | Marketplace lifecycle density / verified complexity | Creator, review, install, runtime, finance, feedback, and readiness rules overlap route/service families. | Marketplace workflows/entities | High / High / `HIGH` | Strong focused service/static tests | Phase 9 should produce detailed extensibility contracts without changing behavior; Yes; Open |

## Classification Notes

- `DR-002`, `DR-003`, `DR-006`, `DR-009`, `DR-011`, and `DR-012` are verified implementation characteristics or inconsistencies.
- `DR-004` and `DR-008` are suspected concurrency risks; no failing concurrency test proves exploitation.
- `DR-005`, `DR-007`, `DR-010`, `DR-013`, `DR-015`, and `DR-016` primarily represent missing enforcement or evidence.
- `DR-014` is a current code/documentation/product-contract conflict, not proof that fixed-default delivery is defective.

