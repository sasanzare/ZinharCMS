use crate::services::marketplace_manifest::SUPPORTED_MARKETPLACE_PRODUCT_TYPES;

pub const CREATOR_STATUS_PENDING: &str = "pending";
pub const CREATOR_STATUS_APPROVED: &str = "approved";
pub const CREATOR_STATUS_SUSPENDED: &str = "suspended";
pub const CREATOR_STATUS_REJECTED: &str = "rejected";

pub const LISTING_STATUS_DRAFT: &str = "draft";
pub const LISTING_STATUS_SUBMITTED: &str = "submitted";

const MAX_TEXT_LENGTH: usize = 8_000;
const MAX_SCREENSHOTS: usize = 8;
const MAX_PRICE_CENTS: i32 = 10_000_000;

#[derive(Debug, Clone)]
pub struct ListingReviewInput<'a> {
    pub product_type: &'a str,
    pub title: &'a str,
    pub slug: &'a str,
    pub summary: &'a str,
    pub description: &'a str,
    pub category: &'a str,
    pub pricing_type: &'a str,
    pub price_cents: i32,
    pub license: &'a str,
    pub support_url: Option<&'a str>,
    pub screenshots: &'a [String],
}

pub fn validate_creator_profile(
    slug: &str,
    display_name: &str,
    bio: Option<&str>,
    support_email: Option<&str>,
) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    validate_slug(slug, "creator slug", &mut errors);
    validate_required(display_name, "display name", 2, 120, &mut errors);

    if let Some(bio) = bio {
        validate_optional_text(bio, "bio", MAX_TEXT_LENGTH, &mut errors);
    }

    if let Some(email) = support_email
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        if !email.contains('@') || email.contains(char::is_whitespace) || email.len() > 254 {
            errors.push("support email must be a valid email address".to_owned());
        }
    }

    finish(errors)
}

pub fn validate_creator_verification_status(status: &str) -> bool {
    matches!(
        status,
        CREATOR_STATUS_PENDING
            | CREATOR_STATUS_APPROVED
            | CREATOR_STATUS_SUSPENDED
            | CREATOR_STATUS_REJECTED
    )
}

pub fn creator_can_submit_public_products(status: &str) -> bool {
    status == CREATOR_STATUS_APPROVED
}

pub fn validate_listing_review_input(input: &ListingReviewInput<'_>) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    if !SUPPORTED_MARKETPLACE_PRODUCT_TYPES.contains(&input.product_type) {
        errors.push("product type is not supported".to_owned());
    }
    validate_required(input.title, "title", 3, 160, &mut errors);
    validate_slug(input.slug, "listing slug", &mut errors);
    validate_required(input.summary, "summary", 12, 280, &mut errors);
    validate_required(
        input.description,
        "description",
        40,
        MAX_TEXT_LENGTH,
        &mut errors,
    );
    validate_required(input.category, "category", 2, 80, &mut errors);
    validate_pricing(input.pricing_type, input.price_cents, &mut errors);
    validate_required(input.license, "license", 2, 120, &mut errors);

    if let Some(url) = input
        .support_url
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        validate_http_url(url, "support URL", &mut errors);
    }

    if input.screenshots.is_empty() {
        errors.push("at least one screenshot URL is required".to_owned());
    }
    if input.screenshots.len() > MAX_SCREENSHOTS {
        errors.push(format!("screenshots cannot exceed {MAX_SCREENSHOTS} items"));
    }
    for screenshot in input.screenshots {
        validate_http_url(screenshot, "screenshot URL", &mut errors);
    }

    finish(errors)
}

pub fn validate_listing_for_review(
    creator_status: &str,
    input: &ListingReviewInput<'_>,
) -> Result<(), Vec<String>> {
    let mut errors = validate_listing_review_input(input)
        .err()
        .unwrap_or_default();
    if !creator_can_submit_public_products(creator_status) {
        errors.push("creator must be approved before submitting public products".to_owned());
    }
    finish(errors)
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let value = value.trim().to_owned();
        (!value.is_empty()).then_some(value)
    })
}

pub fn sanitize_screenshot_urls(values: &[String]) -> Vec<String> {
    values
        .iter()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .collect()
}

fn validate_pricing(pricing_type: &str, price_cents: i32, errors: &mut Vec<String>) {
    match pricing_type {
        "free" | "custom" => {
            if price_cents < 0 {
                errors.push("price cannot be negative".to_owned());
            }
        }
        "paid" => {
            if price_cents <= 0 {
                errors.push("paid listings require a positive price".to_owned());
            }
        }
        _ => errors.push("pricing type is not supported".to_owned()),
    }

    if price_cents > MAX_PRICE_CENTS {
        errors.push("price is above the Marketplace limit".to_owned());
    }
}

