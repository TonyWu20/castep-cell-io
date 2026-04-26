# Full CellDocument Overhaul

## Context

Issue #8 originally requested adding `kpoints_mp_grid` to `CellDocument`, but the root cause is deeper: `CellDocument` has no mutual-exclusion validation, doesn't use the builder pattern in its parser, and is missing many keyword types that already exist. This plan covers all of it.

### Already done (commit `04f4d1e`)

KPOINT/KPOINTS singular/plural alias support for all existing k-point keyword types:

- Added `BLOCK_ALIASES: &'static [&'static str]` to `FromBlock` trait (defaults to `&[]`, backward compatible)
- Added `KEY_ALIASES: &'static [&'static str]` to `FromKeyValue` trait (defaults to `&[]`, backward compatible)
- Added `find_block_any(tokens, &[&str])` helper in `castep_cell_fmt/src/query.rs`
- Changed canonical `BLOCK_NAME` from plural → singular for 6 types (KPOINT_LIST, KPOINT_MP_GRID, KPOINT_MP_SPACING, MAGRES_KPOINT_LIST, OPTICS_KPOINT_LIST, BS_KPOINT_LIST)
- Added plural `BLOCK_ALIASES` to types already using singular canonical names
- Updated `CellDocument::from_cell_file` to use `find_block_any` for all k-point lookups

**Pattern to follow:** All new keyword types must set `BLOCK_ALIASES`/`KEY_ALIASES` for the KPOINT/KPOINTS plural variant (and any other accepted alternative spellings).

**Backward compat principle:** All existing BS_ types are kept as-is (names, BLOCK_NAMEs, KEY_NAMEs unchanged). During parsing, CellDocument accepts both BS_ and SPECTRAL_ keyword names. Serialization emits BS_ (so old CASTEP binaries can read the output). Only genuinely new types (no BS_ equivalent) are created fresh.

**Key decisions:**
- `cell_constraints` supersedes `fix_all_cell` (if both present, drop `fix_all_cell`)
- `Lattice` enum gains `Abc(LatticeABC)` variant
- `CellDocument` adopts sub-group strategy (like `ParamDocument`) to avoid bon field limits and improve readability

---

## Part 1: Create new keyword types

All in `castep_cell_io/src/cell/`.

### 1a. SCF k-point companions (in `bz_sampling_kpoints/`)
- **`kpoints_mp_offset.rs`** — `KpointsMpOffset(pub [f64; 3])`, `KEY_NAME = "KPOINTS_MP_OFFSET"`, `KEY_ALIASES = &["KPOINT_MP_OFFSET"]`
- *Already exist:* `KpointsMpGrid`, `KpointsMpSpacing`
- Update `bz_sampling_kpoints/mod.rs` to register + re-export

### 1b. Spectral k-point types (in `bz_sampling_kpoints/`)

**Counterparts to existing BS_ types (first-class types, not parsing aliases). Each type sets `BLOCK_ALIASES`/`KEY_ALIASES` for the KPOINT/KPOINTS plural variant:**

- **`spectral_kpoint_path.rs`** — `SpectralKpointPath { points: Vec<SpectralKpointPathEntry> }`, `BLOCK_NAME = "SPECTRAL_KPOINT_PATH"`, `BLOCK_ALIASES = &["SPECTRAL_KPOINTS_PATH"]`
  - Entry type decision: `SpectralKpointPathEntry` is structurally identical to `BsKpointPathEntry` (a `[f64; 3]` coord). Use a separate type for now (keeps the BS_ ↔ SPECTRAL_ split clean). Share via a common alias if they remain identical after implementation.
- **`spectral_kpoints_list.rs`** — `SpectralKpointsList { ... }`, `BLOCK_NAME = "SPECTRAL_KPOINTS_LIST"`, `BLOCK_ALIASES = &["SPECTRAL_KPOINT_LIST"]`
- **`spectral_kpoint_path_spacing.rs`** — `SpectralKpointPathSpacing { value: f64, unit: Option<InvLengthUnit> }`, `KEY_NAME = "SPECTRAL_KPOINT_PATH_SPACING"`, `KEY_ALIASES = &["SPECTRAL_KPOINTS_PATH_SPACING"]`

