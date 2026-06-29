import { useCallback, useEffect, useMemo, useState } from "react";
import { Activity, AlertTriangle, Building2, Copy, Crown, Gauge, Globe2, LogOut, Mail, Plus, RefreshCw, Send, Shield, Trash2, UserPlus, Users } from "lucide-react";
import { useLocation } from "react-router-dom";

import { StatusBadge } from "../components/StatusBadge";
import { useI18n, type MessageKey } from "../i18n";
import { ApiError, api } from "../services/api";
import { useAppStore } from "../stores/useAppStore";
import type {
  CreatedInvitationResponse,
  EmailDeliveryResponse,
  AuditLogResponse,
  JsonRecord,
  OrganizationDetailResponse,
  OrganizationDomainResponse,
  OrganizationInvitationResponse,
  OrganizationMemberResponse,
  OrganizationRole,
  UpdateRateLimitRequest,
  SaasAlertRuleResponse,
  RateLimitResponse,
  OrganizationWorkspaceResponse,
} from "../types/api";

const ROLE_OPTIONS: OrganizationRole[] = ["owner", "admin", "editor", "author", "viewer", "billing_manager"];
const ROLE_LABEL_KEYS: Record<OrganizationRole, MessageKey> = {
  owner: "organization.role.owner",
  admin: "organization.role.admin",
  editor: "organization.role.editor",
  author: "organization.role.author",
  viewer: "organization.role.viewer",
  billing_manager: "organization.role.billing_manager",
};
const MANAGER_ROLES = new Set<OrganizationRole>(["owner", "admin"]);

type OrganizationDraft = {
  name: string;
  slug: string;
  settingsJson: string;
};

type CreateOrganizationDraft = {
  name: string;
  slug: string;
};

type InviteDraft = {
  email: string;
  role: OrganizationRole;
};

type DomainDraft = {
  domain: string;
  is_primary: boolean;
};

type RateLimitDraft = UpdateRateLimitRequest;

function toRateLimitDraft(rateLimit: RateLimitResponse | null): RateLimitDraft {
  return {
    requests_per_minute: rateLimit?.requests_per_minute ?? 600,
    user_requests_per_minute: rateLimit?.user_requests_per_minute ?? 120,
    burst: rateLimit?.burst ?? 120,
  };
}

function apiMessage(caught: unknown, fallback: string) {
  return caught instanceof ApiError ? caught.message : fallback;
}

function slugify(value: string) {
  return value
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "")
    .replace(/--+/g, "-");
}

function toDraft(detail: OrganizationDetailResponse): OrganizationDraft {
  return {
    name: detail.organization.name,
    slug: detail.organization.slug,
    settingsJson: JSON.stringify(detail.organization.settings ?? {}, null, 2),
  };
}

function parseSettingsJson(value: string): JsonRecord {
  const parsed = JSON.parse(value) as unknown;
  if (!parsed || typeof parsed !== "object" || Array.isArray(parsed)) {
    throw new Error("settings must be an object");
  }
  return parsed as JsonRecord;
}

