import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { useAppStore } from "../stores/useAppStore";
import { MarketplacePage } from "./MarketplacePage";

const apiMocks = vi.hoisted(() => ({
  creator: vi.fn(),
  creatorBalance: vi.fn(),
  creatorAnalytics: vi.fn(),
  adminAnalytics: vi.fn(),
  listings: vi.fn(),
  catalog: vi.fn(),
  catalogDetail: vi.fn(),
  installations: vi.fn(),
  purchases: vi.fn(),
  checkout: vi.fn(),
  submitReview: vi.fn(),
  reviewModerationQueue: vi.fn(),
  moderateReview: vi.fn(),
  submitAbuseReport: vi.fn(),
  abuseReports: vi.fn(),
  resolveAbuseReport: vi.fn(),
  permissions: vi.fn(),
  runtimeStatus: vi.fn(),
  hooks: vi.fn(),
  activateOrganizationKillSwitch: vi.fn(),
  activateGlobalKillSwitch: vi.fn(),
  liftKillSwitch: vi.fn(),
  install: vi.fn(),
  installationUpdates: vi.fn(),
  enableInstallation: vi.fn(),
  disableInstallation: vi.fn(),
  uninstallInstallation: vi.fn(),
  rollbackInstallation: vi.fn(),
  updateInstallation: vi.fn(),
}));

vi.mock("../services/api", () => ({
  ApiError: class ApiError extends Error {
    status = 500;
  },
  setApiAccessToken: vi.fn(),
  setApiOrganizationId: vi.fn(),
  setApiRefreshToken: vi.fn(),
  getStoredRefreshToken: vi.fn(),
  api: {
    baseUrl: "http://localhost:8080",
    marketplace: {
      creator: apiMocks.creator,
      listings: apiMocks.listings,
      catalog: apiMocks.catalog,
      catalogDetail: apiMocks.catalogDetail,
      installations: apiMocks.installations,
      purchases: apiMocks.purchases,
      checkout: apiMocks.checkout,
      submitReview: apiMocks.submitReview,
      reviewModerationQueue: apiMocks.reviewModerationQueue,
      moderateReview: apiMocks.moderateReview,
      submitAbuseReport: apiMocks.submitAbuseReport,
      abuseReports: apiMocks.abuseReports,
      resolveAbuseReport: apiMocks.resolveAbuseReport,
      creatorBalance: apiMocks.creatorBalance,
      creatorAnalytics: apiMocks.creatorAnalytics,
      adminAnalytics: apiMocks.adminAnalytics,
      permissions: apiMocks.permissions,
      runtimeStatus: apiMocks.runtimeStatus,
      hooks: apiMocks.hooks,
      activateOrganizationKillSwitch: apiMocks.activateOrganizationKillSwitch,
      activateGlobalKillSwitch: apiMocks.activateGlobalKillSwitch,
      liftKillSwitch: apiMocks.liftKillSwitch,
      install: apiMocks.install,
      installationUpdates: apiMocks.installationUpdates,
      enableInstallation: apiMocks.enableInstallation,
      disableInstallation: apiMocks.disableInstallation,
      uninstallInstallation: apiMocks.uninstallInstallation,
      rollbackInstallation: apiMocks.rollbackInstallation,
      updateInstallation: apiMocks.updateInstallation,
      submissions: vi.fn().mockResolvedValue([]),
      reviewQueue: vi.fn().mockResolvedValue([]),
      reviewEvents: vi.fn().mockResolvedValue([]),
    },
    marketplaceAdapters: {
      hooks: apiMocks.hooks,
    },
  },
}));

const freeItem = {
  id: "listing-free",
  title: "Free Pack",
  slug: "free-pack",
  summary: "A safe component pack",
  category: "components",
  product_type: "component_pack" as const,
  pricing_type: "free" as const,
  price_cents: 0,
  creator_display_name: "Creator",
  latest_version_id: "version-1",
  latest_version: "1.0.0",
  badge: "compatible",
  rating_average: 0,
  rating_count: 0,
  active_installations: 0,
  compatibility_report: { compatible: true, install_eligible: true },
  permissions: ["page.read"],
  screenshots: [],
  support_url: null,
  updated_at: "2026-07-10T00:00:00Z",
};

