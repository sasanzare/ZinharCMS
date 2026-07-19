---
okf_document_id: "api-endpoints-media-library"
title: "Media Library Endpoints"
project: "ZinharCMS"
category: "api-endpoint-family"
phase: 6
status: "current"
review_status: "verified"
source_of_truth: false
implementation_view: "observed"
last_verified_commit: "eed1e0dbdf6d873457d1165158b3c8fbfd6647e1"
last_verified_date: "2026-07-18"
primary_sources:
  - "backend/src/routes"
related_documents:
  - "api/endpoint-catalog.md"
  - "api/groups/media.md"
  - "backend/modules/media.md"
  - "database/entities/media-and-variants.md"
uncertainty_markers:
  - "REQUEST_CONTRACT_UNCLEAR RCU-02"
  - "RESPONSE_CONTRACT_UNCLEAR RSCU-02"
---

# Media Library Endpoints

[Back to the Endpoint Catalog](../endpoint-catalog.md) · [Owning route group](../groups/media.md)

## Family Boundary

This family contains 5 registered handler-method endpoints. Access is **Tenant protected with media writer/manager rules**.

Paged media list, multipart upload, detail, metadata update, and delete.

Exact method/path, stable endpoint ID, handler, access zone, input extractor, return type, OpenAPI status, and frontend coverage are recorded in the [Endpoint Catalog](../endpoint-catalog.md).

## Request Contract

List accepts page/per-page/mime-type. Upload uses multipart `file` plus metadata. Update uses JSON; detail/update/delete use media UUID.

## Response Contract

Media objects, detail objects with variants, and paged list wrapper.

## Ownership and Persistence

- Backend owner: [Media](../../backend/modules/media.md)
- Persistence: [relevant entity documentation](../../database/entities/media-and-variants.md)
- Route group: [Media](../groups/media.md)

## Frontend Contract

Four operations have wrappers; media detail GET does not.

## OpenAPI and Verification

All handlers are listed, but multipart and upload-limit detail is incomplete.

Upload validation/services have selected tests; multipart-to-static-download behavior lacks an end-to-end suite.

## Change Checklist

Review route registration, middleware zone, DTO and Serde behavior, role and ownership checks, tenant/RLS use, success and error statuses, frontend method/type/callers, OpenAPI, persistence and side effects, and representative positive and negative tests.
