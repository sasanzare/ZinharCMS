import DOMPurify from "dompurify";

export const MAX_RICH_TEXT_INPUT_BYTES = 128 * 1024;
export const MAX_RICH_TEXT_TAGS = 4_096;
export const MAX_RICH_TEXT_ATTRIBUTES = 4_096;
export const MAX_RICH_TEXT_NESTING = 128;
export const MAX_INTERACTIVE_URL_BYTES = 2_048;
export const RICH_CONTENT_TRUSTED_TYPES_POLICY = "zinhar-rich-content";
export const DOMPURIFY_TRUSTED_TYPES_POLICY = "dompurify";

const ALLOWED_TAGS = [
  "a",
  "b",
  "blockquote",
  "br",
  "code",
  "del",
  "em",
  "h1",
  "h2",
  "h3",
  "h4",
  "h5",
  "h6",
  "hr",
  "i",
  "img",
  "li",
  "ol",
  "p",
  "pre",
  "s",
  "span",
  "strong",
  "table",
  "tbody",
  "td",
  "tfoot",
  "th",
  "thead",
  "tr",
  "u",
  "ul",
] as const;

const ALLOWED_ATTRIBUTES = [
  "alt",
  "aria-label",
  "colspan",
  "height",
  "href",
  "rel",
  "rowspan",
  "scope",
  "src",
  "target",
  "title",
  "width",
] as const;

const ALLOWED_ATTRIBUTES_BY_ELEMENT: Readonly<Record<string, ReadonlySet<string>>> =
  {
    a: new Set(["href", "target", "title", "aria-label", "rel"]),
    img: new Set(["src", "alt", "title", "width", "height", "aria-label"]),
    td: new Set(["colspan", "rowspan"]),
    th: new Set(["colspan", "rowspan", "scope"]),
  };

const sanitizedRichHtmlBrand: unique symbol = Symbol("sanitizedRichHtml");

export type SanitizedRichHtml = {
  readonly [sanitizedRichHtmlBrand]: true;
  toString(): string;
};

type SanitizedRichHtmlValue = SanitizedRichHtml & {
  readonly trustedValue: string | TrustedHTML;
};

type HtmlTrustedTypesPolicy = Pick<
  TrustedTypePolicy,
  "name" | "createHTML"
>;

let trustedTypesPolicy: HtmlTrustedTypesPolicy | null | undefined;
let hooksInstalled = false;

export function createSanitizedRichHtml(input: string): SanitizedRichHtml {
  assertRichContentBounds(input);
  installSanitizerHooks();
  const policy = getTrustedTypesPolicy();
  const trustedValue = policy
    ? policy.createHTML(input)
    : sanitizeToString(input);
  const serialized = String(trustedValue);
  return Object.freeze({
    [sanitizedRichHtmlBrand]: true as const,
    trustedValue,
    toString: () => serialized,
  });
}

export function sanitizedRichHtmlValue(
  value: SanitizedRichHtml,
): string | TrustedHTML {
  return (value as SanitizedRichHtmlValue).trustedValue;
}

function hasAsciiControlCharacter(value: string): boolean {
  return Array.from(value).some((character) => {
    const codePoint = character.codePointAt(0) ?? 0;
    return codePoint <= 0x1f || codePoint === 0x7f;
  });
}

export function sanitizeInteractiveUrl(input: string): string | null {
  if (new TextEncoder().encode(input).byteLength > MAX_INTERACTIVE_URL_BYTES) {
    return null;
  }
  const value = input.trim();
  if (
    value.length === 0 ||
    value.includes("\\") ||
    hasAsciiControlCharacter(value) ||
    value.startsWith("//")
  ) {
    return null;
  }
  if (
    value.startsWith("/") ||
    value.startsWith("./") ||
    value.startsWith("../") ||
    (value.startsWith("#") && !value.slice(1).includes("#"))
  ) {
    return value;
  }

  const decoded = decodeUrlPrefix(value);
  if (decoded === null) return null;
  try {
    const parsed = new URL(decoded);
    if (!["https:", "mailto:", "tel:"].includes(parsed.protocol)) return null;
    if (
      parsed.protocol === "https:" &&
      (parsed.username.length > 0 || parsed.password.length > 0)
    ) {
      return null;
    }
    return value;
  } catch {
    return null;
  }
}

export function sanitizeImageUrl(input: string): string | null {
  const safe = sanitizeInteractiveUrl(input);
  if (safe === null || safe.startsWith("#")) return null;
  if (
    safe.startsWith("/") ||
    safe.startsWith("./") ||
    safe.startsWith("../")
  ) {
    return safe;
  }
  try {
    return new URL(safe).protocol === "https:" ? safe : null;
  } catch {
    return null;
  }
}

export function navigateToTrustedExternalUrl(input: string) {
  const safe = sanitizeInteractiveUrl(input);
  if (safe === null) {
    throw new Error("External navigation URL is not allowed");
  }
  const parsed = new URL(safe);
  if (parsed.protocol !== "https:") {
    throw new Error("External navigation requires HTTPS");
  }
  window.location.assign(safe);
}

function sanitizeToString(input: string): string {
  return String(
    DOMPurify.sanitize(input, {
      ALLOWED_TAGS: [...ALLOWED_TAGS],
      ALLOWED_ATTR: [...ALLOWED_ATTRIBUTES],
      ALLOW_ARIA_ATTR: false,
      ALLOW_DATA_ATTR: false,
      FORBID_TAGS: [
        "applet",
        "audio",
        "base",
        "button",
        "canvas",
        "embed",
        "form",
        "frame",
        "frameset",
        "iframe",
        "input",
        "link",
        "math",
        "meta",
        "noscript",
        "object",
        "portal",
        "script",
        "style",
        "svg",
        "template",
        "video",
      ],
      FORBID_ATTR: ["class", "id", "name", "srcdoc", "style"],
      RETURN_TRUSTED_TYPE: false,
    }),
  );
}

