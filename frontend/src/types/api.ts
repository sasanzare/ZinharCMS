export type HealthResponse = {
  status: string;
  version: string;
};

export type DependencyCheck = {
  name: string;
  ok: boolean;
  message: string;
};

export type ReadyResponse = {
  status: string;
  checks: DependencyCheck[];
};

export type ApiInfo = {
  name: string;
  version: string;
  docs: string;
  health: string;
};

export type JsonPrimitive = string | number | boolean | null;
export type JsonValue = JsonPrimitive | JsonValue[] | { [key: string]: JsonValue };
export type JsonRecord = Record<string, JsonValue>;

export type AuthUser = {
  id: string;
  email: string;
  name: string;
  avatar_url: string | null;
  role: string;
};

export type OrganizationMembership = {
  id: string;
  name: string;
  slug: string;
  role: string;
  status: string;
};

export type MeResponse = {
  user: AuthUser;
  organizations: OrganizationMembership[];
  default_organization_id: string | null;
};

export type AuthResponse = MeResponse & {
  access_token: string;
  refresh_token: string | null;
  token_type: string;
  expires_in: number;
};
export type OrganizationRole = "owner" | "admin" | "editor" | "author" | "viewer" | "billing_manager";

export type OrganizationResponse = {
  id: string;
  name: string;
  slug: string;
  status: string;
  owner_id: string | null;
  settings: JsonRecord;
  created_at: string;
  updated_at: string;
};

export type PlanLimitResponse = {
  plan: string;
  plan_slug: string;
  members_limit: number;
  content_limit: number;
  media_limit_mb: number;
  api_requests_limit: number;
};

export type OrganizationDetailResponse = {
  organization: OrganizationResponse;
  membership: OrganizationMembership;
  plan_limits: PlanLimitResponse;
};

export type OrganizationMemberResponse = {
  user_id: string;
  email: string;
  name: string;
  avatar_url: string | null;
  role: OrganizationRole;
  status: string;
  joined_at: string | null;
  created_at: string;
  updated_at: string;
};

export type OrganizationInvitationResponse = {
  id: string;
  email: string;
  role: OrganizationRole;
  status: string;
  invited_by: string | null;
  invited_by_name: string | null;
  expires_at: string;
  accepted_at: string | null;
  created_at: string;
  updated_at: string;
};

export type CreatedInvitationResponse = OrganizationInvitationResponse & {
  token: string;
  accept_path: string;
};

export type CreateOrganizationRequest = {
  name: string;
  slug: string;
};

export type UpdateOrganizationRequest = CreateOrganizationRequest & {
  settings?: JsonRecord;
};

export type InviteMemberRequest = {
  email: string;
  role: OrganizationRole;
};

export type UpdateMemberRoleRequest = {
  role: OrganizationRole;
};

export type AcceptInvitationRequest = {
  token: string;
};

export type OrganizationDomainResponse = {
  id: string;
  domain: string;
  status: string;
  is_primary: boolean;
  verification_token: string;
  verified_at: string | null;
  created_by: string | null;
  created_at: string;
  updated_at: string;
};

export type OrganizationWorkspaceResponse = {
  slug: string;
  workspace_url: string;
  domains: OrganizationDomainResponse[];
};

export type OrganizationDomainRequest = {
  domain: string;
  is_primary?: boolean;
};

export type RateLimitResponse = {
  requests_per_minute: number;
  user_requests_per_minute: number;
  burst: number;
  updated_by: string | null;
  created_at: string;
  updated_at: string;
};

export type UpdateRateLimitRequest = {
  requests_per_minute: number;
  user_requests_per_minute: number;
  burst: number;
};

export type AuditLogResponse = {
  id: string;
  actor_id: string | null;
  actor_email: string | null;
  action: string;
  entity_type: string;
  entity_id: string | null;
  metadata: JsonRecord;
  created_at: string;
};