const freeDetail = {
  item: freeItem,
  description: "Long free description",
  license: "commercial",
  support_url: null,
  screenshots: [],
  permissions: ["page.read"],
  changelog: ["Initial release"],
  versions: [{
    id: "version-1",
    version: "1.0.0",
    compatibility_report: { compatible: true, install_eligible: true },
    permissions: ["page.read"],
    changelog: ["Initial release"],
    created_at: "2026-07-10T00:00:00Z",
  }],
  reviews: [],
};

const installation = {
  id: "installation-1",
  organization_id: "org-1",
  listing_id: "listing-installed",
  listing_title: "Installed Pack",
  listing_slug: "installed-pack",
  product_type: "component_pack" as const,
  pricing_type: "free" as const,
  version_id: "version-old",
  installed_version: "1.0.0",
  status: "active" as const,
  permissions: ["page.read"],
  permission_approved_by: "user-1",
  permission_approved_at: "2026-07-10T00:00:00Z",
  rollback_version_id: "version-rollback",
  rollback_version: "0.9.0",
  cleanup_policy: "preserve_organization_data",
  version_pinned: true,
  installed_by: "user-1",
  installed_at: "2026-07-10T00:00:00Z",
  enabled_at: "2026-07-10T00:00:00Z",
  disabled_at: null,
  uninstalled_at: null,
  version_changed_at: "2026-07-10T00:00:00Z",
  updated_at: "2026-07-10T00:00:00Z",
};

function setMembership(role: string, globalRole = "viewer") {
  useAppStore.setState({
    user: {
      id: "user-1",
      email: "user@example.com",
      name: "User",
      avatar_url: null,
      role: globalRole,
    },
    organizations: [{ id: "org-1", name: "Organization", slug: "organization", role, status: "active" }],
    activeOrganizationId: "org-1",
  });
}

beforeEach(() => {
  vi.clearAllMocks();
  setMembership("owner");
  apiMocks.creator.mockResolvedValue({ creator: null });
  apiMocks.listings.mockResolvedValue([]);
  apiMocks.creatorBalance.mockResolvedValue({
    creator_id: "creator-1",
    currency: "usd",
    pending_cents: 0,
    available_cents: 0,
    paid_cents: 0,
    net_earned_cents: 0,
    settlement_days: 7,
  });
  apiMocks.creatorAnalytics.mockResolvedValue({
    creator_id: "creator-1",
    listing_count: 0,
    total_installs: 0,
    active_installs: 0,
    purchase_attempts: 0,
    completed_purchases: 0,
    refunded_purchases: 0,
    gross_revenue_cents: 0,
    creator_revenue_cents: 0,
    conversion_rate: 0,
    error_count: 0,
    products: [],
  });
  apiMocks.adminAnalytics.mockResolvedValue({
    generated_at: "2026-07-11T00:00:00Z",
    submission_count_30d: 0,
    submission_rate_per_day: 0,
    average_approval_hours: 0,
    total_installs: 0,
    active_installs: 0,
    refund_count: 0,
    report_count: 0,
    critical_report_count: 0,
    blocked_package_count: 0,
    risky_products: [],
  });
  apiMocks.catalog.mockResolvedValue([freeItem]);
  apiMocks.catalogDetail.mockResolvedValue(freeDetail);
  apiMocks.installations.mockResolvedValue([]);
  apiMocks.purchases.mockResolvedValue([]);
  apiMocks.permissions.mockResolvedValue([
    {
      permission_key: "page.read",
      description: "Read pages",
      category: "page",
      risk_level: "low",
      product_types: ["component_pack"],
      runtime_operations: ["component.render"],
      enabled: true,
    },
  ]);
  apiMocks.runtimeStatus.mockResolvedValue({
    global_blocked: false,
    organization_blocked: false,
    organization_id: "org-1",
    status_message: "Marketplace runtime is ready",
    active_kill_switches: [],
  });
  apiMocks.hooks.mockResolvedValue([
    {
      installation_id: "installation-plugin-1",
      hook_key: "sidebar-item",
      hook_type: "sidebar.item",
      label: "Plugin navigation",
      contract_version: "2026-07",
      config: {},
      listing_title: "Public Plugin",
      version: "1.0.0",
      enabled: true,
    },
  ]);
  apiMocks.install.mockResolvedValue(installation);
  apiMocks.submitReview.mockResolvedValue({ id: "review-1", status: "pending" });
  apiMocks.reviewModerationQueue.mockResolvedValue([]);
  apiMocks.moderateReview.mockResolvedValue({ id: "review-1", status: "published" });
  apiMocks.submitAbuseReport.mockResolvedValue({ id: "report-1", status: "open" });
  apiMocks.abuseReports.mockResolvedValue([]);
  apiMocks.resolveAbuseReport.mockResolvedValue({ id: "report-1", status: "investigating" });
  apiMocks.installationUpdates.mockResolvedValue({
    installation_id: installation.id,
    current_version_id: installation.version_id,
    current_version: installation.installed_version,
    current_status: installation.status,
    version_pinned: installation.version_pinned,
    update_available: true,
    target_version_id: "version-2",
    target_version: "2.0.0",
    changelog: ["Adds a new editor"],
    permissions: ["page.read", "content.write"],
    permission_reapproval_required: true,
    compatibility_report: { compatible: true, install_eligible: true },
    reasons: [],
  });
  apiMocks.updateInstallation.mockResolvedValue({ ...installation, version_id: "version-2", installed_version: "2.0.0" });
});

