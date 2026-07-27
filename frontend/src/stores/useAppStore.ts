import { create } from "zustand";

import { api, setApiOrganizationId } from "../services/api";
import {
  acceptBrowserSession,
  clearBrowserSession,
  subscribeBrowserSession,
} from "../services/authSession";
import type { AuthUser, OrganizationMembership } from "../types/api";

const USER_KEY = "zinhar.user";
const ORGANIZATIONS_KEY = "zinhar.organizations";
const ACTIVE_ORGANIZATION_KEY = "zinhar.active_organization_id";

type AuthSession = {
  accessToken: string;
  user: AuthUser;
  organizations: OrganizationMembership[];
  defaultOrganizationId?: string | null;
};

type AppStore = {
  sidebarCollapsed: boolean;
  authStatus: "unknown" | "refreshing" | "authenticated" | "unauthenticated";
  accessToken: string | null;
  user: AuthUser | null;
  organizations: OrganizationMembership[];
  activeOrganizationId: string | null;
  toggleSidebar: () => void;
  bootstrapSession: () => Promise<void>;
  setSession: (session: AuthSession) => void;
  setActiveOrganization: (organizationId: string) => void;
  setOrganizations: (organizations: OrganizationMembership[], preferredOrganizationId?: string | null) => void;
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
let bootstrapPromise: Promise<void> | null = null;

export const useAppStore = create<AppStore>((set, get) => ({
  sidebarCollapsed: false,
  authStatus: "unknown",
  accessToken: null,
  user: readStoredUser(),
  organizations: storedOrganizations,
  activeOrganizationId: storedActiveOrganizationId,
  toggleSidebar: () => set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed })),
  bootstrapSession: () => {
    if (bootstrapPromise) return bootstrapPromise;
    if (get().authStatus === "authenticated") return Promise.resolve();

    set({ authStatus: "refreshing" });
    bootstrapPromise = api.auth
      .refresh()
      .then(() => undefined)
      .catch(() => {
        clearBrowserSession(false);
      })
      .finally(() => {
        bootstrapPromise = null;
      });
    return bootstrapPromise;
  },
  setSession: ({ accessToken, user, organizations, defaultOrganizationId }) => {
    acceptBrowserSession({
      access_token: accessToken,
      token_type: "Bearer",
      expires_in: 0,
      user,
      organizations,
      default_organization_id: defaultOrganizationId ?? null,
    });
  },
  setActiveOrganization: (organizationId) => {
    const exists = get().organizations.some((organization) => organization.id === organizationId);
    if (!exists) return;
    setApiOrganizationId(organizationId);
    set({ activeOrganizationId: organizationId });
  },
  setOrganizations: (organizations, preferredOrganizationId) => {
    const activeOrganizationId = selectActiveOrganization(
      organizations,
      preferredOrganizationId ?? get().activeOrganizationId,
    );
    setApiOrganizationId(activeOrganizationId);
    window.localStorage.setItem(ORGANIZATIONS_KEY, JSON.stringify(organizations));
    set({ organizations, activeOrganizationId });
  },
  clearSession: () => {
    clearBrowserSession();
  },
}));

subscribeBrowserSession((event) => {
  if (event.type === "logout") {
    setApiOrganizationId(null);
    window.localStorage.removeItem(USER_KEY);
    window.localStorage.removeItem(ORGANIZATIONS_KEY);
    useAppStore.setState({
      authStatus: "unauthenticated",
      accessToken: null,
      user: null,
      organizations: [],
      activeOrganizationId: null,
    });
    return;
  }

  const { session } = event;
  const activeOrganizationId = selectActiveOrganization(
    session.organizations,
    session.default_organization_id,
  );
  setApiOrganizationId(activeOrganizationId);
  window.localStorage.setItem(USER_KEY, JSON.stringify(session.user));
  window.localStorage.setItem(ORGANIZATIONS_KEY, JSON.stringify(session.organizations));
  useAppStore.setState({
    authStatus: "authenticated",
    accessToken: session.access_token,
    user: session.user,
    organizations: session.organizations,
    activeOrganizationId,
  });
});
