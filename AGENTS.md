# AGENTS.md

## Goal
Implement a minimal inventory tracker using Rust + Loco + SeaORM + Tera (SSR) with minimal vanilla JS, following the same general structure as the Bestbefors Loco app (SSR pages, simple controllers, templates in assets). :contentReference[oaicite:3]{index=3}

## Scope
- Inventory items with current stock, minimum stock, and an "ordered" flag.
- Dictionary entries: flat strings grouped by category for convenient form filling/autocomplete.
- Exactly four UI pages:
  1) Inventory list
  2) To-be-ordered list
  3) Add/Edit inventory item
  4) Add/Edit dictionary entry

## Non-goals
- No auth/users/roles.
- No stock history/audit log.
- No uniqueness on barcode (collisions allowed).

## Data model
Two tables:
- `inventory_items`
- `dictionary_entries` (unique on (category, value))

## Conventions
- Use SSR via TeraView; templates live under `assets/`. :contentReference[oaicite:4]{index=4}
- Keep JS minimal (search scroll + optional fetch-based +/- updates).
- Prefer POST actions for +/- and reorder flags.

## Workflow
1. Add migrations using SeaORM migration conventions (`mYYYYMMDD_HHMMSS_*.rs`). :contentReference[oaicite:5]{index=5}
2. Run: `cargo loco db migrate`
3. Generate entities if needed: use Loco’s integrated SeaORM entity generation flow. :contentReference[oaicite:6]{index=6}
4. Implement controllers/routes for the 4 pages + small POST actions.
5. Implement Tera templates + minimal JS.
6. Add smoke tests for routes and stock adjustment.
7. Use `cargo clippy` to check for linting errors.
