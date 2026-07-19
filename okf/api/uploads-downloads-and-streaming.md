---
okf_document_id: "api-uploads-downloads-streaming"
title: "API Uploads, Downloads, and Streaming"
project: "ZinharCMS"
category: "api"
phase: 6
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "eed1e0dbdf6d873457d1165158b3c8fbfd6647e1"
last_verified_date: "2026-07-18"
primary_sources:
  - "backend/src/routes/mod.rs"
  - "backend/src/routes/media.rs"
  - "backend/src/routes/marketplace.rs"
  - "backend/src/routes/pages.rs"
  - "backend/src/config.rs"
related_documents:
  - "api/request-contracts.md"
  - "api/groups/media.md"
  - "api/endpoints/pages-workflow-versions-and-preview.md"
uncertainty_markers:
  - "REQUEST_CONTRACT_UNCLEAR RCU-02"
  - "RESPONSE_CONTRACT_UNCLEAR RSCU-02"
---

# API Uploads, Downloads, and Streaming

## Media Upload

`POST /api/media/upload` is tenant-protected and accepts multipart data. The required `file` part is read into memory; optional metadata parts include `alt_text` and `caption`. The handler validates the detected content type, enforces media quota, writes the original file, creates database records, and may create variants.

The configured default `MAX_UPLOAD_SIZE` is 52,428,800 bytes. The tenant router permits `MAX_UPLOAD_SIZE + 1,048,576` bytes to allow multipart overhead, while the media handler rejects a file whose bytes exceed `MAX_UPLOAD_SIZE`.

## Marketplace Package Upload

`POST /api/marketplace/listings/{listing_id}/versions/upload` is tenant-protected and accepts multipart parts named `file` and `manifest`. The handler performs creator ownership, listing state, manifest, artifact, checksum, compatibility, and review-pipeline work. Its route is absent from the current OpenAPI path list.

Multipart rejection details and exact overhead behavior are framework-dependent (`REQUEST_CONTRACT_UNCLEAR RCU-02`). Review handler validation and Marketplace services before changing package limits.

## Public Downloads

The backend mounts the configured upload directory at `/uploads` with `ServeDir`. This surface is public and is not represented by a typed route handler or OpenAPI operation. File content type, `HEAD`, conditional requests, ranges, and directory/error behavior are delegated to Tower HTTP and were not demonstrated by repository contract tests (`RESPONSE_CONTRACT_UNCLEAR RSCU-02`).

There is no signed URL, per-file authorization, application-level download handler, or export endpoint in this snapshot.

## WebSocket Preview

`GET /api/preview/{page_id}` upgrades to WebSocket. Because browsers cannot add the standard authentication and tenant headers to the WebSocket constructor, preview may use `access_token` or `token` and `organization_id` query parameters. The server broadcasts text messages containing serialized page JSON on a per-page channel.

Connection establishment, upgrade failures, close codes, reconnection, heartbeat, ordering, and message-version guarantees are not defined as a public protocol. This is an internal editor-preview channel.

## Other Representations

Sitemap XML and robots text are normal finite HTTP responses, not streams. The inbound Stripe webhook and outbound CMS webhooks transfer raw bytes/JSON but do not expose client-controlled streaming. No SSE, resumable/chunked upload protocol, long-polling, or background job progress endpoint was found.
