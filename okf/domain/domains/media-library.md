---
okf_document_id: "domain-media-library"
title: "Media Library Domain"
project: "ZinharCMS"
category: "domain-detail"
phase: 8
status: "current"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "5a6f4f3147cc44a22c00ca0f02c8599fd927244f"
last_verified_date: "2026-07-19"
domain_id: "DOM-MEDIA"
domain_name: "Media Library"
domain_status: "IMPLEMENTED"
boundary_status: "OBSERVED"
primary_sources:
  - "backend/src/routes/media.rs"
  - "backend/src/services/media_processing.rs"
  - "backend/migrations/0001_initial_schema.sql"
  - "frontend/src/pages/MediaPage.tsx"
related_documents:
  - "../domain-catalog.md"
  - "../workflows/media-upload-and-processing.md"
  - "../../security/resource-ownership.md"
related_diagrams:
  - "../diagrams/cross-module-orchestration.mmd"
---

# Media Library Domain

## Domain Identity

- Domain ID: `DOM-MEDIA`
- Terminology: media, source file, variant, MIME type, uploader, upload quota.
- Implementation: `IMPLEMENTED`; boundary `OBSERVED`; confidence High.

## Responsibility

- Verified: validate/upload files, persist metadata, generate image variants, edit alt/caption, list/detail, and delete DB/files.
- Inferred: local upload directory is current media object storage.
- Shared: quotas with Billing, static delivery with application bootstrap, and media references with dynamic content/Page Builder.
- Unclear: orphan cleanup, virus/malware scanning, retention, external object storage, and reference-aware deletion.

## Core Entities

`media`, `media_variants`, tenant filesystem paths, and uploader attribution.

## Core Services

Media route, media processing, quota, RLS/RBAC, audit, Tokio filesystem, and image library.

## API Surface

Tenant media list/upload/detail/update/delete and static `/uploads` delivery. See [Media Endpoints](../../api/endpoints/media-library.md).

## Frontend Surface

`MediaPage` upload, list, metadata edit, and delete UI.

## Actors

Organization media readers/writers and admin/editor media managers.

## Business Rules

`BR-MEDIA-001` through `BR-MEDIA-004`.

## Invariants

Nonnegative source size, positive optional dimensions, unique variant names, tenant-scoped DB paths, and media-parent cascade.

## State and Lifecycle

Media has no status enum. Upload creates source and optional variants; update changes descriptive metadata; delete is hard. No archive/restore/retention state exists.

## Access Rules

Organization role authorizes operations; `uploader_id` is attribution rather than a current update-own ACL. Tenant path and RLS protect rows.

## Validation Rules

File field required, configured max size, plan capacity, byte-derived MIME allowlist, declaration match, sanitized display filename, and decoded image validity.

## Workflows

[Media Upload and Processing](../workflows/media-upload-and-processing.md).

## Side Effects

Filesystem directories/files, CPU-bound image conversion, media/variant rows, and deletion audit. No cache invalidation, background retry, or reference cleanup was found.

## Tests

No focused media route/processing workflow tests were located. Database checks exist but are not exercised by a dedicated integration suite.

## Risks and Unknowns

DB/files are non-atomic, delete ignores filesystem errors, partial variant failures can leave artifacts, media references are not protected from deletion, and production storage topology is unknown.

Return to the [Domain Catalog](../domain-catalog.md).

