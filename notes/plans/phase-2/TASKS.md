# TASKS.md — CellDocument Group Substruct Migration

**Phase**: phase-2
**Plan**: PLAN.md (Parts 3-5), DECISIONS.md
**Date**: 2026-05-13
**ODD Pattern**: `/Users/tony/programming/rust-development-pipeline/skills/drive-outcomes/references/odd-pattern.md`

## Declared Fixtures

| Alias | Path | Description |
|-------|------|-------------|
| `fixture-cell-forsterite` | `castep_cell_io/tests/fixtures/Mg2SiO4_Cr_1.cell` | Forsterite Cr-doped, 28 atoms, exercises ~12 block types |
| `fixture-cell-fe2o3` | `castep_cell_io/tests/fixtures/Fe2O3.cell` | Fe₂O₃ (haematite) single-point, adds HUBBARD_U, EXTERNAL_PRESSURE, FIX_ALL_CELL, QUANTIZATION_AXIS |
| `fixture-cell-zno-lr` | `castep_cell_io/tests/fixtures/ZnO_LR.cell` | ZnO (zinc oxide) linear response, 12 symmetry ops, CELL_CONSTRAINTS |

## Exploration Notes

**2026-05-13 — Pre-migration baseline**:
- 1080 tests pass in `castep-cell-io` (all library tests + doctests).
- One `#[ignore]`d test (`test_parse_mg2sio4_cell`) uses empty input `let input = "";` — fixture file exists but is not wired up. This test is rewritten in Task D-1.
- `Mg2SiO4_Cr_1.cell` fixture exercises ~12 of 43 CellDocument fields. Gap: 31 fields not exercised.
- New fixtures added:
  - **Fe2O3.cell**: LATTICE_CART, POSITIONS_FRAC (30 atoms: 18 O + 12 Fe with SPIN), KPOINTS_LIST (5 k-points), FIX_ALL_CELL, FIX_COM, IONIC_CONSTRAINTS (empty), EXTERNAL_EFIELD, EXTERNAL_PRESSURE, SPECIES_MASS, SPECIES_POT, SPECIES_LCAO_STATES, HUBBARD_U (12 entries), QUANTIZATION_AXIS
  - **ZnO_LR.cell**: LATTICE_CART, POSITIONS_FRAC (4 atoms: 2 O + 2 Zn), KPOINTS_LIST (10 k-points), SYMMETRY_OPS (12 ops), CELL_CONSTRAINTS, FIX_COM, IONIC_CONSTRAINTS (empty), EXTERNAL_EFIELD, SPECIES_MASS, SPECIES_POT, SPECIES_LCAO_STATES

**LatticeCart row-major convention**:
- `LatticeCart` field names are already `.a`, `.b`, `.c` → row-major is implicit.
- CASTEP `.cell` file format: LATTICE_CART block has 3 rows, each is one lattice vector:
  - Row 1 → lattice vector **A** (field `.a`)
  - Row 2 → lattice vector **B** (field `.b`)
  - Row 3 → lattice vector **C** (field `.c`)
- Known gaps: doc comments don't explicitly call out this convention. See Task E-1.

**Adjusted criteria during exploration**: None yet (migration is behavior-preserving — criteria will be validated during implementation).

---

## Group A: Create Cell Document Group Structs

### TASK-A-1: Create KpointsParams group

**Kind:** lib-tdd
**Goal:** Create `cell/kpoints_params.rs` with `KpointsParams` struct aggregating SCF k-point fields.

**New file:** `castep_cell_io/src/cell/kpoints_params.rs`

**Struct fields** (all `Option<T>`):
- `kpoints_list: Option<KpointsList>`
- `kpoints_mp_grid: Option<KpointsMpGrid>`
- `kpoints_mp_spacing: Option<KpointsMpSpacing>`
- `kpoints_mp_offset: Option<KpointsMpOffset>`

**Pattern**: `#[derive(Debug, Clone, Default, Builder)]`, `FromCellFile` with `.maybe_` chaining from each type's `from_cells`, `ToCellFile` with `if let Some` guards, `validate() -> Result<Self, String>`.

