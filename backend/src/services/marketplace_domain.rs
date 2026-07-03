pub const V3_MARKETPLACE_PHASE_ONE_DOCS: &[&str] = &[
    "docs/V3_PHASE_ONE.md",
    "docs/V3_MARKETPLACE_DOMAIN_MODEL.md",
    "docs/V3_MARKETPLACE_MANIFEST_SCHEMA.md",
    "docs/V3_PACKAGE_STORAGE.md",
];

pub const V3_MARKETPLACE_DOMAIN_ENTITIES: &[&str] = &[
    "Creator",
    "Listing",
    "Package",
    "Version",
    "Submission",
    "Installation",
    "Purchase",
];

pub const V3_MARKETPLACE_PHASE_ONE_TABLES: &[&str] = &[
    "marketplace_creators",
    "marketplace_listings",
    "marketplace_versions",
    "marketplace_submissions",
    "marketplace_installations",
];

#[cfg(test)]
mod tests {
    use super::{
        V3_MARKETPLACE_DOMAIN_ENTITIES, V3_MARKETPLACE_PHASE_ONE_DOCS,
        V3_MARKETPLACE_PHASE_ONE_TABLES,
    };

    const PHASE_ONE_DOC: &str = include_str!("../../../docs/V3_PHASE_ONE.md");
    const DOMAIN_MODEL_DOC: &str = include_str!("../../../docs/V3_MARKETPLACE_DOMAIN_MODEL.md");
    const MANIFEST_SCHEMA_DOC: &str =
        include_str!("../../../docs/V3_MARKETPLACE_MANIFEST_SCHEMA.md");
    const PACKAGE_STORAGE_DOC: &str = include_str!("../../../docs/V3_PACKAGE_STORAGE.md");
    const PHASE_ONE_MIGRATION: &str =
        include_str!("../../migrations/0015_v3_phase_one_marketplace_foundation.sql");

    #[test]
    fn phase_one_docs_are_manifested() {
        for required_doc in [
            "docs/V3_PHASE_ONE.md",
            "docs/V3_MARKETPLACE_DOMAIN_MODEL.md",
            "docs/V3_MARKETPLACE_MANIFEST_SCHEMA.md",
            "docs/V3_PACKAGE_STORAGE.md",
        ] {
            assert!(
                V3_MARKETPLACE_PHASE_ONE_DOCS.contains(&required_doc),
                "{required_doc} is missing from the V3 phase one docs manifest"
            );
        }
    }

    #[test]
    fn domain_model_covers_required_entities() {
        for entity in V3_MARKETPLACE_DOMAIN_ENTITIES {
            assert!(
                DOMAIN_MODEL_DOC.contains(entity),
                "domain model is missing {entity}"
            );
            assert!(
                PHASE_ONE_DOC.contains(entity),
                "phase one summary is missing {entity}"
            );
        }

        assert!(DOMAIN_MODEL_DOC.contains("organization_id"));
        assert!(DOMAIN_MODEL_DOC.contains("creator_id"));
        assert!(DOMAIN_MODEL_DOC.contains("Paid purchase"));
    }

    #[test]
    fn migration_creates_base_marketplace_tables() {
        for table in V3_MARKETPLACE_PHASE_ONE_TABLES {
            assert!(
                PHASE_ONE_MIGRATION.contains(&format!("CREATE TABLE IF NOT EXISTS {table}")),
                "phase one migration is missing table {table}"
            );
        }

        assert!(PHASE_ONE_MIGRATION.contains("marketplace_installations_listing_version_fk"));
        assert!(PHASE_ONE_MIGRATION.contains("idx_marketplace_installations_active_listing"));
        assert!(!PHASE_ONE_MIGRATION.contains("DROP TABLE"));
        assert!(!PHASE_ONE_MIGRATION.contains("DROP COLUMN"));
    }

    #[test]
    fn manifest_and_package_storage_contracts_are_enforced_by_migration() {
        for field in [
            "manifest_json ? 'manifest_version'",
            "manifest_json ? 'name'",
            "manifest_json ? 'version'",
            "manifest_json ? 'type'",
            "manifest_json ? 'permissions'",
            "manifest_json ? 'compatibility'",
            "manifest_json ? 'entry_points'",
            "manifest_json ? 'assets'",
            "artifact_object_key",
            "artifact_sha256",
            "artifact_size_bytes",
            "storage_metadata",
        ] {
            assert!(
                PHASE_ONE_MIGRATION.contains(field),
                "phase one migration is missing manifest/storage field: {field}"
            );
        }

        assert!(
            PHASE_ONE_MIGRATION.contains("marketplace_versions_manifest_schema_version_supported")
        );
        assert!(
            PHASE_ONE_MIGRATION
                .contains("manifest_json->>'manifest_version' = manifest_schema_version")
        );
        assert!(MANIFEST_SCHEMA_DOC.contains("Required Fields"));
        assert!(PACKAGE_STORAGE_DOC.contains("SHA-256"));
        assert!(PACKAGE_STORAGE_DOC.contains("52428800 bytes"));
        assert!(PHASE_ONE_MIGRATION.contains("marketplace_prevent_version_artifact_mutation"));
    }

    #[test]
    fn tenant_owned_installations_have_forced_rls() {
        assert!(
            PHASE_ONE_MIGRATION
                .contains("ALTER TABLE marketplace_installations FORCE ROW LEVEL SECURITY")
        );
        assert!(PHASE_ONE_MIGRATION.contains("app_rls_tenant_matches(organization_id)"));
    }
}
