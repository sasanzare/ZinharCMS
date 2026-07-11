import type {
  ApiInfo,
  AuditLogResponse,
  AuthResponse,
  BillingUsageResponse,
  BetaDashboardResponse,
  BetaFeedbackRequest,
  BetaFeedbackResponse,
  BetaGaBlockerRequest,
  BetaGaBlockerResponse,
  BetaParticipantRequest,
  BetaParticipantResponse,
  BetaProductDashboardResponse,
  UpdateBetaFeedbackRequest,
  UpdateBetaGaBlockerRequest,
  ChangePlanRequest,
  CheckoutSessionRequest,
  CheckoutSessionResponse,
  CustomerPortalResponse,
  AcceptInvitationRequest,
  CreateOrganizationRequest,
  CreatedInvitationResponse,
  InviteMemberRequest,
  CommentEntityType,
  CommentRequest,
  CommentResponse,
  ComponentRegistryResponse,
  MarketplaceComponentResponse,
  MarketplaceHookAuthorizationResponse,
  MarketplaceHookResponse,
  ContentEntryResponse,
  ContentTypeResponse,
  EntryListResponse,
  FieldSchemaDocument,
  HealthResponse,
  JsonRecord,
  MeResponse,
  EmailDeliveryResponse,
  OrganizationDetailResponse,
  OrganizationDomainRequest,
  OrganizationDomainResponse,
  OrganizationInvitationResponse,
  OrganizationMemberResponse,
  OrganizationMembership,
  OrganizationWorkspaceResponse,
  MediaDetailResponse,
  MarketplaceCatalogDetailResponse,
  MarketplaceCatalogItemResponse,
  MarketplaceProductReviewRequest,
  MarketplaceProductReviewModerationRequest,
  MarketplaceProductReviewResponse,
  MarketplaceProductReviewListResponse,
  MarketplaceAbuseReportRequest,
  MarketplaceAbuseReportResolutionRequest,
  MarketplaceAbuseReportResponse,
  MarketplaceCreatorRequest,
  MarketplaceCreatorStateResponse,
  MarketplaceCreatorResponse,
  MarketplaceCheckoutResponse,
  MarketplaceCreatorBalanceResponse,
  MarketplaceInstallRequest,
  MarketplaceKillSwitchResponse,
  MarketplaceInstallationResponse,
  MarketplaceInstallationUpdateCheckResponse,
  MarketplaceInstallationUpdateRequest,
  MarketplaceListingRequest,
  MarketplaceListingResponse,
  MarketplaceModerationRequest,
  MarketplaceReviewDecisionRequest,
  MarketplaceReviewEventResponse,
  MarketplacePermissionCatalogResponse,
  MarketplacePayoutAccountResponse,
  MarketplacePayoutResponse,
  MarketplacePurchaseResponse,
  MarketplaceRuntimeAuthorizeRequest,
  MarketplaceRuntimeAuthorizationResponse,
  MarketplaceRuntimeStatusResponse,
  MarketplaceVersionSubmissionResponse,
  MarketplaceValidationReportResponse,
  MediaListResponse,
  PageJson,
  PageListResponse,
  PageResponse,
  PageVersionResponse,
  TemplateImportRequest,
  TemplatePreviewResponse,
  PlanResponse,
  PluginResponse,
  PluginUpdateRequest,
  RateLimitResponse,
  ReadyResponse,
  SaasAlertRuleResponse,
  SubscriptionResponse,
  TransferOwnershipRequest,
  UpdateMemberRoleRequest,
  UpdateOrganizationRequest,
  UpdateRateLimitRequest,
  WebhookTestResponse,
  WebhookResponse,
  WebhookRequest,
  WebhookDeliveryResponse,
} from "../types/api";

const API_BASE_URL = import.meta.env.VITE_API_URL ?? "http://localhost:8080";
const ACCESS_TOKEN_KEY = "zinhar.access_token";
const REFRESH_TOKEN_KEY = "zinhar.refresh_token";
const ACTIVE_ORGANIZATION_KEY = "zinhar.active_organization_id";

