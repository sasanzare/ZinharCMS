import {
  Database,
  FileText,
  Gauge,
  Image,
  Layers3,
  LogOut,
  Menu,
  Settings,
  Workflow,
} from "lucide-react";
import type { LucideIcon } from "lucide-react";
import { NavLink, Outlet, useLocation } from "react-router-dom";

import { useHealth } from "../hooks/useHealth";
import { api, getStoredRefreshToken } from "../services/api";
import { useAppStore } from "../stores/useAppStore";
import { StatusBadge } from "./StatusBadge";

type NavItem = {
  label: string;
  path: string;
  icon: LucideIcon;
};

const navItems: NavItem[] = [
  { label: "Dashboard", path: "/", icon: Gauge },
  { label: "Content Types", path: "/content-types", icon: Layers3 },
  { label: "Entries", path: "/entries", icon: FileText },
  { label: "Media", path: "/media", icon: Image },
  { label: "Pages", path: "/pages", icon: Workflow },
  { label: "Workflow", path: "/workflow", icon: Workflow },
  { label: "Settings", path: "/settings", icon: Settings },
];

const titles = new Map(navItems.map((item) => [item.path, item.label]));

export function AppShell() {
  const location = useLocation();
  const { sidebarCollapsed, toggleSidebar, user, clearSession } = useAppStore();
  const { readiness, error } = useHealth();
  const title = titles.get(location.pathname) ?? "Dashboard";
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

  return (
    <div className={`app-shell ${sidebarCollapsed ? "app-shell--collapsed" : ""}`}>
      <aside className="sidebar">
        <div className="brand-row">
          <div className="brand-mark">Z</div>
          <div className="brand-copy">
            <strong>ZinharCMS</strong>
            <span>Headless admin</span>
          </div>
        </div>

        <nav className="side-nav" aria-label="Primary navigation">
          {navItems.map((item) => (
            <NavLink
              key={item.path}
              to={item.path}
              end={item.path === "/"}
              className={({ isActive }) => `nav-link ${isActive ? "nav-link--active" : ""}`}
            >
              <item.icon aria-hidden="true" size={18} />
              <span>{item.label}</span>
            </NavLink>
          ))}
        </nav>
      </aside>

      <div className="workspace">
        <header className="topbar">
          <button className="icon-button" type="button" onClick={toggleSidebar} aria-label="Toggle navigation">
            <Menu size={18} aria-hidden="true" />
          </button>
          <div className="topbar-title">
            <h1>{title}</h1>
            <span className="topbar-subtitle">{api.baseUrl}</span>
          </div>
          <div className="topbar-status">
            <Database size={16} aria-hidden="true" />
            <StatusBadge label={ready ? "Ready" : "Checking"} tone={ready ? "success" : "warning"} />
            {user && <span className="user-chip">{user.role}</span>}
            <button className="icon-button" type="button" onClick={handleLogout} aria-label="Logout">
              <LogOut size={18} aria-hidden="true" />
            </button>
          </div>
        </header>

        <main className="content-area">
          <Outlet />
        </main>
      </div>
    </div>
  );
}