import { create } from "zustand";

import { setApiAccessToken, setApiOrganizationId, setApiRefreshToken } from "../services/api";
import type { AuthUser, OrganizationMembership } from "../types/api";

const ACCESS_TOKEN_KEY = "zinhar.access_token";
const REFRESH_TOKEN_KEY = "zinhar.refresh_token";
const USER_KEY = "zinhar.user";
const ORGANIZATIONS_KEY = "zinhar.organizations";
const ACTIVE_ORGANIZATION_KEY = "zinhar.active_organization_id";

type AuthSession = {
  accessToken: string;
  refreshToken?: string | null;
  user: AuthUser;
  organizations: OrganizationMembership[];
  defaultOrganizationId?: string | null;
};

type AppStore = {
  sidebarCollapsed: boolean;
  accessToken: string | null;
  refreshToken: string | null;
  user: AuthUser | null;
  organizations: OrganizationMembership[];
  activeOrganizationId: string | null;
  toggleSidebar: () => void;
  setSession: (session: AuthSession) => void;
  setActiveOrganization: (organizationId: string) => void;
  clearSession: () => void;
};

function readJson<T>(key: string, fallback: T): T {
  const raw = window.localStorage.getItem(key);
  if (!raw) return fallback;
  try {
    return JSON.parse(raw) as T;
  } catch {
    window.localStorage.removeItem(key);
    return fallback;
  }
}

function readStoredUser() {
  return readJson<AuthUser | null>(USER_KEY, null);
}

function readStoredOrganizations() {
  return readJson<OrganizationMembership[]>(ORGANIZATIONS_KEY, []);
}

function selectActiveOrganization(
  organizations: OrganizationMembership[],
  preferredId?: string | null,
) {
  const stored = window.localStorage.getItem(ACTIVE_ORGANIZATION_KEY);
  const candidate = preferredId ?? stored;
  if (candidate && organizations.some((organization) => organization.id === candidate)) {
    return candidate;
  }
  return organizations[0]?.id ?? null;
}

const storedOrganizations = readStoredOrganizations();
const storedActiveOrganizationId = selectActiveOrganization(storedOrganizations);
setApiOrganizationId(storedActiveOrganizationId);

export const useAppStore = create<AppStore>((set, get) => ({
  sidebarCollapsed: false,
  accessToken: window.localStorage.getItem(ACCESS_TOKEN_KEY),
  refreshToken: window.localStorage.getItem(REFRESH_TOKEN_KEY),
  user: readStoredUser(),
  organizations: storedOrganizations,
  activeOrganizationId: storedActiveOrganizationId,
  toggleSidebar: () => set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed })),
  setSession: ({ accessToken, refreshToken, user, organizations, defaultOrganizationId }) => {
    const activeOrganizationId = selectActiveOrganization(organizations, defaultOrganizationId);
    setApiAccessToken(accessToken);
    setApiRefreshToken(refreshToken ?? null);
    setApiOrganizationId(activeOrganizationId);
    window.localStorage.setItem(USER_KEY, JSON.stringify(user));
    window.localStorage.setItem(ORGANIZATIONS_KEY, JSON.stringify(organizations));
    set({
      accessToken,
      refreshToken: refreshToken ?? null,
      user,
      organizations,
      activeOrganizationId,
    });
  },
  setActiveOrganization: (organizationId) => {
    const exists = get().organizations.some((organization) => organization.id === organizationId);
    if (!exists) return;
    setApiOrganizationId(organizationId);
    set({ activeOrganizationId: organizationId });
  },
  clearSession: () => {
    setApiAccessToken(null);
    setApiRefreshToken(null);
    setApiOrganizationId(null);
    window.localStorage.removeItem(USER_KEY);
    window.localStorage.removeItem(ORGANIZATIONS_KEY);
    set({
      accessToken: null,
      refreshToken: null,
      user: null,
      organizations: [],
      activeOrganizationId: null,
    });
  },
}));
