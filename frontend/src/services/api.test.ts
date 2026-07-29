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

function failedResponse(status: number, error: string) {
  return {
    ok: false,
    status,
    statusText: status === 401 ? "Unauthorized" : "Forbidden",
    json: vi.fn().mockResolvedValue({ error, message: error }),
  };
}

describe("browser authentication contract", () => {
  beforeEach(() => {
    vi.resetModules();
    window.localStorage.clear();
    vi.stubGlobal("fetch", vi.fn());
  });

  it("removes legacy browser-readable access and refresh tokens", async () => {
    window.localStorage.setItem("zinhar.access_token", "legacy-access-token");
    window.localStorage.setItem("zinhar.refresh_token", "legacy-refresh-token");

    await import("./api");

    expect(window.localStorage.getItem("zinhar.access_token")).toBeNull();
    expect(window.localStorage.getItem("zinhar.refresh_token")).toBeNull();
  });

  it("keeps the access token in memory only", async () => {
    const { setApiAccessToken } = await import("./api");

    setApiAccessToken("memory-only-access-token");

    expect(window.localStorage.getItem("zinhar.access_token")).toBeNull();
    expect(window.sessionStorage.getItem("zinhar.access_token")).toBeNull();
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

  it("single-flights concurrent refreshes and retries each request only once", async () => {
    const fetchMock = vi.mocked(fetch);
    let refreshCount = 0;
    fetchMock.mockImplementation(async (input, init) => {
      const url = String(input);
      if (url.endsWith("/api/auth/refresh")) {
        refreshCount += 1;
        return successfulResponse(authResponse) as unknown as Response;
      }
      const authorization = new Headers(init?.headers).get("Authorization");
      if (authorization === "Bearer access-token") {
        return successfulResponse({ user: authResponse.user, organizations: [] }) as unknown as Response;
      }
      return failedResponse(401, "access_token_invalid") as unknown as Response;
    });
    const { api, setApiAccessToken } = await import("./api");
    setApiAccessToken("expired-access-token");

    await Promise.all([api.auth.me(), api.auth.me()]);

    expect(refreshCount).toBe(1);
    expect(fetchMock).toHaveBeenCalledTimes(5);
  });

  it("does not refresh for authorization failures or generic unauthorized responses", async () => {
    const fetchMock = vi.mocked(fetch);
    fetchMock
      .mockResolvedValueOnce(failedResponse(403, "forbidden") as unknown as Response)
      .mockResolvedValueOnce(failedResponse(401, "unauthorized") as unknown as Response);
    const { api, setApiAccessToken } = await import("./api");
    setApiAccessToken("access-token");

    await expect(api.auth.me()).rejects.toMatchObject({ status: 403 });
    await expect(api.auth.me()).rejects.toMatchObject({ status: 401 });

    expect(fetchMock).toHaveBeenCalledTimes(2);
  });

  it("never attaches Authorization to an absolute untrusted origin", async () => {
    const fetchMock = vi.mocked(fetch);
    fetchMock.mockResolvedValueOnce(successfulResponse({ ok: true }) as unknown as Response);
    const { requestForTest, setApiAccessToken } = await import("./api");
    setApiAccessToken("access-token");

    await requestForTest("https://untrusted.example.invalid/resource", {
      auth: true,
      stepUpToken: "one-time-step-up",
    });

    expect(new Headers(fetchMock.mock.calls[0]?.[1]?.headers).has("Authorization")).toBe(false);
    expect(new Headers(fetchMock.mock.calls[0]?.[1]?.headers).has("X-Step-Up-Token")).toBe(false);
  });

  it("sends a one-time step-up grant only to the configured API origin", async () => {
    const fetchMock = vi.mocked(fetch);
    fetchMock.mockResolvedValueOnce(successfulResponse({ ok: true }) as unknown as Response);
    const { requestForTest, setApiAccessToken } = await import("./api");
    setApiAccessToken("access-token");

    await requestForTest("/api/auth/logout-all", {
      method: "POST",
      auth: true,
      stepUpToken: "one-time-step-up",
    });

    const headers = new Headers(fetchMock.mock.calls[0]?.[1]?.headers);
    expect(headers.get("X-Step-Up-Token")).toBe("one-time-step-up");
    expect(window.localStorage.getItem("X-Step-Up-Token")).toBeNull();
    expect(window.sessionStorage.getItem("X-Step-Up-Token")).toBeNull();
  });
});
