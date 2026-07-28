import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig, loadEnv } from "vite";

import {
  buildDevelopmentCsp,
  buildProductionCsp,
  webSocketOriginFor,
} from "./securityHeaders";

export default defineConfig(({ mode }) => {
  const environment = loadEnv(mode, process.cwd(), "");
  const apiOrigin = new URL(
    environment.VITE_API_URL ?? "http://localhost:8080",
  ).origin;
  const commonSecurityHeaders = {
    "Cross-Origin-Opener-Policy": "same-origin",
    "Cross-Origin-Resource-Policy": "same-origin",
    "Permissions-Policy": "camera=(), microphone=(), geolocation=()",
    "Referrer-Policy": "strict-origin-when-cross-origin",
    "X-Content-Type-Options": "nosniff",
    "X-Frame-Options": "DENY",
  };
  return {
    plugins: [react(), tailwindcss()],
    server: {
      host: "0.0.0.0",
      port: 5173,
      headers: {
        "Content-Security-Policy": buildDevelopmentCsp({
          apiOrigin,
          webSocketOrigin: webSocketOriginFor(apiOrigin),
          developmentOrigin: "http://localhost:5173",
        }),
        ...commonSecurityHeaders,
      },
    },
    preview: {
      host: "0.0.0.0",
      port: 5173,
      headers: {
        "Content-Security-Policy": buildProductionCsp({
          apiOrigin,
          webSocketOrigin: webSocketOriginFor(apiOrigin),
          mediaOrigin: apiOrigin,
        }),
        ...commonSecurityHeaders,
      },
    },
  };
});
