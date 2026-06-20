import { create } from "zustand";

import { setApiAccessToken, setApiRefreshToken } from "../services/api";
import type { AuthUser } from "../types/api";

const ACCESS_TOKEN_KEY = "zinhar.access_token";
const REFRESH_TOKEN_KEY = "zinhar.refresh_token";
const USER_KEY = "zinhar.user";

type AuthSession = {
  accessToken: string;
  refreshToken?: string | null;
  user: AuthUser;
};

type AppStore = {
  sidebarCollapsed: boolean;
  accessToken: string | null;
  refreshToken: string | null;
  user: AuthUser | null;
  toggleSidebar: () => void;
  setSession: (session: AuthSession) => void;
  clearSession: () => void;
};

function readStoredUser() {
  const raw = window.localStorage.getItem(USER_KEY);
  if (!raw) return null;
  try {
    return JSON.parse(raw) as AuthUser;
  } catch {
    window.localStorage.removeItem(USER_KEY);
    return null;
  }
}

export const useAppStore = create<AppStore>((set) => ({
  sidebarCollapsed: false,
  accessToken: window.localStorage.getItem(ACCESS_TOKEN_KEY),
  refreshToken: window.localStorage.getItem(REFRESH_TOKEN_KEY),
  user: readStoredUser(),
  toggleSidebar: () => set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed })),
  setSession: ({ accessToken, refreshToken, user }) => {
    setApiAccessToken(accessToken);
    setApiRefreshToken(refreshToken ?? null);
    window.localStorage.setItem(USER_KEY, JSON.stringify(user));
    set({ accessToken, refreshToken: refreshToken ?? null, user });
  },
  clearSession: () => {
    setApiAccessToken(null);
    setApiRefreshToken(null);
    window.localStorage.removeItem(USER_KEY);
    set({ accessToken: null, refreshToken: null, user: null });
  },
}));