**Success Criteria:**
- `KpointsParams::from_cell_file(parse("fixture-cell-forsterite"))` → `kpoints_list.is_some()`, `kpoints_mp_grid.is_none()`, `kpoints_mp_spacing.is_none()` (Source: Mg2SiO4_Cr_1.cell, line 39 — KPOINTS_LIST block present)
- `validate()` rejects ≥2 of {list, mp_grid, mp_spacing}: `KpointsParams { kpoints_list: Some(...), kpoints_mp_grid: Some(...), ..Default::default() }.validate()` → `Err(...)` (Source: supplemented_official_docs/cell_keywords.md, footnote 3)
- `validate()` passes for single-method + optional offset: `KpointsParams { kpoints_list: Some(...), ..Default::default() }.validate()` → `Ok(...)` (Source: supplemented_official_docs/cell_keywords.md, footnote 3 — offset has no mutex constraint)
- `to_cell_file()` emits `Cell::Block("KPOINT_LIST", ...)` for the set field, no output for `None` fields

**Register in `cell/mod.rs`:** `pub mod kpoints_params;`

---

### TASK-A-2: Create SpectralParams group

**Kind:** lib-tdd
**Goal:** Create `cell/spectral_params.rs` with `SpectralParams` aggregating BS_ and SPECTRAL_ k-point fields.

**New file:** `castep_cell_io/src/cell/spectral_params.rs`

**Struct fields** (all `Option<T>`):
- `bs_kpoint_path: Option<BsKpointPath>`
- `bs_kpoints_list: Option<BSKpointList>`
- `bs_kpoint_path_spacing: Option<BsKpointPathSpacing>`
- `spectral_kpoint_path: Option<SpectralKpointPath>`
- `spectral_kpoints_list: Option<SpectralKpointsList>`
- `spectral_kpoint_path_spacing: Option<SpectralKpointPathSpacing>`
- `spectral_kpoints_mp_grid: Option<SpectralKpointsMpGrid>`
- `spectral_kpoints_mp_spacing: Option<SpectralKpointsMpSpacing>`
- `spectral_kpoints_mp_offset: Option<SpectralKpointsMpOffset>`

**Success Criteria:**
- `validate()` rejects ≥2 set fields across all 9 types: e.g., `bs_kpoint_path` + `spectral_kpoint_path` → `Err(...)` (same category: path), `spectral_kpoints_mp_grid` + `spectral_kpoints_mp_spacing` → `Err(...)` (same category: mp_method) (Source: supplemented_official_docs/cell_keywords.md, footnote 4 — only one spectral method)
- Mutual exclusion groups: path methods (`bs_kpoint_path` OR `spectral_kpoint_path`), list methods (`bs_kpoints_list` OR `spectral_kpoints_list`), path_spacing (`bs_kpoint_path_spacing` OR `spectral_kpoint_path_spacing`), mp_grid (`spectral_kpoints_mp_grid` only), mp_spacing (`spectral_kpoints_mp_spacing` only), mp_offset (`spectral_kpoints_mp_offset` only — no mutex, companion to mp_grid/mp_spacing). Count set fields across ALL groups; if ≥2 total set → error.
- `FromCellFile` uses each type's own `from_cells` (which handles BLOCK_NAME/KEY_NAME + BLOCK_ALIASES/KEY_ALIASES — no dual-name logic needed at the group level)

**Register in `cell/mod.rs`:** `pub mod spectral_params;`

---

### TASK-A-3: Create OpticsMagresParams group

**Kind:** lib-tdd
**Goal:** Create `cell/optics_magres_params.rs` with `OpticsMagresParams` for optics and magres k-point lists.

**New file:** `castep_cell_io/src/cell/optics_magres_params.rs`

**Struct fields** (all `Option<T>`):
- `optics_kpoints_list: Option<OpticsKpointsList>`
- `magres_kpoints_list: Option<MagresKpointsList>`

