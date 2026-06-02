import { Plus, Search } from "lucide-react";

const rows = [
  { name: "Article", slug: "article", fields: 4, status: "Draft model" },
  { name: "Landing Page", slug: "landing-page", fields: 6, status: "Draft model" },
  { name: "Author", slug: "author", fields: 5, status: "Draft model" },
];

export function ContentTypesPage() {
  return (
    <div className="page-stack">
      <div className="toolbar">
        <label className="search-field">
          <Search size={16} aria-hidden="true" />
          <input type="search" placeholder="Search content types" />
        </label>
        <button className="primary-button" type="button">
          <Plus size={16} aria-hidden="true" />
          New type
        </button>
      </div>

      <section className="panel">
        <table className="data-table">
          <thead>
            <tr>
              <th>Name</th>
              <th>Slug</th>
              <th>Fields</th>
              <th>Status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row) => (
              <tr key={row.slug}>
                <td>{row.name}</td>
                <td>{row.slug}</td>
                <td>{row.fields}</td>
                <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </section>
    </div>
  );
}
