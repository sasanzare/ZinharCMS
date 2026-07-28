export const RICH_CONTENT_TRUSTED_TYPES_POLICY = "zinhar-rich-content";
export const DOMPURIFY_TRUSTED_TYPES_POLICY = "dompurify";

type ProductionCspOptions = {
  apiOrigin: string;
  webSocketOrigin: string;
  mediaOrigin?: string;
};

type DevelopmentCspOptions = {
  apiOrigin: string;
  webSocketOrigin: string;
  developmentOrigin: string;
};

export function buildProductionCsp(options: ProductionCspOptions) {
  const apiOrigin = canonicalOrigin(options.apiOrigin);
  const webSocketOrigin = canonicalWebSocketOrigin(options.webSocketOrigin);
  const mediaOrigin = options.mediaOrigin
    ? canonicalOrigin(options.mediaOrigin)
    : apiOrigin;
  return serializeDirectives([
    ["default-src", "'none'"],
    ["base-uri", "'none'"],
    ["object-src", "'none'"],
    ["frame-ancestors", "'none'"],
    ["form-action", "'self'"],
    ["script-src", "'self'"],
    ["script-src-attr", "'none'"],
    ["style-src", "'self'"],
    ["img-src", "'self'", mediaOrigin, "https:"],
    ["font-src", "'self'"],
    ["connect-src", "'self'", apiOrigin, webSocketOrigin],
    ["worker-src", "'self'"],
    ["manifest-src", "'self'"],
    ["media-src", "'self'", mediaOrigin],
    ["frame-src", "'none'"],
    ["require-trusted-types-for", "'script'"],
    [
      "trusted-types",
      RICH_CONTENT_TRUSTED_TYPES_POLICY,
      DOMPURIFY_TRUSTED_TYPES_POLICY,
    ],
    ["upgrade-insecure-requests"],
  ]);
}

export function buildDevelopmentCsp(options: DevelopmentCspOptions) {
  const apiOrigin = canonicalOrigin(options.apiOrigin);
  const webSocketOrigin = canonicalWebSocketOrigin(options.webSocketOrigin);
  const developmentOrigin = canonicalOrigin(options.developmentOrigin);
  const developmentWebSocketOrigin = developmentOrigin.replace(/^http/u, "ws");
  return serializeDirectives([
    ["default-src", "'none'"],
    ["base-uri", "'none'"],
    ["object-src", "'none'"],
    ["frame-ancestors", "'none'"],
    ["form-action", "'self'"],
    ["script-src", "'self'"],
    ["script-src-attr", "'none'"],
    ["style-src", "'self'", "'unsafe-inline'"],
    ["img-src", "'self'", apiOrigin, "https:"],
    ["font-src", "'self'"],
    [
      "connect-src",
      "'self'",
      apiOrigin,
      webSocketOrigin,
      developmentWebSocketOrigin,
    ],
    ["worker-src", "'self'"],
    ["manifest-src", "'self'"],
    ["media-src", "'self'", apiOrigin],
    ["frame-src", "'none'"],
  ]);
}

export function webSocketOriginFor(origin: string) {
  return canonicalOrigin(origin).replace(/^http/u, "ws");
}

function canonicalOrigin(value: string) {
  const url = new URL(value);
  if (
    !["http:", "https:"].includes(url.protocol) ||
    url.username ||
    url.password ||
    url.pathname !== "/" ||
    url.search ||
    url.hash ||
    url.origin === "null"
  ) {
    throw new Error("CSP origin must be a canonical HTTP(S) origin");
  }
  return url.origin;
}

function canonicalWebSocketOrigin(value: string) {
  const url = new URL(value);
  if (
    !["ws:", "wss:"].includes(url.protocol) ||
    url.username ||
    url.password ||
    url.pathname !== "/" ||
    url.search ||
    url.hash ||
    url.origin === "null"
  ) {
    throw new Error("CSP WebSocket origin must be canonical");
  }
  return url.origin;
}

function serializeDirectives(directives: string[][]) {
  return directives.map((directive) => directive.join(" ")).join("; ");
}
