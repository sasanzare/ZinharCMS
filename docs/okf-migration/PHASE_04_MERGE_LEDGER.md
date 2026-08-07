# Phase 4 Merge Ledger

**Source HEAD:** `e37e94e2e6960a2547f33bf1ebb4225f818b3a4b`  
**Selected groups:** G002 (Architecture), G003 (Backend), G008 (Frontend)  
**Legacy source records considered:** 72  
**Constructed targets:** `TARGET-008`, `TARGET-012`, `TARGET-016`

This ledger records every legacy input in the three selected merge groups.
`ROUTED_TO_EXISTING_TARGET` means the material already belongs to a Phase 3
Concept. `DEFERRED_OTHER_TARGET` means the material remains represented by a
non-Phase-4 target in the Phase 3 disposition matrix. `DUPLICATE_OMITTED`
means the wrapper or repeated view added no current knowledge. Historical
material is not promoted into current Concepts.

## Target records

### TARGET-008 — Architecture / Integrations and Side Effects

| Field | Record |
| --- | --- |
| Legacy inputs | G002: `okf/architecture/architecture-risks.md`, `boundaries.md`, `components.md`, `decisions/decision-register.md`, `decisions/README.md`, `dependency-model.md`, `integration-points.md`, `overview.md`, `README.md`, `runtime-flows.md`. |
| Material claims | CLAIM-0004 for the current modular-monolith boundary; current cache, webhook, email, file-cleanup, outbound HTTP, audit, and transaction behavior from the Phase 4 source head. |
| Current evidence | `backend/src/routes/mod.rs`, `backend/src/routes/content.rs`, `backend/src/routes/delivery.rs`, `backend/src/routes/pages.rs`, `backend/src/services/cache.rs`, `webhooks.rs`, `email.rs`, `outbound_http.rs`, `file_cleanup.rs`, and `config.rs`. |
| Excluded stale, contradicted, or unverified material | Legacy deployment/provider topology, universal atomicity, retry/compensation guarantees, and owner policy were not corroborated. NOC-02 and NOC-09 remain explicit. The historical decision register and legacy README wrappers were not promoted. |
| Result | Built one current side-effect boundary without adding a diagram. G002 diagrams assigned to request/module concepts remain owned by existing Concepts or deferred targets. |

### TARGET-012 — Backend / Persistence, Services, and Configuration

| Field | Record |
| --- | --- |
| Legacy inputs | G003: `okf/backend/backend-risks.md`, `configuration-and-state.md`, `dependency-map.md`, `error-handling.md`, `module-boundaries.md`, `module-catalog.md`, every `okf/backend/modules/*.md` source listed below, `overview.md`, `persistence-access.md`, `README.md`, `request-handling.md`, `services-and-domain.md`, `shared-infrastructure.md`, and `testing-map.md`. |
| Material claims | CLAIM-0006 for PostgreSQL, Redis, key material, preview settings, and configured upload boundary; current startup, `AppState`, SQLx, environment validation, and service registry behavior. |
| Current evidence | `backend/src/main.rs`, `lib.rs`, `state.rs`, `db/mod.rs`, `config.rs`, `services/mod.rs`, `Cargo.toml`, current route/service composition, and the repository configuration boundary. |
| Excluded stale, contradicted, or unverified material | Production storage provider, shared filesystem/object storage, backup/durability, deployment scaling, durable module ownership, and exhaustive test inventory were not promoted. Feature-specific API/domain mechanics remain with existing or deferred domain/API targets. NOC-02 remains explicit. |
| Result | Built one current persistence/configuration/service-composition view without duplicating the existing runtime and module Concepts and without adding a diagram. |

### TARGET-016 — Frontend / Feature Boundaries

