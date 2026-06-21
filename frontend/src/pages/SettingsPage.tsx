import { useCallback, useEffect, useState } from "react";
import { PlugZap, Plus, RefreshCw, Send, Shield, Trash2, UserRound } from "lucide-react";

import { StatusBadge } from "../components/StatusBadge";
import { useHealth } from "../hooks/useHealth";
import { useI18n } from "../i18n";
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
  const { t } = useI18n();
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

  const loadMe = useCallback(async function loadMe() {
    setLoading(true);
    setError(null);
    try {
      setUser(await api.auth.me());
    } catch (caught) {
      setError(apiMessage(caught, t("settings.error.loadUser")));
    } finally {
      setLoading(false);
    }
  }, [t]);

  const loadWebhooks = useCallback(async function loadWebhooks() {
    setWebhookLoading(true);
    setWebhookError(null);
    try {
      setWebhooks(await api.webhooks.list());
    } catch (caught) {
      setWebhookError(apiMessage(caught, t("settings.error.loadWebhooks")));
    } finally {
      setWebhookLoading(false);
    }
  }, [t]);

  useEffect(() => {
    void loadMe();
    void loadWebhooks();
  }, [loadMe, loadWebhooks]);

  async function logout() {
    const refreshToken = getStoredRefreshToken();
    try {
      await api.auth.logout(refreshToken);
    } catch {
      // Local logout remains valid when the refresh token is already revoked.
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
      setWebhookError(t("settings.selectEvent"));
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
      setWebhookMessage(t("settings.webhookSaved"));
    } catch (caught) {
      setWebhookError(apiMessage(caught, t("settings.error.saveWebhook")));
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
      setWebhookError(apiMessage(caught, t("settings.error.updateWebhook")));
    }
  }

  async function deleteWebhook(webhook: WebhookResponse) {
    if (!window.confirm(t("settings.confirmDeleteWebhook", { name: webhook.name }))) return;
    setWebhookError(null);
    setWebhookMessage(null);
    try {
      const deleted = await api.webhooks.delete(webhook.id);
      setWebhooks((current) => current.filter((item) => item.id !== deleted.id));
      setWebhookMessage(t("settings.webhookDeleted"));
    } catch (caught) {
      setWebhookError(apiMessage(caught, t("settings.error.deleteWebhook")));
    }
  }

  async function testWebhook(webhook: WebhookResponse) {
    setWebhookError(null);
    setWebhookMessage(null);
    try {
      const result = await api.webhooks.test(webhook.id);
      setWebhookMessage(t("settings.testSent", { event: result.event }));
    } catch (caught) {
      setWebhookError(apiMessage(caught, t("settings.error.testWebhook")));
    }
  }

  return (
    <div className="page-stack two-column-workspace">
      <section className="panel editor-panel">
        <div className="panel-header">
          <div>
            <h2>{t("settings.currentUser")}</h2>
            <span>{t("settings.currentUserDescription")}</span>
          </div>
          <button className="icon-button" type="button" onClick={() => void loadMe()} aria-label={t("settings.refreshUser")}>
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
            <input value={getStoredRefreshToken() ? "Legacy localStorage" : "HttpOnly cookie"} readOnly />
          </label>
        </div>

        <div className="panel-actions padded">
          {error && <StatusBadge label={error} tone="danger" />}
          <button className="secondary-button" type="button" onClick={() => void loadMe()} disabled={loading}>
            <UserRound size={16} aria-hidden="true" />
            {loading ? t("settings.refreshing") : t("settings.refreshProfile")}
          </button>
          <button className="primary-button" type="button" onClick={() => void logout()}>
            <Shield size={16} aria-hidden="true" />
            {t("app.action.logout")}
          </button>
        </div>
      </section>

      <section className="panel list-panel">
        <div className="panel-header">
          <div>
            <h2>{t("settings.environment")}</h2>
            <span>{t("settings.environmentDescription")}</span>
          </div>
          <StatusBadge label={readiness?.status ?? "unknown"} tone={readiness?.status === "ready" ? "success" : "warning"} />
        </div>

        <div className="settings-grid">
          <label>
            {t("settings.apiUrl")}
            <input value={api.baseUrl} readOnly />
          </label>
          <label>
            {t("settings.uploadLimit")}
            <input value="50 MB" readOnly />
          </label>
          <label>
            {t("settings.accessTokenTtl")}
            <input value="3600 seconds" readOnly />
          </label>
          <label>
            {t("settings.deliveryApi")}
            <input value="/api/v1" readOnly />
          </label>
        </div>
      </section>

      <section className="panel list-panel full-width-panel">
        <div className="panel-header">
          <div>
            <h2>{t("settings.webhooks")}</h2>
            <span>{t("settings.webhooksDescription")}</span>
          </div>
          <button className="icon-button" type="button" onClick={() => void loadWebhooks()} aria-label={t("settings.refreshWebhooks")}>
            <RefreshCw size={16} aria-hidden="true" />
          </button>
        </div>

        <div className="webhook-workspace">
          <div className="webhook-form">
            <div className="form-grid">
              <label>
                {t("common.name")}
                <input
                  value={webhookDraft.name}
                  onChange={(event) => setWebhookDraft((current) => ({ ...current, name: event.target.value }))}
                />
              </label>
              <label>
                {t("common.url")}
                <input
                  value={webhookDraft.url}
                  onChange={(event) => setWebhookDraft((current) => ({ ...current, url: event.target.value }))}
                />
              </label>
              <label>
                {t("settings.secret")}
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
                {t("common.active")}
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
                {t("settings.saveWebhook")}
              </button>
            </div>
          </div>

          <div className="table-scroll webhook-table-wrap">
            <table className="data-table">
              <thead>
                <tr>
                  <th>{t("common.name")}</th>
                  <th>{t("common.url")}</th>
                  <th>{t("settings.events")}</th>
                  <th>{t("common.status")}</th>
                  <th>{t("common.actions")}</th>
                </tr>
              </thead>
              <tbody>
                {webhooks.length === 0 ? (
                  <tr>
                    <td colSpan={5}>{t("settings.noWebhooks")}</td>
                  </tr>
                ) : (
                  webhooks.map((webhook) => (
                    <tr key={webhook.id}>
                      <td>{webhook.name}</td>
                      <td className="truncate-cell">{webhook.url}</td>
                      <td>{webhook.events.join(", ")}</td>
                      <td>
                        <StatusBadge label={webhook.is_active ? t("common.active") : t("common.paused")} tone={webhook.is_active ? "success" : "neutral"} />
                      </td>
                      <td>
                        <div className="table-actions">
                          <button className="secondary-button" type="button" onClick={() => void toggleWebhook(webhook)}>
                            <PlugZap size={16} aria-hidden="true" />
                            {webhook.is_active ? t("common.pause") : t("settings.activate")}
                          </button>
                          <button className="secondary-button" type="button" onClick={() => void testWebhook(webhook)}>
                            <Send size={16} aria-hidden="true" />
                            {t("common.test")}
                          </button>
                          <button className="icon-button" type="button" onClick={() => void deleteWebhook(webhook)} aria-label={t("settings.deleteWebhook", { name: webhook.name })}>
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