afterEach(() => {
  cleanup();
});

describe("Marketplace Phase 6", () => {
  it("requires explicit permission approval and organization confirmation before install", async () => {
    render(<MarketplacePage />);

    const catalogHeading = await screen.findByRole("heading", { name: "Free Pack" });
    const catalogCard = catalogHeading.closest("article");
    expect(catalogCard).not.toBeNull();
    fireEvent.click(within(catalogCard!).getByRole("button", { name: "Details" }));

    await screen.findByText("Long free description");
    fireEvent.click(screen.getByRole("button", { name: "Review and install" }));

    const installButton = screen.getByRole("button", { name: "Install approved version" });
    expect(installButton).toBeDisabled();
    fireEvent.click(screen.getByLabelText("page.read"));
    expect(installButton).toBeDisabled();
    fireEvent.click(screen.getByLabelText(/I confirm this version/));
    expect(installButton).toBeEnabled();
    fireEvent.click(installButton);

    await waitFor(() => expect(apiMocks.install).toHaveBeenCalledWith({
      listing_id: "listing-free",
      version_id: "version-1",
      approved_permissions: ["page.read"],
    }));
  });

  it("uses the active organization role instead of the global role for install gating", async () => {
    setMembership("viewer", "super_admin");
    render(<MarketplacePage />);

    const catalogHeading = await screen.findByRole("heading", { name: "Free Pack" });
    fireEvent.click(within(catalogHeading.closest("article")!).getByRole("button", { name: "Details" }));

    expect(await screen.findByText(/Only an organization owner or admin/)).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Review and install" })).toBeDisabled();
  });

  it("offers paid checkout while preserving unsupported runtime gates", async () => {
    const paidItem = {
      ...freeItem,
      id: "listing-paid",
      slug: "paid-pack",
      title: "Paid Pack",
      pricing_type: "paid" as const,
      price_cents: 4900,
    };
    const unsupportedItem = {
      ...freeItem,
      id: "listing-plugin",
      slug: "integration-plugin",
      title: "Integration Plugin",
      product_type: "integration_plugin" as const,
    };
    apiMocks.catalog.mockResolvedValue([paidItem, unsupportedItem]);
    apiMocks.catalogDetail.mockImplementation(async (slug: string) => (
      slug === paidItem.slug
        ? { ...freeDetail, item: paidItem, description: "Paid product details" }
        : { ...freeDetail, item: unsupportedItem, description: "Plugin product details" }
    ));
    render(<MarketplacePage />);

    const paidHeading = await screen.findByRole("heading", { name: "Paid Pack" });
    fireEvent.click(within(paidHeading.closest("article")!).getByRole("button", { name: "Details" }));
    expect(await screen.findByText(/Complete the organization purchase/i)).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Purchase for $49.00" })).toBeEnabled();
    expect(screen.getByRole("button", { name: "Review and install" })).toBeDisabled();

    const pluginHeading = screen.getByRole("heading", { name: "Integration Plugin" });
    fireEvent.click(within(pluginHeading.closest("article")!).getByRole("button", { name: "Details" }));
    expect(await screen.findByText(/controlled runtime that is not available/i)).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Review and install" })).toBeDisabled();
  });

  it("blocks duplicate active installations", async () => {
    apiMocks.installations.mockResolvedValue([{ ...installation, listing_id: freeItem.id }]);
    render(<MarketplacePage />);

    const catalogHeading = await screen.findByRole("heading", { name: "Free Pack" });
    fireEvent.click(within(catalogHeading.closest("article")!).getByRole("button", { name: "Details" }));

    expect(await screen.findByText(/already installed for the active organization/i)).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Review and install" })).toBeDisabled();
  });

  it("limits blocked installations to soft-uninstall controls", async () => {
    apiMocks.catalog.mockResolvedValue([]);
    apiMocks.installations.mockResolvedValue([{ ...installation, status: "blocked" as const }]);
    render(<MarketplacePage />);

    await screen.findByRole("heading", { name: "Installed Pack" });
    expect(screen.queryByRole("button", { name: "Check for updates" })).not.toBeInTheDocument();
    expect(screen.queryByRole("button", { name: /Rollback to/ })).not.toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Disable" })).not.toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Uninstall" })).toBeEnabled();
  });

  it("shows permission changes and requires re-approval plus changelog confirmation before update", async () => {
    apiMocks.catalog.mockResolvedValue([]);
    apiMocks.installations.mockResolvedValue([installation]);
    render(<MarketplacePage />);

    await screen.findByRole("heading", { name: "Installed Pack" });
    fireEvent.click(screen.getByRole("button", { name: "Check for updates" }));

    expect(await screen.findByText("Permission re-approval required")).toBeInTheDocument();
    expect(screen.getByText("+ content.write")).toBeInTheDocument();
    const updateButton = screen.getByRole("button", { name: "Update to v2.0.0" });
    expect(updateButton).toBeDisabled();

    fireEvent.click(screen.getByLabelText("page.read"));
    fireEvent.click(screen.getByLabelText("content.write"));
    fireEvent.click(screen.getByLabelText(/I reviewed the changelog/));
    expect(updateButton).toBeDisabled();
    fireEvent.click(screen.getByLabelText(/I approve the complete target permission set/));
    expect(updateButton).toBeEnabled();
    fireEvent.click(updateButton);

    await waitFor(() => expect(apiMocks.updateInstallation).toHaveBeenCalledWith("installation-1", {
      version_id: "version-2",
      changelog_confirmed: true,
      approved_permissions: ["page.read", "content.write"],
    }));
  });

  it("shows the Phase 7 permission catalog and organization kill switch control", async () => {
    const prompt = vi.spyOn(window, "prompt").mockReturnValue("suspicious artifact");
    apiMocks.activateOrganizationKillSwitch.mockResolvedValue({
      id: "switch-1",
      scope: "organization",
      organization_id: "org-1",
      reason: "suspicious artifact",
      active: true,
      created_by: "user-1",
      created_at: "2026-07-10T00:00:00Z",
      lifted_by: null,
      lifted_at: null,
    });
    render(<MarketplacePage />);

    const runtimeHeading = await screen.findByRole("heading", { name: "Runtime safety" });
    expect(runtimeHeading).toBeInTheDocument();
    expect(await screen.findByRole("heading", { name: "Extension hooks" })).toBeInTheDocument();
    expect(screen.getByText(/Plugin navigation/)).toBeInTheDocument();
    expect(within(runtimeHeading.closest("section")!).getByText("page.read")).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Block this organization" }));

    await waitFor(() => expect(apiMocks.activateOrganizationKillSwitch).toHaveBeenCalledWith("suspicious artifact"));
    prompt.mockRestore();
  });
});

