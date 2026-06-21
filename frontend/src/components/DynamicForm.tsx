import { useI18n } from "../i18n";
import type { FieldSchema, JsonRecord, JsonValue } from "../types/api";

type DynamicFormProps = {
  fields: FieldSchema[];
  value: JsonRecord;
  onChange: (value: JsonRecord) => void;
};

function toInputValue(value: JsonValue | undefined) {
  if (value === undefined || value === null) return "";
  return typeof value === "object" ? JSON.stringify(value, null, 2) : String(value);
}


function parseFieldValue(field: FieldSchema, raw: string | boolean): JsonValue {
  if (field.type === "boolean") return Boolean(raw);
  if (typeof raw !== "string") return raw;
  if (field.type === "number") return raw.trim() ? Number(raw) : null;
  return raw;
}

export function DynamicForm({ fields, value, onChange }: DynamicFormProps) {
  const { t } = useI18n();
  function updateField(field: FieldSchema, raw: string | boolean) {
    onChange({ ...value, [field.name]: parseFieldValue(field, raw) });
  }

  if (fields.length === 0) {
    return <p className="empty-copy">{t("dynamicForm.empty")}</p>;
  }

  return (
    <div className="dynamic-form">
      {fields.map((field) => {
        const label = field.label || field.name;
        const current = value[field.name];

        if (field.type === "boolean") {
          return (
            <label className="checkbox-row" key={field.name}>
              <input
                type="checkbox"
                checked={Boolean(current)}
                onChange={(event) => updateField(field, event.target.checked)}
              />
              <span>{label}</span>
            </label>
          );
        }

        if (field.type === "longtext" || field.type === "richtext") {
          return (
            <label key={field.name}>
              {label}
              <textarea
                rows={4}
                value={toInputValue(current)}
                onChange={(event) => updateField(field, event.target.value)}
                required={field.required}
              />
            </label>
          );
        }

        return (
          <label key={field.name}>
            {label}
            <input
              type={field.type === "number" ? "number" : field.type === "datetime" ? "datetime-local" : "text"}
              value={toInputValue(current)}
              onChange={(event) => updateField(field, event.target.value)}
              required={field.required}
            />
          </label>
        );
      })}
    </div>
  );
}