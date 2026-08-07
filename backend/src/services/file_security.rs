use std::collections::HashSet;
use std::fmt;
use std::fs::File as StdFile;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Component, Path, PathBuf};
use std::time::{Duration, Instant, SystemTime};

use crc32fast::Hasher as Crc32Hasher;
use flate2::read::DeflateDecoder;
use sha2::{Digest, Sha256};
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use uuid::Uuid;

pub const MIB: u64 = 1_048_576;
pub const MAX_IMAGE_WIDTH: u32 = 8_192;
pub const MAX_IMAGE_HEIGHT: u32 = 8_192;
pub const MAX_IMAGE_PIXELS: u64 = 40_000_000;
pub const MAX_IMAGE_DECODE_ALLOC: u64 = 192 * MIB;
pub const MAX_METADATA_BYTES: usize = 8_192;

const PUBLIC_IMAGE_KINDS: &[FileKind] = &[FileKind::Jpeg, FileKind::Png, FileKind::Webp];
const PRIVATE_DOCUMENT_KINDS: &[FileKind] = &[FileKind::Pdf, FileKind::Text];
const MARKETPLACE_PACKAGE_KINDS: &[FileKind] = &[FileKind::Zip];

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum UploadPurpose {
    PublicImage,
    PrivateDocument,
    MarketplacePackage,
}