| Field | Record |
| --- | --- |
| Legacy inputs | G008: `okf/frontend/api-client.md`, `application-catalog.md`, `authentication-and-access.md`, `component-architecture.md`, `configuration-and-build.md`, `feature-boundaries.md`, `feature-catalog.md`, every `okf/frontend/features/*.md` source listed below, `forms-and-validation.md`, `frontend-risks.md`, `loading-errors-and-notifications.md`, `overview.md`, `page-builder.md`, `pages-and-layouts.md`, `README.md`, `routing.md`, `state-management.md`, `styling-and-design-system.md`, and `testing-map.md`. |
| Material claims | CLAIM-0014 for centralized frontend API calls, volatile access-token state, and organization context; current React/Vite shell, route, page, shared-component, i18n, and feature API boundaries. |
| Current evidence | `frontend/src/main.tsx`, `router.tsx`, `components/AppShell.tsx`, `services/api.ts`, `stores/useAppStore.ts`, `i18n/I18nProvider.tsx`, current page/component directories, and `package.json`. |
| Excluded stale, contradicted, or unverified material | Future package/team ownership, owner-approved browser/accessibility/compatibility policy, legacy build snapshots, and exhaustive test/build inventory were not promoted. Existing auth/routing concepts retain their detailed behavior; TARGET-017 remains deferred for regeneration. NOC-12 and NOC-18 remain explicit. |
| Result | Built one current feature-boundary view without adding a diagram. Frontend flow/state visuals assigned to existing routing/state Concepts remain unchanged. |

## Complete input outcome crosswalk

### G002 — Architecture (10 inputs)

| Legacy input | Outcome | Current destination or reason |
| --- | --- | --- |
| `okf/architecture/architecture-risks.md` | MERGED_INTO_TARGET | Current side-effect risks and explicit NOC caveats in TARGET-008. |
| `okf/architecture/boundaries.md` | ROUTED_TO_EXISTING_TARGET | Request and module boundaries remain in TARGET-006, TARGET-007, and TARGET-011. |
| `okf/architecture/components.md` | ROUTED_TO_EXISTING_TARGET | Current composition remains in TARGET-006 and TARGET-011. |
| `okf/architecture/decisions/decision-register.md` | HISTORICAL_PRESERVED_OUTSIDE_CURRENT | Historical decision material remains outside the current Phase 4 Concepts. |
| `okf/architecture/decisions/README.md` | DUPLICATE_OMITTED | Legacy directory wrapper adds no current claim. |
| `okf/architecture/dependency-model.md` | ROUTED_TO_EXISTING_TARGET | Dependency direction remains in TARGET-006, TARGET-007, and TARGET-011. |
| `okf/architecture/integration-points.md` | MERGED_INTO_TARGET | Current integrations and side-effect boundaries in TARGET-008. |
| `okf/architecture/overview.md` | ROUTED_TO_EXISTING_TARGET | Repository composition remains in TARGET-006. |
| `okf/architecture/README.md` | DUPLICATE_OMITTED | Legacy directory wrapper adds no current claim. |
| `okf/architecture/runtime-flows.md` | ROUTED_TO_EXISTING_TARGET | Request flow remains in TARGET-007; side-effect portions are consolidated in TARGET-008. |

### G003 — Backend (31 inputs)

