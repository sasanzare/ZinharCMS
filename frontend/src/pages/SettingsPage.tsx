import { useEffect, useState } from "react";
import { RefreshCw, Shield, UserRound } from "lucide-react";

import { StatusBadge } from "../components/StatusBadge";
import { useHealth } from "../hooks/useHealth";
import { ApiError, api, getStoredRefreshToken } from "../services/api";
import { useAppStore } from "../stores/useAppStore";
import type { AuthUser } from "../types/api";

export function SettingsPage() {
  const storedUser = useAppStore((state) => state.user);
  const clearSession = useAppStore((state) => state.clearSession);
  const { readiness } = useHealth();
  const [user, setUser] = useState<AuthUser | null>(storedUser);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  async function loadMe() {
    setLoading(true);
    setError(null);
    try {
      setUser(await api.auth.me());
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : "Failed to load current user");
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    void loadMe();
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
            Users API
            <input value="Planned with workflow/collaboration phase" readOnly />
          </label>
        </div>
      </section>
    </div>
  );
}