use serde_json::Value;

pub const DEFAULT_COMMISSION_BPS: i32 = 2_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevenueSplit {
    pub platform_fee_cents: i32,
    pub creator_share_cents: i32,
}

pub fn calculate_revenue_split(
    gross_cents: i32,
    tax_cents: i32,
    commission_bps: i32,
) -> Result<RevenueSplit, String> {
    if gross_cents < 0 || tax_cents < 0 || tax_cents > gross_cents {
        return Err("gross and tax amounts are invalid".to_owned());
    }
    if !(0..=10_000).contains(&commission_bps) {
        return Err("commission_bps must be between 0 and 10000".to_owned());
    }
    let platform_fee_cents =
        ((gross_cents - tax_cents) as i64 * commission_bps as i64 / 10_000) as i32;
    let creator_share_cents = gross_cents - tax_cents - platform_fee_cents;
    Ok(RevenueSplit {
        platform_fee_cents,
        creator_share_cents,
    })
}

pub fn validate_purchase_amount(price_cents: i32, pricing_type: &str) -> Result<(), String> {
    if pricing_type == "paid" && price_cents <= 0 {
        return Err("paid Marketplace products must have a positive price".to_owned());
    }
    if pricing_type == "free" && price_cents != 0 {
        return Err("free Marketplace products must have a zero price".to_owned());
    }
    if pricing_type == "custom" {
        return Err("custom Marketplace pricing is not supported in Phase 9".to_owned());
    }
    Ok(())
}

pub fn payout_eligibility(
    creator_status: &str,
    payout_status: &str,
    payouts_enabled: bool,
) -> Result<(), String> {
    if creator_status != "approved" {
        return Err("creator must be approved before payout".to_owned());
    }
    if payout_status != "verified" || !payouts_enabled {
        return Err("creator payout account is not verified and enabled".to_owned());
    }
    Ok(())
}

pub fn tax_metadata(currency: &str, tax_cents: i32) -> Value {
    serde_json::json!({ "provider": "host_declared", "currency": currency, "tax_cents": tax_cents })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_purchase_after_tax() {
        let split = calculate_revenue_split(10_000, 1_000, 2_000).unwrap();
        assert_eq!(split.platform_fee_cents, 1_800);
        assert_eq!(split.creator_share_cents, 7_200);
    }

    #[test]
    fn rejects_invalid_pricing_and_unverified_payouts() {
        assert!(validate_purchase_amount(0, "paid").is_err());
        assert!(validate_purchase_amount(100, "free").is_err());
        assert!(payout_eligibility("pending", "verified", true).is_err());
        assert!(payout_eligibility("approved", "verified", true).is_ok());
    }

    #[test]
    fn phase_nine_contract_is_migrated_documented_and_routed() {
        let migration = include_str!("../../migrations/0022_v3_phase_nine_marketplace_finance.sql");
        let routes = include_str!("../routes/marketplace_finance.rs");
        let install_routes = include_str!("../routes/marketplace.rs");
        let stripe = include_str!("stripe_billing.rs");
        let docs = include_str!("../../../docs/V3_PHASE_NINE.md");
        let diagram = include_str!("../../../docs/diagrams/36-marketplace-finance-lifecycle.mmd");

        for table in [
            "marketplace_purchases",
            "marketplace_entitlements",
            "marketplace_revenue_ledger",
            "marketplace_payout_accounts",
            "marketplace_payouts",
        ] {
            assert!(migration.contains(table));
        }
        assert!(migration.contains("FORCE ROW LEVEL SECURITY"));
        assert!(routes.contains("/api/marketplace/purchases/checkout"));
        assert!(routes.contains("/api/marketplace/revenue-ledger"));
        assert!(routes.contains("/payout/verify"));
        assert!(install_routes.contains("ensure_paid_entitlement"));
        assert!(stripe.contains("marketplace_purchase_id"));
        assert!(docs.contains("## 9.4 Payout provider onboarding"));
        assert!(diagram.starts_with("sequenceDiagram"));
    }
}
