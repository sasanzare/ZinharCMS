pub const V3_MARKETPLACE_PHASE_0_1_DOCS: &[&str] = &[
    "docs/V3_PHASE_0_1.md",
    "docs/V3_MARKETPLACE_SCOPE.md",
    "docs/V3_PRODUCT_TAXONOMY.md",
    "docs/V3_MARKETPLACE_POLICY.md",
];

pub const V3_MARKETPLACE_PRODUCT_TYPES: &[&str] = &[
    "Component Pack",
    "Design Template",
    "Integration Plugin",
    "Backend Extension",
    "Unsupported",
];

#[cfg(test)]
mod tests {
    use super::{V3_MARKETPLACE_PHASE_0_1_DOCS, V3_MARKETPLACE_PRODUCT_TYPES};

    const PHASE_0_1: &str = include_str!("../../../docs/V3_PHASE_0_1.md");
    const SCOPE: &str = include_str!("../../../docs/V3_MARKETPLACE_SCOPE.md");
    const TAXONOMY: &str = include_str!("../../../docs/V3_PRODUCT_TAXONOMY.md");
    const POLICY: &str = include_str!("../../../docs/V3_MARKETPLACE_POLICY.md");

    #[test]
    fn phase_0_1_required_docs_are_manifested() {
        for required_doc in [
            "docs/V3_PHASE_0_1.md",
            "docs/V3_MARKETPLACE_SCOPE.md",
            "docs/V3_PRODUCT_TAXONOMY.md",
            "docs/V3_MARKETPLACE_POLICY.md",
        ] {
            assert!(
                V3_MARKETPLACE_PHASE_0_1_DOCS.contains(&required_doc),
                "{required_doc} is missing from the V3 phase 0.1 docs manifest"
            );
        }
    }

    #[test]
    fn taxonomy_classifies_supported_and_unsupported_products() {
        for product_type in V3_MARKETPLACE_PRODUCT_TYPES {
            assert!(
                TAXONOMY.contains(product_type),
                "{product_type} is missing from the Marketplace taxonomy"
            );
            assert!(
                PHASE_0_1.contains(product_type),
                "{product_type} is missing from the phase acceptance criteria"
            );
        }
    }

    #[test]
    fn scope_locks_mvp_and_out_of_scope_boundaries() {
        for phrase in [
            "V3 MVP Scope",
            "Out Of Scope For Initial V3",
            "no arbitrary server-side code execution",
            "no unreviewed public listing",
            "no paid products until the free install path is stable",
        ] {
            assert!(
                SCOPE.contains(phrase),
                "Marketplace scope is missing required boundary: {phrase}"
            );
        }
    }

    #[test]
    fn policy_covers_review_decisions_and_rejection_rules() {
        for phrase in [
            "Approve",
            "Request changes",
            "Reject",
            "Quarantine",
            "Suspend",
            "Takedown",
            "Rejection Rules",
            "Moderation And Takedown",
        ] {
            assert!(
                POLICY.contains(phrase),
                "Marketplace policy is missing required decision or rule: {phrase}"
            );
        }
    }
}
