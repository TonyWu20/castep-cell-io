---
name: castep-cell-io crate boundary rules
description: What belongs in which crate; rules for keeping concerns separated
type: project
---

## castep_cell_serde (→ castep_cell_io after rename)

- Owns: parser, IR types (`Cell`, `CellValue`), query helpers, parse traits (`FromCellValue`, `FromBlock`, `FromKeyValue`, `FromCellFile`), formatter
- Must NOT contain: any CASTEP domain types (lattice, positions, params)
- Must NOT contain: serde dependency after Phase 3

## castep_cell_data

- Owns: all CASTEP `.cell` domain types and all `.param` data types
- New `.param` types go in `castep_cell_data/src/param/`, NOT in `castep-param-io`
- After migration: must NOT import `serde`; uses only `castep_cell_io` traits

## castep-param-io

- Deprecated, not under active development
- Must still compile after rename (Cargo.toml dep pointer update only)
- Do NOT add new types here

## Why the split matters

`castep_cell_data` domain types are the consumer of `castep_cell_serde`/`castep_cell_io` traits. The trait definitions live in the provider crate to avoid circular dependencies. Domain types implement the traits in `castep_cell_data`.
