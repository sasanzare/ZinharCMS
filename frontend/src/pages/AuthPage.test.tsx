import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { MemoryRouter } from "react-router";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { I18nProvider } from "../i18n";
import { AuthPage } from "./AuthPage";

const authMocks = vi.hoisted(() => ({
  login: vi.fn(),
  register: vi.fn(),
  verifyMfa: vi.fn(),
  setSession: vi.fn(),
}));

vi.mock("../services/api", () => ({
  ApiError: class ApiError extends Error {},
  api: {
    auth: {
      login: authMocks.login,
      register: authMocks.register,
      verifyMfa: authMocks.verifyMfa,
    },
  },
}));

vi.mock("../stores/useAppStore", () => ({
  useAppStore: (selector: (state: unknown) => unknown) =>
    selector({
      accessToken: null,
      authStatus: "unauthenticated",
      setSession: authMocks.setSession,
    }),
}));

describe("AuthPage", () => {
  afterEach(cleanup);

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("does not prefill a deterministic administrator identity or password", () => {
    render(
      <I18nProvider>
        <MemoryRouter initialEntries={["/login"]}>
          <AuthPage />
        </MemoryRouter>
      </I18nProvider>,
    );

    expect(screen.getByLabelText("Email")).toHaveValue("");
    expect(screen.getByLabelText("Password")).toHaveValue("");
  });

  it("keeps pre-auth state in memory and creates a session only after MFA", async () => {
    authMocks.login.mockResolvedValue({
      mfa_required: true,
      pre_auth_token: "memory-only-pre-auth",
      expires_in: 300,
      methods: ["totp", "recovery"],
    });
    authMocks.verifyMfa.mockResolvedValue({
      access_token: "access-token",
      token_type: "Bearer",
      expires_in: 3600,
      user: {
        id: "user-1",
        email: "user@example.invalid",
        name: "User",
        avatar_url: null,
        role: "author",
      },
      organizations: [],
      default_organization_id: null,
    });
    render(
      <I18nProvider>
        <MemoryRouter initialEntries={["/login"]}>
          <AuthPage />
        </MemoryRouter>
      </I18nProvider>,
    );

    fireEvent.change(screen.getByLabelText("Email"), {
      target: { value: "user@example.invalid" },
    });
    fireEvent.change(screen.getByLabelText("Password"), {
      target: { value: "password-value" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Enter admin" }));

    expect(await screen.findByLabelText("Six-digit code")).toBeInTheDocument();
    expect(authMocks.setSession).not.toHaveBeenCalled();
    const storedValues = [window.localStorage, window.sessionStorage].flatMap(
      (storage) =>
        Array.from({ length: storage.length }, (_, index) => {
          const key = storage.key(index);
          return key ? storage.getItem(key) : null;
        }),
    );
    expect(storedValues).not.toContain("memory-only-pre-auth");

    fireEvent.change(screen.getByLabelText("Six-digit code"), {
      target: { value: "012345" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Verify and sign in" }));

    await waitFor(() =>
      expect(authMocks.verifyMfa).toHaveBeenCalledWith(
        "memory-only-pre-auth",
        "totp",
        "012345",
      ),
    );
    expect(authMocks.setSession).toHaveBeenCalledTimes(1);
  });

  it("expires pre-auth state and never places the challenge in storage or the URL", async () => {
    authMocks.login.mockResolvedValue({
      mfa_required: true,
      pre_auth_token: "short-lived-pre-auth",
      expires_in: 0.01,
      methods: ["totp", "recovery"],
    });
    render(
      <I18nProvider>
        <MemoryRouter initialEntries={["/login"]}>
          <AuthPage />
        </MemoryRouter>
      </I18nProvider>,
    );

    fireEvent.change(screen.getByLabelText("Email"), {
      target: { value: "user@example.invalid" },
    });
    fireEvent.change(screen.getByLabelText("Password"), {
      target: { value: "password-value" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Enter admin" }));

    expect(await screen.findByLabelText("Six-digit code")).toBeInTheDocument();
    await waitFor(() =>
      expect(screen.getByText("The MFA challenge expired. Sign in again.")).toBeInTheDocument(),
    );
    expect(screen.getByLabelText("Email")).toBeInTheDocument();
    expect(authMocks.setSession).not.toHaveBeenCalled();
    expect(window.location.href).not.toContain("short-lived-pre-auth");
    for (const storage of [window.localStorage, window.sessionStorage]) {
      const values = Array.from({ length: storage.length }, (_, index) => {
        const key = storage.key(index);
        return key ? storage.getItem(key) : null;
      });
      expect(values).not.toContain("short-lived-pre-auth");
    }
  });

  it("supports recovery mode and clears a submitted recovery code after failure", async () => {
    authMocks.login.mockResolvedValue({
      mfa_required: true,
      pre_auth_token: "recovery-pre-auth",
      expires_in: 300,
      methods: ["totp", "recovery"],
    });
    authMocks.verifyMfa.mockRejectedValue(new Error("invalid proof"));
    render(
      <I18nProvider>
        <MemoryRouter initialEntries={["/login"]}>
          <AuthPage />
        </MemoryRouter>
      </I18nProvider>,
    );

    fireEvent.change(screen.getByLabelText("Email"), {
      target: { value: "user@example.invalid" },
    });
    fireEvent.change(screen.getByLabelText("Password"), {
      target: { value: "password-value" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Enter admin" }));
    await screen.findByLabelText("Six-digit code");

    fireEvent.change(screen.getByLabelText("Verification method"), {
      target: { value: "recovery" },
    });
    const recoveryInput = screen.getByLabelText("Recovery code");
    fireEvent.change(recoveryInput, { target: { value: "<RECOVERY_CODE>" } });
    fireEvent.click(screen.getByRole("button", { name: "Verify and sign in" }));

    await waitFor(() =>
      expect(authMocks.verifyMfa).toHaveBeenCalledWith(
        "recovery-pre-auth",
        "recovery",
        "<RECOVERY_CODE>",
      ),
    );
    await waitFor(() => expect(recoveryInput).toHaveValue(""));
    expect(authMocks.setSession).not.toHaveBeenCalled();
  });
});