**Pattern**: Same as GeneralParams — `Default`, `Builder`, `FromCellFile`, `ToCellFile`, `validate()` (pass-through — no mutex between optics and magres).

**Success Criteria:**
- `validate()` passes for any combination (including both set, both None)
- Both fields independently parse/serialize via their own `from_cells`/`to_cell`

**Register in `cell/mod.rs`:** `pub mod optics_magres_params;`

---

### TASK-A-4: Create SymmetryParams group

**Kind:** lib-tdd
**Goal:** Create `cell/symmetry_params.rs` with `SymmetryParams` for symmetry operations and tolerances.

**New file:** `castep_cell_io/src/cell/symmetry_params.rs`

**Struct fields** (all `Option<T>`):
- `symmetry_ops: Option<SymmetryOps>`
- `symmetry_generate: Option<SymmetryGenerate>`
- `symmetry_tol: Option<SymmetryTol>`

**Success Criteria:**
- `SymmetryParams::from_cell_file(parse("fixture-cell-forsterite"))` → `symmetry_ops.is_some()`, `symmetry_generate.is_none()` (Source: Mg2SiO4_Cr_1.cell, line 45 — SYMMETRY_OPS block present)
- `validate()` rejects both ops + generate set: `SymmetryParams { symmetry_ops: Some(...), symmetry_generate: Some(...), ..Default::default() }.validate()` → `Err(...)` (Source: supplemented_official_docs/cell_keywords.md, footnote 6)
- `symmetry_tol` is independent — no mutex with ops/generate

**Register in `cell/mod.rs`:** `pub mod symmetry_params;`

---

### TASK-A-5: Create ConstraintsParams group

**Kind:** lib-tdd
**Goal:** Create `cell/constraints_params.rs` with `ConstraintsParams` for ionic/cell movement constraints.

**New file:** `castep_cell_io/src/cell/constraints_params.rs`

**Struct fields** (all `Option<T>`):
- `fix_com: Option<FixCOM>`
- `ionic_constraints: Option<IonicConstraints>`
- `nonlinear_constraints: Option<NonlinearConstraints>`
- `fix_all_ions: Option<FixAllIons>`
- `fix_all_cell: Option<FixAllCell>`
- `cell_constraints: Option<CellConstraints>`
- `fix_vol: Option<FixVOL>`

**Success Criteria:**
- `ConstraintsParams::from_cell_file(parse("fixture-cell-forsterite"))` → `fix_com.is_some()`, `ionic_constraints.is_some()`, `cell_constraints.is_some()` (Source: Mg2SiO4_Cr_1.cell, lines 56-63)
- `validate()`: if `cell_constraints.is_some()`, sets `fix_all_cell = None` and emits warning via `eprintln!`. This supersedes `fix_all_cell` per PLAN.md decision. (Source: PLAN.md, line 4 — "cell_constraints supersedes fix_all_cell")
- `validate()` on self with only `cell_constraints` set passes: `Ok(self)` (with fix_all_cell already None)

**Register in `cell/mod.rs`:** `pub mod constraints_params;`

---

### TASK-A-6: Create ExternalFieldParams group

**Kind:** lib-tdd
**Goal:** Create `cell/external_field_params.rs` with `ExternalFieldParams` for external electric field and pressure.

**New file:** `castep_cell_io/src/cell/external_field_params.rs`

**Struct fields** (all `Option<T>`):
- `external_efield: Option<ExternalEfield>`
- `external_pressure: Option<ExternalPressure>`

**Pattern**: Standard group. `validate()` is pass-through (no intra-group mutex).

**Success Criteria:**
- `ExternalFieldParams::from_cell_file(parse("fixture-cell-forsterite"))` → `external_efield.is_some()` (Source: Mg2SiO4_Cr_1.cell, line 65)
- Both fields optional, independently parse/serialize

**Register in `cell/mod.rs`:** `pub mod external_field_params;`

---

### TASK-A-7: Create SpeciesParams group

