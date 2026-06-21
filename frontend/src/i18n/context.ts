import { createContext } from "react";

import { defaultLocale, fallbackLocale, getLocaleDirection, type Locale } from "./locales";
import { messages, type MessageKey, type MessageParams } from "./messages";

export type I18nContextValue = {
  locale: Locale;
  direction: "ltr" | "rtl";
  setLocale: (locale: Locale) => void;
  t: (key: MessageKey, params?: MessageParams) => string;
};

export function interpolate(message: string, params?: MessageParams) {
  if (!params) return message;
  return message.replace(/\{(\w+)\}/g, (_, key: string) => String(params[key] ?? `{${key}}`));
}

const fallbackDirection = getLocaleDirection(defaultLocale);

export const I18nContext = createContext<I18nContextValue>({
  locale: defaultLocale,
  direction: fallbackDirection,
  setLocale: () => undefined,
  t: (key, params) => interpolate(messages[fallbackLocale][key] ?? key, params),
});