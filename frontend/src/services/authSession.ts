import type { AuthResponse } from "../types/api";

const CHANNEL_NAME = "zinhar.auth.session.v1";
const REFRESH_LOCK_NAME = "zinhar.auth.refresh.v1";
const ELECTION_WINDOW_MS = 60;
const REMOTE_REFRESH_TIMEOUT_MS = 5_000;

type SessionEvent =
  | { type: "session"; session: AuthResponse }
  | { type: "logout" };

type CoordinationMessage =
  | { type: "session"; sender: string; session: AuthResponse }
  | { type: "logout"; sender: string }
  | { type: "refresh-intent"; sender: string; sentAt: number }
  | { type: "refresh-failed"; sender: string };

type SessionListener = (event: SessionEvent) => void;

const listeners = new Set<SessionListener>();
const tabId = crypto.randomUUID();
const refreshCandidates = new Map<string, number>();
let refreshPromise: Promise<AuthResponse> | null = null;
let latestSession: AuthResponse | null = null;
let sessionGeneration = 0;
let refreshFailureGeneration = 0;
let channel: BroadcastChannel | null = null;
const remoteRefreshWaiters = new Set<{
  resolve: (session: AuthResponse) => void;
  reject: (error: Error) => void;
}>();

function getChannel() {
  if (!channel && typeof BroadcastChannel !== "undefined") {
    channel = new BroadcastChannel(CHANNEL_NAME);
    channel.addEventListener("message", handleMessage);
  }
  return channel;
}

function handleMessage(event: MessageEvent<CoordinationMessage>) {
  const message = event.data;
  if (!message || message.sender === tabId) return;

  if (message.type === "refresh-intent") {
    refreshCandidates.set(message.sender, message.sentAt);
  } else if (message.type === "session") {
    publishLocalSession(message.session);
    resolveRemoteRefreshWaiters(message.session);
  } else if (message.type === "logout") {
    publishLocalLogout();
    rejectRemoteRefreshWaiters(new Error("browser session ended"));
  } else if (message.type === "refresh-failed") {
    refreshFailureGeneration += 1;
    rejectRemoteRefreshWaiters(new Error("cross-tab refresh failed"));
  }
}

function publishLocalSession(session: AuthResponse) {
  latestSession = session;
  sessionGeneration += 1;
  for (const listener of listeners) listener({ type: "session", session });
}

function publishLocalLogout() {
  latestSession = null;
  sessionGeneration += 1;
  for (const listener of listeners) listener({ type: "logout" });
}

function postMessage(message: CoordinationMessage) {
  getChannel()?.postMessage(message);
}

function resolveRemoteRefreshWaiters(session: AuthResponse) {
  for (const waiter of remoteRefreshWaiters) waiter.resolve(session);
  remoteRefreshWaiters.clear();
}

function rejectRemoteRefreshWaiters(error: Error) {
  for (const waiter of remoteRefreshWaiters) waiter.reject(error);
  remoteRefreshWaiters.clear();
}

function announceRefreshFailure() {
  refreshFailureGeneration += 1;
  postMessage({ type: "refresh-failed", sender: tabId });
}

function delay(milliseconds: number) {
  return new Promise<void>((resolve) => window.setTimeout(resolve, milliseconds));
}

async function refreshUnderWebLock(
  refresh: () => Promise<AuthResponse>,
  observedGeneration: number,
  observedFailureGeneration: number,
) {
  const locks = navigator.locks;
  return locks.request(REFRESH_LOCK_NAME, async () => {
    await delay(0);
    if (sessionGeneration > observedGeneration && latestSession) {
      return latestSession;
    }
    if (refreshFailureGeneration > observedFailureGeneration) {
      throw new Error("cross-tab refresh failed");
    }

    try {
      const session = await refresh();
      acceptBrowserSession(session);
      // Keep the lock briefly so waiting tabs receive the transient session message
      // before they enter the critical section.
      await delay(ELECTION_WINDOW_MS);
      return session;
    } catch (error) {
      announceRefreshFailure();
      await delay(ELECTION_WINDOW_MS);
      throw error;
    }
  });
}

function waitForRemoteSession() {
  return new Promise<AuthResponse>((resolve, reject) => {
    const waiter = {
      resolve: (session: AuthResponse) => {
        window.clearTimeout(timeout);
        remoteRefreshWaiters.delete(waiter);
        resolve(session);
      },
      reject: (error: Error) => {
        window.clearTimeout(timeout);
        remoteRefreshWaiters.delete(waiter);
        reject(error);
      },
    };
    const timeout = window.setTimeout(() => {
      remoteRefreshWaiters.delete(waiter);
      reject(new Error("cross-tab refresh coordination timed out"));
    }, REMOTE_REFRESH_TIMEOUT_MS);
    remoteRefreshWaiters.add(waiter);
  });
}

async function refreshWithBroadcastElection(
  refresh: () => Promise<AuthResponse>,
) {
  const activeChannel = getChannel();
  if (!activeChannel) {
    throw new Error("cross-tab refresh coordination is unavailable");
  }

  const sentAt = Date.now();
  refreshCandidates.set(tabId, sentAt);
  postMessage({ type: "refresh-intent", sender: tabId, sentAt });
  await delay(ELECTION_WINDOW_MS);

  const cutoff = sentAt - ELECTION_WINDOW_MS;
  const candidates = [...refreshCandidates.entries()]
    .filter(([, candidateTime]) => candidateTime >= cutoff)
    .map(([candidate]) => candidate)
    .sort();
  if (candidates[0] !== tabId) {
    return waitForRemoteSession();
  }

  try {
    const session = await refresh();
    acceptBrowserSession(session);
    return session;
  } catch (error) {
    announceRefreshFailure();
    throw error;
  } finally {
    refreshCandidates.delete(tabId);
  }
}

export function acceptBrowserSession(session: AuthResponse, broadcast = true) {
  publishLocalSession(session);
  if (broadcast) {
    postMessage({ type: "session", sender: tabId, session });
  }
}

export function clearBrowserSession(broadcast = true) {
  publishLocalLogout();
  if (broadcast) postMessage({ type: "logout", sender: tabId });
}

export function subscribeBrowserSession(listener: SessionListener) {
  listeners.add(listener);
  getChannel();
  return () => listeners.delete(listener);
}

export function coordinatedRefresh(refresh: () => Promise<AuthResponse>) {
  if (refreshPromise) return refreshPromise;

  const observedGeneration = sessionGeneration;
  const observedFailureGeneration = refreshFailureGeneration;
  refreshPromise = (
    navigator.locks
      ? refreshUnderWebLock(
          refresh,
          observedGeneration,
          observedFailureGeneration,
        )
      : refreshWithBroadcastElection(refresh)
  ).finally(() => {
    refreshPromise = null;
  });
  return refreshPromise;
}