**Kind:** lib-tdd
**Goal:** Create `cell/species_params.rs` with `SpeciesParams` for species properties blocks.

**New file:** `castep_cell_io/src/cell/species_params.rs`

**Struct fields** (all `Option<T>`):
- `species_mass: Option<SpeciesMass>`
- `species_pot: Option<SpeciesPot>`
- `species_lcao_states: Option<SpeciesLcaoStates>`
- `species_q: Option<SpeciesQ>`
- `hubbard_u: Option<HubbardU>`
- `sedc_custom_params: Option<SedcCustomParams>`

**Success Criteria:**
- `SpeciesParams::from_cell_file(parse("fixture-cell-forsterite"))` → `species_mass.is_some()`, `species_pot.is_some()`, `species_lcao_states.is_some()` (Source: Mg2SiO4_Cr_1.cell, lines 69-88)
- `species_q`, `hubbard_u`, `sedc_custom_params` are `None` for this fixture (not present in file)
- `validate()` is pass-through (no intra-group mutex — all independent blocks)

**Register in `cell/mod.rs`:** `pub mod species_params;`

---

### TASK-A-8: Create PhononParams group

**Kind:** lib-tdd
**Goal:** Create `cell/phonon_params.rs` with `PhononParams` for coarse phonon settings.

**New file:** `castep_cell_io/src/cell/phonon_params.rs`

**Struct fields** (all `Option<T>`):
- `phonon_kpoint_list: Option<PhononKpointList>`
- `phonon_kpoint_path: Option<PhononKpointPath>`
- `phonon_kpoint_path_spacing: Option<PhononKpointPathSpacing>`
- `phonon_kpoints_mp_grid: Option<PhononKpointsMpGrid>`
- `phonon_kpoints_mp_spacing: Option<PhononKpointsMpSpacing>`
- `phonon_kpoints_mp_offset: Option<PhononKpointsMpOffset>`
- `phonon_gamma_directions: Option<PhononGammaDirections>`
- `phonon_supercell_matrix: Option<PhononSupercellMatrix>`
- `supercell_kpoint_list: Option<SupercellKpointListCastep>`

**Success Criteria:**
- `validate()` rejects ≥2 of {path, list, mp_grid, mp_spacing}: `PhononParams { phonon_kpoint_path: Some(...), phonon_kpoints_mp_grid: Some(...), ..Default::default() }.validate()` → `Err(...)` (Source: supplemented_official_docs/cell_keywords.md, footnote 5 — only one phonon k-point method)
- `phonon_kpoints_mp_offset` has no mutex constraint (companion to mp_grid/mp_spacing, like KpointsParams)
- `phonon_gamma_directions`, `phonon_supercell_matrix`, `supercell_kpoint_list` are independent — no mutex

**Register in `cell/mod.rs`:** `pub mod phonon_params;`

---

### TASK-A-9: Create PhononFineParams group

**Kind:** lib-tdd
**Goal:** Create `cell/phonon_fine_params.rs` with `PhononFineParams` for fine phonon settings.

**New file:** `castep_cell_io/src/cell/phonon_fine_params.rs`

**Struct fields** (all `Option<T>`):
- `phonon_fine_kpoint_list: Option<PhononFineKpointList>`
- `phonon_fine_kpoint_path: Option<PhononFineKpointPath>`
- `phonon_fine_kpoint_path_spacing: Option<PhononFineKpointPathSpacing>`
- `phonon_fine_kpoints_mp_grid: Option<PhononFineKpointsMpGrid>`
- `phonon_fine_kpoints_mp_spacing: Option<PhononFineKpointsMpSpacing>`
- `phonon_fine_kpoints_mp_offset: Option<PhononFineKpointsMpOffset>`

**Success Criteria:**
- `validate()` rejects ≥2 of {path, list, mp_grid, mp_spacing} (same logic as PhononParams, per CASTEP spec — phonon fine k-points follow same mutex rules as coarse)
- `phonon_fine_kpoints_mp_offset` has no mutex constraint

