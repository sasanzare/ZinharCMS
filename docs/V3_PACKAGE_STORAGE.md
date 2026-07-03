# V3 Package Storage And Checksum Contract

Marketplace package artifacts are untrusted input. Phase 1 defines the storage and checksum contract that later upload, review, and install APIs must enforce.

## Storage Model

Each package version stores artifact metadata on `marketplace_versions`:

- `artifact_object_key`
- `artifact_sha256`
- `artifact_size_bytes`
- `artifact_file_name`
- `artifact_content_type`
- `storage_metadata`

The package registry stores artifacts by object key. Files must not be executed directly from upload storage.

## Object Key Format

Object keys are deterministic and scoped by creator, listing, version, and checksum:

```text
marketplace/packages/{creator_slug}/{listing_slug}/{version}/{sha256}.zip
```

Rules:

- creator slug must be lowercase ASCII slug format
- listing slug must be lowercase ASCII slug format
- version must be semantic version format
- checksum must be lowercase SHA-256 hex
- object key must not contain user-controlled path traversal

## Checksum Rules

- The checksum algorithm is SHA-256.
- The stored value must be 64 lowercase hex characters.
- The package bytes must match `artifact_sha256` before review.
- The package bytes must match `artifact_sha256` before install.
- A checksum mismatch blocks review and install.

## Size Limit

Phase 1 sets the maximum package artifact size to 50 MiB:

```text
52428800 bytes
```

Packages larger than this limit are rejected before review.

## Metadata Rules

`storage_metadata` must be a JSON object. Suggested fields:

```json
{
  "uploaded_by": "user-id",
  "source": "creator-portal",
  "scanner": "static-validation-v1"
}
```

Do not store secrets in storage metadata.

## Immutability Rules

Approved, deprecated, or blocked package version artifact fields are immutable:

- `version`
- `manifest_json`
- `artifact_object_key`
- `artifact_sha256`
- `artifact_size_bytes`
- `artifact_file_name`
- `artifact_content_type`

If a creator needs to change package contents, they must submit a new version.

## Review And Install Gate

A package version cannot move to review or install unless it has:

- valid manifest
- object key
- valid SHA-256 checksum
- size within limit
- file name
- content type
- storage metadata object