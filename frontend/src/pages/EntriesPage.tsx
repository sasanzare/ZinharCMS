import { FormEvent, useEffect, useState } from "react";
import { Edit3, FilePlus2, Filter, Send, Trash2 } from "lucide-react";

import { DynamicForm } from "../components/DynamicForm";
import { StatusBadge } from "../components/StatusBadge";
import { useI18n, workflowActionKey, workflowStatusKey } from "../i18n";
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

export function EntriesPage() {
  const { t } = useI18n();
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
        setError(caught instanceof ApiError ? caught.message : t("entries.error.loadTypes"));
      }
    }

    void loadTypes();
  }, [t]);

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
        setError(caught instanceof ApiError ? caught.message : t("entries.error.load"));
      } finally {
        setLoading(false);
      }
    }

    void loadEntries();
  }, [selectedSlug, status, t]);

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
      setError(caught instanceof ApiError ? caught.message : t("entries.error.save"));
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
      setError(caught instanceof ApiError ? caught.message : t("entries.error.status"));
    }
  }

  async function deleteEntry(entry: ContentEntryResponse) {
    if (!selectedSlug || !window.confirm(t("entries.confirmDelete", { title: titleForEntry(entry) }))) return;
    setError(null);
    try {
      await api.entries.delete(selectedSlug, entry.id);
      await reloadEntries();
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : t("entries.error.delete"));
    }
  }

  return (
    <div className="page-stack two-column-workspace">
      <section className="panel editor-panel">
        <div className="panel-header">
          <div>
            <h2>{editingId ? t("entries.editor.edit") : t("entries.editor.new")}</h2>
            <span>{selectedType ? selectedType.name : t("entries.createTypeFirst")}</span>
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
            {t("common.new")}
          </button>
        </div>

        <form className="form-grid padded" onSubmit={handleSubmit}>
          <label>
            {t("entries.contentType")}
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
            {saving ? t("common.saving") : t("entries.saveEntry")}
          </button>
        </form>
      </section>

      <section className="panel list-panel">
        <div className="panel-header">
          <div>
            <h2>{t("entries.list.title")}</h2>
            <span>{loading ? t("common.loading") : t("entries.recordsCount", { count: entries.length })}</span>
          </div>
          <label className="filter-select">
            <Filter size={16} aria-hidden="true" />
            <select value={status} onChange={(event) => setStatus(event.target.value)}>
              <option value="">{t("common.all")}</option>
              <option value="draft">{t("common.draft")}</option>
              <option value="pending_review">{t("common.pending")}</option>
              <option value="published">{t("common.published")}</option>
              <option value="archived">{t("common.archived")}</option>
            </select>
          </label>
        </div>

        <table className="data-table">
          <thead>
            <tr>
              <th>{t("common.title")}</th>
              <th>{t("common.status")}</th>
              <th>{t("common.version")}</th>
              <th>{t("common.updated")}</th>
              <th>{t("common.actions")}</th>
            </tr>
          </thead>
          <tbody>
            {entries.map((entry) => (
              <tr key={entry.id}>
                <td>{titleForEntry(entry)}</td>
                <td>
                  <StatusBadge label={t(workflowStatusKey(entry.status))} tone={statusTone(entry.status)} />
                </td>
                <td>{entry.version}</td>
                <td>{new Date(entry.updated_at).toLocaleString()}</td>
                <td>
                  <div className="table-actions">
                    <button className="icon-button" type="button" onClick={() => editEntry(entry)} aria-label={t("entries.edit")}>
                      <Edit3 size={16} aria-hidden="true" />
                    </button>
                    <button className="secondary-button" type="button" onClick={() => void transitionEntry(entry)}>
                      {t(workflowActionKey(entry.status))}
                    </button>
                    <button className="icon-button" type="button" onClick={() => void deleteEntry(entry)} aria-label={t("entries.delete")}>
                      <Trash2 size={16} aria-hidden="true" />
                    </button>
                  </div>
                </td>
              </tr>
            ))}
            {!loading && entries.length === 0 && (
              <tr>
                <td colSpan={5}>{t("entries.empty")}</td>
              </tr>
            )}
          </tbody>
        </table>
      </section>
    </div>
  );
}