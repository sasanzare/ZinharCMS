import { Filter, Plus, Search } from "lucide-react";

const entries = [
  { title: "Welcome article", type: "Article", status: "Draft", updated: "Today" },
  { title: "Builder overview", type: "Article", status: "Draft", updated: "Today" },
  { title: "Homepage", type: "Landing Page", status: "Draft", updated: "Today" },
];

export function EntriesPage() {
  return (
    <div className="page-stack">
      <div className="toolbar">
        <label className="search-field">
          <Search size={16} aria-hidden="true" />
          <input type="search" placeholder="Search entries" />
        </label>
        <button className="secondary-button" type="button">
          <Filter size={16} aria-hidden="true" />
          Filter
        </button>
        <button className="primary-button" type="button">
          <Plus size={16} aria-hidden="true" />
          New entry
        </button>
      </div>

      <section className="panel">
        <table className="data-table">
          <thead>
            <tr>
              <th>Title</th>
              <th>Type</th>
              <th>Status</th>
              <th>Updated</th>
            </tr>
          </thead>
          <tbody>
            {entries.map((entry) => (
              <tr key={entry.title}>
                <td>{entry.title}</td>
                <td>{entry.type}</td>
                <td>{entry.status}</td>
                <td>{entry.updated}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </section>
    </div>
  );
}
