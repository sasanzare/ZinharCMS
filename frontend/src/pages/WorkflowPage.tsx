import { FormEvent, useCallback, useEffect, useMemo, useState } from "react";
import { Check, MessageSquare, PlugZap, RefreshCw, Send, X } from "lucide-react";

import { StatusBadge } from "../components/StatusBadge";
import { useI18n, workflowStatusKey } from "../i18n";
import { ApiError, api } from "../services/api";
import type {
  CommentResponse,
  ContentEntryResponse,
  ContentTypeResponse,
  PageResponse,
  PluginResponse,
  WorkflowStatus,
} from "../types/api";

type ReviewItem = {
  key: string;
  kind: "entry" | "page";
  id: string;
  title: string;
  subtitle: string;
  status: WorkflowStatus;
  updated_at: string;
  typeSlug?: string;
};

function apiMessage(caught: unknown, fallback: string) {
  return caught instanceof ApiError ? caught.message : fallback;
}

function statusTone(status: WorkflowStatus) {
  if (status === "published") return "success";
  if (status === "pending_review") return "warning";
  if (status === "archived") return "danger";
  return "neutral";
}

function titleForEntry(entry: ContentEntryResponse) {
  const title = entry.data.title ?? entry.data.name ?? entry.data.slug ?? entry.id;
  return typeof title === "string" ? title : entry.id;
}

function entryReviewItems(type: ContentTypeResponse, entries: ContentEntryResponse[]): ReviewItem[] {
  return entries.map((entry) => ({
    key: `entry:${entry.id}`,
    kind: "entry",
    id: entry.id,
    title: titleForEntry(entry),
    subtitle: type.name,
    status: entry.status,
    updated_at: entry.updated_at,
    typeSlug: type.slug,
  }));
}

function pageReviewItems(pages: PageResponse[]): ReviewItem[] {
  return pages.map((page) => ({
    key: `page:${page.id}`,
    kind: "page",
    id: page.id,
    title: page.title,
    subtitle: `/${page.slug}`,
    status: page.status,
    updated_at: page.updated_at,
  }));
}