**Register in `cell/mod.rs`:** `pub mod phonon_fine_params;`

---

### TASK-A-10: Create DynamicsParams group

**Kind:** lib-tdd
**Goal:** Create `cell/dynamics_params.rs` with `DynamicsParams` for MD dynamics (ionic velocities).

**New file:** `castep_cell_io/src/cell/dynamics_params.rs`

**Struct field** (all `Option<T>`):
- `ionic_velocities: Option<IonicVelocities>`

**Pattern**: Standard group. `validate()` is pass-through.

**Register in `cell/mod.rs`:** `pub mod dynamics_params;`

---

## Group B: Restructure CellDocument

### TASK-B-1: Replace flat fields with group fields in CellDocument

**Kind:** lib-tdd
**Goal:** Replace all 41 optional flat fields in `CellDocument` with 10 group fields. Required fields (`lattice`, `positions`) stay top-level.

**Files modified:** `castep_cell_io/src/cell_document.rs`

**After migration, CellDocument struct:**
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

All group fields use `#[builder(default)]` (bon fills in `KpointsParams::default()` which is all-None).

**Changes to `from_cell_file`:**
- Instead of 41 individual `maybe_*` calls, call each group's `FromCellFile::from_cell_file(tokens)` once, then `.maybe_kpoints(...)` etc.
- Required fields (lattice, positions) parsing unchanged.
- Remove 41 flat field `.maybe_*` builder calls.

**Changes to `to_cell_file`:**
- Instead of 41 `if let Some` guards, call each group's `to_cell_file()` and `cells.extend(...)`.
- Required fields serialization unchanged.

**Changes to builder `build()`:**
- Remove inline mutual-exclusion checks for fields now validated by group `validate()`. Keep only top-level checks.
- Call each group's `validate()` with `?` propagation converted from `String` to `Error::Message(...)`.
- Validate required fields (lattice, positions) still at top level.

**Success Criteria:**
- `CellDocument::from_cell_file(parse("fixture-cell-forsterite"))` produces a document where:
  - `lattice` is `Lattice::Cart(...)` (Source: Mg2SiO4_Cr_1.cell, line 1 — LATTICE_CART)
  - `positions` is `Positions::Frac(...)` with 28 entries (Source: Mg2SiO4_Cr_1.cell, lines 8-37)
  - `kpoints.kpoints_list.is_some()` (3 k-points) (Source: Mg2SiO4_Cr_1.cell, lines 39-43)
  - `symmetry.symmetry_ops.is_some()` (2 ops) (Source: Mg2SiO4_Cr_1.cell, lines 45-54)
  - `constraints.cell_constraints.is_some()` (2 constraints) (Source: Mg2SiO4_Cr_1.cell, lines 56-59)
  - `constraints.fix_com.is_some()` (false) (Source: Mg2SiO4_Cr_1.cell, line 61)
  - `constraints.ionic_constraints.is_some()` (empty block) (Source: Mg2SiO4_Cr_1.cell, line 62)
  - `external_fields.external_efield.is_some()` (Source: Mg2SiO4_Cr_1.cell, line 65)
  - `species.species_mass.is_some()` (4 entries) (Source: Mg2SiO4_Cr_1.cell, lines 69-74)
  - `species.species_pot.is_some()` (4 entries) (Source: Mg2SiO4_Cr_1.cell, lines 76-81)
  - `species.species_lcao_states.is_some()` (4 entries) (Source: Mg2SiO4_Cr_1.cell, lines 83-88)
- `CellDocument::to_cell_file()` on the parsed fixture emits identically structured output (same blocks in same order) as the pre-migration code.
- All 1080 pre-existing tests pass after the migration.

---

## Group C: Update imports and exports

### TASK-C-1: Update cell/mod.rs with new group modules

**Kind:** direct
**Goal:** Add `pub mod` declarations for all 10 new group modules in `cell/mod.rs`.

**File:** `castep_cell_io/src/cell/mod.rs`

