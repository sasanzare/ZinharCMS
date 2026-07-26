import { render, screen } from "@testing-library/react";
import { MemoryRouter } from "react-router";
import { describe, expect, it, vi } from "vitest";

import { I18nProvider } from "../i18n";
import { AuthPage } from "./AuthPage";

const authMocks = vi.hoisted(() => ({
  login: vi.fn(),
  register: vi.fn(),
  setSession: vi.fn(),
}));

vi.mock("../services/api", () => ({
  ApiError: class ApiError extends Error {},
  api: {
    auth: {
      login: authMocks.login,
      register: authMocks.register,
    },
  },
}));

vi.mock("../stores/useAppStore", () => ({
  useAppStore: (selector: (state: unknown) => unknown) =>
    selector({
      accessToken: null,
      setSession: authMocks.setSession,
    }),
}));

describe("AuthPage", () => {
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
});
