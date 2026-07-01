import { useCallback, useEffect, useState } from "react";
import { CreditCard, ExternalLink, Gauge, RefreshCw, RotateCcw, ShieldCheck } from "lucide-react";

import { StatusBadge } from "../components/StatusBadge";
import { useI18n } from "../i18n";
import { ApiError, api } from "../services/api";
import { useAppStore } from "../stores/useAppStore";
import type { BillingUsageResponse, PlanResponse, UsageMetricResponse } from "../types/api";

const BILLING_MANAGER_ROLES = new Set(["owner", "admin", "billing_manager"]);

type UsageItem = {
  key: keyof Pick<BillingUsageResponse, "members" | "content_records" | "media_bytes" | "api_requests">;
  labelKey: "billing.usage.members" | "billing.usage.content" | "billing.usage.media" | "billing.usage.api";
};

const USAGE_ITEMS: UsageItem[] = [
  { key: "members", labelKey: "billing.usage.members" },
  { key: "content_records", labelKey: "billing.usage.content" },
  { key: "media_bytes", labelKey: "billing.usage.media" },
  { key: "api_requests", labelKey: "billing.usage.api" },
];

function apiMessage(caught: unknown, fallback: string) {
  return caught instanceof ApiError ? caught.message : fallback;
}

function formatMoney(cents: number) {
  if (cents === 0) return "$0";
  return `$${(cents / 100).toFixed(0)}`;
}

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

function isCustomPriced(plan: PlanResponse) {
  return plan.slug === "enterprise" && plan.price_monthly_cents === 0;
}

