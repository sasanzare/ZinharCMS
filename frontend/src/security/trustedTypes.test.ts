import { afterEach, describe, expect, it, vi } from "vitest";

describe("Trusted Types rich-content policy", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
    vi.resetModules();
  });

  it("creates one named policy and sanitizes every accepted value", async () => {
    const createPolicy = vi.fn(
      (name: string, rules: TrustedTypePolicyOptions) =>
        ({
          name,
          createHTML: (input: string) => rules.createHTML?.(input) ?? "",
        }) as unknown as TrustedTypePolicy,
    );
    vi.stubGlobal("trustedTypes", { createPolicy });
    vi.resetModules();

    const richContent = await import("./richContent");
    const first = richContent.createSanitizedRichHtml(
      "<p>visible</p><script>blocked</script>",
    );
    const second = richContent.createSanitizedRichHtml(
      "<img src=x onerror=blocked>",
    );

    expect(createPolicy).toHaveBeenCalledTimes(2);
    expect(createPolicy.mock.calls.map(([name]) => name)).toEqual(
      expect.arrayContaining([
        richContent.DOMPURIFY_TRUSTED_TYPES_POLICY,
        richContent.RICH_CONTENT_TRUSTED_TYPES_POLICY,
      ]),
    );
    expect(createPolicy).toHaveBeenCalledWith(
      richContent.RICH_CONTENT_TRUSTED_TYPES_POLICY,
      expect.objectContaining({ createHTML: expect.any(Function) }),
    );
    expect(first.toString()).toBe("<p>visible</p>");
    expect(second.toString()).not.toContain("onerror");
  });

  it("keeps sanitization active when Trusted Types are unsupported", async () => {
    vi.stubGlobal("trustedTypes", undefined);
    vi.resetModules();

    const richContent = await import("./richContent");
    const result = richContent.createSanitizedRichHtml(
      "<svg><script>blocked</script></svg><p>visible</p>",
    );

    expect(result.toString()).toBe("<p>visible</p>");
  });
});
