import { ApiError, api } from "./api";
import { subscribeBrowserSession } from "./authSession";

export const PREVIEW_APPLICATION_PROTOCOL = "zinhar.preview.v1";
const PREVIEW_TICKET_PROTOCOL_PREFIX = "zinhar.ticket.";
const MAX_RECONNECT_ATTEMPTS = 4;

export type PreviewSocketStatus =
  | "connecting"
  | "connected"
  | "reconnecting"
  | "rejected"
  | "closed";

export type PreviewSocketHandlers = {
  onMessage?: (payload: unknown) => void;
  onStatus?: (status: PreviewSocketStatus) => void;
};

export type PreviewSocketController = {
  close: () => void;
};

export function previewWebSocketUrl(apiBaseUrl: string, pageId: string) {
  const url = new URL(`/api/preview/${encodeURIComponent(pageId)}`, apiBaseUrl);
  url.protocol = url.protocol === "https:" ? "wss:" : "ws:";
  url.search = "";
  url.hash = "";
  return url.toString().replace(/\/$/, "");
}

export function previewProtocols(ticket: string) {
  return [
    PREVIEW_APPLICATION_PROTOCOL,
    `${PREVIEW_TICKET_PROTOCOL_PREFIX}${ticket}`,
  ];
}

export function connectPreviewSocket(
  pageId: string,
  handlers: PreviewSocketHandlers = {},
): PreviewSocketController {
  let socket: WebSocket | null = null;
  let reconnectTimer: number | null = null;
  let reconnectAttempts = 0;
  let stopped = false;

  const unsubscribe = subscribeBrowserSession((event) => {
    if (event.type === "logout") stop();
  });

  function setStatus(status: PreviewSocketStatus) {
    handlers.onStatus?.(status);
  }

  function stop() {
    if (stopped) return;
    stopped = true;
    unsubscribe();
    if (reconnectTimer !== null) window.clearTimeout(reconnectTimer);
    socket?.close(1000, "preview closed");
    socket = null;
    setStatus("closed");
  }

  function scheduleReconnect() {
    if (stopped) return;
    if (reconnectAttempts >= MAX_RECONNECT_ATTEMPTS) {
      stopped = true;
      unsubscribe();
      setStatus("rejected");
      return;
    }
    const delay = Math.min(4_000, 250 * 2 ** reconnectAttempts);
    reconnectAttempts += 1;
    setStatus("reconnecting");
    reconnectTimer = window.setTimeout(() => {
      reconnectTimer = null;
      void open();
    }, delay);
  }

  async function open() {
    if (stopped) return;
    setStatus(reconnectAttempts === 0 ? "connecting" : "reconnecting");
    try {
      const issued = await api.pages.previewTicket(pageId);
      if (stopped) return;
      const nextSocket = new WebSocket(
        previewWebSocketUrl(api.baseUrl, pageId),
        previewProtocols(issued.ticket),
      );
      socket = nextSocket;
      nextSocket.addEventListener("open", () => {
        if (nextSocket.protocol !== PREVIEW_APPLICATION_PROTOCOL) {
          stopped = true;
          unsubscribe();
          nextSocket.close(1002, "preview protocol rejected");
          setStatus("rejected");
          return;
        }
        reconnectAttempts = 0;
        setStatus("connected");
      });
      nextSocket.addEventListener("message", (event) => {
        try {
          handlers.onMessage?.(JSON.parse(String(event.data)) as unknown);
        } catch {
          nextSocket.close(1003, "invalid preview payload");
        }
      });
      nextSocket.addEventListener("close", (event) => {
        if (socket === nextSocket) socket = null;
        if (stopped) return;
        if ([1002, 1003, 1008].includes(event.code)) {
          stopped = true;
          unsubscribe();
          setStatus("rejected");
          return;
        }
        scheduleReconnect();
      });
      nextSocket.addEventListener("error", () => {
        // The close event owns reconnect behavior and intentionally reveals no ticket details.
      });
    } catch (error) {
      if (
        error instanceof ApiError &&
        [400, 401, 403, 404, 422].includes(error.status)
      ) {
        stopped = true;
        unsubscribe();
        setStatus("rejected");
        return;
      }
      scheduleReconnect();
    }
  }

  void open();
  return { close: stop };
}