let accessToken = window.localStorage.getItem(ACCESS_TOKEN_KEY);
let activeOrganizationId = window.localStorage.getItem(ACTIVE_ORGANIZATION_KEY);

export function setApiAccessToken(token: string | null) {
  accessToken = token;
  if (token) {
    window.localStorage.setItem(ACCESS_TOKEN_KEY, token);
  } else {
    window.localStorage.removeItem(ACCESS_TOKEN_KEY);
  }
}

export function setApiRefreshToken(token: string | null) {
  if (token) {
    window.localStorage.setItem(REFRESH_TOKEN_KEY, token);
  } else {
    window.localStorage.removeItem(REFRESH_TOKEN_KEY);
  }
}

export function setApiOrganizationId(organizationId: string | null) {
  activeOrganizationId = organizationId;
  if (organizationId) {
    window.localStorage.setItem(ACTIVE_ORGANIZATION_KEY, organizationId);
  } else {
    window.localStorage.removeItem(ACTIVE_ORGANIZATION_KEY);
  }
}

export function getStoredRefreshToken() {
  return window.localStorage.getItem(REFRESH_TOKEN_KEY);
}

export class ApiError extends Error {
  status: number;

  constructor(status: number, message: string) {
    super(message);
    this.status = status;
  }
}

type RequestOptions = Omit<RequestInit, "body"> & {
  body?: unknown;
  formData?: FormData;
  auth?: boolean;
};

async function request<T>(path: string, options: RequestOptions = {}): Promise<T> {
  const headers = new Headers(options.headers);
  const needsAuth = options.auth ?? false;

  if (options.formData) {
    // Let the browser set multipart boundaries.
  } else if (options.body !== undefined) {
    headers.set("Content-Type", "application/json");
  }

  if (needsAuth && accessToken) {
    headers.set("Authorization", `Bearer ${accessToken}`);
    if (activeOrganizationId) {
      headers.set("X-Organization-Id", activeOrganizationId);
    }
  }

  const response = await fetch(`${API_BASE_URL}${path}`, {
    ...options,
    credentials: "include",
    headers,
    body: options.formData ?? (options.body === undefined ? undefined : JSON.stringify(options.body)),
  });

  if (!response.ok) {
    let message = `${response.status} ${response.statusText}`;
    try {
      const payload = (await response.json()) as { message?: string; error?: string };
      message = payload.message ?? payload.error ?? message;
    } catch {
      // Preserve the status text when the backend returns an empty body.
    }
    throw new ApiError(response.status, message);
  }

  return response.json() as Promise<T>;
}

function query(params: Record<string, string | number | boolean | undefined>) {
  const search = new URLSearchParams();
  for (const [key, value] of Object.entries(params)) {
    if (value !== undefined && value !== "") search.set(key, String(value));
  }
  const value = search.toString();
  return value ? `?${value}` : "";
}

