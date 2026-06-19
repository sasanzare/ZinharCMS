import { type CSSProperties, type FormEvent, useEffect, useRef, useState } from "react";
import {
  DndContext,
  DragOverlay,
  PointerSensor,
  closestCenter,
  useDraggable,
  useDroppable,
  useSensor,
  useSensors,
} from "@dnd-kit/core";
import type { DragEndEvent, DragStartEvent } from "@dnd-kit/core";
import { SortableContext, arrayMove, useSortable, verticalListSortingStrategy } from "@dnd-kit/sortable";
import { Copy, Edit3, Eye, GripVertical, History, Layers3, Plus, Save, Trash2, X } from "lucide-react";

import { StatusBadge } from "../components/StatusBadge";
import { ApiError, api } from "../services/api";
import { useAppStore } from "../stores/useAppStore";
import type { ComponentRegistryResponse, JsonRecord, JsonValue, PageJson, PageNode, PageResponse, PageVersionResponse } from "../types/api";

const CANVAS_DROP_ID = "page-builder-canvas";

const defaultPageJson: PageJson = {
  version: "1.0",
  metadata: {},
  layout: {
    id: "root",
    type: "root",
    children: [],
  },
};

type PageDraft = {
  id: string | null;
  title: string;
  slug: string;
  pageJson: PageJson;
};

type PropDefinition = {
  name: string;
  label: string;
  type: string;
  required: boolean;
  options: string[];
  defaultValue?: JsonValue;
};

function clonePageJson(pageJson: PageJson): PageJson {
  return JSON.parse(JSON.stringify(pageJson)) as PageJson;
}

function normalizePageJson(pageJson: PageJson): PageJson {
  const next = clonePageJson(pageJson);
  return {
    ...next,
    metadata: next.metadata ?? {},
    layout: {
      id: next.layout?.id || "root",
      type: "root",
      props: next.layout?.props,
      styles: next.layout?.styles,
      children: next.layout?.children ?? [],
    },
  };
}

function createDraft(page?: PageResponse): PageDraft {
  if (page) {
    return {
      id: page.id,
      title: page.title,
      slug: page.slug,
      pageJson: normalizePageJson(page.page_json),
    };
  }

  return {
    id: null,
    title: "Home",
    slug: "home",
    pageJson: normalizePageJson(defaultPageJson),
  };
}

function countNodes(pageJson: PageJson) {
  function walk(node: PageNode): number {
    return 1 + (node.children ?? []).reduce((total, child) => total + walk(child), 0);
  }
  return Math.max(0, walk(pageJson.layout) - 1);
}

