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
