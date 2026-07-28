import { describe, expect, it } from "vitest";

import nginxConfig from "../../nginx.conf.template?raw";
import {
  DOMPURIFY_TRUSTED_TYPES_POLICY,
  RICH_CONTENT_TRUSTED_TYPES_POLICY,
  buildDevelopmentCsp,
  buildProductionCsp,
} from "../../securityHeaders";

function directives(policy: string) {
  return new Map(
    policy.split(";").map((directive) => {
      const [name, ...values] = directive.trim().split(/\s+/);
      return [name, values] as const;
    }),
  );
}

describe("frontend CSP contract", () => {
  it("builds an enforced production policy without unsafe script directives", () => {
    const policy = buildProductionCsp({
      apiOrigin: "https://api.example.invalid",
      webSocketOrigin: "wss://api.example.invalid",
      mediaOrigin: "https://media.example.invalid",
    });
    const parsed = directives(policy);

    expect(parsed.get("default-src")).toEqual(["'none'"]);
    expect(parsed.get("base-uri")).toEqual(["'none'"]);
    expect(parsed.get("object-src")).toEqual(["'none'"]);
    expect(parsed.get("frame-ancestors")).toEqual(["'none'"]);
    expect(parsed.get("script-src")).toEqual(["'self'"]);
    expect(parsed.get("script-src-attr")).toEqual(["'none'"]);
    expect(parsed.get("connect-src")).toEqual([
      "'self'",
      "https://api.example.invalid",
      "wss://api.example.invalid",
    ]);
    expect(parsed.get("img-src")).toEqual([
      "'self'",
      "https://media.example.invalid",
      "https:",
    ]);
    expect(parsed.get("require-trusted-types-for")).toEqual(["'script'"]);
    expect(parsed.get("trusted-types")).toEqual([
      RICH_CONTENT_TRUSTED_TYPES_POLICY,
      DOMPURIFY_TRUSTED_TYPES_POLICY,
    ]);
    expect(policy).not.toContain("'unsafe-eval'");
    expect(parsed.get("script-src")).not.toContain("'unsafe-inline'");
    expect(parsed.get("script-src")).not.toContain("*");
  });

  it("keeps development origins explicit and does not weaken script policy", () => {
    const policy = buildDevelopmentCsp({
      apiOrigin: "http://localhost:8080",
      webSocketOrigin: "ws://localhost:8080",
      developmentOrigin: "http://localhost:5173",
    });
    const parsed = directives(policy);

    expect(parsed.get("connect-src")).toEqual([
      "'self'",
      "http://localhost:8080",
      "ws://localhost:8080",
      "ws://localhost:5173",
    ]);
    expect(parsed.get("script-src")).toEqual(["'self'"]);
    expect(policy).not.toContain("'unsafe-eval'");
  });

  it("keeps the Nginx template aligned with enforced production requirements", () => {
    expect(nginxConfig).toContain("Content-Security-Policy");
    expect(nginxConfig).toContain("always");
    expect(nginxConfig).toContain("default-src 'none'");
    expect(nginxConfig).toContain("script-src 'self'");
    expect(nginxConfig).toContain("script-src-attr 'none'");
    expect(nginxConfig).toContain("require-trusted-types-for 'script'");
    expect(nginxConfig).toContain(
      `trusted-types ${RICH_CONTENT_TRUSTED_TYPES_POLICY} ${DOMPURIFY_TRUSTED_TYPES_POLICY}`,
    );
    expect(nginxConfig).not.toContain("'unsafe-eval'");
  });
});
