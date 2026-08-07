# Legacy Metadata and Provenance Translation

Phase 1 does not create the target schema. This document records the semantic
treatment that Phase 2 should apply when designing Google OKF v0.2 concepts.
The legacy frontmatter is not a drop-in schema: it mixes concept identity,
navigation, implementation classification, review bookkeeping, source
provenance, phase history, and unresolved uncertainty.

Google OKF v0.2 has one always-required concept field, type. It also provides
optional title/description/resource/tags, sources, generated, verified, status,
stale_after, computations, and extension keys. The translation below preserves
meaning rather than field names.

## Inventory and treatment table

| Legacy field(s) | Present in legacy docs | Legacy meaning | Proposed future treatment | Translation rule |
| --- | ---: | --- | --- | --- |
| okf_document_id | 339 | Custom document identifier independent of the file path | DROP | Derive the Google concept ID from the final bundle path; retain the old ID only in historical provenance if needed. |
| title | 339 | Human-readable document title | MAP_TO_GOOGLE_OKF | Use title when it remains accurate; otherwise regenerate from the approved concept name. |
| project | 339 | Project label, always ZinharCMS in the legacy corpus | MOVE_TO_BODY | The project identity belongs in the Project concept body or links, not a non-standard project key. |
| category | 339 | Legacy taxonomy/category | MAP_TO_GOOGLE_OKF | Use it as input to a descriptive Google type such as API Contract, Database Model, or Domain Workflow; do not treat the old category list as a registry. |
| phase | 339 | Legacy documentation phase and sequencing history | MOVE_TO_BODY | Preserve phase history in a historical/decision section or provenance record; do not use it as current lifecycle status. |
| status | 339 | Legacy current marker; all frontmatter docs currently say current | MAP_TO_GOOGLE_OKF | Translate only after review: current verified knowledge may become stable, planned knowledge draft, and retired knowledge deprecated. Do not copy current mechanically. |
| review_status | 216 | Legacy verified/mixed review label | DERIVE_WHEN_GENERATING | Produce Google verified only when actor and timestamp are known; derive trust from the actual verifier, not from the bare label. |
| source_of_truth | 339 | Boolean claim that the document is or is not authoritative | DERIVE_WHEN_GENERATING | Authority must be derived from the source hierarchy and governance decision; no direct Google core equivalent exists. |
| last_verified_commit | 339 | Git commit used by the legacy review | KEEP_AS_EXTENSION_CANDIDATE | Preserve as optional commit/source lineage when useful, but do not turn a commit hash into a Google verified actor. |
| last_verified_date | 339 | Date of the legacy review snapshot | DERIVE_WHEN_GENERATING | Use it as source or verification time only when the reviewing actor and scope are known; derive stale_after from policy, never from a guessed interval. |
| primary_sources | 339 | Repository paths treated as primary evidence | MAP_TO_GOOGLE_OKF | Convert each material source to sources[].resource with a stable source id and suitable process/human author. Validate path scope at generation time. |
| related_documents | 339 | Cross-document navigation and relationships | MOVE_TO_BODY | Re-express useful relationships as ordinary Markdown links whose surrounding prose explains the relationship. |
| related_diagrams | 298 | Links from concepts to Mermaid source files | MOVE_TO_BODY | Link a retained or regenerated diagram from the concept body; do not make diagram metadata an unvalidated global registry. |
| uncertainty_markers | 169 | UNKNOWN, owner-confirmation, conflict, and implementation-status labels | KEEP_AS_EXTENSION_CANDIDATE | Preserve unresolved markers in a decision/risk extension or body section; never silently resolve them during generation. |
| implementation_view | 323 | observed, inferred, planned, or similar implementation stance | KEEP_AS_EXTENSION_CANDIDATE | Retain a project-specific classification only if it remains useful; current trust must still come from sources/verified/status. |
| implementation_status | 77; security_status 40; boundary_status 41; architecture_status 10; domain_status 10; compatibility_status 1; extensibility_status 39 | Topic-specific state labels | KEEP_AS_EXTENSION_CANDIDATE | Preserve as extensions only after vocabulary review; translate a value to status only when its lifecycle semantics match draft/stable/deprecated. |
| confidence | 18 | Legacy confidence annotation | KEEP_AS_EXTENSION_CANDIDATE | Keep as an advisory extension or body statement; Google OKF does not define a project confidence field. |
| assignment_type; registration_type; module_type; plugin_type | 11; 6; 18; 1 | Local classification of module/extension/registration kind | KEEP_AS_EXTENSION_CANDIDATE | Keep only where it explains a current boundary; otherwise move the meaning into the concept body. |
| domain_id; domain_name; domain_status | 10 each | Legacy domain catalog identity and state | KEEP_AS_EXTENSION_CANDIDATE | Derive final concept identity from the bundle path and describe domain boundaries in body/links. |
| entity_id; entity_name; entity_domain | 18 each | Legacy database entity catalog identity | KEEP_AS_EXTENSION_CANDIDATE | Use as input to Database/Domain concepts; do not preserve a parallel entity registry without an owner decision. |
| feature_id; feature_name; feature_paths | 13 each | Frontend feature taxonomy and source paths | KEEP_AS_EXTENSION_CANDIDATE | Source paths can become sources; the feature label belongs in type/title/body after current verification. |
| module_id; module_name; module_paths; owning_module; module_type | 18 each for module_id/module_name/module_paths/module_type; owning_module 18 | Backend module catalog and ownership relationship | KEEP_AS_EXTENSION_CANDIDATE | Preserve useful boundaries and source paths; leave accountable ownership unresolved until NOC-15 is answered. |
| marketplace_area_id; marketplace_area_name | 6 each | Legacy Marketplace area taxonomy | KEEP_AS_EXTENSION_CANDIDATE | Use as candidate grouping input; final Marketplace concept boundaries come from verified route/schema/service evidence. |
| permission_group_id; permission_group_name; permission_scope | 8 each | Permission-group catalog | KEEP_AS_EXTENSION_CANDIDATE | Preserve as RBAC extension/body data; do not imply that names alone authorize an operation. |
| role_id; role_name; role_scope | 11 each | Global and organization role catalog | KEEP_AS_EXTENSION_CANDIDATE | Map verified role behavior into Authorization concepts and keep role identifiers as local provenance only. |
| resource_domain; tenant_scope | 8; 18 | Ownership/tenant scope annotations | KEEP_AS_EXTENSION_CANDIDATE | Preserve only after comparing route checks, ownership queries, and RLS; never use a label as proof of isolation. |
| workflow_id; workflow_name; workflow_domain | 14 each | Domain workflow catalog | KEEP_AS_EXTENSION_CANDIDATE | Use as body headings or extension identifiers; verify state transitions and side effects from source. |
| schema_objects | 18 | Database object names associated with an entity | MAP_TO_GOOGLE_OKF | Convert validated object paths/names into sources or body schema tables; regenerate from migrations rather than copying lists. |
| plugin_id; plugin_name; plugin_scope | 1 each | Built-in plugin identity and scope | KEEP_AS_EXTENSION_CANDIDATE | Preserve in the Extensibility concept only if the current plugin boundary remains supported. |

