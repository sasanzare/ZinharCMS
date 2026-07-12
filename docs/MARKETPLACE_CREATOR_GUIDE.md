# Marketplace Creator Guide

This guide explains how to build a ZinharCMS Marketplace package locally before
uploading it for review. The current Marketplace supports reviewed package
metadata, installation lifecycle, host-owned adapters, one-time purchase flows,
feedback/abuse moderation, and analytics. Uploaded package code is still never
executed by ZinharCMS.

## Package Layout

A package directory should include `manifest.json` at the root plus every file
referenced by `entry_points` and `assets`.

```text
my-package/
  manifest.json
  components/
    index.json
    hero-banner.json
  assets/
    preview.txt
```

Use the Phase 12 CLI from the repository root:

```powershell
npm run marketplace -- validate my-package
npm run marketplace -- pack my-package --force
```

Generated ZIP files are written to `marketplace-dist/` by default. That directory
is ignored by Git.

## Manifest Basics

Required manifest fields:

| Field | Type | Notes |
| --- | --- | --- |
| `manifest_version` | string | Must be `2026-07`. |
| `name` | string | Human-readable package name. |
| `version` | string | Semantic version, for example `1.0.0`. |
| `type` | string | One of the supported product types. |
| `permissions` | string array | Requested Marketplace permissions. |
| `compatibility` | object | ZinharCMS version/plan/feature requirements. |
| `entry_points` | object | Artifact-local package paths. |
| `assets` | string array | Artifact-local package paths referenced by the package. |

Supported product types:

- `component_pack`
- `design_template`
- `integration_plugin`
- `backend_extension`

`backend_extension` is modeled in the manifest vocabulary, but it is blocked
until a separately reviewed sandbox runtime exists.

## Permissions

Supported permission keys:

- `content.read`
- `content.write`
- `page.read`
- `page.write`
- `media.read`
- `media.write`
- `webhook.send`
- `settings.read`
- `external_network.request`

Permission guidance:

- Request the smallest set required by your package.
- Write permissions and `webhook.send` are treated as sensitive and need careful
  review.
- `external_network.request` is high risk and blocks local packaging because the
  current Marketplace does not execute arbitrary external-network package code.

## Component Pack Example

See `docs/marketplace-samples/component-pack`.

Validate and pack it:

```powershell
npm run marketplace -- validate docs/marketplace-samples/component-pack
npm run marketplace -- pack docs/marketplace-samples/component-pack --force
```

The sample declares a `components` array with a safe Page Builder component
definition. When installed and active, Phase 8 host adapters namespace those
definitions into the organization component registry.

## Integration Plugin Example

See `docs/marketplace-samples/integration-plugin`.

Validate and pack it:

```powershell
npm run marketplace -- validate docs/marketplace-samples/integration-plugin
npm run marketplace -- pack docs/marketplace-samples/integration-plugin --force
```

The sample declares a public `webhook.adapter` hook. ZinharCMS authorizes the
public hook contract but returns `execution = not_executed`; package code is not
delegated to uploaded artifacts.

## Submit With the CLI

The submit command targets the existing version upload API:

```powershell
$env:ZINHAR_API_URL = "http://localhost:8080"
$env:ZINHAR_TOKEN = "<access token>"
$env:ZINHAR_ORGANIZATION_ID = "<organization id>"
npm run marketplace -- submit my-package --listing-id "<listing id>"
```

Equivalent explicit flags:

```powershell
npm run marketplace -- submit my-package `
  --api-url "http://localhost:8080" `
  --token "<access token>" `
  --organization-id "<organization id>" `
  --listing-id "<listing id>"
```

The CLI sends multipart fields `manifest` and `file` to:

```text
POST /api/marketplace/listings/{listing_id}/versions/upload
```

The creator must already have an approved creator profile and a complete listing
that belongs to them. The backend review pipeline remains the final source of
truth for validation, review status, risk level, and publication.

## Review Policy Summary

Before approval, packages are checked for:

- required manifest fields and supported product type;
- semantic package version;
- supported permissions;
- safe artifact-local entry point and asset paths;
- readable ZIP structure;
- missing files, duplicate paths, and path traversal;
- forbidden secret files such as `.env`, private keys, and credential files;
- executable artifacts such as `.exe`, `.dll`, `.sh`, `.ps1`, `.jar`;
- external references and remote dependency sources;
- compatibility with ZinharCMS version, plan, and supported Marketplace features.

High or critical risk packages are blocked. Medium-risk findings can enter human
review but should be justified in the package description and support notes.
