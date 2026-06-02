import { Eye, Plus } from "lucide-react";

const pages = [
  { title: "Home", slug: "home", status: "Draft", components: 3 },
  { title: "Pricing", slug: "pricing", status: "Draft", components: 2 },
  { title: "Docs", slug: "docs", status: "Draft", components: 4 },
];

export function PagesPage() {
  return (
    <div className="page-stack">
      <div className="toolbar toolbar--end">
        <button className="secondary-button" type="button">
          <Eye size={16} aria-hidden="true" />
          Preview
        </button>
        <button className="primary-button" type="button">
          <Plus size={16} aria-hidden="true" />
          New page
        </button>
      </div>

      <section className="panel">
        <table className="data-table">
          <thead>
            <tr>
              <th>Title</th>
              <th>Slug</th>
              <th>Status</th>
              <th>Components</th>
            </tr>
          </thead>
          <tbody>
            {pages.map((page) => (
              <tr key={page.slug}>
                <td>{page.title}</td>
                <td>{page.slug}</td>
                <td>{page.status}</td>
                <td>{page.components}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </section>
    </div>
  );
}
