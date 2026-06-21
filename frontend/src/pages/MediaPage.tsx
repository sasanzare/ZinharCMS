import { FormEvent, useCallback, useEffect, useRef, useState } from "react";
import { Copy, ImagePlus, Save, Search, Trash2, Upload } from "lucide-react";

import { StatusBadge } from "../components/StatusBadge";
import { useI18n } from "../i18n";
import { ApiError, api } from "../services/api";
import type { MediaResponse } from "../types/api";

function formatSize(bytes: number) {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${Math.round(bytes / 1024)} KB`;
  return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
}

function mediaUrl(url: string) {
  return `${api.baseUrl}${url}`;
}

export function MediaPage() {
  const { t } = useI18n();
  const fileInputRef = useRef<HTMLInputElement | null>(null);
  const [items, setItems] = useState<MediaResponse[]>([]);
  const [selected, setSelected] = useState<MediaResponse | null>(null);
  const [file, setFile] = useState<File | null>(null);
  const [altText, setAltText] = useState("");
  const [caption, setCaption] = useState("");
  const [query, setQuery] = useState("");
  const [loading, setLoading] = useState(true);
  const [working, setWorking] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async function load() {
    setLoading(true);
    setError(null);
    try {
      const response = await api.media.list();
      setItems(response.data);
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : t("media.error.load"));
    } finally {
      setLoading(false);
    }
  }, [t]);

  useEffect(() => {
    void load();
  }, [load]);

  const filtered = items.filter((item) => `${item.filename} ${item.mime_type}`.toLowerCase().includes(query.toLowerCase()));

  function selectItem(item: MediaResponse) {
    setSelected(item);
    setAltText(item.alt_text ?? "");
    setCaption(item.caption ?? "");
  }

  async function handleUpload(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!file) return;
    setWorking(true);
    setError(null);
    try {
      const detail = await api.media.upload(file, { alt_text: altText, caption });
      setFile(null);
      if (fileInputRef.current) fileInputRef.current.value = "";
      setAltText("");
      setCaption("");
      setSelected(detail.media);
      await load();
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : t("media.error.upload"));
    } finally {
      setWorking(false);
    }
  }

  async function handleUpdate() {
    if (!selected) return;
    setWorking(true);
    setError(null);
    try {
      const detail = await api.media.update(selected.id, { alt_text: altText, caption });
      setSelected(detail.media);
      await load();
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : t("media.error.update"));
    } finally {
      setWorking(false);
    }
  }

  async function handleDelete(item: MediaResponse) {
    if (!window.confirm(t("media.confirmDelete", { filename: item.filename }))) return;
    setError(null);
    try {
      await api.media.delete(item.id);
      if (selected?.id === item.id) setSelected(null);
      await load();
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : t("media.error.delete"));
    }
  }

  async function copyUrl(item: MediaResponse) {
    await navigator.clipboard.writeText(mediaUrl(item.url));
  }

  return (
    <div className="page-stack two-column-workspace">
      <section className="panel editor-panel">
        <div className="panel-header">
          <div>
            <h2>{t("media.upload.title")}</h2>
            <span>{t("media.upload.description")}</span>
          </div>
        </div>

        <form className="form-grid padded" onSubmit={handleUpload}>
          <label>
            {t("media.file")}
            <div className="file-picker">
              <input
                ref={fileInputRef}
                className="file-picker-input"
                type="file"
                onChange={(event) => setFile(event.target.files?.[0] ?? null)}
              />
              <button className="secondary-button" type="button" onClick={() => fileInputRef.current?.click()}>
                {t("media.chooseFile")}
              </button>
              <span>{file?.name ?? t("media.noFileChosen")}</span>
            </div>
          </label>
          <label>
            {t("media.altText")}
            <input value={altText} onChange={(event) => setAltText(event.target.value)} />
          </label>
          <label>
            {t("media.caption")}
            <textarea rows={3} value={caption} onChange={(event) => setCaption(event.target.value)} />
          </label>
          {error && <StatusBadge label={error} tone="danger" />}
          <button className="primary-button" type="submit" disabled={working || !file}>
            <Upload size={16} aria-hidden="true" />
            {working ? t("auth.submit.working") : t("media.upload.submit")}
          </button>
        </form>

        {selected && (
          <div className="metadata-editor">
            <div className="section-title-row">
              <strong>{t("media.selectedAsset")}</strong>
              <StatusBadge label={selected.mime_type} tone="neutral" />
            </div>
            <label>
              {t("media.altText")}
              <input value={altText} onChange={(event) => setAltText(event.target.value)} />
            </label>
            <label>
              {t("media.caption")}
              <textarea rows={3} value={caption} onChange={(event) => setCaption(event.target.value)} />
            </label>
            <button className="secondary-button" type="button" onClick={() => void handleUpdate()} disabled={working}>
              <Save size={16} aria-hidden="true" />
              {t("media.saveMetadata")}
            </button>
          </div>
        )}
      </section>

      <section className="panel list-panel">
        <div className="panel-header">
          <div>
            <h2>{t("media.library.title")}</h2>
            <span>{loading ? t("common.loading") : t("media.assetsCount", { count: filtered.length })}</span>
          </div>
          <label className="search-field compact-search">
            <Search size={16} aria-hidden="true" />
            <input type="search" placeholder={t("common.search")} value={query} onChange={(event) => setQuery(event.target.value)} />
          </label>
        </div>

        <div className="media-grid padded" aria-label={t("media.library.aria")}>
          {filtered.map((item) => (
            <article className={`media-tile ${selected?.id === item.id ? "media-tile--selected" : ""}`} key={item.id}>
              <button className="media-preview" type="button" onClick={() => selectItem(item)}>
                {item.mime_type.startsWith("image/") ? <img src={mediaUrl(item.url)} alt={item.alt_text ?? item.filename} /> : <ImagePlus size={22} aria-hidden="true" />}
              </button>
              <strong>{item.filename}</strong>
              <span>{item.mime_type}</span>
              <small>{formatSize(item.size)}</small>
              <div className="table-actions">
                <button className="icon-button" type="button" onClick={() => void copyUrl(item)} aria-label={t("media.copyUrl")}>
                  <Copy size={16} aria-hidden="true" />
                </button>
                <button className="icon-button" type="button" onClick={() => void handleDelete(item)} aria-label={t("media.delete")}>
                  <Trash2 size={16} aria-hidden="true" />
                </button>
              </div>
            </article>
          ))}
          {!loading && filtered.length === 0 && <p className="empty-copy">{t("media.empty")}</p>}
        </div>
      </section>
    </div>
  );
}