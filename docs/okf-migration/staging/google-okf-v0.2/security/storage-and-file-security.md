---
type: Security Control
title: Storage and File Security
description: Upload validation, storage-key safety, media lifecycle, artifact quarantine, and cleanup controls observed in the repository.
status: draft
sources:
  - id: source-file-security
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/src/services/file_security.rs
    title: backend/src/services/file_security.rs at Phase 5 source HEAD
  - id: source-media-routes
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/src/routes/media.rs
    title: backend/src/routes/media.rs at Phase 5 source HEAD
  - id: source-file-cleanup
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/src/services/file_cleanup.rs
    title: backend/src/services/file_cleanup.rs at Phase 5 source HEAD
  - id: source-media-processing
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/src/services/media_processing.rs
    title: backend/src/services/media_processing.rs at Phase 5 source HEAD
  - id: source-file-migration
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/migrations/0030_security_phase_seven_file_storage.sql
    title: backend/migrations/0030_security_phase_seven_file_storage.sql at Phase 5 source HEAD
  - id: source-file-tests
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/tests/security_phase7_files.rs
    title: backend/tests/security_phase7_files.rs at Phase 5 source HEAD
  - id: source-file-migration-tests
    resource: https://github.com/sasanzare/ZinharCMS/blob/6f51612c5716c7d9c3365553811053fd24a03947/backend/tests/security_phase7_migration.rs
    title: backend/tests/security_phase7_migration.rs at Phase 5 source HEAD
---

# Upload and storage policy

The source defines separate upload purposes. Public images allow JPEG, PNG,
and WebP with a 10 MiB limit and are processed into verified public media.
Private documents allow PDF and text with a 25 MiB limit and remain restricted
to authenticated organization delivery. Marketplace packages accept ZIP input
with a 50 MiB limit and use a quarantine namespace rather than public media
storage.

The upload path detects content from bytes, checks it against the declared
content type and purpose policy, validates PDF structure when applicable,
normalizes client filenames, rejects traversal and unsafe storage-key
components, and rejects existing symbolic-link or Windows reparse-point
components before file operations. Stored and source SHA-256 values, source
size, verification status, malware-scan status, lifecycle status, and security
metadata are persisted for media records.

Public media delivery selects only active, verified public rows and image
variants. Restricted downloads require the tenant route and organization
predicate. Media rows move through publishing, active, failed, and deletion
pending states; database transactions and cleanup jobs reconcile filesystem
publication or rollback. Cleanup jobs are organization-scoped, de-duplicate
active keys, and use pending/retry/failed/complete states.

The current malware scanner is represented by an explicit no-op scanner and
can produce an unavailable status. This is a control boundary and audit
signal, not proof of an external malware service or a clean verdict for every
artifact. Migration `0030` classifies unresolved legacy media as restricted
and legacy-unverified and adds database constraints for safe keys, checksums,
visibility, lifecycle, and verification combinations.

Marketplace package validation adds manifest, ZIP-structure, checksum,
permission, and artifact-state checks. The runtime execution boundary is
described in [Marketplace runtime and safety boundary](/domain/marketplace-runtime-and-safety-boundary.md), while the tenant database policy is in [tenant data policy](/database/tenant-data-policy.md).

## Open decision dependencies

* NOC-02: the repository does not establish production object storage, CDN,
  shared filesystem durability, deployment configuration, or final asset
  authorization policy.
* NOC-05: retention, residency, deletion, legal hold, and artifact-cleanup
  authority are not inferred from lifecycle columns or cleanup code.

