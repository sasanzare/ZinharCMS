# V3 Phase 5 Public Catalog, Search, And Listing Details

Phase 5 turns the reviewed Marketplace inventory into an organization-aware catalog. It does not install products yet; it exposes only products that are approved, safe enough for display, and compatible with the active organization.

## 5.1 Public Catalog

Delivered scope:

- `GET /api/marketplace/catalog` returns product cards for the active organization.
- Catalog rows require `listing.status = 'approved'`.
- Catalog rows require an approved package version with `version.status = 'approved'`.
- Catalog rows require package validation status `passed` or `warning`.
- Catalog rows require security risk `low` or `medium`.
- The response includes product title, slug, summary, category, product type, pricing type, price, creator name, compatibility badge, active install count, latest version, screenshots, support URL, and permissions.
- Compatibility is recalculated against the active organization's billing plan before a product is shown.

Acceptance coverage:

- Unreviewed, suspended, rejected, blocked, high-risk, or incompatible products are excluded.
- The `install_eligible` field in the compatibility report must be true for every visible catalog product.

## 5.2 Search And Filter

Delivered scope:

- `GET /api/marketplace/catalog?search=...` searches title, summary, slug, category, and creator display name.
- `category`, `product_type`, and `pricing_type` filters can be combined with search.
- Search and filters run before organization compatibility filtering.
- Suspended or incompatible products remain excluded even when they match search text.

Acceptance coverage:

- Search never returns products that are not approved and compatible.
- The admin UI exposes search, category, product type, and pricing filters without introducing horizontal page overflow.

## 5.3 Listing Detail Page

Delivered scope:

- `GET /api/marketplace/catalog/{listing_slug}` returns the full approved product detail.
- The detail response includes description, screenshots, changelog, permissions, compatible approved versions, review placeholder data, license, support link, and the same catalog item summary.
- The detail endpoint returns `404` when the listing is missing, not approved, has no approved safe version, or is not compatible with the active organization.
- The admin UI shows requested permissions before any future install action.

Acceptance coverage:

- Users can inspect product permissions, compatibility, version history, screenshots, changelog, and support information before install or purchase flows are added.
- Reviews are represented in the response contract even though review submission is scheduled for a later Marketplace phase.

## API Contract

Catalog endpoint:

```http
GET /api/marketplace/catalog?search=hero&category=components&product_type=component_pack&pricing_type=free
```

Detail endpoint:

```http
GET /api/marketplace/catalog/saas-hero-pack
```

Compatibility contract:

- `compatibility_report.compatible = true`
- `compatibility_report.install_eligible = true`
- `compatibility_report.organization_plan` reflects the active tenant plan.
- `compatibility_report.reasons` is empty for visible catalog products.

Install flow boundary:

- Phase 5 does not create installations.
- Phase 6 must enforce the same compatibility and permission contract at install time.