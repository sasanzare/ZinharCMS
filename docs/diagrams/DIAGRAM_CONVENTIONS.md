# Diagram Conventions

These rules apply to every future `.mmd` file in `docs/diagrams/`.

## File Rules

- Use exactly one Mermaid diagram declaration per `.mmd` file, such as `flowchart TD`, `sequenceDiagram`, `classDiagram`, or `erDiagram`.
- Do not use Markdown code fences in `.mmd` files.
- Use stable ASCII identifiers for Mermaid node IDs, subgraph IDs, classes, and edge IDs.
- Use human-readable labels inside nodes.
- Make implementation status visible in node text, not only by class or color.
- Visually separate planned, documented-only, and decision-required items from implemented runtime items.
- Add evidence comments with the format `%% Evidence: path`.
- Keep each diagram understandable without needing another diagram.

## Evidence Rules

- Use current SQL migrations and database constraints as the highest-priority source for schema claims.
- Copy database table, column, enum, trigger, and policy names exactly from migrations.
- Copy backend route boundaries exactly from route composition and route files.
- Treat frontend route guards as UX behavior, not backend security.
- Treat README and historical phase documents as lower-priority evidence than code and migrations.
- If documentation and implementation disagree, mark the node with `[CONFLICT]` or reference an ambiguity ID.

## Prohibited Assumptions

- Do not invent microservices.
- Do not invent a queue.
- Do not invent a worker.
- Do not invent S3.
- Do not invent a CDN.
- Do not invent an API gateway deployment.
- Do not invent Marketplace purchase or payout entities.
- Do not infer implementation from names alone.

## Common Status Labels

- `[IMPLEMENTED]`: Current schema and runtime code support the feature.
- `[PARTIAL]`: Some schema/runtime/UI exists, but important behavior is absent or limited.
- `[PLANNED]`: Explicitly planned or required by future docs, with no current runtime implementation.
- `[DOCUMENTED ONLY]`: Described in documentation but not found in schema or code.
- `[CONFLICT]`: Documentation and implementation evidence disagree.
- `[DECISION REQUIRED]`: Current implementation does not answer a product or architecture question safely.
- `[EXTERNAL]`: Outside ZinharCMS runtime, such as Stripe, Redis, browser, or webhook receiver.

## Common Mermaid Classes

Every `.mmd` file should include only the class definitions it uses. Classes must not be the only carrier of meaning; the node label must also include a status label.

- `implemented`: solid border for implemented runtime/schema.
- `partial`: dashed border for implemented but incomplete behavior.
- `planned`: dotted border for future behavior.
- `documented`: dotted border for documentation-only claims.
- `conflict`: heavy dashed border for documentation conflicts.
- `decision`: heavy dash-dot border for decision-required areas.
- `external`: double-stroked style for external dependencies.
- `data`: database/storage nodes.
- `frontend`: frontend UI/API-client nodes.
- `backend`: backend route/service/middleware nodes.

Recommended class definitions:

- `classDef implemented fill:#e8f5ee,stroke:#17745b,stroke-width:2px,color:#111;`
- `classDef partial fill:#fff8e8,stroke:#9a6500,stroke-width:2px,stroke-dasharray: 6 3,color:#111;`
- `classDef planned fill:#f4f4f4,stroke:#666,stroke-width:2px,stroke-dasharray: 2 4,color:#111;`
- `classDef documented fill:#f5f0ff,stroke:#6a4fb3,stroke-width:2px,stroke-dasharray: 2 4,color:#111;`
- `classDef conflict fill:#ffecec,stroke:#b3261e,stroke-width:3px,stroke-dasharray: 8 3,color:#111;`
- `classDef decision fill:#fff0f6,stroke:#9b1b5a,stroke-width:3px,stroke-dasharray: 8 2 2 2,color:#111;`
- `classDef external fill:#eef6ff,stroke:#3367a8,stroke-width:2px,color:#111;`
- `classDef data fill:#edf7ff,stroke:#24577a,stroke-width:2px,color:#111;`
- `classDef frontend fill:#f2fbf8,stroke:#24735a,stroke-width:2px,color:#111;`
- `classDef backend fill:#f7f7ff,stroke:#4b4b8f,stroke-width:2px,color:#111;`

## Naming Rules

- Use lower snake-case or short ASCII identifiers, for example `tenant_middleware`, `marketplace_catalog`, and `stripe_webhooks`.
- Keep labels concise and include the implementation status, for example `Marketplace Catalog [IMPLEMENTED]`.
- Include ambiguity IDs in labels when a future reader needs context, for example `Artifact cleanup [DECISION REQUIRED] AMB-015`.
- Prefer route names from code, table names from migrations, and component names from frontend source.

## Planned Item Separation

- Planned-only items should be grouped in a clearly labeled planned/future subgraph or styled with the `planned` class.
- Decision-required items should be styled with the `decision` class and linked to the ambiguity register.
- Documentation-only claims should not be connected as runtime dependencies unless code/schema evidence exists.
