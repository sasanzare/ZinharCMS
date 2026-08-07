use std::fs::OpenOptions;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use image::{ImageFormat, ImageReader, Limits};
use tokio::task;
use uuid::Uuid;

use crate::error::AppError;
use crate::services::file_security::{
    MAX_IMAGE_DECODE_ALLOC, MAX_IMAGE_HEIGHT, MAX_IMAGE_PIXELS, MAX_IMAGE_WIDTH,
    secure_join_no_symlinks, sha256_file,
};

const MAX_ENCODED_IMAGE_BYTES: u64 = 64 * 1_048_576;

#[derive(Debug)]
pub struct ProcessedImageFile {
    pub name: String,
    pub path: PathBuf,
    pub storage_key: String,
    pub url: String,
    pub sha256: String,
    pub size: u64,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug)]
pub struct ProcessedImageSet {
    pub original: ProcessedImageFile,
    pub variants: Vec<ProcessedImageFile>,
}

struct ImageVariantSpec {
    name: &'static str,
    max_width: u32,
    max_height: u32,
}

const VARIANTS: &[ImageVariantSpec] = &[
    ImageVariantSpec {
        name: "thumbnail",
        max_width: 150,
        max_height: 150,
    },
    ImageVariantSpec {
        name: "small",
        max_width: 400,
        max_height: 400,
    },
    ImageVariantSpec {
        name: "medium",
        max_width: 800,
        max_height: 800,
    },
    ImageVariantSpec {
        name: "large",
        max_width: 1920,
        max_height: 1080,
    },
];

pub fn is_supported_image_mime(mime_type: &str) -> bool {
    matches!(mime_type, "image/jpeg" | "image/png" | "image/webp")
}

pub async fn process_image_upload(
    source: &Path,
    storage_root: &Path,
    organization_id: Uuid,
    media_id: Uuid,
) -> Result<ProcessedImageSet, AppError> {
    let source = source.to_owned();
    let processing_key = format!("quarantine/processed/{media_id}");
    let processing_dir = secure_join_no_symlinks(storage_root, &processing_key)
        .map_err(|error| AppError::Internal(error.to_string()))?;
    tokio::fs::create_dir_all(&processing_dir)
        .await
        .map_err(|error| AppError::Internal(error.to_string()))?;
    secure_join_no_symlinks(storage_root, &processing_key)
        .map_err(|error| AppError::Internal(error.to_string()))?;
    let processing_dir_for_task = processing_dir.clone();
    let result = task::spawn_blocking(move || {
        process_image_blocking(&source, &processing_dir_for_task, organization_id, media_id)
    })
    .await
    .map_err(|error| AppError::Internal(error.to_string()))?;
    let result = match result {
        Ok(result) => result,
        Err(error) => {
            let _ = tokio::fs::remove_dir_all(&processing_dir).await;
            return Err(error);
        }
    };

    let mut original = result.0;
    let mut variants = result.1;
    let (checksum, size) = sha256_file(&original.path, MAX_ENCODED_IMAGE_BYTES)
        .await
        .map_err(|error| AppError::Internal(error.to_string()))?;
    original.sha256 = checksum;
    original.size = size;
    for variant in &mut variants {
        let (checksum, size) = sha256_file(&variant.path, MAX_ENCODED_IMAGE_BYTES)
            .await
            .map_err(|error| AppError::Internal(error.to_string()))?;
        variant.sha256 = checksum;
        variant.size = size;
    }
    Ok(ProcessedImageSet { original, variants })
}

fn process_image_blocking(
    source: &Path,
    processing_dir: &Path,
    organization_id: Uuid,
    media_id: Uuid,
) -> Result<(ProcessedImageFile, Vec<ProcessedImageFile>), AppError> {
    std::fs::create_dir_all(processing_dir)
        .map_err(|error| AppError::Internal(error.to_string()))?;

    let mut reader = ImageReader::open(source)
        .map_err(|error| AppError::BadRequest(format!("invalid image: {error}")))?
        .with_guessed_format()
        .map_err(|error| AppError::BadRequest(format!("invalid image: {error}")))?;
    let mut limits = Limits::default();
    limits.max_image_width = Some(MAX_IMAGE_WIDTH);
    limits.max_image_height = Some(MAX_IMAGE_HEIGHT);
    limits.max_alloc = Some(MAX_IMAGE_DECODE_ALLOC);
    reader.limits(limits);
    let image = reader
        .decode()
        .map_err(|error| AppError::BadRequest(format!("invalid or oversized image: {error}")))?;
    let pixels = u64::from(image.width())
        .checked_mul(u64::from(image.height()))
        .ok_or_else(|| AppError::Validation("image dimensions overflowed".to_owned()))?;
    if pixels > MAX_IMAGE_PIXELS {
        return Err(AppError::Validation(format!(
            "image exceeds the {MAX_IMAGE_PIXELS} pixel processing limit"
        )));
    }

    let namespace = format!("public/media/{organization_id}/{media_id}");
    let url_prefix = format!("/uploads/{namespace}");
    let original_path = processing_dir.join("original.webp");
    write_webp_create_new(&image, &original_path)?;
    let original = ProcessedImageFile {
        name: "original".to_owned(),
        path: original_path,
        storage_key: format!("{namespace}/original.webp"),
        url: format!("{url_prefix}/original.webp"),
        sha256: String::new(),
        size: 0,
        width: image.width() as i32,
        height: image.height() as i32,
    };

    let mut variants = Vec::with_capacity(VARIANTS.len());
    for spec in VARIANTS {
        let resized = image.thumbnail(spec.max_width, spec.max_height);
        let filename = format!("{}.webp", spec.name);
        let path = processing_dir.join(&filename);
        write_webp_create_new(&resized, &path)?;
        variants.push(ProcessedImageFile {
            name: spec.name.to_owned(),
            path,
            storage_key: format!("{namespace}/variants/{filename}"),
            url: format!("{url_prefix}/variants/{filename}"),
            sha256: String::new(),
            size: 0,
            width: resized.width() as i32,
            height: resized.height() as i32,
        });
    }
    Ok((original, variants))
}

fn write_webp_create_new(image: &image::DynamicImage, path: &Path) -> Result<(), AppError> {
    let file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .map_err(|error| AppError::Internal(error.to_string()))?;
    let mut writer = BufWriter::new(file);
    image
        .write_to(&mut writer, ImageFormat::WebP)
        .map_err(|error| AppError::Internal(error.to_string()))
}

pub async fn remove_processing_directory(path: &Path) {
    if let Some(parent) = path.parent() {
        let _ = tokio::fs::remove_dir_all(parent).await;
    }
}