**Changes:** Add these `pub mod` lines:
```rust
pub mod kpoints_params;
pub mod spectral_params;
pub mod optics_magres_params;
pub mod symmetry_params;
pub mod constraints_params;
pub mod external_field_params;
pub mod species_params;
pub mod phonon_params;
pub mod phonon_fine_params;
pub mod dynamics_params;
```

**Success Criteria:**
- `cargo check -p castep-cell-io` compiles successfully
- All group types are accessible via `castep_cell_io::cell::*` path

---

### TASK-C-2: Update imports in cell_document.rs

**Kind:** direct
**Goal:** Add `use` imports for all new group structs in `cell_document.rs`, remove imports for flat fields no longer directly referenced.

**File:** `castep_cell_io/src/cell_document.rs`

**Success Criteria:**
- `cargo check -p castep-cell-io` compiles successfully
- No unused import warnings from `cargo clippy`

---

## Group D: Tests and Verification

### TASK-D-1: Implement fixture-anchored tests for all three fixtures

**Kind:** lib-tdd
**Goal:** Write tests that read each real fixture file and assert concrete values against the group-based CellDocument. Replace the empty-input `#[ignore]`d test.

**File:** `castep_cell_io/src/cell_document.rs` (tests module)

**Success Criteria:**
- **Test for Mg2SiO4_Cr_1.cell** (`test_parse_forsterite`):
  - Reads from `tests/fixtures/Mg2SiO4_Cr_1.cell`
  - `doc.lattice` is `Cart` with vectors: a ≈ (10.183, 0, 0) in bohr, b ≈ (0, 5.970, 0), c ≈ (0, 0, 4.751) (Source: Mg2SiO4_Cr_1.cell, lines 1-6)
  - `doc.positions` is `Frac` with 28 entries, O=16, Mg=6, Si=4, Cr=2 (Source: Mg2SiO4_Cr_1.cell, lines 8-37)
  - `doc.kpoints.kpoints_list` has 3 entries (Source: Mg2SiO4_Cr_1.cell, lines 39-43)
  - `doc.symmetry.symmetry_ops` has 2 operations (Source: Mg2SiO4_Cr_1.cell, lines 45-54)
  - `doc.constraints.fix_com` is `Some(FixCOM { enabled: false })` (Source: Mg2SiO4_Cr_1.cell, line 61)
  - `doc.species.species_mass` → 4 entries: O, Mg, Si, Cr (Source: Mg2SiO4_Cr_1.cell, lines 69-74)

- **Test for Fe2O3.cell** (`test_parse_fe2o3`):
  - Reads from `tests/fixtures/Fe2O3.cell`
  - `doc.constraints.fix_all_cell` is `Some(FixAllCell { .. })` (Source: Fe2O3.cell, line 48)
  - `doc.external_fields.external_pressure` is `Some(ExternalPressure { .. })` (Source: Fe2O3.cell, lines 58-62)
  - `doc.species.hubbard_u` is `Some(HubbardU { .. })` with 12 Fe entries (Source: Fe2O3.cell, lines 79-92)
  - `doc.lattice` is `Lattice::Cart(LatticeCart { a: [4.360, ..], b: [0, 5.035, 0], c: [0, 0, 13.72], .. })` (Source: Fe2O3.cell, lines 1-5 — truncated to 4dp tolerance)

- **Test for ZnO_LR.cell** (`test_parse_zno_lr`):
  - Reads from `tests/fixtures/ZnO_LR.cell`
  - `doc.symmetry.symmetry_ops` has 12 operations (Source: ZnO_LR.cell, lines 27-76)
  - `doc.constraints.cell_constraints` has 2 constraints (Source: ZnO_LR.cell, lines 78-81)
  - `doc.positions` is `Frac` with 4 entries: 2 O + 2 Zn (Source: ZnO_LR.cell, lines 7-11)

- Tests are NOT `#[ignore]`d
- No vacuous assertions — every assertion checks a concrete value, not just `is_some()`

---

### TASK-D-2: Add mutual-exclusion validation tests

