# Phase 1: New Keyword Types + Flat CellDocument Wiring

**Date:** 2026-05-03
**Status:** Reviewed

## Goals

1. **Fix gaps, wire orphan types, fix exports and aliases** (small) — Wire 6 existing-but-unused keyword types into `CellDocument` as flat optional fields. Add `pub use cell_constraints::CellConstraints` to `constraints/mod.rs`. Add missing `KEY_ALIASES` to `PhononKpointPathSpacing`. Resolves the long-standing `CellConstraints` public-API gap and missing `KpointsMpGrid` export.

2. **Add `Lattice::Abc` variant and `LatticeABC` parsing** (small) — Add `Lattice::Abc(LatticeABC)` to the `Lattice` enum, `From<LatticeABC>` and `From<LatticeCart>` impls, `LATTICE_ABC` block parsing in `CellDocument::from_cell_file`, and update `ToCell for Lattice`. `LatticeABC` already exists in `cell/lattice_param.rs` with `FromBlock`, `ToCell`, `bon::Builder`, and test coverage — the net-new work is the enum variant, `From` impls, `ToCell` match arm, and `from_cell_file` wiring. If both `LATTICE_CART` and `LATTICE_ABC` blocks are present in the input, return an error indicating only one lattice specification is allowed.

3. **Create `KpointsMpOffset` and `SymmetryGenerate` keyword types** (medium) — Two new keyword type files: `KpointsMpOffset(pub [f64; 3])` (key-value, `KEY_ALIASES`) and `SymmetryGenerate` (flag/tag type, uses `has_flag`). Register in respective `mod.rs`, wire into `CellDocument`.

4. **Create spectral k-point keyword types** (medium) — Six new files in `bz_sampling_kpoints/`: `SpectralKpointPath` (block, with `SpectralKpointPathEntry`), `SpectralKpointsList` (block), `SpectralKpointPathSpacing` (key-value), `SpectralKpointsMpGrid`, `SpectralKpointsMpSpacing`, `SpectralKpointsMpOffset`. All follow `bon::Builder` + `FromBlock`/`FromKeyValue` + `ToCell` pattern. Register in `bz_sampling_kpoints/mod.rs`, wire into `CellDocument`.

5. **Create phonon MP and fine keyword types** (medium) — Eight new files in `phonon/`: MP family (`PhononKpointsMpGrid`, `PhononKpointsMpSpacing`, `PhononKpointsMpOffset`), fine family (`PhononFineKpointPath` with `PhononKpointPathEntry`, `PhononFineKpointPathSpacing`, `PhononFineKpointsMpGrid`, `PhononFineKpointsMpSpacing`, `PhononFineKpointsMpOffset`). Same patterns. Register in `phonon/mod.rs`, wire into `CellDocument`.

## Scope Boundaries

**In scope:**
- Adding flat `Option<T>` fields to `CellDocument` for all new and orphan keyword types
- New keyword type files with `bon::Builder`, `FromBlock`/`FromKeyValue`, `ToCell`, `KEY_ALIASES`/`BLOCK_ALIASES`
- Module registration (`pub mod`, `pub use`) and re-export wiring
- `Lattice::Abc` variant and parsing
- `pub use cell_constraints::CellConstraints` in `constraints/mod.rs`
- `KEY_ALIASES` on `PhononKpointPathSpacing`

**Out of scope:**
- 10 sub-group param structs (`KpointsParams`, `SpectralParams`, `SymmetryParams`, `ConstraintsParams`, `ExternalFieldParams`, `SpeciesParams`, `PhononParams`, `PhononFineParams`, `OpticsMagresParams`, `DynamicsParams`)
- `CellDocument` restructure to use sub-group fields
- Validation logic (mutual exclusion, `cell_constraints` superseding `fix_all_cell`)
- Unit tests for validation (no validation code to test)
- Serialization behavior changes beyond adding new fields

## Design Notes

- **Existing types to wire**: `KpointsMpGrid`, `KpointsMpSpacing`, `BsKpointPathSpacing`, `CellConstraints`, `FixVOL`, `SymmetryTol` — all have source files but not field on `CellDocument`
- **Wiring pattern**: Add `field: Option<Type>` to `CellDocument`, add parsing in `from_cell_file`, add serialization in `to_cell_file`
- **Key-value parsing**: In `from_cell_file`, use `FromKeyValue::from_cells(tokens)?` for all key-value types (KpointsMpGrid, KpointsMpSpacing, BsKpointPathSpacing, FixVOL, SymmetryTol, KpointsMpOffset, and all new key-value types). This properly handles `KEY_ALIASES` without duplicating alias-resolution logic in `CellDocument`. For block types, use `find_block_any(tokens, &[...])`.
- **Spectral naming**: All new SPECTRAL_ types coexist with existing BS_ types. SPECTRAL_ k-point path/list use structurally identical entry types as their BS_ counterparts but kept separate for a clean prefix boundary. Net-new types (MpGrid, MpSpacing, MpOffset) have no BS_ equivalent.
- **bon builders on keyword types**: Use `#[derive(bon::Builder)]` per project convention. Simple newtype structs (`KpointsMpOffset`) can use tuple struct + manual impl if builder is unnecessary overhead.
- **Alias patterns**: All k-point types set `KEY_ALIASES`/`BLOCK_ALIASES` for KPOINT/KPOINTS plural variant. Spectral types also set SPECTRAL_KPOINT/SPECTRAL_KPOINTS aliases. For BS_-equivalent spectral types (`SpectralKpointPath`, `SpectralKpointsList`, `SpectralKpointPathSpacing`), also include BS_ aliases for backward compatibility with legacy CASTEP input files (e.g., `SpectralKpointPath` gets `BLOCK_ALIASES = &["BS_KPOINT_PATH", "BS_KPOINTS_PATH", "SPECTRAL_KPOINTS_PATH"]`). Net-new types (`SpectralKpointsMpGrid`, `SpectralKpointsMpSpacing`, `SpectralKpointsMpOffset`) have no BS_ equivalent and do not need BS_ aliases.
- **No breaking changes** in this phase: `CellDocument` API remains the same (just new optional fields added).

## Deferred Items Absorbed

None. This is the first phase of new work after the branch reset.

## Resolved Questions

- **SpectralKpointPathEntry vs BsKpointPathEntry**: Keep separate types for now. The BS_/SPECTRAL_ prefix boundary justifies distinct types; consolidation can happen later if they remain structurally identical.
- **PhononFineKpointPath entry type**: Use the existing `PhononKpointPathEntry` type. Both fine and non-fine k-point path entries are structurally identical (`[f64; 3]` coords), and a separate type adds boilerplate without semantic benefit.
