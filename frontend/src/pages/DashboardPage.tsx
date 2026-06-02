import { Activity, Database, FileJson, Server, ShieldCheck } from "lucide-react";

import { useHealth } from "../hooks/useHealth";
import { StatusBadge } from "../components/StatusBadge";

const foundationItems = [
  { label: "API Gateway", value: "Axum 0.8", icon: Server },
  { label: "Database", value: "PostgreSQL 16", icon: Database },
  { label: "Cache", value: "Redis 7", icon: Activity },
  { label: "Schema", value: "JSONB ready", icon: FileJson },
  { label: "Auth Base", value: "JWT scaffold", icon: ShieldCheck },
];

export function DashboardPage() {
  const { health, readiness, loading, error } = useHealth();

  return (
    <div className="page-stack">
      <section className="metric-grid" aria-label="Foundation metrics">
        {foundationItems.map((item) => (
          <article className="metric-card" key={item.label}>
            <item.icon size={20} aria-hidden="true" />
            <span>{item.label}</span>
            <strong>{item.value}</strong>
          </article>
        ))}
      </section>

      <section className="panel">
        <div className="panel-header">
          <div>
            <h2>Runtime</h2>
            <span>Version {health?.version ?? "0.1.0"}</span>
          </div>
          <StatusBadge
            label={loading ? "Checking" : error ? "Unavailable" : readiness?.status ?? "Unknown"}
            tone={error ? "danger" : readiness?.status === "ready" ? "success" : "warning"}
          />
        </div>

        <div className="check-list">
          {(readiness?.checks ?? []).map((check) => (
            <div className="check-row" key={check.name}>
              <span>{check.name}</span>
              <StatusBadge label={check.ok ? "OK" : "Issue"} tone={check.ok ? "success" : "danger"} />
            </div>
          ))}
          {!readiness && (
            <div className="check-row">
              <span>backend</span>
              <StatusBadge label={error ?? "Waiting"} tone={error ? "danger" : "neutral"} />
            </div>
          )}
        </div>
      </section>
    </div>
  );
}