**Kind:** lib-tdd
**Goal:** Add unit tests for each group's `validate()` method covering all mutual-exclusion rules from `cell_keywords.md`.

**Files:** Each `*_params.rs` file (inline tests)

**Success Criteria per group:**
- **KpointsParams**: 3 tests — (1) list + mp_grid → Err, (2) mp_grid + mp_spacing → Err, (3) list alone → Ok
- **SpectralParams**: 4 tests — (1) bs_kpoint_path + spectral_kpoint_path → Err, (2) bs_kpoints_list + spectral_kpoints_list → Err, (3) spectral_kpoints_mp_grid + spectral_kpoints_mp_spacing → Err, (4) single method alone → Ok
- **SymmetryParams**: 2 tests — (1) ops + generate → Err, (2) ops alone → Ok
- **PhononParams**: 3 tests — (1) path + list → Err, (2) path + mp_grid → Err, (3) path alone → Ok
- **PhononFineParams**: 3 tests — same pattern as PhononParams
- **ConstraintsParams**: 1 test — cell_constraints + fix_all_cell → fix_all_cell is None in output

---

### TASK-D-3: Run full test suite and clippy

**Kind:** direct
**Goal:** Verify no regressions. All previously passing tests must still pass.

**Success Criteria:**
- `cargo test -p castep-cell-io` — all tests pass (≥1080)
- `cargo test -p castep-cell-fmt` — all tests pass
- `cargo test --workspace` — all tests pass
- `cargo clippy --workspace -- -D warnings` — zero warnings
- The previously `#[ignore]`d `test_parse_mg2sio4_cell` now passes

---

---

## Group E: LatticeCart Row-Major Documentation

### TASK-E-1: Clarify LatticeCart row-major convention in doc comments

**Kind:** direct
**Goal:** `LatticeCart` already uses named fields `.a`, `.b`, `.c` (row-major is implicit). Add explicit doc comments confirming the convention: row 1 in the CASTEP file maps to field `.a` (lattice vector A), row 2 → `.b`, row 3 → `.c`. This prevents confusion when downstream users convert to matrix types.

**File:** `castep_cell_io/src/cell/lattice_param.rs`

**Changes:**
- In `LatticeCart` struct doc comment, explicitly state: "Row 1 = lattice vector A (.a), Row 2 = lattice vector B (.b), Row 3 = lattice vector C (.c)"
- In `LatticeCart::from_block_rows()` doc, note the row mapping convention
- In `LatticeCart::to_cell()` doc, note the row-major emission order

**Success Criteria:**
- Doc comment on `LatticeCart` struct explicitly states which row maps to which field
- No code changes — comments only
- `cargo doc -p castep-cell-io --no-deps` generates the updated docs without warnings

---

## Additional Fixture Verification

### TASK-E-2: Verify new fixtures parse correctly with current code

**Kind:** lib-tdd
**Goal:** Before the migration, verify that `Fe2O3.cell` and `ZnO_LR.cell` parse correctly with the CURRENT (pre-migration) CellDocument. This establishes that the new fixture files are valid and known-good, so post-migration tests can compare against pre-migration output.

**File:** `castep_cell_io/src/cell_document.rs` (tests module, temporary — removed after migration)

**Success Criteria:**
- `parse::<CellDocument>("tests/fixtures/Fe2O3.cell")` succeeds with current code (1080-pre-migration baseline)
- `parse::<CellDocument>("tests/fixtures/ZnO_LR.cell")` succeeds with current code

---

## Deferred: Unit Round-Trip Consistency

### DISCOVERED: All unit enums emit CellValue::String instead of CellValue::Str

**Finding**: Every unit `ToCellValue` impl emits `CellValue::String(...)` (owned) but the
corresponding `from_cell_value()` uses `value_as_str()` which only accepts `CellValue::Str`
(borrowed). This breaks the direct IR round-trip (`to_cell()` → `from_block_rows()`) for any
block type that includes a unit field, even though the full text round-trip (file → parse →
format → file) works because the formatter outputs the string and the parser re-reads it.

