import { FormEvent, useEffect, useState } from "react";
import { Edit3, Eye, History, Plus, Save, Trash2 } from "lucide-react";

import { StatusBadge } from "../components/StatusBadge";
import { ApiError, api } from "../services/api";
import { useAppStore } from "../stores/useAppStore";
import type { ComponentRegistryResponse, PageJson, PageResponse, PageVersionResponse } from "../types/api";

const defaultPageJson: PageJson = {
  version: "1.0",
  metadata: {},
  layout: {
    id: "root",
    type: "root",
    children: [],
  },
};

function createDraft() {
  return {
    id: null as string | null,
    title: "Home",
    slug: "home",
    jsonText: JSON.stringify(defaultPageJson, null, 2),
  };
}

function countNodes(pageJson: PageJson) {
  function walk(node: PageJson["layout"]): number {
    return 1 + (node.children ?? []).reduce((total, child) => total + walk(child), 0);
  }
  return Math.max(0, walk(pageJson.layout) - 1);
}

export function PagesPage() {
  const accessToken = useAppStore((state) => state.accessToken);
  const [pages, setPages] = useState<PageResponse[]>([]);
  const [components, setComponents] = useState<ComponentRegistryResponse[]>([]);
  const [versions, setVersions] = useState<PageVersionResponse[]>([]);
  const [draft, setDraft] = useState(createDraft);
  const [selectedPage, setSelectedPage] = useState<PageResponse | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function load() {
    setLoading(true);
    setError(null);
    try {
      const [pageResponse, componentResponse] = await Promise.all([
        api.pages.list({ sort: "updated_at:desc" }),
        api.components.list(),
      ]);
      setPages(pageResponse.data);
      setComponents(componentResponse);
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : "Failed to load page builder data");
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    void load();
  }, []);

  async function loadVersions(page: PageResponse) {
    setSelectedPage(page);
    setVersions([]);
    try {
      setVersions(await api.pages.versions(page.id));
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : "Failed to load versions");
    }
  }

  function editPage(page: PageResponse) {
    setDraft({
      id: page.id,
      title: page.title,
      slug: page.slug,
      jsonText: JSON.stringify(page.page_json, null, 2),
    });
    void loadVersions(page);
  }

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setSaving(true);
    setError(null);

    try {
      const page_json = JSON.parse(draft.jsonText) as PageJson;
      if (draft.id) {
        await api.pages.update(draft.id, { title: draft.title, slug: draft.slug, page_json });
      } else {
        await api.pages.create({ title: draft.title, slug: draft.slug, page_json });
      }
      setDraft(createDraft());
      await load();
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : caught instanceof Error ? caught.message : "Failed to save page");
    } finally {
      setSaving(false);
    }
  }

  async function transitionPage(page: PageResponse) {
    setError(null);
    try {
      if (page.status === "published") {
        await api.pages.unpublish(page.id);
      } else {
        await api.pages.publish(page.id);
      }
      await load();
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : "Failed to change page status");
    }
  }

  async function deletePage(page: PageResponse) {
    if (!window.confirm(`Delete page ${page.title}?`)) return;
    setError(null);
    try {
      await api.pages.delete(page.id);
      if (selectedPage?.id === page.id) {
        setSelectedPage(null);
        setVersions([]);
      }
      await load();
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : "Failed to delete page");
    }
  }

  async function restoreVersion(version: PageVersionResponse) {
    setError(null);
    try {
      await api.pages.restore(version.page_id, version.version);
      await load();
      const page = pages.find((item) => item.id === version.page_id);
      if (page) await loadVersions(page);
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : "Failed to restore version");
    }
  }

  async function copyPreviewUrl(page: PageResponse) {
    const token = accessToken ? `?access_token=${encodeURIComponent(accessToken)}` : "";
    await navigator.clipboard.writeText(`${api.baseUrl.replace(/^http/, "ws")}/api/preview/${page.id}${token}`);
  }

  return (
    <div className="page-stack two-column-workspace">
      <section className="panel editor-panel">
        <div className="panel-header">
          <div>
            <h2>{draft.id ? "Edit page JSON" : "New page"}</h2>
            <span>Phase 3 uses a structured JSON editor; drag and drop comes in phase 4.</span>
          </div>
          <button className="secondary-button" type="button" onClick={() => setDraft(createDraft())}>
            <Plus size={16} aria-hidden="true" />
            New
          </button>
        </div>

        <form className="form-grid padded" onSubmit={handleSubmit}>
          <label>
            Title
            <input value={draft.title} onChange={(event) => setDraft({ ...draft, title: event.target.value })} required />
          </label>
          <label>
            Slug
            <input value={draft.slug} onChange={(event) => setDraft({ ...draft, slug: event.target.value })} required />
          </label>
          <label>
            Page JSON
            <textarea
              className="code-editor"
              rows={18}
              value={draft.jsonText}
              onChange={(event) => setDraft({ ...draft, jsonText: event.target.value })}
              spellCheck={false}
            />
          </label>
          {error && <StatusBadge label={error} tone="danger" />}
          <button className="primary-button" type="submit" disabled={saving}>
            <Save size={16} aria-hidden="true" />
            {saving ? "Saving..." : "Save page"}
          </button>
        </form>
      </section>

      <section className="panel list-panel">
        <div className="panel-header">
          <div>
            <h2>Pages</h2>
            <span>{loading ? "Loading" : `${pages.length} pages`}</span>
          </div>
          <StatusBadge label={`${components.length} components`} tone="neutral" />
        </div>

        <table className="data-table">
          <thead>
            <tr>
              <th>Title</th>
              <th>Status</th>
              <th>Components</th>
              <th>Updated</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {pages.map((page) => (
              <tr key={page.id}>
                <td>{page.title}</td>
                <td>
                  <StatusBadge label={page.status} tone={page.status === "published" ? "success" : "neutral"} />
                </td>
                <td>{countNodes(page.page_json)}</td>
                <td>{new Date(page.updated_at).toLocaleString()}</td>
                <td>
                  <div className="table-actions">
                    <button className="icon-button" type="button" onClick={() => editPage(page)} aria-label="Edit page">
                      <Edit3 size={16} aria-hidden="true" />
                    </button>
                    <button className="icon-button" type="button" onClick={() => void loadVersions(page)} aria-label="Show versions">
                      <History size={16} aria-hidden="true" />
                    </button>
                    <button className="icon-button" type="button" onClick={() => void copyPreviewUrl(page)} aria-label="Copy preview socket URL">
                      <Eye size={16} aria-hidden="true" />
                    </button>
                    <button className="secondary-button" type="button" onClick={() => void transitionPage(page)}>
                      {page.status === "published" ? "Unpublish" : "Publish"}
                    </button>
                    <button className="icon-button" type="button" onClick={() => void deletePage(page)} aria-label="Delete page">
                      <Trash2 size={16} aria-hidden="true" />
                    </button>
                  </div>
                </td>
              </tr>
            ))}
            {!loading && pages.length === 0 && (
              <tr>
                <td colSpan={5}>No pages found.</td>
              </tr>
            )}
          </tbody>
        </table>

        <div className="registry-strip">
          {components.slice(0, 12).map((component) => (
            <span key={component.id}>{component.component_key}</span>
          ))}
        </div>
      </section>

      {selectedPage && (
        <section className="panel full-width-panel">
          <div className="panel-header">
            <div>
              <h2>{selectedPage.title} versions</h2>
              <span>{versions.length} snapshots</span>
            </div>
          </div>
          <table className="data-table">
            <thead>
              <tr>
                <th>Version</th>
                <th>Snapshot</th>
                <th>Action</th>
              </tr>
            </thead>
            <tbody>
              {versions.map((version) => (
                <tr key={version.id}>
                  <td>{version.version}</td>
                  <td>{new Date(version.snapshot_at).toLocaleString()}</td>
                  <td>
                    <button className="secondary-button" type="button" onClick={() => void restoreVersion(version)}>
                      Restore
                    </button>
                  </td>
                </tr>
              ))}
              {versions.length === 0 && (
                <tr>
                  <td colSpan={3}>No snapshots found.</td>
                </tr>
              )}
            </tbody>
          </table>
        </section>
      )}
    </div>
  );
}