export type EmailDeliveryResponse = {
  id: string;
  recipient_email: string;
  template: string;
  subject: string;
  provider: string;
  status: string;
  provider_message_id: string | null;
  error: string | null;
  sent_at: string | null;
  created_at: string;
  updated_at: string;
};

export type SaasAlertRuleResponse = {
  id: string;
  alert_key: string;
  severity: string;
  is_enabled: boolean;
  config: JsonRecord;
  created_at: string;
  updated_at: string;
};

export type BetaFeedbackCategory = "bug" | "ux" | "billing" | "performance" | "tenant_isolation" | "onboarding" | "other";
export type BetaFeedbackSeverity = "low" | "medium" | "high" | "critical";
export type BetaFeedbackStatus = "open" | "triaged" | "planned" | "fixed" | "closed";
export type BetaBlockerPriority = "p0" | "p1" | "p2" | "p3";
export type BetaBlockerStatus = "open" | "in_progress" | "blocked" | "resolved" | "deferred";
export type BetaParticipantStatus = "candidate" | "invited" | "onboarding" | "active" | "paused" | "graduated" | "rejected";

export type BetaFeedbackRequest = {
  category?: BetaFeedbackCategory;
  severity?: BetaFeedbackSeverity;
  title: string;
  description: string;
  page_url?: string;
  metadata?: JsonRecord;
};

export type UpdateBetaFeedbackRequest = {
  status?: BetaFeedbackStatus;
  severity?: BetaFeedbackSeverity;
};

export type BetaFeedbackResponse = {
  id: string;
  organization_id: string;
  submitted_by: string | null;
  submitted_by_email: string | null;
  category: BetaFeedbackCategory;
  severity: BetaFeedbackSeverity;
  status: BetaFeedbackStatus;
  title: string;
  description: string;
  page_url: string | null;
  metadata: JsonRecord;
  created_at: string;
  updated_at: string;
};

export type BetaGaBlockerRequest = {
  feedback_id?: string;
  priority?: BetaBlockerPriority;
  area: string;
  title: string;
  owner?: string;
  due_at?: string;
  metadata?: JsonRecord;
};

export type UpdateBetaGaBlockerRequest = {
  priority?: BetaBlockerPriority;
  status?: BetaBlockerStatus;
  owner?: string;
  due_at?: string;
};

export type BetaGaBlockerResponse = {
  id: string;
  organization_id: string;
  feedback_id: string | null;
  priority: BetaBlockerPriority;
  area: string;
  title: string;
  status: BetaBlockerStatus;
  owner: string | null;
  due_at: string | null;
  metadata: JsonRecord;
  created_at: string;
  updated_at: string;
};


export type BetaParticipantRequest = {
  cohort_label?: string;
  contact_name?: string;
  contact_email?: string;
  status: BetaParticipantStatus;
  notes?: string;
  metadata?: JsonRecord;
};

export type BetaParticipantResponse = {
  organization_id: string;
  organization_name: string;
  organization_slug: string;
  cohort_label: string;
  contact_name: string | null;
  contact_email: string | null;
  status: BetaParticipantStatus;
  onboarded_at: string | null;
  last_check_in_at: string | null;
  notes: string | null;
  metadata: JsonRecord;
  created_at: string;
  updated_at: string;
};

export type BetaOrganizationDashboardResponse = {
  organization_id: string;
  organization_name: string;
  organization_slug: string;
  participant_status: BetaParticipantStatus | null;
  cohort_label: string | null;
  current_plan: string | null;
  current_plan_slug: string | null;
  open_feedback: number;
  critical_feedback: number;
  open_ga_blockers: number;
  failed_billing_events: number;
  failed_email_deliveries: number;
};

export type BetaDashboardResponse = {
  organization: BetaOrganizationDashboardResponse;
  exceeded_usage_metrics: string[];
};

export type BetaProductTotalsResponse = {
  beta_organizations: number;
  active_organizations: number;
  open_feedback: number;
  critical_feedback: number;
  open_ga_blockers: number;
  failed_billing_events: number;
  failed_email_deliveries: number;
};