**Affected files** (all under `castep_cell_io/src/units/`):

| File | Emits | Reads via | Round-trip |
|------|-------|-----------|------------|
| `length_units.rs` | `String` | `value_as_str` | ❌ (FIXED) |
| `energy_units.rs` | `String` | `value_as_str` | ❌ |
| `force_units.rs` | `String` | `value_as_str` | ❌ |
| `frequency_unit.rs` | `String` | `value_as_str` | ❌ |
| `pressure_unit.rs` | `String` | `value_as_str` | ❌ |
| `velocity_unit.rs` | `String` | `value_as_str` | ❌ |
| `inv_length_units.rs` | `String` | `value_as_str` | ❌ |
| `mass_units.rs` | `String` | `value_as_str` | ❌ |
| `temperature_unit.rs` | `String` | `value_as_str` | ❌ |
| `time_unit.rs` | `String` | `value_as_str` | ❌ |
| `volume_unit.rs` | `String` | `value_as_str` | ❌ |
| `efield_units.rs` | `String` | `value_as_str` | ❌ |
| `force_constant_unit.rs` | `String` | `value_as_string` | ✅ (reads both) |
| `quadrupole_moment_units.rs` | `String` | `value_as_string` | ✅ (reads both) |

**Fix**: All static-string unit enums should emit `CellValue::Str(...)` (static `'a` str)
instead of `CellValue::String(...)` (owned). 12 of 14 need the fix; `length_units.rs` is
already fixed. Update corresponding test assertions that check for `CellValue::String`.

**Filed as**: Clean-up task for implementation after CellDocument migration completes.

---

## Task Dependency Graph

```
Group A (all create-* tasks — independent, can parallelize)
  ├── TASK-A-1 (KpointsParams)
  ├── TASK-A-2 (SpectralParams)
  ├── TASK-A-3 (OpticsMagresParams)
  ├── TASK-A-4 (SymmetryParams)
  ├── TASK-A-5 (ConstraintsParams)
  ├── TASK-A-6 (ExternalFieldParams)
  ├── TASK-A-7 (SpeciesParams)
  ├── TASK-A-8 (PhononParams)
  ├── TASK-A-9 (PhononFineParams)
  └── TASK-A-10 (DynamicsParams)
       │
       v (all must complete before B-1)
Group B
  └── TASK-B-1 (Restructure CellDocument)
       │
       v (B-1 must complete before C tasks)
Group C
  ├── TASK-C-1 (Update cell/mod.rs)  ← must run after all A tasks (needs modules to exist)
  └── TASK-C-2 (Update imports)     ← must run after B-1
       │
       v (C tasks must complete before D)
Group D
  ├── TASK-D-1 (Fixture-anchored tests)   ← after B-1 + C-1
  ├── TASK-D-2 (Validation tests)         ← after A tasks
  └── TASK-D-3 (Full suite)               ← after all above
Group E
  ├── TASK-E-1 (LatticeCart docs)         ← independent, can run any time
  └── TASK-E-2 (Verify new fixtures)      ← runs before migration (pre-migration baseline)
```

## Implementation Order

1. **TASK-E-2** (pre-migration): Verify new fixture files parse correctly with current code.
2. **Group A** (all 10 in parallel or sequentially): Create group struct files with `FromCellFile`, `ToCellFile`, `validate()`. Each is a standalone file with no dependencies between groups.
3. **TASK-C-1**: Register all 10 new modules in `cell/mod.rs`.
4. **TASK-B-1**: Rewrite `CellDocument` to use group fields instead of 41 flat fields.
5. **TASK-C-2**: Clean up imports in `cell_document.rs`.
6. **TASK-D-2**: Add validation unit tests for each group.
7. **TASK-D-1**: Write fixture-anchored tests for all three fixtures.
8. **TASK-D-3**: Full test suite + clippy validation.
9. **TASK-E-1**: LatticeCart doc comments (any time after baseline, but after D-3 for clippy check).
