export const localeDefinitions = [
  {
    code: "en",
    name: "English",
    nativeName: "English",
    direction: "ltr",
  },
  {
    code: "fa-IR",
    name: "Persian",
    nativeName: "فارسی",
    direction: "rtl",
  },
] as const;

export type Locale = (typeof localeDefinitions)[number]["code"];
export type LocaleDirection = (typeof localeDefinitions)[number]["direction"];

export const defaultLocale: Locale = "en";
export const fallbackLocale: Locale = "en";
export const localeStorageKey = "zinhar.locale";

export const supportedLocales = localeDefinitions.map((locale) => locale.code);

export function isLocale(value: string): value is Locale {
  return supportedLocales.includes(value as Locale);
}

export function normalizeLocale(value: string | null | undefined): Locale | null {
  if (!value) return null;
  const normalized = value.trim();
  if (isLocale(normalized)) return normalized;

  const language = normalized.split("-")[0]?.toLowerCase();
  if (language === "fa") return "fa-IR";
  if (language === "en") return "en";
  return null;
}

export function getLocaleDirection(locale: Locale): LocaleDirection {
  return localeDefinitions.find((definition) => definition.code === locale)?.direction ?? "ltr";
}

export function getLocaleNativeName(locale: Locale): string {
  return localeDefinitions.find((definition) => definition.code === locale)?.nativeName ?? locale;
}