export type BetaProductDashboardResponse = {
  organizations: BetaOrganizationDashboardResponse[];
  totals: BetaProductTotalsResponse;
};
export type TransferOwnershipRequest = {
  user_id: string;
};

export type PlanResponse = {
  id: string;
  slug: string;
  name: string;
  description: string;
  price_monthly_cents: number;
  member_limit: number;
  content_limit: number;
  media_limit_mb: number;
  api_requests_limit: number;
  stripe_checkout_available: boolean;
  features: JsonRecord;
};

export type SubscriptionResponse = {
  organization_id: string;
  plan_id: string;
  plan_slug: string;
  plan_name: string;
  status: string;
  provider: string;
  current_period_start: string;
  current_period_end: string;
  cancel_at_period_end: boolean;
};

export type UsageMetricResponse = {
  metric: string;
  used: number;
  limit: number;
  remaining: number | null;
  percent: number | null;
  near_limit: boolean;
  exceeded: boolean;
};

export type BillingUsageResponse = {
  period_start: string;
  plan: PlanResponse;
  subscription: SubscriptionResponse;
  members: UsageMetricResponse;
  content_records: UsageMetricResponse;
  media_bytes: UsageMetricResponse;
  api_requests: UsageMetricResponse;
};

export type ChangePlanRequest = {
  plan_slug: string;
};

export type CheckoutSessionRequest = {
  plan_slug: string;
};

export type CheckoutSessionResponse = {
  session_id: string;
  url: string;
};

export type CustomerPortalResponse = {
  url: string;
};

export type FieldType = "text" | "longtext" | "richtext" | "number" | "boolean" | "datetime" | "media";

export type FieldSchema = {
  id?: string;
  name: string;
  label?: string;
  type: FieldType;
  required?: boolean;
  min_length?: number;
  max_length?: number;
  min?: number;
  max?: number;
  allowed_types?: string[];
  multiple?: boolean;
  target_type?: string;
  cardinality?: string;
  source_field?: string;
};

export type FieldSchemaDocument = {
  fields: FieldSchema[];
};

export type ContentTypeResponse = {
  id: string;
  name: string;
  slug: string;
  fields: FieldSchemaDocument;
  created_by: string | null;
  created_at: string;
  updated_at: string;
};

export type WorkflowStatus = "draft" | "pending_review" | "published" | "archived";
export type EntryStatus = WorkflowStatus;

export type ContentEntryResponse = {
  id: string;
  type_id: string;
  data: JsonRecord;
  status: EntryStatus;
  version: number;
  author_id: string | null;
  published_at: string | null;
  created_at: string;
  updated_at: string;
};

export type EntryListResponse = {
  data: ContentEntryResponse[];
  page: number;
  per_page: number;
};

export type MediaResponse = {
  id: string;
  filename: string;
  url: string;
  mime_type: string;
  size: number;
  alt_text: string | null;
  caption: string | null;
  uploader_id: string | null;
  created_at: string;
  updated_at: string;
};

export type MediaVariantResponse = {
  id: string;
  media_id: string;
  variant_name: string;
  url: string;
  width: number | null;
  height: number | null;
  created_at: string;
};

export type MediaDetailResponse = {
  media: MediaResponse;
  variants: MediaVariantResponse[];
};

export type MediaListResponse = {
  data: MediaResponse[];
  page: number;
  per_page: number;
};

export type PageNode = {
  id: string;
  type: string;
  props?: JsonRecord;
  styles?: JsonRecord;
  children?: PageNode[];
};

export type PageJson = {
  version?: string;
  metadata?: {
    title?: string;
    description?: string;
    og_image?: string;
  };
  layout: PageNode;
};

export type PageResponse = {
  id: string;
  title: string;
  slug: string;
  page_json: PageJson;
  status: WorkflowStatus;
  author_id: string | null;
  published_at: string | null;
  created_at: string;
  updated_at: string;
};

export type PageListResponse = {
  data: PageResponse[];
  page: number;
  per_page: number;
};

