import { useEffect, useMemo, useState } from "react";
import type { ReactNode } from "react";

import { I18nContext, interpolate, type I18nContextValue } from "./context";
import { defaultLocale, fallbackLocale, getLocaleDirection, localeStorageKey, normalizeLocale, type Locale } from "./locales";
import { messages, type MessageKey, type MessageParams } from "./messages";

function readInitialLocale(): Locale {
  if (typeof window === "undefined") return defaultLocale;

  const stored = normalizeLocale(window.localStorage.getItem(localeStorageKey));
  if (stored) return stored;

  for (const browserLocale of window.navigator.languages ?? []) {
    const matched = normalizeLocale(browserLocale);
    if (matched) return matched;
  }

  return normalizeLocale(window.navigator.language) ?? defaultLocale;
}

type I18nProviderProps = {
  children: ReactNode;
};

export function I18nProvider({ children }: I18nProviderProps) {
  const [locale, setLocaleState] = useState<Locale>(readInitialLocale);
  const direction = getLocaleDirection(locale);

  useEffect(() => {
    document.documentElement.lang = locale;
    document.documentElement.dir = direction;
    window.localStorage.setItem(localeStorageKey, locale);
  }, [direction, locale]);

  const value = useMemo<I18nContextValue>(() => {
    function setLocale(nextLocale: Locale) {
      setLocaleState(nextLocale);
    }

    function t(key: MessageKey, params?: MessageParams) {
      const localized = messages[locale][key] ?? messages[fallbackLocale][key] ?? key;
      return interpolate(localized, params);
    }

    return { locale, direction, setLocale, t };
  }, [direction, locale]);

  return <I18nContext.Provider value={value}>{children}</I18nContext.Provider>;
}