**Net-new types (no BS_ equivalent):**
- **`spectral_kpoints_mp_grid.rs`** — `SpectralKpointsMpGrid(pub [u32; 3])`, `KEY_NAME = "SPECTRAL_KPOINTS_MP_GRID"`, `KEY_ALIASES = &["SPECTRAL_KPOINT_MP_GRID"]`
- **`spectral_kpoints_mp_spacing.rs`** — `SpectralKpointsMpSpacing { value: f64, unit: Option<InvLengthUnit> }`, `KEY_NAME = "SPECTRAL_KPOINTS_MP_SPACING"`, `KEY_ALIASES = &["SPECTRAL_KPOINT_MP_SPACING"]`
- **`spectral_kpoints_mp_offset.rs`** — `SpectralKpointsMpOffset(pub [f64; 3])`, `KEY_NAME = "SPECTRAL_KPOINTS_MP_OFFSET"`, `KEY_ALIASES = &["SPECTRAL_KPOINT_MP_OFFSET"]`

Update `bz_sampling_kpoints/mod.rs`

### 1c. Symmetry types (in `symmetry/`)
- **`symmetry_generate.rs`** — `SymmetryGenerate` (tag type, flag keyword — uses `has_flag`)
- Update `symmetry/mod.rs`

### 1d. Phonon MP types (in `phonon/`)
- **`phonon_kpoints_mp_grid.rs`** — `PhononKpointsMpGrid(pub [u32; 3])`, `KEY_NAME = "PHONON_KPOINTS_MP_GRID"`, `KEY_ALIASES = &["PHONON_KPOINT_MP_GRID"]`
- **`phonon_kpoints_mp_spacing.rs`** — `PhononKpointsMpSpacing { value: f64, unit: Option<InvLengthUnit> }`, `KEY_NAME = "PHONON_KPOINTS_MP_SPACING"`, `KEY_ALIASES = &["PHONON_KPOINT_MP_SPACING"]`
- **`phonon_kpoints_mp_offset.rs`** — `PhononKpointsMpOffset(pub [f64; 3])`, `KEY_NAME = "PHONON_KPOINTS_MP_OFFSET"`, `KEY_ALIASES = &["PHONON_KPOINT_MP_OFFSET"]`

### 1e. Phonon fine types (in `phonon/`)
- **`phonon_fine_kpoint_path.rs`** — `PhononFineKpointPath { points: Vec<PhononKpointPathEntry> }`, `BLOCK_NAME = "PHONON_FINE_KPOINT_PATH"`, `BLOCK_ALIASES = &["PHONON_FINE_KPOINTS_PATH"]`
- **`phonon_fine_kpoint_path_spacing.rs`** — `PhononFineKpointPathSpacing { value: f64, unit: Option<InvLengthUnit> }`, `KEY_NAME = "PHONON_FINE_KPOINT_PATH_SPACING"`, `KEY_ALIASES = &["PHONON_FINE_KPOINTS_PATH_SPACING"]`
- **`phonon_fine_kpoints_mp_grid.rs`** — `PhononFineKpointsMpGrid(pub [u32; 3])`, `KEY_NAME = "PHONON_FINE_KPOINTS_MP_GRID"`, `KEY_ALIASES = &["PHONON_FINE_KPOINT_MP_GRID"]`
- **`phonon_fine_kpoints_mp_spacing.rs`** — `PhononFineKpointsMpSpacing { value: f64, unit: Option<InvLengthUnit> }`, `KEY_NAME = "PHONON_FINE_KPOINTS_MP_SPACING"`, `KEY_ALIASES = &["PHONON_FINE_KPOINT_MP_SPACING"]`
- **`phonon_fine_kpoints_mp_offset.rs`** — `PhononFineKpointsMpOffset(pub [f64; 3])`, `KEY_NAME = "PHONON_FINE_KPOINTS_MP_OFFSET"`, `KEY_ALIASES = &["PHONON_FINE_KPOINT_MP_OFFSET"]`
- Update `phonon/mod.rs`

### 1f. Fix missing aliases on existing types
- **`phonon_kpoint_path_spacing.rs`** — Add `KEY_ALIASES = &["PHONON_KPOINTS_PATH_SPACING"]`

---

## Part 2: Add `Lattice::Abc` variant

