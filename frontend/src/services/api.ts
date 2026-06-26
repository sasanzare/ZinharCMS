import type {
  ApiInfo,
  AuthResponse,
  AcceptInvitationRequest,
  CreateOrganizationRequest,
  CreatedInvitationResponse,
  InviteMemberRequest,
  CommentEntityType,
  CommentRequest,
  CommentResponse,
  ComponentRegistryResponse,
  ContentEntryResponse,
  ContentTypeResponse,
  EntryListResponse,
  FieldSchemaDocument,
  HealthResponse,
  JsonRecord,
  MeResponse,
  OrganizationDetailResponse,
  OrganizationInvitationResponse,
  OrganizationMemberResponse,
  OrganizationMembership,
  MediaDetailResponse,
  MediaListResponse,
  PageJson,
  PageListResponse,
  PageResponse,
  PageVersionResponse,
  PluginResponse,
  PluginUpdateRequest,
  ReadyResponse,
  TransferOwnershipRequest,
  UpdateMemberRoleRequest,
  UpdateOrganizationRequest,
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
