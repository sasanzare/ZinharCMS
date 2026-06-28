import { useEffect, useState } from "react";
import { Activity, CreditCard, Database, FileJson, Image, Layers3, Server, ShieldCheck, Workflow } from "lucide-react";
import type { LucideIcon } from "lucide-react";

import { StatusBadge } from "../components/StatusBadge";
import { useHealth } from "../hooks/useHealth";
import { useI18n, type MessageKey } from "../i18n";
import { ApiError, api } from "../services/api";
import type { BillingUsageResponse, UsageMetricResponse } from "../types/api";

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

type UsageItem = {
  key: keyof Pick<BillingUsageResponse, "members" | "content_records" | "media_bytes" | "api_requests">;
  labelKey: "billing.usage.members" | "billing.usage.content" | "billing.usage.media" | "billing.usage.api";
};

const foundationItems: DashboardMetric[] = [
  { labelKey: "dashboard.foundation.api", value: "Axum 0.8", icon: Server },
  { labelKey: "dashboard.foundation.database", value: "PostgreSQL 16", icon: Database },
  { labelKey: "dashboard.foundation.cache", value: "Redis 7", icon: Activity },
  { labelKey: "dashboard.foundation.schema", value: "dashboard.foundation.schemaValue", icon: FileJson },
  { labelKey: "dashboard.foundation.auth", value: "dashboard.foundation.authValue", icon: ShieldCheck },
];

const usageItems: UsageItem[] = [
  { key: "members", labelKey: "billing.usage.members" },
  { key: "content_records", labelKey: "billing.usage.content" },
  { key: "media_bytes", labelKey: "billing.usage.media" },
  { key: "api_requests", labelKey: "billing.usage.api" },
];

function formatBytes(bytes: number) {
  const gib = 1024 * 1024 * 1024;
  const mib = 1024 * 1024;
  if (bytes >= gib) return `${(bytes / gib).toFixed(1)} GB`;
  return `${(bytes / mib).toFixed(0)} MB`;
}

function formatUsageValue(metric: string, value: number) {
  return metric === "media_bytes" ? formatBytes(value) : value.toLocaleString();
}

function formatLimit(metric: UsageMetricResponse, unlimitedLabel: string) {
  if (metric.limit < 0) return unlimitedLabel;
  return formatUsageValue(metric.metric, metric.limit);
}

export function DashboardPage() {
  const { t } = useI18n();
  const { health, readiness, loading, error } = useHealth();
const [stats, setStats] = useState<DashboardStats>({ contentTypes: 0, entries: 0, media: 0, pages: 0 });
  const [usage, setUsage] = useState<BillingUsageResponse | null>(null);
  const [statsError, setStatsError] = useState<string | null>(null);
  const [usageError, setUsageError] = useState<string | null>(null);

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

    async function loadUsage() {
      try {
        const billingUsage = await api.billing.usage();
        if (!active) return;
        setUsage(billingUsage);
        setUsageError(null);
      } catch (caught) {
        if (!active) return;
        setUsageError(caught instanceof ApiError ? caught.message : t("billing.error.load"));
      }
    }

    void loadStats();
    void loadUsage();
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
            <h2>{t("billing.usage.title")}</h2>
            <span>{t("billing.usage.description")}</span>
          </div>
          <CreditCard size={18} aria-hidden="true" />
        </div>
        <div className="usage-grid padded">
          {usage ? (
            usageItems.map((item) => {
              const metric = usage[item.key];
              const percent = metric.percent ?? 0;
              return (
                <div className="usage-meter" key={item.key}>
                  <div className="usage-meter__header">
                    <strong>{t(item.labelKey)}</strong>
                    <StatusBadge
                      label={metric.exceeded ? t("billing.usage.exceeded") : metric.near_limit ? t("billing.usage.nearLimit") : t("app.status.ok")}
                      tone={metric.exceeded ? "danger" : metric.near_limit ? "warning" : "success"}
                    />
                  </div>
                  <div className="usage-meter__values">
                    <span>{formatUsageValue(metric.metric, metric.used)}</span>
                    <span>{formatLimit(metric, t("billing.unlimited"))}</span>
                  </div>
                  <div className="usage-meter__track">
                    <span style={{ width: `${Math.min(percent, 100)}%` }} />
                  </div>
                </div>
              );
            })
          ) : (
            <div className="empty-state">
              <strong>{t("billing.usage.empty")}</strong>
              <span>{usageError ?? t("billing.usage.emptyDescription")}</span>
            </div>
          )}
        </div>
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