import { ImagePlus, Search, Upload } from "lucide-react";

const mediaItems = [
  { name: "hero-placeholder.jpg", type: "image/jpeg", size: "1.2 MB" },
  { name: "product-grid.png", type: "image/png", size: "860 KB" },
  { name: "brand-mark.svg", type: "image/svg+xml", size: "24 KB" },
];

export function MediaPage() {
  return (
    <div className="page-stack">
      <div className="toolbar">
        <label className="search-field">
          <Search size={16} aria-hidden="true" />
          <input type="search" placeholder="Search media" />
        </label>
        <button className="primary-button" type="button">
          <Upload size={16} aria-hidden="true" />
          Upload
        </button>
      </div>

      <section className="media-grid" aria-label="Media library">
        {mediaItems.map((item) => (
          <article className="media-tile" key={item.name}>
            <div className="media-preview">
              <ImagePlus size={22} aria-hidden="true" />
            </div>
            <strong>{item.name}</strong>
            <span>{item.type}</span>
            <small>{item.size}</small>
          </article>
        ))}
      </section>
    </div>
  );
}
