import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { useAppStore } from "../stores/useAppStore";
import { MarketplacePage } from "./MarketplacePage";

const apiMocks = vi.hoisted(() => ({
  catalog: vi.fn(),
  catalogDetail: vi.fn(),
  installations: vi.fn(),
  permissions: vi.fn(),
  runtimeStatus: vi.fn(),
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
      creator: vi.fn().mockResolvedValue({ creator: null }),
      listings: vi.fn().mockResolvedValue([]),
      catalog: apiMocks.catalog,
      catalogDetail: apiMocks.catalogDetail,
      installations: apiMocks.installations,
      permissions: apiMocks.permissions,
      runtimeStatus: apiMocks.runtimeStatus,
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
  apiMocks.catalog.mockResolvedValue([freeItem]);
  apiMocks.catalogDetail.mockResolvedValue(freeDetail);
  apiMocks.installations.mockResolvedValue([]);
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
  apiMocks.install.mockResolvedValue(installation);
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

  it("explains paid entitlement and unsupported runtime gates", async () => {
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
    expect(await screen.findByText(/paid or custom entitlement is required/i)).toBeInTheDocument();
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
    expect(within(runtimeHeading.closest("section")!).getByText("page.read")).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Block this organization" }));

    await waitFor(() => expect(apiMocks.activateOrganizationKillSwitch).toHaveBeenCalledWith("suspicious artifact"));
    prompt.mockRestore();
  });
});
