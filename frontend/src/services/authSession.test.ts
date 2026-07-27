import { beforeEach, describe, expect, it, vi } from "vitest";

const response = {
  access_token: "memory-access-token",
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

describe("auth session coordination", () => {
  beforeEach(() => {
    vi.resetModules();
    window.localStorage.clear();
    window.sessionStorage.clear();
  });

  it("single-flights refresh work within a tab", async () => {
    const refresh = vi.fn().mockResolvedValue(response);
    const { coordinatedRefresh } = await import("./authSession");

    const [first, second] = await Promise.all([
      coordinatedRefresh(refresh),
      coordinatedRefresh(refresh),
    ]);

    expect(first.access_token).toBe("memory-access-token");
    expect(second.access_token).toBe("memory-access-token");
    expect(refresh).toHaveBeenCalledTimes(1);
  });

  it("broadcasts logout without writing token material to storage", async () => {
    const { acceptBrowserSession, clearBrowserSession } = await import("./authSession");

    acceptBrowserSession(response, false);
    clearBrowserSession(false);

    expect(window.localStorage.getItem("zinhar.access_token")).toBeNull();
    expect(window.sessionStorage.getItem("zinhar.access_token")).toBeNull();
  });

  it("rejects every in-tab waiter after one failed refresh", async () => {
    const refresh = vi.fn().mockRejectedValue(new Error("refresh failed"));
    const { coordinatedRefresh } = await import("./authSession");

    const results = await Promise.allSettled([
      coordinatedRefresh(refresh),
      coordinatedRefresh(refresh),
    ]);

    expect(results.every((result) => result.status === "rejected")).toBe(true);
    expect(refresh).toHaveBeenCalledTimes(1);
  });

  it("ignores attacker-controlled storage events", async () => {
    const listener = vi.fn();
    const { subscribeBrowserSession } = await import("./authSession");
    const unsubscribe = subscribeBrowserSession(listener);

    window.dispatchEvent(
      new StorageEvent("storage", {
        key: "zinhar.access_token",
        newValue: "attacker-controlled-token",
      }),
    );

    expect(listener).not.toHaveBeenCalled();
    unsubscribe();
  });
});
