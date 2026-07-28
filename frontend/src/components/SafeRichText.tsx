import type { HTMLAttributes } from "react";

import {
  type SanitizedRichHtml,
  sanitizedRichHtmlValue,
} from "../security/richContent";

type SafeRichTextProps = Omit<HTMLAttributes<HTMLDivElement>, "children"> & {
  value: SanitizedRichHtml;
};

export function SafeRichText({ value, ...props }: SafeRichTextProps) {
  return (
    <div
      {...props}
      data-rich-content="sanitized"
      dangerouslySetInnerHTML={{
        __html: sanitizedRichHtmlValue(value) as string,
      }}
    />
  );
}