export type PageVersionResponse = {
  id: string;
  page_id: string;
  version: number;
  page_json: PageJson;
  snapshot_at: string;
  created_by: string | null;
};

export type ComponentRegistryResponse = {
  id: string;
  component_key: string;
  name: string;
  category: string;
  props_schema: JsonRecord;
  is_system: boolean;
  created_at: string;
  updated_at: string;
};
export type WebhookEvent = "entry.publish" | "entry.unpublish" | "page.publish" | "page.unpublish";

export type WebhookRequest = {
  name: string;
  url: string;
  events: WebhookEvent[];
  secret?: string;
  is_active?: boolean;
};

export type WebhookResponse = {
  id: string;
  name: string;
  url: string;
  events: WebhookEvent[];
  secret: string;
  is_active: boolean;
  created_at: string;
  updated_at: string;
};

export type WebhookDeliveryResponse = {
  id: string;
  webhook_id: string;
  event: WebhookEvent;
  payload: JsonRecord;
  status: "delivered" | "failed";
  status_code: number | null;
  response_body: string | null;
  error: string | null;
  attempted_at: string;
};

export type WebhookTestResponse = {
  sent: boolean;
  event: WebhookEvent;
};
export type CommentEntityType = "entry" | "page";

export type CommentRequest = {
  entity_type: CommentEntityType;
  entity_id: string;
  body: string;
};

export type CommentResponse = {
  id: string;
  entity_type: CommentEntityType;
  entity_id: string;
  body: string;
  author_id: string | null;
  author_name: string | null;
  resolved_at: string | null;
  resolved_by: string | null;
  created_at: string;
  updated_at: string;
};

export type PluginUpdateRequest = {
  is_enabled: boolean;
};

export type PluginResponse = {
  id: string;
  plugin_key: string;
  name: string;
  version: string;
  description: string;
  hooks: string[];
  is_enabled: boolean;
  is_system: boolean;
  created_at: string;
  updated_at: string;
};

export type MarketplaceCreatorStatus = "pending" | "approved" | "suspended" | "rejected";
export type MarketplaceProductType = "component_pack" | "design_template" | "integration_plugin" | "backend_extension";
export type MarketplacePricingType = "free" | "paid" | "custom";
export type MarketplaceListingStatus = "draft" | "submitted" | "approved" | "changes_requested" | "suspended" | "blocked" | "archived";
export type MarketplaceValidationStatus = "pending" | "passed" | "warning" | "failed";
export type MarketplaceSecurityRiskLevel = "unreviewed" | "low" | "medium" | "high" | "critical";

export type MarketplaceCreatorRequest = {
  slug: string;
  display_name: string;
  bio?: string;
  support_email?: string;
};

export type MarketplaceCreatorResponse = {
  id: string;
  user_id: string;
  slug: string;
  display_name: string;
  bio: string | null;
  status: MarketplaceCreatorStatus;
  payout_status: string;
  support_email: string | null;
  verification_notes: string | null;
  verified_by: string | null;
  verified_at: string | null;
  metadata: JsonRecord;
  requested_at: string;
  created_at: string;
  updated_at: string;
};

export type MarketplaceComponentResponse = ComponentRegistryResponse & {
  installation_id: string;
  listing_title: string;
  version: string;
  enabled: boolean;
};

export type TemplateAdapterRequest = {
  template_key?: string;
  asset_mapping?: Record<string, string>;
};

export type TemplatePreviewResponse = {
  installation_id: string;
  template_key: string;
  page_json: PageJson;
  required_assets: string[];
  mapped_assets: string[];
};

export type TemplateImportRequest = {
  title: string;
  slug: string;
  template_key?: string;
  asset_mapping?: Record<string, string>;
};

export type MarketplaceHookResponse = {
  installation_id: string;
  hook_key: string;
  hook_type: "sidebar.item" | "dashboard.widget" | "form.field" | "webhook.adapter";
  label: string;
  contract_version: string;
  config: JsonRecord;
  listing_title: string;
  version: string;
  enabled: boolean;
};

