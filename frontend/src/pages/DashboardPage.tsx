import { useEffect, useState } from "react";
import { Activity, Database, FileJson, Image, Layers3, Server, ShieldCheck, Workflow } from "lucide-react";

import { StatusBadge } from "../components/StatusBadge";
import { useHealth } from "../hooks/useHealth";
import { ApiError, api } from "../services/api";

type DashboardStats = {
  contentTypes: number;
  entries: number;
  media: number;
  pages: number;
};

const foundationItems = [
  { label: "API Gateway", value: "Axum 0.8", icon: Server },
  { label: "Database", value: "PostgreSQL 16", icon: Database },
  { label: "Cache", value: "Redis 7", icon: Activity },
  { label: "Schema", value: "JSONB ready", icon: FileJson },
  { label: "Auth Base", value: "JWT scaffold", icon: ShieldCheck },
];

export function DashboardPage() {
  const { health, readiness, loading, error } = useHealth();
  const [stats, setStats] = useState<DashboardStats>({ contentTypes: 0, entries: 0, media: 0, pages: 0 });
  const [statsError, setStatsError] = useState<string | null>(null);

  useEffect(() => {
    let active = true;

    async function loadStats() {
      try {
        const [contentTypes, media, pages] = await Promise.all([
          api.contentTypes.list(),
          api.media.list({ per_page: 1 }),
          api.pages.list({ per_page: 100 }),
        ]);
        const firstType = contentTypes[0];
        const entries = firstType ? await api.entries.list(firstType.slug, { per_page: 100 }) : { data: [] };
        if (!active) return;
        setStats({
          contentTypes: contentTypes.length,
          entries: entries.data.length,
          media: media.data.length,
          pages: pages.data.filter((page) => page.status === "published").length,
        });
        setStatsError(null);
      } catch (caught) {
        if (!active) return;
        setStatsError(caught instanceof ApiError ? caught.message : "Dashboard stats unavailable");
      }
    }

    void loadStats();
    return () => {
      active = false;
    };
  }, []);

  const productMetrics = [
    { label: "Content Types", value: String(stats.contentTypes), icon: Layers3 },
    { label: "Entries", value: String(stats.entries), icon: FileJson },
    { label: "Media Assets", value: String(stats.media), icon: Image },
    { label: "Published Pages", value: String(stats.pages), icon: Workflow },
  ];

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

      <section className="metric-grid" aria-label="CMS metrics">
        {productMetrics.map((item) => (
          <article className="metric-card metric-card--strong" key={item.label}>
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
          {statsError && (
            <div className="check-row">
              <span>admin data</span>
              <StatusBadge label={statsError} tone="warning" />
            </div>
          )}
        </div>
      </section>
    </div>
  );
}