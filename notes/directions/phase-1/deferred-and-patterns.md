# Phase 1 Deferred Items & Architectural Patterns

## Key Architectural Patterns

- **Flat wiring**: Add `Option<T>` fields directly on `CellDocument`. Sub-group param structs (10 planned) are deferred — resist the urge to nest early.
- **Per-type triad**: Every keyword type gets `#[derive(bon::Builder)]` + `FromBlock`/`FromKeyValue` + `ToCell`. Simple newtypes (e.g., `KpointsMpOffset`) can use manual impls instead of `bon::Builder`.
- **Parsing in CellDocument**: Use `FromKeyValue::from_cells(tokens)?` for key-value types and `find_block_any(tokens, &[...])` for block types. This avoids duplicating alias resolution.
- **Alias rules**: All k-point types need `KEY_ALIASES`/`BLOCK_ALIASES` for KPOINT/KPOINTS plural. Spectral types also include SPECTRAL_KPOINT/KPOINTS aliases. BS_-equivalent spectral types additionally need BS_ backward compat aliases. Net-new spectral types (MpGrid, MpSpacing, MpOffset) do not.
- **PhononFineKpointPath entry**: Reuse `PhononKpointPathEntry` — a separate fine-entry type adds boilerplate with no semantic gain.

## Known Failure Modes

- **Missing `pub use` exports**: The `CellConstraints` gap (existing type, no public re-export) is the canonical example. After adding any type in a `mod.rs`, verify the `pub use` chain all the way to `lib.rs`.
- **Missing `KEY_ALIASES`**: `PhononKpointPathSpacing` lacks them. Every keyword type that goes through `FromKeyValue` must have its aliases set, or silent parse failures occur.
- **Dual-lattice error**: If both `LATTICE_CART` and `LATTICE_ABC` blocks are present, return an error (not silent override).
- **Omitted module registration**: Adding a new `.rs` file without registering in its `mod.rs` + wiring in `CellDocument` is the most common integration failure.
- **BS_ backward compat omission**: Spectral path/list/spacing types _must_ include BS_ aliases or legacy CASTEP files will fail to parse.

## Deferred Improvements

- All 10 sub-group param structs (KpointsParams, SpectralParams, etc.) and `CellDocument` restructure to nest them.
- Validation logic: mutual exclusion, `cell_constraints` superseding `fix_all_cell`.
- Unit tests for validation (no validation code exists yet).
- Consolidation of structurally identical BS_/SPECTRAL_ types.
