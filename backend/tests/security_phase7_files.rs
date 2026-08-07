use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;

use cms_backend::services::file_security::{
    ArchiveLimits, FileKind, MalwareScanOutcome, NoopMalwareScanner, SecureTempUpload,
    UploadPurpose, cleanup_stale_processing_directories, cleanup_stale_temporary_files,
    content_disposition, detect_file_kind, extract_archive_to_quarantine, inspect_archive,
    media_storage_key, normalize_client_filename, policy_for, secure_join, secure_join_no_symlinks,
    validate_pdf_structure,
};
use cms_backend::services::marketplace_package::sha256_hex;
use cms_backend::services::media_processing::{process_image_upload, remove_processing_directory};
use crc32fast::Hasher;
use flate2::Compression;
use flate2::write::DeflateEncoder;
use uuid::Uuid;

#[test]
fn central_policy_separates_public_images_private_documents_and_packages() {
    let image = policy_for(UploadPurpose::PublicImage);
    let document = policy_for(UploadPurpose::PrivateDocument);
    let package = policy_for(UploadPurpose::MarketplacePackage);

    assert!(image.allowed_kinds.contains(&FileKind::Png));
    assert!(!image.allowed_kinds.contains(&FileKind::Pdf));
    assert!(document.allowed_kinds.contains(&FileKind::Pdf));
    assert!(!document.allowed_kinds.contains(&FileKind::Svg));
    assert_eq!(package.allowed_kinds, &[FileKind::Zip]);
    assert!(image.max_bytes < package.max_bytes);
}

#[test]
fn content_detection_uses_bytes_and_rejects_active_markup() {
    assert_eq!(
        detect_file_kind(b"\x89PNG\r\n\x1a\nrest"),
        Some(FileKind::Png)
    );
    assert_eq!(detect_file_kind(b"%PDF-1.7\n"), Some(FileKind::Pdf));
    assert_eq!(detect_file_kind(b"PK\x03\x04rest"), Some(FileKind::Zip));
    assert_eq!(
        detect_file_kind(b"<svg onload='alert(1)'></svg>"),
        Some(FileKind::Svg)
    );
    assert_eq!(
        detect_file_kind(b"<!doctype html><script>alert(1)</script>"),
        Some(FileKind::Html)
    );
}

#[test]
fn pdf_validation_requires_a_supported_header_and_terminal_eof_marker() {
    assert!(validate_pdf_structure(b"%PDF-1.7\n1 0 obj", b"trailer\n%%EOF\r\n").is_ok());
    assert!(validate_pdf_structure(b"%PDF-9.9\n", b"%%EOF\n").is_err());
    assert!(validate_pdf_structure(b"%PDF-1.7\n", b"missing marker").is_err());
    assert!(validate_pdf_structure(b"%PDF-1.7\n", b"%%EOF<script>").is_err());
}

#[test]
fn filenames_and_disposition_are_header_and_path_safe() {
    assert_eq!(
        normalize_client_filename("../Q3 report\r\n\".pdf"),
        "Q3-report.pdf"
    );
    let disposition = content_disposition("attachment", "Résumé \"Q3\".pdf").unwrap();
    assert!(disposition.starts_with("attachment; filename=\""));
    assert!(disposition.contains("filename*=UTF-8''"));
    assert!(!disposition.contains('\r'));
    assert!(!disposition.contains('\n'));
    assert_eq!(normalize_client_filename("NUL.txt"), "upload-NUL.txt");
    assert_eq!(normalize_client_filename("résumé.pdf"), "résumé.pdf");
}

#[test]
fn server_generated_storage_keys_are_tenant_scoped_and_contained() {
    let organization_id = Uuid::now_v7();
    let media_id = Uuid::now_v7();
    let key = media_storage_key(
        UploadPurpose::PrivateDocument,
        organization_id,
        media_id,
        "pdf",
    )
    .expect("storage key should be valid");

    assert_eq!(
        key,
        format!("private/media/{organization_id}/{media_id}/original.pdf")
    );
    assert!(secure_join(Path::new("storage"), &key).is_ok());
    assert!(secure_join(Path::new("storage"), "../outside").is_err());
    assert!(secure_join(Path::new("storage"), r"private\..\outside").is_err());
    assert!(secure_join(Path::new("storage"), "C:/outside").is_err());
    assert!(secure_join(Path::new("storage"), "%2e%2e/outside").is_err());
}

