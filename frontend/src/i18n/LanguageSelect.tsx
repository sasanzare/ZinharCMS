import { Languages } from "lucide-react";

import { localeDefinitions, type Locale } from "./locales";
import { useI18n } from "./useI18n";

type LanguageSelectProps = {
  compact?: boolean;
};

export function LanguageSelect({ compact = false }: LanguageSelectProps) {
  const { locale, setLocale, t } = useI18n();

  return (
    <label className={compact ? "language-select language-select--compact" : "language-select"}>
      <Languages size={16} aria-hidden="true" />
      <span>{t("language.label")}</span>
      <select value={locale} onChange={(event) => setLocale(event.target.value as Locale)} aria-label={t("language.label")}>
        {localeDefinitions.map((definition) => (
          <option key={definition.code} value={definition.code}>
            {definition.nativeName}
          </option>
        ))}
      </select>
    </label>
  );
}