export function WorkflowPage() {
  const { t } = useI18n();
  const [items, setItems] = useState<ReviewItem[]>([]);
  const [selectedKey, setSelectedKey] = useState<string | null>(null);
  const [comments, setComments] = useState<CommentResponse[]>([]);
  const [commentBody, setCommentBody] = useState("");
  const [includeResolved, setIncludeResolved] = useState(false);
  const [plugins, setPlugins] = useState<PluginResponse[]>([]);
  const [loading, setLoading] = useState(true);
  const [commentLoading, setCommentLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [message, setMessage] = useState<string | null>(null);

  const selectedItem = useMemo(() => items.find((item) => item.key === selectedKey) ?? null, [items, selectedKey]);

  const loadWorkflow = useCallback(async function loadWorkflow() {
    setLoading(true);
    setError(null);
    try {
      const [types, pages, pluginRows] = await Promise.all([
        api.contentTypes.list(),
        api.pages.list({ status: "pending_review", sort: "updated_at:desc" }),
        api.plugins.list(),
      ]);
      const entryGroups = await Promise.all(
        types.map(async (type) => {
          const response = await api.entries.list(type.slug, { status: "pending_review", sort: "updated_at:desc" });
          return entryReviewItems(type, response.data);
        }),
      );
      const nextItems = [...entryGroups.flat(), ...pageReviewItems(pages.data)].sort(
        (a, b) => new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime(),
      );
      setItems(nextItems);
      setPlugins(pluginRows);
      setSelectedKey((current) => (current && nextItems.some((item) => item.key === current) ? current : nextItems[0]?.key ?? null));
    } catch (caught) {
      setError(apiMessage(caught, t("workflow.error.load")));
    } finally {
      setLoading(false);
    }
  }, [t]);

  const loadComments = useCallback(async (item = selectedItem) => {
    if (!item) {
      setComments([]);
      return;
    }
    setCommentLoading(true);
    try {
      setComments(await api.comments.list(item.kind, item.id, includeResolved));
    } catch (caught) {
      setError(apiMessage(caught, t("workflow.error.comments")));
    } finally {
      setCommentLoading(false);
    }
  }, [includeResolved, selectedItem, t]);

  useEffect(() => {
    void loadWorkflow();
  }, [loadWorkflow]);

  useEffect(() => {
    void loadComments();
  }, [loadComments]);

  async function approve(item: ReviewItem) {
    setError(null);
    setMessage(null);
    try {
      if (item.kind === "entry" && item.typeSlug) {
        await api.entries.publish(item.typeSlug, item.id);
      } else {
        await api.pages.publish(item.id);
      }
      setMessage(t("workflow.itemPublished", { title: item.title }));
      await loadWorkflow();
    } catch (caught) {
      setError(apiMessage(caught, t("workflow.error.publish")));
    }
  }

  async function reject(item: ReviewItem) {
    setError(null);
    setMessage(null);
    try {
      if (item.kind === "entry" && item.typeSlug) {
        await api.entries.reject(item.typeSlug, item.id);
      } else {
        await api.pages.reject(item.id);
      }
      setMessage(t("workflow.itemRejected", { title: item.title }));
      await loadWorkflow();
    } catch (caught) {
      setError(apiMessage(caught, t("workflow.error.reject")));
    }
  }

  async function createComment(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!selectedItem || !commentBody.trim()) return;
    setError(null);
    try {
      await api.comments.create({
        entity_type: selectedItem.kind,
        entity_id: selectedItem.id,
        body: commentBody.trim(),
      });
      setCommentBody("");
      await loadComments(selectedItem);
    } catch (caught) {
      setError(apiMessage(caught, t("workflow.error.addComment")));
    }
  }

  async function toggleResolved(comment: CommentResponse) {
    setError(null);
    try {
      if (comment.resolved_at) {
        await api.comments.unresolve(comment.id);
      } else {
        await api.comments.resolve(comment.id);
      }
      await loadComments();
    } catch (caught) {
      setError(apiMessage(caught, t("workflow.error.updateComment")));
    }
  }

  async function togglePlugin(plugin: PluginResponse) {
    setError(null);
    setMessage(null);
    try {
      const updated = plugin.is_enabled ? await api.plugins.disable(plugin.plugin_key) : await api.plugins.enable(plugin.plugin_key);
      setPlugins((current) => current.map((item) => (item.id === updated.id ? updated : item)));
      setMessage(t("workflow.pluginUpdated", { name: updated.name, status: t(updated.is_enabled ? "common.enabled" : "common.disabled") }));
    } catch (caught) {
      setError(apiMessage(caught, t("workflow.error.updatePlugin")));
    }
  }

  return (
    <div className="page-stack workflow-grid">
      <section className="panel workflow-review-panel">
        <div className="panel-header">
          <div>
            <h2>{t("workflow.reviewQueue")}</h2>
            <span>{loading ? t("common.loading") : t("workflow.pendingItems", { count: items.length })}</span>
          </div>
          <button className="icon-button" type="button" onClick={() => void loadWorkflow()} aria-label={t("workflow.refresh")}>
            <RefreshCw size={16} aria-hidden="true" />
          </button>
        </div>

        <div className="workflow-status-row">
          {error && <StatusBadge label={error} tone="danger" />}
          {message && <StatusBadge label={message} tone="success" />}
        </div>

        <div className="review-list">
          {items.map((item) => (
            <button
              className={`review-row ${item.key === selectedKey ? "review-row--selected" : ""}`}
              key={item.key}
              type="button"
              onClick={() => setSelectedKey(item.key)}
            >
              <span className="review-main">
                <strong>{item.title}</strong>
                <span>{item.subtitle}</span>
              </span>
              <span className="review-meta">
                <StatusBadge label={t(workflowStatusKey(item.status))} tone={statusTone(item.status)} />
                <small>{new Date(item.updated_at).toLocaleString()}</small>
              </span>
            </button>
          ))}
          {!loading && items.length === 0 && <p className="empty-copy padded">{t("workflow.noItems")}</p>}
        </div>
      </section>

      <section className="panel workflow-detail-panel">
        <div className="panel-header">
          <div>
            <h2>{selectedItem?.title ?? t("workflow.noSelected")}</h2>
            <span>{selectedItem ? t("workflow.controls", { kind: selectedItem.kind }) : t("workflow.selectPending")}</span>
          </div>
          <div className="panel-actions">
            <button className="secondary-button" type="button" onClick={() => selectedItem && void reject(selectedItem)} disabled={!selectedItem}>
              <X size={16} aria-hidden="true" />
              Reject
            </button>
            <button className="primary-button" type="button" onClick={() => selectedItem && void approve(selectedItem)} disabled={!selectedItem}>
              <Check size={16} aria-hidden="true" />
              Publish
            </button>
          </div>
        </div>

        <div className="comments-toolbar">
          <div>
            <MessageSquare size={16} aria-hidden="true" />
            <strong>{t("workflow.comments")}</strong>
            <span>{commentLoading ? "Loading" : comments.length}</span>
          </div>
          <label className="checkbox-row compact-checkbox">
            <input type="checkbox" checked={includeResolved} onChange={(event) => setIncludeResolved(event.target.checked)} />
            {t("workflow.resolved")}
          </label>
        </div>

        <form className="comment-composer" onSubmit={createComment}>
          <textarea
            disabled={!selectedItem}
            onChange={(event) => setCommentBody(event.target.value)}
            placeholder={t("workflow.addFeedback")}
            rows={3}
            value={commentBody}
          />
          <button className="primary-button" type="submit" disabled={!selectedItem || !commentBody.trim()}>
            <Send size={16} aria-hidden="true" />
            Comment
          </button>
        </form>

        <div className="comment-list">
          {comments.map((comment) => (
            <div className="comment-item" key={comment.id}>
              <div>
                <strong>{comment.author_name ?? t("workflow.teamMember")}</strong>
                <span>{new Date(comment.created_at).toLocaleString()}</span>
              </div>
              <p>{comment.body}</p>
              <button className="secondary-button" type="button" onClick={() => void toggleResolved(comment)}>
                {comment.resolved_at ? t("common.reopen") : t("common.resolve")}
              </button>
            </div>
          ))}
          {!commentLoading && selectedItem && comments.length === 0 && <p className="empty-copy">{t("workflow.noComments")}</p>}
        </div>
      </section>

      <section className="panel workflow-plugin-panel">
        <div className="panel-header">
          <div>
            <h2>{t("workflow.plugins")}</h2>
            <span>{t("workflow.pluginsRegistered", { count: plugins.length })}</span>
          </div>
          <PlugZap size={18} aria-hidden="true" />
        </div>
        <div className="plugin-list">
          {plugins.map((plugin) => (
            <div className="plugin-row" key={plugin.id}>
              <div className="plugin-main">
                <strong>{plugin.name}</strong>
                <span>{plugin.description}</span>
                <small>{plugin.hooks.join(", ") || t("common.noHooks")}</small>
              </div>
              <StatusBadge label={plugin.is_enabled ? t("common.enabled") : t("common.disabled")} tone={plugin.is_enabled ? "success" : "neutral"} />
              <button className="secondary-button" type="button" onClick={() => void togglePlugin(plugin)}>
                {plugin.is_enabled ? t("common.disable") : t("common.enable")}
              </button>
            </div>
          ))}
          {!loading && plugins.length === 0 && <p className="empty-copy padded">{t("workflow.noPlugins")}</p>}
        </div>
      </section>
    </div>
  );
}
