import { FormEvent, useEffect, useState } from "react";
import { Copy, ImagePlus, Save, Search, Trash2, Upload } from "lucide-react";

import { StatusBadge } from "../components/StatusBadge";
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
  const [items, setItems] = useState<MediaResponse[]>([]);
  const [selected, setSelected] = useState<MediaResponse | null>(null);
  const [file, setFile] = useState<File | null>(null);
  const [altText, setAltText] = useState("");
  const [caption, setCaption] = useState("");
  const [query, setQuery] = useState("");
  const [loading, setLoading] = useState(true);
  const [working, setWorking] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function load() {
    setLoading(true);
    setError(null);
    try {
      const response = await api.media.list();
      setItems(response.data);
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : "Failed to load media");
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    void load();
  }, []);

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
      setAltText("");
      setCaption("");
      setSelected(detail.media);
      await load();
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : "Failed to upload media");
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
      setError(caught instanceof ApiError ? caught.message : "Failed to update media");
    } finally {
      setWorking(false);
    }
  }

  async function handleDelete(item: MediaResponse) {
    if (!window.confirm(`Delete ${item.filename}?`)) return;
    setError(null);
    try {
      await api.media.delete(item.id);
      if (selected?.id === item.id) setSelected(null);
      await load();
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : "Failed to delete media");
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
            <h2>Upload media</h2>
            <span>Images generate WebP variants on the backend.</span>
          </div>
        </div>

        <form className="form-grid padded" onSubmit={handleUpload}>
          <label>
            File
            <input type="file" onChange={(event) => setFile(event.target.files?.[0] ?? null)} />
          </label>
          <label>
            Alt text
            <input value={altText} onChange={(event) => setAltText(event.target.value)} />
          </label>
          <label>
            Caption
            <textarea rows={3} value={caption} onChange={(event) => setCaption(event.target.value)} />
          </label>
          {error && <StatusBadge label={error} tone="danger" />}
          <button className="primary-button" type="submit" disabled={working || !file}>
            <Upload size={16} aria-hidden="true" />
            {working ? "Working..." : "Upload"}
          </button>
        </form>

        {selected && (
          <div className="metadata-editor">
            <div className="section-title-row">
              <strong>Selected asset</strong>
              <StatusBadge label={selected.mime_type} tone="neutral" />
            </div>
            <label>
              Alt text
              <input value={altText} onChange={(event) => setAltText(event.target.value)} />
            </label>
            <label>
              Caption
              <textarea rows={3} value={caption} onChange={(event) => setCaption(event.target.value)} />
            </label>
            <button className="secondary-button" type="button" onClick={() => void handleUpdate()} disabled={working}>
              <Save size={16} aria-hidden="true" />
              Save metadata
            </button>
          </div>
        )}
      </section>

      <section className="panel list-panel">
        <div className="panel-header">
          <div>
            <h2>Media library</h2>
            <span>{loading ? "Loading" : `${filtered.length} assets`}</span>
          </div>
          <label className="search-field compact-search">
            <Search size={16} aria-hidden="true" />
            <input type="search" placeholder="Search" value={query} onChange={(event) => setQuery(event.target.value)} />
          </label>
        </div>

        <div className="media-grid padded" aria-label="Media library">
          {filtered.map((item) => (
            <article className={`media-tile ${selected?.id === item.id ? "media-tile--selected" : ""}`} key={item.id}>
              <button className="media-preview" type="button" onClick={() => selectItem(item)}>
                {item.mime_type.startsWith("image/") ? <img src={mediaUrl(item.url)} alt={item.alt_text ?? item.filename} /> : <ImagePlus size={22} aria-hidden="true" />}
              </button>
              <strong>{item.filename}</strong>
              <span>{item.mime_type}</span>
              <small>{formatSize(item.size)}</small>
              <div className="table-actions">
                <button className="icon-button" type="button" onClick={() => void copyUrl(item)} aria-label="Copy media URL">
                  <Copy size={16} aria-hidden="true" />
                </button>
                <button className="icon-button" type="button" onClick={() => void handleDelete(item)} aria-label="Delete media">
                  <Trash2 size={16} aria-hidden="true" />
                </button>
              </div>
            </article>
          ))}
          {!loading && filtered.length === 0 && <p className="empty-copy">No media found.</p>}
        </div>
      </section>
    </div>
  );
}