function getTrustedTypesPolicy(): HtmlTrustedTypesPolicy | null {
  if (trustedTypesPolicy !== undefined) return trustedTypesPolicy;
  const factory = (
    globalThis as typeof globalThis & {
      trustedTypes?: TrustedTypePolicyFactory;
    }
  ).trustedTypes;
  if (factory === undefined) {
    trustedTypesPolicy = null;
    return trustedTypesPolicy;
  }
  const policy = factory.createPolicy(
    RICH_CONTENT_TRUSTED_TYPES_POLICY,
    {
      createHTML: (input: string) => sanitizeToString(input),
    },
  );
  trustedTypesPolicy = policy;
  return policy;
}

function installSanitizerHooks() {
  if (hooksInstalled) return;
  hooksInstalled = true;
  DOMPurify.addHook("uponSanitizeAttribute", (node, event) => {
    const element = node.nodeName.toLowerCase();
    const attribute = event.attrName.toLowerCase();
    if (attribute.startsWith("on")) {
      event.keepAttr = false;
      return;
    }
    if (!ALLOWED_ATTRIBUTES_BY_ELEMENT[element]?.has(attribute)) {
      event.keepAttr = false;
      return;
    }
    if (element === "a" && attribute === "href") {
      const safe = sanitizeInteractiveUrl(event.attrValue);
      event.keepAttr = safe !== null;
      if (safe !== null) event.attrValue = safe;
      return;
    }
    if (
      element === "img" &&
      (attribute === "width" || attribute === "height")
    ) {
      event.keepAttr = isBoundedInteger(event.attrValue, 1, 4_096);
      return;
    }
    if (
      (element === "td" || element === "th") &&
      (attribute === "colspan" || attribute === "rowspan")
    ) {
      event.keepAttr = isBoundedInteger(event.attrValue, 1, 100);
      return;
    }
    if (element === "img" && attribute === "src") {
      const safe = sanitizeSameOriginMediaUrl(event.attrValue);
      event.keepAttr = safe !== null;
      if (safe !== null) event.attrValue = safe;
      return;
    }
    if (
      element === "a" &&
      attribute === "target" &&
      !["_blank", "_self"].includes(event.attrValue)
    ) {
      event.keepAttr = false;
    }
  });
  DOMPurify.addHook("afterSanitizeAttributes", (node) => {
    if (node.nodeName.toLowerCase() === "a") {
      node.setAttribute("rel", "noopener noreferrer");
    }
  });
}

function isBoundedInteger(value: string, minimum: number, maximum: number) {
  if (!/^[0-9]+$/u.test(value)) return false;
  const parsed = Number(value);
  return Number.isSafeInteger(parsed) && parsed >= minimum && parsed <= maximum;
}

function sanitizeSameOriginMediaUrl(input: string): string | null {
  const value = input.trim();
  if (
    new TextEncoder().encode(value).byteLength > MAX_INTERACTIVE_URL_BYTES ||
    !value.startsWith("/") ||
    value.startsWith("//") ||
    value.includes("\\") ||
    hasAsciiControlCharacter(value)
  ) {
    return null;
  }
  return value;
}

function assertRichContentBounds(input: string) {
  if (new TextEncoder().encode(input).byteLength > MAX_RICH_TEXT_INPUT_BYTES) {
    throw new Error("Rich text exceeds the maximum size");
  }

  let cursor = 0;
  let depth = 0;
  let tags = 0;
  let attributes = 0;
  while (cursor < input.length) {
    const start = input.indexOf("<", cursor);
    if (start === -1) break;
    const end = input.indexOf(">", start);
    if (end === -1) break;
    const tag = input.slice(start + 1, end).trim();
    cursor = end + 1;
    if (tag.length === 0 || tag.startsWith("!") || tag.startsWith("?")) continue;

    tags += 1;
    attributes += [...tag].filter((character) => character === "=").length;
    if (tags > MAX_RICH_TEXT_TAGS || attributes > MAX_RICH_TEXT_ATTRIBUTES) {
      throw new Error("Rich text is too complex");
    }

    if (tag.startsWith("/")) {
      depth = Math.max(0, depth - 1);
    } else if (!tag.endsWith("/") && !isVoidTag(tag)) {
      depth += 1;
      if (depth > MAX_RICH_TEXT_NESTING) {
        throw new Error("Rich text is nested too deeply");
      }
    }
  }
}

function isVoidTag(tag: string) {
  const name = tag.split(/\s/u, 1)[0]?.toLowerCase() ?? "";
  return [
    "area",
    "base",
    "br",
    "col",
    "embed",
    "hr",
    "img",
    "input",
    "link",
    "meta",
    "param",
    "source",
    "track",
    "wbr",
  ].includes(name);
}

function decodeUrlPrefix(value: string): string | null {
  const delimiter = value.search(/[/?#]/u);
  const limit = Math.min(delimiter === -1 ? value.length : delimiter, 64);
  const prefix = value.slice(0, limit);
  try {
    const decoded = prefix.replace(/%[0-9a-f]{2}/giu, (encoded) =>
      String.fromCharCode(Number.parseInt(encoded.slice(1), 16)),
    );
    return `${decoded}${value.slice(limit)}`;
  } catch {
    return null;
  }
}