#[test]
fn scanner_unavailability_is_explicit_and_not_a_clean_verdict() {
    let scanner = NoopMalwareScanner;
    assert_eq!(
        scanner.scan_verdict(&sha256_hex(b"sample")),
        MalwareScanOutcome::Unavailable
    );
}

#[test]
fn archive_defaults_bound_entries_size_ratio_and_nesting() {
    let limits = ArchiveLimits::marketplace();
    assert!(limits.max_entries > 0);
    assert!(limits.max_entries <= 500);
    assert!(limits.max_total_uncompressed_bytes > 0);
    assert!(limits.max_total_uncompressed_bytes <= 100 * 1024 * 1024);
    assert!(limits.max_compression_ratio <= 100);
    assert_eq!(limits.max_nested_archives, 0);
    assert_eq!(limits.max_path_depth, 16);
    assert!(limits.max_processing_millis <= 10_000);
}

#[derive(Clone)]
struct ZipSpec<'a> {
    path: &'a str,
    data: &'a [u8],
    deflate: bool,
    external_attributes: u32,
    declared_uncompressed: Option<u32>,
    crc_override: Option<u32>,
}

fn regular_file<'a>(path: &'a str, data: &'a [u8]) -> ZipSpec<'a> {
    ZipSpec {
        path,
        data,
        deflate: false,
        external_attributes: 0o100_644 << 16,
        declared_uncompressed: None,
        crc_override: None,
    }
}

fn build_zip(specs: &[ZipSpec<'_>]) -> Vec<u8> {
    let mut archive = Vec::new();
    let mut central = Vec::new();
    for spec in specs {
        let local_offset = archive.len() as u32;
        let compressed = if spec.deflate {
            let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(spec.data).expect("deflate input");
            encoder.finish().expect("deflate finish")
        } else {
            spec.data.to_vec()
        };
        let mut crc = Hasher::new();
        crc.update(spec.data);
        let crc = spec.crc_override.unwrap_or_else(|| crc.finalize());
        let uncompressed = spec.declared_uncompressed.unwrap_or(spec.data.len() as u32);
        let method = if spec.deflate { 8u16 } else { 0u16 };
        let name = spec.path.as_bytes();

        archive.extend_from_slice(&0x0403_4b50u32.to_le_bytes());
        archive.extend_from_slice(&20u16.to_le_bytes());
        archive.extend_from_slice(&0x0800u16.to_le_bytes());
        archive.extend_from_slice(&method.to_le_bytes());
        archive.extend_from_slice(&0u16.to_le_bytes());
        archive.extend_from_slice(&0u16.to_le_bytes());
        archive.extend_from_slice(&crc.to_le_bytes());
        archive.extend_from_slice(&(compressed.len() as u32).to_le_bytes());
        archive.extend_from_slice(&uncompressed.to_le_bytes());
        archive.extend_from_slice(&(name.len() as u16).to_le_bytes());
        archive.extend_from_slice(&0u16.to_le_bytes());
        archive.extend_from_slice(name);
        archive.extend_from_slice(&compressed);

        central.extend_from_slice(&0x0201_4b50u32.to_le_bytes());
        central.extend_from_slice(&0x0314u16.to_le_bytes());
        central.extend_from_slice(&20u16.to_le_bytes());
        central.extend_from_slice(&0x0800u16.to_le_bytes());
        central.extend_from_slice(&method.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&crc.to_le_bytes());
        central.extend_from_slice(&(compressed.len() as u32).to_le_bytes());
        central.extend_from_slice(&uncompressed.to_le_bytes());
        central.extend_from_slice(&(name.len() as u16).to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&spec.external_attributes.to_le_bytes());
        central.extend_from_slice(&local_offset.to_le_bytes());
        central.extend_from_slice(name);
    }
    let central_offset = archive.len() as u32;
    let central_size = central.len() as u32;
    archive.extend_from_slice(&central);
    archive.extend_from_slice(&0x0605_4b50u32.to_le_bytes());
    archive.extend_from_slice(&0u16.to_le_bytes());
    archive.extend_from_slice(&0u16.to_le_bytes());
    archive.extend_from_slice(&(specs.len() as u16).to_le_bytes());
    archive.extend_from_slice(&(specs.len() as u16).to_le_bytes());
    archive.extend_from_slice(&central_size.to_le_bytes());
    archive.extend_from_slice(&central_offset.to_le_bytes());
    archive.extend_from_slice(&0u16.to_le_bytes());
    archive
}

fn temp_case(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("zinhar-phase7-{name}-{}", Uuid::now_v7()))
}

