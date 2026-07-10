# V3 Marketplace Phase 8 — Runtime Adapters

Phase 8 connects the committed Phase 7 policy boundary to three safe, host-owned
adapter surfaces. Package code is still never executed; every adapter reads
reviewed manifest data, checks the active installation and kill-switch state, and
keeps organization-owned data behind tenant RLS.

## 8.1 Component Pack Runtime

`GET /api/marketplace/runtime/components` materializes declared `components`
manifest entries into the organization-scoped `component_registry`. Keys are
namespaced by organization, listing, and installation, so a package cannot
replace another organization’s component. The Page Builder merges this registry
with the system palette. Only active, runtime-ready `component_pack`
installations are exposed.

Example manifest extension:

```json
{
  "components": [{
    "key": "hero",
    "name": "Hero",
    "category": "sections",
    "props_schema": {"title": {"type": "text", "label": "Title"}}
  }]
}
```

## 8.2 Template Import

`POST /api/marketplace/templates/{installation_id}/preview` validates a
design-template installation and returns a resolved, non-persistent page JSON
preview. `POST /api/marketplace/templates/{installation_id}/import` creates a
new organization page, its first version snapshot, an import record, and an
audit event. Template nodes are validated against the organization component
registry; the imported page has no source-organization reference.

Templates may declare `asset:<key>` references. The request supplies
`asset_mapping` from those keys to organization media UUIDs. Every mapped media
row is checked for ownership before preview or import; missing or cross-tenant
assets are rejected.

## 8.3 Plugin Hook MVP

`GET /api/marketplace/hooks` exposes public hooks declared by active
`integration_plugin` installations. The supported contract types are:

- `sidebar.item`
- `dashboard.widget`
- `form.field`
- `webhook.adapter`

`POST /api/marketplace/hooks/{hook_type}/authorize` validates a hook against
the public contract and active installation. It returns `execution =
not_executed`; actual adapter execution remains host-owned and is not delegated
to uploaded package code.

## Schema and safety

Migration `0021_v3_phase_eight_runtime_adapters.sql` adds the installation link
for registry components, tenant-scoped template import records, and forced-RLS
public hook records. Runtime and adapter endpoints reject disabled/uninstalled
or kill-switched installations and preserve Phase 7’s policy-only boundary.

## Acceptance

- An installed Component Pack appears in the same organization’s Page Builder palette.
- A Design Template can be previewed and cloned as an independent page with verified asset mapping.
- An Integration Plugin can register only the four public hook contracts.
- All adapter reads and writes are tenant-scoped and audited where data is created.
- No uploaded Marketplace package code is executed.