export type MarketplaceHookAuthorizationResponse = {
  allowed: boolean;
  hook_key: string;
  hook_type: string;
  contract_version: string;
  execution: "not_executed";
  reason_code: string | null;
  message: string | null;
};

export type MarketplaceCreatorStateResponse = {
  creator: MarketplaceCreatorResponse | null;
};

export type MarketplaceListingRequest = {
  product_type: MarketplaceProductType;
  title: string;
  slug: string;
  summary: string;
  description: string;
  category: string;
  pricing_type: MarketplacePricingType;
  price_cents: number;
  license: string;
  support_url?: string;
  screenshots: string[];
};

export type MarketplaceListingResponse = {
  id: string;
  creator_id: string;
  product_type: MarketplaceProductType;
  title: string;
  slug: string;
  summary: string;
  description: string;
  category: string;
  status: MarketplaceListingStatus;
  pricing_type: MarketplacePricingType;
  price_cents: number;
  license: string;
  support_url: string | null;
  screenshots: string[];
  metadata: JsonRecord;
  submitted_by: string | null;
  submitted_at: string | null;
  created_at: string;
  updated_at: string;
};

export type MarketplaceCatalogItemResponse = {
  id: string;
  title: string;
  slug: string;
  summary: string;
  category: string;
  product_type: MarketplaceProductType;
  pricing_type: MarketplacePricingType;
  price_cents: number;
  creator_display_name: string;
  latest_version_id: string;
  latest_version: string;
  badge: string;
  rating_average: number;
  rating_count: number;
  active_installations: number;
  compatibility_report: JsonRecord;
  permissions: JsonValue;
  screenshots: JsonValue;
  support_url: string | null;
  updated_at: string;
};

export type MarketplaceCatalogVersionResponse = {
  id: string;
  version: string;
  compatibility_report: JsonRecord;
  permissions: JsonValue;
  changelog: JsonValue;
  created_at: string;
};

export type MarketplaceCatalogReviewResponse = {
  author: string;
  rating: number;
  body: string;
  created_at: string | null;
};

export type MarketplaceCatalogDetailResponse = {
  item: MarketplaceCatalogItemResponse;
  description: string;
  license: string;
  support_url: string | null;
  screenshots: JsonValue;
  permissions: JsonValue;
  changelog: JsonValue;
  versions: MarketplaceCatalogVersionResponse[];
  reviews: MarketplaceCatalogReviewResponse[];
};

export type MarketplaceInstallationStatus = "active" | "disabled" | "uninstalled" | "rollback_pending" | "blocked";

export type MarketplaceInstallRequest = {
  listing_id: string;
  version_id: string;
  approved_permissions: string[];
};

export type MarketplaceInstallationUpdateRequest = {
  version_id: string;
  changelog_confirmed: boolean;
  approved_permissions?: string[] | null;
};

export type MarketplaceInstallationResponse = {
  id: string;
  organization_id: string;
  listing_id: string;
  listing_title: string;
  listing_slug: string;
  product_type: MarketplaceProductType;
  pricing_type: MarketplacePricingType;
  version_id: string;
  installed_version: string;
  status: MarketplaceInstallationStatus;
  permissions: JsonValue;
  permission_approved_by: string | null;
  permission_approved_at: string | null;
  rollback_version_id: string | null;
  rollback_version: string | null;
  cleanup_policy: string;
  version_pinned: boolean;
  installed_by: string | null;
  installed_at: string;
  enabled_at: string;
  disabled_at: string | null;
  uninstalled_at: string | null;
  version_changed_at: string;
  updated_at: string;
};

export type MarketplaceInstallationUpdateCheckResponse = {
  installation_id: string;
  current_version_id: string;
  current_version: string;
  current_status: MarketplaceInstallationStatus;
  version_pinned: boolean;
  update_available: boolean;
  target_version_id: string | null;
  target_version: string | null;
  changelog: JsonValue;
  permissions: JsonValue;
  permission_reapproval_required: boolean;
  compatibility_report: JsonValue;
  reasons: string[];
};

