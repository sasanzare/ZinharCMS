import { useCallback, useEffect, useMemo, useState } from "react";
import { AlertTriangle, BadgeCheck, BarChart3, CheckCircle2, Eye, FileCheck, History, MessageSquare, PackagePlus, Power, RefreshCw, RotateCcw, Save, Search, Send, ShieldAlert, Star, Store, Tag, Trash2, Upload, X, XCircle } from "lucide-react";

import { StatusBadge } from "../components/StatusBadge";
import { useI18n } from "../i18n";
import { ApiError, api } from "../services/api";
import { useAppStore } from "../stores/useAppStore";
import type {
  JsonValue,
  JsonRecord,
  MarketplaceCatalogDetailResponse,
  MarketplaceCatalogItemResponse,
  MarketplaceCreatorResponse,
  MarketplaceCreatorBalanceResponse,
  MarketplaceInstallationResponse,
  MarketplaceInstallationUpdateCheckResponse,
  MarketplaceHookResponse,
  MarketplacePermissionCatalogResponse,
  MarketplaceProductReviewResponse,
  MarketplaceAbuseReportResponse,
  MarketplaceAdminAnalyticsResponse,
  MarketplaceCreatorAnalyticsResponse,
  MarketplacePurchaseResponse,
  MarketplaceListingRequest,
  MarketplaceListingResponse,
  MarketplaceModerationAction,
  MarketplaceModerationRequest,
  MarketplacePricingType,
  MarketplaceProductType,
  MarketplaceReviewDecision,
  MarketplaceReviewEventResponse,
  MarketplaceRuntimeStatusResponse,
  MarketplaceValidationReportResponse,
} from "../types/api";

const PRODUCT_TYPES: MarketplaceProductType[] = [
  "component_pack",
  "design_template",
  "integration_plugin",
  "backend_extension",
];
const PRICING_TYPES: MarketplacePricingType[] = ["free", "paid", "custom"];

const defaultCatalogFilters = {
  search: "",
  category: "",
  product_type: "" as "" | MarketplaceProductType,
  pricing_type: "" as "" | MarketplacePricingType,
};

const defaultCreatorDraft = {
  slug: "",
  display_name: "",
  bio: "",
  support_email: "",
};

const defaultListingDraft = {
  product_type: "component_pack" as MarketplaceProductType,
  title: "",
  slug: "",
  summary: "",
  description: "",
  category: "components",
  pricing_type: "free" as MarketplacePricingType,
  price_cents: 0,
  license: "commercial",
  support_url: "",
  screenshots: "",
};

const defaultManifest = JSON.stringify(
  {
    manifest_version: "2026-07",
    name: "SaaS Hero Pack",
    version: "1.0.0",
    type: "component_pack",
    permissions: ["page.read"],
    compatibility: {
      min_zinhar_version: "2.0.0",
      max_zinhar_version: "3.0.0",
    },
    entry_points: {
      components: "components/index.json",
    },
    assets: ["components/hero.json", "assets/preview.png"],
  },
  null,
  2,
);

type CreatorDraft = typeof defaultCreatorDraft;
type ListingDraft = typeof defaultListingDraft;
type CatalogFilterDraft = typeof defaultCatalogFilters;

const defaultReviewDraft = {
  internal_comment: "",
  creator_message: "",
  reason: "",
};

const defaultCustomerFeedback = {
  rating: 5,
  body: "",
  report_type: "other" as "malware" | "copyright" | "spam" | "fraud" | "privacy" | "other",
  severity: "medium" as "low" | "medium" | "high" | "critical",
  description: "",
  evidence: "{}",
};

type ReviewDraft = typeof defaultReviewDraft;

function apiMessage(caught: unknown, fallback: string) {
  return caught instanceof ApiError ? caught.message : fallback;
}

function creatorTone(status?: string) {
  if (status === "approved") return "success";
  if (status === "rejected" || status === "suspended") return "danger";
  if (status === "pending") return "warning";
  return "neutral";
}

function listingTone(status: string) {
  if (status === "approved") return "success";
  if (status === "submitted") return "warning";
  if (status === "blocked" || status === "suspended") return "danger";
  return "neutral";
}

function validationTone(status: string) {
  if (status === "passed") return "success";
  if (status === "warning" || status === "pending") return "warning";
  if (status === "failed") return "danger";
  return "neutral";
}

function riskTone(risk: string) {
  if (risk === "low") return "success";
  if (risk === "medium" || risk === "unreviewed") return "warning";
  if (risk === "high" || risk === "critical") return "danger";
  return "neutral";
}

function eventTone(action: string) {
  if (action === "approve") return "success";
  if (action === "request_changes" || action === "suspend_listing" || action === "unpublish_version") return "warning";
  if (action === "reject" || action === "emergency_block") return "danger";
  return "neutral";
}

function installationTone(status: string) {
  if (status === "active") return "success";
  if (status === "disabled" || status === "rollback_pending") return "warning";
  if (status === "blocked") return "danger";
  return "neutral";
}

function cleanOptional(value: string) {
  const trimmed = value.trim();
  return trimmed ? trimmed : undefined;
}

function canApproveReport(report: MarketplaceValidationReportResponse) {
  return report.version_status !== "blocked"
    && report.validation_status !== "failed"
    && report.security_risk_level !== "high"
    && report.security_risk_level !== "critical";
}

function formatReportJson(value: unknown) {
  return JSON.stringify(value, null, 2);
}

function screenshotsFromText(value: string) {
  return value
    .split(/\r?\n/)
    .map((item) => item.trim())
    .filter(Boolean);
}

function listingToDraft(listing: MarketplaceListingResponse): ListingDraft {
  return {
    product_type: listing.product_type,
    title: listing.title,
    slug: listing.slug,
    summary: listing.summary,
    description: listing.description,
    category: listing.category,
    pricing_type: listing.pricing_type,
    price_cents: listing.price_cents,
    license: listing.license,
    support_url: listing.support_url ?? "",
    screenshots: listing.screenshots.join("\n"),
  };
}

function stringListFromValue(value: JsonValue | unknown) {
  if (!Array.isArray(value)) return [];
  return value
    .map((item) => {
      if (typeof item === "string") return item;
      if (item === null || item === undefined) return "";
      return JSON.stringify(item);
    })
    .map((item) => item.trim())
    .filter(Boolean);
}

function catalogQuery(filters: CatalogFilterDraft) {
  return {
    search: cleanOptional(filters.search),
    category: cleanOptional(filters.category),
    product_type: filters.product_type || undefined,
    pricing_type: filters.pricing_type || undefined,
  };
}

function compatibilityAllowsInstall(item: MarketplaceCatalogItemResponse) {
  return item.compatibility_report.install_eligible === true
    && item.compatibility_report.compatible !== false;
}

function hasEveryPermission(approved: string[], requested: string[]) {
  return requested.every((permission) => approved.includes(permission));
}

