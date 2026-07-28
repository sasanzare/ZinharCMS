---
okf_document_id: "security-input-validation"
title: "Input Validation and Sanitization"
project: "ZinharCMS"
category: "security-input"
phase: 7
status: "current"
source_of_truth: false
implementation_view: "observed"
security_status: "partially_verified"
last_verified_commit: "8b8c091bdcbba340287d7d31dbae31544ff21d59"
last_verified_date: "2026-07-19"
primary_sources:
  - "backend/src/routes/auth.rs"
  - "backend/src/services/entry_validation.rs"
  - "backend/src/services/security.rs"
  - "backend/src/services/media_processing.rs"
  - "backend/src/services/marketplace_manifest.rs"
  - "backend/src/services/marketplace_package.rs"
  - "backend/src/services/webhooks.rs"
related_documents:
  - "browser-and-http-security.md"
  - "threat-register.md"
  - "../frontend/forms-and-validation.md"
related_diagrams:
  - "diagrams/trust-boundaries.mmd"
---

# Input Validation and Sanitization

## Validation Layers

| Layer | Observed behavior |
| --- | --- |
| Axum/Serde extractors | Parse path, query, JSON, multipart, headers, and extensions into typed values |
| Handler checks | Validate required strings, supported statuses, identifiers, confirmation flags, and lifecycle preconditions |
| Domain services | Validate entry schemas/workflows, packages/manifests, runtime operations, webhooks, billing, and quotas |
| Database | Types, enums, foreign keys, uniqueness, checks, triggers, and RLS |
| Frontend | HTML constraints and local preconditions for feedback only |

## Authentication Input

Registration checks only basic email shape, minimum password length, and name presence. Login normalizes email. No maximum field lengths or centralized credential schema were found.

## Rich Text

Entry and Page Builder rich text uses Ammonia's HTML5 parser and an explicit
allowlist. The boundary applies before storage and again to historical
authenticated/public reads, versions, Marketplace template imports, and Preview
WebSocket payloads. The frontend applies the aligned DOMPurify policy before the
single approved HTML sink. IDs, names, classes, styles, `data-*`, event
attributes, forms, frames, SVG, MathML, objects, embeds, and unknown elements
are denied. Rich-text input is limited to 128 KiB, URLs to 2,048 bytes, parser
complexity to 4,096 tags/attributes and 128 nesting levels, and page documents
to 1 MiB.

Markdown is not accepted by a runtime renderer. Raw HTML inside Markdown is
therefore unavailable rather than conditionally trusted.

## Uploads and Packages

Tenant routes apply a configured request-body ceiling. Media logic performs content-signature/MIME checks and processing rules. Marketplace package validation checks manifest shape, permission allowlists, archive paths/structure, compatibility, and security findings. Static upload serving remains a separate public boundary.

## URLs and External Effects

Rich links allow only HTTPS, mail, telephone, constrained relative, and fragment
destinations. Rich-text images are same-origin. External navigation requires
canonical HTTPS. Control characters, credentials, protocol-relative forms,
backslashes, encoded unsafe schemes, and dangerous schemes are rejected.
Webhook validation and pinned outbound delivery remain separate SSRF controls.
Marketplace runtime external-network operations are bounded by declared
permissions and host operations rather than executing arbitrary package code.

## SQL and Output Handling

Inspected database access uses SQLx parameter binding for values. Dynamic SQL exists for controlled schema/policy generation and selected query assembly; this review did not prove every dynamic fragment safe. React escapes normal text interpolation, but rich HTML/rendering and file delivery require context-specific review.

## Uncertainty

`INPUT_VALIDATION_UNCLEAR IVU-01` remains for non-rich-content validation:
maximum lengths and schemas are still distributed and no repository-wide fuzz
suite exists. The Phase 4 report now supplies the content source-to-sink
inventory and canonical rich-content contract.
