---
okf_document_id: "component-block-registration"
title: "Component and Block Registration"
project: "ZinharCMS"
category: "extensibility"
phase: 9
status: "current"
source_of_truth: false
implementation_view: "observed"
extensibility_status: "mixed"
last_verified_commit: "56d733985fdd7aa3f25ee6981b88cf29c52f65c9"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/routes/pages.rs"
  - "backend/src/services/marketplace_adapters.rs"
  - "backend/src/routes/marketplace_adapters.rs"
  - "frontend/src/pages/PagesPage.tsx"
  - "backend/migrations/0001_initial_schema.sql"
  - "backend/migrations/0021_v3_phase_eight_runtime_adapters.sql"
related_documents:
  - "../frontend/page-builder.md"
  - "../domain/page-builder-rules.md"
  - "extension-points/page-builder-component-registry.md"
related_diagrams:
  - "diagrams/component-registration.mmd"
---

# Component and Block Registration

The Page Builder registry stores component_key, name, category, props_schema, is_system, organization_id, and optional marketplace_installation_id. System components are seeded. Organization managers can CRUD custom metadata through the Pages API. Marketplace component packs provide manifest components; the adapter namespaces their keys and upserts tenant registry rows.

PagesPage merges normal registry responses with Marketplace runtime component responses. Page JSON validation checks component keys against the system-or-tenant registry.

A registered component is a declarative schema/catalog entry. The inspected frontend still owns rendering behavior; registration does not load package JavaScript. Product declarations use a props_schema object, while template imports provide complete page_json.

COMPONENT_REGISTRATION_UNCLEAR remains for collision ownership, stale-row cleanup after disable/uninstall, and whether every declarative Marketplace component has a corresponding visual renderer. See [Page Builder](../frontend/page-builder.md) and [Page Builder Rules](../domain/page-builder-rules.md).

## Verified System Component Catalog

All rows are seeded by backend/migrations/0004_phase_two_page_builder.sql, use component_registry, serialize as component keys and props inside page_json, require tenant component-read/page permissions for use, and have IMPLEMENTED registry status. Page validation checks key registration and document structure but does not validate every props_schema field value. Editor and published renderer completeness is COMPONENT_REGISTRATION_UNCLEAR.

| Identifier | Display name | Category | Configuration schema summary | Renderer | Validation | Plugin source |
|---|---|---|---|---|---|---|
| hero-banner | Hero Banner | sections | title, subtitle, background image, CTA, alignment | Host Page Builder | Registered key; object props; schema not fully enforced | Built-in migration |
| feature-grid | Feature Grid | sections | columns, features | Host Page Builder | Same | Built-in migration |
| testimonial | Testimonial | sections | quote, author | Host Page Builder | Same | Built-in migration |
| cta-section | CTA Section | sections | title, button text/URL | Host Page Builder | Same | Built-in migration |
| about-section | About Section | sections | title, rich-text body | Host Page Builder | Same | Built-in migration |
| text-block | Text Block | content | required rich-text body | Host Page Builder | Same | Built-in migration |
| image-block | Image Block | content | required image, alt text | Host Page Builder | Same | Built-in migration |
| video-embed | Video Embed | content | required URL | Host Page Builder | Same | Built-in migration |
| code-block | Code Block | content | language, required code | Host Page Builder | Same | Built-in migration |
| quote | Quote | content | required quote, citation | Host Page Builder | Same | Built-in migration |
| two-column | Two Column | layout | gap | Host Page Builder | Same; nesting follows page rules | Built-in migration |
| three-column | Three Column | layout | gap | Host Page Builder | Same; nesting follows page rules | Built-in migration |
| container | Container | layout | maximum width | Host Page Builder | Same; nesting follows page rules | Built-in migration |
| divider | Divider | layout | style | Host Page Builder | Same | Built-in migration |
| spacer | Spacer | layout | height | Host Page Builder | Same | Built-in migration |
| image-gallery | Image Gallery | media | images array | Host Page Builder | Same | Built-in migration |
| carousel | Carousel | media | slides array | Host Page Builder | Same | Built-in migration |
| video-player | Video Player | media | media reference | Host Page Builder | Same | Built-in migration |
| contact-form | Contact Form | forms | required recipient email | Host Page Builder | Same | Built-in migration |
| newsletter-signup | Newsletter Signup | forms | list identifier | Host Page Builder | Same | Built-in migration |
| survey | Survey | forms | questions array | Host Page Builder | Same | Built-in migration |
| header | Header | navigation | menu array | Host Page Builder | Same | Built-in migration |
| footer | Footer | navigation | columns array | Host Page Builder | Same | Built-in migration |
| breadcrumb | Breadcrumb | navigation | show-home flag | Host Page Builder | Same | Built-in migration |
| pricing-table | Pricing Table | data | plans array | Host Page Builder | Same | Built-in migration |
| comparison-table | Comparison Table | data | rows array | Host Page Builder | Same | Built-in migration |
| faq-accordion | FAQ Accordion | data | items array | Host Page Builder | Same | Built-in migration |
| team-grid | Team Grid | data | members array | Host Page Builder | Same | Built-in migration |

## Non-System Registration Types

| Identifier | Display name | Source | Registration path | Renderer | Schema | Validation | Persistence | Permissions | Plugin source | Status |
|---|---|---|---|---|---|---|---|---|---|---|
| Tenant-defined key | Tenant-defined | Component API | Authorized CRUD into component_registry | Host renderer availability unclear | Required JSON object | Key format/uniqueness and page document checks | Tenant registry row plus page_json references | Component manager | Organization user, not executable plugin | IMPLEMENTED registry |
| mp-{namespace}-{source_key} | Manifest-defined | Active component-pack manifest | Host adapter extraction and upsert | Host renderer availability unclear | props_schema or empty object | Public adapter fields, namespacing, registry/page validation | Tenant registry row linked to installation | Installer/permission/runtime and page/component permissions | Marketplace package declaration | PARTIALLY_IMPLEMENTED |

Icons, default properties outside props_schema, dedicated inspector/editor/preview component registration, parent-child type constraints, and renderer version negotiation were not found as public extension contracts.