export function MarketplacePage() {
  const { t } = useI18n();
  const user = useAppStore((state) => state.user);
  const organizations = useAppStore((state) => state.organizations);
  const activeOrganizationId = useAppStore((state) => state.activeOrganizationId);
  const [creator, setCreator] = useState<MarketplaceCreatorResponse | null>(null);
  const [creatorBalance, setCreatorBalance] = useState<MarketplaceCreatorBalanceResponse | null>(null);
  const [creatorAnalytics, setCreatorAnalytics] = useState<MarketplaceCreatorAnalyticsResponse | null>(null);
  const [adminAnalytics, setAdminAnalytics] = useState<MarketplaceAdminAnalyticsResponse | null>(null);
  const [creatorDraft, setCreatorDraft] = useState<CreatorDraft>(defaultCreatorDraft);
  const [listings, setListings] = useState<MarketplaceListingResponse[]>([]);
  const [listingDraft, setListingDraft] = useState<ListingDraft>(defaultListingDraft);
  const [editingListingId, setEditingListingId] = useState<string | null>(null);
  const [selectedListingId, setSelectedListingId] = useState("");
  const [submissionReports, setSubmissionReports] = useState<MarketplaceValidationReportResponse[]>([]);
  const [reviewReports, setReviewReports] = useState<MarketplaceValidationReportResponse[]>([]);
  const [reviewEvents, setReviewEvents] = useState<MarketplaceReviewEventResponse[]>([]);
  const [reviewDrafts, setReviewDrafts] = useState<Record<string, ReviewDraft>>({});
  const [customerFeedback, setCustomerFeedback] = useState(defaultCustomerFeedback);
  const [customerReviewQueue, setCustomerReviewQueue] = useState<MarketplaceProductReviewResponse[]>([]);
  const [abuseReportQueue, setAbuseReportQueue] = useState<MarketplaceAbuseReportResponse[]>([]);
  const [customerReviewReasons, setCustomerReviewReasons] = useState<Record<string, string>>({});
  const [abuseResolutionNotes, setAbuseResolutionNotes] = useState<Record<string, string>>({});
  const [catalogFilters, setCatalogFilters] = useState<CatalogFilterDraft>(defaultCatalogFilters);
  const [catalogItems, setCatalogItems] = useState<MarketplaceCatalogItemResponse[]>([]);
  const [catalogDetail, setCatalogDetail] = useState<MarketplaceCatalogDetailResponse | null>(null);
  const [installations, setInstallations] = useState<MarketplaceInstallationResponse[]>([]);
  const [purchases, setPurchases] = useState<MarketplacePurchaseResponse[]>([]);
  const [permissionCatalog, setPermissionCatalog] = useState<MarketplacePermissionCatalogResponse[]>([]);
  const [runtimeStatus, setRuntimeStatus] = useState<MarketplaceRuntimeStatusResponse | null>(null);
  const [marketplaceHooks, setMarketplaceHooks] = useState<MarketplaceHookResponse[]>([]);
  const [installTarget, setInstallTarget] = useState<MarketplaceCatalogDetailResponse | null>(null);
  const [approvedInstallPermissions, setApprovedInstallPermissions] = useState<string[]>([]);
  const [installConfirmed, setInstallConfirmed] = useState(false);
  const [installationUpdates, setInstallationUpdates] = useState<Record<string, MarketplaceInstallationUpdateCheckResponse>>({});
  const [approvedUpdatePermissions, setApprovedUpdatePermissions] = useState<Record<string, string[]>>({});
  const [confirmedUpdatePermissions, setConfirmedUpdatePermissions] = useState<Record<string, boolean>>({});
  const [confirmedUpdates, setConfirmedUpdates] = useState<Record<string, boolean>>({});
  const [uninstallConfirmId, setUninstallConfirmId] = useState<string | null>(null);
  const [manifest, setManifest] = useState(defaultManifest);
  const [packageFile, setPackageFile] = useState<File | null>(null);
  const [loading, setLoading] = useState(false);
  const [catalogLoading, setCatalogLoading] = useState(false);
  const [installationsLoading, setInstallationsLoading] = useState(true);
  const [actionLoading, setActionLoading] = useState(false);
  const [message, setMessage] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const approvedCreator = creator?.status === "approved";
  const canReviewMarketplace = user?.role === "admin" || user?.role === "super_admin";
  const activeOrganizationRole = organizations.find((organization) => organization.id === activeOrganizationId)?.role ?? null;
  const canManageInstallations = activeOrganizationRole === "owner" || activeOrganizationRole === "admin";
  const canManageKillSwitch = canManageInstallations;
  const canManageGlobalKillSwitch = user?.role === "admin" || user?.role === "super_admin";
  const selectedListing = useMemo(
    () => listings.find((listing) => listing.id === selectedListingId) ?? null,
    [listings, selectedListingId],
  );
  const catalogCategories = useMemo(
    () => Array.from(new Set(catalogItems.map((item) => item.category).filter(Boolean))).sort(),
    [catalogItems],
  );

  function catalogPriceLabel(item: MarketplaceCatalogItemResponse) {
    if (item.pricing_type === "custom") return t("marketplace.catalog.customPrice");
    if (item.price_cents <= 0) return t("marketplace.catalog.free");
    return `$${(item.price_cents / 100).toFixed(2)}`;
  }

  function formatAnalyticsMoney(cents: number) {
    return `$${(cents / 100).toFixed(2)}`;
  }

  function formatAnalyticsRate(value: number) {
    return `${(value * 100).toFixed(1)}%`;
  }

  const loadCatalog = useCallback(async function loadCatalog() {
    setCatalogLoading(true);
    setError(null);
    try {
      setCatalogItems(await api.marketplace.catalog(catalogQuery(catalogFilters)));
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.catalog")));
    } finally {
      setCatalogLoading(false);
    }
  }, [catalogFilters, t]);

  const loadInstallations = useCallback(async function loadInstallations() {
    setInstallationsLoading(true);
    try {
      const [nextInstallations, nextPurchases] = await Promise.all([
        api.marketplace.installations(),
        api.marketplace.purchases(),
      ]);
      setInstallations(nextInstallations);
      setPurchases(nextPurchases);
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.installations")));
    } finally {
      setInstallationsLoading(false);
    }
  }, [t]);

  const loadRuntimeControls = useCallback(async function loadRuntimeControls() {
    try {
      const [permissions, status, hooks] = await Promise.all([
        api.marketplace.permissions(),
        api.marketplace.runtimeStatus(),
        api.marketplaceAdapters.hooks(),
      ]);
      setPermissionCatalog(permissions);
      setRuntimeStatus(status);
      setMarketplaceHooks(hooks);
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.runtime")));
    }
  }, [t]);

  const loadMarketplace = useCallback(async function loadMarketplace() {
    setLoading(true);
    setError(null);
    try {
      const [creatorState, nextListings] = await Promise.all([
        api.marketplace.creator(),
        api.marketplace.listings(),
      ]);
      setCreator(creatorState.creator);
      setListings(nextListings);
      if (creatorState.creator) {
        const [balance, analytics] = await Promise.all([
          api.marketplace.creatorBalance(creatorState.creator.id),
          api.marketplace.creatorAnalytics(creatorState.creator.id),
        ]);
        setCreatorBalance(balance);
        setCreatorAnalytics(analytics);
        setCreatorDraft({
          slug: creatorState.creator.slug,
          display_name: creatorState.creator.display_name,
          bio: creatorState.creator.bio ?? "",
          support_email: creatorState.creator.support_email ?? "",
        });
      } else {
        setCreatorBalance(null);
        setCreatorAnalytics(null);
      }
      if (!selectedListingId && nextListings.length > 0) {
        setSelectedListingId(nextListings[0].id);
      }
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.load")));
    } finally {
      setLoading(false);
    }
  }, [selectedListingId, t]);

  useEffect(() => {
    void loadMarketplace();
  }, [loadMarketplace]);

  useEffect(() => {
    void loadCatalog();
  }, [loadCatalog]);

  useEffect(() => {
    void loadInstallations();
  }, [loadInstallations]);

  useEffect(() => {
    void loadRuntimeControls();
  }, [loadRuntimeControls]);

  async function runKillSwitchAction(action: "organization" | "global" | "lift", killSwitchId?: string) {
    if (action === "organization" && !canManageKillSwitch) return;
    if (action === "global" && !canManageGlobalKillSwitch) return;
    if (action === "lift" && !killSwitchId) return;
    const reason = action === "lift" ? undefined : window.prompt(t("marketplace.runtime.reasonPrompt"));
    if (action !== "lift" && !reason?.trim()) return;
    setActionLoading(true);
    setError(null);
    try {
      if (action === "organization") await api.marketplace.activateOrganizationKillSwitch(reason!.trim());
      if (action === "global") await api.marketplace.activateGlobalKillSwitch(reason!.trim());
      if (action === "lift") await api.marketplace.liftKillSwitch(killSwitchId!);
      setMessage(t("marketplace.runtime.actionSaved"));
      await Promise.all([loadRuntimeControls(), loadInstallations()]);
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.runtimeAction")));
    } finally {
      setActionLoading(false);
    }
  }

  const loadSubmissionReports = useCallback(async function loadSubmissionReports() {
    if (!selectedListingId) {
      setSubmissionReports([]);
      return;
    }

    try {
      setSubmissionReports(await api.marketplace.submissions(selectedListingId));
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.reports")));
    }
  }, [selectedListingId, t]);

  const loadReviewReports = useCallback(async function loadReviewReports() {
    if (!canReviewMarketplace) {
      setReviewReports([]);
      return;
    }

    try {
      setReviewReports(await api.marketplace.reviewQueue());
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.reports")));
    }
  }, [canReviewMarketplace, t]);

  const loadReviewEvents = useCallback(async function loadReviewEvents() {
    if (!canReviewMarketplace) {
      setReviewEvents([]);
      return;
    }

    try {
      setReviewEvents(await api.marketplace.reviewEvents());
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.events")));
    }
  }, [canReviewMarketplace, t]);

  const loadCustomerModerationQueues = useCallback(async function loadCustomerModerationQueues() {
    if (!canReviewMarketplace) {
      setCustomerReviewQueue([]);
      setAbuseReportQueue([]);
      return;
    }
    try {
      const [reviews, reports] = await Promise.all([
        api.marketplace.reviewModerationQueue(),
        api.marketplace.abuseReports(),
      ]);
      setCustomerReviewQueue(reviews);
      setAbuseReportQueue(reports);
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.feedback.error.queue")));
    }
  }, [canReviewMarketplace, t]);

  const loadAdminAnalytics = useCallback(async function loadAdminAnalytics() {
    if (!canReviewMarketplace) {
      setAdminAnalytics(null);
      return;
    }

    try {
      setAdminAnalytics(await api.marketplace.adminAnalytics());
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.analytics")));
    }
  }, [canReviewMarketplace, t]);

  useEffect(() => {
    void loadSubmissionReports();
  }, [loadSubmissionReports]);

  useEffect(() => {
    void loadReviewReports();
  }, [loadReviewReports]);

  useEffect(() => {
    void loadReviewEvents();
  }, [loadReviewEvents]);

  useEffect(() => {
    void loadCustomerModerationQueues();
  }, [loadCustomerModerationQueues]);

  useEffect(() => {
    void loadAdminAnalytics();
  }, [loadAdminAnalytics]);

  async function refreshMarketplace() {
    await Promise.all([loadMarketplace(), loadCatalog(), loadInstallations(), loadRuntimeControls(), loadAdminAnalytics()]);
  }

  async function openCatalogDetail(item: MarketplaceCatalogItemResponse) {
    setCatalogLoading(true);
    setError(null);
    setCustomerFeedback(defaultCustomerFeedback);
    try {
      setCatalogDetail(await api.marketplace.catalogDetail(item.slug));
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.catalog")));
    } finally {
      setCatalogLoading(false);
    }
  }

  function resetCatalogFilters() {
    setCatalogFilters(defaultCatalogFilters);
  }

  function installationForListing(listingId: string) {
    return installations.find((installation) => (
      installation.listing_id === listingId && installation.status !== "uninstalled"
    )) ?? null;
  }

  function purchaseForListing(listingId: string) {
    return purchases.find((purchase) => purchase.listing_id === listingId && purchase.status === "completed") ?? null;
  }

  function installRestriction(detail: MarketplaceCatalogDetailResponse) {
    if (installationsLoading) return t("marketplace.install.checkingInstallations");
    if (installationForListing(detail.item.id)) return t("marketplace.install.alreadyInstalled");
    if (runtimeStatus?.global_blocked || runtimeStatus?.organization_blocked) return t("marketplace.install.runtimeBlocked");
    if (detail.item.pricing_type === "custom") return t("marketplace.install.entitlementRequired");
    if (detail.item.pricing_type === "paid" && !purchaseForListing(detail.item.id)) {
      return t("marketplace.install.purchaseRequired");
    }
    if (detail.item.product_type !== "component_pack" && detail.item.product_type !== "design_template") {
      return t("marketplace.install.runtimeUnsupported");
    }
    if (!compatibilityAllowsInstall(detail.item)) return t("marketplace.install.incompatible");
    if (!canManageInstallations) return t("marketplace.install.roleRequired");
    return null;
  }

  function openInstallApproval(detail: MarketplaceCatalogDetailResponse) {
    if (installRestriction(detail)) return;
    setInstallTarget(detail);
    setApprovedInstallPermissions([]);
    setInstallConfirmed(false);
  }

  function closeInstallApproval() {
    if (actionLoading) return;
    setInstallTarget(null);
    setApprovedInstallPermissions([]);
    setInstallConfirmed(false);
  }

  function toggleInstallPermission(permission: string) {
    setApprovedInstallPermissions((current) => (
      current.includes(permission)
        ? current.filter((item) => item !== permission)
        : [...current, permission]
    ));
  }

  async function refreshInstallationSurfaces() {
    await Promise.all([loadInstallations(), loadCatalog()]);
    if (catalogDetail) {
      setCatalogDetail(await api.marketplace.catalogDetail(catalogDetail.item.slug));
    }
  }

  async function installProduct() {
    if (!installTarget) return;
    const requestedPermissions = stringListFromValue(installTarget.permissions);
    if (!installConfirmed || !hasEveryPermission(approvedInstallPermissions, requestedPermissions)) return;

    setActionLoading(true);
    setMessage(null);
    setError(null);
    try {
      await api.marketplace.install({
        listing_id: installTarget.item.id,
        version_id: installTarget.item.latest_version_id,
        approved_permissions: requestedPermissions,
        purchase_id: purchaseForListing(installTarget.item.id)?.id,
      });
      setMessage(t("marketplace.message.installed", { title: installTarget.item.title }));
      setInstallTarget(null);
      setApprovedInstallPermissions([]);
      setInstallConfirmed(false);
      await refreshInstallationSurfaces();
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.install")));
    } finally {
      setActionLoading(false);
    }
  }

  async function purchaseProduct(detail: MarketplaceCatalogDetailResponse) {
    if (!canManageInstallations || detail.item.pricing_type !== "paid") return;
    setActionLoading(true);
    setMessage(null);
    setError(null);
    try {
      const checkout = await api.marketplace.checkout(detail.item.id, detail.item.latest_version_id);
      if (checkout.entitlement_granted) {
        await loadInstallations();
        setMessage(t("marketplace.purchase.completed"));
      } else if (checkout.checkout_url) {
        window.location.assign(checkout.checkout_url);
      } else {
        setMessage(t("marketplace.purchase.pending"));
      }
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.purchase")));
    } finally {
      setActionLoading(false);
    }
  }

  async function submitCustomerReview(detail: MarketplaceCatalogDetailResponse) {
    if (!canManageInstallations || !customerFeedback.body.trim()) return;
    setActionLoading(true);
    setError(null);
    try {
      await api.marketplace.submitReview(detail.item.id, {
        version_id: detail.item.latest_version_id,
        rating: customerFeedback.rating,
        body: customerFeedback.body.trim(),
      });
      setCustomerFeedback((current) => ({ ...current, body: "" }));
      setMessage(t("marketplace.feedback.reviewSubmitted"));
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.feedback.error.review")));
    } finally {
      setActionLoading(false);
    }
  }

  async function submitAbuseReport(detail: MarketplaceCatalogDetailResponse) {
    if (!customerFeedback.description.trim()) return;
    let evidence: JsonRecord;
    try {
      const parsed: unknown = JSON.parse(customerFeedback.evidence);
      if (!parsed || Array.isArray(parsed) || typeof parsed !== "object") throw new Error("evidence must be an object");
      evidence = parsed as JsonRecord;
    } catch {
      setError(t("marketplace.feedback.error.evidence"));
      return;
    }
    setActionLoading(true);
    setError(null);
    try {
      await api.marketplace.submitAbuseReport(detail.item.id, {
        version_id: detail.item.latest_version_id,
        report_type: customerFeedback.report_type,
        severity: customerFeedback.severity,
        description: customerFeedback.description.trim(),
        evidence,
      });
      setCustomerFeedback((current) => ({ ...current, description: "" }));
      setMessage(t(customerFeedback.severity === "critical" ? "marketplace.feedback.criticalSubmitted" : "marketplace.feedback.reportSubmitted"));
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.feedback.error.report")));
    } finally {
      setActionLoading(false);
    }
  }

  async function moderateCustomerReview(reviewId: string, status: "published" | "rejected") {
    setActionLoading(true);
    setError(null);
    try {
      await api.marketplace.moderateReview(reviewId, {
        status,
        moderation_reason: cleanOptional(customerReviewReasons[reviewId] ?? ""),
      });
      setMessage(t("marketplace.feedback.reviewModerated"));
      await Promise.all([loadCustomerModerationQueues(), loadAdminAnalytics()]);
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.feedback.error.moderation")));
    } finally {
      setActionLoading(false);
    }
  }

  async function resolveCustomerAbuseReport(reportId: string, status: "investigating" | "resolved" | "dismissed") {
    setActionLoading(true);
    setError(null);
    try {
      await api.marketplace.resolveAbuseReport(reportId, {
        status,
        resolution_note: cleanOptional(abuseResolutionNotes[reportId] ?? ""),
      });
      setMessage(t("marketplace.feedback.reportUpdated"));
      await Promise.all([loadCustomerModerationQueues(), loadAdminAnalytics()]);
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.feedback.error.resolution")));
    } finally {
      setActionLoading(false);
    }
  }

  async function startPayoutOnboarding() {
    if (!creator || !approvedCreator) return;
    const providerAccountId = window.prompt(t("marketplace.payout.accountPrompt"));
    if (!providerAccountId?.trim()) return;
    setActionLoading(true);
    setError(null);
    try {
      await api.marketplace.onboardPayout(creator.id, providerAccountId.trim());
      setMessage(t("marketplace.payout.pending"));
      await loadMarketplace();
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.payout")));
    } finally {
      setActionLoading(false);
    }
  }

  async function requestCreatorPayout() {
    if (!creator || !approvedCreator) return;
    setActionLoading(true);
    setError(null);
    try {
      await api.marketplace.requestPayout(creator.id);
      setMessage(t("marketplace.payout.requested"));
      const [balance, analytics] = await Promise.all([
        api.marketplace.creatorBalance(creator.id),
        api.marketplace.creatorAnalytics(creator.id),
      ]);
      setCreatorBalance(balance);
      setCreatorAnalytics(analytics);
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.payoutRequest")));
    } finally {
      setActionLoading(false);
    }
  }

  async function runInstallationAction(
    installation: MarketplaceInstallationResponse,
    action: "enable" | "disable" | "uninstall" | "rollback",
  ) {
    if (!canManageInstallations) return;
    setActionLoading(true);
    setMessage(null);
    setError(null);
    try {
      if (action === "enable") await api.marketplace.enableInstallation(installation.id);
      if (action === "disable") await api.marketplace.disableInstallation(installation.id);
      if (action === "uninstall") await api.marketplace.uninstallInstallation(installation.id);
      if (action === "rollback") await api.marketplace.rollbackInstallation(installation.id);
      const actionLabels = {
        enable: t("marketplace.installations.enable"),
        disable: t("marketplace.installations.disable"),
        uninstall: t("marketplace.installations.uninstall"),
        rollback: t("marketplace.installations.rollbackLabel"),
      };
      setMessage(t("marketplace.message.installationAction", { action: actionLabels[action] }));
      setUninstallConfirmId(null);
      setInstallationUpdates((current) => {
        const next = { ...current };
        delete next[installation.id];
        return next;
      });
      await refreshInstallationSurfaces();
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.installationAction")));
    } finally {
      setActionLoading(false);
    }
  }

  async function checkInstallationUpdates(installation: MarketplaceInstallationResponse) {
    setActionLoading(true);
    setMessage(null);
    setError(null);
    try {
      const update = await api.marketplace.installationUpdates(installation.id);
      setInstallationUpdates((current) => ({ ...current, [installation.id]: update }));
      setApprovedUpdatePermissions((current) => ({ ...current, [installation.id]: [] }));
      setConfirmedUpdatePermissions((current) => ({ ...current, [installation.id]: false }));
      setConfirmedUpdates((current) => ({ ...current, [installation.id]: false }));
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.updateCheck")));
    } finally {
      setActionLoading(false);
    }
  }

  function toggleUpdatePermission(installationId: string, permission: string) {
    setApprovedUpdatePermissions((current) => {
      const approved = current[installationId] ?? [];
      return {
        ...current,
        [installationId]: approved.includes(permission)
          ? approved.filter((item) => item !== permission)
          : [...approved, permission],
      };
    });
  }

  async function updateInstallation(installation: MarketplaceInstallationResponse) {
    const update = installationUpdates[installation.id];
    if (!update?.update_available || !update.target_version_id || !confirmedUpdates[installation.id]) return;
    const requestedPermissions = stringListFromValue(update.permissions);
    const approvedPermissions = approvedUpdatePermissions[installation.id] ?? [];
    if (update.permission_reapproval_required && !hasEveryPermission(approvedPermissions, requestedPermissions)) return;

    setActionLoading(true);
    setMessage(null);
    setError(null);
    try {
      await api.marketplace.updateInstallation(installation.id, {
        version_id: update.target_version_id,
        changelog_confirmed: true,
        approved_permissions: update.permission_reapproval_required ? requestedPermissions : undefined,
      });
      setMessage(t("marketplace.message.updated", { title: installation.listing_title, version: update.target_version ?? "" }));
      setInstallationUpdates((current) => {
        const next = { ...current };
        delete next[installation.id];
        return next;
      });
      await refreshInstallationSurfaces();
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.update")));
    } finally {
      setActionLoading(false);
    }
  }

  async function saveCreator() {
    setActionLoading(true);
    setMessage(null);
    setError(null);
    try {
      const saved = await api.marketplace.requestCreator({
        slug: creatorDraft.slug,
        display_name: creatorDraft.display_name,
        bio: creatorDraft.bio || undefined,
        support_email: creatorDraft.support_email || undefined,
      });
      setCreator(saved);
      setMessage(t("marketplace.message.creatorSaved"));
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.creator")));
    } finally {
      setActionLoading(false);
    }
  }

  function listingPayload(): MarketplaceListingRequest {
    return {
      product_type: listingDraft.product_type,
      title: listingDraft.title,
      slug: listingDraft.slug,
      summary: listingDraft.summary,
      description: listingDraft.description,
      category: listingDraft.category,
      pricing_type: listingDraft.pricing_type,
      price_cents: Number(listingDraft.price_cents) || 0,
      license: listingDraft.license,
      support_url: listingDraft.support_url || undefined,
      screenshots: screenshotsFromText(listingDraft.screenshots),
    };
  }

  async function saveListing() {
    setActionLoading(true);
    setMessage(null);
    setError(null);
    try {
      const payload = listingPayload();
      const saved = editingListingId
        ? await api.marketplace.updateListing(editingListingId, payload)
        : await api.marketplace.createListing(payload);
      setListings((current) => {
        const exists = current.some((listing) => listing.id === saved.id);
        return exists ? current.map((listing) => (listing.id === saved.id ? saved : listing)) : [saved, ...current];
      });
      setSelectedListingId(saved.id);
      setEditingListingId(saved.id);
      setMessage(t("marketplace.message.listingSaved"));
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.listing")));
    } finally {
      setActionLoading(false);
    }
  }

  async function submitListing(listingId: string) {
    setActionLoading(true);
    setMessage(null);
    setError(null);
    try {
      const submitted = await api.marketplace.submitListing(listingId);
      setListings((current) => current.map((listing) => (listing.id === submitted.id ? submitted : listing)));
      setMessage(t("marketplace.message.listingSubmitted"));
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.submit")));
    } finally {
      setActionLoading(false);
    }
  }

  async function uploadVersion() {
    if (!selectedListingId || !packageFile) return;
    setActionLoading(true);
    setMessage(null);
    setError(null);
    try {
      const submitted = await api.marketplace.uploadVersion(selectedListingId, packageFile, manifest);
      setMessage(t("marketplace.message.versionSubmitted").replace("{version}", submitted.version.version));
      setPackageFile(null);
      setListings(await api.marketplace.listings());
      await loadSubmissionReports();
      await loadReviewReports();
      await loadReviewEvents();
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.upload")));
    } finally {
      setActionLoading(false);
    }
  }

  function editListing(listing: MarketplaceListingResponse) {
    setEditingListingId(listing.id);
    setSelectedListingId(listing.id);
    setListingDraft(listingToDraft(listing));
  }

  function newListing() {
    setEditingListingId(null);
    setListingDraft(defaultListingDraft);
  }

  function reviewDraftFor(reportId: string) {
    return reviewDrafts[reportId] ?? defaultReviewDraft;
  }

  function updateReviewDraft(reportId: string, patch: Partial<ReviewDraft>) {
    setReviewDrafts((current) => ({
      ...current,
      [reportId]: {
        ...defaultReviewDraft,
        ...current[reportId],
        ...patch,
      },
    }));
  }

  function clearReviewDraft(reportId: string) {
    setReviewDrafts((current) => {
      const next = { ...current };
      delete next[reportId];
      return next;
    });
  }

  async function refreshReviewSurfaces() {
    setListings(await api.marketplace.listings());
    await loadSubmissionReports();
    await loadReviewReports();
    await loadReviewEvents();
    await loadAdminAnalytics();
  }

  async function decideSubmission(report: MarketplaceValidationReportResponse, decision: MarketplaceReviewDecision) {
    const draft = reviewDraftFor(report.submission_id);
    setActionLoading(true);
    setMessage(null);
    setError(null);
    try {
      await api.marketplace.reviewDecision(report.submission_id, {
        decision,
        internal_comment: cleanOptional(draft.internal_comment),
        creator_message: cleanOptional(draft.creator_message),
      });
      clearReviewDraft(report.submission_id);
      setMessage(t("marketplace.review.decisionSaved"));
      await refreshReviewSurfaces();
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.reviewAction")));
    } finally {
      setActionLoading(false);
    }
  }

  async function moderateListing(report: MarketplaceValidationReportResponse, action: MarketplaceModerationAction) {
    const draft = reviewDraftFor(report.submission_id);
    const reason = cleanOptional(draft.reason);
    if (!reason) {
      setError(t("marketplace.error.reasonRequired"));
      return;
    }

    const payload: MarketplaceModerationRequest = {
      action,
      reason,
      internal_comment: cleanOptional(draft.internal_comment),
      creator_message: cleanOptional(draft.creator_message),
    };
    if (action === "unpublish_version") {
      payload.version_id = report.version_id;
    }

    setActionLoading(true);
    setMessage(null);
    setError(null);
    try {
      await api.marketplace.moderateListing(report.listing_id, payload);
      clearReviewDraft(report.submission_id);
      setMessage(t("marketplace.review.moderationSaved"));
      await refreshReviewSurfaces();
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.moderation")));
    } finally {
      setActionLoading(false);
    }
  }
  function renderReportCard(report: MarketplaceValidationReportResponse, includeListing: boolean) {
    const installEligible = report.compatibility_report.install_eligible;
    const installLabel = typeof installEligible === "boolean"
      ? t(installEligible ? "marketplace.reports.installEligible" : "marketplace.reports.installBlocked")
      : t("marketplace.reports.installUnknown");
    const installTone = installEligible === false ? "danger" : installEligible === true ? "success" : "neutral";
    const draft = reviewDraftFor(report.submission_id);
    const approvalEnabled = canApproveReport(report);

    return (
      <article className="validation-report-card" key={report.submission_id}>
        <div className="validation-report-card-header">
          <div>
            <strong>{includeListing ? `${report.listing_title} v${report.version}` : `v${report.version}`}</strong>
            <span>{includeListing ? report.creator_display_name : new Date(report.submitted_at).toLocaleString()}</span>
          </div>
          <StatusBadge label={report.review_status} tone={listingTone(report.version_status)} />
        </div>
        <div className="validation-report-badges">
          <StatusBadge label={`${t("marketplace.reports.validationStatus")}: ${report.validation_status}`} tone={validationTone(report.validation_status)} />
          <StatusBadge label={`${t("marketplace.reports.securityRisk")}: ${report.security_risk_level}`} tone={riskTone(report.security_risk_level)} />
          <StatusBadge label={installLabel} tone={installTone} />
        </div>
        <pre className="validation-report-json">{formatReportJson(report.validation_report)}</pre>

        {includeListing && canReviewMarketplace && (
          <div className="marketplace-review-controls">
            <div className="review-field-grid">
              <label>
                {t("marketplace.review.internalComment")}
                <textarea
                  rows={2}
                  value={draft.internal_comment}
                  onChange={(event) => updateReviewDraft(report.submission_id, { internal_comment: event.target.value })}
                />
              </label>
              <label>
                {t("marketplace.review.creatorMessage")}
                <textarea
                  rows={2}
                  value={draft.creator_message}
                  onChange={(event) => updateReviewDraft(report.submission_id, { creator_message: event.target.value })}
                />
              </label>
            </div>
            <div className="review-action-row">
              <button className="primary-button" type="button" onClick={() => decideSubmission(report, "approve")} disabled={!approvalEnabled || actionLoading}>
                <CheckCircle2 size={16} aria-hidden="true" />
                {t("marketplace.review.approve")}
              </button>
              <button className="secondary-button" type="button" onClick={() => decideSubmission(report, "request_changes")} disabled={actionLoading}>
                <MessageSquare size={16} aria-hidden="true" />
                {t("marketplace.review.requestChanges")}
              </button>
              <button className="secondary-button button-danger" type="button" onClick={() => decideSubmission(report, "reject")} disabled={actionLoading}>
                <XCircle size={16} aria-hidden="true" />
                {t("marketplace.review.reject")}
              </button>
            </div>
            {!approvalEnabled && <span className="review-note">{t("marketplace.review.approvalBlocked")}</span>}
            <label className="review-reason-field">
              {t("marketplace.review.moderationReason")}
              <input
                value={draft.reason}
                onChange={(event) => updateReviewDraft(report.submission_id, { reason: event.target.value })}
              />
            </label>
            <div className="review-action-row moderation-actions">
              <button className="secondary-button" type="button" onClick={() => moderateListing(report, "suspend_listing")} disabled={actionLoading}>
                <AlertTriangle size={16} aria-hidden="true" />
                {t("marketplace.review.suspendListing")}
              </button>
              <button className="secondary-button" type="button" onClick={() => moderateListing(report, "unpublish_version")} disabled={actionLoading}>
                <ShieldAlert size={16} aria-hidden="true" />
                {t("marketplace.review.unpublishVersion")}
              </button>
              <button className="secondary-button button-danger" type="button" onClick={() => moderateListing(report, "emergency_block")} disabled={actionLoading}>
                <ShieldAlert size={16} aria-hidden="true" />
                {t("marketplace.review.emergencyBlock")}
              </button>
            </div>
          </div>
        )}
      </article>
    );
  }

  return (
    <div className="page-stack marketplace-page">
      <div className="toolbar toolbar--end">
        <button className="secondary-button" type="button" onClick={() => void refreshMarketplace()} disabled={loading || catalogLoading || installationsLoading}>
          <RefreshCw size={16} aria-hidden="true" />
          {t("marketplace.refresh")}
        </button>
      </div>

      {message && <StatusBadge label={message} tone="success" />}
      {error && <StatusBadge label={error} tone="danger" />}

      <section className="metric-grid" aria-label={t("marketplace.metrics.aria")}>
        <div className="metric-card">
          <Store size={20} aria-hidden="true" />
          <span>{t("marketplace.creator.status")}</span>
          <strong>{creator ? creator.status : t("marketplace.creator.none")}</strong>
        </div>
        <div className="metric-card">
          <PackagePlus size={20} aria-hidden="true" />
          <span>{t("marketplace.listings.title")}</span>
          <strong>{listings.length}</strong>
        </div>
        <div className="metric-card">
          <BadgeCheck size={20} aria-hidden="true" />
          <span>{t("marketplace.creator.reviewGate")}</span>
          <strong>{approvedCreator ? t("common.enabled") : t("common.disabled")}</strong>
        </div>
      </section>

      <section className="panel marketplace-runtime-panel">
        <div className="panel-header">
          <div>
            <h2>{t("marketplace.runtime.title")}</h2>
            <span>{t("marketplace.runtime.description")}</span>
          </div>
          <StatusBadge
            label={runtimeStatus?.global_blocked || runtimeStatus?.organization_blocked
              ? t("marketplace.runtime.blocked")
              : t("marketplace.runtime.ready")}
            tone={runtimeStatus?.global_blocked || runtimeStatus?.organization_blocked ? "danger" : "success"}
          />
        </div>
        <div className="padded">
          <p className="installation-gate-note" role="status">
            <ShieldAlert size={16} aria-hidden="true" />
            {runtimeStatus?.status_message ?? t("marketplace.runtime.loading")}
          </p>
          <div className="permission-approval-list">
            <h3>{t("marketplace.runtime.permissionCatalog")}</h3>
            {permissionCatalog.map((permission) => (
              <div key={permission.permission_key} className="catalog-permission-row">
                <span><strong>{permission.permission_key}</strong> — {permission.description}</span>
                <StatusBadge label={permission.risk_level} tone={riskTone(permission.risk_level)} />
              </div>
            ))}
          </div>
          {(canManageKillSwitch || canManageGlobalKillSwitch) && (
            <div className="installation-actions">
              {canManageKillSwitch && (
                <button className="secondary-button button-danger" type="button" onClick={() => void runKillSwitchAction("organization")} disabled={actionLoading || Boolean(runtimeStatus?.organization_blocked)}>
                  <ShieldAlert size={16} aria-hidden="true" />
                  {t("marketplace.runtime.blockOrganization")}
                </button>
              )}
              {canManageGlobalKillSwitch && (
                <button className="secondary-button button-danger" type="button" onClick={() => void runKillSwitchAction("global")} disabled={actionLoading || Boolean(runtimeStatus?.global_blocked)}>
                  <ShieldAlert size={16} aria-hidden="true" />
                  {t("marketplace.runtime.blockGlobal")}
                </button>
              )}
              {runtimeStatus?.active_kill_switches.map((killSwitch) => (
                ((killSwitch.scope === "global" && canManageGlobalKillSwitch)
                  || (killSwitch.scope === "organization" && canManageKillSwitch)) && (
                  <button className="secondary-button" type="button" key={killSwitch.id} onClick={() => void runKillSwitchAction("lift", killSwitch.id)} disabled={actionLoading}>
                    {t("marketplace.runtime.lift", { scope: killSwitch.scope })}
                  </button>
                )
              ))}
            </div>
          )}
        </div>
      </section>

      <section className="panel marketplace-runtime-panel" aria-label="Marketplace plugin hooks">
        <div className="panel-header">
          <div>
            <h2>Extension hooks</h2>
            <span>Public Phase 8 contracts exposed by active integration plugins</span>
          </div>
          <StatusBadge label={`${marketplaceHooks.length}`} tone="neutral" />
        </div>
        <div className="padded">
          {marketplaceHooks.length === 0 ? (
            <p className="empty-copy">No public extension hooks are installed.</p>
          ) : (
            <div className="permission-approval-list">
              {marketplaceHooks.map((hook) => (
                <div key={`${hook.installation_id}:${hook.hook_key}`} className="catalog-permission-row">
                  <span><strong>{hook.hook_type}</strong> — {hook.label} ({hook.listing_title})</span>
                  <StatusBadge label={hook.contract_version} tone="success" />
                </div>
              ))}
            </div>
          )}
        </div>
      </section>

      <section className="panel marketplace-catalog-panel">
        <div className="panel-header">
          <div>
            <h2>{t("marketplace.catalog.title")}</h2>
            <span>{t("marketplace.catalog.description")}</span>
          </div>
          <StatusBadge label={`${catalogItems.length}`} tone="neutral" />
        </div>
        <div className="padded marketplace-catalog-body">
          <div className="marketplace-catalog-controls">
            <label>
              {t("marketplace.catalog.search")}
              <div className="input-with-icon">
                <Search size={16} aria-hidden="true" />
                <input value={catalogFilters.search} onChange={(event) => setCatalogFilters({ ...catalogFilters, search: event.target.value })} />
              </div>
            </label>
            <label>
              {t("marketplace.catalog.categoryFilter")}
              <input list="marketplace-catalog-categories" value={catalogFilters.category} onChange={(event) => setCatalogFilters({ ...catalogFilters, category: event.target.value })} />
              <datalist id="marketplace-catalog-categories">
                {catalogCategories.map((category) => <option key={category} value={category} />)}
              </datalist>
            </label>
            <label>
              {t("marketplace.catalog.productTypeFilter")}
              <select value={catalogFilters.product_type} onChange={(event) => setCatalogFilters({ ...catalogFilters, product_type: event.target.value as CatalogFilterDraft["product_type"] })}>
                <option value="">{t("marketplace.catalog.allTypes")}</option>
                {PRODUCT_TYPES.map((type) => <option key={type} value={type}>{type}</option>)}
              </select>
            </label>
            <label>
              {t("marketplace.catalog.pricingFilter")}
              <select value={catalogFilters.pricing_type} onChange={(event) => setCatalogFilters({ ...catalogFilters, pricing_type: event.target.value as CatalogFilterDraft["pricing_type"] })}>
                <option value="">{t("marketplace.catalog.allPricing")}</option>
                {PRICING_TYPES.map((type) => <option key={type} value={type}>{type}</option>)}
              </select>
            </label>
            <button className="secondary-button" type="button" onClick={resetCatalogFilters} disabled={catalogLoading}>
              {t("marketplace.catalog.resetFilters")}
            </button>
          </div>

          <div className="marketplace-catalog-grid">
            {catalogItems.map((item) => {
              const permissions = stringListFromValue(item.permissions);
              return (
                <article className="catalog-product-card" key={item.id}>
                  <div className="catalog-product-heading">
                    <div>
                      <h3>{item.title}</h3>
                      <span>{item.creator_display_name}</span>
                    </div>
                    <strong>{catalogPriceLabel(item)}</strong>
                  </div>
                  <p>{item.summary}</p>
                  <div className="catalog-product-meta">
                    <StatusBadge label={item.badge} tone="success" />
                    <span><Tag size={14} aria-hidden="true" />{item.category}</span>
                    <span>{item.product_type}</span>
                    <span>{t("marketplace.catalog.version")}: {item.latest_version}</span>
                    <span>{t("marketplace.catalog.activeInstalls")}: {item.active_installations.toLocaleString()}</span>
                    <span><Star size={14} aria-hidden="true" />{item.rating_count > 0 ? item.rating_average.toFixed(1) : t("marketplace.catalog.noReviews")}</span>
                  </div>
                  <div className="catalog-permission-row" aria-label={t("marketplace.catalog.permissions")}>
                    {permissions.slice(0, 3).map((permission) => <span key={permission}>{permission}</span>)}
                    {permissions.length === 0 && <span>{t("marketplace.catalog.noPermissions")}</span>}
                    {permissions.length > 3 && <span>+{permissions.length - 3}</span>}
                  </div>
                  <button className="secondary-button" type="button" onClick={() => void openCatalogDetail(item)} disabled={catalogLoading}>
                    <Eye size={16} aria-hidden="true" />
                    {t("marketplace.catalog.viewDetails")}
                  </button>
                </article>
              );
            })}
            {catalogItems.length === 0 && <p className="empty-state catalog-empty-state">{t("marketplace.catalog.empty")}</p>}
          </div>
        </div>
      </section>

      {catalogDetail && (
        <section className="panel marketplace-catalog-detail">
          <div className="panel-header">
            <div>
              <h2>{catalogDetail.item.title}</h2>
              <span>{catalogDetail.item.summary}</span>
            </div>
            <StatusBadge label={catalogDetail.item.badge} tone="success" />
          </div>
          <div className="padded catalog-detail-layout">
            <div className="catalog-detail-main">
              <div className="catalog-detail-copy">
                <h3>{t("common.description")}</h3>
                <p>{catalogDetail.description}</p>
              </div>
              <div className="catalog-detail-copy">
                <h3>{t("marketplace.catalog.changelog")}</h3>
                <pre className="validation-report-json">{formatReportJson(catalogDetail.changelog)}</pre>
              </div>
              <div className="catalog-detail-copy">
                <h3>{t("marketplace.catalog.screenshots")}</h3>
                <div className="catalog-screenshot-grid">
                  {stringListFromValue(catalogDetail.screenshots).map((url) => (
                    <a key={url} href={url} target="_blank" rel="noreferrer">
                      <img src={url} alt="" />
                    </a>
                  ))}
                  {stringListFromValue(catalogDetail.screenshots).length === 0 && <span>{t("marketplace.catalog.noScreenshots")}</span>}
                </div>
              </div>
            </div>
            <aside className="catalog-detail-side">
              <div>
                <h3>{t("marketplace.catalog.permissions")}</h3>
                <div className="catalog-permission-row catalog-permission-row--detail">
                  {stringListFromValue(catalogDetail.permissions).map((permission) => <span key={permission}>{permission}</span>)}
                  {stringListFromValue(catalogDetail.permissions).length === 0 && <span>{t("marketplace.catalog.noPermissions")}</span>}
                </div>
              </div>
              <div className="catalog-detail-facts">
                <span>{t("marketplace.catalog.creator")}</span>
                <strong>{catalogDetail.item.creator_display_name}</strong>
                <span>{t("marketplace.catalog.license")}</span>
                <strong>{catalogDetail.license}</strong>
                <span>{t("marketplace.catalog.support")}</span>
                {catalogDetail.support_url ? <a href={catalogDetail.support_url} target="_blank" rel="noreferrer">{catalogDetail.support_url}</a> : <strong>-</strong>}
              </div>
              <div>
                <h3>{t("marketplace.catalog.versions")}</h3>
                <div className="catalog-version-list">
                  {catalogDetail.versions.map((version) => (
                    <article key={version.id}>
                      <strong>v{version.version}</strong>
                      <span>{new Date(version.created_at).toLocaleDateString()}</span>
                    </article>
                  ))}
                </div>
              </div>
              <div>
                <h3>{t("marketplace.catalog.reviews")}</h3>
                <div className="catalog-version-list">
                  {catalogDetail.reviews.map((review) => (
                    <article key={`${review.author}-${review.created_at ?? review.rating}`}>
                      <strong>{review.author} - {review.rating}/5</strong>
                      <span>{review.body}</span>
                    </article>
                  ))}
                  {catalogDetail.reviews.length === 0 && <p className="empty-state">{t("marketplace.catalog.noReviews")}</p>}
                </div>
              </div>
              <div className="marketplace-feedback-form">
                <h3>{t("marketplace.feedback.rateTitle")}</h3>
                <p className="review-note">
                  {canManageInstallations && (installationForListing(catalogDetail.item.id) || purchaseForListing(catalogDetail.item.id))
                    ? t("marketplace.feedback.reviewEligible")
                    : t("marketplace.feedback.reviewIneligible")}
                </p>
                <label>
                  {t("marketplace.feedback.rating")}
                  <select
                    value={customerFeedback.rating}
                    onChange={(event) => setCustomerFeedback((current) => ({ ...current, rating: Number(event.target.value) }))}
                  >
                    {[5, 4, 3, 2, 1].map((rating) => <option key={rating} value={rating}>{rating} / 5</option>)}
                  </select>
                </label>
                <label>
                  {t("marketplace.feedback.review")}
                  <textarea
                    rows={3}
                    value={customerFeedback.body}
                    onChange={(event) => setCustomerFeedback((current) => ({ ...current, body: event.target.value }))}
                    maxLength={4000}
                  />
                </label>
                <button
                  className="secondary-button"
                  type="button"
                  onClick={() => void submitCustomerReview(catalogDetail)}
                  disabled={!canManageInstallations || !(installationForListing(catalogDetail.item.id) || purchaseForListing(catalogDetail.item.id)) || customerFeedback.body.trim().length < 3 || actionLoading}
                >
                  <Star size={16} aria-hidden="true" /> {t("marketplace.feedback.submitReview")}
                </button>
              </div>
              <div className="marketplace-feedback-form">
                <h3>{t("marketplace.feedback.reportTitle")}</h3>
                <label>
                  {t("marketplace.feedback.violationType")}
                  <select value={customerFeedback.report_type} onChange={(event) => setCustomerFeedback((current) => ({ ...current, report_type: event.target.value as typeof current.report_type }))}>
                    <option value="malware">{t("marketplace.feedback.type.malware")}</option><option value="copyright">{t("marketplace.feedback.type.copyright")}</option><option value="spam">{t("marketplace.feedback.type.spam")}</option><option value="fraud">{t("marketplace.feedback.type.fraud")}</option><option value="privacy">{t("marketplace.feedback.type.privacy")}</option><option value="other">{t("marketplace.feedback.type.other")}</option>
                  </select>
                </label>
                <label>
                  {t("marketplace.feedback.severity")}
                  <select value={customerFeedback.severity} onChange={(event) => setCustomerFeedback((current) => ({ ...current, severity: event.target.value as typeof current.severity }))}>
                    <option value="low">{t("marketplace.feedback.severity.low")}</option><option value="medium">{t("marketplace.feedback.severity.medium")}</option><option value="high">{t("marketplace.feedback.severity.high")}</option><option value="critical">{t("marketplace.feedback.severity.critical")}</option>
                  </select>
                </label>
                <label>
                  {t("common.description")}
                  <textarea rows={3} value={customerFeedback.description} onChange={(event) => setCustomerFeedback((current) => ({ ...current, description: event.target.value }))} maxLength={4000} />
                </label>
                <label>
                  {t("marketplace.feedback.evidence")}
                  <textarea rows={2} value={customerFeedback.evidence} onChange={(event) => setCustomerFeedback((current) => ({ ...current, evidence: event.target.value }))} />
                </label>
                <button className="secondary-button button-danger" type="button" onClick={() => void submitAbuseReport(catalogDetail)} disabled={customerFeedback.description.trim().length < 10 || actionLoading}>
                  <ShieldAlert size={16} aria-hidden="true" /> {t("marketplace.feedback.submitReport")}
                </button>
              </div>
              {installRestriction(catalogDetail) && (
                <p className="installation-gate-note" role="status">
                  <AlertTriangle size={16} aria-hidden="true" />
                  {installRestriction(catalogDetail)}
                </p>
              )}
              {catalogDetail.item.pricing_type === "paid" && !purchaseForListing(catalogDetail.item.id) && (
                <button
                  className="primary-button"
                  type="button"
                  onClick={() => void purchaseProduct(catalogDetail)}
                  disabled={!canManageInstallations || actionLoading}
                >
                  <BadgeCheck size={16} aria-hidden="true" />
                  {t("marketplace.purchase.action", { price: catalogPriceLabel(catalogDetail.item) })}
                </button>
              )}
              <button
                className="primary-button"
                type="button"
                onClick={() => openInstallApproval(catalogDetail)}
                disabled={Boolean(installRestriction(catalogDetail)) || actionLoading}
              >
                <PackagePlus size={16} aria-hidden="true" />
                {t("marketplace.install.openApproval")}
              </button>
            </aside>
          </div>
        </section>
      )}

      {installTarget && (
        <div className="marketplace-dialog-backdrop" role="presentation">
          <section
            className="marketplace-install-dialog"
            role="dialog"
            aria-modal="true"
            aria-labelledby="marketplace-install-title"
          >
            <div className="panel-header">
              <div>
                <h2 id="marketplace-install-title">{t("marketplace.install.title", { title: installTarget.item.title })}</h2>
                <span>{t("marketplace.install.description")}</span>
              </div>
              <button className="icon-button" type="button" onClick={closeInstallApproval} aria-label={t("common.close")} disabled={actionLoading}>
                <X size={18} aria-hidden="true" />
              </button>
            </div>
            <div className="padded marketplace-install-dialog-body">
              <div className="install-summary-grid">
                <div>
                  <span>{t("marketplace.catalog.version")}</span>
                  <strong>v{installTarget.item.latest_version}</strong>
                </div>
                <div>
                  <span>{t("marketplace.install.organizationRole")}</span>
                  <strong>{activeOrganizationRole ?? "-"}</strong>
                </div>
                <div>
                  <span>{t("marketplace.install.compatibility")}</span>
                  <StatusBadge label={t("marketplace.install.compatible")} tone="success" />
                </div>
              </div>
              <div className="permission-approval-list">
                <h3>{t("marketplace.install.permissionsTitle")}</h3>
                <p>{t("marketplace.install.permissionsDescription")}</p>
                {stringListFromValue(installTarget.permissions).map((permission) => (
                  <label key={permission}>
                    <input
                      type="checkbox"
                      checked={approvedInstallPermissions.includes(permission)}
                      onChange={() => toggleInstallPermission(permission)}
                    />
                    <span>{permission}</span>
                  </label>
                ))}
                {stringListFromValue(installTarget.permissions).length === 0 && (
                  <p className="empty-state">{t("marketplace.catalog.noPermissions")}</p>
                )}
              </div>
              <label className="installation-confirmation">
                <input type="checkbox" checked={installConfirmed} onChange={(event) => setInstallConfirmed(event.target.checked)} />
                <span>{t("marketplace.install.confirm")}</span>
              </label>
              <div className="toolbar toolbar--end">
                <button className="secondary-button" type="button" onClick={closeInstallApproval} disabled={actionLoading}>
                  {t("common.cancel")}
                </button>
                <button
                  className="primary-button"
                  type="button"
                  onClick={() => void installProduct()}
                  disabled={
                    actionLoading
                    || !installConfirmed
                    || !hasEveryPermission(
                      approvedInstallPermissions,
                      stringListFromValue(installTarget.permissions),
                    )
                  }
                >
                  <PackagePlus size={16} aria-hidden="true" />
                  {t("marketplace.install.confirmAction")}
                </button>
              </div>
            </div>
          </section>
        </div>
      )}

      <section className="panel marketplace-installations-panel">
        <div className="panel-header">
          <div>
            <h2>{t("marketplace.installations.title")}</h2>
            <span>{t("marketplace.installations.description")}</span>
          </div>
          <StatusBadge label={`${installations.length}`} tone="neutral" />
        </div>
        {!canManageInstallations && (
          <div className="installation-role-note">
            <ShieldAlert size={18} aria-hidden="true" />
            <span>{t("marketplace.installations.roleNotice")}</span>
          </div>
        )}
        <div className="padded marketplace-installations-grid">
          {installations.map((installation) => {
            const update = installationUpdates[installation.id];
            const currentPermissions = stringListFromValue(installation.permissions);
            const targetPermissions = stringListFromValue(update?.permissions);
            const lifecycleAllowsVersionChange = installation.status === "active" || installation.status === "disabled";
            const addedPermissions = targetPermissions.filter((permission) => !currentPermissions.includes(permission));
            const removedPermissions = currentPermissions.filter((permission) => !targetPermissions.includes(permission));
            const updatePermissionsApproved = hasEveryPermission(
              approvedUpdatePermissions[installation.id] ?? [],
              targetPermissions,
            );
            const canUpdate = Boolean(
              canManageInstallations
              && update?.update_available
              && update.target_version_id
              && confirmedUpdates[installation.id]
              && (
                !update.permission_reapproval_required
                || (updatePermissionsApproved && confirmedUpdatePermissions[installation.id])
              ),
            );

            return (
              <article className="marketplace-installation-card" key={installation.id}>
                <div className="installation-card-heading">
                  <div>
                    <h3>{installation.listing_title}</h3>
                    <span>{installation.product_type}</span>
                  </div>
                  <StatusBadge label={installation.status} tone={installationTone(installation.status)} />
                </div>
                <div className="installation-facts">
                  <span>{t("marketplace.installations.installedVersion")}</span>
                  <strong>v{installation.installed_version}</strong>
                  <span>{t("marketplace.installations.versionPinned")}</span>
                  <strong>{t(installation.version_pinned ? "common.yes" : "common.no")}</strong>
                  <span>{t("marketplace.installations.cleanupPolicy")}</span>
                  <strong>{installation.cleanup_policy}</strong>
                  <span>{t("marketplace.installations.installedAt")}</span>
                  <strong>{new Date(installation.installed_at).toLocaleString()}</strong>
                </div>
                <div>
                  <h4>{t("marketplace.catalog.permissions")}</h4>
                  <div className="catalog-permission-row">
                    {currentPermissions.map((permission) => <span key={permission}>{permission}</span>)}
                    {currentPermissions.length === 0 && <span>{t("marketplace.catalog.noPermissions")}</span>}
                  </div>
                </div>
                <div className="installation-actions">
                  {installation.status === "disabled" && (
                    <button className="secondary-button" type="button" onClick={() => void runInstallationAction(installation, "enable")} disabled={!canManageInstallations || actionLoading}>
                      <Power size={16} aria-hidden="true" />
                      {t("marketplace.installations.enable")}
                    </button>
                  )}
                  {installation.status === "active" && (
                    <button className="secondary-button" type="button" onClick={() => void runInstallationAction(installation, "disable")} disabled={!canManageInstallations || actionLoading}>
                      <Power size={16} aria-hidden="true" />
                      {t("marketplace.installations.disable")}
                    </button>
                  )}
                  {lifecycleAllowsVersionChange && (
                    <button className="secondary-button" type="button" onClick={() => void checkInstallationUpdates(installation)} disabled={!canManageInstallations || actionLoading}>
                      <RefreshCw size={16} aria-hidden="true" />
                      {t("marketplace.installations.checkUpdates")}
                    </button>
                  )}
                  {installation.rollback_version_id && lifecycleAllowsVersionChange && (
                    <button className="secondary-button" type="button" onClick={() => void runInstallationAction(installation, "rollback")} disabled={!canManageInstallations || actionLoading}>
                      <RotateCcw size={16} aria-hidden="true" />
                      {t("marketplace.installations.rollback", { version: installation.rollback_version ?? "" })}
                    </button>
                  )}
                  {installation.status !== "uninstalled" && (
                    <button className="secondary-button button-danger" type="button" onClick={() => setUninstallConfirmId(installation.id)} disabled={!canManageInstallations || actionLoading}>
                      <Trash2 size={16} aria-hidden="true" />
                      {t("marketplace.installations.uninstall")}
                    </button>
                  )}
                </div>

                {uninstallConfirmId === installation.id && (
                  <div className="installation-confirm-box" role="alert">
                    <AlertTriangle size={18} aria-hidden="true" />
                    <div>
                      <strong>{t("marketplace.installations.uninstallConfirmTitle")}</strong>
                      <span>{t("marketplace.installations.uninstallConfirmDescription", { policy: installation.cleanup_policy })}</span>
                    </div>
                    <button className="secondary-button" type="button" onClick={() => setUninstallConfirmId(null)} disabled={actionLoading}>{t("common.cancel")}</button>
                    <button className="secondary-button button-danger" type="button" onClick={() => void runInstallationAction(installation, "uninstall")} disabled={actionLoading}>{t("marketplace.installations.confirmUninstall")}</button>
                  </div>
                )}

                {update && (
                  <div className="installation-update-panel">
                    <div className="installation-update-heading">
                      <div>
                        <h4>{t("marketplace.installations.updateTitle")}</h4>
                        <span>{t("marketplace.installations.currentVersion", { version: update.current_version })}</span>
                      </div>
                      {update.version_pinned && <StatusBadge label={t("marketplace.installations.pinned")} tone="warning" />}
                    </div>
                    {!update.update_available ? (
                      <div className="installation-update-empty">
                        <CheckCircle2 size={18} aria-hidden="true" />
                        <span>{t("marketplace.installations.upToDate")}</span>
                        {update.reasons.map((reason) => <small key={reason}>{reason}</small>)}
                      </div>
                    ) : (
                      <>
                        <label className="installation-version-select">
                          {t("marketplace.installations.targetVersion")}
                          <select value={update.target_version_id ?? ""} disabled>
                            <option value={update.target_version_id ?? ""}>v{update.target_version}</option>
                          </select>
                        </label>
                        <div>
                          <h4>{t("marketplace.catalog.changelog")}</h4>
                          <pre className="validation-report-json">{formatReportJson(update.changelog)}</pre>
                        </div>
                        {(addedPermissions.length > 0 || removedPermissions.length > 0) && (
                          <div className="permission-change-summary">
                            <h4>{t("marketplace.installations.permissionChanges")}</h4>
                            {addedPermissions.map((permission) => <span className="permission-added" key={`added-${permission}`}>+ {permission}</span>)}
                            {removedPermissions.map((permission) => <span className="permission-removed" key={`removed-${permission}`}>- {permission}</span>)}
                          </div>
                        )}
                        {update.permission_reapproval_required && (
                          <div className="permission-approval-list permission-approval-list--update">
                            <h4>{t("marketplace.installations.reapprovalTitle")}</h4>
                            <p>{t("marketplace.installations.reapprovalDescription")}</p>
                            {targetPermissions.map((permission) => (
                              <label key={permission}>
                                <input
                                  type="checkbox"
                                  checked={(approvedUpdatePermissions[installation.id] ?? []).includes(permission)}
                                  onChange={() => toggleUpdatePermission(installation.id, permission)}
                                />
                                <span>{permission}</span>
                              </label>
                            ))}
                            <label className="installation-confirmation">
                              <input
                                type="checkbox"
                                checked={confirmedUpdatePermissions[installation.id] ?? false}
                                onChange={(event) => setConfirmedUpdatePermissions((current) => ({
                                  ...current,
                                  [installation.id]: event.target.checked,
                                }))}
                              />
                              <span>{t("marketplace.installations.reapprovalConfirm")}</span>
                            </label>
                          </div>
                        )}
                        <label className="installation-confirmation">
                          <input
                            type="checkbox"
                            checked={confirmedUpdates[installation.id] ?? false}
                            onChange={(event) => setConfirmedUpdates((current) => ({ ...current, [installation.id]: event.target.checked }))}
                          />
                          <span>{t("marketplace.installations.confirmChangelog", { version: update.target_version ?? "" })}</span>
                        </label>
                        <button className="primary-button" type="button" onClick={() => void updateInstallation(installation)} disabled={!canUpdate || actionLoading}>
                          <History size={16} aria-hidden="true" />
                          {t("marketplace.installations.updateAction", { version: update.target_version ?? "" })}
                        </button>
                      </>
                    )}
                  </div>
                )}
              </article>
            );
          })}
          {installations.length === 0 && <p className="empty-state">{t("marketplace.installations.empty")}</p>}
        </div>
      </section>

      <section className="two-column-workspace marketplace-workspace">
        <div className="panel">
          <div className="panel-header">
            <div>
              <h2>{t("marketplace.creator.title")}</h2>
              <span>{t("marketplace.creator.description")}</span>
            </div>
            <StatusBadge label={creator?.status ?? t("marketplace.creator.none")} tone={creatorTone(creator?.status)} />
          </div>
          <div className="padded form-grid">
            <label>
              {t("common.slug")}
              <input value={creatorDraft.slug} onChange={(event) => setCreatorDraft({ ...creatorDraft, slug: event.target.value })} />
            </label>
            <label>
              {t("marketplace.creator.displayName")}
              <input value={creatorDraft.display_name} onChange={(event) => setCreatorDraft({ ...creatorDraft, display_name: event.target.value })} />
            </label>
            <label>
              {t("marketplace.creator.supportEmail")}
              <input value={creatorDraft.support_email} onChange={(event) => setCreatorDraft({ ...creatorDraft, support_email: event.target.value })} />
            </label>
            <label>
              {t("marketplace.creator.bio")}
              <textarea rows={4} value={creatorDraft.bio} onChange={(event) => setCreatorDraft({ ...creatorDraft, bio: event.target.value })} />
            </label>
            <button className="primary-button" type="button" onClick={saveCreator} disabled={actionLoading}>
              <Save size={16} aria-hidden="true" />
              {t("marketplace.creator.save")}
            </button>
            {creator && (
              <button className="secondary-button" type="button" onClick={() => void startPayoutOnboarding()} disabled={!approvedCreator || actionLoading}>
                <BadgeCheck size={16} aria-hidden="true" />
                {t("marketplace.payout.action", { status: creator.payout_status })}
              </button>
            )}
            {creatorBalance && (
              <div className="form-grid form-grid--inline">
                <span>{t("marketplace.payout.available", { amount: `${(creatorBalance.available_cents / 100).toFixed(2)} ${creatorBalance.currency.toUpperCase()}` })}</span>
                <span>{t("marketplace.payout.pendingBalance", { days: creatorBalance.settlement_days })}</span>
                <button className="secondary-button" type="button" onClick={() => void requestCreatorPayout()} disabled={!approvedCreator || creator?.payout_status !== "verified" || actionLoading}>
                  {t("marketplace.payout.request")}
                </button>
              </div>
            )}
          </div>
        </div>

        <div className="panel">
          <div className="panel-header">
            <div>
              <h2>{t("marketplace.listingForm.title")}</h2>
              <span>{t("marketplace.listingForm.description")}</span>
            </div>
            <button className="secondary-button" type="button" onClick={newListing}>
              {t("common.new")}
            </button>
          </div>
          <div className="padded form-grid">
            <label>
              {t("common.title")}
              <input value={listingDraft.title} onChange={(event) => setListingDraft({ ...listingDraft, title: event.target.value })} />
            </label>
            <label>
              {t("common.slug")}
              <input value={listingDraft.slug} onChange={(event) => setListingDraft({ ...listingDraft, slug: event.target.value })} />
            </label>
            <label>
              {t("common.type")}
              <select value={listingDraft.product_type} onChange={(event) => setListingDraft({ ...listingDraft, product_type: event.target.value as MarketplaceProductType })}>
                {PRODUCT_TYPES.map((type) => <option key={type} value={type}>{type}</option>)}
              </select>
            </label>
            <label>
              {t("marketplace.listing.category")}
              <input value={listingDraft.category} onChange={(event) => setListingDraft({ ...listingDraft, category: event.target.value })} />
            </label>
            <label>
              {t("marketplace.listing.summary")}
              <input value={listingDraft.summary} onChange={(event) => setListingDraft({ ...listingDraft, summary: event.target.value })} />
            </label>
            <label>
              {t("common.description")}
              <textarea rows={5} value={listingDraft.description} onChange={(event) => setListingDraft({ ...listingDraft, description: event.target.value })} />
            </label>
            <div className="form-grid form-grid--inline">
              <label>
                {t("marketplace.listing.pricing")}
                <select value={listingDraft.pricing_type} onChange={(event) => setListingDraft({ ...listingDraft, pricing_type: event.target.value as MarketplacePricingType })}>
                  {PRICING_TYPES.map((type) => <option key={type} value={type}>{type}</option>)}
                </select>
              </label>
              <label>
                {t("marketplace.listing.priceCents")}
                <input type="number" min="0" value={listingDraft.price_cents} onChange={(event) => setListingDraft({ ...listingDraft, price_cents: Number(event.target.value) })} />
              </label>
            </div>
            <label>
              {t("marketplace.listing.license")}
              <input value={listingDraft.license} onChange={(event) => setListingDraft({ ...listingDraft, license: event.target.value })} />
            </label>
            <label>
              {t("marketplace.listing.supportUrl")}
              <input value={listingDraft.support_url} onChange={(event) => setListingDraft({ ...listingDraft, support_url: event.target.value })} />
            </label>
            <label>
              {t("marketplace.listing.screenshots")}
              <textarea rows={3} value={listingDraft.screenshots} onChange={(event) => setListingDraft({ ...listingDraft, screenshots: event.target.value })} />
            </label>
            <button className="primary-button" type="button" onClick={saveListing} disabled={!creator || actionLoading}>
              <Save size={16} aria-hidden="true" />
              {editingListingId ? t("marketplace.listing.update") : t("marketplace.listing.save")}
            </button>
          </div>
        </div>
      </section>

      {creatorAnalytics && (
        <section className="panel">
          <div className="panel-header">
            <div>
              <h2>{t("marketplace.analytics.creatorTitle")}</h2>
              <span>{t("marketplace.analytics.creatorDescription")}</span>
            </div>
            <BarChart3 size={18} aria-hidden="true" />
          </div>
          <div className="padded validation-report-list">
            <div className="metric-grid" aria-label={t("marketplace.metrics.aria")}>
              <article className="metric-card metric-card--strong">
                <span>{t("marketplace.analytics.listings")}</span>
                <strong>{creatorAnalytics.listing_count}</strong>
              </article>
              <article className="metric-card metric-card--strong">
                <span>{t("marketplace.analytics.installs")}</span>
                <strong>{creatorAnalytics.active_installs}/{creatorAnalytics.total_installs}</strong>
              </article>
              <article className="metric-card metric-card--strong">
                <span>{t("marketplace.analytics.revenue")}</span>
                <strong>{formatAnalyticsMoney(creatorAnalytics.creator_revenue_cents)}</strong>
              </article>
              <article className="metric-card metric-card--strong">
                <span>{t("marketplace.analytics.conversion")}</span>
                <strong>{formatAnalyticsRate(creatorAnalytics.conversion_rate)}</strong>
              </article>
              <article className="metric-card metric-card--strong">
                <span>{t("marketplace.analytics.errors")}</span>
                <strong>{creatorAnalytics.error_count}</strong>
              </article>
            </div>
            <div className="validation-report-list">
              {creatorAnalytics.products.map((product) => (
                <article className="validation-report-card" key={product.listing_id}>
                  <div className="validation-report-card-header">
                    <div>
                      <strong>{product.title}</strong>
                      <span>{product.product_type} · {product.pricing_type}</span>
                    </div>
                    <StatusBadge label={product.status} tone={listingTone(product.status)} />
                  </div>
                  <div className="validation-report-badges">
                    <StatusBadge label={t("marketplace.analytics.productInstalls", { active: product.active_installs, total: product.total_installs })} tone="neutral" />
                    <StatusBadge label={t("marketplace.analytics.productRevenue", { amount: formatAnalyticsMoney(product.creator_revenue_cents) })} tone="success" />
                    <StatusBadge label={t("marketplace.analytics.productConversion", { rate: formatAnalyticsRate(product.conversion_rate) })} tone="neutral" />
                    <StatusBadge label={t("marketplace.analytics.productErrors", { count: product.error_count, reports: product.report_count })} tone={product.error_count > 0 ? "warning" : "success"} />
                    <StatusBadge label={t("marketplace.analytics.productRating", { rating: product.average_rating.toFixed(1), count: product.rating_count })} tone="neutral" />
                  </div>
                </article>
              ))}
              {creatorAnalytics.products.length === 0 && <p className="empty-state">{t("marketplace.analytics.noProducts")}</p>}
            </div>
          </div>
        </section>
      )}

      <section className="panel">
        <div className="panel-header">
          <div>
            <h2>{t("marketplace.listings.title")}</h2>
            <span>{t("marketplace.listings.description")}</span>
          </div>
        </div>
        <div className="marketplace-listings-table">
          <table className="data-table">
            <thead>
              <tr>
                <th>{t("common.title")}</th>
                <th>{t("common.type")}</th>
                <th>{t("common.status")}</th>
                <th>{t("marketplace.listing.priceCents")}</th>
                <th>{t("common.actions")}</th>
              </tr>
            </thead>
            <tbody>
              {listings.map((listing) => (
                <tr key={listing.id}>
                  <td>{listing.title}</td>
                  <td>{listing.product_type}</td>
                  <td><StatusBadge label={listing.status} tone={listingTone(listing.status)} /></td>
                  <td>{listing.price_cents}</td>
                  <td>
                    <div className="toolbar">
                      <button className="secondary-button" type="button" onClick={() => editListing(listing)}>
                        {t("common.edit")}
                      </button>
                      <button className="secondary-button" type="button" onClick={() => submitListing(listing.id)} disabled={!approvedCreator || actionLoading}>
                        <Send size={16} aria-hidden="true" />
                        {t("common.submit")}
                      </button>
                    </div>
                  </td>
                </tr>
              ))}
              {listings.length === 0 && (
                <tr>
                  <td colSpan={5}>{t("marketplace.listings.empty")}</td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
      </section>

      <section className="panel">
        <div className="panel-header">
          <div>
            <h2>{t("marketplace.version.title")}</h2>
            <span>{selectedListing ? selectedListing.title : t("marketplace.version.noListing")}</span>
          </div>
        </div>
        <div className="padded form-grid">
          <label>
            {t("marketplace.version.listing")}
            <select value={selectedListingId} onChange={(event) => setSelectedListingId(event.target.value)}>
              <option value="">{t("common.select")}</option>
              {listings.map((listing) => <option key={listing.id} value={listing.id}>{listing.title}</option>)}
            </select>
          </label>
          <label className="file-picker">
            <span className="secondary-button">
              <Upload size={16} aria-hidden="true" />
              {t("marketplace.version.file")}
            </span>
            <input className="file-picker-input" type="file" accept=".zip,application/zip" onChange={(event) => setPackageFile(event.target.files?.[0] ?? null)} />
            <span>{packageFile ? packageFile.name : t("media.noFileChosen")}</span>
          </label>
          <label>
            {t("marketplace.version.manifest")}
            <textarea className="code-editor" rows={12} value={manifest} onChange={(event) => setManifest(event.target.value)} />
          </label>
          <button className="primary-button" type="button" onClick={uploadVersion} disabled={!approvedCreator || !selectedListingId || !packageFile || actionLoading}>
            <Upload size={16} aria-hidden="true" />
            {t("marketplace.version.upload")}
          </button>
        </div>
      </section>

      <section className="two-column-workspace marketplace-report-grid">
        <div className="panel">
          <div className="panel-header">
            <div>
              <h2>{t("marketplace.reports.creatorTitle")}</h2>
              <span>{t("marketplace.reports.creatorDescription")}</span>
            </div>
            <FileCheck size={18} aria-hidden="true" />
          </div>
          <div className="validation-report-list padded">
            {submissionReports.map((report) => renderReportCard(report, false))}
            {submissionReports.length === 0 && <p className="empty-state">{t("marketplace.reports.empty")}</p>}
          </div>
        </div>

        {canReviewMarketplace && (
          <div className="panel">
            <div className="panel-header">
              <div>
                <h2>{t("marketplace.reports.reviewerTitle")}</h2>
                <span>{t("marketplace.reports.reviewerDescription")}</span>
              </div>
              <ShieldAlert size={18} aria-hidden="true" />
            </div>
            <div className="validation-report-list padded">
              {reviewReports.map((report) => renderReportCard(report, true))}
              {reviewReports.length === 0 && <p className="empty-state">{t("marketplace.reports.empty")}</p>}
            </div>
          </div>
        )}
      </section>

      {canReviewMarketplace && adminAnalytics && (
        <section className="panel">
          <div className="panel-header">
            <div>
              <h2>{t("marketplace.analytics.adminTitle")}</h2>
              <span>{t("marketplace.analytics.adminDescription")}</span>
            </div>
            <ShieldAlert size={18} aria-hidden="true" />
          </div>
          <div className="padded validation-report-list">
            <div className="metric-grid" aria-label={t("marketplace.metrics.aria")}>
              <article className="metric-card metric-card--strong">
                <span>{t("marketplace.analytics.submissionRate")}</span>
                <strong>{adminAnalytics.submission_rate_per_day.toFixed(1)}/day</strong>
              </article>
              <article className="metric-card metric-card--strong">
                <span>{t("marketplace.analytics.approvalTime")}</span>
                <strong>{adminAnalytics.average_approval_hours.toFixed(1)}h</strong>
              </article>
              <article className="metric-card metric-card--strong">
                <span>{t("marketplace.analytics.installs")}</span>
                <strong>{adminAnalytics.active_installs}/{adminAnalytics.total_installs}</strong>
              </article>
              <article className="metric-card metric-card--strong">
                <span>{t("marketplace.analytics.refunds")}</span>
                <strong>{adminAnalytics.refund_count}</strong>
              </article>
              <article className="metric-card metric-card--strong">
                <span>{t("marketplace.analytics.reports")}</span>
                <strong>{adminAnalytics.report_count}</strong>
              </article>
              <article className="metric-card metric-card--strong">
                <span>{t("marketplace.analytics.blockedPackages")}</span>
                <strong>{adminAnalytics.blocked_package_count}</strong>
              </article>
            </div>
            <div className="validation-report-list">
              <strong>{t("marketplace.analytics.riskyProducts")}</strong>
              {adminAnalytics.risky_products.map((product) => (
                <article className="validation-report-card" key={product.listing_id}>
                  <div className="validation-report-card-header">
                    <div>
                      <strong>{product.title}</strong>
                      <span>{t("marketplace.analytics.riskCreator", { creator: product.creator_display_name })}</span>
                    </div>
                    <StatusBadge label={product.security_risk_level} tone={riskTone(product.security_risk_level)} />
                  </div>
                  <div className="validation-report-badges">
                    <StatusBadge label={product.status} tone={listingTone(product.status)} />
                    <StatusBadge label={t("marketplace.analytics.riskReports", { count: product.report_count, critical: product.critical_report_count })} tone={product.critical_report_count > 0 ? "danger" : "warning"} />
                    <StatusBadge label={t("marketplace.analytics.riskBlocked", { count: product.blocked_package_count })} tone={product.blocked_package_count > 0 ? "danger" : "neutral"} />
                    <StatusBadge label={t("marketplace.analytics.riskRefunds", { count: product.refund_count })} tone="warning" />
                    <StatusBadge label={t("marketplace.analytics.activeInstalls", { count: product.active_installs })} tone="neutral" />
                  </div>
                </article>
              ))}
              {adminAnalytics.risky_products.length === 0 && <p className="empty-state">{t("marketplace.analytics.riskEmpty")}</p>}
            </div>
          </div>
        </section>
      )}

      {canReviewMarketplace && (
        <section className="two-column-workspace marketplace-report-grid">
          <div className="panel">
            <div className="panel-header">
              <div>
                <h2>{t("marketplace.feedback.reviewQueueTitle")}</h2>
                <span>{t("marketplace.feedback.reviewQueueDescription")}</span>
              </div>
              <Star size={18} aria-hidden="true" />
            </div>
            <div className="validation-report-list padded">
              {customerReviewQueue.map((review) => (
                <article className="validation-report-card" key={review.id}>
                  <div className="validation-report-card-header">
                    <div><strong>{review.author} · {review.rating}/5</strong><span>{review.listing_id}</span></div>
                    <StatusBadge label={review.status} tone="neutral" />
                  </div>
                  <p>{review.body}</p>
                  <label>
                    {t("marketplace.feedback.moderationReason")}
                    <textarea
                      aria-label={`${t("marketplace.feedback.moderationReason")} ${review.author}`}
                      rows={2}
                      value={customerReviewReasons[review.id] ?? ""}
                      onChange={(event) => setCustomerReviewReasons((current) => ({ ...current, [review.id]: event.target.value }))}
                    />
                  </label>
                  <div className="review-action-row moderation-actions">
                    <button className="secondary-button" type="button" disabled={actionLoading} onClick={() => void moderateCustomerReview(review.id, "published")}>{t("marketplace.feedback.publishReview")}</button>
                    <button className="secondary-button button-danger" type="button" disabled={actionLoading} onClick={() => void moderateCustomerReview(review.id, "rejected")}>{t("marketplace.feedback.rejectReview")}</button>
                  </div>
                </article>
              ))}
              {customerReviewQueue.length === 0 && <p className="empty-state">{t("marketplace.feedback.reviewQueueEmpty")}</p>}
            </div>
          </div>

          <div className="panel">
            <div className="panel-header">
              <div>
                <h2>{t("marketplace.feedback.abuseQueueTitle")}</h2>
                <span>{t("marketplace.feedback.abuseQueueDescription")}</span>
              </div>
              <ShieldAlert size={18} aria-hidden="true" />
            </div>
            <div className="validation-report-list padded">
              {abuseReportQueue.map((report) => (
                <article className="validation-report-card" key={report.id}>
                  <div className="validation-report-card-header">
                    <div><strong>{report.report_type} · {report.severity}</strong><span>{report.listing_id}</span></div>
                    <StatusBadge label={report.status} tone={report.severity === "critical" ? "danger" : "neutral"} />
                  </div>
                  <p>{report.description}</p>
                  <label>
                    {t("marketplace.feedback.resolutionNote")}
                    <textarea
                      aria-label={`${t("marketplace.feedback.resolutionNote")} ${report.id}`}
                      rows={2}
                      value={abuseResolutionNotes[report.id] ?? ""}
                      onChange={(event) => setAbuseResolutionNotes((current) => ({ ...current, [report.id]: event.target.value }))}
                    />
                  </label>
                  <div className="review-action-row moderation-actions">
                    <button className="secondary-button" type="button" disabled={actionLoading} onClick={() => void resolveCustomerAbuseReport(report.id, "investigating")}>{t("marketplace.feedback.investigateReport")}</button>
                    <button className="secondary-button" type="button" disabled={actionLoading} onClick={() => void resolveCustomerAbuseReport(report.id, "resolved")}>{t("marketplace.feedback.resolveReport")}</button>
                    <button className="secondary-button button-danger" type="button" disabled={actionLoading} onClick={() => void resolveCustomerAbuseReport(report.id, "dismissed")}>{t("marketplace.feedback.dismissReport")}</button>
                  </div>
                </article>
              ))}
              {abuseReportQueue.length === 0 && <p className="empty-state">{t("marketplace.feedback.abuseQueueEmpty")}</p>}
            </div>
          </div>
        </section>
      )}

      {canReviewMarketplace && (
        <section className="panel marketplace-review-events">
          <div className="panel-header">
            <div>
              <h2>{t("marketplace.review.eventsTitle")}</h2>
              <span>{t("marketplace.review.eventsDescription")}</span>
            </div>
            <button className="secondary-button" type="button" onClick={loadReviewEvents} disabled={actionLoading}>
              <RefreshCw size={16} aria-hidden="true" />
              {t("common.refresh")}
            </button>
          </div>
          <div className="review-event-list padded">
            {reviewEvents.map((event) => (
              <article className="review-event-card" key={event.id}>
                <div>
                  <strong>{event.listing_title}{event.version ? ` v${event.version}` : ""}</strong>
                  <span>{new Date(event.created_at).toLocaleString()}</span>
                </div>
                <StatusBadge label={event.action} tone={eventTone(event.action)} />
                <p>{event.reason}</p>
                <small>{event.actor_email ?? t("marketplace.review.unknownActor")} - {event.previous_status ?? "-"} {"->"} {event.next_status}</small>
              </article>
            ))}
            {reviewEvents.length === 0 && <p className="empty-state">{t("marketplace.review.noEvents")}</p>}
          </div>
        </section>
      )}
    </div>
  );
}
