import { useCallback, useEffect, useState } from "react";
import {
  AlertTriangle,
  BarChart3,
  ClipboardList,
  MessageSquarePlus,
  RefreshCw,
  Rocket,
  ShieldAlert,
} from "lucide-react";

import { StatusBadge } from "../components/StatusBadge";
import { useI18n } from "../i18n";
import { ApiError, api } from "../services/api";
import { useAppStore } from "../stores/useAppStore";
import type {
  BetaDashboardResponse,
  BetaFeedbackCategory,
  BetaFeedbackResponse,
  BetaFeedbackSeverity,
  BetaGaBlockerResponse,
  BetaParticipantStatus,
  BetaProductDashboardResponse,
} from "../types/api";

const FEEDBACK_CATEGORIES: BetaFeedbackCategory[] = [
  "bug",
  "ux",
  "billing",
  "performance",
  "tenant_isolation",
  "onboarding",
  "other",
];
const FEEDBACK_SEVERITIES: BetaFeedbackSeverity[] = ["low", "medium", "high", "critical"];
const PARTICIPANT_STATUSES: BetaParticipantStatus[] = [
  "candidate",
  "invited",
  "onboarding",
  "active",
  "paused",
  "graduated",
  "rejected",
];

const BETA_MANAGER_ROLES = new Set(["owner", "admin", "editor"]);

type FeedbackDraft = {
  category: BetaFeedbackCategory;
  severity: BetaFeedbackSeverity;
  title: string;
  description: string;
  page_url: string;
};

type BlockerDraft = {
  priority: "p0" | "p1" | "p2" | "p3";
  area: string;
  title: string;
  owner: string;
  due_at: string;
};

type ParticipantDraft = {
  organization_id: string;
  cohort_label: string;
  contact_name: string;
  contact_email: string;
  status: BetaParticipantStatus;
  notes: string;
};

const defaultFeedbackDraft: FeedbackDraft = {
  category: "ux",
  severity: "medium",
  title: "",
  description: "",
  page_url: "",
};

const defaultBlockerDraft: BlockerDraft = {
  priority: "p2",
  area: "ux",
  title: "",
  owner: "",
  due_at: "",
};

const defaultParticipantDraft: ParticipantDraft = {
  organization_id: "",
  cohort_label: "private-beta",
  contact_name: "",
  contact_email: "",
  status: "candidate",
  notes: "",
};

function apiMessage(caught: unknown, fallback: string) {
  return caught instanceof ApiError ? caught.message : fallback;
}

function toneForSeverity(severity: string) {
  if (severity === "critical" || severity === "high") return "danger";
  if (severity === "medium") return "warning";
  return "neutral";
}

function toneForBlocker(priority: string) {
  if (priority === "p0" || priority === "p1") return "danger";
  if (priority === "p2") return "warning";
  return "neutral";
}

