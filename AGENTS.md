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
- Agents are not allowed to modify which lints are allowed; lint-policy changes must come from the user.
- The live app routes are `/`, `/inventory`, `/inventory/to-order`, `/inventory/new`, `/dictionary`, and their related POST actions. `/` should redirect to `/inventory`.
- The navbar should reflect the three user-facing destinations: `Inventory`, `To Order`, and `Dictionary`.
- Dictionary category labels should use the inventory-form wording: `Manufacturer`, `Type`, `Size`, `Unit`, `Order source`.
- The dictionary category selector should always include the starter set: `manufacturer`, `item_type`, `size`, `uom`, `order_source`.
- `order_source` may be a plain string or an HTTP(S) URL. When it is a URL, `{nr}` should be replaced with the product number in both the visible label and the link target on the to-order page.
- Keep the inventory and to-order pages compact and functional; avoid decorative hero banners, pills, or informational chrome that does not help the user complete a task.
- Quick-reference entries on the inventory form should actively fill their matching field when clicked.
- `scripts/appcurl.sh` is the local helper for calling app routes through `http://localhost:5150`.

## Workflow
1. Add migrations using SeaORM migration conventions (`mYYYYMMDD_HHMMSS_*.rs`). :contentReference[oaicite:5]{index=5}
2. Run: `cargo loco db migrate`
3. Generate entities if needed: use Loco’s integrated SeaORM entity generation flow. :contentReference[oaicite:6]{index=6}
4. Implement controllers/routes for the 4 pages + small POST actions.
5. Implement Tera templates + minimal JS.
6. Add smoke tests for routes and stock adjustment.
7. Use `cargo clippy` to check for linting errors.
