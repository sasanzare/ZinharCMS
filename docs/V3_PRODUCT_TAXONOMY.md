# V3 Marketplace Product Taxonomy

This taxonomy defines how Marketplace submissions are classified during V3.

## Product Types

| Type | Priority | Accepted In Initial V3 | Description |
| --- | --- | --- | --- |
| Component Pack | P0 | Yes | A package of reusable visual page-builder components. |
| Design Template | P0 | Yes | A page, layout, or starter template that imports into an organization. |
| Integration Plugin | P1 | Later | A reviewed integration with an external service through public contracts. |
| Backend Extension | P2 | Deferred | A controlled backend behavior extension, allowed only after sandboxing. |
| Unsupported | P0 decision | No | Anything outside the accepted Marketplace contract or risk boundary. |

## Component Pack

A Component Pack is accepted when it:

- provides page-builder blocks, sections, or visual components
- declares component schemas in the manifest
- ships static assets or safe client-side behavior only
- works inside the existing page-builder runtime
- does not require direct database access
- does not request network access unless a later permission model allows it

Examples:

- Hero section
- Pricing table
- Testimonial section
- Contact form section without external delivery code

## Design Template

A Design Template is accepted when it:

- imports page layout JSON into one organization
- declares required components and assets
- has no dependency on another organization's data
- can be previewed before installation
- can be removed or rolled back

Examples:

- SaaS home page
- Blog layout
- Landing page bundle
- Product launch page

## Integration Plugin

An Integration Plugin is accepted later when it:

- uses public integration contracts
- declares all permissions in the manifest
- passes security review
- has predictable network targets
- can be disabled without corrupting organization content

Examples:

- Analytics integration
- Email provider adapter
- CRM sync connector
- Webhook adapter

## Backend Extension

A Backend Extension is deferred until sandboxing and runtime controls exist. It may eventually include:

- custom workflow action
- delivery modifier
- validation hook
- controlled automation

It is not accepted for production execution in the first V3 MVP.

## Unsupported

A submission is Unsupported when it:

- executes arbitrary server-side code without sandboxing
- requires direct database credentials
- bypasses tenant context or RBAC
- modifies another organization's data
- cannot be reviewed from source, manifest, and package checksum
- cannot be disabled, rolled back, or blocked
- hides network behavior
- collects secrets without a declared permission model
- depends on external paid placement, advertising, bidding, affiliate, or coupon systems

Unsupported submissions are rejected before public review. When risk is unclear, the submission is quarantined for internal evaluation.