import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { SettingsPage } from "./SettingsPage";

const mocks = vi.hoisted(() => ({
  clearSession: vi.fn(),
  listSessions: vi.fn(),
  logout: vi.fn(),
  logoutAll: vi.fn(),
  me: vi.fn(),
  revokeSession: vi.fn(),
}));

vi.mock("../hooks/useHealth", () => ({
  useHealth: () => ({ readiness: { status: "ready" } }),
}));

vi.mock("../stores/useAppStore", () => ({
  useAppStore: (selector: (state: unknown) => unknown) =>
    selector({
      user: {
        id: "user-1",
        email: "user@example.invalid",
        name: "Test User",
        avatar_url: null,
        role: "author",
      },
      clearSession: mocks.clearSession,
    }),
}));

vi.mock("../services/api", () => ({
  ApiError: class ApiError extends Error {},
  api: {
    baseUrl: "http://localhost:8080",
    auth: {
      logout: mocks.logout,
      logoutAll: mocks.logoutAll,
      me: mocks.me,
      revokeSession: mocks.revokeSession,
      sessions: mocks.listSessions,
    },
    webhooks: {
      create: vi.fn(),
      delete: vi.fn(),
      list: vi.fn().mockResolvedValue([]),
      test: vi.fn(),
      update: vi.fn(),
    },
  },
}));

const currentSession = {
  session_id: "10000000-0000-7000-8000-000000000001",
  created_at: "2026-07-28T08:00:00Z",
  last_used_at: "2026-07-28T08:30:00Z",
  expires_at: "2026-08-04T08:00:00Z",
  current: true,
  revoked: false,
  compromised: false,
};

const otherSession = {
  ...currentSession,
  session_id: "10000000-0000-7000-8000-000000000002",
  created_at: "<img data-session-xss src=x onerror=alert(1)>",
  current: false,
};

describe("SettingsPage session management", () => {
  afterEach(cleanup);

  beforeEach(() => {
    vi.clearAllMocks();
    mocks.me.mockResolvedValue({
      user: {
        id: "user-1",
        email: "user@example.invalid",
        name: "Test User",
        avatar_url: null,
        role: "author",
      },
    });
    mocks.listSessions.mockResolvedValue({
      sessions: [currentSession, otherSession],
      total: 2,
      page: 1,
      per_page: 20,
    });
    mocks.revokeSession.mockResolvedValue({
      revoked: true,
      current_session: false,
    });
    mocks.logoutAll.mockResolvedValue({
      revoked_sessions: 2,
      auth_version: 3,
    });
    vi.spyOn(window, "confirm").mockReturnValue(true);
  });

  it("renders plain session metadata, identifies current, and revokes another session", async () => {
    render(<SettingsPage />);

    const panel = await screen.findByRole("region", { name: "Active sessions" });
    expect(within(panel).getByText("Current session")).toBeInTheDocument();
    expect(within(panel).getByText(otherSession.created_at)).toBeInTheDocument();
    expect(document.querySelector("img[data-session-xss]")).toBeNull();
    expect(panel.textContent).not.toContain("token");

    const otherRow = within(panel).getByTestId(`session-${otherSession.session_id}`);
    fireEvent.click(within(otherRow).getByRole("button", { name: "Revoke session" }));
    await waitFor(() =>
      expect(mocks.revokeSession).toHaveBeenCalledWith(otherSession.session_id),
    );
    expect(mocks.listSessions).toHaveBeenCalledTimes(2);
  });

  it("confirms logout-all, disables duplicate submission, and clears browser auth", async () => {
    let resolveLogoutAll: ((value: unknown) => void) | undefined;
    mocks.logoutAll.mockReturnValue(
      new Promise((resolve) => {
        resolveLogoutAll = resolve;
      }),
    );
    render(<SettingsPage />);
    const button = await screen.findByRole("button", { name: "Log out all sessions" });
    fireEvent.click(button);
    fireEvent.click(button);
    expect(mocks.logoutAll).toHaveBeenCalledTimes(1);
    expect(button).toBeDisabled();

    resolveLogoutAll?.({ revoked_sessions: 2, auth_version: 3 });
    await waitFor(() => expect(mocks.clearSession).toHaveBeenCalledTimes(1));
    expect(window.confirm).toHaveBeenCalledTimes(1);
  });

  it("clears browser auth when the current session is revoked", async () => {
    mocks.revokeSession.mockResolvedValue({
      revoked: true,
      current_session: true,
    });
    render(<SettingsPage />);
    const panel = await screen.findByRole("region", { name: "Active sessions" });
    const currentRow = within(panel).getByTestId(`session-${currentSession.session_id}`);
    fireEvent.click(within(currentRow).getByRole("button", { name: "Revoke session" }));
    await waitFor(() => expect(mocks.clearSession).toHaveBeenCalledTimes(1));
    expect(mocks.revokeSession).toHaveBeenCalledWith(currentSession.session_id);
  });
});