export function BetaPage() {
  const { t } = useI18n();
  const user = useAppStore((state) => state.user);
  const organizations = useAppStore((state) => state.organizations);
  const activeOrganizationId = useAppStore((state) => state.activeOrganizationId);
  const activeMembership = organizations.find((organization) => organization.id === activeOrganizationId);
  const canManageBeta = activeMembership ? BETA_MANAGER_ROLES.has(activeMembership.role) : false;
  const canViewProductDashboard = user?.role === "super_admin" || user?.role === "admin";

  const [dashboard, setDashboard] = useState<BetaDashboardResponse | null>(null);
  const [feedback, setFeedback] = useState<BetaFeedbackResponse[]>([]);
  const [blockers, setBlockers] = useState<BetaGaBlockerResponse[]>([]);
  const [productDashboard, setProductDashboard] = useState<BetaProductDashboardResponse | null>(null);
  const [feedbackDraft, setFeedbackDraft] = useState<FeedbackDraft>(defaultFeedbackDraft);
  const [blockerDraft, setBlockerDraft] = useState<BlockerDraft>(defaultBlockerDraft);
  const [participantDraft, setParticipantDraft] = useState<ParticipantDraft>(defaultParticipantDraft);
  const [loading, setLoading] = useState(false);
  const [actionLoading, setActionLoading] = useState(false);
  const [message, setMessage] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const loadBeta = useCallback(async function loadBeta() {
    setLoading(true);
    setError(null);
    try {
      const [nextDashboard, nextFeedback] = await Promise.all([
        api.beta.dashboard(),
        api.beta.feedback(),
      ]);
      setDashboard(nextDashboard);
      setFeedback(nextFeedback);

      if (canManageBeta) {
        setBlockers(await api.beta.blockers());
      } else {
        setBlockers([]);
      }

      if (canViewProductDashboard) {
        setProductDashboard(await api.beta.productDashboard());
      } else {
        setProductDashboard(null);
      }
    } catch (caught) {
      setError(apiMessage(caught, t("beta.error.load")));
    } finally {
      setLoading(false);
    }
  }, [canManageBeta, canViewProductDashboard, t]);

  useEffect(() => {
    void loadBeta();
  }, [activeOrganizationId, loadBeta]);

  async function submitFeedback() {
    setActionLoading(true);
    setMessage(null);
    setError(null);
    try {
      const created = await api.beta.createFeedback({
        ...feedbackDraft,
        page_url: feedbackDraft.page_url.trim() || window.location.pathname,
      });
      setFeedback((current) => [created, ...current]);
      setFeedbackDraft(defaultFeedbackDraft);
      setMessage(t("beta.message.feedbackCreated"));
      setDashboard(await api.beta.dashboard());
    } catch (caught) {
      setError(apiMessage(caught, t("beta.error.feedback")));
    } finally {
      setActionLoading(false);
    }
  }

  async function triageFeedback(item: BetaFeedbackResponse) {
    setActionLoading(true);
    setError(null);
    try {
      const updated = await api.beta.updateFeedback(item.id, { status: "triaged" });
      setFeedback((current) => current.map((entry) => (entry.id === updated.id ? updated : entry)));
      setMessage(t("beta.message.feedbackTriaged"));
      setDashboard(await api.beta.dashboard());
    } catch (caught) {
      setError(apiMessage(caught, t("beta.error.update")));
    } finally {
      setActionLoading(false);
    }
  }

  async function createBlocker() {
    setActionLoading(true);
    setMessage(null);
    setError(null);
    try {
      const created = await api.beta.createBlocker({
        priority: blockerDraft.priority,
        area: blockerDraft.area,
        title: blockerDraft.title,
        owner: blockerDraft.owner || undefined,
        due_at: blockerDraft.due_at || undefined,
      });
      setBlockers((current) => [created, ...current]);
      setBlockerDraft(defaultBlockerDraft);
      setMessage(t("beta.message.blockerCreated"));
      setDashboard(await api.beta.dashboard());
    } catch (caught) {
      setError(apiMessage(caught, t("beta.error.blocker")));
    } finally {
      setActionLoading(false);
    }
  }

  async function resolveBlocker(item: BetaGaBlockerResponse) {
    setActionLoading(true);
    setError(null);
    try {
      const updated = await api.beta.updateBlocker(item.id, { status: "resolved" });
      setBlockers((current) => current.map((blocker) => (blocker.id === updated.id ? updated : blocker)));
      setMessage(t("beta.message.blockerResolved"));
      setDashboard(await api.beta.dashboard());
    } catch (caught) {
      setError(apiMessage(caught, t("beta.error.update")));
    } finally {
      setActionLoading(false);
    }
  }

  async function saveParticipant() {
    setActionLoading(true);
    setMessage(null);
    setError(null);
    try {
      await api.beta.upsertParticipant(participantDraft.organization_id, {
        cohort_label: participantDraft.cohort_label,
        contact_name: participantDraft.contact_name || undefined,
        contact_email: participantDraft.contact_email || undefined,
        status: participantDraft.status,
        notes: participantDraft.notes || undefined,
      });
      setParticipantDraft(defaultParticipantDraft);
      setProductDashboard(await api.beta.productDashboard());
      setMessage(t("beta.message.participantSaved"));
    } catch (caught) {
      setError(apiMessage(caught, t("beta.error.participant")));
    } finally {
      setActionLoading(false);
    }
  }

  const org = dashboard?.organization;

  return (
    <div className="page-stack beta-page">
      <div className="panel-actions">
        <div className="status-stack">
          {loading && <StatusBadge label={t("common.loading")} tone="neutral" />}
          {error && <StatusBadge label={error} tone="danger" />}
          {message && <StatusBadge label={message} tone="success" />}
        </div>
        <button className="secondary-button" type="button" onClick={() => void loadBeta()} disabled={loading}>
          <RefreshCw size={16} aria-hidden="true" />
          {t("beta.refresh")}
        </button>
      </div>

      <section className="metric-grid">
        <article className="metric-card metric-card--strong">
          <Rocket size={20} aria-hidden="true" />
          <span>{t("beta.dashboard.openFeedback")}</span>
          <strong>{org?.open_feedback ?? 0}</strong>
        </article>
        <article className="metric-card">
          <ShieldAlert size={20} aria-hidden="true" />
          <span>{t("beta.dashboard.criticalFeedback")}</span>
          <strong>{org?.critical_feedback ?? 0}</strong>
        </article>
        <article className="metric-card">
          <ClipboardList size={20} aria-hidden="true" />
          <span>{t("beta.dashboard.blockers")}</span>
          <strong>{org?.open_ga_blockers ?? 0}</strong>
        </article>
        <article className="metric-card">
          <BarChart3 size={20} aria-hidden="true" />
          <span>{t("beta.dashboard.currentPlan")}</span>
          <strong>{org?.current_plan ?? "-"}</strong>
        </article>
      </section>

      <section className="panel full-width-panel">
        <div className="panel-header">
          <div>
            <h2>{t("beta.dashboard.title")}</h2>
            <span>{t("beta.dashboard.description")}</span>
          </div>
          <StatusBadge label={org?.participant_status ?? "untracked"} tone={org?.participant_status === "active" ? "success" : "neutral"} />
        </div>
        <div className="beta-health-grid padded">
          <div>
            <span>{t("beta.dashboard.billingFailures")}</span>
            <strong>{org?.failed_billing_events ?? 0}</strong>
          </div>
          <div>
            <span>{t("beta.dashboard.emailFailures")}</span>
            <strong>{org?.failed_email_deliveries ?? 0}</strong>
          </div>
          <div>
            <span>{t("beta.dashboard.exceededUsage")}</span>
            <strong>
              {dashboard?.exceeded_usage_metrics.length
                ? dashboard.exceeded_usage_metrics.join(", ")
                : t("beta.dashboard.noExceededUsage")}
            </strong>
          </div>
        </div>
      </section>

      <section className="panel beta-feedback-panel">
        <div className="panel-header">
          <div>
            <h2>{t("beta.feedback.title")}</h2>
            <span>{t("beta.feedback.description")}</span>
          </div>
          <MessageSquarePlus size={18} aria-hidden="true" />
        </div>
        <div className="form-grid padded">
          <label>
            {t("beta.feedback.category")}
            <select
              value={feedbackDraft.category}
              onChange={(event) => setFeedbackDraft((current) => ({ ...current, category: event.target.value as BetaFeedbackCategory }))}
            >
              {FEEDBACK_CATEGORIES.map((category) => (
                <option key={category} value={category}>
                  {category}
                </option>
              ))}
            </select>
          </label>
          <label>
            {t("beta.feedback.severity")}
            <select
              value={feedbackDraft.severity}
              onChange={(event) => setFeedbackDraft((current) => ({ ...current, severity: event.target.value as BetaFeedbackSeverity }))}
            >
              {FEEDBACK_SEVERITIES.map((severity) => (
                <option key={severity} value={severity}>
                  {severity}
                </option>
              ))}
            </select>
          </label>
          <label>
            {t("common.title")}
            <input
              value={feedbackDraft.title}
              onChange={(event) => setFeedbackDraft((current) => ({ ...current, title: event.target.value }))}
            />
          </label>
          <label>
            {t("common.description")}
            <textarea
              rows={5}
              value={feedbackDraft.description}
              onChange={(event) => setFeedbackDraft((current) => ({ ...current, description: event.target.value }))}
            />
          </label>
          <label>
            {t("beta.feedback.pageUrl")}
            <input
              value={feedbackDraft.page_url}
              onChange={(event) => setFeedbackDraft((current) => ({ ...current, page_url: event.target.value }))}
              placeholder={window.location.pathname}
            />
          </label>
          <button
            className="primary-button"
            type="button"
            onClick={() => void submitFeedback()}
            disabled={actionLoading || !feedbackDraft.title.trim() || !feedbackDraft.description.trim()}
          >
            <MessageSquarePlus size={16} aria-hidden="true" />
            {t("beta.feedback.submit")}
          </button>
        </div>
      </section>

      <section className="panel beta-list-panel">
        <div className="panel-header">
          <div>
            <h2>{t("beta.feedback.recent")}</h2>
            <span>{t("beta.feedback.description")}</span>
          </div>
          <AlertTriangle size={18} aria-hidden="true" />
        </div>
        <div className="table-scroll">
          <table className="data-table">
            <thead>
              <tr>
                <th>{t("common.title")}</th>
                <th>{t("beta.feedback.category")}</th>
                <th>{t("beta.feedback.severity")}</th>
                <th>{t("common.status")}</th>
                <th>{t("beta.feedback.submittedBy")}</th>
                {canManageBeta && <th>{t("common.actions")}</th>}
              </tr>
            </thead>
            <tbody>
              {feedback.length === 0 ? (
                <tr><td colSpan={canManageBeta ? 6 : 5}>{t("beta.feedback.empty")}</td></tr>
              ) : (
                feedback.map((item) => (
                  <tr key={item.id}>
                    <td>
                      <strong>{item.title}</strong>
                      <span className="table-subtext">{item.page_url ?? "-"}</span>
                    </td>
                    <td>{item.category}</td>
                    <td><StatusBadge label={item.severity} tone={toneForSeverity(item.severity)} /></td>
                    <td>{item.status}</td>
                    <td>{item.submitted_by_email ?? "-"}</td>
                    {canManageBeta && (
                      <td>
                        <button className="secondary-button" type="button" onClick={() => void triageFeedback(item)} disabled={actionLoading || item.status !== "open"}>
                          {t("common.submit")}
                        </button>
                      </td>
                    )}
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>
      </section>

      {canManageBeta && (
        <section className="panel full-width-panel">
          <div className="panel-header">
            <div>
              <h2>{t("beta.blockers.title")}</h2>
              <span>{t("beta.blockers.description")}</span>
            </div>
            <ClipboardList size={18} aria-hidden="true" />
          </div>
          <div className="form-grid beta-inline-form padded">
            <label>
              {t("beta.blockers.priority")}
              <select value={blockerDraft.priority} onChange={(event) => setBlockerDraft((current) => ({ ...current, priority: event.target.value as BlockerDraft["priority"] }))}>
                {["p0", "p1", "p2", "p3"].map((priority) => <option key={priority} value={priority}>{priority}</option>)}
              </select>
            </label>
            <label>
              {t("beta.blockers.area")}
              <input value={blockerDraft.area} onChange={(event) => setBlockerDraft((current) => ({ ...current, area: event.target.value }))} />
            </label>
            <label>
              {t("common.title")}
              <input value={blockerDraft.title} onChange={(event) => setBlockerDraft((current) => ({ ...current, title: event.target.value }))} />
            </label>
            <label>
              {t("beta.blockers.owner")}
              <input value={blockerDraft.owner} onChange={(event) => setBlockerDraft((current) => ({ ...current, owner: event.target.value }))} />
            </label>
            <label>
              {t("beta.blockers.dueAt")}
              <input type="date" value={blockerDraft.due_at} onChange={(event) => setBlockerDraft((current) => ({ ...current, due_at: event.target.value }))} />
            </label>
            <button className="primary-button" type="button" onClick={() => void createBlocker()} disabled={actionLoading || !blockerDraft.area.trim() || !blockerDraft.title.trim()}>
              {t("beta.blockers.submit")}
            </button>
          </div>
          <div className="table-scroll">
            <table className="data-table">
              <thead>
                <tr>
                  <th>{t("common.title")}</th>
                  <th>{t("beta.blockers.priority")}</th>
                  <th>{t("beta.blockers.area")}</th>
                  <th>{t("common.status")}</th>
                  <th>{t("beta.blockers.owner")}</th>
                  <th>{t("common.actions")}</th>
                </tr>
              </thead>
              <tbody>
                {blockers.length === 0 ? (
                  <tr><td colSpan={6}>{t("beta.blockers.empty")}</td></tr>
                ) : (
                  blockers.map((item) => (
                    <tr key={item.id}>
                      <td>{item.title}</td>
                      <td><StatusBadge label={item.priority} tone={toneForBlocker(item.priority)} /></td>
                      <td>{item.area}</td>
                      <td>{item.status}</td>
                      <td>{item.owner ?? "-"}</td>
                      <td>
                        <button className="secondary-button" type="button" onClick={() => void resolveBlocker(item)} disabled={actionLoading || item.status === "resolved"}>
                          {t("beta.blockers.resolve")}
                        </button>
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </section>
      )}

      {canViewProductDashboard && (
        <section className="panel full-width-panel">
          <div className="panel-header">
            <div>
              <h2>{t("beta.product.title")}</h2>
              <span>{t("beta.product.description")}</span>
            </div>
            <BarChart3 size={18} aria-hidden="true" />
          </div>
          <div className="metric-grid padded">
            <div className="metric-card">
              <span>{t("beta.product.organizations")}</span>
              <strong>{productDashboard?.totals.beta_organizations ?? 0}</strong>
            </div>
            <div className="metric-card">
              <span>{t("beta.product.activeOrganizations")}</span>
              <strong>{productDashboard?.totals.active_organizations ?? 0}</strong>
            </div>
            <div className="metric-card">
              <span>{t("beta.dashboard.criticalFeedback")}</span>
              <strong>{productDashboard?.totals.critical_feedback ?? 0}</strong>
            </div>
          </div>
          <div className="form-grid beta-inline-form padded">
            <label>
              {t("beta.product.organizationId")}
              <input value={participantDraft.organization_id} onChange={(event) => setParticipantDraft((current) => ({ ...current, organization_id: event.target.value }))} />
            </label>
            <label>
              {t("beta.product.cohort")}
              <input value={participantDraft.cohort_label} onChange={(event) => setParticipantDraft((current) => ({ ...current, cohort_label: event.target.value }))} />
            </label>
            <label>
              {t("beta.product.status")}
              <select value={participantDraft.status} onChange={(event) => setParticipantDraft((current) => ({ ...current, status: event.target.value as BetaParticipantStatus }))}>
                {PARTICIPANT_STATUSES.map((status) => <option key={status} value={status}>{status}</option>)}
              </select>
            </label>
            <label>
              {t("beta.product.contactEmail")}
              <input value={participantDraft.contact_email} onChange={(event) => setParticipantDraft((current) => ({ ...current, contact_email: event.target.value }))} />
            </label>
            <label>
              {t("beta.product.notes")}
              <input value={participantDraft.notes} onChange={(event) => setParticipantDraft((current) => ({ ...current, notes: event.target.value }))} />
            </label>
            <button className="primary-button" type="button" onClick={() => void saveParticipant()} disabled={actionLoading || !participantDraft.organization_id.trim()}>
              {t("beta.product.saveParticipant")}
            </button>
          </div>
          <div className="table-scroll">
            <table className="data-table">
              <thead>
                <tr>
                  <th>{t("common.name")}</th>
                  <th>{t("beta.product.status")}</th>
                  <th>{t("beta.dashboard.openFeedback")}</th>
                  <th>{t("beta.dashboard.blockers")}</th>
                  <th>{t("beta.dashboard.billingFailures")}</th>
                  <th>{t("beta.dashboard.emailFailures")}</th>
                </tr>
              </thead>
              <tbody>
                {!productDashboard || productDashboard.organizations.length === 0 ? (
                  <tr><td colSpan={6}>{t("beta.product.noOrganizations")}</td></tr>
                ) : (
                  productDashboard.organizations.map((item) => (
                    <tr key={item.organization_id}>
                      <td>
                        <strong>{item.organization_name}</strong>
                        <span className="table-subtext">{item.organization_slug}</span>
                      </td>
                      <td>{item.participant_status ?? "-"}</td>
                      <td>{item.open_feedback}</td>
                      <td>{item.open_ga_blockers}</td>
                      <td>{item.failed_billing_events}</td>
                      <td>{item.failed_email_deliveries}</td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </section>
      )}
    </div>
  );
}
