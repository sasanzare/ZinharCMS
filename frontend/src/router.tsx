import { createBrowserRouter, Navigate } from "react-router-dom";

import { AppShell } from "./components/AppShell";
import { ContentTypesPage } from "./pages/ContentTypesPage";
import { DashboardPage } from "./pages/DashboardPage";
import { EntriesPage } from "./pages/EntriesPage";
import { MediaPage } from "./pages/MediaPage";
import { PagesPage } from "./pages/PagesPage";
import { SettingsPage } from "./pages/SettingsPage";

export const router = createBrowserRouter([
  {
    path: "/",
    element: <AppShell />,
    children: [
      { index: true, element: <DashboardPage /> },
      { path: "content-types", element: <ContentTypesPage /> },
      { path: "entries", element: <EntriesPage /> },
      { path: "media", element: <MediaPage /> },
      { path: "pages", element: <PagesPage /> },
      { path: "settings", element: <SettingsPage /> },
      { path: "*", element: <Navigate to="/" replace /> },
    ],
  },
]);
