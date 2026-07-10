import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { PagesPage } from "./PagesPage";

vi.mock("../services/api", () => ({
  ApiError: class ApiError extends Error {
    status = 500;
  },
  setApiAccessToken: vi.fn(),
  setApiOrganizationId: vi.fn(),
  setApiRefreshToken: vi.fn(),
  api: {
    baseUrl: "http://localhost:8080",
    pages: {
      list: vi.fn().mockResolvedValue({ data: [], page: 1, per_page: 20 }),
      create: vi.fn(),
      update: vi.fn(),
      delete: vi.fn(),
      publish: vi.fn(),
      unpublish: vi.fn(),
      versions: vi.fn().mockResolvedValue([]),
      restore: vi.fn(),
    },
    components: {
      list: vi.fn().mockResolvedValue([
        {
          id: "component-1",
          component_key: "hero-banner",
          name: "Hero Banner",
          category: "sections",
          props_schema: {
            title: { type: "text", label: "Title", required: true, default: "Welcome" },
            alignment: { type: "select", label: "Alignment", options: ["left", "center", "right"] },
          },
          is_system: true,
          created_at: "2026-06-19T00:00:00Z",
          updated_at: "2026-06-19T00:00:00Z",
        },
      ]),
    },
    marketplaceAdapters: {
      components: vi.fn().mockResolvedValue([
        {
          id: "marketplace-component-1",
          installation_id: "installation-1",
          component_key: "mp-pack-hero",
          name: "Marketplace Hero",
          category: "sections",
          props_schema: {},
          is_system: false,
          listing_title: "Pack",
          version: "1.0.0",
          enabled: true,
          created_at: "2026-07-10T00:00:00Z",
          updated_at: "2026-07-10T00:00:00Z",
        },
      ]),
    },
    marketplace: {
      installations: vi.fn().mockResolvedValue([]),
    },
  },
}));

describe("PagesPage", () => {
  it("renders the phase four page builder shell", async () => {
    render(<PagesPage />);

    expect(await screen.findByText("New page builder")).toBeInTheDocument();
    expect(await screen.findByText("Hero Banner")).toBeInTheDocument();
    expect(await screen.findByText("Marketplace Hero")).toBeInTheDocument();
    expect(screen.getByText("Drop components here")).toBeInTheDocument();
    expect(screen.getByText("Props editor")).toBeInTheDocument();
  });
});
