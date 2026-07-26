import { render, screen } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";

import { DashboardPage } from "./DashboardPage";

const apiMocks = vi.hoisted(() => ({
  billingUsage: vi.fn(),
  contentTypes: vi.fn(),
  entries: vi.fn(),
  media: vi.fn(),
  pages: vi.fn(),
}));

vi.mock("../services/api", () => ({
  ApiError: class ApiError extends Error {},
  api: {
    billing: { usage: apiMocks.billingUsage },
    contentTypes: { list: apiMocks.contentTypes },
    entries: { list: apiMocks.entries },
    media: { list: apiMocks.media },
    pages: { list: apiMocks.pages },
  },
}));

vi.mock("../hooks/useHealth", () => ({
  useHealth: () => ({
    health: { status: "ok", version: "3.0.0" },
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

const usageMetric = (metric: string, used: number, limit: number) => ({
  metric,
  used,
  limit,
  percent: (used / limit) * 100,
  near_limit: false,
  exceeded: false,
});

describe("DashboardPage", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    apiMocks.contentTypes.mockResolvedValue([{ slug: "articles" }]);
    apiMocks.entries.mockResolvedValue({ data: [{ id: "entry-1" }, { id: "entry-2" }] });
    apiMocks.media.mockResolvedValue({ data: [{ id: "media-1" }, { id: "media-2" }, { id: "media-3" }] });
    apiMocks.pages.mockResolvedValue({
      data: [
        { id: "page-1", status: "published" },
        { id: "page-2", status: "published" },
        { id: "page-3", status: "published" },
        { id: "page-4", status: "published" },
      ],
    });
    apiMocks.billingUsage.mockResolvedValue({
      members: usageMetric("members", 7, 10),
      content_records: usageMetric("content_records", 12, 100),
      media_bytes: usageMetric("media_bytes", 1024 * 1024, 10 * 1024 * 1024),
      api_requests: usageMetric("api_requests", 25, 1000),
    });
  });

  it("renders foundation cards after dashboard data settles", async () => {
    render(<DashboardPage />);

    expect(screen.getByText("Axum 0.8")).toBeInTheDocument();
    expect(screen.getByText("PostgreSQL 16")).toBeInTheDocument();
    expect(screen.getByText("Redis 7")).toBeInTheDocument();

    expect(await screen.findByText("7")).toBeInTheDocument();
    expect(await screen.findByText("2")).toBeInTheDocument();
    expect(apiMocks.contentTypes).toHaveBeenCalledTimes(1);
    expect(apiMocks.entries).toHaveBeenCalledWith("articles", { per_page: 100 });
    expect(apiMocks.media).toHaveBeenCalledWith({ per_page: 1 });
    expect(apiMocks.pages).toHaveBeenCalledWith({ per_page: 100 });
    expect(apiMocks.billingUsage).toHaveBeenCalledTimes(1);
  });
});
