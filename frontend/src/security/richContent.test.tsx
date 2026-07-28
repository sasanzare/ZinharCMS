import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import corpusDocument from "../../../security/phase4-xss-corpus.json";
import { SafeRichText } from "../components/SafeRichText";
import {
  MAX_RICH_TEXT_INPUT_BYTES,
  createSanitizedRichHtml,
  navigateToTrustedExternalUrl,
  sanitizeImageUrl,
  sanitizeInteractiveUrl,
} from "./richContent";

type Corpus = {
  malicious: Array<{ name: string; html: string }>;
  safe: Array<{ name: string; html: string; fragments: string[] }>;
};

function loadCorpus(): Corpus {
  return corpusDocument as Corpus;
}

describe("approved rich-content rendering boundary", () => {
  it("does not accept an arbitrary string at the approved sink", () => {
    // @ts-expect-error Raw strings must be branded by the sanitizer first.
    const rejectedBoundary = <SafeRichText value="<p>raw</p>" />;
    expect(rejectedBoundary).toBeDefined();
  });

  it("removes executable properties from the shared malicious corpus", () => {
    for (const testCase of loadCorpus().malicious) {
      const sanitized = createSanitizedRichHtml(testCase.html);
      const output = sanitized.toString().toLowerCase();

      for (const forbidden of [
        "<script",
        "<svg",
        "<math",
        "<iframe",
        "<object",
        "<embed",
        "<form",
        "<input",
        "<base",
        "<meta",
        "javascript:",
        "data:",
        "srcdoc",
        "style=",
        " id=",
        " name=",
      ]) {
        expect(output, testCase.name).not.toContain(forbidden);
      }
      expect(output, testCase.name).not.toMatch(/\son[a-z0-9_-]*\s*=/i);
    }
  });

  it("preserves supported safe formatting and link protections", () => {
    for (const testCase of loadCorpus().safe) {
      const output = createSanitizedRichHtml(testCase.html).toString();
      for (const fragment of testCase.fragments) {
        expect(output, testCase.name).toContain(fragment);
      }
    }
  });

  it("enforces attributes per element and canonical numeric bounds", () => {
    const output = createSanitizedRichHtml(
      [
        '<a href="https://example.invalid/docs" aria-label="docs" aria-hidden="true">docs</a>',
        '<p href="https://example.invalid/wrong-element">paragraph</p>',
        '<img src="/media/safe.png" href="/wrong-element" width="0" height="4096" aria-label="preview" aria-hidden="true">',
        '<table><tbody><tr><td colspan="101">cell</td><th rowspan="2" scope="col">heading</th></tr></tbody></table>',
      ].join(""),
    ).toString();
    const container = document.createElement("div");
    container.innerHTML = output;

    const link = container.querySelector("a");
    expect(link).toHaveAttribute("href", "https://example.invalid/docs");
    expect(link).toHaveAttribute("aria-label", "docs");
    expect(link).toHaveAttribute("rel", "noopener noreferrer");
    expect(link).not.toHaveAttribute("aria-hidden");
    expect(container.querySelector("p")).not.toHaveAttribute("href");

    const image = container.querySelector("img");
    expect(image).toHaveAttribute("src", "/media/safe.png");
    expect(image).toHaveAttribute("height", "4096");
    expect(image).toHaveAttribute("aria-label", "preview");
    expect(image).not.toHaveAttribute("width");
    expect(image).not.toHaveAttribute("href");
    expect(image).not.toHaveAttribute("aria-hidden");

    expect(container.querySelector("td")).not.toHaveAttribute("colspan");
    expect(container.querySelector("th")).toHaveAttribute("rowspan", "2");
    expect(container.querySelector("th")).toHaveAttribute("scope", "col");
  });

  it("renders branded sanitized HTML while plain React text remains encoded", () => {
    const plainText = "<img src=x onerror=window.__xssExecuted=true>";
    const { rerender } = render(<p>{plainText}</p>);
    expect(screen.getByText(plainText)).toBeInTheDocument();
    expect(document.querySelector("img")).toBeNull();

    rerender(
      <SafeRichText
        value={createSanitizedRichHtml(
          "<p>Supported <strong>formatting</strong></p><img src=x onerror=window.__xssExecuted=true>",
        )}
      />,
    );
    expect(screen.getByText("formatting")).toBeInTheDocument();
    expect(document.querySelector("[onerror]")).toBeNull();
  });

  it("rejects oversized input and dangerous interactive URLs", () => {
    expect(() =>
      createSanitizedRichHtml("a".repeat(MAX_RICH_TEXT_INPUT_BYTES + 1)),
    ).toThrow();
    expect(sanitizeInteractiveUrl("javascript:alert(1)")).toBeNull();
    expect(sanitizeInteractiveUrl("//example.invalid/path")).toBeNull();
    expect(sanitizeInteractiveUrl("https://user:password@example.invalid")).toBeNull();
    expect(sanitizeInteractiveUrl("https://example.invalid/path")).toBe(
      "https://example.invalid/path",
    );
    expect(sanitizeInteractiveUrl("/guide")).toBe("/guide");
    expect(sanitizeImageUrl("https://images.example.invalid/preview.webp")).toBe(
      "https://images.example.invalid/preview.webp",
    );
    expect(sanitizeImageUrl("mailto:security@example.invalid")).toBeNull();
    expect(() => navigateToTrustedExternalUrl("javascript:blocked")).toThrow(
      "External navigation URL is not allowed",
    );
  });
});