export type MarketplacePermissionCatalogResponse = {
  permission_key: string;
  description: string;
  category: string;
  risk_level: string;
  product_types: JsonValue;
  runtime_operations: JsonValue;
  enabled: boolean;
};

export type MarketplaceKillSwitchResponse = {
  id: string;
  scope: "global" | "organization";
  organization_id: string | null;
  reason: string;
  active: boolean;
  created_by: string | null;
  created_at: string;
  lifted_by: string | null;
  lifted_at: string | null;
};

export type MarketplaceRuntimeStatusResponse = {
  global_blocked: boolean;
  organization_blocked: boolean;
  organization_id: string;
  status_message: string;
  active_kill_switches: MarketplaceKillSwitchResponse[];
};

export type MarketplaceRuntimeAuthorizeRequest = {
  operation: string;
  entry_point: string;
  payload: JsonRecord;
};

export type MarketplaceRuntimeAuthorizationResponse = {
  allowed: boolean;
  installation_id: string;
  operation: string;
  required_permission: string | null;
  entry_point: string;
  sandbox_policy: string;
  execution: string;
  reason_code: string | null;
  message: string | null;
};

export type MarketplacePackageVersionResponse = {
  id: string;
  listing_id: string;
  version: string;
  manifest_schema_version: string;
  manifest_json: JsonRecord;
  artifact_object_key: string;
  artifact_sha256: string;
  artifact_size_bytes: number;
  artifact_file_name: string;
  artifact_content_type: string;
  storage_metadata: JsonRecord;
  validation_status: MarketplaceValidationStatus;
  validation_report: JsonRecord;
  security_risk_level: MarketplaceSecurityRiskLevel;
  compatibility_report: JsonRecord;
  status: string;
  created_by: string | null;
  created_at: string;
  updated_at: string;
};

export type MarketplaceSubmissionResponse = {
  id: string;
  version_id: string;
  submitted_by: string | null;
  review_status: string;
  risk_level: string;
  review_notes: string | null;
  validation_report: JsonRecord;
  metadata: JsonRecord;
  reviewed_by: string | null;
  reviewed_at: string | null;
  created_at: string;
  updated_at: string;
};

export type MarketplaceVersionSubmissionResponse = {
  version: MarketplacePackageVersionResponse;
  submission: MarketplaceSubmissionResponse;
};

export type MarketplaceValidationReportResponse = {
  listing_id: string;
  listing_title: string;
  listing_slug: string;
  creator_id: string;
  creator_display_name: string;
  version_id: string;
  version: string;
  version_status: string;
  validation_status: MarketplaceValidationStatus;
  security_risk_level: MarketplaceSecurityRiskLevel;
  validation_report: JsonRecord;
  compatibility_report: JsonRecord;
  submission_id: string;
  review_status: string;
  risk_level: MarketplaceSecurityRiskLevel;
  review_notes: string | null;
  submitted_at: string;
  updated_at: string;
};

export type MarketplaceReviewDecision = "approve" | "reject" | "request_changes";

export type MarketplaceModerationAction =
  | "suspend_listing"
  | "unpublish_version"
  | "emergency_block";

export type MarketplaceReviewDecisionRequest = {
  decision: MarketplaceReviewDecision;
  internal_comment?: string;
  creator_message?: string;
};

export type MarketplaceModerationRequest = {
  action: MarketplaceModerationAction;
  version_id?: string;
  reason: string;
  internal_comment?: string;
  creator_message?: string;
};

export type MarketplaceReviewEventResponse = {
  id: string;
  submission_id: string | null;
  listing_id: string;
  listing_title: string;
  version_id: string | null;
  version: string | null;
  actor_id: string | null;
  actor_email: string | null;
  action: string;
  previous_status: string | null;
  next_status: string;
  internal_comment: string | null;
  creator_message: string | null;
  reason: string;
  metadata: JsonRecord;
  created_at: string;
};
