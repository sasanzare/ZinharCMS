import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const previewMocks = vi.hoisted(() => ({
  issueTicket: vi.fn(),
}));

vi.mock("./api", () => ({
  ApiError: class ApiError extends Error {
    constructor(public status: number) {
      super("api error");
    }
  },
  api: {
    baseUrl: "https://api.example.invalid",
    pages: {
      previewTicket: previewMocks.issueTicket,
    },
  },
}));

vi.mock("./authSession", () => ({
  subscribeBrowserSession: () => () => undefined,
}));

import {
  PREVIEW_APPLICATION_PROTOCOL,
  connectPreviewSocket,
  previewProtocols,
  previewWebSocketUrl,
} from "./previewSocket";

type SocketListener = (event: Event & { code?: number; data?: unknown }) => void;

class FakeWebSocket {
  static instances: FakeWebSocket[] = [];
  readonly listeners = new Map<string, SocketListener[]>();
  protocol = "";

  constructor(
    public readonly url: string,
    public readonly protocols: string[],
  ) {
    FakeWebSocket.instances.push(this);
  }

  addEventListener(type: string, listener: SocketListener) {
    this.listeners.set(type, [...(this.listeners.get(type) ?? []), listener]);
  }

  close(code = 1000) {
    this.emit("close", { code });
  }

  emit(type: string, values: { code?: number; data?: unknown } = {}) {
    const event = Object.assign(new Event(type), values);
    for (const listener of this.listeners.get(type) ?? []) listener(event);
  }
}

describe("preview WebSocket authentication", () => {
  beforeEach(() => {
    vi.useFakeTimers();
    FakeWebSocket.instances = [];
    previewMocks.issueTicket.mockReset();
    vi.stubGlobal("WebSocket", FakeWebSocket);
  });

  afterEach(() => {
    vi.useRealTimers();
    vi.unstubAllGlobals();
  });

  it("keeps credentials and organization context out of the URL", () => {
    const url = previewWebSocketUrl(
      "https://api.example.invalid",
      "01900000-0000-7000-8000-000000000002",
    );

    expect(url).toBe(
      "wss://api.example.invalid/api/preview/01900000-0000-7000-8000-000000000002",
    );
    expect(url).not.toContain("?");
  });

  it("offers the stable application protocol and one ticket protocol", () => {
    const protocols = previewProtocols("opaque-ticket");

    expect(protocols).toEqual([
      PREVIEW_APPLICATION_PROTOCOL,
      "zinhar.ticket.opaque-ticket",
    ]);
  });

  it("obtains a fresh ticket for each bounded reconnect", async () => {
    previewMocks.issueTicket
      .mockResolvedValueOnce({
        ticket: "first-ticket",
        expires_in: 30,
        protocol: PREVIEW_APPLICATION_PROTOCOL,
      })
      .mockResolvedValueOnce({
        ticket: "second-ticket",
        expires_in: 30,
        protocol: PREVIEW_APPLICATION_PROTOCOL,
      });

    const controller = connectPreviewSocket("page-id");
    await vi.advanceTimersByTimeAsync(0);
    expect(FakeWebSocket.instances[0]?.url).not.toContain("?");
    expect(FakeWebSocket.instances[0]?.protocols).toEqual([
      PREVIEW_APPLICATION_PROTOCOL,
      "zinhar.ticket.first-ticket",
    ]);

    FakeWebSocket.instances[0]?.emit("close", { code: 1006 });
    await vi.advanceTimersByTimeAsync(250);

    expect(previewMocks.issueTicket).toHaveBeenCalledTimes(2);
    expect(FakeWebSocket.instances[1]?.protocols[1]).toBe(
      "zinhar.ticket.second-ticket",
    );
    controller.close();
  });

  it("stops reconnecting after a definitive policy rejection", async () => {
    previewMocks.issueTicket.mockResolvedValue({
      ticket: "one-ticket",
      expires_in: 30,
      protocol: PREVIEW_APPLICATION_PROTOCOL,
    });
    connectPreviewSocket("page-id");
    await vi.advanceTimersByTimeAsync(0);

    FakeWebSocket.instances[0]?.emit("close", { code: 1008 });
    await vi.advanceTimersByTimeAsync(10_000);

    expect(previewMocks.issueTicket).toHaveBeenCalledTimes(1);
  });
});
