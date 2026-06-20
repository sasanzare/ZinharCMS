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

export type AuthResponse = {
  access_token: string;
  refresh_token: string;
  token_type: string;
  expires_in: number;
  user: AuthUser;
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

export type EntryStatus = "draft" | "pending_review" | "published" | "archived";

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
  status: "draft" | "published" | "archived";
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