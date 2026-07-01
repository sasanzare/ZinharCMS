pub const V3_MARKETPLACE_PHASE_0_2_DOCS: &[&str] = &[
    "docs/V3_PHASE_0_2.md",
    "docs/V3_V2_MARKETPLACE_READINESS_AUDIT.md",
    "docs/V3_MARKETPLACE_V2_DEPENDENCY_MATRIX.md",
    "docs/V3_MARKETPLACE_GAP_LIST.md",
];

pub const V3_MARKETPLACE_AUDITED_V2_SURFACES: &[&str] =
    &["Organizations", "Billing", "RBAC", "Audit logs", "RLS"];

pub const V3_MARKETPLACE_REQUIRED_GAP_AREAS: &[&str] = &[
    "Plugin Install Gaps",
    "Creator Payment Gaps",
    "Permission Gaps",
];

#[cfg(test)]
mod tests {
    use super::{
        V3_MARKETPLACE_AUDITED_V2_SURFACES, V3_MARKETPLACE_PHASE_0_2_DOCS,
        V3_MARKETPLACE_REQUIRED_GAP_AREAS,
    };

    const PHASE_0_2: &str = include_str!("../../../docs/V3_PHASE_0_2.md");
    const READINESS_AUDIT: &str =
        include_str!("../../../docs/V3_V2_MARKETPLACE_READINESS_AUDIT.md");
    const DEPENDENCY_MATRIX: &str =
        include_str!("../../../docs/V3_MARKETPLACE_V2_DEPENDENCY_MATRIX.md");
    const GAP_LIST: &str = include_str!("../../../docs/V3_MARKETPLACE_GAP_LIST.md");

    #[test]
    fn phase_0_2_required_docs_are_manifested() {
        for required_doc in [
            "docs/V3_PHASE_0_2.md",
            "docs/V3_V2_MARKETPLACE_READINESS_AUDIT.md",
            "docs/V3_MARKETPLACE_V2_DEPENDENCY_MATRIX.md",
            "docs/V3_MARKETPLACE_GAP_LIST.md",
        ] {
            assert!(
                V3_MARKETPLACE_PHASE_0_2_DOCS.contains(&required_doc),
                "{required_doc} is missing from the V3 phase 0.2 docs manifest"
            );
        }
    }

    #[test]
    fn readiness_audit_covers_required_v2_surfaces() {
        for surface in V3_MARKETPLACE_AUDITED_V2_SURFACES {
            assert!(
                READINESS_AUDIT.contains(surface),
                "readiness audit is missing V2 surface: {surface}"
            );
            assert!(
                PHASE_0_2.contains(surface) || PHASE_0_2.contains(&surface.to_lowercase()),
                "phase summary is missing V2 surface: {surface}"
            );
        }
    }

    #[test]
    fn gap_list_covers_install_payment_and_permission_outputs() {
        for area in V3_MARKETPLACE_REQUIRED_GAP_AREAS {
            assert!(GAP_LIST.contains(area), "gap list is missing {area}");
        }

        for phrase in [
            "plugin install",
            "creator payment",
            "permissions",
            "marketplace_installations",
            "purchase records",
            "permission snapshot",
        ] {
            assert!(
                GAP_LIST.to_lowercase().contains(phrase),
                "gap list is missing required phrase: {phrase}"
            );
        }
    }

    #[test]
    fn dependency_matrix_resolves_critical_ambiguity() {
        assert!(DEPENDENCY_MATRIX.contains("Fixed Decisions For Phase 1.1"));
        assert!(PHASE_0_2.contains("No critical V2 dependency remains ambiguous"));
        assert!(READINESS_AUDIT.contains("No critical V2 dependency remains ambiguous"));
        assert!(GAP_LIST.contains("No critical V2 dependency remains ambiguous"));
    }
}