fn validate_required(
    value: &str,
    label: &str,
    min_length: usize,
    max_length: usize,
    errors: &mut Vec<String>,
) {
    let value = value.trim();
    if value.len() < min_length {
        errors.push(format!("{label} is too short"));
    }
    if value.len() > max_length {
        errors.push(format!("{label} is too long"));
    }
}

fn validate_optional_text(value: &str, label: &str, max_length: usize, errors: &mut Vec<String>) {
    if value.trim().len() > max_length {
        errors.push(format!("{label} is too long"));
    }
}

fn validate_slug(value: &str, label: &str, errors: &mut Vec<String>) {
    let value = value.trim();
    let valid = !value.is_empty()
        && value.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
        && !value.starts_with('-')
        && !value.ends_with('-')
        && !value.contains("--");

    if !valid {
        errors.push(format!(
            "{label} must use lowercase letters, digits, and single hyphens"
        ));
    }
}

fn validate_http_url(value: &str, label: &str, errors: &mut Vec<String>) {
    let value = value.trim();
    let valid = (value.starts_with("https://") || value.starts_with("http://"))
        && value.len() <= 2_048
        && !value.contains(char::is_whitespace);
    if !valid {
        errors.push(format!("{label} must be an http or https URL"));
    }
}

fn finish(errors: Vec<String>) -> Result<(), Vec<String>> {
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ListingReviewInput, creator_can_submit_public_products, validate_creator_profile,
        validate_creator_verification_status, validate_listing_for_review,
    };

    fn valid_listing<'a>(screenshots: &'a [String]) -> ListingReviewInput<'a> {
        ListingReviewInput {
            product_type: "component_pack",
            title: "SaaS Hero Components",
            slug: "saas-hero-components",
            summary: "Reusable hero sections for SaaS landing pages.",
            description: "A complete package of reviewed hero blocks, supporting assets, and import metadata for ZinharCMS pages.",
            category: "marketing",
            pricing_type: "paid",
            price_cents: 2900,
            license: "commercial",
            support_url: Some("https://example.com/support"),
            screenshots,
        }
    }

    #[test]
    fn creator_profile_requires_safe_public_identity() {
        assert!(
            validate_creator_profile(
                "zinhar-labs",
                "Zinhar Labs",
                Some("CMS products"),
                Some("support@example.com")
            )
            .is_ok()
        );
        assert!(validate_creator_profile("Bad Slug", "Z", None, Some("invalid")).is_err());
    }

    #[test]
    fn only_approved_creator_can_submit_public_products() {
        assert!(validate_creator_verification_status("approved"));
        assert!(!validate_creator_verification_status("verified"));
        assert!(creator_can_submit_public_products("approved"));
        assert!(!creator_can_submit_public_products("pending"));
    }

    #[test]
    fn listing_requires_complete_review_metadata() {
        let screenshots = vec!["https://example.com/preview.png".to_owned()];
        assert!(validate_listing_for_review("approved", &valid_listing(&screenshots)).is_ok());

        let empty_screenshots = Vec::new();
        let incomplete = ListingReviewInput {
            title: "No",
            description: "short",
            price_cents: 0,
            screenshots: &empty_screenshots,
            ..valid_listing(&empty_screenshots)
        };
        let errors =
            validate_listing_for_review("pending", &incomplete).expect_err("listing should fail");
        assert!(
            errors
                .iter()
                .any(|message| message.contains("creator must be approved"))
        );
        assert!(errors.iter().any(|message| message.contains("screenshot")));
        assert!(
            errors
                .iter()
                .any(|message| message.contains("positive price"))
        );
    }
    #[test]
    fn phase_two_docs_cover_creator_submission_acceptance() {
        const PHASE_TWO_DOC: &str = include_str!("../../../docs/V3_PHASE_TWO.md");

        for required in [
            "Creator profile request",
            "pending, approved, suspended, and rejected",
            "Listing submission metadata",
            "Package version upload endpoint",
            "Incomplete listing metadata cannot enter review",
            "immutable version connected to a submission row",
        ] {
            assert!(
                PHASE_TWO_DOC.contains(required),
                "phase two doc is missing {required}"
            );
        }
    }

    #[test]
    fn phase_two_migration_adds_submission_workflow_contract() {
        const PHASE_TWO_MIGRATION: &str =
            include_str!("../../migrations/0016_v3_phase_two_creator_submission.sql");

        for required in [
            "verification_notes",
            "status IN ('pending', 'approved', 'suspended', 'rejected')",
            "description TEXT NOT NULL DEFAULT ''",
            "price_cents INTEGER NOT NULL DEFAULT 0",
            "screenshots JSONB NOT NULL DEFAULT '[]'::jsonb",
            "idx_marketplace_listings_submission_queue",
            "OLD.status IN ('submitted', 'validating', 'approved', 'deprecated', 'blocked')",
        ] {
            assert!(
                PHASE_TWO_MIGRATION.contains(required),
                "phase two migration is missing {required}"
            );
        }
    }
}