| Legacy input | Outcome | Current destination or reason |
| --- | --- | --- |
| `okf/backend/backend-risks.md` | MERGED_INTO_TARGET | Storage/configuration caveats in TARGET-012. |
| `okf/backend/configuration-and-state.md` | MERGED_INTO_TARGET | Current `Config` and `AppState` boundary in TARGET-012. |
| `okf/backend/dependency-map.md` | ROUTED_TO_EXISTING_TARGET | Module dependency view remains in TARGET-011. |
| `okf/backend/error-handling.md` | ROUTED_TO_EXISTING_TARGET | Response/error boundary remains in TARGET-010. |
| `okf/backend/module-boundaries.md` | ROUTED_TO_EXISTING_TARGET | Existing current module Concept remains authoritative. |
| `okf/backend/module-catalog.md` | ROUTED_TO_EXISTING_TARGET | Detailed module inventory remains in TARGET-011. |
| `okf/backend/modules/authentication.md` | DEFERRED_OTHER_TARGET | Authentication details remain in existing security/API Concepts. |
| `okf/backend/modules/beta-release-operations.md` | DEFERRED_OTHER_TARGET | Beta/domain operations remain outside Phase 4. |
| `okf/backend/modules/billing-quotas.md` | DEFERRED_OTHER_TARGET | Billing and quota merge remains TARGET-037. |
| `okf/backend/modules/bootstrap-runtime.md` | ROUTED_TO_EXISTING_TARGET | Startup/runtime behavior remains in TARGET-010. |
| `okf/backend/modules/built-in-plugins.md` | DEFERRED_OTHER_TARGET | Plugin boundary remains in TARGET-039. |
| `okf/backend/modules/cms-webhooks.md` | MERGED_INTO_TARGET | Webhook side-effect behavior is consolidated in TARGET-008. |
| `okf/backend/modules/comments.md` | DEFERRED_OTHER_TARGET | Comments remain with editorial/API domain targets. |
| `okf/backend/modules/content-workflow.md` | DEFERRED_OTHER_TARGET | Editorial workflow remains in TARGET-033. |
| `okf/backend/modules/marketplace-catalog-installation.md` | DEFERRED_OTHER_TARGET | Marketplace contract/domain targets remain deferred or existing. |
| `okf/backend/modules/marketplace-creator-review.md` | DEFERRED_OTHER_TARGET | Marketplace domain targets remain outside Phase 4. |
| `okf/backend/modules/marketplace-feedback-analytics-readiness.md` | DEFERRED_OTHER_TARGET | Marketplace readiness material remains outside Phase 4. |
| `okf/backend/modules/marketplace-finance.md` | DEFERRED_OTHER_TARGET | Marketplace finance/billing material remains outside Phase 4. |
| `okf/backend/modules/marketplace-runtime-adapters.md` | DEFERRED_OTHER_TARGET | Runtime adapter boundary remains in existing/deferred Marketplace Concepts. |
| `okf/backend/modules/media.md` | DEFERRED_OTHER_TARGET | Media/file domain material remains outside Phase 4. |
| `okf/backend/modules/organizations.md` | DEFERRED_OTHER_TARGET | Organization domain material remains outside Phase 4. |
| `okf/backend/modules/pages-builder-preview.md` | DEFERRED_OTHER_TARGET | Page-builder behavior remains in TARGET-034. |
| `okf/backend/modules/public-delivery-cache.md` | MERGED_INTO_TARGET | Cache and public-delivery effect boundary is consolidated in TARGET-008. |
| `okf/backend/modules/tenant-authorization.md` | DEFERRED_OTHER_TARGET | Tenant controls remain in existing security Concepts. |
| `okf/backend/overview.md` | ROUTED_TO_EXISTING_TARGET | Backend process overview remains in TARGET-010. |
| `okf/backend/persistence-access.md` | MERGED_INTO_TARGET | SQLx, pool, tenant connections, and persistence boundary in TARGET-012. |
| `okf/backend/README.md` | DUPLICATE_OMITTED | Legacy directory wrapper adds no current claim. |
| `okf/backend/request-handling.md` | ROUTED_TO_EXISTING_TARGET | Request composition remains in TARGET-007 and TARGET-010. |
| `okf/backend/services-and-domain.md` | ROUTED_TO_EXISTING_TARGET | Service/module composition remains in TARGET-011; cross-cutting effects are in TARGET-008. |
| `okf/backend/shared-infrastructure.md` | MERGED_INTO_TARGET | Shared state, persistence, and configuration boundary in TARGET-012. |
| `okf/backend/testing-map.md` | DEFERRED_OTHER_TARGET | Exhaustive backend validation remains TARGET-013. |

### G008 — Frontend (31 inputs)