describe("Marketplace Phase 10", () => {
  it("keeps review submission disabled when the organization has not installed or purchased the product", async () => {
    render(<MarketplacePage />);

    const catalogHeading = await screen.findByRole("heading", { name: "Free Pack" });
    fireEvent.click(within(catalogHeading.closest("article")!).getByRole("button", { name: "Details" }));
    await screen.findByText("Long free description");
    fireEvent.change(screen.getByLabelText("Review"), { target: { value: "This should remain gated" } });

    expect(screen.getByRole("button", { name: "Submit review" })).toBeDisabled();
    expect(apiMocks.submitReview).not.toHaveBeenCalled();
  });

  it("submits a customer review only after the product is installed", async () => {
    apiMocks.installations.mockResolvedValue([{ ...installation, listing_id: freeItem.id }]);
    render(<MarketplacePage />);

    const catalogHeading = await screen.findByRole("heading", { name: "Free Pack" });
    fireEvent.click(within(catalogHeading.closest("article")!).getByRole("button", { name: "Details" }));
    await screen.findByText("Long free description");
    fireEvent.change(screen.getByLabelText("Review"), { target: { value: "Useful and reliable pack" } });
    fireEvent.click(screen.getByRole("button", { name: "Submit review" }));

    await waitFor(() => expect(apiMocks.submitReview).toHaveBeenCalledWith("listing-free", {
      version_id: "version-1",
      rating: 5,
      body: "Useful and reliable pack",
    }));
  });

  it("allows any authenticated organization member to submit an abuse report", async () => {
    setMembership("viewer");
    render(<MarketplacePage />);

    const catalogHeading = await screen.findByRole("heading", { name: "Free Pack" });
    fireEvent.click(within(catalogHeading.closest("article")!).getByRole("button", { name: "Details" }));
    await screen.findByText("Long free description");
    const reportForm = screen.getByRole("heading", { name: "Report abuse" }).closest("div");
    expect(reportForm).not.toBeNull();
    fireEvent.change(within(reportForm!).getByLabelText("Description"), { target: { value: "This package contains suspicious copied code" } });
    fireEvent.click(within(reportForm!).getByRole("button", { name: "Submit report" }));

    await waitFor(() => expect(apiMocks.submitAbuseReport).toHaveBeenCalledWith("listing-free", {
      version_id: "version-1",
      report_type: "other",
      severity: "medium",
      description: "This package contains suspicious copied code",
      evidence: {},
    }));
  });

  it("lets global admins moderate customer reviews and investigate abuse reports", async () => {
    setMembership("owner", "admin");
    apiMocks.reviewModerationQueue.mockResolvedValue([{
      id: "review-pending",
      organization_id: "org-2",
      listing_id: "listing-free",
      version_id: "version-1",
      author_id: "user-2",
      author: "Reviewer",
      rating: 2,
      body: "Needs moderation",
      status: "pending",
      moderation_reason: null,
      moderated_by: null,
      moderated_at: null,
      created_at: "2026-07-11T00:00:00Z",
      updated_at: "2026-07-11T00:00:00Z",
    }]);
    apiMocks.abuseReports.mockResolvedValue([{
      id: "report-open",
      organization_id: "org-2",
      listing_id: "listing-free",
      version_id: "version-1",
      reporter_id: "user-2",
      report_type: "malware",
      severity: "critical",
      description: "Suspicious executable payload",
      evidence: {},
      status: "open",
      resolution_note: null,
      notification_status: "created",
      critical_notified_at: "2026-07-11T00:00:00Z",
      resolved_by: null,
      resolved_at: null,
      created_at: "2026-07-11T00:00:00Z",
      updated_at: "2026-07-11T00:00:00Z",
    }]);
    render(<MarketplacePage />);

    expect(await screen.findByRole("heading", { name: "Customer review queue" })).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Publish customer review" }));
    await waitFor(() => expect(apiMocks.moderateReview).toHaveBeenCalledWith("review-pending", {
      status: "published",
      moderation_reason: undefined,
    }));

    fireEvent.click(screen.getByRole("button", { name: "Investigate abuse report" }));
    await waitFor(() => expect(apiMocks.resolveAbuseReport).toHaveBeenCalledWith("report-open", {
      status: "investigating",
      resolution_note: undefined,
    }));
  });
});

