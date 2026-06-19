import { Navigate } from "react-router-dom";

import { AppShell } from "./AppShell";
import { useAppStore } from "../stores/useAppStore";

export function RequireAuth() {
  const accessToken = useAppStore((state) => state.accessToken);
  return accessToken ? <AppShell /> : <Navigate to="/login" replace />;
}