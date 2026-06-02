import { Save } from "lucide-react";

export function SettingsPage() {
  return (
    <div className="page-stack">
      <section className="panel settings-panel">
        <div className="panel-header">
          <div>
            <h2>Environment</h2>
            <span>Local development defaults</span>
          </div>
          <button className="primary-button" type="button">
            <Save size={16} aria-hidden="true" />
            Save
          </button>
        </div>

        <div className="settings-grid">
          <label>
            API URL
            <input value={import.meta.env.VITE_API_URL ?? "http://localhost:8080"} readOnly />
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
            Refresh token TTL
            <input value="604800 seconds" readOnly />
          </label>
        </div>
      </section>
    </div>
  );
}