impl UploadPurpose {
    fn namespace(self) -> &'static str {
        match self {
            Self::PublicImage => "public",
            Self::PrivateDocument => "private",
            Self::MarketplacePackage => "quarantine",
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum FileKind {
    Jpeg,
    Png,
    Webp,
    Pdf,
    Text,
    Zip,
    Svg,
    Html,
}

impl FileKind {
    pub fn mime_type(self) -> &'static str {
        match self {
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::Webp => "image/webp",
            Self::Pdf => "application/pdf",
            Self::Text => "text/plain",
            Self::Zip => "application/zip",
            Self::Svg => "image/svg+xml",
            Self::Html => "text/html",
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            Self::Jpeg => "jpg",
            Self::Png => "png",
            Self::Webp => "webp",
            Self::Pdf => "pdf",
            Self::Text => "txt",
            Self::Zip => "zip",
            Self::Svg => "svg",
            Self::Html => "html",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UploadPolicy {
    pub purpose: UploadPurpose,
    pub max_bytes: u64,
    pub allowed_kinds: &'static [FileKind],
    pub public_inline: bool,
}

const PUBLIC_IMAGE_POLICY: UploadPolicy = UploadPolicy {
    purpose: UploadPurpose::PublicImage,
    max_bytes: 10 * MIB,
    allowed_kinds: PUBLIC_IMAGE_KINDS,
    public_inline: true,
};

const PRIVATE_DOCUMENT_POLICY: UploadPolicy = UploadPolicy {
    purpose: UploadPurpose::PrivateDocument,
    max_bytes: 25 * MIB,
    allowed_kinds: PRIVATE_DOCUMENT_KINDS,
    public_inline: false,
};

const MARKETPLACE_PACKAGE_POLICY: UploadPolicy = UploadPolicy {
    purpose: UploadPurpose::MarketplacePackage,
    max_bytes: 50 * MIB,
    allowed_kinds: MARKETPLACE_PACKAGE_KINDS,
    public_inline: false,
};

pub fn policy_for(purpose: UploadPurpose) -> &'static UploadPolicy {
    match purpose {
        UploadPurpose::PublicImage => &PUBLIC_IMAGE_POLICY,
        UploadPurpose::PrivateDocument => &PRIVATE_DOCUMENT_POLICY,
        UploadPurpose::MarketplacePackage => &MARKETPLACE_PACKAGE_POLICY,
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FileSecurityError(String);

impl FileSecurityError {
    pub fn message(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FileSecurityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for FileSecurityError {}

fn error(message: impl Into<String>) -> FileSecurityError {
    FileSecurityError(message.into())
}

pub fn detect_file_kind(bytes: &[u8]) -> Option<FileKind> {
    if bytes.starts_with(&[0xff, 0xd8, 0xff]) {
        return Some(FileKind::Jpeg);
    }
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        return Some(FileKind::Png);
    }
    if bytes.len() >= 12 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WEBP" {
        return Some(FileKind::Webp);
    }
    if bytes.starts_with(b"%PDF-") {
        return Some(FileKind::Pdf);
    }
    if bytes.starts_with(b"PK\x03\x04")
        || bytes.starts_with(b"PK\x05\x06")
        || bytes.starts_with(b"PK\x07\x08")
    {
        return Some(FileKind::Zip);
    }

    if bytes.is_empty() || bytes.contains(&0) {
        return None;
    }
    let text = std::str::from_utf8(bytes).ok()?;
    let normalized = text
        .trim_start_matches('\u{feff}')
        .trim_start()
        .to_ascii_lowercase();
    if normalized.starts_with("<svg")
        || normalized.starts_with("<?xml") && normalized.contains("<svg")
    {
        return Some(FileKind::Svg);
    }
    if normalized.starts_with("<!doctype html")
        || normalized.starts_with("<html")
        || normalized.starts_with("<script")
    {
        return Some(FileKind::Html);
    }
    Some(FileKind::Text)
}

pub fn validate_detected_kind(
    purpose: UploadPurpose,
    declared_content_type: &str,
    prefix: &[u8],
) -> Result<FileKind, FileSecurityError> {
    let kind = detect_file_kind(prefix)
        .ok_or_else(|| error("file type could not be detected from content"))?;
    let policy = policy_for(purpose);
    if !policy.allowed_kinds.contains(&kind) {
        return Err(error(format!(
            "detected file type '{}' is not allowed for this upload",
            kind.mime_type()
        )));
    }

    let declared = declared_content_type
        .split(';')
        .next()
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase();
    if !declared.is_empty()
        && declared != "application/octet-stream"
        && !(purpose == UploadPurpose::MarketplacePackage
            && matches!(declared.as_str(), "application/x-zip-compressed"))
        && declared != kind.mime_type()
    {
        return Err(error(format!(
            "declared content type does not match detected type '{}'",
            kind.mime_type()
        )));
    }
    Ok(kind)
}

pub fn validate_pdf_structure(prefix: &[u8], suffix: &[u8]) -> Result<(), FileSecurityError> {
    let version_is_supported = prefix.len() >= 9
        && prefix.starts_with(b"%PDF-")
        && ((prefix[5] == b'1' && prefix[6] == b'.' && (b'0'..=b'7').contains(&prefix[7]))
            || &prefix[5..8] == b"2.0")
        && matches!(prefix[8], b'\r' | b'\n');
    if !version_is_supported {
        return Err(error("PDF header or version is malformed"));
    }

    let Some(eof_offset) = suffix.windows(5).rposition(|window| window == b"%%EOF") else {
        return Err(error("PDF end marker is missing"));
    };
    if !suffix[eof_offset + 5..]
        .iter()
        .all(|byte| byte.is_ascii_whitespace())
    {
        return Err(error("PDF contains data after its end marker"));
    }
    Ok(())
}

pub fn normalize_client_filename(value: &str) -> String {
    let leaf = value.rsplit(['/', '\\']).next().unwrap_or_default().trim();
    let mut output = String::with_capacity(leaf.len().min(180));
    let mut previous_separator = false;

    for character in leaf.chars() {
        if output.len() >= 180 {
            break;
        }
        let safe = character.is_alphanumeric() || matches!(character, '.' | '-' | '_');
        if safe {
            output.push(character);
            previous_separator = false;
        } else if (character.is_whitespace() || !character.is_control()) && !previous_separator {
            output.push('-');
            previous_separator = true;
        }
    }

    let normalized = output
        .trim_matches(|character| matches!(character, '.' | '-' | '_'))
        .replace("..", ".")
        .replace("-.", ".");
    if normalized.is_empty() {
        "upload.bin".to_owned()
    } else if is_reserved_windows_filename(&normalized) {
        format!("upload-{normalized}")
    } else {
        normalized
    }
}

fn is_reserved_windows_filename(filename: &str) -> bool {
    let stem = filename
        .split('.')
        .next()
        .unwrap_or_default()
        .to_ascii_uppercase();
    matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || stem.strip_prefix("COM").is_some_and(|suffix| {
            matches!(suffix, "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
        })
        || stem.strip_prefix("LPT").is_some_and(|suffix| {
            matches!(suffix, "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
        })
}

pub fn content_disposition(disposition: &str, filename: &str) -> Result<String, FileSecurityError> {
    if !matches!(disposition, "inline" | "attachment") {
        return Err(error("unsupported content disposition"));
    }
    let normalized = normalize_client_filename(filename);
    let fallback: String = normalized
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '_') {
                character
            } else {
                '-'
            }
        })
        .collect();
    let fallback = if fallback.is_empty() {
        "download.bin".to_owned()
    } else {
        fallback
    };
    let encoded = percent_encode_header_value(filename);
    Ok(format!(
        "{disposition}; filename=\"{fallback}\"; filename*=UTF-8''{encoded}"
    ))
}

fn percent_encode_header_value(value: &str) -> String {
    let mut output = String::new();
    for byte in value.as_bytes() {
        if byte.is_ascii_alphanumeric() || matches!(*byte, b'.' | b'-' | b'_') {
            output.push(char::from(*byte));
        } else {
            output.push_str(&format!("%{byte:02X}"));
        }
    }
    output
}

pub fn media_storage_key(
    purpose: UploadPurpose,
    organization_id: Uuid,
    media_id: Uuid,
    extension: &str,
) -> Result<String, FileSecurityError> {
    if purpose == UploadPurpose::MarketplacePackage {
        return Err(error("Marketplace package keys use the package key policy"));
    }
    let extension = extension.trim().to_ascii_lowercase();
    if extension.is_empty()
        || extension.len() > 8
        || !extension
            .chars()
            .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit())
    {
        return Err(error("storage extension is invalid"));
    }
    Ok(format!(
        "{}/media/{organization_id}/{media_id}/original.{extension}",
        purpose.namespace()
    ))
}

pub fn secure_join(root: &Path, key: &str) -> Result<PathBuf, FileSecurityError> {
    if key.is_empty()
        || key.contains(['\\', ':', '\0', '%'])
        || Path::new(key).is_absolute()
        || key.starts_with('/')
    {
        return Err(error("storage key is not a safe relative path"));
    }

    let relative = Path::new(key);
    if relative
        .components()
        .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(error("storage key contains unsafe path components"));
    }
    Ok(root.join(relative))
}

pub fn secure_join_no_symlinks(root: &Path, key: &str) -> Result<PathBuf, FileSecurityError> {
    let destination = secure_join(root, key)?;
    reject_existing_link_components(root, Path::new(key))?;
    Ok(destination)
}

fn reject_existing_link_components(root: &Path, relative: &Path) -> Result<(), FileSecurityError> {
    let mut current = root.to_owned();
    reject_link_if_present(&current)?;
    for component in relative.components() {
        let Component::Normal(segment) = component else {
            return Err(error("storage key contains unsafe path components"));
        };
        current.push(segment);
        reject_link_if_present(&current)?;
    }
    Ok(())
}

fn reject_link_if_present(path: &Path) -> Result<(), FileSecurityError> {
    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata_is_link_like(&metadata) => Err(error(
            "storage path contains a symbolic link or reparse point",
        )),
        Ok(_) => Ok(()),
        Err(failure) if failure.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(failure) => Err(error(format!("could not inspect storage path: {failure}"))),
    }
}

#[cfg(windows)]
fn metadata_is_link_like(metadata: &std::fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
    metadata.file_type().is_symlink()
        || metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
}

#[cfg(not(windows))]
fn metadata_is_link_like(metadata: &std::fs::Metadata) -> bool {
    metadata.file_type().is_symlink()
}

pub async fn publish_generated_file(
    source: &Path,
    storage_root: &Path,
    storage_key: &str,
) -> Result<PathBuf, FileSecurityError> {
    let destination = secure_join_no_symlinks(storage_root, storage_key)?;
    let parent = destination
        .parent()
        .ok_or_else(|| error("storage destination has no parent directory"))?;
    fs::create_dir_all(parent)
        .await
        .map_err(|failure| error(format!("could not create storage directory: {failure}")))?;
    set_directory_permissions(parent)?;
    reject_existing_link_components(storage_root, Path::new(storage_key))?;
    if fs::try_exists(&destination)
        .await
        .map_err(|failure| error(format!("could not inspect storage destination: {failure}")))?
    {
        return Err(error("storage destination already exists"));
    }
    fs::rename(source, &destination)
        .await
        .map_err(|failure| error(format!("could not publish generated file: {failure}")))?;
    Ok(destination)
}

pub async fn sha256_file(path: &Path, max_bytes: u64) -> Result<(String, u64), FileSecurityError> {
    let mut file = File::open(path)
        .await
        .map_err(|failure| error(format!("could not open stored file: {failure}")))?;
    let mut digest = Sha256::new();
    let mut total = 0u64;
    let mut buffer = vec![0u8; 64 * 1_024];
    loop {
        let read = tokio::io::AsyncReadExt::read(&mut file, &mut buffer)
            .await
            .map_err(|failure| error(format!("could not read stored file: {failure}")))?;
        if read == 0 {
            break;
        }
        total = total
            .checked_add(read as u64)
            .ok_or_else(|| error("stored file size overflowed"))?;
        if total > max_bytes {
            return Err(error("stored file exceeds verification limit"));
        }
        digest.update(&buffer[..read]);
    }
    Ok((format!("{:x}", digest.finalize()), total))
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MalwareScanOutcome {
    Clean,
    Infected,
    Error,
    Unavailable,
}

pub trait MalwareScanner {
    fn scan_verdict(&self, sha256: &str) -> MalwareScanOutcome;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NoopMalwareScanner;

impl NoopMalwareScanner {
    pub fn scan_verdict(&self, sha256: &str) -> MalwareScanOutcome {
        <Self as MalwareScanner>::scan_verdict(self, sha256)
    }
}

impl MalwareScanner for NoopMalwareScanner {
    fn scan_verdict(&self, _sha256: &str) -> MalwareScanOutcome {
        MalwareScanOutcome::Unavailable
    }
}

pub struct SecureTempUpload {
    path: PathBuf,
    file: Option<File>,
    sha256: Sha256,
    size: u64,
    max_bytes: u64,
    original_filename: String,
    declared_content_type: String,
    persisted: bool,
}

impl SecureTempUpload {
    pub async fn create(
        storage_root: &Path,
        max_bytes: u64,
        original_filename: &str,
        declared_content_type: &str,
    ) -> Result<Self, FileSecurityError> {
        fs::create_dir_all(storage_root)
            .await
            .map_err(|failure| error(format!("could not create storage root: {failure}")))?;
        let temp_dir = secure_join_no_symlinks(storage_root, "quarantine/tmp")?;
        fs::create_dir_all(&temp_dir).await.map_err(|failure| {
            error(format!(
                "could not create upload staging directory: {failure}"
            ))
        })?;
        set_directory_permissions(&temp_dir)?;
        reject_existing_link_components(storage_root, Path::new("quarantine/tmp"))?;

        let path = temp_dir.join(format!("{}.part", Uuid::now_v7()));
        let mut options = OpenOptions::new();
        options.create_new(true).write(true);
        set_secure_open_permissions(&mut options);
        let file = options
            .open(&path)
            .await
            .map_err(|failure| error(format!("could not create upload staging file: {failure}")))?;

        Ok(Self {
            path,
            file: Some(file),
            sha256: Sha256::new(),
            size: 0,
            max_bytes,
            original_filename: normalize_client_filename(original_filename),
            declared_content_type: declared_content_type.to_owned(),
            persisted: false,
        })
    }

    pub async fn write_chunk(&mut self, chunk: &[u8]) -> Result<(), FileSecurityError> {
        let chunk_size =
            u64::try_from(chunk.len()).map_err(|_| error("upload chunk size is invalid"))?;
        let next_size = self
            .size
            .checked_add(chunk_size)
            .ok_or_else(|| error("upload size overflowed"))?;
        if next_size > self.max_bytes {
            return Err(error(format!(
                "file exceeds maximum size of {} bytes",
                self.max_bytes
            )));
        }
        let file = self
            .file
            .as_mut()
            .ok_or_else(|| error("upload staging file is closed"))?;
        file.write_all(chunk)
            .await
            .map_err(|failure| error(format!("could not write upload staging file: {failure}")))?;
        self.sha256.update(chunk);
        self.size = next_size;
        Ok(())
    }

    pub async fn finish(mut self) -> Result<StagedUpload, FileSecurityError> {
        if self.size == 0 {
            return Err(error("uploaded file must not be empty"));
        }
        if let Some(mut file) = self.file.take() {
            file.flush().await.map_err(|failure| {
                error(format!("could not flush upload staging file: {failure}"))
            })?;
            file.sync_data().await.map_err(|failure| {
                error(format!("could not sync upload staging file: {failure}"))
            })?;
        }
        let sha256 = format!("{:x}", self.sha256.clone().finalize());
        self.persisted = true;
        Ok(StagedUpload {
            path: self.path.clone(),
            size: self.size,
            sha256,
            original_filename: self.original_filename.clone(),
            declared_content_type: self.declared_content_type.clone(),
            persisted: false,
        })
    }
}

impl Drop for SecureTempUpload {
    fn drop(&mut self) {
        if !self.persisted {
            self.file.take();
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

pub struct StagedUpload {
    path: PathBuf,
    pub size: u64,
    pub sha256: String,
    pub original_filename: String,
    pub declared_content_type: String,
    persisted: bool,
}

impl StagedUpload {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub async fn read_prefix(&self, limit: usize) -> Result<Vec<u8>, FileSecurityError> {
        let mut file = File::open(&self.path)
            .await
            .map_err(|failure| error(format!("could not open staged upload: {failure}")))?;
        let mut output = Vec::with_capacity(limit.min(self.size as usize));
        let mut buffer = vec![0u8; 8_192.min(limit.max(1))];
        while output.len() < limit {
            let remaining = limit - output.len();
            let read_limit = buffer.len().min(remaining);
            let read = tokio::io::AsyncReadExt::read(&mut file, &mut buffer[..read_limit])
                .await
                .map_err(|failure| error(format!("could not read staged upload: {failure}")))?;
            if read == 0 {
                break;
            }
            output.extend_from_slice(&buffer[..read]);
        }
        Ok(output)
    }

    pub async fn read_suffix(&self, limit: usize) -> Result<Vec<u8>, FileSecurityError> {
        let mut file = File::open(&self.path)
            .await
            .map_err(|failure| error(format!("could not open staged upload: {failure}")))?;
        let requested = u64::try_from(limit).unwrap_or(u64::MAX);
        let read_size = self.size.min(requested);
        file.seek(std::io::SeekFrom::Start(self.size - read_size))
            .await
            .map_err(|failure| error(format!("could not seek staged upload: {failure}")))?;
        let output_size =
            usize::try_from(read_size).map_err(|_| error("staged upload suffix is too large"))?;
        let mut output = vec![0u8; output_size];
        file.read_exact(&mut output)
            .await
            .map_err(|failure| error(format!("could not read staged upload: {failure}")))?;
        Ok(output)
    }

    pub async fn validate_utf8_text(&self) -> Result<(), FileSecurityError> {
        let mut file = File::open(&self.path)
            .await
            .map_err(|failure| error(format!("could not open staged upload: {failure}")))?;
        let mut buffer = [0u8; 8_192];
        let mut carry = Vec::with_capacity(8_195);
        loop {
            let read = file
                .read(&mut buffer)
                .await
                .map_err(|failure| error(format!("could not read staged upload: {failure}")))?;
            if read == 0 {
                break;
            }
            if buffer[..read].contains(&0) {
                return Err(error("plain text file contains a NUL byte"));
            }
            carry.extend_from_slice(&buffer[..read]);
            match std::str::from_utf8(&carry) {
                Ok(_) => carry.clear(),
                Err(failure) if failure.error_len().is_some() => {
                    return Err(error("plain text file is not valid UTF-8"));
                }
                Err(failure) => {
                    let incomplete = carry.split_off(failure.valid_up_to());
                    if incomplete.len() > 3 {
                        return Err(error("plain text file is not valid UTF-8"));
                    }
                    carry = incomplete;
                }
            }
        }
        if !carry.is_empty() {
            return Err(error("plain text file ends with incomplete UTF-8"));
        }
        Ok(())
    }

    pub async fn read_all_bounded(&self, limit: u64) -> Result<Vec<u8>, FileSecurityError> {
        if self.size > limit {
            return Err(error("staged upload exceeds in-memory processing limit"));
        }
        fs::read(&self.path)
            .await
            .map_err(|failure| error(format!("could not read staged upload: {failure}")))
    }

    pub async fn persist(
        mut self,
        storage_root: &Path,
        storage_key: &str,
    ) -> Result<PathBuf, FileSecurityError> {
        let destination = secure_join_no_symlinks(storage_root, storage_key)?;
        let parent = destination
            .parent()
            .ok_or_else(|| error("storage destination has no parent directory"))?;
        fs::create_dir_all(parent)
            .await
            .map_err(|failure| error(format!("could not create storage directory: {failure}")))?;
        set_directory_permissions(parent)?;
        reject_existing_link_components(storage_root, Path::new(storage_key))?;
        if fs::try_exists(&destination)
            .await
            .map_err(|failure| error(format!("could not inspect storage destination: {failure}")))?
        {
            return Err(error("storage destination already exists"));
        }
        fs::rename(&self.path, &destination)
            .await
            .map_err(|failure| error(format!("could not publish staged upload: {failure}")))?;
        self.persisted = true;
        Ok(destination)
    }

    pub async fn remove(mut self) -> Result<(), FileSecurityError> {
        match fs::remove_file(&self.path).await {
            Ok(()) => {
                self.persisted = true;
                Ok(())
            }
            Err(failure) if failure.kind() == std::io::ErrorKind::NotFound => {
                self.persisted = true;
                Ok(())
            }
            Err(failure) => Err(error(format!("could not remove staged upload: {failure}"))),
        }
    }
}

impl Drop for StagedUpload {
    fn drop(&mut self) {
        if !self.persisted {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

pub async fn cleanup_stale_temporary_files(
    storage_root: &Path,
    minimum_age: Duration,
    limit: usize,
) -> Result<usize, FileSecurityError> {
    if limit == 0 || limit > 1_000 {
        return Err(error("temporary cleanup batch size is invalid"));
    }
    let temp_dir = secure_join_no_symlinks(storage_root, "quarantine/tmp")?;
    let mut directory = match fs::read_dir(&temp_dir).await {
        Ok(directory) => directory,
        Err(failure) if failure.kind() == std::io::ErrorKind::NotFound => return Ok(0),
        Err(failure) => {
            return Err(error(format!(
                "could not read upload staging directory: {failure}"
            )));
        }
    };
    let cutoff = SystemTime::now()
        .checked_sub(minimum_age)
        .ok_or_else(|| error("temporary cleanup age is invalid"))?;
    let mut inspected = 0usize;
    let mut removed = 0usize;
    while inspected < limit {
        let Some(entry) = directory
            .next_entry()
            .await
            .map_err(|failure| error(format!("could not read staging entry: {failure}")))?
        else {
            break;
        };
        inspected += 1;
        let file_type = entry
            .file_type()
            .await
            .map_err(|failure| error(format!("could not inspect staging entry: {failure}")))?;
        if !file_type.is_file() || file_type.is_symlink() {
            continue;
        }
        let filename = entry.file_name();
        let Some(filename) = filename.to_str() else {
            continue;
        };
        let Some(id) = filename.strip_suffix(".part") else {
            continue;
        };
        if Uuid::parse_str(id).is_err() {
            continue;
        }
        let metadata = entry
            .metadata()
            .await
            .map_err(|failure| error(format!("could not inspect staging metadata: {failure}")))?;
        let modified = metadata
            .modified()
            .map_err(|failure| error(format!("could not inspect staging age: {failure}")))?;
        if modified > cutoff {
            continue;
        }
        match fs::remove_file(entry.path()).await {
            Ok(()) => removed += 1,
            Err(failure) if failure.kind() == std::io::ErrorKind::NotFound => {}
            Err(failure) => {
                return Err(error(format!(
                    "could not remove stale staging file: {failure}"
                )));
            }
        }
    }
    Ok(removed)
}

pub async fn cleanup_stale_processing_directories(
    storage_root: &Path,
    minimum_age: Duration,
    limit: usize,
) -> Result<usize, FileSecurityError> {
    if limit == 0 || limit > 1_000 {
        return Err(error("processing cleanup batch size is invalid"));
    }
    let processing_root = secure_join_no_symlinks(storage_root, "quarantine/processed")?;
    let mut directory = match fs::read_dir(&processing_root).await {
        Ok(directory) => directory,
        Err(failure) if failure.kind() == std::io::ErrorKind::NotFound => return Ok(0),
        Err(failure) => {
            return Err(error(format!(
                "could not read image processing directory: {failure}"
            )));
        }
    };
    let cutoff = SystemTime::now()
        .checked_sub(minimum_age)
        .ok_or_else(|| error("processing cleanup age is invalid"))?;
    let allowed_files = [
        "original.webp",
        "thumbnail.webp",
        "small.webp",
        "medium.webp",
        "large.webp",
    ];
    let mut inspected = 0usize;
    let mut removed = 0usize;
    while inspected < limit {
        let Some(entry) = directory
            .next_entry()
            .await
            .map_err(|failure| error(format!("could not read processing entry: {failure}")))?
        else {
            break;
        };
        inspected += 1;
        let Some(name) = entry.file_name().to_str().map(ToOwned::to_owned) else {
            continue;
        };
        if Uuid::parse_str(&name).is_err() {
            continue;
        }
        let metadata = fs::symlink_metadata(entry.path())
            .await
            .map_err(|failure| error(format!("could not inspect processing entry: {failure}")))?;
        if metadata_is_link_like(&metadata) || !metadata.is_dir() {
            continue;
        }
        let modified = metadata
            .modified()
            .map_err(|failure| error(format!("could not inspect processing age: {failure}")))?;
        if modified > cutoff {
            continue;
        }

        let mut children = fs::read_dir(entry.path())
            .await
            .map_err(|failure| error(format!("could not read processing files: {failure}")))?;
        let mut files = Vec::new();
        let mut safe = true;
        while let Some(child) = children
            .next_entry()
            .await
            .map_err(|failure| error(format!("could not read processing file: {failure}")))?
        {
            let Some(filename) = child.file_name().to_str().map(ToOwned::to_owned) else {
                safe = false;
                break;
            };
            let child_metadata = fs::symlink_metadata(child.path())
                .await
                .map_err(|failure| {
                    error(format!("could not inspect processing file: {failure}"))
                })?;
            if metadata_is_link_like(&child_metadata)
                || !child_metadata.is_file()
                || !allowed_files.contains(&filename.as_str())
            {
                safe = false;
                break;
            }
            files.push(child.path());
        }
        drop(children);
        if !safe {
            continue;
        }
        for file in files {
            match fs::remove_file(file).await {
                Ok(()) => {}
                Err(failure) if failure.kind() == std::io::ErrorKind::NotFound => {}
                Err(failure) => {
                    return Err(error(format!(
                        "could not remove stale processing file: {failure}"
                    )));
                }
            }
        }
        match fs::remove_dir(entry.path()).await {
            Ok(()) => removed += 1,
            Err(failure) if failure.kind() == std::io::ErrorKind::NotFound => {}
            Err(failure) => {
                return Err(error(format!(
                    "could not remove stale processing directory: {failure}"
                )));
            }
        }
    }
    Ok(removed)
}

#[cfg(unix)]
fn set_secure_open_permissions(options: &mut OpenOptions) {
    use std::os::unix::fs::OpenOptionsExt;
    options.mode(0o600);
}

#[cfg(not(unix))]
fn set_secure_open_permissions(_options: &mut OpenOptions) {}

#[cfg(unix)]
fn set_directory_permissions(path: &Path) -> Result<(), FileSecurityError> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700))
        .map_err(|failure| error(format!("could not secure storage directory: {failure}")))
}

#[cfg(not(unix))]
fn set_directory_permissions(_path: &Path) -> Result<(), FileSecurityError> {
    Ok(())
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ArchiveLimits {
    pub max_entries: usize,
    pub max_entry_uncompressed_bytes: u64,
    pub max_total_uncompressed_bytes: u64,
    pub max_compression_ratio: u64,
    pub max_nested_archives: usize,
    pub max_path_bytes: usize,
    pub max_path_depth: usize,
    pub max_processing_millis: u64,
}

impl ArchiveLimits {
    pub const fn marketplace() -> Self {
        Self {
            max_entries: 500,
            max_entry_uncompressed_bytes: 25 * MIB,
            max_total_uncompressed_bytes: 100 * MIB,
            max_compression_ratio: 100,
            max_nested_archives: 0,
            max_path_bytes: 240,
            max_path_depth: 16,
            max_processing_millis: 10_000,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ArchiveEntryReport {
    pub path: String,
    pub compressed_bytes: u64,
    pub uncompressed_bytes: u64,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ArchiveReport {
    pub entries: Vec<ArchiveEntryReport>,
    pub total_compressed_bytes: u64,
    pub total_uncompressed_bytes: u64,
}

#[derive(Debug, Clone)]
struct VerifiedArchive {
    report: ArchiveReport,
    entries: Vec<VerifiedArchiveEntry>,
}

#[derive(Debug, Clone)]
struct VerifiedArchiveEntry {
    report: ArchiveEntryReport,
    raw_name: Vec<u8>,
    local_header_offset: u64,
    data_start: u64,
    compression_method: u16,
    flags: u16,
    crc32: u32,
}

pub fn inspect_archive(
    archive_path: &Path,
    limits: ArchiveLimits,
) -> Result<ArchiveReport, FileSecurityError> {
    inspect_archive_internal(archive_path, limits).map(|verified| verified.report)
}

fn inspect_archive_internal(
    archive_path: &Path,
    limits: ArchiveLimits,
) -> Result<VerifiedArchive, FileSecurityError> {
    let deadline = archive_deadline(limits)?;
    let mut file = StdFile::open(archive_path)
        .map_err(|failure| error(format!("could not open archive: {failure}")))?;
    let file_size = file
        .metadata()
        .map_err(|failure| error(format!("could not inspect archive: {failure}")))?
        .len();
    if file_size < 22 {
        return Err(error("archive is too small to contain a ZIP directory"));
    }

    let search_size = file_size.min(65_557);
    let search_start = file_size - search_size;
    file.seek(SeekFrom::Start(search_start))
        .map_err(|failure| error(format!("could not seek archive: {failure}")))?;
    let mut tail = vec![
        0u8;
        usize::try_from(search_size)
            .map_err(|_| error("archive search window is too large"))?
    ];
    file.read_exact(&mut tail)
        .map_err(|failure| error(format!("could not read ZIP directory: {failure}")))?;
    let eocd_in_tail = tail
        .windows(4)
        .rposition(|window| window == b"PK\x05\x06")
        .ok_or_else(|| error("ZIP end-of-central-directory record was not found"))?;
    let eocd = tail
        .get(eocd_in_tail..eocd_in_tail + 22)
        .ok_or_else(|| error("ZIP end-of-central-directory record is truncated"))?;
    let comment_length = usize::from(le_u16(eocd, 20)?);
    if eocd_in_tail
        .checked_add(22)
        .and_then(|value| value.checked_add(comment_length))
        != Some(tail.len())
    {
        return Err(error("ZIP contains trailing or malformed end metadata"));
    }
    if le_u16(eocd, 4)? != 0 || le_u16(eocd, 6)? != 0 {
        return Err(error("multi-disk ZIP archives are not accepted"));
    }
    let entries_on_disk = le_u16(eocd, 8)?;
    let entry_count = le_u16(eocd, 10)?;
    if entries_on_disk != entry_count {
        return Err(error("ZIP central-directory entry counts do not match"));
    }
    if entry_count == u16::MAX || le_u32(eocd, 12)? == u32::MAX || le_u32(eocd, 16)? == u32::MAX {
        return Err(error("ZIP64 archives are not accepted"));
    }
    let entry_count = usize::from(entry_count);
    if entry_count > limits.max_entries {
        return Err(error("archive contains too many entries"));
    }
    let central_size = u64::from(le_u32(eocd, 12)?);
    let central_offset = u64::from(le_u32(eocd, 16)?);
    let eocd_offset = search_start
        .checked_add(
            u64::try_from(eocd_in_tail).map_err(|_| error("ZIP directory offset overflowed"))?,
        )
        .ok_or_else(|| error("ZIP directory offset overflowed"))?;
    if central_offset
        .checked_add(central_size)
        .is_none_or(|end| end != eocd_offset)
    {
        return Err(error("ZIP central directory is out of bounds"));
    }

    file.seek(SeekFrom::Start(central_offset))
        .map_err(|failure| error(format!("could not seek ZIP central directory: {failure}")))?;
    let mut paths = HashSet::with_capacity(entry_count);
    let mut local_offsets = HashSet::with_capacity(entry_count);
    let mut nested_archives = 0usize;
    let mut total_compressed_bytes = 0u64;
    let mut total_uncompressed_bytes = 0u64;
    let mut entries = Vec::with_capacity(entry_count);

    for _ in 0..entry_count {
        ensure_archive_deadline(deadline)?;
        let mut header = [0u8; 46];
        file.read_exact(&mut header)
            .map_err(|failure| error(format!("could not read ZIP central entry: {failure}")))?;
        if &header[..4] != b"PK\x01\x02" {
            return Err(error("ZIP central-directory entry signature is invalid"));
        }
        let made_by_system = header[5];
        let flags = le_u16(&header, 8)?;
        let compression_method = le_u16(&header, 10)?;
        let crc32 = le_u32(&header, 16)?;
        let compressed = le_u32(&header, 20)?;
        let uncompressed = le_u32(&header, 24)?;
        if compressed == u32::MAX || uncompressed == u32::MAX {
            return Err(error("ZIP64 entries are not accepted"));
        }
        if flags & 0x0001 != 0 {
            return Err(error("encrypted ZIP entries are not accepted"));
        }
        if !matches!(compression_method, 0 | 8) {
            return Err(error("archive uses an unsupported compression method"));
        }
        let name_length = usize::from(le_u16(&header, 28)?);
        let extra_length = usize::from(le_u16(&header, 30)?);
        let comment_length = usize::from(le_u16(&header, 32)?);
        if le_u16(&header, 34)? != 0 {
            return Err(error("archive entry refers to another ZIP disk"));
        }
        let external_attributes = le_u32(&header, 38)?;
        let local_header_offset = u64::from(le_u32(&header, 42)?);
        if !local_offsets.insert(local_header_offset) {
            return Err(error("archive entries share a local-header offset"));
        }
        let mut raw_name = vec![0u8; name_length];
        file.read_exact(&mut raw_name)
            .map_err(|failure| error(format!("could not read ZIP entry name: {failure}")))?;
        file.seek(SeekFrom::Current(
            i64::try_from(extra_length + comment_length)
                .map_err(|_| error("ZIP entry metadata is too large"))?,
        ))
        .map_err(|failure| error(format!("could not skip ZIP entry metadata: {failure}")))?;

        let raw_name_text = std::str::from_utf8(&raw_name)
            .map_err(|_| error("archive entry paths must be UTF-8"))?;
        let is_directory = raw_name_text.ends_with('/');
        let normalized = normalize_archive_entry_path(raw_name_text, is_directory, limits)?;
        let comparison_key = normalized.to_ascii_lowercase();
        if !paths.insert(comparison_key) {
            return Err(error("archive contains duplicate or case-colliding paths"));
        }
        let unix_mode = if made_by_system == 3 {
            external_attributes >> 16
        } else {
            0
        };
        if unix_mode & 0o170000 == 0o120000 {
            return Err(error("symbolic links are not accepted in archives"));
        }
        if unix_mode & 0o170000 != 0
            && unix_mode & 0o170000 != 0o100000
            && unix_mode & 0o170000 != 0o040000
        {
            return Err(error("archive contains an unsupported special file"));
        }

        let compressed = u64::from(compressed);
        let uncompressed = u64::from(uncompressed);
        if is_directory {
            if compressed != 0 || uncompressed != 0 {
                return Err(error("archive directory entry has file content"));
            }
            continue;
        }
        if raw_name.len() > limits.max_path_bytes {
            return Err(error("archive entry path is unsafe"));
        }

        if is_nested_archive_name(&normalized) {
            nested_archives = nested_archives
                .checked_add(1)
                .ok_or_else(|| error("nested archive count overflowed"))?;
            if nested_archives > limits.max_nested_archives {
                return Err(error("nested archives are not accepted"));
            }
        }

        if uncompressed > limits.max_entry_uncompressed_bytes {
            return Err(error("archive entry exceeds the uncompressed size limit"));
        }
        if uncompressed > 0
            && (compressed == 0
                || uncompressed > compressed.saturating_mul(limits.max_compression_ratio))
        {
            return Err(error("archive entry exceeds the compression ratio limit"));
        }
        total_compressed_bytes = total_compressed_bytes
            .checked_add(compressed)
            .ok_or_else(|| error("archive compressed size overflowed"))?;
        total_uncompressed_bytes = total_uncompressed_bytes
            .checked_add(uncompressed)
            .ok_or_else(|| error("archive uncompressed size overflowed"))?;
        if total_uncompressed_bytes > limits.max_total_uncompressed_bytes {
            return Err(error("archive exceeds the total uncompressed size limit"));
        }
        entries.push(VerifiedArchiveEntry {
            report: ArchiveEntryReport {
                path: normalized,
                compressed_bytes: compressed,
                uncompressed_bytes: uncompressed,
            },
            raw_name,
            local_header_offset,
            data_start: 0,
            compression_method,
            flags,
            crc32,
        });
    }

    if entries.is_empty() {
        return Err(error("archive must contain at least one regular file"));
    }
    let central_end = file
        .stream_position()
        .map_err(|failure| error(format!("could not inspect ZIP cursor: {failure}")))?;
    if central_end != eocd_offset {
        return Err(error(
            "ZIP central-directory size does not match its entries",
        ));
    }

    for entry in &mut entries {
        entry.data_start = validate_local_zip_header(&mut file, entry, central_offset)?;
    }
    let mut ranges = entries
        .iter()
        .map(|entry| {
            let end = entry
                .data_start
                .checked_add(entry.report.compressed_bytes)
                .ok_or_else(|| error("ZIP entry data range overflowed"))?;
            Ok((entry.local_header_offset, end))
        })
        .collect::<Result<Vec<_>, FileSecurityError>>()?;
    ranges.sort_unstable();
    if ranges.windows(2).any(|pair| pair[1].0 < pair[0].1) {
        return Err(error("archive contains overlapping local file data"));
    }

    let report = ArchiveReport {
        entries: entries.iter().map(|entry| entry.report.clone()).collect(),
        total_compressed_bytes,
        total_uncompressed_bytes,
    };
    Ok(VerifiedArchive { report, entries })
}

pub fn extract_archive_to_quarantine(
    archive_path: &Path,
    destination: &Path,
    limits: ArchiveLimits,
) -> Result<ArchiveReport, FileSecurityError> {
    let verified = inspect_archive_internal(archive_path, limits)?;
    std::fs::create_dir(destination)
        .map_err(|failure| error(format!("could not create extraction quarantine: {failure}")))?;
    set_directory_permissions(destination)?;

    let extraction = extract_verified_entries(
        archive_path,
        destination,
        &verified,
        archive_deadline(limits)?,
    );
    if extraction.is_err() {
        let _ = std::fs::remove_dir_all(destination);
    }
    extraction.map(|()| verified.report)
}

fn extract_verified_entries(
    archive_path: &Path,
    destination: &Path,
    verified: &VerifiedArchive,
    deadline: Instant,
) -> Result<(), FileSecurityError> {
    let mut archive = StdFile::open(archive_path)
        .map_err(|failure| error(format!("could not reopen archive: {failure}")))?;
    for entry in &verified.entries {
        ensure_archive_deadline(deadline)?;
        let output_path = destination.join(&entry.report.path);
        if !output_path.starts_with(destination) {
            return Err(error("archive output escapes the extraction root"));
        }
        let parent = output_path
            .parent()
            .ok_or_else(|| error("archive output has no parent directory"))?;
        std::fs::create_dir_all(parent).map_err(|failure| {
            error(format!("could not create extraction directory: {failure}"))
        })?;
        set_directory_permissions(parent)?;

        let mut options = std::fs::OpenOptions::new();
        options.create_new(true).write(true);
        set_secure_std_open_permissions(&mut options);
        let mut output = options
            .open(&output_path)
            .map_err(|failure| error(format!("could not create extracted file: {failure}")))?;
        archive
            .seek(SeekFrom::Start(entry.data_start))
            .map_err(|failure| error(format!("could not seek ZIP entry data: {failure}")))?;
        let compressed = (&mut archive).take(entry.report.compressed_bytes);
        let (copied, crc32) = match entry.compression_method {
            0 => copy_archive_entry(
                compressed,
                &mut output,
                entry.report.uncompressed_bytes,
                deadline,
            )?,
            8 => copy_archive_entry(
                DeflateDecoder::new(compressed),
                &mut output,
                entry.report.uncompressed_bytes,
                deadline,
            )?,
            _ => return Err(error("archive uses an unsupported compression method")),
        };
        output
            .flush()
            .map_err(|failure| error(format!("could not flush extracted file: {failure}")))?;
        if copied != entry.report.uncompressed_bytes {
            return Err(error("extracted ZIP entry size does not match metadata"));
        }
        if crc32 != entry.crc32 {
            return Err(error(
                "extracted ZIP entry checksum does not match metadata",
            ));
        }
    }
    Ok(())
}

fn validate_local_zip_header(
    file: &mut StdFile,
    entry: &VerifiedArchiveEntry,
    central_offset: u64,
) -> Result<u64, FileSecurityError> {
    if entry.local_header_offset >= central_offset {
        return Err(error("ZIP local header overlaps the central directory"));
    }
    file.seek(SeekFrom::Start(entry.local_header_offset))
        .map_err(|failure| error(format!("could not seek ZIP local header: {failure}")))?;
    let mut header = [0u8; 30];
    file.read_exact(&mut header)
        .map_err(|failure| error(format!("could not read ZIP local header: {failure}")))?;
    if &header[..4] != b"PK\x03\x04" {
        return Err(error("ZIP local header signature is invalid"));
    }
    if le_u16(&header, 6)? != entry.flags || le_u16(&header, 8)? != entry.compression_method {
        return Err(error("ZIP local and central headers disagree"));
    }
    let name_length = usize::from(le_u16(&header, 26)?);
    let extra_length = usize::from(le_u16(&header, 28)?);
    let mut local_name = vec![0u8; name_length];
    file.read_exact(&mut local_name)
        .map_err(|failure| error(format!("could not read ZIP local name: {failure}")))?;
    if local_name != entry.raw_name {
        return Err(error("ZIP local and central entry names disagree"));
    }
    let data_start = entry
        .local_header_offset
        .checked_add(30)
        .and_then(|value| value.checked_add(u64::try_from(name_length).ok()?))
        .and_then(|value| value.checked_add(u64::try_from(extra_length).ok()?))
        .ok_or_else(|| error("ZIP local data offset overflowed"))?;
    if data_start
        .checked_add(entry.report.compressed_bytes)
        .is_none_or(|end| end > central_offset)
    {
        return Err(error("ZIP entry data is out of bounds"));
    }
    Ok(data_start)
}

fn normalize_archive_entry_path(
    raw_name: &str,
    is_directory: bool,
    limits: ArchiveLimits,
) -> Result<String, FileSecurityError> {
    if raw_name.len() > limits.max_path_bytes
        || !raw_name.is_ascii()
        || raw_name.contains(['\\', ':', '\0', '%'])
        || raw_name.starts_with('/')
    {
        return Err(error("archive entry path is unsafe"));
    }
    let candidate = if is_directory {
        raw_name.trim_end_matches('/')
    } else {
        raw_name
    };
    if candidate.is_empty()
        || candidate
            .split('/')
            .any(|segment| segment.is_empty() || segment == "." || segment == "..")
    {
        return Err(error("archive entry contains unsafe path components"));
    }
    if candidate.split('/').count() > limits.max_path_depth {
        return Err(error("archive entry path exceeds the depth limit"));
    }
    if candidate.chars().any(|character| character.is_control()) {
        return Err(error("archive entry path contains control characters"));
    }
    Ok(candidate.to_owned())
}

fn copy_archive_entry(
    mut reader: impl Read,
    writer: &mut impl Write,
    expected_size: u64,
    deadline: Instant,
) -> Result<(u64, u32), FileSecurityError> {
    let mut crc32 = Crc32Hasher::new();
    let mut copied = 0u64;
    let mut buffer = [0u8; 65_536];
    loop {
        ensure_archive_deadline(deadline)?;
        let read = reader
            .read(&mut buffer)
            .map_err(|failure| error(format!("could not decompress ZIP entry: {failure}")))?;
        if read == 0 {
            break;
        }
        copied = copied
            .checked_add(u64::try_from(read).map_err(|_| error("ZIP output size overflowed"))?)
            .ok_or_else(|| error("ZIP output size overflowed"))?;
        if copied > expected_size {
            return Err(error("ZIP entry produced more bytes than declared"));
        }
        crc32.update(&buffer[..read]);
        writer
            .write_all(&buffer[..read])
            .map_err(|failure| error(format!("could not write extracted file: {failure}")))?;
    }
    Ok((copied, crc32.finalize()))
}

fn archive_deadline(limits: ArchiveLimits) -> Result<Instant, FileSecurityError> {
    if limits.max_processing_millis == 0 || limits.max_processing_millis > 60_000 {
        return Err(error("archive processing time limit is invalid"));
    }
    Instant::now()
        .checked_add(Duration::from_millis(limits.max_processing_millis))
        .ok_or_else(|| error("archive processing deadline overflowed"))
}

fn ensure_archive_deadline(deadline: Instant) -> Result<(), FileSecurityError> {
    if Instant::now() >= deadline {
        Err(error("archive processing exceeded its time limit"))
    } else {
        Ok(())
    }
}

fn le_u16(bytes: &[u8], offset: usize) -> Result<u16, FileSecurityError> {
    let value = bytes
        .get(offset..offset + 2)
        .ok_or_else(|| error("ZIP metadata is truncated"))?;
    Ok(u16::from_le_bytes([value[0], value[1]]))
}

fn le_u32(bytes: &[u8], offset: usize) -> Result<u32, FileSecurityError> {
    let value = bytes
        .get(offset..offset + 4)
        .ok_or_else(|| error("ZIP metadata is truncated"))?;
    Ok(u32::from_le_bytes([value[0], value[1], value[2], value[3]]))
}

#[cfg(unix)]
fn set_secure_std_open_permissions(options: &mut std::fs::OpenOptions) {
    use std::os::unix::fs::OpenOptionsExt;
    options.mode(0o600);
}

#[cfg(not(unix))]
fn set_secure_std_open_permissions(_options: &mut std::fs::OpenOptions) {}

fn is_nested_archive_name(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    [".zip", ".tar", ".tgz", ".gz", ".bz2", ".xz", ".7z", ".rar"]
        .iter()
        .any(|extension| lower.ends_with(extension))
}

#[cfg(test)]
mod tests {
    use super::{
        FileKind, UploadPurpose, detect_file_kind, normalize_client_filename,
        validate_detected_kind,
    };

    #[test]
    fn octet_stream_is_accepted_only_after_content_detection() {
        assert_eq!(
            validate_detected_kind(
                UploadPurpose::PrivateDocument,
                "application/octet-stream",
                b"%PDF-1.7\n"
            )
            .unwrap(),
            FileKind::Pdf
        );
        assert!(
            validate_detected_kind(
                UploadPurpose::PrivateDocument,
                "image/svg+xml",
                b"<svg></svg>"
            )
            .is_err()
        );
    }

    #[test]
    fn text_and_active_markup_are_distinct() {
        assert_eq!(detect_file_kind(b"plain text"), Some(FileKind::Text));
        assert_eq!(detect_file_kind(b"<html></html>"), Some(FileKind::Html));
    }

    #[test]
    fn filename_normalization_removes_path_and_header_controls() {
        assert_eq!(
            normalize_client_filename("../report\r\n final.pdf"),
            "report-final.pdf"
        );
    }
}
