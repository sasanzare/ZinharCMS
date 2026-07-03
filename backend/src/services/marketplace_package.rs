use sha2::{Digest, Sha256};
use std::error::Error;
use std::fmt;

use crate::services::marketplace_manifest::is_semver;

pub const MARKETPLACE_PACKAGE_OBJECT_PREFIX: &str = "marketplace/packages";
pub const MAX_MARKETPLACE_PACKAGE_BYTES: u64 = 52_428_800;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PackageStorageError {
    pub message: String,
}

impl fmt::Display for PackageStorageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for PackageStorageError {}

pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!("{digest:x}")
}

pub fn verify_package_checksum(bytes: &[u8], expected_sha256: &str) -> bool {
    is_valid_sha256_hex(expected_sha256) && sha256_hex(bytes) == expected_sha256
}

pub fn validate_package_size(size_bytes: u64) -> Result<(), PackageStorageError> {
    if size_bytes == 0 {
        return Err(error("package artifact must not be empty"));
    }

    if size_bytes > MAX_MARKETPLACE_PACKAGE_BYTES {
        return Err(error("package artifact exceeds the Marketplace size limit"));
    }

    Ok(())
}

pub fn marketplace_package_object_key(
    creator_slug: &str,
    listing_slug: &str,
    version: &str,
    sha256: &str,
) -> Result<String, PackageStorageError> {
    let creator_slug = safe_slug(creator_slug, "creator slug")?;
    let listing_slug = safe_slug(listing_slug, "listing slug")?;

    if !is_semver(version) {
        return Err(error("package version must use semantic version format"));
    }

    if !is_valid_sha256_hex(sha256) {
        return Err(error("package checksum must be lowercase SHA-256 hex"));
    }

    Ok(format!(
        "{MARKETPLACE_PACKAGE_OBJECT_PREFIX}/{creator_slug}/{listing_slug}/{version}/{sha256}.zip"
    ))
}

pub fn is_valid_sha256_hex(value: &str) -> bool {
    value.len() == 64
        && value
            .chars()
            .all(|character| character.is_ascii_hexdigit() && !character.is_ascii_uppercase())
}

fn safe_slug(value: &str, label: &str) -> Result<String, PackageStorageError> {
    let value = value.trim();
    let valid = !value.is_empty()
        && value.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
        && !value.starts_with('-')
        && !value.ends_with('-')
        && !value.contains("--");

    if valid {
        Ok(value.to_owned())
    } else {
        Err(error(format!("{label} must be a safe Marketplace slug")))
    }
}

fn error(message: impl Into<String>) -> PackageStorageError {
    PackageStorageError {
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        MAX_MARKETPLACE_PACKAGE_BYTES, marketplace_package_object_key, sha256_hex,
        validate_package_size, verify_package_checksum,
    };

    #[test]
    fn sha256_checksum_matches_package_bytes() {
        let bytes = b"marketplace package";
        let checksum = sha256_hex(bytes);

        assert_eq!(checksum.len(), 64);
        assert!(verify_package_checksum(bytes, &checksum));
        assert!(!verify_package_checksum(b"tampered", &checksum));
    }

    #[test]
    fn object_key_uses_creator_listing_version_and_checksum() {
        let checksum = sha256_hex(b"package");
        let key = marketplace_package_object_key("zinhar", "hero-pack", "1.0.0", &checksum)
            .expect("object key should be valid");

        assert_eq!(
            key,
            format!("marketplace/packages/zinhar/hero-pack/1.0.0/{checksum}.zip")
        );
    }

    #[test]
    fn object_key_rejects_path_like_slugs_and_invalid_checksums() {
        let checksum = sha256_hex(b"package");
        assert!(
            marketplace_package_object_key("../zinhar", "hero-pack", "1.0.0", &checksum).is_err()
        );
        assert!(marketplace_package_object_key("zinhar", "hero-pack", "1.0.0", "ABC").is_err());
    }

    #[test]
    fn package_size_must_be_nonzero_and_within_limit() {
        assert!(validate_package_size(1).is_ok());
        assert!(validate_package_size(MAX_MARKETPLACE_PACKAGE_BYTES).is_ok());
        assert!(validate_package_size(0).is_err());
        assert!(validate_package_size(MAX_MARKETPLACE_PACKAGE_BYTES + 1).is_err());
    }
}
