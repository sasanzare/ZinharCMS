import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { DashboardPage } from "./DashboardPage";

vi.mock("../hooks/useHealth", () => ({
  useHealth: () => ({
    health: { status: "ok", version: "0.1.0" },
    readiness: {
      status: "ready",
      checks: [
        { name: "postgres", ok: true, message: "reachable" },
        { name: "redis", ok: true, message: "reachable" },
      ],
    },
    loading: false,
    error: null,
  }),
}));

describe("DashboardPage", () => {
  it("renders foundation cards", () => {
    render(<DashboardPage />);
    expect(screen.getByText("Axum 0.8")).toBeInTheDocument();
    expect(screen.getByText("PostgreSQL 16")).toBeInTheDocument();
    expect(screen.getByText("Redis 7")).toBeInTheDocument();
  });
});
