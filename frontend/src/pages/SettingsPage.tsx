import { useEffect, useState } from "react";
import { PlugZap, Plus, RefreshCw, Send, Shield, Trash2, UserRound } from "lucide-react";

import { StatusBadge } from "../components/StatusBadge";
import { useHealth } from "../hooks/useHealth";
import { ApiError, api, getStoredRefreshToken } from "../services/api";
import { useAppStore } from "../stores/useAppStore";
import type { AuthUser, WebhookEvent, WebhookResponse } from "../types/api";

const WEBHOOK_EVENTS: WebhookEvent[] = ["entry.publish", "entry.unpublish", "page.publish", "page.unpublish"];

type WebhookDraft = {
  name: string;
  url: string;
  events: WebhookEvent[];
  secret: string;
  is_active: boolean;
};

function createWebhookDraft(): WebhookDraft {
  return {
    name: "",
    url: "https://example.com/webhook",
    events: ["entry.publish", "page.publish"],
    secret: randomSecret(),
    is_active: true,
  };
}

function randomSecret() {
  if (window.crypto?.randomUUID) {
    return `${window.crypto.randomUUID()}${window.crypto.randomUUID()}`.replaceAll("-", "");
  }
  return Math.random().toString(36).slice(2).padEnd(32, "0");
}

function apiMessage(caught: unknown, fallback: string) {
  return caught instanceof ApiError ? caught.message : fallback;
}

