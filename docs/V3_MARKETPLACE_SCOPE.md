# V3 Marketplace Scope

This document locks the initial V3 Marketplace scope. It is the source of truth for deciding whether a product idea belongs in the first V3 Marketplace implementation.

## Product Goal

V3 turns ZinharCMS from a multi-tenant SaaS CMS into an ecosystem platform where approved creators can publish installable products and organizations can discover, install, buy, update, disable, uninstall, and review them from inside the product.

The Marketplace must cover the complete lifecycle:

- creator profile
- listing submission
- package and version review
- compatibility validation
- security review
- catalog discovery
- organization-scoped installation
- permission approval
- purchase or entitlement
- update and rollback
- report abuse
- takedown and kill switch
- creator and Marketplace analytics

## In Scope For V3

- Marketplace domain model for Creator, Listing, Package, Version, Submission, Installation, Purchase, and Review.
- Standard manifest for publishable Component Pack, Design Template, Integration Plugin, and tightly controlled Backend Extension products.
- Creator portal for profile, listing, package version submission, and analytics.
- Automated package validation, checksum verification, compatibility checks, and security scanning.
- Human review workflow with approve, reject, request changes, suspend, and takedown decisions.
- Public catalog with category, type, price, rating, compatibility, and install filters.
- Tenant-aware install, disable, uninstall, update, and rollback operations.
- Permission model for installed products.
- Free and paid product flows.
- Revenue split ledger and payout eligibility tracking.
- User rating, review, report abuse, and moderation tools.
- Audit trail for submission, review, install, purchase, payout, rollback, and takedown.

## V3 MVP Scope

The first practical V3 beta should be limited to:

- free Component Pack products
- free Design Template products
- reviewed creators only
- package registry with manifest and checksum
- manual human approval
- tenant-aware install and uninstall
- no arbitrary server-side code execution
- no unreviewed public listing
- no paid products until the free install path is stable

## Out Of Scope For Initial V3

- Arbitrary server-side code execution without an official sandbox.
- Public Marketplace publishing without human review.
- Full multi-language plugin runtime support.
- External Marketplace sales outside ZinharCMS.
- Paid placement, bidding, advertising, affiliate flows, or advanced coupons.
- Products that require direct database access.
- Products that bypass ZinharCMS RBAC, tenant context, audit logs, or review state.

## Scope Lock Rules

- If a product cannot be classified through `docs/V3_PRODUCT_TAXONOMY.md`, it is Unsupported.
- If a product asks for a permission that does not exist yet, it is blocked or sent back for changes.
- If a product needs execution capabilities outside the reviewed runtime, it is blocked until the sandbox phase.
- If a product can expose tenant data across organizations, it is rejected.
- If a product cannot be rolled back or disabled, it is not eligible for production installation.