No legacy file has a Google-required type field. No legacy frontmatter field
should be copied to type without a candidate-level decision; category and
document shape are only inputs to that decision.

## Provenance translation plan

| Legacy semantic | Google OKF v0.2 treatment | Required safeguard |
| --- | --- | --- |
| Repository source paths | sources entries with resource, stable id, title where useful, and a process author | Check that every path exists at the generation snapshot and distinguish current source from historical source. |
| Legacy review date/commit | sources.last_modified or a verified entry when the actor and scope are known | A date alone is not a human verification; do not copy review_status blindly. |
| Machine-generated catalogs | generated.by with a process actor and generated.at, plus sources for the inputs | Record the generator contract and input snapshot; regenerate endpoint/schema/index data. |
| Human review | verified entry with human: actor and timestamp | Require an identified reviewer and preserve failed/stale results instead of silently promoting them. |
| Current/mixed/planned markers | status draft/stable/deprecated only after semantic translation | current is not automatically stable; mixed content may need split concepts or draft status. |
| Freshness | stale_after only when a domain owner defines the freshness policy | Do not derive a universal freshness window from legacy last_verified_date. |
| Unknowns and owner questions | body sections, links to DecisionDebt concepts, or a project extension | Keep the question and its status; never convert an unknown into a false current-state statement. |
| Related concepts and diagrams | Standard Markdown links with relationship meaning in surrounding prose | Avoid a central legacy registry as authority; links remain contextual. |
| Git commit identity | Optional source/reference extension or source resource context | A commit identifies a snapshot, not a verifier or attestation. |
| Attestation/computation | Only use type Attested Computation for an actual sanctioned computation contract | The Phase 1 manifest is evidence, not a justification for inventing an attested runtime concept. |

## Metadata decisions that remain open

1. The project owner must decide whether commit hashes remain in a small
   provenance extension or are represented only by immutable Git source URLs.
2. NOC-13 must establish canonical ownership and retirement policy before
   generated status/freshness fields can be trusted.
3. NOC-18 must settle preferred product terminology before type names and tags
   are finalized.
4. NOC-03 through NOC-06 must supply operational evidence before deployment,
   recovery, observability, and retention metadata is promoted to stable.
