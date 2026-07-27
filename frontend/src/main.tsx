import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { RouterProvider } from "react-router/dom";

import { I18nProvider } from "./i18n";
import { SessionBootstrap } from "./components/SessionBootstrap";
import { router } from "./router";
import "./styles/index.css";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <I18nProvider>
      <SessionBootstrap>
        <RouterProvider router={router} />
      </SessionBootstrap>
    </I18nProvider>
  </StrictMode>,
);