describe("Marketplace Phase 11", () => {
  it("shows creator product performance and admin Marketplace health analytics", async () => {
    setMembership("owner", "admin");
    apiMocks.creator.mockResolvedValue({
      creator: {
        id: "creator-1",
        user_id: "user-1",
        slug: "zinhar",
        display_name: "Zinhar Creator",
        bio: "Builds safe extensions",
        status: "approved",
        payout_status: "verified",
        support_email: "creator@example.com",
        verification_notes: null,
        verified_by: "admin-1",
        verified_at: "2026-07-11T00:00:00Z",
        metadata: {},
        requested_at: "2026-07-11T00:00:00Z",
        created_at: "2026-07-11T00:00:00Z",
        updated_at: "2026-07-11T00:00:00Z",
      },
    });
    apiMocks.creatorAnalytics.mockResolvedValue({
      creator_id: "creator-1",
      listing_count: 1,
      total_installs: 12,
      active_installs: 10,
      purchase_attempts: 8,
      completed_purchases: 4,
      refunded_purchases: 1,
      gross_revenue_cents: 19900,
      creator_revenue_cents: 12345,
      conversion_rate: 0.5,
      error_count: 2,
      products: [{
        listing_id: "listing-analytics",
        title: "SaaS Hero Pack",
        slug: "saas-hero-pack",
        status: "approved",
        product_type: "component_pack",
        pricing_type: "paid",
        total_installs: 12,
        active_installs: 10,
        purchase_attempts: 8,
        completed_purchases: 4,
        refunded_purchases: 1,
        gross_revenue_cents: 19900,
        creator_revenue_cents: 12345,
        conversion_rate: 0.5,
        error_count: 2,
        report_count: 1,
        average_rating: 4.5,
        rating_count: 6,
        last_activity_at: "2026-07-11T00:00:00Z",
      }],
    });
    apiMocks.adminAnalytics.mockResolvedValue({
      generated_at: "2026-07-11T00:00:00Z",
      submission_count_30d: 75,
      submission_rate_per_day: 2.5,
      average_approval_hours: 12,
      total_installs: 40,
      active_installs: 33,
      refund_count: 3,
      report_count: 5,
      critical_report_count: 1,
      blocked_package_count: 2,
      risky_products: [{
        listing_id: "listing-risky",
        title: "Risky Plugin",
        slug: "risky-plugin",
        creator_display_name: "Repeat Creator",
        status: "blocked",
        product_type: "integration_plugin",
        security_risk_level: "critical",
        report_count: 4,
        critical_report_count: 1,
        blocked_package_count: 2,
        refund_count: 1,
        error_count: 8,
        active_installs: 0,
      }],
    });

    render(<MarketplacePage />);

    expect(await screen.findByRole("heading", { name: "Creator analytics" })).toBeInTheDocument();
    expect(screen.getByText("SaaS Hero Pack")).toBeInTheDocument();
    expect(screen.getByText("$123.45")).toBeInTheDocument();
    expect(screen.getAllByText("50.0%").length).toBeGreaterThan(0);
    expect(await screen.findByRole("heading", { name: "Marketplace health analytics" })).toBeInTheDocument();
    expect(screen.getByText("Risky Plugin")).toBeInTheDocument();
    expect(screen.getByText("2.5/day")).toBeInTheDocument();
    await waitFor(() => expect(apiMocks.creatorAnalytics).toHaveBeenCalledWith("creator-1"));
    await waitFor(() => expect(apiMocks.adminAnalytics).toHaveBeenCalled());
  });
});
