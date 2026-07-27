import { render, screen } from "@testing-library/react";
import { MemoryRouter, Route, Routes, useLocation } from "react-router";
import { beforeEach, describe, expect, it, vi } from "vitest";

import { RequireAuth } from "./RequireAuth";

let authStatus = "unknown";

vi.mock("../stores/useAppStore", () => ({
  useAppStore: (selector: (state: unknown) => unknown) =>
    selector({ authStatus }),
}));

vi.mock("./AppShell", () => ({
  AppShell: () => <div>protected application</div>,
}));

function LocationProbe() {
  const location = useLocation();
  return (
    <div>
      {location.pathname}
      {String((location.state as { from?: string } | null)?.from ?? "")}
    </div>
  );
}

describe("RequireAuth", () => {
  beforeEach(() => {
    authStatus = "unknown";
  });

  it("does not flash protected UI or redirect while bootstrap is unresolved", () => {
    render(
      <MemoryRouter initialEntries={["/pages?draft=1"]}>
        <RequireAuth />
      </MemoryRouter>,
    );

    expect(screen.getByText("Restoring session…")).toBeInTheDocument();
    expect(screen.queryByText("protected application")).not.toBeInTheDocument();
  });

  it("preserves the target route when an unauthenticated bootstrap redirects", () => {
    authStatus = "unauthenticated";
    render(
      <MemoryRouter initialEntries={["/pages?draft=1"]}>
        <Routes>
          <Route path="/pages" element={<RequireAuth />} />
          <Route path="/login" element={<LocationProbe />} />
        </Routes>
      </MemoryRouter>,
    );

    expect(screen.getByText("/login/pages?draft=1")).toBeInTheDocument();
  });

  it("renders protected UI only after authentication is established", () => {
    authStatus = "authenticated";
    render(
      <MemoryRouter initialEntries={["/pages"]}>
        <RequireAuth />
      </MemoryRouter>,
    );

    expect(screen.getByText("protected application")).toBeInTheDocument();
  });
});