fn write_archive(case: &str, bytes: &[u8]) -> (PathBuf, PathBuf) {
    let root = temp_case(case);
    std::fs::create_dir(&root).expect("create test root");
    let archive = root.join("package.zip");
    std::fs::write(&archive, bytes).expect("write archive");
    (root, archive)
}

#[test]
fn safe_archive_is_inspected_and_extracted_with_crc_verification() {
    let specs = [
        regular_file("manifest.json", br#"{"name":"safe"}"#),
        ZipSpec {
            path: "assets/readme.txt",
            data: b"compressed package content",
            deflate: true,
            ..regular_file("", b"")
        },
    ];
    let (root, archive) = write_archive("safe", &build_zip(&specs));
    let report = inspect_archive(&archive, ArchiveLimits::marketplace()).expect("inspect safe ZIP");
    assert_eq!(report.entries.len(), 2);
    let extraction = root.join("quarantine");
    extract_archive_to_quarantine(&archive, &extraction, ArchiveLimits::marketplace())
        .expect("extract safe ZIP");
    assert_eq!(
        std::fs::read(extraction.join("assets/readme.txt")).expect("read extracted"),
        b"compressed package content"
    );
    let mut invalid_time_limit = ArchiveLimits::marketplace();
    invalid_time_limit.max_processing_millis = 0;
    assert!(inspect_archive(&archive, invalid_time_limit).is_err());
    std::fs::remove_dir_all(root).expect("clean safe case");
}

#[test]
fn archive_rejects_unix_windows_encoded_and_absolute_traversal_paths() {
    for (index, path) in [
        "../escape.txt",
        "safe/../../escape.txt",
        r"safe\..\escape.txt",
        "%2e%2e/escape.txt",
        "/absolute.txt",
        "C:/windows.txt",
    ]
    .iter()
    .enumerate()
    {
        let (root, archive) = write_archive(
            &format!("traversal-{index}"),
            &build_zip(&[regular_file(path, b"x")]),
        );
        assert!(
            inspect_archive(&archive, ArchiveLimits::marketplace()).is_err(),
            "unsafe path was accepted: {path}"
        );
        std::fs::remove_dir_all(root).expect("clean traversal case");
    }
}

#[test]
fn archive_rejects_case_collisions_symlinks_nested_archives_and_bomb_ratios() {
    let cases = [
        build_zip(&[
            regular_file("Asset.txt", b"a"),
            regular_file("asset.txt", b"b"),
        ]),
        build_zip(&[ZipSpec {
            path: "link",
            data: b"target",
            external_attributes: 0o120_777 << 16,
            ..regular_file("", b"")
        }]),
        build_zip(&[regular_file(
            "a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/file.txt",
            b"deep",
        )]),
        build_zip(&[regular_file("unicode/é.txt", b"collision-safe rejection")]),
        build_zip(&[regular_file("nested/package.zip", b"PK\x03\x04")]),
        build_zip(&[ZipSpec {
            path: "bomb.txt",
            data: b"x",
            declared_uncompressed: Some(10_000),
            ..regular_file("", b"")
        }]),
    ];
    for (index, bytes) in cases.iter().enumerate() {
        let (root, archive) = write_archive(&format!("policy-{index}"), bytes);
        assert!(inspect_archive(&archive, ArchiveLimits::marketplace()).is_err());
        std::fs::remove_dir_all(root).expect("clean policy case");
    }
}

#[test]
fn extraction_crc_failure_removes_the_entire_quarantine_directory() {
    let bytes = build_zip(&[ZipSpec {
        path: "payload.txt",
        data: b"actual",
        crc_override: Some(0x1234_5678),
        ..regular_file("", b"")
    }]);
    let (root, archive) = write_archive("bad-crc", &bytes);
    let extraction = root.join("quarantine");
    assert!(
        extract_archive_to_quarantine(&archive, &extraction, ArchiveLimits::marketplace()).is_err()
    );
    assert!(!extraction.exists());
    std::fs::remove_dir_all(root).expect("clean CRC case");
}

#[tokio::test]
async fn streaming_stage_enforces_limits_and_publishes_without_overwrite() {
    let root = temp_case("streaming");
    let organization_id = Uuid::now_v7();
    let media_id = Uuid::now_v7();
    let key = media_storage_key(
        UploadPurpose::PrivateDocument,
        organization_id,
        media_id,
        "txt",
    )
    .expect("storage key");
    let mut temp = SecureTempUpload::create(&root, 8, "../note.txt", "text/plain")
        .await
        .expect("create stage");
    temp.write_chunk(b"safe").await.expect("write first chunk");
    temp.write_chunk(b"text").await.expect("write second chunk");
    assert!(temp.write_chunk(b"!").await.is_err());
    let staged = temp.finish().await.expect("finish stage");
    assert_eq!(staged.size, 8);
    assert_eq!(staged.original_filename, "note.txt");
    let destination = staged.persist(&root, &key).await.expect("publish stage");
    assert_eq!(
        std::fs::read(&destination).expect("read published"),
        b"safetext"
    );

    let mut duplicate = SecureTempUpload::create(&root, 8, "other.txt", "text/plain")
        .await
        .expect("create duplicate stage");
    duplicate
        .write_chunk(b"other")
        .await
        .expect("write duplicate");
    let duplicate = duplicate.finish().await.expect("finish duplicate");
    assert!(duplicate.persist(&root, &key).await.is_err());
    assert_eq!(
        std::fs::read(&destination).expect("read original"),
        b"safetext"
    );
    std::fs::remove_dir_all(root).expect("clean streaming case");
}

#[tokio::test]
async fn text_validation_streams_the_entire_file_and_rejects_late_invalid_utf8() {
    let root = temp_case("utf8-validation");
    let mut valid = SecureTempUpload::create(&root, 16_384, "valid.txt", "text/plain")
        .await
        .expect("create valid stage");
    valid
        .write_chunk(&vec![b'a'; 8_191])
        .await
        .expect("write text prefix");
    valid
        .write_chunk("€".as_bytes())
        .await
        .expect("write split-safe UTF-8");
    let valid = valid.finish().await.expect("finish valid stage");
    valid
        .validate_utf8_text()
        .await
        .expect("accept valid UTF-8");
    valid.remove().await.expect("remove valid stage");

    let mut invalid = SecureTempUpload::create(&root, 16_384, "invalid.txt", "text/plain")
        .await
        .expect("create invalid stage");
    invalid
        .write_chunk(&vec![b'a'; 8_193])
        .await
        .expect("write valid prefix");
    invalid
        .write_chunk(&[0xff])
        .await
        .expect("write invalid suffix");
    let invalid = invalid.finish().await.expect("finish invalid stage");
    assert!(invalid.validate_utf8_text().await.is_err());
    invalid.remove().await.expect("remove invalid stage");
    std::fs::remove_dir_all(root).expect("clean UTF-8 case");
}

#[tokio::test]
async fn failed_or_cancelled_stage_removes_partial_file_and_stale_cleanup_is_bounded() {
    let root = temp_case("temp-cleanup");
    let staged_path = {
        let mut temp = SecureTempUpload::create(&root, 4, "partial.txt", "text/plain")
            .await
            .expect("create partial stage");
        temp.write_chunk(b"data").await.expect("write partial");
        temp.write_chunk(b"overflow")
            .await
            .expect_err("reject overflow");
        let path = root.join("quarantine/tmp");
        let entry = std::fs::read_dir(&path)
            .expect("read staging directory")
            .next()
            .expect("partial entry")
            .expect("read partial entry")
            .path();
        drop(temp);
        entry
    };
    assert!(!staged_path.exists());

    let temp_dir = root.join("quarantine/tmp");
    let stale = temp_dir.join(format!("{}.part", Uuid::now_v7()));
    let unrelated = temp_dir.join("keep.txt");
    std::fs::write(&stale, b"stale").expect("write stale fixture");
    assert_eq!(
        cleanup_stale_temporary_files(&root, Duration::ZERO, 1)
            .await
            .expect("bounded cleanup"),
        1
    );
    assert!(!stale.exists());
    std::fs::write(&unrelated, b"keep").expect("write unrelated fixture");
    assert_eq!(
        cleanup_stale_temporary_files(&root, Duration::ZERO, 10)
            .await
            .expect("safe cleanup"),
        0
    );
    assert!(unrelated.exists());
    std::fs::remove_dir_all(root).expect("clean temporary case");
}

#[tokio::test]
async fn stale_image_processing_cleanup_is_bounded_and_rejects_unexpected_contents() {
    let root = temp_case("processing-cleanup");
    let processing_root = root.join("quarantine/processed");
    let stale = processing_root.join(Uuid::now_v7().to_string());
    let unsafe_directory = processing_root.join(Uuid::now_v7().to_string());
    std::fs::create_dir_all(&stale).expect("create stale processing directory");
    std::fs::create_dir_all(&unsafe_directory).expect("create unsafe processing directory");
    std::fs::write(stale.join("original.webp"), b"generated").expect("write generated file");
    std::fs::write(unsafe_directory.join("unexpected.txt"), b"keep")
        .expect("write unexpected file");

    assert_eq!(
        cleanup_stale_processing_directories(&root, Duration::ZERO, 10)
            .await
            .expect("clean processing directories"),
        1
    );
    assert!(!stale.exists());
    assert!(unsafe_directory.exists());
    std::fs::remove_dir_all(root).expect("clean processing case");
}

#[test]
fn storage_paths_reject_existing_symbolic_link_components_when_supported() {
    let root = temp_case("symlink");
    let outside = temp_case("symlink-outside");
    std::fs::create_dir(&root).expect("create symlink root");
    std::fs::create_dir(&outside).expect("create symlink target");
    let link = root.join("private");
    if create_directory_symlink(&outside, &link).is_ok() {
        assert!(
            secure_join_no_symlinks(&root, "private/media/file.txt").is_err(),
            "storage path followed a symbolic link"
        );
    }
    std::fs::remove_dir_all(&root).expect("clean symlink root");
    std::fs::remove_dir_all(&outside).expect("clean symlink target");
}

#[tokio::test]
async fn image_processing_reencodes_pixels_to_webp_and_enforces_dimensions() {
    let root = temp_case("image-processing");
    std::fs::create_dir(&root).expect("create image root");
    let source = root.join("source.png");
    image::DynamicImage::new_rgba8(16, 12)
        .save_with_format(&source, image::ImageFormat::Png)
        .expect("write PNG fixture");
    let organization_id = Uuid::now_v7();
    let media_id = Uuid::now_v7();
    let processed = process_image_upload(&source, &root, organization_id, media_id)
        .await
        .expect("process safe image");
    assert_eq!(processed.original.width, 16);
    assert_eq!(processed.original.height, 12);
    assert!(processed.original.storage_key.ends_with("/original.webp"));
    assert_eq!(processed.variants.len(), 4);
    for file in std::iter::once(&processed.original).chain(processed.variants.iter()) {
        assert!(file.path.starts_with(root.join("quarantine/processed")));
        assert_eq!(
            image::ImageReader::open(&file.path)
                .expect("open generated image")
                .with_guessed_format()
                .expect("guess generated format")
                .format(),
            Some(image::ImageFormat::WebP)
        );
        assert_eq!(file.sha256.len(), 64);
    }
    remove_processing_directory(&processed.original.path).await;

    let oversized = root.join("oversized.png");
    image::DynamicImage::new_rgba8(8_193, 1)
        .save_with_format(&oversized, image::ImageFormat::Png)
        .expect("write oversized fixture");
    let rejected_id = Uuid::now_v7();
    assert!(
        process_image_upload(&oversized, &root, organization_id, rejected_id)
            .await
            .is_err()
    );
    assert!(
        !root
            .join(format!("quarantine/processed/{rejected_id}"))
            .exists()
    );
    std::fs::remove_dir_all(root).expect("clean image case");
}

#[cfg(unix)]
fn create_directory_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn create_directory_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_dir(target, link)
}

#[cfg(unix)]
#[tokio::test]
async fn staging_files_and_directories_use_restrictive_unix_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let root = temp_case("permissions");
    let mut temp = SecureTempUpload::create(&root, 8, "file.txt", "text/plain")
        .await
        .expect("create stage");
    temp.write_chunk(b"safe").await.expect("write stage");
    let staged = temp.finish().await.expect("finish stage");
    assert_eq!(
        std::fs::metadata(staged.path())
            .expect("stage metadata")
            .permissions()
            .mode()
            & 0o777,
        0o600
    );
    assert_eq!(
        std::fs::metadata(root.join("quarantine/tmp"))
            .expect("directory metadata")
            .permissions()
            .mode()
            & 0o777,
        0o700
    );
    staged.remove().await.expect("remove stage");
    std::fs::remove_dir_all(root).expect("clean permissions case");
}
