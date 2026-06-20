import { FormEvent, useEffect, useState } from "react";
import { Edit3, FilePlus2, Filter, Send, Trash2 } from "lucide-react";

import { DynamicForm } from "../components/DynamicForm";
import { StatusBadge } from "../components/StatusBadge";
import { ApiError, api } from "../services/api";
import type { ContentEntryResponse, ContentTypeResponse, JsonRecord } from "../types/api";

function emptyData(contentType: ContentTypeResponse | null): JsonRecord {
  if (!contentType) return {};
  return Object.fromEntries(
    contentType.fields.fields.map((field) => {
      if (field.type === "boolean") return [field.name, false];
      if (field.type === "number") return [field.name, null];
      return [field.name, ""];
    }),
  );
}

function titleForEntry(entry: ContentEntryResponse) {
  const title = entry.data.title ?? entry.data.name ?? entry.data.slug ?? entry.id;
  return typeof title === "string" ? title : entry.id;
}

function statusTone(status: ContentEntryResponse["status"]) {
  if (status === "published") return "success";
  if (status === "pending_review") return "warning";
  if (status === "archived") return "danger";
  return "neutral";
}

function workflowActionLabel(status: ContentEntryResponse["status"]) {
  if (status === "draft") return "Submit";
  if (status === "pending_review") return "Publish";
  if (status === "published") return "Archive";
  return "Restore";
}

export function EntriesPage() {
  const [contentTypes, setContentTypes] = useState<ContentTypeResponse[]>([]);
  const [selectedSlug, setSelectedSlug] = useState("");
  const [entries, setEntries] = useState<ContentEntryResponse[]>([]);
  const [status, setStatus] = useState("");
  const [editingId, setEditingId] = useState<string | null>(null);
  const [entryData, setEntryData] = useState<JsonRecord>({});
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const selectedType = contentTypes.find((type) => type.slug === selectedSlug) ?? null;

  useEffect(() => {
    async function loadTypes() {
      setError(null);
      try {
        const types = await api.contentTypes.list();
        setContentTypes(types);
        setSelectedSlug((current) => current || types[0]?.slug || "");
      } catch (caught) {
        setError(caught instanceof ApiError ? caught.message : "Failed to load content types");
      }
    }

    void loadTypes();
  }, []);

  useEffect(() => {
    if (!selectedSlug) {
      setEntries([]);
      setLoading(false);
      return;
    }

    async function loadEntries() {
      setLoading(true);
      setError(null);
      try {
        const response = await api.entries.list(selectedSlug, { status, sort: "updated_at:desc" });
        setEntries(response.data);
      } catch (caught) {
        setError(caught instanceof ApiError ? caught.message : "Failed to load entries");
      } finally {
        setLoading(false);
      }
    }

    void loadEntries();
  }, [selectedSlug, status]);

  useEffect(() => {
    setEditingId(null);
    setEntryData(emptyData(selectedType));
  }, [selectedType]);

  async function reloadEntries() {
    if (!selectedSlug) return;
    const response = await api.entries.list(selectedSlug, { status, sort: "updated_at:desc" });
    setEntries(response.data);
  }

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!selectedSlug) return;
    setSaving(true);
    setError(null);

    try {
      if (editingId) {
        await api.entries.update(selectedSlug, editingId, entryData);
      } else {
        await api.entries.create(selectedSlug, entryData);
      }
      setEditingId(null);
      setEntryData(emptyData(selectedType));
      await reloadEntries();
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : "Failed to save entry");
    } finally {
      setSaving(false);
    }
  }

  function editEntry(entry: ContentEntryResponse) {
    setEditingId(entry.id);
    setEntryData(entry.data);
  }

  async function transitionEntry(entry: ContentEntryResponse) {
    if (!selectedSlug) return;
    setError(null);
    try {
      if (entry.status === "draft") {
        await api.entries.submitReview(selectedSlug, entry.id);
      } else if (entry.status === "pending_review") {
        await api.entries.publish(selectedSlug, entry.id);
      } else if (entry.status === "published") {
        await api.entries.archive(selectedSlug, entry.id);
      } else {
        await api.entries.restore(selectedSlug, entry.id);
      }
      await reloadEntries();
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : "Failed to change entry status");
    }
  }

  async function deleteEntry(entry: ContentEntryResponse) {
    if (!selectedSlug || !window.confirm(`Delete ${titleForEntry(entry)}?`)) return;
    setError(null);
    try {
      await api.entries.delete(selectedSlug, entry.id);
      await reloadEntries();
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : "Failed to delete entry");
    }
  }

  return (
    <div className="page-stack two-column-workspace">
      <section className="panel editor-panel">
        <div className="panel-header">
          <div>
            <h2>{editingId ? "Edit entry" : "New entry"}</h2>
            <span>{selectedType ? selectedType.name : "Create a content type first"}</span>
          </div>
          <button
            className="secondary-button"
            type="button"
            onClick={() => {
              setEditingId(null);
              setEntryData(emptyData(selectedType));
            }}
          >
            <FilePlus2 size={16} aria-hidden="true" />
            New
          </button>
        </div>

        <form className="form-grid padded" onSubmit={handleSubmit}>
          <label>
            Content type
            <select value={selectedSlug} onChange={(event) => setSelectedSlug(event.target.value)}>
              {contentTypes.map((type) => (
                <option key={type.id} value={type.slug}>
                  {type.name}
                </option>
              ))}
            </select>
          </label>

          <DynamicForm fields={selectedType?.fields.fields ?? []} value={entryData} onChange={setEntryData} />

          {error && <StatusBadge label={error} tone="danger" />}
          <button className="primary-button" type="submit" disabled={saving || !selectedType}>
            <Send size={16} aria-hidden="true" />
            {saving ? "Saving..." : "Save entry"}
          </button>
        </form>
      </section>

      <section className="panel list-panel">
        <div className="panel-header">
          <div>
            <h2>Entries</h2>
            <span>{loading ? "Loading" : `${entries.length} records`}</span>
          </div>
          <label className="filter-select">
            <Filter size={16} aria-hidden="true" />
            <select value={status} onChange={(event) => setStatus(event.target.value)}>
              <option value="">All</option>
              <option value="draft">Draft</option>
              <option value="pending_review">Pending</option>
              <option value="published">Published</option>
              <option value="archived">Archived</option>
            </select>
          </label>
        </div>

        <table className="data-table">
          <thead>
            <tr>
              <th>Title</th>
              <th>Status</th>
              <th>Version</th>
              <th>Updated</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {entries.map((entry) => (
              <tr key={entry.id}>
                <td>{titleForEntry(entry)}</td>
                <td>
                  <StatusBadge label={entry.status} tone={statusTone(entry.status)} />
                </td>
                <td>{entry.version}</td>
                <td>{new Date(entry.updated_at).toLocaleString()}</td>
                <td>
                  <div className="table-actions">
                    <button className="icon-button" type="button" onClick={() => editEntry(entry)} aria-label="Edit entry">
                      <Edit3 size={16} aria-hidden="true" />
                    </button>
                    <button className="secondary-button" type="button" onClick={() => void transitionEntry(entry)}>
                      {workflowActionLabel(entry.status)}
                    </button>
                    <button className="icon-button" type="button" onClick={() => void deleteEntry(entry)} aria-label="Delete entry">
                      <Trash2 size={16} aria-hidden="true" />
                    </button>
                  </div>
                </td>
              </tr>
            ))}
            {!loading && entries.length === 0 && (
              <tr>
                <td colSpan={5}>No entries found.</td>
              </tr>
            )}
          </tbody>
        </table>
      </section>
    </div>
  );
}