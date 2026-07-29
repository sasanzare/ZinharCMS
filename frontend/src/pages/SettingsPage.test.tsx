import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { SettingsPage } from "./SettingsPage";

const mocks = vi.hoisted(() => ({
  clearSession: vi.fn(),
  listSessions: vi.fn(),
  logout: vi.fn(),
  logoutAll: vi.fn(),
  createStepUp: vi.fn(),
  verifyStepUp: vi.fn(),
  startMfaEnrollment: vi.fn(),
  confirmMfaEnrollment: vi.fn(),
  regenerateMfaRecoveryCodes: vi.fn(),
  disableMfa: vi.fn(),
  mfaStatus: vi.fn(),
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
      createStepUp: mocks.createStepUp,
      verifyStepUp: mocks.verifyStepUp,
      mfaStatus: mocks.mfaStatus,
      startMfaEnrollment: mocks.startMfaEnrollment,
      confirmMfaEnrollment: mocks.confirmMfaEnrollment,
      regenerateMfaRecoveryCodes: mocks.regenerateMfaRecoveryCodes,
      disableMfa: mocks.disableMfa,
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

async function completeStepUp() {
  const code = await screen.findByLabelText("Six-digit code");
  fireEvent.change(code, { target: { value: "123456" } });
  fireEvent.click(screen.getByRole("button", { name: "Verify and continue" }));
}

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
    mocks.createStepUp.mockResolvedValue({
      challenge: "step-up-challenge",
      expires_in: 300,
      scope: "session_logout_all",
    });
    mocks.verifyStepUp.mockResolvedValue({
      step_up_token: "step-up-grant",
      expires_in: 300,
      scope: "session_logout_all",
    });
    mocks.mfaStatus.mockResolvedValue({
      enabled: true,
      enrollment_pending: false,
      recovery_codes_remaining: 10,
      required_for_privileged_actions: false,
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
    await completeStepUp();
    await waitFor(() =>
      expect(mocks.revokeSession).toHaveBeenCalledWith(
        otherSession.session_id,
        "step-up-grant",
      ),
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
    expect(mocks.logoutAll).not.toHaveBeenCalled();
    const code = await screen.findByLabelText("Six-digit code");
    fireEvent.change(code, { target: { value: "123456" } });
    fireEvent.click(screen.getByRole("button", { name: "Verify and continue" }));
    await waitFor(() =>
      expect(mocks.logoutAll).toHaveBeenCalledWith("step-up-grant"),
    );

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
    await completeStepUp();
    await waitFor(() => expect(mocks.clearSession).toHaveBeenCalledTimes(1));
    expect(mocks.revokeSession).toHaveBeenCalledWith(
      currentSession.session_id,
      "step-up-grant",
    );
  });

  it("keeps pending enrollment secret material in memory and shows recovery codes once", async () => {
    const recoveryCodes = Array.from(
      { length: 10 },
      (_, index) => `<RECOVERY_CODE_${index + 1}>`,
    );
    mocks.mfaStatus.mockResolvedValue({
      enabled: false,
      enrollment_pending: false,
      recovery_codes_remaining: 0,
      required_for_privileged_actions: true,
    });
    mocks.startMfaEnrollment.mockResolvedValue({
      enrollment_id: "10000000-0000-7000-8000-000000000099",
      qr_code_base64: "cGxhY2Vob2xkZXI=",
      manual_secret: "<TOTP_SECRET>",
      provisioning_uri: "<PROVISIONING_URI>",
      expires_in: 600,
    });
    mocks.confirmMfaEnrollment.mockResolvedValue({
      enabled: true,
      recovery_codes: recoveryCodes,
    });
    const { unmount } = render(<SettingsPage />);

    expect(
      await screen.findByText("MFA enrollment is required for privileged actions."),
    ).toBeInTheDocument();
    fireEvent.change(screen.getByLabelText("Confirm your password"), {
      target: { value: "password-value" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Set up authenticator" }));

    const manualSecret = await screen.findByLabelText("Manual setup key");
    expect(manualSecret).toHaveValue("<TOTP_SECRET>");
    expect(screen.getByText("Disabled")).toBeInTheDocument();
    for (const storage of [window.localStorage, window.sessionStorage]) {
      const values = Array.from({ length: storage.length }, (_, index) => {
        const key = storage.key(index);
        return key ? storage.getItem(key) : null;
      });
      expect(values).not.toContain("<TOTP_SECRET>");
      expect(values).not.toContain("<PROVISIONING_URI>");
    }

    fireEvent.change(screen.getByLabelText("Six-digit confirmation code"), {
      target: { value: "012345" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Enable MFA" }));
    await waitFor(() =>
      expect(mocks.confirmMfaEnrollment).toHaveBeenCalledWith("012345"),
    );
    for (const code of recoveryCodes) {
      expect(await screen.findByText(code)).toBeInTheDocument();
    }
    expect(screen.getByText("Enabled")).toBeInTheDocument();

    fireEvent.click(
      screen.getByRole("checkbox", {
        name: "I saved every recovery code in a secure place.",
      }),
    );
    fireEvent.click(screen.getByRole("button", { name: "Continue" }));
    await waitFor(() => expect(mocks.clearSession).toHaveBeenCalledTimes(1));
    expect(screen.queryByText(recoveryCodes[0])).toBeNull();
    unmount();
    expect(window.localStorage.length).toBe(0);
    expect(window.sessionStorage.length).toBe(0);
  });

  it("requires the exact Step-Up flow before disabling MFA", async () => {
    mocks.disableMfa.mockResolvedValue({
      disabled: true,
      sessions_revoked: true,
    });
    render(<SettingsPage />);

    const disableButton = await screen.findByRole("button", { name: "Disable MFA" });
    fireEvent.click(disableButton);
    expect(mocks.disableMfa).not.toHaveBeenCalled();
    await completeStepUp();

    await waitFor(() =>
      expect(mocks.disableMfa).toHaveBeenCalledWith("step-up-grant"),
    );
    expect(mocks.createStepUp).toHaveBeenCalledWith("mfa_disable");
    expect(mocks.clearSession).toHaveBeenCalledTimes(1);
  });
});
