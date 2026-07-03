import { useCallback, useEffect, useMemo, useState } from "react";
import { BadgeCheck, PackagePlus, RefreshCw, Save, Send, Store, Upload } from "lucide-react";

import { StatusBadge } from "../components/StatusBadge";
import { useI18n } from "../i18n";
import { ApiError, api } from "../services/api";
import type {
  MarketplaceCreatorResponse,
  MarketplaceListingRequest,
  MarketplaceListingResponse,
  MarketplacePricingType,
  MarketplaceProductType,
} from "../types/api";

const PRODUCT_TYPES: MarketplaceProductType[] = [
  "component_pack",
  "design_template",
  "integration_plugin",
  "backend_extension",
];
const PRICING_TYPES: MarketplacePricingType[] = ["free", "paid", "custom"];

const defaultCreatorDraft = {
  slug: "",
  display_name: "",
  bio: "",
  support_email: "",
};

const defaultListingDraft = {
  product_type: "component_pack" as MarketplaceProductType,
  title: "",
  slug: "",
  summary: "",
  description: "",
  category: "components",
  pricing_type: "free" as MarketplacePricingType,
  price_cents: 0,
  license: "commercial",
  support_url: "",
  screenshots: "",
};

const defaultManifest = JSON.stringify(
  {
    manifest_version: "2026-07",
    name: "SaaS Hero Pack",
    version: "1.0.0",
    type: "component_pack",
    permissions: ["page.read"],
    compatibility: {
      min_zinhar_version: "2.0.0",
      max_zinhar_version: "3.0.0",
    },
    entry_points: {
      components: "components/index.json",
    },
    assets: ["components/hero.json", "assets/preview.png"],
  },
  null,
  2,
);

type CreatorDraft = typeof defaultCreatorDraft;
type ListingDraft = typeof defaultListingDraft;

function apiMessage(caught: unknown, fallback: string) {
  return caught instanceof ApiError ? caught.message : fallback;
}

function creatorTone(status?: string) {
  if (status === "approved") return "success";
  if (status === "rejected" || status === "suspended") return "danger";
  if (status === "pending") return "warning";
  return "neutral";
}

function listingTone(status: string) {
  if (status === "approved") return "success";
  if (status === "submitted") return "warning";
  if (status === "blocked" || status === "suspended") return "danger";
  return "neutral";
}

function screenshotsFromText(value: string) {
  return value
    .split(/\r?\n/)
    .map((item) => item.trim())
    .filter(Boolean);
}

function listingToDraft(listing: MarketplaceListingResponse): ListingDraft {
  return {
    product_type: listing.product_type,
    title: listing.title,
    slug: listing.slug,
    summary: listing.summary,
    description: listing.description,
    category: listing.category,
    pricing_type: listing.pricing_type,
    price_cents: listing.price_cents,
    license: listing.license,
    support_url: listing.support_url ?? "",
    screenshots: listing.screenshots.join("\n"),
  };
}

