# V3 Marketplace Phase 12 — Creator Tooling and Samples

Phase 12 makes the Marketplace usable by external creators before they upload a
package. It adds local tooling, creator documentation, and sample packages. It
does not change Marketplace runtime execution, review authority, pricing,
entitlements, or backend upload/review decisions.

## 12.1 CLI / SDK Packaging

The repository now includes a dependency-free Node CLI at
`scripts/marketplace-cli.mjs`, exposed through:

```powershell
npm run marketplace -- --help
```

Supported commands:

| Command | Purpose |
| --- | --- |
| `validate <package-dir|package.zip>` | Validate `manifest.json`, permissions, compatibility, entry points, assets, package file tree, adapter declarations, and security findings before upload. |
| `pack <package-dir>` | Run local validation and create a ZIP artifact in `marketplace-dist/` or a caller-provided `--out` path. |
| `submit <package-dir|package.zip>` | Upload a validated package to `POST /api/marketplace/listings/{listing_id}/versions/upload` using `file` and `manifest` multipart fields. |

Local validation mirrors the backend contract where practical:

- manifest schema version `2026-07`;
- supported product types and permissions from `marketplace_manifest.rs`;
- semantic versions including pre-release/build metadata;
- entry point and asset path safety;
- ZIP central directory/file tree checks;
- package size, file count, and uncompressed-size limits;
- forbidden secret files and executable artifacts;
- sensitive permissions and external references;
- Phase 8 component, design-template, and public hook declaration shapes.

The backend remains the final authority. The CLI is a creator-side preflight tool
so creators can find manifest and packaging mistakes before using the web UI or
the upload API.

## 12.2 Documentation and Sample Packages

The creator guide is available at `docs/MARKETPLACE_CREATOR_GUIDE.md`.

Sample packages are available under:

- `docs/marketplace-samples/component-pack`
- `docs/marketplace-samples/integration-plugin`

Both samples can be validated and packed locally:

```powershell
npm run marketplace -- validate docs/marketplace-samples/component-pack
npm run marketplace -- pack docs/marketplace-samples/component-pack --force

npm run marketplace -- validate docs/marketplace-samples/integration-plugin
npm run marketplace -- pack docs/marketplace-samples/integration-plugin --force
```

The integration-plugin sample intentionally requests `webhook.send`, which is a
medium-risk permission. Local validation reports it as a security finding, but it
does not block packaging. High or critical findings block packaging because the
backend review pipeline blocks those submissions.

## Submit Contract

For API-based package submission:

```powershell
$env:ZINHAR_API_URL = "http://localhost:8080"
$env:ZINHAR_TOKEN = "<access token>"
$env:ZINHAR_ORGANIZATION_ID = "<organization id>"
npm run marketplace -- submit docs/marketplace-samples/component-pack --listing-id "<listing id>"
```

The CLI sends:

- `Authorization: Bearer <token>`
- `X-Organization-Id: <organization id>`
- multipart field `manifest` with the manifest JSON;
- multipart field `file` with the ZIP artifact.

## Acceptance

- [x] Creators can run local `validate`, `pack`, and `submit` commands.
- [x] Local validation reports manifest errors before upload.
- [x] The pack command creates a ZIP artifact from a package directory.
- [x] The submit command targets the existing Marketplace version upload API.
- [x] Creator documentation covers manifest, permissions, review policy, CLI
  workflow, and sample packages.
- [x] Component Pack and Integration Plugin sample packages are present and pass
  local validation.
- [x] Uploaded package code remains unexecuted; this phase adds creator tooling
  only.
