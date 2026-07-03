import {
  Building2,
  CreditCard,
  Database,
  FileText,
  Gauge,
  Image,
  Layers3,
  LogOut,
  Menu,
  Rocket,
  Settings,
  Store,
  Workflow,
} from "lucide-react";
import type { LucideIcon } from "lucide-react";
import { NavLink, Outlet, useLocation } from "react-router-dom";

import { useHealth } from "../hooks/useHealth";
import { LanguageSelect, useI18n, type MessageKey } from "../i18n";
import { api, getStoredRefreshToken } from "../services/api";
import { useAppStore } from "../stores/useAppStore";
import { StatusBadge } from "./StatusBadge";

type NavItem = {
  labelKey: MessageKey;
  path: string;
  icon: LucideIcon;
};

const navItems: NavItem[] = [
  { labelKey: "app.nav.dashboard", path: "/", icon: Gauge },
  { labelKey: "app.nav.contentTypes", path: "/content-types", icon: Layers3 },
  { labelKey: "app.nav.entries", path: "/entries", icon: FileText },
  { labelKey: "app.nav.media", path: "/media", icon: Image },
  { labelKey: "app.nav.marketplace", path: "/marketplace", icon: Store },
  { labelKey: "app.nav.pages", path: "/pages", icon: Workflow },
  { labelKey: "app.nav.workflow", path: "/workflow", icon: Workflow },
  { labelKey: "app.nav.organization", path: "/organization", icon: Building2 },
  { labelKey: "app.nav.billing", path: "/billing", icon: CreditCard },
  { labelKey: "app.nav.beta", path: "/beta", icon: Rocket },
  { labelKey: "app.nav.settings", path: "/settings", icon: Settings },
];

const titleKeys = new Map(navItems.map((item) => [item.path, item.labelKey]));

export function AppShell() {
  const location = useLocation();
  const { t } = useI18n();
  const { sidebarCollapsed, toggleSidebar, user, organizations, activeOrganizationId, setActiveOrganization, clearSession } = useAppStore();
  const { readiness, error } = useHealth();
  const title = t(titleKeys.get(location.pathname) ?? "app.nav.dashboard");
  const ready = readiness?.status === "ready" && !error;

  async function handleLogout() {
    const refreshToken = getStoredRefreshToken();
    try {
      await api.auth.logout(refreshToken);
    } catch {
      // Local logout should still complete if the token is already invalid.
    }
    clearSession();
  }
  function handleOrganizationChange(nextOrganizationId: string) {
    if (!nextOrganizationId || nextOrganizationId === activeOrganizationId) return;
    setActiveOrganization(nextOrganizationId);
  }

  return (
    <div className={`app-shell ${sidebarCollapsed ? "app-shell--collapsed" : ""}`}>
      <aside className="sidebar">
        <div className="brand-row">
          <div className="brand-mark">Z</div>
          <div className="brand-copy">
            <strong>ZinharCMS</strong>
            <span>{t("app.brand.subtitle")}</span>
          </div>
        </div>

        <nav className="side-nav" aria-label={t("app.nav.aria")}>
          {navItems.map((item) => (
            <NavLink
              key={item.path}
              to={item.path}
              end={item.path === "/"}
              className={({ isActive }) => `nav-link ${isActive ? "nav-link--active" : ""}`}
            >
              <item.icon aria-hidden="true" size={18} />
              <span>{t(item.labelKey)}</span>
            </NavLink>
          ))}
        </nav>
      </aside>

      <div className="workspace">
        <header className="topbar">
          <button className="icon-button" type="button" onClick={toggleSidebar} aria-label={t("app.action.toggleNavigation")}>
            <Menu size={18} aria-hidden="true" />
          </button>
          <div className="topbar-title">
            <h1>{title}</h1>
            <span className="topbar-subtitle">{api.baseUrl}</span>
          </div>
          <div className="topbar-status">
            {organizations.length > 0 && (
              <label className="organization-switcher" aria-label={t("organization.switcher.label")}>
                <Building2 size={16} aria-hidden="true" />
                <select
                  value={activeOrganizationId ?? ""}
                  onChange={(event) => handleOrganizationChange(event.target.value)}
                >
                  {organizations.map((organization) => (
                    <option key={organization.id} value={organization.id}>
                      {organization.name}
                    </option>
                  ))}
                </select>
              </label>
            )}
            <LanguageSelect compact />
            <Database size={16} aria-hidden="true" />
            <StatusBadge label={ready ? t("app.status.ready") : t("app.status.checking")} tone={ready ? "success" : "warning"} />
            {user && <span className="user-chip">{user.role}</span>}
            <button className="icon-button" type="button" onClick={handleLogout} aria-label={t("app.action.logout")}>
              <LogOut size={18} aria-hidden="true" />
            </button>
          </div>
        </header>

        <main className="content-area" key={activeOrganizationId ?? "no-organization"}>
          <Outlet />
        </main>
      </div>
    </div>
  );
}