export function OrganizationPage() {
  const { t } = useI18n();
  const location = useLocation();
  const user = useAppStore((state) => state.user);
  const activeOrganizationId = useAppStore((state) => state.activeOrganizationId);
  const setOrganizations = useAppStore((state) => state.setOrganizations);

  const [detail, setDetail] = useState<OrganizationDetailResponse | null>(null);
  const [members, setMembers] = useState<OrganizationMemberResponse[]>([]);
  const [invitations, setInvitations] = useState<OrganizationInvitationResponse[]>([]);
  const [workspace, setWorkspace] = useState<OrganizationWorkspaceResponse | null>(null);
  const [domains, setDomains] = useState<OrganizationDomainResponse[]>([]);
  const [rateLimit, setRateLimit] = useState<RateLimitResponse | null>(null);
  const [auditLogs, setAuditLogs] = useState<AuditLogResponse[]>([]);
  const [emailDeliveries, setEmailDeliveries] = useState<EmailDeliveryResponse[]>([]);
  const [alerts, setAlerts] = useState<SaasAlertRuleResponse[]>([]);
  const [organizationDraft, setOrganizationDraft] = useState<OrganizationDraft>({
    name: "",
    slug: "",
    settingsJson: "{}",
  });
  const [createDraft, setCreateDraft] = useState<CreateOrganizationDraft>({ name: "", slug: "" });
  const [inviteDraft, setInviteDraft] = useState<InviteDraft>({ email: "", role: "editor" });
  const [domainDraft, setDomainDraft] = useState<DomainDraft>({ domain: "", is_primary: false });
  const [rateLimitDraft, setRateLimitDraft] = useState<RateLimitDraft>(toRateLimitDraft(null));
  const [acceptToken, setAcceptToken] = useState("");
  const [roleDrafts, setRoleDrafts] = useState<Record<string, OrganizationRole>>({});
  const [createdInvitation, setCreatedInvitation] = useState<CreatedInvitationResponse | null>(null);
  const [loading, setLoading] = useState(false);
  const [actionLoading, setActionLoading] = useState(false);
  const [message, setMessage] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const membershipRole = detail?.membership.role as OrganizationRole | undefined;
  const canManage = membershipRole ? MANAGER_ROLES.has(membershipRole) : false;
  const isOwner = membershipRole === "owner";
  const ownerCount = members.filter((member) => member.role === "owner" && member.status === "active").length;
  const inviteLink = createdInvitation ? `${window.location.origin}${createdInvitation.accept_path}` : "";

  const visibleRoleOptions = useMemo(() => {
    return isOwner ? ROLE_OPTIONS : ROLE_OPTIONS.filter((role) => role !== "owner");
  }, [isOwner]);

  const refreshOrganizationList = useCallback(
    async function refreshOrganizationList(preferredOrganizationId?: string | null) {
      const organizations = await api.organizations.list();
      setOrganizations(organizations, preferredOrganizationId);
      return organizations;
    },
    [setOrganizations],
  );

  const loadWorkspace = useCallback(
    async function loadWorkspace() {
      setLoading(true);
      setError(null);
      try {
        await refreshOrganizationList(activeOrganizationId);
        if (!activeOrganizationId) {
          setDetail(null);
          setMembers([]);
          setInvitations([]);
          setWorkspace(null);
          setDomains([]);
          setRateLimit(null);
          setAuditLogs([]);
          setEmailDeliveries([]);
          setAlerts([]);
          return;
        }

        const [current, nextWorkspace] = await Promise.all([
          api.organizations.current(),
          api.organizations.workspace(),
        ]);
        setDetail(current);
        setOrganizationDraft(toDraft(current));
        setWorkspace(nextWorkspace);
        setDomains(nextWorkspace.domains);

        const role = current.membership.role as OrganizationRole;
        if (MANAGER_ROLES.has(role)) {
          const [nextMembers, nextInvitations, nextDomains, nextRateLimit, nextAuditLogs, nextEmailDeliveries, nextAlerts] = await Promise.all([
            api.organizations.members(),
            api.organizations.invitations(),
            api.organizations.domains(),
            api.organizations.rateLimit(),
            api.organizations.auditLogs(),
            api.organizations.emailDeliveries(),
            api.organizations.alerts(),
          ]);
          setMembers(nextMembers);
          setInvitations(nextInvitations);
          setDomains(nextDomains);
          setRateLimit(nextRateLimit);
          setRateLimitDraft(toRateLimitDraft(nextRateLimit));
          setAuditLogs(nextAuditLogs);
          setEmailDeliveries(nextEmailDeliveries);
          setAlerts(nextAlerts);
          setRoleDrafts(Object.fromEntries(nextMembers.map((member) => [member.user_id, member.role])));
        } else {
          setMembers([]);
          setInvitations([]);
          setRateLimit(null);
          setAuditLogs([]);
          setEmailDeliveries([]);
          setAlerts([]);
          setRoleDrafts({});
        }
      } catch (caught) {
        setError(apiMessage(caught, t("organization.error.load")));
      } finally {
        setLoading(false);
      }
    },
    [activeOrganizationId, refreshOrganizationList, t],
  );

  useEffect(() => {
    const token = new URLSearchParams(location.search).get("invite");
    if (token) setAcceptToken(token);
  }, [location.search]);

  useEffect(() => {
    void loadWorkspace();
  }, [loadWorkspace]);

  function setCreateName(name: string) {
    setCreateDraft((current) => ({
      name,
      slug: current.slug ? current.slug : slugify(name),
    }));
  }

  async function saveCurrentOrganization() {
    setActionLoading(true);
    setError(null);
    setMessage(null);
    try {
      const settings = parseSettingsJson(organizationDraft.settingsJson);
      const updated = await api.organizations.updateCurrent({
        name: organizationDraft.name,
        slug: organizationDraft.slug,
        settings,
      });
      setDetail(updated);
      setOrganizationDraft(toDraft(updated));
      await refreshOrganizationList(updated.organization.id);
      setMessage(t("organization.message.updated"));
    } catch (caught) {
      setError(apiMessage(caught, t("organization.error.save")));
    } finally {
      setActionLoading(false);
    }
  }

  async function createOrganization() {
    setActionLoading(true);
    setError(null);
    setMessage(null);
    try {
      const created = await api.organizations.create(createDraft);
      setDetail(created);
      setOrganizationDraft(toDraft(created));
      setCreateDraft({ name: "", slug: "" });
      await refreshOrganizationList(created.organization.id);
      setMessage(t("organization.message.created"));
    } catch (caught) {
      setError(apiMessage(caught, t("organization.error.create")));
    } finally {
      setActionLoading(false);
    }
  }

  async function inviteMember() {
    setActionLoading(true);
    setError(null);
    setMessage(null);
    try {
      const invitation = await api.organizations.invite(inviteDraft);
      setCreatedInvitation(invitation);
      setInviteDraft({ email: "", role: "editor" });
      setInvitations(await api.organizations.invitations());
      setMessage(t("organization.message.invited"));
    } catch (caught) {
      setError(apiMessage(caught, t("organization.error.invite")));
    } finally {
      setActionLoading(false);
    }
  }

  async function acceptInvitation() {
    setActionLoading(true);
    setError(null);
    setMessage(null);
    try {
      const membership = await api.organizations.acceptInvitation({ token: acceptToken });
      await refreshOrganizationList(membership.id);
      setAcceptToken("");
      setMessage(t("organization.message.accepted"));
    } catch (caught) {
      setError(apiMessage(caught, t("organization.error.accept")));
    } finally {
      setActionLoading(false);
    }
  }

  async function updateMemberRole(member: OrganizationMemberResponse) {
    const nextRole = roleDrafts[member.user_id] ?? member.role;
    setActionLoading(true);
    setError(null);
    setMessage(null);
    try {
      const updated = await api.organizations.updateMember(member.user_id, { role: nextRole });
      setMembers((current) => current.map((item) => (item.user_id === updated.user_id ? updated : item)));
      setRoleDrafts((current) => ({ ...current, [updated.user_id]: updated.role }));
      setMessage(t("organization.message.roleUpdated"));
    } catch (caught) {
      setError(apiMessage(caught, t("organization.error.updateRole")));
    } finally {
      setActionLoading(false);
    }
  }

  async function removeMember(member: OrganizationMemberResponse) {
    if (!window.confirm(t("organization.confirm.removeMember", { email: member.email }))) return;
    setActionLoading(true);
    setError(null);
    setMessage(null);
    try {
      const removed = await api.organizations.removeMember(member.user_id);
      setMembers((current) => current.filter((item) => item.user_id !== removed.user_id));
      setMessage(t("organization.message.removed"));
    } catch (caught) {
      setError(apiMessage(caught, t("organization.error.remove")));
    } finally {
      setActionLoading(false);
    }
  }

  async function revokeInvitation(invitation: OrganizationInvitationResponse) {
    setActionLoading(true);
    setError(null);
    setMessage(null);
    try {
      const revoked = await api.organizations.revokeInvitation(invitation.id);
      setInvitations((current) => current.map((item) => (item.id === revoked.id ? revoked : item)));
      setMessage(t("organization.message.revoked"));
    } catch (caught) {
      setError(apiMessage(caught, t("organization.error.revoke")));
    } finally {
      setActionLoading(false);
    }
  }

  async function leaveOrganization() {
    if (!window.confirm(t("organization.confirm.leave"))) return;
    setActionLoading(true);
    setError(null);
    setMessage(null);
    try {
      await api.organizations.leave();
      await refreshOrganizationList(null);
      setDetail(null);
      setMembers([]);
      setInvitations([]);
      setMessage(t("organization.message.left"));
    } catch (caught) {
      setError(apiMessage(caught, t("organization.error.leave")));
    } finally {
      setActionLoading(false);
    }
  }

  async function transferOwnership(member: OrganizationMemberResponse) {
    if (!window.confirm(t("organization.confirm.transfer", { email: member.email }))) return;
    setActionLoading(true);
    setError(null);
    setMessage(null);
    try {
      await api.organizations.transferOwnership({ user_id: member.user_id });
      await loadWorkspace();
      setMessage(t("organization.message.transferred"));
    } catch (caught) {
      setError(apiMessage(caught, t("organization.error.transfer")));
    } finally {
      setActionLoading(false);
    }
  }

  async function copyInviteLink() {
    await window.navigator.clipboard.writeText(inviteLink);
    setMessage(t("organization.message.copied"));
  }

  async function copyWorkspaceUrl() {
    if (!workspace?.workspace_url) return;
    await window.navigator.clipboard.writeText(workspace.workspace_url);
    setMessage(t("organization.message.workspaceCopied"));
  }

  async function createDomain() {
    setActionLoading(true);
    setError(null);
    setMessage(null);
    try {
      await api.organizations.createDomain(domainDraft);
      const [nextWorkspace, nextDomains] = await Promise.all([
        api.organizations.workspace(),
        api.organizations.domains(),
      ]);
      setWorkspace(nextWorkspace);
      setDomains(nextDomains);
      setDomainDraft({ domain: "", is_primary: false });
      setMessage(t("organization.message.domainCreated"));
    } catch (caught) {
      setError(apiMessage(caught, t("organization.error.domain")));
    } finally {
      setActionLoading(false);
    }
  }

  async function deleteDomain(domain: OrganizationDomainResponse) {
    if (!window.confirm(t("organization.confirm.deleteDomain", { domain: domain.domain }))) return;
    setActionLoading(true);
    setError(null);
    setMessage(null);
    try {
      await api.organizations.deleteDomain(domain.id);
      const [nextWorkspace, nextDomains] = await Promise.all([
        api.organizations.workspace(),
        api.organizations.domains(),
      ]);
      setWorkspace(nextWorkspace);
      setDomains(nextDomains);
      setMessage(t("organization.message.domainDeleted"));
    } catch (caught) {
      setError(apiMessage(caught, t("organization.error.domain")));
    } finally {
      setActionLoading(false);
    }
  }

  async function saveRateLimit() {
    setActionLoading(true);
    setError(null);
    setMessage(null);
    try {
      const updated = await api.organizations.updateRateLimit(rateLimitDraft);
      setRateLimit(updated);
      setRateLimitDraft(toRateLimitDraft(updated));
      setMessage(t("organization.message.rateLimitUpdated"));
    } catch (caught) {
      setError(apiMessage(caught, t("organization.error.rateLimit")));
    } finally {
      setActionLoading(false);
    }
  }

  function roleLabel(role: OrganizationRole) {
    return t(ROLE_LABEL_KEYS[role]);
  }
  function memberRoleOptions(member: OrganizationMemberResponse) {
    return visibleRoleOptions.includes(member.role) ? visibleRoleOptions : [member.role, ...visibleRoleOptions];
  }

  function canRemoveMember(member: OrganizationMemberResponse) {
    if (member.user_id === user?.id) return false;
    if (member.role === "owner" && (!isOwner || ownerCount <= 1)) return false;
    return canManage;
  }

  function canUpdateMember(member: OrganizationMemberResponse) {
    const nextRole = roleDrafts[member.user_id] ?? member.role;
    if (nextRole === member.role) return false;
    if ((member.role === "owner" || nextRole === "owner") && !isOwner) return false;
    if (member.role === "owner" && nextRole !== "owner" && ownerCount <= 1) return false;
    return canManage;
  }

  return (
    <div className="page-stack organization-page">
      <div className="panel-actions">
        <div className="status-stack">
          {loading && <StatusBadge label={t("common.loading")} tone="neutral" />}
          {error && <StatusBadge label={error} tone="danger" />}
          {message && <StatusBadge label={message} tone="success" />}
        </div>
        <button className="secondary-button" type="button" onClick={() => void loadWorkspace()} disabled={loading}>
          <RefreshCw size={16} aria-hidden="true" />
          {t("organization.refresh")}
        </button>
      </div>

      <div className="organization-grid">
        <section className="panel">
          <div className="panel-header">
            <div>
              <h2>{t("organization.current.title")}</h2>
              <span>{t("organization.current.description")}</span>
            </div>
            <Building2 size={18} aria-hidden="true" />
          </div>
          {detail ? (
            <div className="form-grid padded">
              <label>
                {t("common.name")}
                <input
                  value={organizationDraft.name}
                  onChange={(event) => setOrganizationDraft((current) => ({ ...current, name: event.target.value }))}
                  disabled={!canManage}
                />
              </label>
              <label>
                {t("common.slug")}
                <input
                  value={organizationDraft.slug}
                  onChange={(event) => setOrganizationDraft((current) => ({ ...current, slug: slugify(event.target.value) }))}
                  disabled={!canManage}
                />
              </label>
              <label>
                {t("organization.settingsJson")}
                <textarea
                  rows={5}
                  value={organizationDraft.settingsJson}
                  onChange={(event) => setOrganizationDraft((current) => ({ ...current, settingsJson: event.target.value }))}
                  disabled={!canManage}
                />
              </label>
              <div className="organization-meta">
                <StatusBadge label={detail.membership.role} tone="neutral" />
                <span>{t("organization.membershipStatus", { status: detail.membership.status })}</span>
              </div>
              <div className="panel-actions">
                <button className="primary-button" type="button" onClick={() => void saveCurrentOrganization()} disabled={!canManage || actionLoading}>
                  <Shield size={16} aria-hidden="true" />
                  {t("organization.saveSettings")}
                </button>
                <button className="secondary-button" type="button" onClick={() => void leaveOrganization()} disabled={actionLoading || (membershipRole === "owner" && ownerCount <= 1)}>
                  <LogOut size={16} aria-hidden="true" />
                  {t("organization.leave")}
                </button>
              </div>
            </div>
          ) : (
            <div className="empty-state padded">
              <strong>{t("organization.noActive.title")}</strong>
              <span>{t("organization.noActive.description")}</span>
            </div>
          )}
        </section>

        <section className="panel">
          <div className="panel-header">
            <div>
              <h2>{t("organization.plan.title")}</h2>
              <span>{t("organization.plan.description")}</span>
            </div>
            <Crown size={18} aria-hidden="true" />
          </div>
          <div className="metric-grid padded">
            <div className="metric-card metric-card--strong">
              <span>{t("organization.plan.current")}</span>
              <strong>{detail?.plan_limits.plan ?? "Free"}</strong>
            </div>
            <div className="metric-card">
              <span>{t("organization.plan.members")}</span>
              <strong>{members.length}/{detail?.plan_limits.members_limit ?? 3}</strong>
            </div>
            <div className="metric-card">
              <span>{t("organization.plan.content")}</span>
              <strong>{detail?.plan_limits.content_limit ?? 50}</strong>
            </div>
            <div className="metric-card">
              <span>{t("organization.plan.media")}</span>
              <strong>{detail?.plan_limits.media_limit_mb ?? 1024} MB</strong>
            </div>
          </div>
        </section>
      </div>

      {detail && (
        <section className="panel full-width-panel">
          <div className="panel-header">
            <div>
              <h2>{t("organization.workspace.title")}</h2>
              <span>{t("organization.workspace.description")}</span>
            </div>
            <Globe2 size={18} aria-hidden="true" />
          </div>
          <div className="form-grid organization-inline-form padded">
            <label>
              {t("organization.workspace.url")}
              <input value={workspace?.workspace_url ?? ""} readOnly />
            </label>
            <button className="secondary-button" type="button" onClick={() => void copyWorkspaceUrl()} disabled={!workspace?.workspace_url}>
              <Copy size={16} aria-hidden="true" />
              {t("organization.copyLink")}
            </button>
          </div>
          {canManage && (
            <div className="form-grid organization-inline-form padded">
              <label>
                {t("organization.domain.domain")}
                <input
                  value={domainDraft.domain}
                  onChange={(event) => setDomainDraft((current) => ({ ...current, domain: event.target.value }))}
                  placeholder="cms.example.com"
                />
              </label>
              <label>
                {t("organization.domain.primary")}
                <input
                  type="checkbox"
                  checked={domainDraft.is_primary}
                  onChange={(event) => setDomainDraft((current) => ({ ...current, is_primary: event.target.checked }))}
                />
              </label>
              <button className="primary-button" type="button" onClick={() => void createDomain()} disabled={actionLoading || !domainDraft.domain.trim()}>
                <Plus size={16} aria-hidden="true" />
                {t("organization.domain.add")}
              </button>
            </div>
          )}
          <div className="table-scroll">
            <table className="data-table">
              <thead>
                <tr>
                  <th>{t("organization.domain.domain")}</th>
                  <th>{t("common.status")}</th>
                  <th>{t("organization.domain.primary")}</th>
                  <th>{t("organization.domain.verification")}</th>
                  {canManage && <th>{t("common.actions")}</th>}
                </tr>
              </thead>
              <tbody>
                {domains.length === 0 ? (
                  <tr>
                    <td colSpan={canManage ? 5 : 4}>{t("organization.domain.empty")}</td>
                  </tr>
                ) : (
                  domains.map((domain) => (
                    <tr key={domain.id}>
                      <td>{domain.domain}</td>
                      <td><StatusBadge label={domain.status} tone={domain.status === "verified" ? "success" : "warning"} /></td>
                      <td>{domain.is_primary ? t("common.active") : "-"}</td>
                      <td><code>{domain.verification_token}</code></td>
                      {canManage && (
                        <td>
                          <button className="icon-button" type="button" onClick={() => void deleteDomain(domain)} disabled={actionLoading} aria-label={t("organization.domain.delete", { domain: domain.domain })}>
                            <Trash2 size={16} aria-hidden="true" />
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
      )}

      {canManage && (
        <section className="panel full-width-panel">
          <div className="panel-header">
            <div>
              <h2>{t("organization.rateLimit.title")}</h2>
              <span>{t("organization.rateLimit.description")}</span>
            </div>
            <Gauge size={18} aria-hidden="true" />
          </div>
          <div className="form-grid organization-inline-form padded">
            <label>
              {t("organization.rateLimit.organization")}
              <input type="number" min={1} value={rateLimitDraft.requests_per_minute} onChange={(event) => setRateLimitDraft((current) => ({ ...current, requests_per_minute: Number(event.target.value) }))} />
            </label>
            <label>
              {t("organization.rateLimit.user")}
              <input type="number" min={1} value={rateLimitDraft.user_requests_per_minute} onChange={(event) => setRateLimitDraft((current) => ({ ...current, user_requests_per_minute: Number(event.target.value) }))} />
            </label>
            <label>
              {t("organization.rateLimit.burst")}
              <input type="number" min={0} value={rateLimitDraft.burst} onChange={(event) => setRateLimitDraft((current) => ({ ...current, burst: Number(event.target.value) }))} />
            </label>
            <button className="primary-button" type="button" onClick={() => void saveRateLimit()} disabled={actionLoading}>
              <Shield size={16} aria-hidden="true" />
              {t("organization.rateLimit.save")}
            </button>
          </div>
          <div className="organization-meta padded">
            <StatusBadge label={rateLimit ? t("common.active") : t("app.status.waiting")} tone={rateLimit ? "success" : "neutral"} />
            <span>{rateLimit?.updated_at ? new Date(rateLimit.updated_at).toLocaleString() : "-"}</span>
          </div>
        </section>
      )}

      {canManage && (
        <section className="panel full-width-panel">
          <div className="panel-header">
            <div>
              <h2>{t("organization.audit.title")}</h2>
              <span>{t("organization.audit.description")}</span>
            </div>
            <Activity size={18} aria-hidden="true" />
          </div>
          <div className="table-scroll">
            <table className="data-table">
              <thead>
                <tr>
                  <th>{t("organization.audit.action")}</th>
                  <th>{t("organization.audit.actor")}</th>
                  <th>{t("organization.audit.entity")}</th>
                  <th>{t("organization.audit.metadata")}</th>
                  <th>{t("common.updated")}</th>
                </tr>
              </thead>
              <tbody>
                {auditLogs.length === 0 ? (
                  <tr><td colSpan={5}>{t("organization.audit.empty")}</td></tr>
                ) : (
                  auditLogs.map((log) => (
                    <tr key={log.id}>
                      <td>{log.action}</td>
                      <td>{log.actor_email ?? log.actor_id ?? "-"}</td>
                      <td>{log.entity_type}</td>
                      <td><code>{JSON.stringify(log.metadata)}</code></td>
                      <td>{new Date(log.created_at).toLocaleString()}</td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </section>
      )}

      {canManage && (
        <section className="panel full-width-panel">
          <div className="panel-header">
            <div>
              <h2>{t("organization.email.title")}</h2>
              <span>{t("organization.email.description")}</span>
            </div>
            <Mail size={18} aria-hidden="true" />
          </div>
          <div className="table-scroll">
            <table className="data-table">
              <thead>
                <tr>
                  <th>{t("common.email")}</th>
                  <th>{t("organization.email.template")}</th>
                  <th>{t("organization.email.provider")}</th>
                  <th>{t("common.status")}</th>
                  <th>{t("common.updated")}</th>
                </tr>
              </thead>
              <tbody>
                {emailDeliveries.length === 0 ? (
                  <tr><td colSpan={5}>{t("organization.email.empty")}</td></tr>
                ) : (
                  emailDeliveries.map((delivery) => (
                    <tr key={delivery.id}>
                      <td>{delivery.recipient_email}</td>
                      <td>{delivery.template}</td>
                      <td>{delivery.provider}</td>
                      <td><StatusBadge label={delivery.status} tone={delivery.status === "sent" ? "success" : delivery.status === "failed" ? "danger" : "neutral"} /></td>
                      <td>{new Date(delivery.updated_at).toLocaleString()}</td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </section>
      )}

      {canManage && (
        <section className="panel full-width-panel">
          <div className="panel-header">
            <div>
              <h2>{t("organization.alerts.title")}</h2>
              <span>{t("organization.alerts.description")}</span>
            </div>
            <AlertTriangle size={18} aria-hidden="true" />
          </div>
          <div className="metric-grid padded">
            {alerts.map((alert) => (
              <div className="metric-card" key={alert.id}>
                <span>{alert.alert_key}</span>
                <strong>{alert.severity}</strong>
                <StatusBadge label={alert.is_enabled ? t("common.enabled") : t("common.disabled")} tone={alert.is_enabled ? "success" : "neutral"} />
              </div>
            ))}
            {alerts.length === 0 && <div className="empty-state"><span>{t("organization.alerts.empty")}</span></div>}
          </div>
        </section>
      )}

      <section className="panel full-width-panel">
        <div className="panel-header">
          <div>
            <h2>{t("organization.create.title")}</h2>
            <span>{t("organization.create.description")}</span>
          </div>
          <Plus size={18} aria-hidden="true" />
        </div>
        <div className="form-grid organization-inline-form padded">
          <label>
            {t("common.name")}
            <input
              value={createDraft.name}
              onChange={(event) => setCreateName(event.target.value)}
              placeholder={t("organization.create.namePlaceholder")}
            />
          </label>
          <label>
            {t("common.slug")}
            <input
              value={createDraft.slug}
              onChange={(event) => setCreateDraft((current) => ({ ...current, slug: slugify(event.target.value) }))}
              placeholder={t("organization.create.slugPlaceholder")}
            />
          </label>
          <button className="primary-button" type="button" onClick={() => void createOrganization()} disabled={actionLoading}>
            <Plus size={16} aria-hidden="true" />
            {t("organization.create.submit")}
          </button>
        </div>
      </section>

      <section className="panel full-width-panel">
        <div className="panel-header">
          <div>
            <h2>{t("organization.accept.title")}</h2>
            <span>{t("organization.accept.description")}</span>
          </div>
          <UserPlus size={18} aria-hidden="true" />
        </div>
        <div className="form-grid organization-inline-form padded">
          <label>
            {t("organization.invitationToken")}
            <input value={acceptToken} onChange={(event) => setAcceptToken(event.target.value)} />
          </label>
          <button className="secondary-button" type="button" onClick={() => void acceptInvitation()} disabled={actionLoading || !acceptToken.trim()}>
            <Send size={16} aria-hidden="true" />
            {t("organization.accept.submit")}
          </button>
        </div>
      </section>

      {canManage && (
        <section className="panel full-width-panel">
          <div className="panel-header">
            <div>
              <h2>{t("organization.invite.title")}</h2>
              <span>{t("organization.invite.description")}</span>
            </div>
            <UserPlus size={18} aria-hidden="true" />
          </div>
          <div className="form-grid organization-inline-form padded">
            <label>
              {t("common.email")}
              <input
                value={inviteDraft.email}
                onChange={(event) => setInviteDraft((current) => ({ ...current, email: event.target.value }))}
                placeholder="editor@example.com"
              />
            </label>
            <label>
              {t("settings.role")}
              <select
                value={inviteDraft.role}
                onChange={(event) => setInviteDraft((current) => ({ ...current, role: event.target.value as OrganizationRole }))}
              >
                {visibleRoleOptions.map((role) => (
                  <option key={role} value={role}>
                    {roleLabel(role)}
                  </option>
                ))}
              </select>
            </label>
            <button className="primary-button" type="button" onClick={() => void inviteMember()} disabled={actionLoading}>
              <Send size={16} aria-hidden="true" />
              {t("organization.invite.submit")}
            </button>
          </div>
          {createdInvitation && (
            <div className="invite-link-box padded">
              <span>{t("organization.invite.link")}</span>
              <code>{inviteLink}</code>
              <button className="secondary-button" type="button" onClick={() => void copyInviteLink()}>
                <Copy size={16} aria-hidden="true" />
                {t("organization.copyLink")}
              </button>
            </div>
          )}
        </section>
      )}

      {canManage && (
        <section className="panel full-width-panel">
          <div className="panel-header">
            <div>
              <h2>{t("organization.members.title")}</h2>
              <span>{t("organization.members.description")}</span>
            </div>
            <Users size={18} aria-hidden="true" />
          </div>
          <div className="table-scroll">
            <table className="data-table">
              <thead>
                <tr>
                  <th>{t("common.name")}</th>
                  <th>{t("common.email")}</th>
                  <th>{t("settings.role")}</th>
                  <th>{t("common.status")}</th>
                  <th>{t("common.actions")}</th>
                </tr>
              </thead>
              <tbody>
                {members.length === 0 ? (
                  <tr>
                    <td colSpan={5}>{t("organization.members.empty")}</td>
                  </tr>
                ) : (
                  members.map((member) => (
                    <tr key={member.user_id}>
                      <td>
                        <div className="member-name">
                          <strong>{member.name}</strong>
                          {member.user_id === user?.id && <span>{t("organization.member.you")}</span>}
                        </div>
                      </td>
                      <td>{member.email}</td>
                      <td>
                        <select
                          className="table-select"
                          value={roleDrafts[member.user_id] ?? member.role}
                          onChange={(event) =>
                            setRoleDrafts((current) => ({
                              ...current,
                              [member.user_id]: event.target.value as OrganizationRole,
                            }))
                          }
                        >
                          {memberRoleOptions(member).map((role) => (
                            <option key={role} value={role}>
                              {roleLabel(role)}
                            </option>
                          ))}
                        </select>
                      </td>
                      <td>
                        <StatusBadge label={member.status} tone={member.status === "active" ? "success" : "neutral"} />
                      </td>
                      <td>
                        <div className="table-actions">
                          <button className="secondary-button" type="button" onClick={() => void updateMemberRole(member)} disabled={!canUpdateMember(member) || actionLoading}>
                            <Shield size={16} aria-hidden="true" />
                            {t("organization.member.updateRole")}
                          </button>
                          {isOwner && member.user_id !== user?.id && (
                            <button className="secondary-button" type="button" onClick={() => void transferOwnership(member)} disabled={actionLoading}>
                              <Crown size={16} aria-hidden="true" />
                              {t("organization.member.transfer")}
                            </button>
                          )}
                          <button className="icon-button" type="button" onClick={() => void removeMember(member)} disabled={!canRemoveMember(member) || actionLoading} aria-label={t("organization.member.remove", { email: member.email })}>
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
        </section>
      )}

      {canManage && (
        <section className="panel full-width-panel">
          <div className="panel-header">
            <div>
              <h2>{t("organization.invitations.title")}</h2>
              <span>{t("organization.invitations.description")}</span>
            </div>
            <Send size={18} aria-hidden="true" />
          </div>
          <div className="table-scroll">
            <table className="data-table">
              <thead>
                <tr>
                  <th>{t("common.email")}</th>
                  <th>{t("settings.role")}</th>
                  <th>{t("common.status")}</th>
                  <th>{t("organization.expiresAt")}</th>
                  <th>{t("common.actions")}</th>
                </tr>
              </thead>
              <tbody>
                {invitations.length === 0 ? (
                  <tr>
                    <td colSpan={5}>{t("organization.invitations.empty")}</td>
                  </tr>
                ) : (
                  invitations.map((invitation) => (
                    <tr key={invitation.id}>
                      <td>{invitation.email}</td>
                      <td>{roleLabel(invitation.role)}</td>
                      <td>
                        <StatusBadge label={invitation.status} tone={invitation.status === "pending" ? "warning" : "neutral"} />
                      </td>
                      <td>{new Date(invitation.expires_at).toLocaleDateString()}</td>
                      <td>
                        <button className="secondary-button" type="button" onClick={() => void revokeInvitation(invitation)} disabled={invitation.status !== "pending" || actionLoading}>
                          <Trash2 size={16} aria-hidden="true" />
                          {t("organization.invitations.revoke")}
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
    </div>
  );
}
