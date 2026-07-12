#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::services::marketplace_runtime::{
        RUNTIME_STATUS_BLOCKED, authorize_runtime_operation,
    };
    use crate::services::marketplace_validation::evaluate_marketplace_package;

    const MARKETPLACE_ROUTES: &str = include_str!("../routes/marketplace.rs");
    const MARKETPLACE_FINANCE_ROUTES: &str = include_str!("../routes/marketplace_finance.rs");
    const STRIPE_BILLING_SERVICE: &str = include_str!("stripe_billing.rs");
    const REVIEW_ABUSE_MIGRATION: &str =
        include_str!("../../migrations/0024_v3_phase_ten_ratings_abuse.sql");
    const FINANCE_HARDENING_MIGRATION: &str =
        include_str!("../../migrations/0023_v3_phase_nine_finance_hardening.sql");

    #[test]
    fn phase_thirteen_idor_review_and_refund_abuse_contracts_are_guarded() {
        for required in [
            "WHERE id = $1 AND creator_id = $2",
            "WHERE listing.id = $1 AND creator.user_id = $2",
            "load_listing_for_submission(&state, claims.sub, listing_id)",
            "WHERE listing.id = $1 AND listing.creator_id = $2",
        ] {
            assert!(
                MARKETPLACE_ROUTES.contains(required),
                "missing creator/listing IDOR guard: {required}"
            );
        }

        for required in [
            "WHERE organization_id = $1 AND listing_id = $2 AND status <> 'uninstalled'",
            "WHERE organization_id = $1 AND listing_id = $2 AND status = 'completed'",
            "ON CONFLICT (organization_id, listing_id) DO UPDATE",
            "review version does not belong to the listing",
            "report version does not belong to the listing",
            "report evidence must be a JSON object",
            "marketplace.abuse_report.critical_notification",
        ] {
            assert!(
                MARKETPLACE_ROUTES.contains(required),
                "missing review/report abuse guard: {required}"
            );
        }

        for required in [
            "idx_marketplace_product_reviews_org_listing",
            "idx_marketplace_abuse_reports_queue",
            "FORCE ROW LEVEL SECURITY",
            "marketplace_abuse_reports_evidence_object",
        ] {
            assert!(
                REVIEW_ABUSE_MIGRATION.contains(required),
                "missing review/report migration guard: {required}"
            );
        }

        for required in [
            "ensure_creator_owner(db.as_mut(), creator_id, tenant.user_id)",
            "ensure_creator_owner(&mut *tx, creator_id, claims.sub)",
        ] {
            assert!(
                MARKETPLACE_FINANCE_ROUTES.contains(required),
                "missing creator finance ownership guard: {required}"
            );
        }

        for required in [
            "apply_marketplace_refund",
            "WHERE id = $1 FOR UPDATE",
            "if row.6 == \"refunded\"",
            "refunded_cents <= 0 || refunded_cents > total_cents as i64",
            "UPDATE marketplace_entitlements SET status = 'revoked'",
            "ON CONFLICT DO NOTHING",
        ] {
            assert!(
                STRIPE_BILLING_SERVICE.contains(required),
                "missing refund abuse guard: {required}"
            );
        }

        for required in [
            "idx_marketplace_ledger_provider_event",
            "WHERE provider_event_id IS NOT NULL",
            "marketplace_revenue_ledger_append_only",
            "BEFORE UPDATE OR DELETE ON marketplace_revenue_ledger",
        ] {
            assert!(
                FINANCE_HARDENING_MIGRATION.contains(required),
                "missing refund idempotency/append-only guard: {required}"
            );
        }
    }

    #[test]
    fn phase_thirteen_permission_bypass_attempts_are_denied() {
        let manifest = json!({
            "entry_points": { "components": "components/index.json" }
        });

        let permission_error = authorize_runtime_operation(
            "active",
            "ready",
            "component_pack",
            &manifest,
            &json!([]),
            "component.render",
            "components",
            &json!({}),
        )
        .expect_err("runtime must deny operations without approved permission");
        assert_eq!(permission_error.code, "permission_not_approved");

        let unknown_operation_error = authorize_runtime_operation(
            "active",
            "ready",
            "component_pack",
            &manifest,
            &json!(["page.read"]),
            "database.root",
            "components",
            &json!({}),
        )
        .expect_err("runtime must deny operations outside the allowlist");
        assert_eq!(unknown_operation_error.code, "operation_not_allowlisted");

        let kill_switch_error = authorize_runtime_operation(
            "active",
            RUNTIME_STATUS_BLOCKED,
            "component_pack",
            &manifest,
            &json!(["page.read"]),
            "component.render",
            "components",
            &json!({}),
        )
        .expect_err("runtime must deny blocked installations");
        assert_eq!(kill_switch_error.code, "runtime_blocked");
    }

    #[test]
    fn phase_thirteen_malicious_package_is_blocked_before_review() {
        let manifest = json!({
            "manifest_version": "2026-07",
            "name": "Security Probe Pack",
            "version": "1.0.0",
            "type": "component_pack",
            "permissions": ["page.read", "external_network.request"],
            "compatibility": {
                "min_zinhar_version": "2.0.0",
                "max_zinhar_version": "3.5.0",
                "required_features": ["component_packs"],
                "required_plan": "free"
            },
            "entry_points": { "components": "components/index.json" },
            "assets": ["assets/preview.png"]
        });
        let bytes = fake_zip(&[
            ("components/index.json", b"{}"),
            ("assets/preview.png", b"png"),
            (".env", b"SECRET=1"),
        ]);

        let decision = evaluate_marketplace_package(
            &manifest,
            &bytes,
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "phase13-malicious.zip",
            "component_pack",
            "free",
        );

        assert_eq!(decision.security_risk_level, "critical");
        assert_eq!(decision.version_status, "blocked");
        assert_eq!(decision.submission_review_status, "blocked");
    }

    fn fake_zip(entries: &[(&str, &[u8])]) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut central = Vec::new();
        let mut local_offset = 0u32;

        for (path, content) in entries {
            let path_bytes = path.as_bytes();
            bytes.extend_from_slice(&0x0403_4b50u32.to_le_bytes());
            bytes.extend_from_slice(&20u16.to_le_bytes());
            bytes.extend_from_slice(&0u16.to_le_bytes());
            bytes.extend_from_slice(&0u16.to_le_bytes());
            bytes.extend_from_slice(&0u16.to_le_bytes());
            bytes.extend_from_slice(&0u16.to_le_bytes());
            bytes.extend_from_slice(&0u32.to_le_bytes());
            bytes.extend_from_slice(&(content.len() as u32).to_le_bytes());
            bytes.extend_from_slice(&(content.len() as u32).to_le_bytes());
            bytes.extend_from_slice(&(path_bytes.len() as u16).to_le_bytes());
            bytes.extend_from_slice(&0u16.to_le_bytes());
            bytes.extend_from_slice(path_bytes);
            bytes.extend_from_slice(content);

            central.extend_from_slice(&0x0201_4b50u32.to_le_bytes());
            central.extend_from_slice(&20u16.to_le_bytes());
            central.extend_from_slice(&20u16.to_le_bytes());
            central.extend_from_slice(&0u16.to_le_bytes());
            central.extend_from_slice(&0u16.to_le_bytes());
            central.extend_from_slice(&0u16.to_le_bytes());
            central.extend_from_slice(&0u16.to_le_bytes());
            central.extend_from_slice(&0u32.to_le_bytes());
            central.extend_from_slice(&(content.len() as u32).to_le_bytes());
            central.extend_from_slice(&(content.len() as u32).to_le_bytes());
            central.extend_from_slice(&(path_bytes.len() as u16).to_le_bytes());
            central.extend_from_slice(&0u16.to_le_bytes());
            central.extend_from_slice(&0u16.to_le_bytes());
            central.extend_from_slice(&0u16.to_le_bytes());
            central.extend_from_slice(&0u16.to_le_bytes());
            central.extend_from_slice(&0u32.to_le_bytes());
            central.extend_from_slice(&local_offset.to_le_bytes());
            central.extend_from_slice(path_bytes);

            local_offset = bytes.len() as u32;
        }

        let central_offset = bytes.len() as u32;
        let central_size = central.len() as u32;
        bytes.extend_from_slice(&central);
        bytes.extend_from_slice(&0x0605_4b50u32.to_le_bytes());
        bytes.extend_from_slice(&0u16.to_le_bytes());
        bytes.extend_from_slice(&0u16.to_le_bytes());
        bytes.extend_from_slice(&(entries.len() as u16).to_le_bytes());
        bytes.extend_from_slice(&(entries.len() as u16).to_le_bytes());
        bytes.extend_from_slice(&central_size.to_le_bytes());
        bytes.extend_from_slice(&central_offset.to_le_bytes());
        bytes.extend_from_slice(&0u16.to_le_bytes());
        bytes
    }
}
