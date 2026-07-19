---
okf_document_id: "domain-validation-rules"
title: "Validation Rules"
project: "ZinharCMS"
category: "domain"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/services/entry_validation.rs"
  - "backend/src/routes"
  - "backend/migrations"
  - "frontend/src"
related_documents:
  - "business-rule-catalog.md"
  - "page-builder-rules.md"
  - "../frontend/forms-and-validation.md"
related_diagrams: []
---

# Validation Rules

Validation is distributed. Structural/type extraction, domain rules, authorization, database constraints, and frontend usability checks are separate concerns.

| ID | Input/entity | Condition and enforcement layers | Error behavior | Tests/confidence | Duplication/conflict |
| --- | --- | --- | --- | --- | --- |
| `VAL-001` | Content type | Nonempty name; slug syntax; fields object and supported field schema; handler + DB | `AppError::Validation` or constraint | No route tests; High | Frontend required fields but no full schema parity |
| `VAL-002` | Field schema | snake_case identifier; types `text`, `longtext`, `richtext`, `number`, `boolean`, `datetime`, `media`, `relation`, `json`, `slug` | Validation with field/type | No direct tests located; High | Frontend content-type builder exposes only seven types: `PARTIALLY_ENFORCED_RULE` |
| `VAL-003` | Entry data | Object; required presence; type/min/max/length/slug; service | Validation with field context | Some security/plugin tests, validator coverage unclear | Frontend required/type inputs duplicate subset |
| `VAL-004` | Entry rich text | Sanitizer removes scripts/attributes before validation | Sanitized value persists | Security unit tests; High | Frontend editor does not provide equivalent security authority |
| `VAL-005` | Page request | Nonempty title, slug, valid page tree | Validation/constraint | No route tests; High | Frontend normalizes tree but backend is authoritative |
| `VAL-006` | Page tree | Root/type/ID/objects/children/depth 12/nodes 500/registered components | Validation | No dedicated tests; High | Frontend does not surface depth/count rules |
| `VAL-007` | Component definition | slug-shaped key/category, nonempty name, object prop schemas, supported prop types | Validation/constraint | No tests found; High | Frontend property support and backend schema allowlist may drift |
| `VAL-008` | Workflow request | Current and target states must match `WorkflowStatus` graph | Validation | Three unit tests; High | DB enum limits values but not transitions |
| `VAL-009` | Organization | Nonempty name, normalized slug, settings object | Validation/constraint | No route tests; High | Settings keys have no typed validation |
| `VAL-010` | Member role | Six organization roles only; owner branch has stronger authorization | Validation/forbidden | RBAC matrix test; High | Frontend role choices are usability only |
| `VAL-011` | Invitation | Normalized email, valid role, capacity, pending uniqueness, nonempty token on accept | Validation/not found/constraint | No end-to-end tests; High | Capacity check can race |
| `VAL-012` | Media upload | Required file, max size, detected allowed MIME, declaration match, plan capacity | Validation/bad request | No route tests; High | Browser `accept` behavior is not authoritative |
| `VAL-013` | Webhook | Nonempty name, safe HTTP(S) URL, supported nonempty events, secret at least 16 chars | Validation | URL/event/signature unit tests; High | DB URL check is weaker than service SSRF checks |
| `VAL-014` | Public delivery query | Bounded pagination, allowed sort/filter field, locale syntax | Validation | Filter/XML tests; Medium | Dynamic filter values are strings; content field types are not reused |
| `VAL-015` | Plan/usage | Limits at least -1; values nonnegative; supported metric/month start | Constraint/service error | Quota tests; High | Product rationale for limits is unverified |
| `VAL-016` | Beta records | Enumerated category/severity/status/priority; bounded required/optional text; metadata object | Validation/constraint | Beta helper tests; High | Destination state is validated, transition history is not |
| `VAL-017` | Marketplace manifest/package | Required schema, semantic version, permission/product type allowlists, artifact size/path/content/security/compatibility | Service validation and persisted report | Strong unit/static tests; High | Multiple validators are intentional stages; exact error vocabulary differs |
| `VAL-018` | Marketplace finance/reviews | Amount equations, currency, rating/body ranges, status allowlists, entitlement/eligibility | Service + DB | Finance/feedback/frontend tests; High | Provider failures and dynamic payloads are branch-specific |

## Authorization Is Not Input Validation

Authentication, global/organization RBAC, tenant membership, creator ownership, entitlements, and runtime permissions decide who may perform an otherwise valid action. They are not interchangeable with structural validation. See [Phase 7 Authorization](../security/authorization-architecture.md).

## Main Validation Conflicts

- `VALIDATION_RULE_UNCLEAR`: the frontend content model builder exposes fewer field types than the backend accepts.
- `VALIDATION_RULE_UNCLEAR`: `DynamicForm` renders relation/media/json/slug fields with generic controls and does not reproduce all backend range/slug/reference checks.
- `DOCUMENTATION_CODE_CONFLICT`: shared `PageStatus` omits `pending_review`.
- `PARTIALLY_ENFORCED_RULE`: database checks validate JSON container shape but not full dynamic content/page semantics.
- `FRONTEND_ONLY_RULE`: browser confirmation dialogs and Page Builder autosave rules do not strengthen backend authorization or constraints.

