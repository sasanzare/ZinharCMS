import { useCallback, useEffect, useRef, useState } from "react";
import { KeyRound, LogOut, PlugZap, Plus, RefreshCw, Send, Shield, Trash2, UserRound } from "lucide-react";

import { StatusBadge } from "../components/StatusBadge";
import { StepUpDialog } from "../components/StepUpDialog";
import { useHealth } from "../hooks/useHealth";
import { useI18n } from "../i18n";
import { ApiError, api } from "../services/api";
import { useAppStore } from "../stores/useAppStore";
import type {
  AuthUser,
  MfaEnrollmentResponse,
  MfaStatusResponse,
  SessionSummary,
  StepUpScope,
  WebhookEvent,
  WebhookResponse,
} from "../types/api";

const WEBHOOK_EVENTS: WebhookEvent[] = ["entry.publish", "entry.unpublish", "page.publish", "page.unpublish"];

type WebhookDraft = {
  name: string;
  url: string;
  events: WebhookEvent[];
  secret: string;
  is_active: boolean;
};

type PendingStepUp = {
  scope: StepUpScope;
  title: string;
  run: (stepUpToken: string) => Promise<void>;
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
  if (window.crypto?.getRandomValues) {
    const bytes = window.crypto.getRandomValues(new Uint8Array(32));
    return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
  }
  // An empty value asks the backend to generate the secret with its OS CSPRNG.
  return "";
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
  const [sessions, setSessions] = useState<SessionSummary[]>([]);
  const [sessionError, setSessionError] = useState<string | null>(null);
  const [sessionMessage, setSessionMessage] = useState<string | null>(null);
  const [sessionLoading, setSessionLoading] = useState(false);
  const [revokingSessionId, setRevokingSessionId] = useState<string | null>(null);
  const [logoutAllLoading, setLogoutAllLoading] = useState(false);
  const [mfaStatus, setMfaStatus] = useState<MfaStatusResponse | null>(null);
  const [mfaPassword, setMfaPassword] = useState("");
  const [mfaCode, setMfaCode] = useState("");
  const [mfaEnrollment, setMfaEnrollment] = useState<MfaEnrollmentResponse | null>(null);
  const [recoveryCodes, setRecoveryCodes] = useState<string[]>([]);
  const [recoveryCodesSaved, setRecoveryCodesSaved] = useState(false);
  const [recoveryRequiresRelogin, setRecoveryRequiresRelogin] = useState(false);
  const [mfaError, setMfaError] = useState<string | null>(null);
  const [mfaMessage, setMfaMessage] = useState<string | null>(null);
  const [mfaLoading, setMfaLoading] = useState(false);
  const [pendingStepUp, setPendingStepUp] = useState<PendingStepUp | null>(null);
  const stepUpPendingRef = useRef(false);
  const [webhooks, setWebhooks] = useState<WebhookResponse[]>([]);
  const [webhookDraft, setWebhookDraft] = useState<WebhookDraft>(() => createWebhookDraft());
  const [webhookError, setWebhookError] = useState<string | null>(null);
  const [webhookMessage, setWebhookMessage] = useState<string | null>(null);
  const [webhookLoading, setWebhookLoading] = useState(false);

  const loadMe = useCallback(async function loadMe() {
    setLoading(true);
    setError(null);
    try {
      const response = await api.auth.me();
      setUser(response.user);
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

  const loadSessions = useCallback(async function loadSessions() {
    setSessionLoading(true);
    setSessionError(null);
    try {
      const response = await api.auth.sessions();
      setSessions(response.sessions);
    } catch (caught) {
      setSessionError(apiMessage(caught, "Unable to load active sessions."));
    } finally {
      setSessionLoading(false);
    }
  }, []);

  const loadMfaStatus = useCallback(async function loadMfaStatus() {
    setMfaError(null);
    try {
      setMfaStatus(await api.auth.mfaStatus());
    } catch (caught) {
      setMfaError(apiMessage(caught, "Unable to load MFA status."));
    }
  }, []);

  useEffect(() => {
    void loadMe();
    void loadSessions();
    void loadMfaStatus();
    void loadWebhooks();
  }, [loadMe, loadMfaStatus, loadSessions, loadWebhooks]);

  useEffect(() => {
    if (!pendingStepUp) stepUpPendingRef.current = false;
  }, [pendingStepUp]);

  async function logout() {
    setPendingStepUp(null);
    setMfaEnrollment(null);
    setRecoveryCodes([]);
    setMfaPassword("");
    setMfaCode("");
    try {
      await api.auth.logout();
    } catch {
      // Local logout remains valid when the refresh token is already revoked.
    }
    clearSession();
  }

  function requireStepUp(
    scope: StepUpScope,
    title: string,
    run: (stepUpToken: string) => Promise<void>,
  ) {
    if (stepUpPendingRef.current) return;
    stepUpPendingRef.current = true;
    setPendingStepUp({ scope, title, run });
  }

  async function startMfaEnrollment() {
    if (mfaLoading) return;
    setMfaLoading(true);
    setMfaError(null);
    setMfaMessage(null);
    try {
      const enrollment = await api.auth.startMfaEnrollment(mfaPassword);
      setMfaEnrollment(enrollment);
      setMfaPassword("");
      setMfaCode("");
    } catch (caught) {
      setMfaError(apiMessage(caught, "Unable to start MFA enrollment."));
    } finally {
      setMfaLoading(false);
    }
  }

  async function confirmMfaEnrollment() {
    if (mfaLoading || !mfaEnrollment) return;
    setMfaLoading(true);
    setMfaError(null);
    setMfaMessage(null);
    try {
      const response = await api.auth.confirmMfaEnrollment(mfaCode);
      setRecoveryCodes(response.recovery_codes);
      setRecoveryCodesSaved(false);
      setRecoveryRequiresRelogin(true);
      setMfaEnrollment(null);
      setMfaCode("");
      setMfaStatus((current) =>
        current
          ? {
              ...current,
              enabled: true,
              enrollment_pending: false,
              recovery_codes_remaining: response.recovery_codes.length,
            }
          : current,
      );
      setMfaMessage("MFA enabled. Save every recovery code before signing in again.");
    } catch (caught) {
      setMfaError(apiMessage(caught, "Unable to confirm MFA enrollment."));
    } finally {
      setMfaLoading(false);
    }
  }

  function regenerateRecoveryCodes() {
    requireStepUp(
      "mfa_recovery_regenerate",
      "Generate replacement recovery codes",
      async (stepUpToken) => {
        const response =
          await api.auth.regenerateMfaRecoveryCodes(stepUpToken);
        setRecoveryCodes(response.recovery_codes);
        setRecoveryCodesSaved(false);
        setRecoveryRequiresRelogin(false);
        setMfaMessage("Previous recovery codes are invalid. Save the replacements now.");
        setPendingStepUp(null);
        await loadMfaStatus();
      },
    );
  }

  function disableMfa() {
    if (!window.confirm("Disable MFA and revoke every active session?")) return;
    requireStepUp("mfa_disable", "Disable MFA", async (stepUpToken) => {
      await api.auth.disableMfa(stepUpToken);
      setRecoveryCodes([]);
      setRecoveryRequiresRelogin(false);
      setPendingStepUp(null);
      clearSession();
    });
  }

  function revokeSession(session: SessionSummary) {
    if (stepUpPendingRef.current) return;
    const prompt = session.current
      ? "Revoke this current session and log out this browser?"
      : "Revoke this session?";
    if (!window.confirm(prompt)) return;
    if (revokingSessionId || logoutAllLoading) return;
    requireStepUp("session_logout_all", "Revoke session", async (stepUpToken) => {
      setRevokingSessionId(session.session_id);
      setSessionError(null);
      setSessionMessage(null);
      try {
        const result = await api.auth.revokeSession(session.session_id, stepUpToken);
        setPendingStepUp(null);
        if (result.current_session) {
          clearSession();
          return;
        }
        setSessionMessage(result.revoked ? "Session revoked." : "Session was already unavailable.");
        await loadSessions();
      } catch (caught) {
        setSessionError(apiMessage(caught, "Unable to revoke the session."));
        throw caught;
      } finally {
        setRevokingSessionId(null);
      }
    });
  }

  async function logoutAllSessions() {
    if (stepUpPendingRef.current) return;
    if (!window.confirm("Log out every session, including this browser?")) return;
    if (logoutAllLoading || revokingSessionId) return;
    requireStepUp(
      "session_logout_all",
      "Log out all sessions",
      async (stepUpToken) => {
        setLogoutAllLoading(true);
        setSessionError(null);
        setSessionMessage(null);
        try {
          await api.auth.logoutAll(stepUpToken);
          setPendingStepUp(null);
          clearSession();
        } catch (caught) {
          setSessionError(apiMessage(caught, "Unable to log out all sessions."));
          setLogoutAllLoading(false);
          throw caught;
        }
      },
    );
  }

  function toggleDraftEvent(event: WebhookEvent) {
    setWebhookDraft((current) => ({
      ...current,
      events: current.events.includes(event)
        ? current.events.filter((item) => item !== event)
        : [...current.events, event],
    }));
  }

  function saveWebhook() {
    setWebhookError(null);
    setWebhookMessage(null);
    if (webhookDraft.events.length === 0) {
      setWebhookError(t("settings.selectEvent"));
      return;
    }
    requireStepUp("webhook_administration", "Create webhook", async (stepUpToken) => {
      try {
        const saved = await api.webhooks.create(
          {
            name: webhookDraft.name,
            url: webhookDraft.url,
            events: webhookDraft.events,
            secret: webhookDraft.secret,
            is_active: webhookDraft.is_active,
          },
          stepUpToken,
        );
        setWebhooks((current) => [saved, ...current]);
        setWebhookDraft(createWebhookDraft());
        setWebhookMessage(t("settings.webhookSaved"));
        setPendingStepUp(null);
      } catch (caught) {
        setWebhookError(apiMessage(caught, t("settings.error.saveWebhook")));
        throw caught;
      }
    });
  }

  function toggleWebhook(webhook: WebhookResponse) {
    setWebhookError(null);
    setWebhookMessage(null);
    requireStepUp("webhook_administration", "Update webhook", async (stepUpToken) => {
      try {
        const updated = await api.webhooks.update(
          webhook.id,
          {
            name: webhook.name,
            url: webhook.url,
            events: webhook.events,
            secret: webhook.secret ?? undefined,
            is_active: !webhook.is_active,
          },
          stepUpToken,
        );
        setWebhooks((current) => current.map((item) => (item.id === updated.id ? updated : item)));
        setPendingStepUp(null);
      } catch (caught) {
        setWebhookError(apiMessage(caught, t("settings.error.updateWebhook")));
        throw caught;
      }
    });
  }

  function deleteWebhook(webhook: WebhookResponse) {
    if (!window.confirm(t("settings.confirmDeleteWebhook", { name: webhook.name }))) return;
    setWebhookError(null);
    setWebhookMessage(null);
    requireStepUp("webhook_administration", "Delete webhook", async (stepUpToken) => {
      try {
        const deleted = await api.webhooks.delete(webhook.id, stepUpToken);
        setWebhooks((current) => current.filter((item) => item.id !== deleted.id));
        setWebhookMessage(t("settings.webhookDeleted"));
        setPendingStepUp(null);
      } catch (caught) {
        setWebhookError(apiMessage(caught, t("settings.error.deleteWebhook")));
        throw caught;
      }
    });
  }

  function testWebhook(webhook: WebhookResponse) {
    setWebhookError(null);
    setWebhookMessage(null);
    requireStepUp("webhook_administration", "Send test webhook", async (stepUpToken) => {
      try {
        const result = await api.webhooks.test(webhook.id, stepUpToken);
        setWebhookMessage(t("settings.testSent", { event: result.event }));
        setPendingStepUp(null);
      } catch (caught) {
        setWebhookError(apiMessage(caught, t("settings.error.testWebhook")));
        throw caught;
      }
    });
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
            <input value="HttpOnly cookie" readOnly />
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

      <section className="panel list-panel full-width-panel" aria-label="Multi-factor authentication">
        <div className="panel-header">
          <div>
            <h2>Multi-factor authentication</h2>
            <span>
              TOTP is optional for normal access and required before privileged actions.
            </span>
          </div>
          <StatusBadge
            label={mfaStatus?.enabled ? "Enabled" : "Disabled"}
            tone={mfaStatus?.enabled ? "success" : "warning"}
          />
        </div>

        <div className="padded form-grid">
          {mfaStatus?.required_for_privileged_actions && !mfaStatus.enabled && (
            <StatusBadge
              label="MFA enrollment is required for privileged actions."
              tone="warning"
            />
          )}

          {!mfaStatus?.enabled && !mfaEnrollment && recoveryCodes.length === 0 && (
            <>
              <label>
                Confirm your password
                <input
                  type="password"
                  autoComplete="current-password"
                  value={mfaPassword}
                  onChange={(event) => setMfaPassword(event.target.value)}
                />
              </label>
              <button
                className="primary-button"
                type="button"
                onClick={() => void startMfaEnrollment()}
                disabled={mfaLoading || !mfaPassword}
              >
                <KeyRound size={16} aria-hidden="true" />
                {mfaLoading ? "Starting..." : "Set up authenticator"}
              </button>
            </>
          )}

          {mfaEnrollment && (
            <>
              <p>
                Scan this QR code with your authenticator. If scanning is unavailable,
                enter the manual key.
              </p>
              <img
                src={`data:image/png;base64,${mfaEnrollment.qr_code_base64}`}
                alt="TOTP enrollment QR code"
                width={220}
                height={220}
              />
              <label>
                Manual setup key
                <input value={mfaEnrollment.manual_secret} readOnly />
              </label>
              <label>
                Six-digit confirmation code
                <input
                  autoComplete="one-time-code"
                  inputMode="numeric"
                  value={mfaCode}
                  onChange={(event) => setMfaCode(event.target.value)}
                />
              </label>
              <button
                className="primary-button"
                type="button"
                onClick={() => void confirmMfaEnrollment()}
                disabled={mfaLoading || mfaCode.length !== 6}
              >
                <Shield size={16} aria-hidden="true" />
                {mfaLoading ? "Confirming..." : "Enable MFA"}
              </button>
            </>
          )}

          {recoveryCodes.length > 0 && (
            <div className="status-stack" aria-live="polite">
              <StatusBadge
                label="These recovery codes are shown once. Store them offline."
                tone="warning"
              />
              <div className="settings-grid">
                {recoveryCodes.map((code) => (
                  <code key={code}>{code}</code>
                ))}
              </div>
              <label className="checkbox-row compact-checkbox">
                <input
                  type="checkbox"
                  checked={recoveryCodesSaved}
                  onChange={(event) => setRecoveryCodesSaved(event.target.checked)}
                />
                I saved every recovery code in a secure place.
              </label>
              <button
                className="primary-button"
                type="button"
                disabled={!recoveryCodesSaved}
                onClick={() => {
                  setRecoveryCodes([]);
                  setRecoveryCodesSaved(false);
                  if (recoveryRequiresRelogin) {
                    clearSession();
                  } else {
                    setMfaMessage("Recovery codes saved.");
                  }
                }}
              >
                Continue
              </button>
            </div>
          )}

          {mfaStatus?.enabled && recoveryCodes.length === 0 && (
            <div className="panel-actions">
              <span>
                {mfaStatus.recovery_codes_remaining} unused recovery codes remain.
              </span>
              <button
                className="secondary-button"
                type="button"
                onClick={regenerateRecoveryCodes}
              >
                Replace recovery codes
              </button>
              <button className="secondary-button" type="button" onClick={disableMfa}>
                Disable MFA
              </button>
            </div>
          )}

          {mfaError && <StatusBadge label={mfaError} tone="danger" />}
          {mfaMessage && <StatusBadge label={mfaMessage} tone="success" />}
        </div>
      </section>

      <section
        className="panel list-panel full-width-panel"
        role="region"
        aria-label="Active sessions"
      >
        <div className="panel-header">
          <div>
            <h2>Active sessions</h2>
            <span>Logical browser sessions. No credential or device fingerprint is stored.</span>
          </div>
          <button
            className="icon-button"
            type="button"
            onClick={() => void loadSessions()}
            aria-label="Refresh sessions"
            disabled={sessionLoading || logoutAllLoading}
          >
            <RefreshCw size={16} aria-hidden="true" />
          </button>
        </div>

        <div className="table-scroll">
          <table className="data-table">
            <thead>
              <tr>
                <th>Session</th>
                <th>Created</th>
                <th>Last used</th>
                <th>Expires</th>
                <th>Status</th>
                <th>Actions</th>
              </tr>
            </thead>
            <tbody>
              {sessions.length === 0 ? (
                <tr>
                  <td colSpan={6}>{sessionLoading ? "Loading sessions..." : "No active sessions."}</td>
                </tr>
              ) : (
                sessions.map((session) => (
                  <tr key={session.session_id} data-testid={`session-${session.session_id}`}>
                    <td>{session.current ? "Current session" : "Other session"}</td>
                    <td>{session.created_at}</td>
                    <td>{session.last_used_at}</td>
                    <td>{session.expires_at}</td>
                    <td>
                      <StatusBadge
                        label={
                          session.compromised
                            ? "Compromised"
                            : session.revoked
                              ? "Revoked"
                              : "Active"
                        }
                        tone={
                          session.compromised
                            ? "danger"
                            : session.revoked
                              ? "warning"
                              : "success"
                        }
                      />
                    </td>
                    <td>
                      <button
                        className="secondary-button"
                        type="button"
                        aria-label="Revoke session"
                        onClick={() => void revokeSession(session)}
                        disabled={Boolean(revokingSessionId) || logoutAllLoading}
                      >
                        <Shield size={16} aria-hidden="true" />
                        {revokingSessionId === session.session_id ? "Revoking..." : "Revoke"}
                      </button>
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>

        <div className="panel-actions padded">
          <div className="status-stack" aria-live="polite">
            {sessionError && <StatusBadge label={sessionError} tone="danger" />}
            {sessionMessage && <StatusBadge label={sessionMessage} tone="success" />}
          </div>
          <button
            className="primary-button"
            type="button"
            aria-label="Log out all sessions"
            onClick={() => void logoutAllSessions()}
            disabled={logoutAllLoading || Boolean(revokingSessionId)}
          >
            <LogOut size={16} aria-hidden="true" />
            {logoutAllLoading ? "Logging out..." : "Log out all sessions"}
          </button>
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

      {pendingStepUp && (
        <StepUpDialog
          open
          scope={pendingStepUp.scope}
          title={pendingStepUp.title}
          onGranted={pendingStepUp.run}
          onCancel={() => setPendingStepUp(null)}
        />
      )}
    </div>
  );
}
