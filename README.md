# Inventory Tracker (Loco)

Minimal SSR inventory tracking app in Rust using Loco + SeaORM + Tera templates (Bestbefors-style). :contentReference[oaicite:7]{index=7}

## Features
- Track items with:
  - manufacturer, type, size, unit of measure, order source, product number
  - barcode/id (non-unique allowed), name
  - stock quantity, minimum stock quantity
  - ordered flag (set when re-ordered)
- Dictionary of insertable strings:
  - category + value (for form convenience/autocomplete)
- UI pages:
  1) Inventory list (search-scroll, +/- stock adjust, reorder)
  2) To-be-ordered list
  3) Add/Edit inventory item
  4) Add/Edit dictionary entry

## Tech
- Loco (Rust web framework) + SeaORM + SeaORM migrations :contentReference[oaicite:8]{index=8}
- Tera server-side templates (SSR) :contentReference[oaicite:9]{index=9}
- Minimal vanilla JS

## Quick Start

```sh
cargo loco start
```

## Getting help

Check out [a quick tour](https://loco.rs/docs/getting-started/tour/) or [the complete guide](https://loco.rs/docs/getting-started/guide/).
