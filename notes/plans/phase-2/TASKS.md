# TASKS.md — CellDocument Group Substruct Migration

**Phase**: phase-2
**Plan**: PLAN.md (Parts 3-5), DECISIONS.md
**Date**: 2026-05-13
**ODD Pattern**: `/Users/tony/programming/rust-development-pipeline/skills/drive-outcomes/references/odd-pattern.md`

## Declared Fixtures

| Alias | Path | Description |
|-------|------|-------------|
| `fixture-cell-forsterite` | `castep_cell_fmt/Mg2SiO4_Cr_1.cell` | Forsterite Cr-doped, 28 atoms, exercises 12 block types |

## Exploration Notes

**2026-05-13 — Pre-migration baseline**:
- 1080 tests pass in `castep-cell-io` (all library tests + doctests).
- One `#[ignore]`d test (`test_parse_mg2sio4_cell`) uses empty input `let input = "";` — fixture file exists but is not wired up. This test is rewritten in Task D-1.
- `Mg2SiO4_Cr_1.cell` fixture exercises 12 of ~43 CellDocument fields. Gap: 31 fields not exercised by existing fixture.
- LATTICE_CART parsed, POSITIONS_FRAC with 28 entries, KPOINTS_LIST with 3 k-points, SYMMETRY_OPS with 2 ops, CELL_CONSTRAINTS, FIX_COM, IONIC_CONSTRAINTS (empty), EXTERNAL_EFIELD, SPECIES_MASS, SPECIES_POT, SPECIES_LCAO_STATES.
- No SPECTRAL_, BS_, OPTICS_, MAGRES_, PHONON_, or velocity blocks in the fixture.

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

### TASK-D-1: Fix fixture-anchored test

**Kind:** lib-tdd
**Goal:** Rewrite `test_parse_mg2sio4_cell` to read the real fixture file and assert concrete values against the group-based CellDocument.

**File:** `castep_cell_io/src/cell_document.rs` (tests module)

**Success Criteria:**
- Test reads `Mg2SiO4_Cr_1.cell` from the filesystem (at `castep_cell_fmt/Mg2SiO4_Cr_1.cell`)
- Parses it as `CellDocument`
- Asserts concrete values matching the fixture:
  - `doc.lattice` is `Cart` with 3 vectors (lattice vector a ≈ (10.183, 0, 0) in bohr) (Source: Mg2SiO4_Cr_1.cell, lines 1-6)
  - `doc.positions` is `Frac` with 28 entries, `count_by_species` yields: O=16, Mg=6, Si=4, Cr=2 (Source: Mg2SiO4_Cr_1.cell, lines 8-37 — counting lines and species labels)
  - `doc.kpoints.kpoints_list` has 3 entries (Source: Mg2SiO4_Cr_1.cell, lines 39-43)
  - `doc.symmetry.symmetry_ops` has 2 operations (Source: Mg2SiO4_Cr_1.cell, lines 45-54)
  - `doc.constraints.fix_com` is `Some(FixCOM { enabled: false })` (Source: Mg2SiO4_Cr_1.cell, line 61)
- No vacuous assertions (no bare `is_some()` without value checks on the contained data)
- Test is NOT `#[ignore]`d

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
  ├── TASK-D-1 (Fix fixture test)   ← after B-1 + C-1
  ├── TASK-D-2 (Validation tests)   ← after A tasks
  └── TASK-D-3 (Full suite)         ← after all above
```

## Implementation Order

1. **Group A** (all 10 in parallel or sequentially): Create group struct files with `FromCellFile`, `ToCellFile`, `validate()`. Each is a standalone file with no dependencies between groups.
2. **Group C-1**: Register all 10 new modules in `cell/mod.rs`.
3. **Group B-1**: Rewrite `CellDocument` to use group fields instead of 41 flat fields.
4. **Group C-2**: Clean up imports in `cell_document.rs`.
5. **Group D-2**: Add validation unit tests for each group.
6. **Group D-1**: Fix the Mg2SiO4 fixture test.
7. **Group D-3**: Full test suite + clippy validation.