In `cell_document.rs`:
- Add `Lattice::Abc(LatticeABC)` to the enum
- Add `impl From<LatticeABC> for Lattice`
- Add `impl From<LatticeCart> for Lattice` (missing — bon `#[builder(on(Lattice, into))]` requires it)
- Update `ToCell for Lattice` to handle both variants
- Add `LATTICE_ABC` parsing support in `CellDocument::from_cell_file`

---

## Part 3: Create sub-group structs

New files in `castep_cell_io/src/cell/` (registered in `cell/mod.rs`):

### Group structs (each gets its own file):

All group structs derive `Debug, Clone, Default, bon::Builder`. `Default` is required so `CellDocument` can use `#[builder(default)]` on each sub-group field. Each field in the sub-group is `Option<T>` — nested `from_cells` calls handle absence.

#### `kpoints_params.rs` — SCF k-point sampling
- `KpointsParams` with fields: `kpoints_list`, `kpoints_mp_grid`, `kpoints_mp_spacing`, `kpoints_mp_offset`
- `FromCellFile` / `ToCellFile` / `validate()` (mutex: list/grid/spacing)

Notes: `kpoints_mp_offset` has no mutual-exclusion constraint (it's a companion to `kpoints_mp_grid`/`kpoints_mp_spacing`). `KpointsList`/`KpointsMpGrid`/`KpointsMpSpacing` already set `BLOCK_ALIASES`/`KEY_ALIASES` for KPOINT/KPOINTS variants (commit 04f4d1e). The new `KpointsMpOffset` will also set `KEY_ALIASES`.

#### `spectral_params.rs` — Spectral/BS k-points
- `SpectralParams` with 9 fields:
  - BS_ side: `bs_kpoint_path`, `bs_kpoints_list`, `bs_kpoint_path_spacing`
  - SPECTRAL_ side: `spectral_kpoint_path`, `spectral_kpoints_list`, `spectral_kpoint_path_spacing`
  - Net-new: `spectral_kpoints_mp_grid`, `spectral_kpoints_mp_spacing`, `spectral_kpoints_mp_offset`
- `FromCellFile`: each field uses its type's own `from_cells` (which uses `BLOCK_NAME`/`KEY_NAME` + `BLOCK_ALIASES`/`KEY_ALIASES` — no dual-name logic needed at the group level)
- `ToCellFile`: each field serializes under its own native name (BS_ for BS_ types, SPECTRAL_ for SPECTRAL_ types)
- `validate()`:
  1. Mutual exclusion per keyword pair (e.g., `bs_kpoint_path` XOR `spectral_kpoint_path`) — group fields by type and count set bits per exclusion group
  2. Warning if any BS_ field is set alongside any SPECTRAL_ field (mixed prefix usage)
  3. Standard mutex: path/list/mp_grid/mp_spacing across all nine fields — implement by counting set fields in each exclusion group (path methods, list methods, mp_grid, mp_spacing, mp_offset), reject if any group has >1 field set

#### `optics_magres_params.rs` — Optics + Magres k-points
- `OpticsMagresParams` with fields: `optics_kpoints_list`, `magres_kpoints_list`
- `FromCellFile` / `ToCellFile` / `validate()`

#### `symmetry_params.rs` — Cell symmetry
- `SymmetryParams` with fields: `symmetry_ops`, `symmetry_generate`, `symmetry_tol`
- `FromCellFile` / `ToCellFile` / `validate()` (mutex: ops/generate)

#### `constraints_params.rs` — Movement constraints
- `ConstraintsParams` with fields: `fix_com`, `ionic_constraints`, `nonlinear_constraints`, `fix_all_ions`, `fix_all_cell`, `cell_constraints`, `fix_vol`
- `FromCellFile` / `ToCellFile`
- `validate()`: if `cell_constraints` is Some, set `fix_all_cell = None`
  - Note: this silently drops user input. At minimum document the override; consider emitting a warning via `eprintln!` or extending validate's return type later.

Note: Resolves pre-existing bugs where `fix_com`/`fix_all_cell`/`fix_all_ions` are parsed by discarding the value (always `true`). The sub-group consistently uses `Type::from_cells(tokens)` which properly parses the boolean value.

#### `external_field_params.rs` — External fields
- `ExternalFieldParams` with fields: `external_efield`, `external_pressure`
- `FromCellFile` / `ToCellFile` / `validate()`

#### `species_params.rs` — Species properties
- `SpeciesParams` with fields: `species_mass`, `species_pot`, `species_lcao_states`, `species_q`, `hubbard_u`, `sedc_custom_params`
- `FromCellFile` / `ToCellFile` / `validate()`

#### `phonon_params.rs` — Phonon (coarse) settings
- `PhononParams` with fields: `phonon_kpoint_list`, `phonon_kpoint_path`, `phonon_kpoint_path_spacing`, `phonon_kpoints_mp_grid`, `phonon_kpoints_mp_spacing`, `phonon_kpoints_mp_offset`, `phonon_gamma_directions`, `phonon_supercell_matrix`, `supercell_kpoint_list`
- `FromCellFile` / `ToCellFile`
- `validate()` (mutex: path/list/mp_grid/mp_spacing)

Note: `PhononKpointPathSpacing` needs `KEY_ALIASES` added (addressed in Part 1f). New types `PhononKpointsMpGrid`/`Spacing`/`Offset` set `KEY_ALIASES` for KPOINT variant.

#### `phonon_fine_params.rs` — Phonon fine settings
- `PhononFineParams` with fields: `phonon_fine_kpoint_list`, `phonon_fine_kpoint_path`, `phonon_fine_kpoint_path_spacing`, `phonon_fine_kpoints_mp_grid`, `phonon_fine_kpoints_mp_spacing`, `phonon_fine_kpoints_mp_offset`
- `FromCellFile` / `ToCellFile`
- `validate()` (mutex: path/list/mp_grid/mp_spacing)

#### `dynamics_params.rs` — MD dynamics
- `DynamicsParams` with fields: `ionic_velocities`
- `FromCellFile` / `ToCellFile` / `validate()`

---

## Part 4: Restructure `CellDocument`

```rust
pub struct CellDocument {
    pub lattice: Lattice,
    pub positions: Positions,
    pub kpoints: KpointsParams,
    pub spectral: SpectralParams,
    pub optics_magres: OpticsMagresParams,
    pub symmetry: SymmetryParams,
    pub constraints: ConstraintsParams,
    pub external_fields: ExternalFieldParams,
    pub species: SpeciesParams,
    pub phonon: PhononParams,
    pub phonon_fine: PhononFineParams,
    pub dynamics: DynamicsParams,
}
```

- `FromCellFile`: parse required fields (lattice, positions), then each group via `GroupType::from_cell_file(tokens)?`, build, validate
- `ToCellFile`: serialize required fields, then `cells.extend(self.group.to_cell_file())`
- `validate()`: call each group's validate(), then inter-group checks if any (currently none needed)
- Parse `LATTICE_ABC` block in addition to `LATTICE_CART`

---

## Part 5: Update imports/exports

### Module structure (following param module pattern):
- `castep_cell_io/src/cell/mod.rs`: register new sub-group modules as `pub mod`
- `castep_cell_io/src/cell/constraints/mod.rs`: add `pub use cell_constraints::CellConstraints;` (existing type, not publicly exported)
- `castep_cell_io/src/cell_document.rs`: update all imports, add `impl From<LatticeCart> for Lattice`, add `impl From<LatticeABC> for Lattice`

### Public API exports:
- `castep_cell_io/src/lib.rs`: export `Lattice`, `LatticeABC`, `Positions`, `CellDocument`, `CellDocumentBuilder`
- Sub-group types (e.g., `KpointsParams`, `SpectralParams`) are accessible via `cell::` module hierarchy — consistent with existing param module pattern (no top-level re-export)

---

## Verification

- For each new keyword type: round-trip tests (`from_cell_value` → `to_cell_value`), including alias parsing
- For each sub-group: `validate()` unit tests covering each mutual-exclusion case
- For `CellDocument`: integration tests for full parse-validate-serialize cycle, including:
  - `cell_constraints` superseding `fix_all_cell`
  - BS_/SPECTRAL_ mutual-exclusion rejection
  - Mixed-prefix warning detection
  - KPOINT/KPOINTS alias round-trip for all k-point types
  - `LATTICE_ABC` parsing
  - Fixed `fix_com`/`fix_all_cell`/`fix_all_ions` boolean value parsing

```bash
cargo build
cargo test -p castep_cell_io
cargo clippy
```