export function MarketplacePage() {
  const { t } = useI18n();
  const [creator, setCreator] = useState<MarketplaceCreatorResponse | null>(null);
  const [creatorDraft, setCreatorDraft] = useState<CreatorDraft>(defaultCreatorDraft);
  const [listings, setListings] = useState<MarketplaceListingResponse[]>([]);
  const [listingDraft, setListingDraft] = useState<ListingDraft>(defaultListingDraft);
  const [editingListingId, setEditingListingId] = useState<string | null>(null);
  const [selectedListingId, setSelectedListingId] = useState("");
  const [manifest, setManifest] = useState(defaultManifest);
  const [packageFile, setPackageFile] = useState<File | null>(null);
  const [loading, setLoading] = useState(false);
  const [actionLoading, setActionLoading] = useState(false);
  const [message, setMessage] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const approvedCreator = creator?.status === "approved";
  const selectedListing = useMemo(
    () => listings.find((listing) => listing.id === selectedListingId) ?? null,
    [listings, selectedListingId],
  );

  const loadMarketplace = useCallback(async function loadMarketplace() {
    setLoading(true);
    setError(null);
    try {
      const [creatorState, nextListings] = await Promise.all([
        api.marketplace.creator(),
        api.marketplace.listings(),
      ]);
      setCreator(creatorState.creator);
      setListings(nextListings);
      if (creatorState.creator) {
        setCreatorDraft({
          slug: creatorState.creator.slug,
          display_name: creatorState.creator.display_name,
          bio: creatorState.creator.bio ?? "",
          support_email: creatorState.creator.support_email ?? "",
        });
      }
      if (!selectedListingId && nextListings.length > 0) {
        setSelectedListingId(nextListings[0].id);
      }
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.load")));
    } finally {
      setLoading(false);
    }
  }, [selectedListingId, t]);

  useEffect(() => {
    void loadMarketplace();
  }, [loadMarketplace]);
  async function saveCreator() {
    setActionLoading(true);
    setMessage(null);
    setError(null);
    try {
      const saved = await api.marketplace.requestCreator({
        slug: creatorDraft.slug,
        display_name: creatorDraft.display_name,
        bio: creatorDraft.bio || undefined,
        support_email: creatorDraft.support_email || undefined,
      });
      setCreator(saved);
      setMessage(t("marketplace.message.creatorSaved"));
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.creator")));
    } finally {
      setActionLoading(false);
    }
  }

  function listingPayload(): MarketplaceListingRequest {
    return {
      product_type: listingDraft.product_type,
      title: listingDraft.title,
      slug: listingDraft.slug,
      summary: listingDraft.summary,
      description: listingDraft.description,
      category: listingDraft.category,
      pricing_type: listingDraft.pricing_type,
      price_cents: Number(listingDraft.price_cents) || 0,
      license: listingDraft.license,
      support_url: listingDraft.support_url || undefined,
      screenshots: screenshotsFromText(listingDraft.screenshots),
    };
  }

  async function saveListing() {
    setActionLoading(true);
    setMessage(null);
    setError(null);
    try {
      const payload = listingPayload();
      const saved = editingListingId
        ? await api.marketplace.updateListing(editingListingId, payload)
        : await api.marketplace.createListing(payload);
      setListings((current) => {
        const exists = current.some((listing) => listing.id === saved.id);
        return exists ? current.map((listing) => (listing.id === saved.id ? saved : listing)) : [saved, ...current];
      });
      setSelectedListingId(saved.id);
      setEditingListingId(saved.id);
      setMessage(t("marketplace.message.listingSaved"));
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.listing")));
    } finally {
      setActionLoading(false);
    }
  }

  async function submitListing(listingId: string) {
    setActionLoading(true);
    setMessage(null);
    setError(null);
    try {
      const submitted = await api.marketplace.submitListing(listingId);
      setListings((current) => current.map((listing) => (listing.id === submitted.id ? submitted : listing)));
      setMessage(t("marketplace.message.listingSubmitted"));
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.submit")));
    } finally {
      setActionLoading(false);
    }
  }

  async function uploadVersion() {
    if (!selectedListingId || !packageFile) return;
    setActionLoading(true);
    setMessage(null);
    setError(null);
    try {
      const submitted = await api.marketplace.uploadVersion(selectedListingId, packageFile, manifest);
      setMessage(t("marketplace.message.versionSubmitted").replace("{version}", submitted.version.version));
      setPackageFile(null);
      setListings(await api.marketplace.listings());
    } catch (caught) {
      setError(apiMessage(caught, t("marketplace.error.upload")));
    } finally {
      setActionLoading(false);
    }
  }

  function editListing(listing: MarketplaceListingResponse) {
    setEditingListingId(listing.id);
    setSelectedListingId(listing.id);
    setListingDraft(listingToDraft(listing));
  }

  function newListing() {
    setEditingListingId(null);
    setListingDraft(defaultListingDraft);
  }

  return (
    <div className="page-stack marketplace-page">
      <div className="toolbar toolbar--end">
        <button className="secondary-button" type="button" onClick={loadMarketplace} disabled={loading}>
          <RefreshCw size={16} aria-hidden="true" />
          {t("marketplace.refresh")}
        </button>
      </div>

      {message && <StatusBadge label={message} tone="success" />}
      {error && <StatusBadge label={error} tone="danger" />}

      <section className="metric-grid" aria-label={t("marketplace.metrics.aria")}>
        <div className="metric-card">
          <Store size={20} aria-hidden="true" />
          <span>{t("marketplace.creator.status")}</span>
          <strong>{creator ? creator.status : t("marketplace.creator.none")}</strong>
        </div>
        <div className="metric-card">
          <PackagePlus size={20} aria-hidden="true" />
          <span>{t("marketplace.listings.title")}</span>
          <strong>{listings.length}</strong>
        </div>
        <div className="metric-card">
          <BadgeCheck size={20} aria-hidden="true" />
          <span>{t("marketplace.creator.reviewGate")}</span>
          <strong>{approvedCreator ? t("common.enabled") : t("common.disabled")}</strong>
        </div>
      </section>

      <section className="two-column-workspace marketplace-workspace">
        <div className="panel">
          <div className="panel-header">
            <div>
              <h2>{t("marketplace.creator.title")}</h2>
              <span>{t("marketplace.creator.description")}</span>
            </div>
            <StatusBadge label={creator?.status ?? t("marketplace.creator.none")} tone={creatorTone(creator?.status)} />
          </div>
          <div className="padded form-grid">
            <label>
              {t("common.slug")}
              <input value={creatorDraft.slug} onChange={(event) => setCreatorDraft({ ...creatorDraft, slug: event.target.value })} />
            </label>
            <label>
              {t("marketplace.creator.displayName")}
              <input value={creatorDraft.display_name} onChange={(event) => setCreatorDraft({ ...creatorDraft, display_name: event.target.value })} />
            </label>
            <label>
              {t("marketplace.creator.supportEmail")}
              <input value={creatorDraft.support_email} onChange={(event) => setCreatorDraft({ ...creatorDraft, support_email: event.target.value })} />
            </label>
            <label>
              {t("marketplace.creator.bio")}
              <textarea rows={4} value={creatorDraft.bio} onChange={(event) => setCreatorDraft({ ...creatorDraft, bio: event.target.value })} />
            </label>
            <button className="primary-button" type="button" onClick={saveCreator} disabled={actionLoading}>
              <Save size={16} aria-hidden="true" />
              {t("marketplace.creator.save")}
            </button>
          </div>
        </div>

        <div className="panel">
          <div className="panel-header">
            <div>
              <h2>{t("marketplace.listingForm.title")}</h2>
              <span>{t("marketplace.listingForm.description")}</span>
            </div>
            <button className="secondary-button" type="button" onClick={newListing}>
              {t("common.new")}
            </button>
          </div>
          <div className="padded form-grid">
            <label>
              {t("common.title")}
              <input value={listingDraft.title} onChange={(event) => setListingDraft({ ...listingDraft, title: event.target.value })} />
            </label>
            <label>
              {t("common.slug")}
              <input value={listingDraft.slug} onChange={(event) => setListingDraft({ ...listingDraft, slug: event.target.value })} />
            </label>
            <label>
              {t("common.type")}
              <select value={listingDraft.product_type} onChange={(event) => setListingDraft({ ...listingDraft, product_type: event.target.value as MarketplaceProductType })}>
                {PRODUCT_TYPES.map((type) => <option key={type} value={type}>{type}</option>)}
              </select>
            </label>
            <label>
              {t("marketplace.listing.category")}
              <input value={listingDraft.category} onChange={(event) => setListingDraft({ ...listingDraft, category: event.target.value })} />
            </label>
            <label>
              {t("marketplace.listing.summary")}
              <input value={listingDraft.summary} onChange={(event) => setListingDraft({ ...listingDraft, summary: event.target.value })} />
            </label>
            <label>
              {t("common.description")}
              <textarea rows={5} value={listingDraft.description} onChange={(event) => setListingDraft({ ...listingDraft, description: event.target.value })} />
            </label>
            <div className="form-grid form-grid--inline">
              <label>
                {t("marketplace.listing.pricing")}
                <select value={listingDraft.pricing_type} onChange={(event) => setListingDraft({ ...listingDraft, pricing_type: event.target.value as MarketplacePricingType })}>
                  {PRICING_TYPES.map((type) => <option key={type} value={type}>{type}</option>)}
                </select>
              </label>
              <label>
                {t("marketplace.listing.priceCents")}
                <input type="number" min="0" value={listingDraft.price_cents} onChange={(event) => setListingDraft({ ...listingDraft, price_cents: Number(event.target.value) })} />
              </label>
            </div>
            <label>
              {t("marketplace.listing.license")}
              <input value={listingDraft.license} onChange={(event) => setListingDraft({ ...listingDraft, license: event.target.value })} />
            </label>
            <label>
              {t("marketplace.listing.supportUrl")}
              <input value={listingDraft.support_url} onChange={(event) => setListingDraft({ ...listingDraft, support_url: event.target.value })} />
            </label>
            <label>
              {t("marketplace.listing.screenshots")}
              <textarea rows={3} value={listingDraft.screenshots} onChange={(event) => setListingDraft({ ...listingDraft, screenshots: event.target.value })} />
            </label>
            <button className="primary-button" type="button" onClick={saveListing} disabled={!creator || actionLoading}>
              <Save size={16} aria-hidden="true" />
              {editingListingId ? t("marketplace.listing.update") : t("marketplace.listing.save")}
            </button>
          </div>
        </div>
      </section>

      <section className="panel">
        <div className="panel-header">
          <div>
            <h2>{t("marketplace.listings.title")}</h2>
            <span>{t("marketplace.listings.description")}</span>
          </div>
        </div>
        <div className="marketplace-listings-table">
          <table className="data-table">
            <thead>
              <tr>
                <th>{t("common.title")}</th>
                <th>{t("common.type")}</th>
                <th>{t("common.status")}</th>
                <th>{t("marketplace.listing.priceCents")}</th>
                <th>{t("common.actions")}</th>
              </tr>
            </thead>
            <tbody>
              {listings.map((listing) => (
                <tr key={listing.id}>
                  <td>{listing.title}</td>
                  <td>{listing.product_type}</td>
                  <td><StatusBadge label={listing.status} tone={listingTone(listing.status)} /></td>
                  <td>{listing.price_cents}</td>
                  <td>
                    <div className="toolbar">
                      <button className="secondary-button" type="button" onClick={() => editListing(listing)}>
                        {t("common.edit")}
                      </button>
                      <button className="secondary-button" type="button" onClick={() => submitListing(listing.id)} disabled={!approvedCreator || actionLoading}>
                        <Send size={16} aria-hidden="true" />
                        {t("common.submit")}
                      </button>
                    </div>
                  </td>
                </tr>
              ))}
              {listings.length === 0 && (
                <tr>
                  <td colSpan={5}>{t("marketplace.listings.empty")}</td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
      </section>

      <section className="panel">
        <div className="panel-header">
          <div>
            <h2>{t("marketplace.version.title")}</h2>
            <span>{selectedListing ? selectedListing.title : t("marketplace.version.noListing")}</span>
          </div>
        </div>
        <div className="padded form-grid">
          <label>
            {t("marketplace.version.listing")}
            <select value={selectedListingId} onChange={(event) => setSelectedListingId(event.target.value)}>
              <option value="">{t("common.select")}</option>
              {listings.map((listing) => <option key={listing.id} value={listing.id}>{listing.title}</option>)}
            </select>
          </label>
          <label className="file-picker">
            <span className="secondary-button">
              <Upload size={16} aria-hidden="true" />
              {t("marketplace.version.file")}
            </span>
            <input className="file-picker-input" type="file" accept=".zip,application/zip" onChange={(event) => setPackageFile(event.target.files?.[0] ?? null)} />
            <span>{packageFile ? packageFile.name : t("media.noFileChosen")}</span>
          </label>
          <label>
            {t("marketplace.version.manifest")}
            <textarea className="code-editor" rows={12} value={manifest} onChange={(event) => setManifest(event.target.value)} />
          </label>
          <button className="primary-button" type="button" onClick={uploadVersion} disabled={!approvedCreator || !selectedListingId || !packageFile || actionLoading}>
            <Upload size={16} aria-hidden="true" />
            {t("marketplace.version.upload")}
          </button>
        </div>
      </section>
    </div>
  );
}