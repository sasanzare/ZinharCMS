# V3 Marketplace Manifest Schema

Marketplace package versions must include a manifest before they can be submitted for review. The manifest is stored in `marketplace_versions.manifest_json` and validated by `backend/src/services/marketplace_manifest.rs`.

## Required Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `manifest_version` | string | yes | Manifest contract version. Phase 1 uses `2026-07`. |
| `name` | string | yes | Human-readable package name. |
| `version` | string | yes | Semantic package version, such as `1.0.0`. |
| `type` | string | yes | Product type. |
| `permissions` | string array | yes | Requested Marketplace permissions. |
| `compatibility` | object | yes | Supported ZinharCMS versions and product runtime limits. |
| `entry_points` | object | yes | Runtime or import entry points. |
| `assets` | array | yes | Files referenced by the package. |

## Supported Product Types

- `component_pack`
- `design_template`
- `integration_plugin`
- `backend_extension`

`component_pack` and `design_template` are the first supported V3 MVP product types. `integration_plugin` and `backend_extension` can be represented in the manifest now but remain blocked by later review, permission, and sandbox phases before production execution.

## Supported Permissions

Phase 1 reserves the initial permission vocabulary:

- `content.read`
- `content.write`
- `page.read`
- `page.write`
- `media.read`
- `media.write`
- `webhook.send`
- `settings.read`
- `external_network.request`

Unsupported permissions must reject the manifest before review.

## Compatibility Object

Required shape:

```json
{
  "min_zinhar_version": "2.0.0",
  "max_zinhar_version": "3.0.0"
}
```

Rules:

- `min_zinhar_version` is required.
- `max_zinhar_version` is optional.
- Version values must use semantic version format.
- Compatibility is checked before review and again before install.

## Entry Points

`entry_points` must be a non-empty object. Supported keys depend on product type and later runtime phases.

Examples:

```json
{
  "components": "components/index.json"
}
```

```json
{
  "template": "templates/landing-page.json"
}
```

## Assets

`assets` must be an array. Entries can be strings or structured objects in later phases. Package-level artifact integrity is enforced through `artifact_sha256` on `marketplace_versions`.

## Example Manifest

```json
{
  "manifest_version": "2026-07",
  "name": "SaaS Hero Pack",
  "version": "1.0.0",
  "type": "component_pack",
  "permissions": ["page.read"],
  "compatibility": {
    "min_zinhar_version": "2.0.0",
    "max_zinhar_version": "3.0.0"
  },
  "entry_points": {
    "components": "components/index.json"
  },
  "assets": ["components/hero.json", "assets/preview.png"]
}
```

## Rejection Rules

Reject a manifest when:

- a required field is missing
- the root value is not an object
- `type` is unsupported
- `version` is not semantic
- `permissions` is not an array of supported permission strings
- `compatibility` is missing `min_zinhar_version`
- `entry_points` is missing or empty
- `assets` is not an array