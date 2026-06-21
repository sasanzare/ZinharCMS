import { useEffect, useState } from "react";
import { Activity, Database, FileJson, Image, Layers3, Server, ShieldCheck, Workflow } from "lucide-react";
import type { LucideIcon } from "lucide-react";

import { StatusBadge } from "../components/StatusBadge";
import { useHealth } from "../hooks/useHealth";
import { useI18n, type MessageKey } from "../i18n";
import { ApiError, api } from "../services/api";

type DashboardStats = {
  contentTypes: number;
  entries: number;
  media: number;
  pages: number;
};

type DashboardMetric = {
  labelKey: MessageKey;
  value: string;
  icon: LucideIcon;
};

const foundationItems: DashboardMetric[] = [
  { labelKey: "dashboard.foundation.api", value: "Axum 0.8", icon: Server },
  { labelKey: "dashboard.foundation.database", value: "PostgreSQL 16", icon: Database },
  { labelKey: "dashboard.foundation.cache", value: "Redis 7", icon: Activity },
  { labelKey: "dashboard.foundation.schema", value: "dashboard.foundation.schemaValue", icon: FileJson },
  { labelKey: "dashboard.foundation.auth", value: "dashboard.foundation.authValue", icon: ShieldCheck },
];

export function DashboardPage() {
  const { t } = useI18n();
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
        setStatsError(caught instanceof ApiError ? caught.message : t("dashboard.error.statsUnavailable"));
      }
    }

    void loadStats();
    return () => {
      active = false;
    };
  }, [t]);

  const productMetrics: DashboardMetric[] = [
    { labelKey: "dashboard.metric.contentTypes", value: String(stats.contentTypes), icon: Layers3 },
    { labelKey: "dashboard.metric.entries", value: String(stats.entries), icon: FileJson },
    { labelKey: "dashboard.metric.mediaAssets", value: String(stats.media), icon: Image },
    { labelKey: "dashboard.metric.publishedPages", value: String(stats.pages), icon: Workflow },
  ];

  return (
    <div className="page-stack">
      <section className="metric-grid" aria-label={t("dashboard.foundation.aria")}>
        {foundationItems.map((item) => (
          <article className="metric-card" key={item.labelKey}>
            <item.icon size={20} aria-hidden="true" />
            <span>{t(item.labelKey)}</span>
            <strong>{item.value.startsWith("dashboard.") ? t(item.value as MessageKey) : item.value}</strong>
          </article>
        ))}
      </section>

      <section className="metric-grid" aria-label={t("dashboard.cms.aria")}>
        {productMetrics.map((item) => (
          <article className="metric-card metric-card--strong" key={item.labelKey}>
            <item.icon size={20} aria-hidden="true" />
            <span>{t(item.labelKey)}</span>
            <strong>{item.value}</strong>
          </article>
        ))}
      </section>

      <section className="panel">
        <div className="panel-header">
          <div>
            <h2>{t("dashboard.runtime")}</h2>
            <span>{t("dashboard.version", { version: health?.version ?? "0.1.0" })}</span>
          </div>
          <StatusBadge
            label={loading ? t("app.status.checking") : error ? t("app.status.unavailable") : readiness?.status ?? t("app.status.unknown")}
            tone={error ? "danger" : readiness?.status === "ready" ? "success" : "warning"}
          />
        </div>

        <div className="check-list">
          {(readiness?.checks ?? []).map((check) => (
            <div className="check-row" key={check.name}>
              <span>{check.name}</span>
              <StatusBadge label={check.ok ? t("app.status.ok") : t("app.status.issue")} tone={check.ok ? "success" : "danger"} />
            </div>
          ))}
          {!readiness && (
            <div className="check-row">
              <span>{t("dashboard.backend")}</span>
              <StatusBadge label={error ?? t("app.status.waiting")} tone={error ? "danger" : "neutral"} />
            </div>
          )}
          {statsError && (
            <div className="check-row">
              <span>{t("dashboard.adminData")}</span>
              <StatusBadge label={statsError} tone="warning" />
            </div>
          )}
        </div>
      </section>
    </div>
  );
}