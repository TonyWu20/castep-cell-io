# DECISIONS.md — CellDocument Group Substruct Migration

**Date**: 2026-05-13
**Based on**: PLAN.md (Full CellDocument Overhaul, Parts 3-5)
**Phase**: phase-2 — CellDocument migration to group substructs

## Goals

Migrate `CellDocument` from 43 flat fields to group substructs mirroring the `ParamDocument` pattern. All keywords defined in `castep_cell_io/src/cell/` must be accessible through `CellDocument`.

**Motivation**: `CellDocument` already exceeds bon's typestate field limit for safety checks. The `ParamDocument` migration (v0.4.0) solved the same problem by grouping keywords into 18 logical sub-structs.

## Fixtures

### Declared

| Fixture | Path | Description | Blocks Exercised |
|---------|------|-------------|-----------------|
| Mg2SiO4_Cr_1.cell | `castep_cell_fmt/Mg2SiO4_Cr_1.cell` | Forsterite with Cr doping, 28 atoms | LATTICE_CART, POSITIONS_FRAC, KPOINTS_LIST, SYMMETRY_OPS, CELL_CONSTRAINTS, FIX_COM, IONIC_CONSTRAINTS, EXTERNAL_EFIELD, SPECIES_MASS, SPECIES_POT, SPECIES_LCAO_STATES |

### Needed (to add during implementation)

Additional `.cell` fixture files exercising groups not covered by Mg2SiO4_Cr_1.cell:
- KPOINTS_MP_GRID / KPOINTS_MP_SPACING / KPOINTS_MP_OFFSET variants
- SPECTRAL_ and BS_ k-point types
- OPTICS_KPOINTS_LIST, MAGRES_KPOINTS_LIST
- SYMMETRY_GENERATE, SYMMETRY_TOL
- FIX_ALL_IONS, FIX_ALL_CELL, FIX_VOL
- NONLINEAR_CONSTRAINTS
- EXTERNAL_PRESSURE
- SPECIES_Q, HUBBARD_U, SEDC_CUSTOM_PARAMS
- PHONON_ blocks (coarse + fine variants)
- IONIC_VELOCITIES

## Architectural Decisions

1. **Group struct pattern**: Follow `ParamDocument` pattern exactly — all fields `Option<T>`, `#[derive(Debug, Clone, Default, Builder)]`, `validate() -> Result<Self, String>`, `FromCellFile` with `.maybe_` chaining, `ToCellFile` with `if let Some` guards.

2. **CellDocument restructure**: After migration, `CellDocument` holds 1 group struct per logical category instead of ~43 flat fields. Required fields (`lattice`, `positions`) remain top-level.

3. **Validation split**: Each group validates intra-group constraints (e.g., k-point method mutual exclusion). `CellDocument` validates inter-group constraints (if any) and required-field presence. This mirrors the `ParamDocument` pattern where groups validate themselves and `ParamDocument.validate()` orchestrates inter-group checks.

4. **Ten groups** (matching PLAN.md Part 3):

| # | Group Struct | Field Count | Mutual Exclusion |
|---|-------------|-------------|------------------|
| 1 | `KpointsParams` | 4 | KPOINTS_LIST / MP_GRID / MP_SPACING (only 1) |
| 2 | `SpectralParams` | 9 | BS_ XOR SPECTRAL_ per block type; 1 total spectral method |
| 3 | `OpticsMagresParams` | 2 | (none — independent) |
| 4 | `SymmetryParams` | 3 | SYMMETRY_OPS / SYMMETRY_GENERATE (only 1) |
| 5 | `ConstraintsParams` | 7 | cell_constraints supersedes fix_all_cell |
| 6 | `ExternalFieldParams` | 2 | (none — independent) |
| 7 | `SpeciesParams` | 6 | (none — independent) |
| 8 | `PhononParams` | 9 | PHONON_KPOINT_PATH / LIST / MP_GRID / MP_SPACING (only 1) |
| 9 | `PhononFineParams` | 6 | PHONON_FINE_KPOINT_PATH / LIST / MP_GRID / MP_SPACING (only 1) |
| 10 | `DynamicsParams` | 1 | (none) |

5. **Breaking change**: The `CellDocument` public API changes — direct field access (`doc.kpoints_list`) becomes `doc.kpoints.kpoints_list`. Accept this per the builder pattern mandate (breaking changes preferred over backward compat shims).

## Domain Language Validation

All terms validated against CONTEXT.md:
- **Cell Block Type**: Each field in the groups is a Cell Block Type (e.g., `KpointsList`, `SpeciesPot`)
- **Cell Document Group**: Each sub-struct is a Cell Document Group (e.g., `KpointsParams`, `SpeciesParams`)
- **Mutual exclusion**: Constraint where only one of several related blocks may be present (from CASTEP spec, not an invention of this library)
- **Superseding**: `cell_constraints` overriding `fix_all_cell` (if both present, `fix_all_cell` is silently dropped with a warning) — this mirrors CASTEP's own behavior

## Success Criteria

### C-1: Fixture parse parity
After migration, `CellDocument::from_cell_file(parse("Mg2SiO4_Cr_1.cell"))` produces the same enum variant values for every field as the pre-migration code.
- Verification: extract every `Option<T>.is_some()` check from the pre-migration code and assert identical results post-migration.
- Source: Mg2SiO4_Cr_1.cell, lines 1-93

### C-2: Fixture format parity
After migration, `CellDocument::to_cell_file()` on the parsed fixture produces identical `Vec<Cell<'_>>` output (same Cell variant count, same block order, same key-value order) as the pre-migration code.
- Source: compare pre-migration and post-migration `to_cell_file()` output for the fixture

### C-3: All existing tests pass
1080 tests currently pass (1080 passed, 1 failed — the `#[ignore]`d fixture test that uses empty input). After migration, all 1080 must pass, and the `#[ignore]`d test must be fixed to actually read the fixture.

### C-4: Mutual exclusion enforcement
Each group's `validate()` correctly rejects invalid combinations per `cell_keywords.md`:
- KpointsParams: ≥2 of {list, mp_grid, mp_spacing} → error
- SpectralParams: ≥2 spectral methods (of any prefix) → error
- SymmetryParams: both ops and generate → error
- PhononParams: ≥2 of {path, list, mp_grid, mp_spacing} → error
- PhononFineParams: ≥2 of {path, list, mp_grid, mp_spacing} → error

### C-5: All cell keywords accessible
Every `pub struct` re-exported from `cell/*/mod.rs` must appear as a field in exactly one CellDocument group (or as a top-level field like `lattice`/`positions`). No orphaned types.

### C-6: No round-trip regression
For the fixture file: `parse(cell_text)` → format back → `parse(formatted)` → values identical to first parse. This catches serialization ordering issues.
