import { Navigate, createBrowserRouter } from "react-router-dom";

import { RequireAuth } from "./components/RequireAuth";
import { AuthPage } from "./pages/AuthPage";
import { BillingPage } from "./pages/BillingPage";
import { ContentTypesPage } from "./pages/ContentTypesPage";
import { DashboardPage } from "./pages/DashboardPage";
import { EntriesPage } from "./pages/EntriesPage";
import { MediaPage } from "./pages/MediaPage";
import { OrganizationPage } from "./pages/OrganizationPage";
import { PagesPage } from "./pages/PagesPage";
import { SettingsPage } from "./pages/SettingsPage";
import { WorkflowPage } from "./pages/WorkflowPage";
import { WorkspaceRedirectPage } from "./pages/WorkspaceRedirectPage";

export const router = createBrowserRouter([
  { path: "/login", element: <AuthPage /> },
  {
    path: "/",
    element: <RequireAuth />,
    children: [
      { index: true, element: <DashboardPage /> },
      { path: "content-types", element: <ContentTypesPage /> },
      { path: "entries", element: <EntriesPage /> },
      { path: "media", element: <MediaPage /> },
      { path: "pages", element: <PagesPage /> },
      { path: "workflow", element: <WorkflowPage /> },
      { path: "organization", element: <OrganizationPage /> },
      { path: "workspace/:slug", element: <WorkspaceRedirectPage /> },
      { path: "billing", element: <BillingPage /> },
      { path: "settings", element: <SettingsPage /> },
      { path: "*", element: <Navigate to="/" replace /> },
    ],
  },
]);