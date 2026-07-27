import { beforeEach, describe, expect, it, vi } from "vitest";

const authResponse = {
  access_token: "access-token",
  token_type: "Bearer",
  expires_in: 3600,
  user: {
    id: "01900000-0000-7000-8000-000000000001",
    email: "user@example.invalid",
    name: "Test User",
    avatar_url: null,
    role: "author",
  },
  organizations: [],
  default_organization_id: null,
};

function successfulResponse(payload: unknown) {
  return {
    ok: true,
    status: 200,
    statusText: "OK",
    json: vi.fn().mockResolvedValue(payload),
  };
}

describe("auth refresh cookie contract", () => {
  beforeEach(() => {
    vi.resetModules();
    window.localStorage.clear();
    vi.stubGlobal("fetch", vi.fn());
  });

  it("removes the legacy browser-readable refresh token", async () => {
    window.localStorage.setItem("zinhar.refresh_token", "legacy-refresh-token");

    await import("./api");

    expect(window.localStorage.getItem("zinhar.refresh_token")).toBeNull();
  });

  it("never sends a refresh token in refresh or logout request bodies", async () => {
    const fetchMock = vi.mocked(fetch);
    fetchMock
      .mockResolvedValueOnce(successfulResponse(authResponse) as unknown as Response)
      .mockResolvedValueOnce(
        successfulResponse({ revoked: true }) as unknown as Response,
      );
    const { api } = await import("./api");

    await (api.auth.refresh as (...args: unknown[]) => Promise<unknown>)(
      "legacy-refresh-token",
    );
    await (api.auth.logout as (...args: unknown[]) => Promise<unknown>)(
      "legacy-refresh-token",
    );

    expect(fetchMock.mock.calls[0]?.[1]?.body).toBeUndefined();
    expect(fetchMock.mock.calls[1]?.[1]?.body).toBeUndefined();
    expect(fetchMock.mock.calls[0]?.[1]?.credentials).toBe("include");
    expect(fetchMock.mock.calls[1]?.[1]?.credentials).toBe("include");
  });
});
