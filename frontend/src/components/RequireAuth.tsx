import { Navigate, useLocation } from "react-router";

import { AppShell } from "./AppShell";
import { useAppStore } from "../stores/useAppStore";

export function RequireAuth() {
  const location = useLocation();
  const authStatus = useAppStore((state) => state.authStatus);
  if (authStatus === "unknown" || authStatus === "refreshing") {
    return <main className="auth-screen" aria-busy="true">Restoring session…</main>;
  }
  return authStatus === "authenticated" ? (
    <AppShell />
  ) : (
    <Navigate
      to="/login"
      replace
      state={{ from: `${location.pathname}${location.search}${location.hash}` }}
    />
  );
}