| Legacy input | Outcome | Current destination or reason |
| --- | --- | --- |
| `okf/frontend/api-client.md` | ROUTED_TO_EXISTING_TARGET | Shared request and state behavior remains in TARGET-015; feature implications are summarized in TARGET-016. |
| `okf/frontend/application-catalog.md` | MERGED_INTO_TARGET | Current feature area map in TARGET-016. |
| `okf/frontend/authentication-and-access.md` | ROUTED_TO_EXISTING_TARGET | Auth behavior remains in existing auth/admin/routing Concepts. |
| `okf/frontend/component-architecture.md` | ROUTED_TO_EXISTING_TARGET | Shell and shared component boundary remains in TARGET-014; feature boundary implications are in TARGET-016. |
| `okf/frontend/configuration-and-build.md` | DEFERRED_OTHER_TARGET | Build/configuration regeneration remains TARGET-017. |
| `okf/frontend/feature-boundaries.md` | MERGED_INTO_TARGET | Current feature areas in TARGET-016. |
| `okf/frontend/feature-catalog.md` | MERGED_INTO_TARGET | Current feature catalog material in TARGET-016. |
| `okf/frontend/features/authentication-and-session.md` | ROUTED_TO_EXISTING_TARGET | Session behavior remains in TARGET-015 and API/security Concepts. |
| `okf/frontend/features/beta-operations.md` | MERGED_INTO_TARGET | Beta feature boundary is listed in TARGET-016. |
| `okf/frontend/features/billing-and-usage.md` | MERGED_INTO_TARGET | Billing feature boundary is listed in TARGET-016; detailed domain policy remains deferred. |
| `okf/frontend/features/content-entries.md` | MERGED_INTO_TARGET | Editorial feature boundary is listed in TARGET-016; detailed workflow remains TARGET-033. |
| `okf/frontend/features/content-modeling.md` | MERGED_INTO_TARGET | Content-modeling feature boundary is listed in TARGET-016. |
| `okf/frontend/features/dashboard-and-application-shell.md` | ROUTED_TO_EXISTING_TARGET | Shell detail remains in TARGET-014; feature grouping is in TARGET-016. |
| `okf/frontend/features/editorial-workflow-and-collaboration.md` | MERGED_INTO_TARGET | Editorial feature boundary is listed in TARGET-016; detailed workflow remains TARGET-033. |
| `okf/frontend/features/localization-and-direction.md` | MERGED_INTO_TARGET | Shared localization boundary is listed in TARGET-016. |
| `okf/frontend/features/marketplace.md` | MERGED_INTO_TARGET | Marketplace feature boundary is listed in TARGET-016; domain detail remains outside Phase 4. |
| `okf/frontend/features/media-library.md` | MERGED_INTO_TARGET | Media feature boundary is listed in TARGET-016; storage detail remains deferred. |
| `okf/frontend/features/organizations-and-workspaces.md` | MERGED_INTO_TARGET | Organization/workspace feature boundary is listed in TARGET-016. |
| `okf/frontend/features/pages-and-page-builder.md` | MERGED_INTO_TARGET | Page feature boundary is listed in TARGET-016; detailed behavior remains TARGET-034. |
| `okf/frontend/features/settings-and-webhooks.md` | MERGED_INTO_TARGET | Settings/webhook feature boundary is listed in TARGET-016. |
| `okf/frontend/forms-and-validation.md` | MERGED_INTO_TARGET | Shared form boundary is summarized in TARGET-016. |
| `okf/frontend/frontend-risks.md` | MERGED_INTO_TARGET | Current ownership/compatibility caveats are summarized in TARGET-016. |
| `okf/frontend/loading-errors-and-notifications.md` | MERGED_INTO_TARGET | Shared presentation behavior is summarized in TARGET-016. |
| `okf/frontend/overview.md` | ROUTED_TO_EXISTING_TARGET | Application shell overview remains in TARGET-014. |
| `okf/frontend/page-builder.md` | ROUTED_TO_EXISTING_TARGET | Detailed page-builder behavior remains in TARGET-034. |
| `okf/frontend/pages-and-layouts.md` | ROUTED_TO_EXISTING_TARGET | Shell/page routing detail remains in TARGET-014 and TARGET-015. |
| `okf/frontend/README.md` | DUPLICATE_OMITTED | Legacy directory wrapper adds no current claim. |
| `okf/frontend/routing.md` | ROUTED_TO_EXISTING_TARGET | Routing and auth guard behavior remains in TARGET-015. |
| `okf/frontend/state-management.md` | ROUTED_TO_EXISTING_TARGET | Store and token behavior remains in TARGET-015. |
| `okf/frontend/styling-and-design-system.md` | MERGED_INTO_TARGET | Shared presentation and localization boundary is summarized in TARGET-016. |
| `okf/frontend/testing-map.md` | DEFERRED_OTHER_TARGET | Exhaustive frontend validation remains TARGET-017. |

## Diagram disposition

No new diagram was added. The Phase 1/2 diagram mapping assigns the relevant
G002, G003, and G008 visuals to the existing runtime, module, and routing
Concepts or to later regenerate targets. Adding a new visual here would repeat
those views or imply unsupported current topology. The staging bundle retains
its 13 existing embedded Mermaid blocks and zero standalone `.mmd` files.