function isJsonRecord(value: JsonValue | undefined): value is JsonRecord {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function toStringArray(value: JsonValue | undefined) {
  return Array.isArray(value) ? value.filter((item): item is string => typeof item === "string") : [];
}

function getPropDefinitions(component: ComponentRegistryResponse | undefined): PropDefinition[] {
  if (!component) return [];

  return Object.entries(component.props_schema)
    .filter((entry): entry is [string, JsonRecord] => isJsonRecord(entry[1]))
    .map(([name, schema]) => ({
      name,
      label: typeof schema.label === "string" ? schema.label : name.replaceAll("_", " "),
      type: typeof schema.type === "string" ? schema.type : "text",
      required: schema.required === true,
      options: toStringArray(schema.options),
      defaultValue: schema.default,
    }));
}

function defaultValueForDefinition(definition: PropDefinition): JsonValue {
  if (definition.defaultValue !== undefined) return definition.defaultValue;
  if (definition.type === "number") return 0;
  if (definition.type === "boolean") return false;
  if (definition.type === "array") return [];
  if (definition.type === "json") return {};
  return "";
}

function propsFromDefaults(component: ComponentRegistryResponse): JsonRecord {
  return Object.fromEntries(
    getPropDefinitions(component)
      .filter((definition) => definition.required || definition.defaultValue !== undefined)
      .map((definition) => [definition.name, defaultValueForDefinition(definition)]),
  );
}

function createNode(component: ComponentRegistryResponse): PageNode {
  const id = `${component.component_key}-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 7)}`;
  return {
    id,
    type: component.component_key,
    props: propsFromDefaults(component),
    styles: {},
    children: [],
  };
}

function findNode(nodes: PageNode[], nodeId: string | null): PageNode | null {
  if (!nodeId) return null;
  for (const node of nodes) {
    if (node.id === nodeId) return node;
    const child = findNode(node.children ?? [], nodeId);
    if (child) return child;
  }
  return null;
}

function updateNode(nodes: PageNode[], nodeId: string, updater: (node: PageNode) => PageNode): PageNode[] {
  return nodes.map((node) => {
    if (node.id === nodeId) return updater(node);
    if (!node.children?.length) return node;
    return { ...node, children: updateNode(node.children, nodeId, updater) };
  });
}

function removeNode(nodes: PageNode[], nodeId: string): PageNode[] {
  return nodes
    .filter((node) => node.id !== nodeId)
    .map((node) => ({ ...node, children: node.children?.length ? removeNode(node.children, nodeId) : node.children }));
}

function toEditorText(value: JsonValue | undefined) {
  if (value === undefined || value === null) return "";
  return typeof value === "object" ? JSON.stringify(value, null, 2) : String(value);
}

function parseJsonEditorValue(raw: string): JsonValue {
  if (!raw.trim()) return null;
  try {
    return JSON.parse(raw) as JsonValue;
  } catch {
    return raw;
  }
}

function parsePropValue(definition: PropDefinition, raw: string | boolean): JsonValue {
  if (definition.type === "boolean") return Boolean(raw);
  if (typeof raw !== "string") return raw;
  if (definition.type === "number") return raw.trim() ? Number(raw) : null;
  if (definition.type === "array" || definition.type === "json") return parseJsonEditorValue(raw);
  return raw;
}

function componentForNode(components: ComponentRegistryResponse[], node: PageNode | null) {
  return node ? components.find((component) => component.component_key === node.type) : undefined;
}

function transformStyle(transform: { x: number; y: number; scaleX: number; scaleY: number } | null): string | undefined {
  if (!transform) return undefined;
  return `translate3d(${transform.x}px, ${transform.y}px, 0) scaleX(${transform.scaleX}) scaleY(${transform.scaleY})`;
}

function PaletteItem({ component, onAdd }: { component: ComponentRegistryResponse; onAdd: (component: ComponentRegistryResponse) => void }) {
  const { attributes, listeners, setNodeRef, transform, isDragging } = useDraggable({
    id: `component:${component.component_key}`,
  });
  const style: CSSProperties = {
    transform: transformStyle(transform),
    opacity: isDragging ? 0.55 : undefined,
  };

  return (
    <div className="palette-item" ref={setNodeRef} style={style} {...listeners} {...attributes}>
      <button type="button" onDoubleClick={() => onAdd(component)}>
        <Layers3 size={16} aria-hidden="true" />
        <span>
          <strong>{component.name}</strong>
          <small>{component.category}</small>
        </span>
      </button>
    </div>
  );
}

function SortableCanvasNode({
  node,
  component,
  selected,
  onSelect,
  onRemove,
}: {
  node: PageNode;
  component: ComponentRegistryResponse | undefined;
  selected: boolean;
  onSelect: (id: string) => void;
  onRemove: (id: string) => void;
}) {
  const { attributes, listeners, setNodeRef, transform, transition, isDragging } = useSortable({ id: node.id });
  const style: CSSProperties = {
    transform: transformStyle(transform),
    transition,
    opacity: isDragging ? 0.55 : undefined,
  };

  const title = component?.name ?? node.type;
  const props = node.props ?? {};
  const previewText = [props.title, props.subtitle, props.body, props.quote]
    .filter((value): value is string => typeof value === "string" && value.trim().length > 0)
    .join(" / ");

  return (
    <div className={`canvas-node ${selected ? "canvas-node--selected" : ""}`} ref={setNodeRef} style={style}>
      <button className="drag-handle" type="button" aria-label={`Move ${title}`} {...listeners} {...attributes}>
        <GripVertical size={16} aria-hidden="true" />
      </button>
      <button className="canvas-node-main" type="button" onClick={() => onSelect(node.id)}>
        <strong>{title}</strong>
        <span>{previewText || node.type}</span>
      </button>
      <button className="icon-button" type="button" onClick={() => onRemove(node.id)} aria-label={`Remove ${title}`}>
        <X size={16} aria-hidden="true" />
      </button>
    </div>
  );
}

function BuilderCanvas({
  pageJson,
  components,
  selectedNodeId,
  onSelect,
  onRemove,
}: {
  pageJson: PageJson;
  components: ComponentRegistryResponse[];
  selectedNodeId: string | null;
  onSelect: (id: string) => void;
  onRemove: (id: string) => void;
}) {
  const { setNodeRef, isOver } = useDroppable({ id: CANVAS_DROP_ID });
  const children = pageJson.layout.children ?? [];

  return (
    <div className={`builder-canvas ${isOver ? "builder-canvas--over" : ""}`} ref={setNodeRef}>
      <div className="canvas-chrome">
        <span>Desktop canvas</span>
        <strong>{children.length} blocks</strong>
      </div>
      {children.length === 0 ? (
        <div className="drop-empty">
          <Layers3 size={22} aria-hidden="true" />
          <strong>Drop components here</strong>
          <span>Drag from the component panel or double-click an item to add it.</span>
        </div>
      ) : (
        <SortableContext items={children.map((node) => node.id)} strategy={verticalListSortingStrategy}>
          <div className="canvas-node-list">
            {children.map((node) => (
              <SortableCanvasNode
                component={componentForNode(components, node)}
                key={node.id}
                node={node}
                onRemove={onRemove}
                onSelect={onSelect}
                selected={selectedNodeId === node.id}
              />
            ))}
          </div>
        </SortableContext>
      )}
    </div>
  );
}

function PreviewNode({ node, component }: { node: PageNode; component: ComponentRegistryResponse | undefined }) {
  const props = node.props ?? {};
  const title = typeof props.title === "string" && props.title ? props.title : component?.name ?? node.type;
  const subtitle = typeof props.subtitle === "string" ? props.subtitle : undefined;
  const body = typeof props.body === "string" ? props.body : undefined;
  const quote = typeof props.quote === "string" ? props.quote : undefined;

  if (node.type === "divider") return <hr className="preview-divider" />;
  if (node.type === "spacer") return <div className="preview-spacer" style={{ height: typeof props.height === "number" ? props.height : 32 }} />;

  return (
    <article className={`preview-block preview-block--${component?.category ?? "content"}`}>
      <small>{component?.name ?? node.type}</small>
      <h3>{title}</h3>
      {subtitle && <p>{subtitle}</p>}
      {body && <p>{body}</p>}
      {quote && <blockquote>{quote}</blockquote>}
    </article>
  );
}

function LivePreview({ pageJson, components }: { pageJson: PageJson; components: ComponentRegistryResponse[] }) {
  const children = pageJson.layout.children ?? [];

  return (
    <div className="live-preview">
      <div className="preview-page">
        {children.length === 0 ? (
          <div className="preview-empty">Preview updates as components are added.</div>
        ) : (
          children.map((node) => <PreviewNode component={componentForNode(components, node)} key={node.id} node={node} />)
        )}
      </div>
    </div>
  );
}

function PropControl({
  definition,
  value,
  onChange,
}: {
  definition: PropDefinition;
  value: JsonValue | undefined;
  onChange: (value: JsonValue) => void;
}) {
  if (definition.type === "boolean") {
    return (
      <label className="checkbox-row">
        <input type="checkbox" checked={Boolean(value)} onChange={(event) => onChange(event.target.checked)} />
        <span>{definition.label}</span>
      </label>
    );
  }

  if (definition.type === "select") {
    return (
      <label>
        {definition.label}
        <select value={typeof value === "string" ? value : ""} onChange={(event) => onChange(event.target.value)} required={definition.required}>
          <option value="">Select</option>
          {definition.options.map((option) => (
            <option key={option} value={option}>
              {option}
            </option>
          ))}
        </select>
      </label>
    );
  }

  if (definition.type === "array" || definition.type === "json" || definition.type === "richtext") {
    return (
      <label>
        {definition.label}
        <textarea
          className={definition.type === "array" || definition.type === "json" ? "code-editor" : undefined}
          rows={definition.type === "richtext" ? 5 : 6}
          value={toEditorText(value)}
          onChange={(event) => onChange(parsePropValue(definition, event.target.value))}
          required={definition.required}
          spellCheck={definition.type === "richtext"}
        />
      </label>
    );
  }

  return (
    <label>
      {definition.label}
      <input
        type={definition.type === "number" ? "number" : definition.type === "email" ? "email" : "text"}
        value={toEditorText(value)}
        onChange={(event) => onChange(parsePropValue(definition, event.target.value))}
        required={definition.required}
      />
    </label>
  );
}

export function PagesPage() {
  const accessToken = useAppStore((state) => state.accessToken);
  const [pages, setPages] = useState<PageResponse[]>([]);
  const [components, setComponents] = useState<ComponentRegistryResponse[]>([]);
  const [versions, setVersions] = useState<PageVersionResponse[]>([]);
  const [draft, setDraft] = useState<PageDraft>(createDraft);
  const [selectedNodeId, setSelectedNodeId] = useState<string | null>(null);
  const [selectedPage, setSelectedPage] = useState<PageResponse | null>(null);
  const [componentQuery, setComponentQuery] = useState("");
  const [activeDragId, setActiveDragId] = useState<string | null>(null);
  const [dirty, setDirty] = useState(false);
  const [autoSaveState, setAutoSaveState] = useState("Manual save");
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const autoSaveRef = useRef<() => void>(() => undefined);

  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint: { distance: 8 },
    }),
  );

  async function load(preferredPageId?: string) {
    setLoading(true);
    setError(null);
    try {
      const [pageResponse, componentResponse] = await Promise.all([
        api.pages.list({ sort: "updated_at:desc" }),
        api.components.list(),
      ]);
      setPages(pageResponse.data);
      setComponents(componentResponse);
      if (preferredPageId) {
        const page = pageResponse.data.find((item) => item.id === preferredPageId);
        if (page) setSelectedPage(page);
      }
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : "Failed to load page builder data");
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    void load();
  }, []);

  async function saveDraftPage(options: { silent?: boolean } = {}) {
    setSaving(true);
    if (!options.silent) setError(null);
    try {
      const payload = { title: draft.title, slug: draft.slug, page_json: normalizePageJson(draft.pageJson) };
      const saved = draft.id ? await api.pages.update(draft.id, payload) : await api.pages.create(payload);
      setDraft(createDraft(saved));
      setDirty(false);
      setAutoSaveState(options.silent ? "Autosaved" : "Saved");
      await load(saved.id);
    } catch (caught) {
      setAutoSaveState("Save failed");
      setError(caught instanceof ApiError ? caught.message : caught instanceof Error ? caught.message : "Failed to save page");
    } finally {
      setSaving(false);
    }
  }

  autoSaveRef.current = () => {
    void saveDraftPage({ silent: true });
  };

  useEffect(() => {
    if (!draft.id || !dirty || saving) return;
    setAutoSaveState("Autosave pending");
    const timeout = window.setTimeout(() => {
      autoSaveRef.current();
    }, 10_000);
    return () => window.clearTimeout(timeout);
  }, [draft, dirty, saving]);

  async function loadVersions(page: PageResponse) {
    setSelectedPage(page);
    setVersions([]);
    try {
      setVersions(await api.pages.versions(page.id));
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : "Failed to load versions");
    }
  }

  function mutatePageJson(updater: (pageJson: PageJson) => PageJson) {
    setDraft((current) => ({ ...current, pageJson: normalizePageJson(updater(normalizePageJson(current.pageJson))) }));
    setDirty(true);
    setAutoSaveState(draft.id ? "Unsaved changes" : "Save to enable autosave");
  }

  function editPage(page: PageResponse) {
    const nextDraft = createDraft(page);
    setDraft(nextDraft);
    setSelectedNodeId(nextDraft.pageJson.layout.children?.[0]?.id ?? null);
    setDirty(false);
    setAutoSaveState("Manual save");
    void loadVersions(page);
  }

  function resetDraft() {
    setDraft(createDraft());
    setSelectedNodeId(null);
    setSelectedPage(null);
    setVersions([]);
    setDirty(false);
    setAutoSaveState("Manual save");
  }

  function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    void saveDraftPage();
  }

  function addComponent(component: ComponentRegistryResponse) {
    const node = createNode(component);
    mutatePageJson((pageJson) => ({
      ...pageJson,
      layout: {
        ...pageJson.layout,
        children: [...(pageJson.layout.children ?? []), node],
      },
    }));
    setSelectedNodeId(node.id);
  }

  function removeComponent(nodeId: string) {
    mutatePageJson((pageJson) => ({
      ...pageJson,
      layout: {
        ...pageJson.layout,
        children: removeNode(pageJson.layout.children ?? [], nodeId),
      },
    }));
    if (selectedNodeId === nodeId) setSelectedNodeId(null);
  }

  function updateSelectedProp(name: string, value: JsonValue) {
    if (!selectedNodeId) return;
    mutatePageJson((pageJson) => ({
      ...pageJson,
      layout: {
        ...pageJson.layout,
        children: updateNode(pageJson.layout.children ?? [], selectedNodeId, (node) => ({
          ...node,
          props: { ...(node.props ?? {}), [name]: value },
        })),
      },
    }));
  }

  function updateMetadata(field: "title" | "description" | "og_image", value: string) {
    mutatePageJson((pageJson) => ({
      ...pageJson,
      metadata: { ...(pageJson.metadata ?? {}), [field]: value },
    }));
  }

  function handleDragStart(event: DragStartEvent) {
    setActiveDragId(String(event.active.id));
  }

  function handleDragEnd(event: DragEndEvent) {
    const activeId = String(event.active.id);
    const overId = event.over ? String(event.over.id) : null;
    setActiveDragId(null);
    if (!overId) return;

    if (activeId.startsWith("component:")) {
      const componentKey = activeId.replace("component:", "");
      const component = components.find((item) => item.component_key === componentKey);
      if (component) addComponent(component);
      return;
    }

    if (activeId === overId) return;
    const children = draft.pageJson.layout.children ?? [];
    const oldIndex = children.findIndex((node) => node.id === activeId);
    const newIndex = children.findIndex((node) => node.id === overId);
    if (oldIndex < 0 || newIndex < 0) return;

    mutatePageJson((pageJson) => ({
      ...pageJson,
      layout: {
        ...pageJson.layout,
        children: arrayMove(pageJson.layout.children ?? [], oldIndex, newIndex),
      },
    }));
  }

  async function transitionPage(page: PageResponse) {
    setError(null);
    try {
      if (page.status === "published") {
        await api.pages.unpublish(page.id);
      } else {
        await api.pages.publish(page.id);
      }
      await load(page.id);
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
      if (draft.id === page.id) resetDraft();
      await load();
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : "Failed to delete page");
    }
  }

  async function restoreVersion(version: PageVersionResponse) {
    setError(null);
    try {
      const restored = await api.pages.restore(version.page_id, version.version);
      editPage(restored);
      await load(restored.id);
      await loadVersions(restored);
    } catch (caught) {
      setError(caught instanceof ApiError ? caught.message : "Failed to restore version");
    }
  }

  async function copyPreviewUrl(pageId: string) {
    const token = accessToken ? `?access_token=${encodeURIComponent(accessToken)}` : "";
    await navigator.clipboard.writeText(`${api.baseUrl.replace(/^http/, "ws")}/api/preview/${pageId}${token}`);
  }

  const filteredComponents = components.filter((component) =>
    `${component.name} ${component.component_key} ${component.category}`.toLowerCase().includes(componentQuery.toLowerCase()),
  );
  const selectedNode = findNode(draft.pageJson.layout.children ?? [], selectedNodeId);
  const selectedComponent = componentForNode(components, selectedNode);
  const activeComponent = activeDragId?.startsWith("component:")
    ? components.find((component) => `component:${component.component_key}` === activeDragId)
    : undefined;

  return (
    <div className="page-stack page-builder-page">
      <section className="panel full-width-panel page-builder-shell">
        <div className="panel-header builder-header">
          <div>
            <h2>{draft.id ? "Page Builder Editor" : "New page builder"}</h2>
            <span>{draft.id ? autoSaveState : "Create the page once, then autosave will keep drafts current."}</span>
          </div>
          <div className="panel-actions">
            {draft.id && <StatusBadge label={`${countNodes(draft.pageJson)} blocks`} tone="neutral" />}
            <button className="secondary-button" type="button" onClick={resetDraft}>
              <Plus size={16} aria-hidden="true" />
              New
            </button>
            <button className="secondary-button" type="button" onClick={() => draft.id && void copyPreviewUrl(draft.id)} disabled={!draft.id}>
              <Copy size={16} aria-hidden="true" />
              Preview socket
            </button>
          </div>
        </div>

        <form className="builder-meta" onSubmit={handleSubmit}>
          <label>
            Title
            <input value={draft.title} onChange={(event) => setDraft({ ...draft, title: event.target.value })} required />
          </label>
          <label>
            Slug
            <input value={draft.slug} onChange={(event) => setDraft({ ...draft, slug: event.target.value })} required />
          </label>
          <button className="primary-button" type="submit" disabled={saving}>
            <Save size={16} aria-hidden="true" />
            {saving ? "Saving..." : "Save page"}
          </button>
        </form>

        {error && (
          <div className="builder-error">
            <StatusBadge label={error} tone="danger" />
          </div>
        )}

        <DndContext sensors={sensors} collisionDetection={closestCenter} onDragStart={handleDragStart} onDragEnd={handleDragEnd}>
          <div className="page-builder-grid">
            <aside className="component-panel" aria-label="Component panel">
              <div className="builder-column-header">
                <strong>Components</strong>
                <span>{filteredComponents.length}</span>
              </div>
              <label className="builder-search">
                <input value={componentQuery} onChange={(event) => setComponentQuery(event.target.value)} placeholder="Search components" />
              </label>
              <div className="palette-list">
                {filteredComponents.map((component) => (
                  <PaletteItem component={component} key={component.id} onAdd={addComponent} />
                ))}
                {!loading && filteredComponents.length === 0 && <p className="empty-copy">No components registered.</p>}
              </div>
            </aside>

            <main className="canvas-panel" aria-label="Canvas">
              <BuilderCanvas
                components={components}
                onRemove={removeComponent}
                onSelect={setSelectedNodeId}
                pageJson={draft.pageJson}
                selectedNodeId={selectedNodeId}
              />
              <LivePreview components={components} pageJson={draft.pageJson} />
            </main>

            <aside className="props-panel" aria-label="Props editor">
              <div className="builder-column-header">
                <strong>Props editor</strong>
                {selectedComponent && <span>{selectedComponent.component_key}</span>}
              </div>

              {selectedNode && selectedComponent ? (
                <div className="props-form">
                  <label>
                    Component ID
                    <input value={selectedNode.id} readOnly />
                  </label>
                  {getPropDefinitions(selectedComponent).map((definition) => (
                    <PropControl
                      definition={definition}
                      key={definition.name}
                      onChange={(value) => updateSelectedProp(definition.name, value)}
                      value={selectedNode.props?.[definition.name]}
                    />
                  ))}
                  {getPropDefinitions(selectedComponent).length === 0 && <p className="empty-copy">This component has no editable props.</p>}
                </div>
              ) : (
                <div className="props-form">
                  <p className="empty-copy">Select a block to edit its props, or update page metadata below.</p>
                  <label>
                    Metadata title
                    <input value={draft.pageJson.metadata?.title ?? ""} onChange={(event) => updateMetadata("title", event.target.value)} />
                  </label>
                  <label>
                    Description
                    <textarea rows={4} value={draft.pageJson.metadata?.description ?? ""} onChange={(event) => updateMetadata("description", event.target.value)} />
                  </label>
                  <label>
                    OG image
                    <input value={draft.pageJson.metadata?.og_image ?? ""} onChange={(event) => updateMetadata("og_image", event.target.value)} />
                  </label>
                </div>
              )}
            </aside>
          </div>
          <DragOverlay>
            {activeComponent && (
              <div className="drag-overlay-card">
                <Layers3 size={16} aria-hidden="true" />
                {activeComponent.name}
              </div>
            )}
          </DragOverlay>
        </DndContext>
      </section>

      <section className="panel list-panel full-width-panel">
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
                    <button className="icon-button" type="button" onClick={() => void copyPreviewUrl(page.id)} aria-label="Copy preview socket URL">
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