export const api = {
  baseUrl: API_BASE_URL,
  info: () => request<ApiInfo>("/"),
  health: () => request<HealthResponse>("/health"),
  readiness: () => request<ReadyResponse>("/ready"),

  auth: {
    login: (email: string, password: string) =>
      request<AuthResponse>("/api/auth/login", { method: "POST", body: { email, password } }),
    register: (email: string, password: string, name: string) =>
      request<AuthResponse>("/api/auth/register", { method: "POST", body: { email, password, name } }),
    refresh: (refresh_token?: string | null) =>
      request<AuthResponse>("/api/auth/refresh", {
        method: "POST",
        body: refresh_token ? { refresh_token } : undefined,
      }),
    logout: (refresh_token?: string | null) =>
      request<{ revoked: boolean }>("/api/auth/logout", {
        method: "POST",
        auth: true,
        body: refresh_token ? { refresh_token } : undefined,
      }),
    me: () => request<MeResponse>("/api/auth/me", { auth: true }),
  },
  billing: {
    plans: () => request<PlanResponse[]>("/api/billing/plans", { auth: true }),
    subscription: () => request<SubscriptionResponse>("/api/billing/subscription", { auth: true }),
changePlan: (payload: ChangePlanRequest) =>
      request<SubscriptionResponse>("/api/billing/subscription", { method: "PUT", auth: true, body: payload }),
    checkout: (payload: CheckoutSessionRequest) =>
      request<CheckoutSessionResponse>("/api/billing/checkout", { method: "POST", auth: true, body: payload }),
    portal: () => request<CustomerPortalResponse>("/api/billing/portal", { method: "POST", auth: true }),
    usage: () => request<BillingUsageResponse>("/api/billing/usage", { auth: true }),
    rebuildUsage: () => request<BillingUsageResponse>("/api/billing/usage/rebuild", { method: "POST", auth: true }),
  },

  beta: {
    dashboard: () => request<BetaDashboardResponse>("/api/beta/dashboard", { auth: true }),
    feedback: (limit = 50) => request<BetaFeedbackResponse[]>(`/api/beta/feedback${query({ limit })}`, { auth: true }),
    createFeedback: (payload: BetaFeedbackRequest) =>
      request<BetaFeedbackResponse>("/api/beta/feedback", { method: "POST", auth: true, body: payload }),
    updateFeedback: (feedbackId: string, payload: UpdateBetaFeedbackRequest) =>
      request<BetaFeedbackResponse>(`/api/beta/feedback/${feedbackId}`, { method: "PATCH", auth: true, body: payload }),
    blockers: (limit = 50) => request<BetaGaBlockerResponse[]>(`/api/beta/ga-blockers${query({ limit })}`, { auth: true }),
    createBlocker: (payload: BetaGaBlockerRequest) =>
      request<BetaGaBlockerResponse>("/api/beta/ga-blockers", { method: "POST", auth: true, body: payload }),
    updateBlocker: (blockerId: string, payload: UpdateBetaGaBlockerRequest) =>
      request<BetaGaBlockerResponse>(`/api/beta/ga-blockers/${blockerId}`, { method: "PATCH", auth: true, body: payload }),
    productDashboard: () => request<BetaProductDashboardResponse>("/api/beta/product-dashboard", { auth: true }),
    upsertParticipant: (organizationId: string, payload: BetaParticipantRequest) =>
      request<BetaParticipantResponse>(`/api/beta/participants/${organizationId}`, { method: "PUT", auth: true, body: payload }),
  },
  organizations: {
    list: () => request<OrganizationMembership[]>("/api/organizations", { auth: true }),
    create: (payload: CreateOrganizationRequest) =>
      request<OrganizationDetailResponse>("/api/organizations", { method: "POST", auth: true, body: payload }),
    current: () => request<OrganizationDetailResponse>("/api/organizations/current", { auth: true }),
    updateCurrent: (payload: UpdateOrganizationRequest) =>
      request<OrganizationDetailResponse>("/api/organizations/current", { method: "PUT", auth: true, body: payload }),
    members: () => request<OrganizationMemberResponse[]>("/api/organizations/current/members", { auth: true }),
    updateMember: (userId: string, payload: UpdateMemberRoleRequest) =>
      request<OrganizationMemberResponse>(`/api/organizations/current/members/${userId}`, {
        method: "PATCH",
        auth: true,
        body: payload,
      }),
    removeMember: (userId: string) =>
      request<OrganizationMemberResponse>(`/api/organizations/current/members/${userId}`, {
        method: "DELETE",
        auth: true,
      }),
    invitations: () => request<OrganizationInvitationResponse[]>("/api/organizations/current/invitations", { auth: true }),
    invite: (payload: InviteMemberRequest) =>
      request<CreatedInvitationResponse>("/api/organizations/current/invitations", {
        method: "POST",
        auth: true,
        body: payload,
      }),
    revokeInvitation: (invitationId: string) =>
      request<OrganizationInvitationResponse>(`/api/organizations/current/invitations/${invitationId}`, {
        method: "DELETE",
        auth: true,
      }),
    acceptInvitation: (payload: AcceptInvitationRequest) =>
      request<OrganizationMembership>("/api/organization-invitations/accept", {
        method: "POST",
        auth: true,
        body: payload,
      }),
    leave: () => request<OrganizationMemberResponse>("/api/organizations/current/leave", { method: "POST", auth: true }),
    transferOwnership: (payload: TransferOwnershipRequest) =>
      request<OrganizationMemberResponse>("/api/organizations/current/transfer-ownership", {
        method: "POST",
        auth: true,
        body: payload,
      }),
    workspace: () => request<OrganizationWorkspaceResponse>("/api/organizations/current/workspace", { auth: true }),
    domains: () => request<OrganizationDomainResponse[]>("/api/organizations/current/domains", { auth: true }),
    createDomain: (payload: OrganizationDomainRequest) =>
      request<OrganizationDomainResponse>("/api/organizations/current/domains", {
        method: "POST",
        auth: true,
        body: payload,
      }),
    deleteDomain: (domainId: string) =>
      request<OrganizationDomainResponse>(`/api/organizations/current/domains/${domainId}`, {
        method: "DELETE",
        auth: true,
      }),
    rateLimit: () => request<RateLimitResponse>("/api/organizations/current/rate-limit", { auth: true }),
    updateRateLimit: (payload: UpdateRateLimitRequest) =>
      request<RateLimitResponse>("/api/organizations/current/rate-limit", {
        method: "PUT",
        auth: true,
        body: payload,
      }),
    auditLogs: (limit = 50) => request<AuditLogResponse[]>(`/api/organizations/current/audit-logs${query({ limit })}`, { auth: true }),
    emailDeliveries: (limit = 25) =>
      request<EmailDeliveryResponse[]>(`/api/organizations/current/email-deliveries${query({ limit })}`, { auth: true }),
    alerts: () => request<SaasAlertRuleResponse[]>("/api/organizations/current/alerts", { auth: true }),
  },

  contentTypes: {
    list: () => request<ContentTypeResponse[]>("/api/content-types", { auth: true }),
    create: (payload: { name: string; slug: string; fields: FieldSchemaDocument }) =>
      request<ContentTypeResponse>("/api/content-types", { method: "POST", auth: true, body: payload }),
    update: (id: string, payload: { name: string; slug: string; fields: FieldSchemaDocument }) =>
      request<ContentTypeResponse>(`/api/content-types/${id}`, { method: "PUT", auth: true, body: payload }),
    delete: (id: string) =>
      request<ContentTypeResponse>(`/api/content-types/${id}?confirm=true`, { method: "DELETE", auth: true }),
  },

  entries: {
    list: (typeSlug: string, params: { status?: string; page?: number; per_page?: number; sort?: string } = {}) =>
      request<EntryListResponse>(`/api/entries/${typeSlug}${query(params)}`, { auth: true }),
    create: (typeSlug: string, data: JsonRecord) =>
      request<ContentEntryResponse>(`/api/entries/${typeSlug}`, { method: "POST", auth: true, body: { data } }),
    update: (typeSlug: string, id: string, data: JsonRecord) =>
      request<ContentEntryResponse>(`/api/entries/${typeSlug}/${id}`, { method: "PUT", auth: true, body: { data } }),
    delete: (typeSlug: string, id: string) =>
      request<ContentEntryResponse>(`/api/entries/${typeSlug}/${id}`, { method: "DELETE", auth: true }),
    submitReview: (typeSlug: string, id: string) =>
      request<ContentEntryResponse>(`/api/entries/${typeSlug}/${id}/submit-review`, { method: "POST", auth: true }),
    publish: (typeSlug: string, id: string) =>
      request<ContentEntryResponse>(`/api/entries/${typeSlug}/${id}/publish`, { method: "POST", auth: true }),
    unpublish: (typeSlug: string, id: string) =>
      request<ContentEntryResponse>(`/api/entries/${typeSlug}/${id}/unpublish`, { method: "POST", auth: true }),
    reject: (typeSlug: string, id: string) =>
      request<ContentEntryResponse>(`/api/entries/${typeSlug}/${id}/reject`, { method: "POST", auth: true }),
    archive: (typeSlug: string, id: string) =>
      request<ContentEntryResponse>(`/api/entries/${typeSlug}/${id}/archive`, { method: "POST", auth: true }),
    restore: (typeSlug: string, id: string) =>
      request<ContentEntryResponse>(`/api/entries/${typeSlug}/${id}/restore`, { method: "POST", auth: true }),
  },

  marketplace: {
    creator: () => request<MarketplaceCreatorStateResponse>("/api/marketplace/creator", { auth: true }),
    requestCreator: (payload: MarketplaceCreatorRequest) =>
      request<MarketplaceCreatorResponse>("/api/marketplace/creator", { method: "POST", auth: true, body: payload }),
    listings: () => request<MarketplaceListingResponse[]>("/api/marketplace/listings", { auth: true }),
    catalog: (params: { search?: string; category?: string; product_type?: string; pricing_type?: string } = {}) =>
      request<MarketplaceCatalogItemResponse[]>(`/api/marketplace/catalog${query(params)}`, { auth: true }),
    catalogDetail: (listingSlug: string) =>
      request<MarketplaceCatalogDetailResponse>(`/api/marketplace/catalog/${encodeURIComponent(listingSlug)}`, { auth: true }),
    reviews: (listingId: string) =>
      request<MarketplaceProductReviewListResponse[]>(`/api/marketplace/listings/${encodeURIComponent(listingId)}/reviews`, { auth: true }),
    submitReview: (listingId: string, payload: MarketplaceProductReviewRequest) =>
      request<MarketplaceProductReviewResponse>(`/api/marketplace/listings/${encodeURIComponent(listingId)}/reviews`, {
        method: "POST", auth: true, body: payload,
      }),
    reviewModerationQueue: () =>
      request<MarketplaceProductReviewResponse[]>("/api/marketplace/reviews", { auth: true }),
    moderateReview: (reviewId: string, payload: MarketplaceProductReviewModerationRequest) =>
      request<MarketplaceProductReviewResponse>(`/api/marketplace/reviews/${encodeURIComponent(reviewId)}/moderation`, {
        method: "PATCH", auth: true, body: payload,
      }),
    submitAbuseReport: (listingId: string, payload: MarketplaceAbuseReportRequest) =>
      request<MarketplaceAbuseReportResponse>(`/api/marketplace/listings/${encodeURIComponent(listingId)}/reports`, {
        method: "POST", auth: true, body: payload,
      }),
    abuseReports: () => request<MarketplaceAbuseReportResponse[]>("/api/marketplace/reports", { auth: true }),
    resolveAbuseReport: (reportId: string, payload: MarketplaceAbuseReportResolutionRequest) =>
      request<MarketplaceAbuseReportResponse>(`/api/marketplace/reports/${encodeURIComponent(reportId)}`, {
        method: "PATCH", auth: true, body: payload,
      }),
    installations: () => request<MarketplaceInstallationResponse[]>("/api/marketplace/installations", { auth: true }),
    purchases: () => request<MarketplacePurchaseResponse[]>("/api/marketplace/purchases", { auth: true }),
    checkout: (listingId: string, versionId: string, currency = "usd") =>
      request<MarketplaceCheckoutResponse>("/api/marketplace/purchases/checkout", {
        method: "POST",
        auth: true,
        body: { listing_id: listingId, version_id: versionId, currency },
      }),
    payoutAccount: (creatorId: string) =>
      request<MarketplacePayoutAccountResponse>(`/api/marketplace/creators/${encodeURIComponent(creatorId)}/payout`, { auth: true }),
    onboardPayout: (creatorId: string, providerAccountId: string, country?: string) =>
      request<MarketplacePayoutAccountResponse>(`/api/marketplace/creators/${encodeURIComponent(creatorId)}/payout`, {
        method: "POST",
        auth: true,
        body: { provider_account_id: providerAccountId, country },
      }),
    creatorBalance: (creatorId: string) =>
      request<MarketplaceCreatorBalanceResponse>(`/api/marketplace/creators/${encodeURIComponent(creatorId)}/balance`, { auth: true }),
    requestPayout: (creatorId: string) =>
      request<MarketplacePayoutResponse>(`/api/marketplace/creators/${encodeURIComponent(creatorId)}/payout/request`, {
        method: "POST",
        auth: true,
      }),
    permissions: () => request<MarketplacePermissionCatalogResponse[]>("/api/marketplace/permissions", { auth: true }),
    runtimeStatus: () => request<MarketplaceRuntimeStatusResponse>("/api/marketplace/runtime/status", { auth: true }),
    authorizeRuntime: (installationId: string, payload: MarketplaceRuntimeAuthorizeRequest) =>
      request<MarketplaceRuntimeAuthorizationResponse>(
        `/api/marketplace/installations/${encodeURIComponent(installationId)}/runtime/authorize`,
        { method: "POST", auth: true, body: payload },
      ),
    activateOrganizationKillSwitch: (reason: string) =>
      request<MarketplaceKillSwitchResponse>("/api/marketplace/kill-switches/organization", {
        method: "POST",
        auth: true,
        body: { reason },
      }),
    activateGlobalKillSwitch: (reason: string) =>
      request<MarketplaceKillSwitchResponse>("/api/marketplace/kill-switches/global", {
        method: "POST",
        auth: true,
        body: { reason },
      }),
    liftKillSwitch: (killSwitchId: string) =>
      request<MarketplaceKillSwitchResponse>(
        `/api/marketplace/kill-switches/${encodeURIComponent(killSwitchId)}/lift`,
        { method: "POST", auth: true },
      ),
    install: (payload: MarketplaceInstallRequest) =>
      request<MarketplaceInstallationResponse>("/api/marketplace/installations", { method: "POST", auth: true, body: payload }),
    installationUpdates: (installationId: string) =>
      request<MarketplaceInstallationUpdateCheckResponse>(
        `/api/marketplace/installations/${encodeURIComponent(installationId)}/updates`,
        { auth: true },
      ),
    enableInstallation: (installationId: string) =>
      request<MarketplaceInstallationResponse>(
        `/api/marketplace/installations/${encodeURIComponent(installationId)}/enable`,
        { method: "POST", auth: true },
      ),
    disableInstallation: (installationId: string) =>
      request<MarketplaceInstallationResponse>(
        `/api/marketplace/installations/${encodeURIComponent(installationId)}/disable`,
        { method: "POST", auth: true },
      ),
    uninstallInstallation: (installationId: string) =>
      request<MarketplaceInstallationResponse>(
        `/api/marketplace/installations/${encodeURIComponent(installationId)}/uninstall`,
        { method: "POST", auth: true },
      ),
    rollbackInstallation: (installationId: string) =>
      request<MarketplaceInstallationResponse>(
        `/api/marketplace/installations/${encodeURIComponent(installationId)}/rollback`,
        { method: "POST", auth: true },
      ),
    updateInstallation: (installationId: string, payload: MarketplaceInstallationUpdateRequest) =>
      request<MarketplaceInstallationResponse>(
        `/api/marketplace/installations/${encodeURIComponent(installationId)}/update`,
        { method: "POST", auth: true, body: payload },
      ),
    createListing: (payload: MarketplaceListingRequest) =>
      request<MarketplaceListingResponse>("/api/marketplace/listings", { method: "POST", auth: true, body: payload }),
    updateListing: (listingId: string, payload: MarketplaceListingRequest) =>
      request<MarketplaceListingResponse>(`/api/marketplace/listings/${listingId}`, { method: "PUT", auth: true, body: payload }),
    submitListing: (listingId: string) =>
      request<MarketplaceListingResponse>(`/api/marketplace/listings/${listingId}/submit`, { method: "POST", auth: true }),
    uploadVersion: (listingId: string, file: File, manifest: string) => {
      const formData = new FormData();
      formData.append("file", file);
      formData.append("manifest", manifest);
      return request<MarketplaceVersionSubmissionResponse>(`/api/marketplace/listings/${listingId}/versions/upload`, {
        method: "POST",
        auth: true,
        formData,
      });
    },
    submissions: (listingId: string) =>
      request<MarketplaceValidationReportResponse[]>(`/api/marketplace/listings/${listingId}/submissions`, { auth: true }),
    reviewQueue: () => request<MarketplaceValidationReportResponse[]>("/api/marketplace/review/queue", { auth: true }),
    reviewReports: () => request<MarketplaceValidationReportResponse[]>("/api/marketplace/review/reports", { auth: true }),
    reviewEvents: () => request<MarketplaceReviewEventResponse[]>("/api/marketplace/review/events", { auth: true }),
    reviewDecision: (submissionId: string, payload: MarketplaceReviewDecisionRequest) =>
      request<MarketplaceReviewEventResponse>(`/api/marketplace/review/submissions/${submissionId}`, {
        method: "PATCH",
        auth: true,
        body: payload,
      }),
    moderateListing: (listingId: string, payload: MarketplaceModerationRequest) =>
      request<MarketplaceReviewEventResponse>(`/api/marketplace/review/listings/${listingId}/moderation`, {
        method: "POST",
        auth: true,
        body: payload,
      }),
  },
  media: {
    list: (params: { mime_type?: string; page?: number; per_page?: number } = {}) =>
      request<MediaListResponse>(`/api/media${query(params)}`, { auth: true }),
    upload: (file: File, metadata: { alt_text?: string; caption?: string }) => {
      const formData = new FormData();
      formData.append("file", file);
      if (metadata.alt_text) formData.append("alt_text", metadata.alt_text);
      if (metadata.caption) formData.append("caption", metadata.caption);
      return request<MediaDetailResponse>("/api/media/upload", { method: "POST", auth: true, formData });
    },
    update: (id: string, payload: { alt_text?: string; caption?: string }) =>
      request<MediaDetailResponse>(`/api/media/${id}`, { method: "PUT", auth: true, body: payload }),
    delete: (id: string) => request<MediaDetailResponse>(`/api/media/${id}`, { method: "DELETE", auth: true }),
  },

  pages: {
    list: (params: { status?: string; page?: number; per_page?: number; sort?: string } = {}) =>
      request<PageListResponse>(`/api/pages${query(params)}`, { auth: true }),
    create: (payload: { title: string; slug: string; page_json: PageJson }) =>
      request<PageResponse>("/api/pages", { method: "POST", auth: true, body: payload }),
    update: (id: string, payload: { title: string; slug: string; page_json: PageJson }) =>
      request<PageResponse>(`/api/pages/${id}`, { method: "PUT", auth: true, body: payload }),
    delete: (id: string) => request<PageResponse>(`/api/pages/${id}?confirm=true`, { method: "DELETE", auth: true }),
    submitReview: (id: string) => request<PageResponse>(`/api/pages/${id}/submit-review`, { method: "POST", auth: true }),
    publish: (id: string) => request<PageResponse>(`/api/pages/${id}/publish`, { method: "POST", auth: true }),
    unpublish: (id: string) => request<PageResponse>(`/api/pages/${id}/unpublish`, { method: "POST", auth: true }),
    reject: (id: string) => request<PageResponse>(`/api/pages/${id}/reject`, { method: "POST", auth: true }),
    archive: (id: string) => request<PageResponse>(`/api/pages/${id}/archive`, { method: "POST", auth: true }),
    restoreStatus: (id: string) => request<PageResponse>(`/api/pages/${id}/restore`, { method: "POST", auth: true }),
    versions: (id: string) => request<PageVersionResponse[]>(`/api/pages/${id}/versions`, { auth: true }),
    restore: (id: string, version: number) =>
      request<PageResponse>(`/api/pages/${id}/versions/${version}/restore`, { method: "POST", auth: true }),
  },

  components: {
    list: (category?: string) =>
      request<ComponentRegistryResponse[]>(`/api/component-registry${query({ category })}`, { auth: true }),
  },
  marketplaceAdapters: {
    components: () => request<MarketplaceComponentResponse[]>("/api/marketplace/runtime/components", { auth: true }),
    previewTemplate: (installationId: string, payload: { template_key?: string; asset_mapping?: Record<string, string> } = {}) =>
      request<TemplatePreviewResponse>(`/api/marketplace/templates/${installationId}/preview`, { method: "POST", auth: true, body: payload }),
    importTemplate: (installationId: string, payload: TemplateImportRequest) =>
      request<PageResponse>(`/api/marketplace/templates/${installationId}/import`, { method: "POST", auth: true, body: payload }),
    hooks: () => request<MarketplaceHookResponse[]>("/api/marketplace/hooks", { auth: true }),
    authorizeHook: (hookType: string, payload: { hook_key: string; context?: JsonRecord }) =>
      request<MarketplaceHookAuthorizationResponse>(`/api/marketplace/hooks/${hookType}/authorize`, { method: "POST", auth: true, body: payload }),
  },
  comments: {
    list: (entity_type: CommentEntityType, entity_id: string, include_resolved = false) =>
      request<CommentResponse[]>(`/api/comments${query({ entity_type, entity_id, include_resolved })}`, { auth: true }),
    create: (payload: CommentRequest) =>
      request<CommentResponse>("/api/comments", { method: "POST", auth: true, body: payload }),
    resolve: (id: string) => request<CommentResponse>(`/api/comments/${id}/resolve`, { method: "POST", auth: true }),
    unresolve: (id: string) => request<CommentResponse>(`/api/comments/${id}/resolve`, { method: "DELETE", auth: true }),
    delete: (id: string) => request<CommentResponse>(`/api/comments/${id}`, { method: "DELETE", auth: true }),
  },
  plugins: {
    list: () => request<PluginResponse[]>("/api/plugins", { auth: true }),
    update: (pluginKey: string, payload: PluginUpdateRequest) =>
      request<PluginResponse>(`/api/plugins/${pluginKey}`, { method: "PUT", auth: true, body: payload }),
    enable: (pluginKey: string) => request<PluginResponse>(`/api/plugins/${pluginKey}/enable`, { method: "POST", auth: true }),
    disable: (pluginKey: string) => request<PluginResponse>(`/api/plugins/${pluginKey}/disable`, { method: "POST", auth: true }),
  },
  webhooks: {
    list: () => request<WebhookResponse[]>("/api/webhooks", { auth: true }),
    create: (payload: WebhookRequest) =>
      request<WebhookResponse>("/api/webhooks", { method: "POST", auth: true, body: payload }),
    update: (id: string, payload: WebhookRequest) =>
      request<WebhookResponse>(`/api/webhooks/${id}`, { method: "PUT", auth: true, body: payload }),
    delete: (id: string) =>
      request<WebhookResponse>(`/api/webhooks/${id}?confirm=true`, { method: "DELETE", auth: true }),
    deliveries: (id: string, limit = 20) =>
      request<WebhookDeliveryResponse[]>(`/api/webhooks/${id}/deliveries${query({ limit })}`, { auth: true }),
    test: (id: string) =>
      request<WebhookTestResponse>(`/api/webhooks/${id}/test`, { method: "POST", auth: true }),
  },
};
