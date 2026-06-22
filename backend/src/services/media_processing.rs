use std::path::{Path, PathBuf};

use image::ImageFormat;
use tokio::task;

use crate::error::AppError;

pub struct ProcessedVariant {
    pub name: String,
    pub path: PathBuf,
    pub url: String,
    pub width: i32,
    pub height: i32,
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

pub async fn process_image_variants(
    bytes: Vec<u8>,
    upload_dir: &Path,
    url_prefix: &str,
    file_id: &str,
) -> Result<Vec<ProcessedVariant>, AppError> {
    let upload_dir = upload_dir.to_owned();
    let url_prefix = url_prefix.trim_end_matches('/').to_owned();
    let file_id = file_id.to_owned();

    task::spawn_blocking(move || {
        process_variants_blocking(bytes, &upload_dir, &url_prefix, &file_id)
    })
    .await
    .map_err(|error| AppError::Internal(error.to_string()))?
}

fn process_variants_blocking(
    bytes: Vec<u8>,
    upload_dir: &Path,
    url_prefix: &str,
    file_id: &str,
) -> Result<Vec<ProcessedVariant>, AppError> {
    let image = image::load_from_memory(&bytes)
        .map_err(|error| AppError::BadRequest(format!("invalid image: {error}")))?;
    let variant_dir = upload_dir.join("variants");
    std::fs::create_dir_all(&variant_dir).map_err(|error| AppError::Internal(error.to_string()))?;

    let mut processed = Vec::with_capacity(VARIANTS.len());
    for variant in VARIANTS {
        let resized = image.thumbnail(variant.max_width, variant.max_height);
        let filename = format!("{file_id}-{}.webp", variant.name);
        let path = variant_dir.join(&filename);
        resized
            .save_with_format(&path, ImageFormat::WebP)
            .map_err(|error| AppError::Internal(error.to_string()))?;

        processed.push(ProcessedVariant {
            name: variant.name.to_owned(),
            url: format!("{url_prefix}/variants/{filename}"),
            path,
            width: resized.width() as i32,
            height: resized.height() as i32,
        });
    }

    Ok(processed)
}