export function SettingsPage() {
  const storedUser = useAppStore((state) => state.user);
  const clearSession = useAppStore((state) => state.clearSession);
  const { readiness } = useHealth();
  const [user, setUser] = useState<AuthUser | null>(storedUser);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [webhooks, setWebhooks] = useState<WebhookResponse[]>([]);
  const [webhookDraft, setWebhookDraft] = useState<WebhookDraft>(() => createWebhookDraft());
  const [webhookError, setWebhookError] = useState<string | null>(null);
  const [webhookMessage, setWebhookMessage] = useState<string | null>(null);
  const [webhookLoading, setWebhookLoading] = useState(false);

  async function loadMe() {
    setLoading(true);
    setError(null);
    try {
      setUser(await api.auth.me());
    } catch (caught) {
      setError(apiMessage(caught, "Failed to load current user"));
    } finally {
      setLoading(false);
    }
  }

  async function loadWebhooks() {
    setWebhookLoading(true);
    setWebhookError(null);
    try {
      setWebhooks(await api.webhooks.list());
    } catch (caught) {
      setWebhookError(apiMessage(caught, "Failed to load webhooks"));
    } finally {
      setWebhookLoading(false);
    }
  }

  useEffect(() => {
    void loadMe();
    void loadWebhooks();
  }, []);

  async function logout() {
    const refreshToken = getStoredRefreshToken();
    if (refreshToken) {
      try {
        await api.auth.logout(refreshToken);
      } catch {
        // Local logout remains valid when the refresh token is already revoked.
      }
    }
    clearSession();
  }

  function toggleDraftEvent(event: WebhookEvent) {
    setWebhookDraft((current) => ({
      ...current,
      events: current.events.includes(event)
        ? current.events.filter((item) => item !== event)
        : [...current.events, event],
    }));
  }

  async function saveWebhook() {
    setWebhookError(null);
    setWebhookMessage(null);
    if (webhookDraft.events.length === 0) {
      setWebhookError("Select at least one event");
      return;
    }
    try {
      const saved = await api.webhooks.create({
        name: webhookDraft.name,
        url: webhookDraft.url,
        events: webhookDraft.events,
        secret: webhookDraft.secret,
        is_active: webhookDraft.is_active,
      });
      setWebhooks((current) => [saved, ...current]);
      setWebhookDraft(createWebhookDraft());
      setWebhookMessage("Webhook saved");
    } catch (caught) {
      setWebhookError(apiMessage(caught, "Failed to save webhook"));
    }
  }

  async function toggleWebhook(webhook: WebhookResponse) {
    setWebhookError(null);
    setWebhookMessage(null);
    try {
      const updated = await api.webhooks.update(webhook.id, {
        name: webhook.name,
        url: webhook.url,
        events: webhook.events,
        secret: webhook.secret,
        is_active: !webhook.is_active,
      });
      setWebhooks((current) => current.map((item) => (item.id === updated.id ? updated : item)));
    } catch (caught) {
      setWebhookError(apiMessage(caught, "Failed to update webhook"));
    }
  }

  async function deleteWebhook(webhook: WebhookResponse) {
    if (!window.confirm(`Delete webhook ${webhook.name}?`)) return;
    setWebhookError(null);
    setWebhookMessage(null);
    try {
      const deleted = await api.webhooks.delete(webhook.id);
      setWebhooks((current) => current.filter((item) => item.id !== deleted.id));
      setWebhookMessage("Webhook deleted");
    } catch (caught) {
      setWebhookError(apiMessage(caught, "Failed to delete webhook"));
    }
  }

  async function testWebhook(webhook: WebhookResponse) {
    setWebhookError(null);
    setWebhookMessage(null);
    try {
      const result = await api.webhooks.test(webhook.id);
      setWebhookMessage(`Test sent: ${result.event}`);
    } catch (caught) {
      setWebhookError(apiMessage(caught, "Failed to test webhook"));
    }
  }

  return (
    <div className="page-stack two-column-workspace">
      <section className="panel editor-panel">
        <div className="panel-header">
          <div>
            <h2>Current user</h2>
            <span>Token-backed profile from `/api/auth/me`</span>
          </div>
          <button className="icon-button" type="button" onClick={() => void loadMe()} aria-label="Refresh user">
            <RefreshCw size={16} aria-hidden="true" />
          </button>
        </div>

        <div className="settings-grid">
          <label>
            Name
            <input value={user?.name ?? ""} readOnly />
          </label>
          <label>
            Email
            <input value={user?.email ?? ""} readOnly />
          </label>
          <label>
            Role
            <input value={user?.role ?? ""} readOnly />
          </label>
          <label>
            Refresh token
            <input value={getStoredRefreshToken() ? "Stored" : "Missing"} readOnly />
          </label>
        </div>

        <div className="panel-actions padded">
          {error && <StatusBadge label={error} tone="danger" />}
          <button className="secondary-button" type="button" onClick={() => void loadMe()} disabled={loading}>
            <UserRound size={16} aria-hidden="true" />
            {loading ? "Refreshing..." : "Refresh profile"}
          </button>
          <button className="primary-button" type="button" onClick={() => void logout()}>
            <Shield size={16} aria-hidden="true" />
            Logout
          </button>
        </div>
      </section>

      <section className="panel list-panel">
        <div className="panel-header">
          <div>
            <h2>Environment</h2>
            <span>Local development defaults</span>
          </div>
          <StatusBadge label={readiness?.status ?? "unknown"} tone={readiness?.status === "ready" ? "success" : "warning"} />
        </div>

        <div className="settings-grid">
          <label>
            API URL
            <input value={api.baseUrl} readOnly />
          </label>
          <label>
            Upload limit
            <input value="50 MB" readOnly />
          </label>
          <label>
            Access token TTL
            <input value="3600 seconds" readOnly />
          </label>
          <label>
            Delivery API
            <input value="/api/v1" readOnly />
          </label>
        </div>
      </section>

      <section className="panel list-panel full-width-panel">
        <div className="panel-header">
          <div>
            <h2>Webhooks</h2>
            <span>Signed delivery events</span>
          </div>
          <button className="icon-button" type="button" onClick={() => void loadWebhooks()} aria-label="Refresh webhooks">
            <RefreshCw size={16} aria-hidden="true" />
          </button>
        </div>

        <div className="webhook-workspace">
          <div className="webhook-form">
            <div className="form-grid">
              <label>
                Name
                <input
                  value={webhookDraft.name}
                  onChange={(event) => setWebhookDraft((current) => ({ ...current, name: event.target.value }))}
                />
              </label>
              <label>
                URL
                <input
                  value={webhookDraft.url}
                  onChange={(event) => setWebhookDraft((current) => ({ ...current, url: event.target.value }))}
                />
              </label>
              <label>
                Secret
                <input
                  value={webhookDraft.secret}
                  onChange={(event) => setWebhookDraft((current) => ({ ...current, secret: event.target.value }))}
                />
              </label>
              <label className="checkbox-row compact-checkbox">
                <input
                  type="checkbox"
                  checked={webhookDraft.is_active}
                  onChange={(event) => setWebhookDraft((current) => ({ ...current, is_active: event.target.checked }))}
                />
                Active
              </label>
            </div>

            <div className="webhook-events">
              {WEBHOOK_EVENTS.map((event) => (
                <label className="checkbox-row compact-checkbox" key={event}>
                  <input
                    type="checkbox"
                    checked={webhookDraft.events.includes(event)}
                    onChange={() => toggleDraftEvent(event)}
                  />
                  {event}
                </label>
              ))}
            </div>

            <div className="panel-actions padded webhook-actions">
              <div className="status-stack">
                {webhookError && <StatusBadge label={webhookError} tone="danger" />}
                {webhookMessage && <StatusBadge label={webhookMessage} tone="success" />}
              </div>
              <button className="primary-button" type="button" onClick={() => void saveWebhook()} disabled={webhookLoading}>
                <Plus size={16} aria-hidden="true" />
                Save webhook
              </button>
            </div>
          </div>

          <div className="table-scroll webhook-table-wrap">
            <table className="data-table">
              <thead>
                <tr>
                  <th>Name</th>
                  <th>URL</th>
                  <th>Events</th>
                  <th>Status</th>
                  <th>Actions</th>
                </tr>
              </thead>
              <tbody>
                {webhooks.length === 0 ? (
                  <tr>
                    <td colSpan={5}>No webhooks configured.</td>
                  </tr>
                ) : (
                  webhooks.map((webhook) => (
                    <tr key={webhook.id}>
                      <td>{webhook.name}</td>
                      <td className="truncate-cell">{webhook.url}</td>
                      <td>{webhook.events.join(", ")}</td>
                      <td>
                        <StatusBadge label={webhook.is_active ? "Active" : "Paused"} tone={webhook.is_active ? "success" : "neutral"} />
                      </td>
                      <td>
                        <div className="table-actions">
                          <button className="secondary-button" type="button" onClick={() => void toggleWebhook(webhook)}>
                            <PlugZap size={16} aria-hidden="true" />
                            {webhook.is_active ? "Pause" : "Activate"}
                          </button>
                          <button className="secondary-button" type="button" onClick={() => void testWebhook(webhook)}>
                            <Send size={16} aria-hidden="true" />
                            Test
                          </button>
                          <button className="icon-button" type="button" onClick={() => void deleteWebhook(webhook)} aria-label={`Delete ${webhook.name}`}>
                            <Trash2 size={16} aria-hidden="true" />
                          </button>
                        </div>
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </div>
      </section>
    </div>
  );
}