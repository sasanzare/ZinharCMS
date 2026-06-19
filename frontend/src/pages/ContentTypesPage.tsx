import { FormEvent, useEffect, useState } from "react";
import { Edit3, Plus, Save, Search, Trash2, X } from "lucide-react";

import { StatusBadge } from "../components/StatusBadge";
import { ApiError, api } from "../services/api";
import type { ContentTypeResponse, FieldSchema, FieldType } from "../types/api";

const fieldTypes: FieldType[] = [
  "text",
  "longtext",
  "richtext",
  "number",
  "boolean",
  "datetime",
  "media",
];

const emptyField: FieldSchema = {
  id: "title",
  name: "title",
  label: "Title",
  type: "text",
  required: true,
};

function createEmptyDraft() {
  return {
    id: null as string | null,
    name: "Article",
    slug: "article",
    fields: [emptyField],
  };
}

export function ContentTypesPage() {
  const [rows, setRows] = useState<ContentTypeResponse[]>([]);
  const [draft, setDraft] = useState(createEmptyDraft);
  const [query, setQuery] = useState("");
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function load() {
    setLoading(true);
    setError(null);
    try {
      setRows(await api.contentTypes.list());
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : "Failed to load content types");
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    void load();
  }, []);

  const filtered = rows.filter((row) => `${row.name} ${row.slug}`.toLowerCase().includes(query.toLowerCase()));

  function updateField(index: number, patch: Partial<FieldSchema>) {
    setDraft((current) => ({
      ...current,
      fields: current.fields.map((field, fieldIndex) => {
        if (fieldIndex !== index) return field;
        const next = { ...field, ...patch };
        return { ...next, id: next.id || next.name };
      }),
    }));
  }

  function editRow(row: ContentTypeResponse) {
    setDraft({
      id: row.id,
      name: row.name,
      slug: row.slug,
      fields: row.fields.fields.length ? row.fields.fields : [emptyField],
    });
  }

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setSaving(true);
    setError(null);
    const fields = draft.fields.map((field) => ({ ...field, id: field.id || field.name }));
    try {
      if (draft.id) {
        await api.contentTypes.update(draft.id, { name: draft.name, slug: draft.slug, fields: { fields } });
      } else {
        await api.contentTypes.create({ name: draft.name, slug: draft.slug, fields: { fields } });
      }
      setDraft(createEmptyDraft());
      await load();
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : "Failed to save content type");
    } finally {
      setSaving(false);
    }
  }

  async function handleDelete(row: ContentTypeResponse) {
    if (!window.confirm(`Delete content type ${row.name}? Entries for this type will be removed.`)) return;
    setError(null);
    try {
      await api.contentTypes.delete(row.id);
      await load();
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : "Failed to delete content type");
    }
  }

  return (
    <div className="page-stack two-column-workspace content-types-workspace">
      <section className="panel editor-panel">
        <div className="panel-header">
          <div>
            <h2>{draft.id ? "Edit content type" : "New content type"}</h2>
            <span>Define the schema used by dynamic entry forms.</span>
          </div>
          {draft.id && (
            <button className="icon-button" type="button" onClick={() => setDraft(createEmptyDraft())} aria-label="Cancel edit">
              <X size={16} aria-hidden="true" />
            </button>
          )}
        </div>

        <form className="form-grid padded" onSubmit={handleSubmit}>
          <label>
            Name
            <input value={draft.name} onChange={(event) => setDraft({ ...draft, name: event.target.value })} required />
          </label>
          <label>
            Slug
            <input value={draft.slug} onChange={(event) => setDraft({ ...draft, slug: event.target.value })} required />
          </label>

          <div className="field-builder">
            <div className="section-title-row">
              <strong>Fields</strong>
              <button
                className="secondary-button"
                type="button"
                onClick={() => setDraft((current) => ({ ...current, fields: [...current.fields, { ...emptyField, name: "field" }] }))}
              >
                <Plus size={16} aria-hidden="true" />
                Field
              </button>
            </div>

            {draft.fields.map((field, index) => (
              <div className="field-row" key={`${field.name}-${index}`}>
                <input
                  aria-label="Field name"
                  value={field.name}
                  onChange={(event) => updateField(index, { name: event.target.value, id: event.target.value })}
                  placeholder="name"
                />
                <input
                  aria-label="Field label"
                  value={field.label ?? ""}
                  onChange={(event) => updateField(index, { label: event.target.value })}
                  placeholder="Label"
                />
                <select
                  aria-label="Field type"
                  value={field.type}
                  onChange={(event) => updateField(index, { type: event.target.value as FieldType })}
                >
                  {fieldTypes.map((type) => (
                    <option key={type} value={type}>
                      {type}
                    </option>
                  ))}
                </select>
                <label className="checkbox-row compact-checkbox">
                  <input
                    type="checkbox"
                    checked={Boolean(field.required)}
                    onChange={(event) => updateField(index, { required: event.target.checked })}
                  />
                  <span>Required</span>
                </label>
                <button
                  className="icon-button"
                  type="button"
                  onClick={() => setDraft((current) => ({ ...current, fields: current.fields.filter((_, fieldIndex) => fieldIndex !== index) }))}
                  aria-label="Remove field"
                >
                  <Trash2 size={16} aria-hidden="true" />
                </button>
              </div>
            ))}
          </div>

          {error && <StatusBadge label={error} tone="danger" />}
          <button className="primary-button" type="submit" disabled={saving}>
            <Save size={16} aria-hidden="true" />
            {saving ? "Saving..." : "Save schema"}
          </button>
        </form>
      </section>

      <section className="panel list-panel">
        <div className="panel-header">
          <div>
            <h2>Content types</h2>
            <span>{loading ? "Loading" : `${filtered.length} models`}</span>
          </div>
          <label className="search-field compact-search">
            <Search size={16} aria-hidden="true" />
            <input type="search" placeholder="Search" value={query} onChange={(event) => setQuery(event.target.value)} />
          </label>
        </div>

        <table className="data-table">
          <thead>
            <tr>
              <th>Name</th>
              <th>Slug</th>
              <th>Fields</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {filtered.map((row) => (
              <tr key={row.id}>
                <td>{row.name}</td>
                <td>{row.slug}</td>
                <td>{row.fields.fields.length}</td>
                <td>
                  <div className="table-actions">
                    <button className="icon-button" type="button" onClick={() => editRow(row)} aria-label={`Edit ${row.name}`}>
                      <Edit3 size={16} aria-hidden="true" />
                    </button>
                    <button className="icon-button" type="button" onClick={() => void handleDelete(row)} aria-label={`Delete ${row.name}`}>
                      <Trash2 size={16} aria-hidden="true" />
                    </button>
                  </div>
                </td>
              </tr>
            ))}
            {!loading && filtered.length === 0 && (
              <tr>
                <td colSpan={4}>No content types found.</td>
              </tr>
            )}
          </tbody>
        </table>
      </section>
    </div>
  );
}