export function BillingPage() {
  const { t } = useI18n();
  const organizations = useAppStore((state) => state.organizations);
  const activeOrganizationId = useAppStore((state) => state.activeOrganizationId);
  const activeMembership = organizations.find((organization) => organization.id === activeOrganizationId);
  const canManageBilling = activeMembership ? BILLING_MANAGER_ROLES.has(activeMembership.role) : false;

  const [plans, setPlans] = useState<PlanResponse[]>([]);
  const [usage, setUsage] = useState<BillingUsageResponse | null>(null);
  const [loading, setLoading] = useState(false);
  const [actionLoading, setActionLoading] = useState(false);
  const [message, setMessage] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const loadBilling = useCallback(async function loadBilling() {
    setLoading(true);
    setError(null);
    try {
      const [nextPlans, nextUsage] = await Promise.all([api.billing.plans(), api.billing.usage()]);
      setPlans(nextPlans);
      setUsage(nextUsage);
    } catch (caught) {
      setError(apiMessage(caught, t("billing.error.load")));
    } finally {
      setLoading(false);
    }
  }, [t]);

  useEffect(() => {
    void loadBilling();
  }, [activeOrganizationId, loadBilling]);

  async function changePlan(plan: PlanResponse) {
    setActionLoading(true);
    setError(null);
    setMessage(null);
    try {
      await api.billing.changePlan({ plan_slug: plan.slug });
      await loadBilling();
      setMessage(t("billing.message.planChanged", { plan: plan.name }));
    } catch (caught) {
      setError(apiMessage(caught, t("billing.error.changePlan")));
    } finally {
      setActionLoading(false);
    }
  }

  async function startCheckout(plan: PlanResponse) {
    setActionLoading(true);
    setError(null);
    setMessage(null);
    try {
      const session = await api.billing.checkout({ plan_slug: plan.slug });
      window.location.assign(session.url);
    } catch (caught) {
      setError(apiMessage(caught, t("billing.error.checkout")));
      setActionLoading(false);
    }
  }

  async function openCustomerPortal() {
    setActionLoading(true);
    setError(null);
    setMessage(null);
    try {
      const session = await api.billing.portal();
      window.location.assign(session.url);
    } catch (caught) {
      setError(apiMessage(caught, t("billing.error.portal")));
      setActionLoading(false);
    }
  }

  async function rebuildUsage() {
    setActionLoading(true);
    setError(null);
    setMessage(null);
    try {
      setUsage(await api.billing.rebuildUsage());
      setMessage(t("billing.message.rebuilt"));
    } catch (caught) {
      setError(apiMessage(caught, t("billing.error.rebuild")));
    } finally {
      setActionLoading(false);
    }
  }

  return (
    <div className="page-stack billing-page">
      <div className="panel-actions">
        <div className="status-stack">
          {loading && <StatusBadge label={t("common.loading")} tone="neutral" />}
          {error && <StatusBadge label={error} tone="danger" />}
          {message && <StatusBadge label={message} tone="success" />}
        </div>
        <div className="toolbar toolbar--end">
          <button className="secondary-button" type="button" onClick={() => void openCustomerPortal()} disabled={!canManageBilling || actionLoading}>
            <ExternalLink size={16} aria-hidden="true" />
            {t("billing.managePortal")}
          </button>
          <button className="secondary-button" type="button" onClick={() => void rebuildUsage()} disabled={!canManageBilling || actionLoading}>
            <RotateCcw size={16} aria-hidden="true" />
            {t("billing.rebuildUsage")}
          </button>
          <button className="secondary-button" type="button" onClick={() => void loadBilling()} disabled={loading}>
            <RefreshCw size={16} aria-hidden="true" />
            {t("billing.refresh")}
          </button>
        </div>
      </div>

      <section className="panel">
        <div className="panel-header">
          <div>
            <h2>{t("billing.current.title")}</h2>
            <span>{t("billing.current.description")}</span>
          </div>
          <CreditCard size={18} aria-hidden="true" />
        </div>
        <div className="billing-summary">
          <div className="metric-card metric-card--strong">
            <span>{t("billing.current.plan")}</span>
            <strong>{usage?.subscription.plan_name ?? "-"}</strong>
          </div>
          <div className="metric-card">
            <span>{t("billing.current.status")}</span>
            <strong>{usage?.subscription.status ?? "-"}</strong>
          </div>
          <div className="metric-card">
            <span>{t("billing.current.period")}</span>
            <strong>
              {usage ? `${new Date(usage.subscription.current_period_start).toLocaleDateString()} - ${new Date(usage.subscription.current_period_end).toLocaleDateString()}` : "-"}
            </strong>
          </div>
        </div>
      </section>

      <section className="panel">
        <div className="panel-header">
          <div>
            <h2>{t("billing.usage.title")}</h2>
            <span>{t("billing.usage.description")}</span>
          </div>
          <Gauge size={18} aria-hidden="true" />
        </div>
        <div className="usage-grid padded">
          {usage ? (
            USAGE_ITEMS.map((item) => {
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
              <span>{t("billing.usage.emptyDescription")}</span>
            </div>
          )}
        </div>
      </section>

      <section className="panel">
        <div className="panel-header">
          <div>
            <h2>{t("billing.plans.title")}</h2>
            <span>{t("billing.plans.description")}</span>
          </div>
          <ShieldCheck size={18} aria-hidden="true" />
        </div>
        <div className="plan-grid padded">
          {plans.map((plan) => {
            const active = usage?.subscription.plan_slug === plan.slug;
            const paidPlan = plan.slug !== "free";
            const usesStripeCheckout = paidPlan && plan.stripe_checkout_available;
            const disabled = active || !canManageBilling || actionLoading;
            const label = active
              ? t("billing.plans.current")
              : usesStripeCheckout
                ? t("billing.plans.checkout")
                : paidPlan
                  ? t("billing.plans.change")
                  : t("billing.plans.switchFree");
            return (
              <article className={`plan-card ${active ? "plan-card--active" : ""}`} key={plan.id}>
                <div>
                  <span>{plan.description}</span>
                  <h3>{plan.name}</h3>
                  <strong>
                    {isCustomPriced(plan) ? (
                      t("billing.plans.customPrice")
                    ) : (
                      <>
                        {formatMoney(plan.price_monthly_cents)}<small>{t("billing.plans.perMonth")}</small>
                      </>
                    )}
                  </strong>
                </div>
                <ul>
                  <li>{t("billing.plans.members", { limit: plan.member_limit < 0 ? t("billing.unlimited") : plan.member_limit })}</li>
                  <li>{t("billing.plans.content", { limit: plan.content_limit < 0 ? t("billing.unlimited") : plan.content_limit })}</li>
                  <li>{t("billing.plans.media", { limit: plan.media_limit_mb < 0 ? t("billing.unlimited") : `${plan.media_limit_mb} MB` })}</li>
                  <li>{t("billing.plans.api", { limit: plan.api_requests_limit < 0 ? t("billing.unlimited") : plan.api_requests_limit })}</li>
                </ul>
                <button
                  className={active ? "secondary-button" : "primary-button"}
                  type="button"
                  onClick={() => void (usesStripeCheckout ? startCheckout(plan) : changePlan(plan))}
                  disabled={disabled}
                >
                  {label}
                </button>
              </article>
            );
          })}
        </div>
      </section>
    </div>
  );
}