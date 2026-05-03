# Phase 1 Task Checklist Review

Review of `notes/directions/phase-1/draft-directions.json` against the actual codebase.

---

## Per-Task Analysis

### TASK-G1-1 (direct) — Add `pub use CellConstraints` to constraints/mod.rs

| Criterion | Verdict | Notes |
|-----------|---------|-------|
| Goal clarity | CLEAR | One-line re-export fix, clearly described |
| Files in scope | CORRECT | Only `constraints/mod.rs` |
| Implementation detail | SUFFICIENT | Exact line number range, exact insertion point between existing pub uses |
| New types/traits | N/A | No new types |
| Dependencies | CORRECT | No dependencies declared |
| Acceptance | SUFFICIENT | `cargo check` + `cargo test cell_constraints` |
| Wiring checklist | CORRECT | One item, accurate |

**Verdict: CLEAR**

---

### TASK-G1-2 (direct) — Add KEY_ALIASES to PhononKpointPathSpacing

| Criterion | Verdict | Notes |
|-----------|---------|-------|
| Goal clarity | CLEAR | Add plural alias to existing type |
| Files in scope | CORRECT | Only `phonon_kpoint_path_spacing.rs` |
| Implementation detail | SUFFICIENT | Exact const value, pattern reference to KpointsMpSpacing/BsKpointPathSpacing |
| New types/traits | N/A | No new types |
| Dependencies | CORRECT | None declared |
| Acceptance | SUFFICIENT | `cargo check` + `cargo test phonon_kpoint_path_spacing` |
| Wiring checklist | CORRECT | Accurate |

The line-number reference "around line 48" is mildly fragile but the file is short (172 lines) and the pattern reference makes it findable.

**Verdict: CLEAR**

---

### TASK-G1-3 (direct) — Wire 6 orphan types into CellDocument

| Criterion | Verdict | Notes |
|-----------|---------|-------|
| Goal clarity | CLEAR | Wire 6 pre-existing but unconnected types |
| Files in scope | CORRECT | Only `cell_document.rs` |
| Implementation detail | SUFFICIENT | All imports, 6 struct fields with doc comments, parsing lines, serialization blocks, Ok() constructor fields listed explicitly |
| New types/traits | N/A | No new types |
| Dependencies | CORRECT | Depends on TASK-G1-1 (CellConstraints re-export) |
| Acceptance | SUFFICIENT | `cargo check` + `cargo test` (full suite) |
| Wiring checklist | COMPLETE | 7 items covering import, type_annotation, fn_call, all comprehensive |

**Minor observations:**
- The guidance adds `has_flag` to the import block in G1-3, but `has_flag` is not used until G3-3 for SymmetryGenerate. This is intentional pre-solving and is documented in the wiring_checklist. Not a problem, just worth noting.
- No integration tests exist to verify round-trip behavior of CellDocument with the new fields. This is a pre-existing gap acknowledged in the codebase-state document.

**Verdict: CLEAR**

---

### TASK-G2-1 (direct) — Add Lattice::Abc variant

| Criterion | Verdict | Notes |
|-----------|---------|-------|
| Goal clarity | CLEAR | Add new enum variant with mutual-exclusion parsing |
| Files in scope | CORRECT | Only `cell_document.rs` |
| Implementation detail | SUFFICIENT | Boolean checks for mutual exclusion, From impls, ToCell match arm, parse logic, error message all specified |
| New types/traits | Lattice::Abc enum variant and dual-lattice error | Well-specified |
| Dependencies | CORRECT | Depends on TASK-G1-3 (modifies same file) |
| Acceptance | SUFFICIENT | `cargo check` + `cargo test lattice` — the test filter may yield 0 tests if no test names match "lattice", but `cargo check` catches compilation errors |
| Wiring checklist | COMPLETE | 6 items covering import, variant, From impls, ToCell, mutual exclusion, parsing |

**Verdict: CLEAR**

---

### TASK-G3-1 (lib-tdd) — Create KpointsMpOffset

| Criterion | Verdict | Notes |
|-----------|---------|-------|
| Goal clarity | CLEAR | New key-value newtype [f64;3], pattern: KpointsMpGrid |
| Files in scope | CORRECT | 2 files: new source + mod.rs registration |
| Implementation detail | SUFFICIENT | Struct definition, 4 trait impls, test structure, mod.rs changes all specified |
| tdd_interface | VALID | `test_code` creates a CellValue::Array of 3 floats, asserts `offset.0 == [0.25, 0.25, 0.25]`. Falsifiable. Signature matches `FromCellValue::from_cell_value`. |
| Dependencies | CORRECT | Depends on TASK-G2-1 |
| Acceptance | SUFFICIENT | `cargo check` + `cargo test kpoints_mp_offset` |
| Wiring checklist | COMPLETE | 2 items (mod + pub use) |

**Verdict: CLEAR**

---

### TASK-G3-2 (lib-tdd) — Create SymmetryGenerate flag type

| Criterion | Verdict | Notes |
|-----------|---------|-------|
| Goal clarity | CLEAR | Unit struct, parsed via has_flag (not FromKeyValue) |
| Files in scope | CORRECT | 2 files: new source + mod.rs registration |
| Implementation detail | SUFFICIENT | Struct derives, ToCell impl, note that no FromKeyValue needed, test structure all specified |
| tdd_interface | VALID | `test_code` creates `SymmetryGenerate`, calls `to_cell()`, matches `Cell::Flag("SYMMETRY_GENERATE")`. Falsifiable. Signature matches `ToCell::to_cell`. |
| Dependencies | CORRECT | Depends on TASK-G2-1 |
| Acceptance | SUFFICIENT | `cargo check` + `cargo test symmetry_generate` |
| Wiring checklist | COMPLETE | 2 items (mod + pub use) |

**Verdict: CLEAR**

---

### TASK-G3-3 (direct) — Wire KpointsMpOffset and SymmetryGenerate

| Criterion | Verdict | Notes |
|-----------|---------|-------|
| Goal clarity | CLEAR | Wire 2 G3 types |
| Files in scope | CORRECT | Only `cell_document.rs` |
| Implementation detail | SUFFICIENT | Imports, fields, parsing (FromKeyValue::from_cells + has_flag), serialization, constructor bindings |
| Dependencies | CORRECT | Depends on G3-1 + G3-2 |
| Acceptance | SUFFICIENT | `cargo check` + `cargo test` |
| Wiring checklist | COMPLETE | 5 items |

**Minor observation:** SymmetryGenerate serialization uses `if self.symmetry_generate.is_some()` while existing code for FixCOM uses `if let Some(_fc) = &self.fix_com`. Functionally equivalent, just slightly inconsistent style.

**Verdict: CLEAR**

---

### TASK-G4-1 (lib-tdd) — Create SpectralKpointPath + SpectralKpointPathEntry

| Criterion | Verdict | Notes |
|-----------|---------|-------|
| Goal clarity | CLEAR | Block type following BsKpointPath. bon::Builder required, BS_ backward compat aliases |
| Files in scope | CORRECT | 2 files: new source + mod.rs registration |
| Implementation detail | SUFFICIENT | Both entry and path types fully described, trait impls, BLOCK_ALIASES with BS_ variants |
| tdd_interface | VALID | `test_code` creates 2 rows, asserts `points.len() == 2`. Falsifiable. Signature matches `FromBlock::from_block_rows`. |
| Dependencies | CORRECT | Depends on TASK-G3-3 |
| Acceptance | SUFFICIENT | `cargo check` + `cargo test spectral_kpoint_path` |
| Wiring checklist | COMPLETE | 2 items (mod + pub use, covering both types) |

**Verdict: CLEAR**

---

### TASK-G4-2 (lib-tdd) — Create SpectralKpointsList

| Criterion | Verdict | Notes |
|-----------|---------|-------|
| Goal clarity | CLEAR | Block type following BSKpointList, reuses Kpoint |
| Files in scope | CORRECT | 2 files |
| Implementation detail | SUFFICIENT | Struct definition, trait impls, BLOCK_ALIASES with BS_ variants, import pattern for Kpoint |
| tdd_interface | VALID | `test_code` creates 2 rows with weights, asserts `kpts.len() == 2` and `kpts[0].weight == 0.25`. Falsifiable. |
| Dependencies | CORRECT | Depends on G4-1 (sequential mod.rs edits) |
| Acceptance | SUFFICIENT | `cargo check` + `cargo test spectral_kpoints_list` |
| Wiring checklist | COMPLETE | 2 items |

**Verdict: CLEAR**

---

### TASK-G4-3 (lib-tdd) — Create SpectralKpointPathSpacing

| Criterion | Verdict | Notes |
|-----------|---------|-------|
| Goal clarity | CLEAR | Key-value spacing type with BS_ aliases |
| Files in scope | CORRECT | 2 files |
| Implementation detail | SUFFICIENT | 2-field struct, trait impls, KEY_ALIASES with BS_ variants |
| tdd_interface | VALID | `test_code` creates Array [0.05, "1/ang"], asserts `value == 0.05` and `unit.is_some()`. Falsifiable. |
| Dependencies | CORRECT | Depends on G4-2 |
| Acceptance | SUFFICIENT | `cargo check` + `cargo test spectral_kpoint_path_spacing` |
| Wiring checklist | COMPLETE | 2 items |

**Verdict: CLEAR**

---

### TASK-G4-4 (lib-tdd) — Create SpectralKpointsMpGrid

| Criterion | Verdict | Notes |
|-----------|---------|-------|
| Goal clarity | CLEAR | Key-value newtype [u32;3], no BS_ aliases (net-new) |
| Files in scope | CORRECT | 2 files |
| Implementation detail | SUFFICIENT | Pattern: KpointsMpGrid, KEY_ALIASES only KPOINTS plural |
| tdd_interface | VALID | `test_code` creates [3,4,6] u32 array, asserts `grid.0 == [3, 4, 6]`. Falsifiable. |
| Dependencies | CORRECT | Depends on G4-3 |
| Acceptance | SUFFICIENT | `cargo check` + `cargo test spectral_kpoints_mp_grid` |
| Wiring checklist | COMPLETE | 2 items |

**Verdict: CLEAR**

---

### TASK-G4-5 (lib-tdd) — Create SpectralKpointsMpSpacing

| Criterion | Verdict | Notes |
|-----------|---------|-------|
| Goal clarity | CLEAR | Key-value spacing type, no BS_ aliases |
| Files in scope | CORRECT | 2 files |
| Implementation detail | SUFFICIENT | Pattern: KpointsMpSpacing |
| tdd_interface | VALID | `test_code` creates scalar 0.05, asserts `value == 0.05` and `unit == None`. Falsifiable. |
| Dependencies | CORRECT | Depends on G4-4 |
| Acceptance | SUFFICIENT | `cargo check` + `cargo test spectral_kpoints_mp_spacing` |
| Wiring checklist | COMPLETE | 2 items |

**Verdict: CLEAR**

---

### TASK-G4-6 (lib-tdd) — Create SpectralKpointsMpOffset

| Criterion | Verdict | Notes |
|-----------|---------|-------|
| Goal clarity | CLEAR | Key-value newtype [f64;3], no BS_ aliases |
| Files in scope | CORRECT | 2 files |
| Implementation detail | SUFFICIENT | Pattern: KpointsMpOffset (G3-1) |
| tdd_interface | VALID | `test_code` creates [0.0, 0.5, 0.5] f64 array, asserts `offset.0 == [0.0, 0.5, 0.5]`. Falsifiable. |
| Dependencies | CORRECT | Depends on G4-5 |
| Acceptance | SUFFICIENT | `cargo check` + `cargo test spectral_kpoints_mp_offset` |
| Wiring checklist | COMPLETE | 2 items |

**Verdict: CLEAR**

---

### TASK-G4-7 (direct) — Wire 6 spectral types into CellDocument

| Criterion | Verdict | Notes |
|-----------|---------|-------|
| Goal clarity | CLEAR | Wire all 6 spectral types with ordering caveat (spectral before BS_) |
| Files in scope | CORRECT | Only `cell_document.rs` |
| Implementation detail | SUFFICIENT | Imports, 6 struct fields, parsing with explicit find_block_any for block types and from_cells for key-value types, 6 serialization blocks, constructor bindings. CRITICAL ordering guidance included. |
| Dependencies | CORRECT | Depends on G4-6 (last type creation in group) |
| Acceptance | SUFFICIENT | `cargo check` + `cargo test` |
| Wiring checklist | COMPLETE | 6 items |

**Minor observation:** The wiring_checklist first item says "All 7 spectral types imported" but only 6 types are wired as fields. The 7th imported item is `SpectralKpointPathEntry` which is not used directly in cell_document.rs (it's consumed by SpectralKpointPath internally). The import of SpectralKpointPathEntry is unnecessary at this level.

**Verdict: CLEAR** (the unnecessary import is harmless)

---

### TASK-G5-1 (lib-tdd) — Create PhononKpointsMpGrid

| Criterion | Verdict | Notes |
|-----------|---------|-------|
| Goal clarity | CLEAR | Newtype [u32;3] with PHONON_ prefix |
| Files in scope | CORRECT | 2 files: new source + mod.rs |
| Implementation detail | SUFFICIENT | Pattern: KpointsMpGrid |
| tdd_interface | VALID | `test_code` creates [4,4,4] u32 array, asserts `grid.0 == [4, 4, 4]`. Falsifiable. |
| Dependencies | CORRECT | Depends on G4-7 (last wiring before G5) |
| Acceptance | SUFFICIENT | `cargo check` + `cargo test phonon_kpoints_mp_grid` |
| Wiring checklist | COMPLETE | 2 items |

**Verdict: CLEAR**

---

### TASK-G5-2 (lib-tdd) — Create PhononKpointsMpSpacing

| Criterion | Verdict | Notes |
|-----------|---------|-------|
| Goal clarity | CLEAR | Spacing type with PHONON_ prefix |
| Files in scope | CORRECT | 2 files |
| Implementation detail | SUFFICIENT | Pattern: KpointsMpSpacing |
| tdd_interface | VALID | `test_code` creates scalar 0.1, asserts `value == 0.1` and `unit == None`. Falsifiable. |
| Dependencies | CORRECT | Depends on G5-1 |
| Acceptance | SUFFICIENT | `cargo check` + `cargo test phonon_kpoints_mp_spacing` |
| Wiring checklist | COMPLETE | 2 items |

**Verdict: CLEAR**

---

### TASK-G5-3 (lib-tdd) — Create PhononKpointsMpOffset

| Criterion | Verdict | Notes |
|-----------|---------|-------|
| Goal clarity | CLEAR | Newtype [f64;3] with PHONON_ prefix |
| Files in scope | CORRECT | 2 files |
| Implementation detail | SUFFICIENT | Pattern: KpointsMpOffset (G3-1) |
| tdd_interface | VALID | `test_code` creates [0.0, 0.0, 0.0] f64 array, asserts `offset.0 == [0.0, 0.0, 0.0]`. Falsifiable (tests zero values which is a valid edge case). |
| Dependencies | CORRECT | Depends on G5-2 |
| Acceptance | SUFFICIENT | `cargo check` + `cargo test phonon_kpoints_mp_offset` |
| Wiring checklist | COMPLETE | 2 items |

**Verdict: CLEAR**

---

### TASK-G5-4 (lib-tdd) — Create PhononFineKpointPath

| Criterion | Verdict | Notes |
|-----------|---------|-------|
| Goal clarity | CLEAR | Block type reusing PhononKpointPathEntry, bon::Builder required |
| Files in scope | CORRECT | 2 files |
| Implementation detail | SUFFICIENT | Import path for entry type, BLOCK_ALIASES, bon::Builder, FromBlock, ToCell all specified |
| tdd_interface | VALID | `test_code` creates 2 rows, asserts `points.len() == 2` and `points[0].coord == [0.0, 0.0, 0.0]`. Falsifiable. |
| Dependencies | CORRECT | Depends on G5-3 |
| Acceptance | SUFFICIENT | `cargo check` + `cargo test phonon_fine_kpoint_path` |
| Wiring checklist | COMPLETE | 2 items |

The import `use super::phonon_kpoint_path::PhononKpointPathEntry;` is correct given the module layout (files are siblings in the `phonon/` directory, so `super` resolves to the `phonon` module).

**Verdict: CLEAR**

---

### TASK-G5-5 (lib-tdd) — Create PhononFineKpointPathSpacing

| Criterion | Verdict | Notes |
|-----------|---------|-------|
| Goal clarity | CLEAR | Spacing type with PHONON_FINE_ prefix |
| Files in scope | CORRECT | 2 files |
| Implementation detail | SUFFICIENT | Pattern: PhononKpointPathSpacing (existing, fixed in G1-2) |
| tdd_interface | VALID | `test_code` creates [0.05, "1/ang"] array, asserts `value == 0.05` and `unit.is_some()`. Falsifiable. |
| Dependencies | CORRECT | Depends on G5-4 |
| Acceptance | SUFFICIENT | `cargo check` + `cargo test phonon_fine_kpoint_path_spacing` |
| Wiring checklist | COMPLETE | 2 items |

**Verdict: CLEAR**

---

### TASK-G5-6 (lib-tdd) — Create PhononFineKpointsMpGrid

| Criterion | Verdict | Notes |
|-----------|---------|-------|
| Goal clarity | CLEAR | Newtype [u32;3] with PHONON_FINE_ prefix |
| Files in scope | CORRECT | 2 files |
| Implementation detail | SUFFICIENT | Pattern: KpointsMpGrid |
| tdd_interface | VALID | `test_code` creates [2,2,2] u32 array, asserts `grid.0 == [2, 2, 2]`. Falsifiable. |
| Dependencies | CORRECT | Depends on G5-5 |
| Acceptance | SUFFICIENT | `cargo check` + `cargo test phonon_fine_kpoints_mp_grid` |
| Wiring checklist | COMPLETE | 2 items |

**Verdict: CLEAR**

---

### TASK-G5-7 (lib-tdd) — Create PhononFineKpointsMpSpacing

| Criterion | Verdict | Notes |
|-----------|---------|-------|
| Goal clarity | CLEAR | Spacing type with PHONON_FINE_ prefix |
| Files in scope | CORRECT | 2 files |
| Implementation detail | SUFFICIENT | Pattern: KpointsMpSpacing |
| tdd_interface | VALID | `test_code` creates scalar 0.15, asserts `value == 0.15` and `unit == None`. Falsifiable. |
| Dependencies | CORRECT | Depends on G5-6 |
| Acceptance | SUFFICIENT | `cargo check` + `cargo test phonon_fine_kpoints_mp_spacing` |
| Wiring checklist | COMPLETE | 2 items |

**Verdict: CLEAR**

---

### TASK-G5-8 (lib-tdd) — Create PhononFineKpointsMpOffset

| Criterion | Verdict | Notes |
|-----------|---------|-------|
| Goal clarity | CLEAR | Newtype [f64;3] with PHONON_FINE_ prefix |
| Files in scope | CORRECT | 2 files |
| Implementation detail | SUFFICIENT | Pattern: KpointsMpOffset (G3-1) |
| tdd_interface | VALID | `test_code` creates [0.25, 0.25, 0.25] f64 array, asserts `offset.0 == [0.25, 0.25, 0.25]`. Falsifiable. |
| Dependencies | CORRECT | Depends on G5-7 |
| Acceptance | SUFFICIENT | `cargo check` + `cargo test phonon_fine_kpoints_mp_offset` |
| Wiring checklist | COMPLETE | 2 items |

**Verdict: CLEAR**

---

### TASK-G5-9 (direct) — Wire 8 phonon types into CellDocument

| Criterion | Verdict | Notes |
|-----------|---------|-------|
| Goal clarity | CLEAR | Wire all 8 phonon types into the phonon section |
| Files in scope | CORRECT | Only `cell_document.rs` |
| Implementation detail | SUFFICIENT | 8 imports, 8 fields, 7 from_cells + 1 find_block_any for PhononFineKpointPath, 8 serialization blocks, 8 constructor bindings. All ordered within the phonon section after phonon_kpoint_path. |
| Dependencies | CORRECT | Depends on G5-8 (last type creation) |
| Acceptance | SUFFICIENT | `cargo check` + `cargo test` |
| Wiring checklist | COMPLETE | 5 items covering imports, fields, 7+1 parsing calls, 8 serialization blocks, 8 constructor bindings |

**Verdict: CLEAR**

---

## Cross-Cutting Observations

### Issues Found (None are blockers, but worth noting)

1. **`has_flag` import in G1-3 before it is used (G3-3)**: The guidance adds `has_flag` to the `castep_cell_fmt::query` import in TASK-G1-3, but this function is only first used in TASK-G3-3 for parsing SymmetryGenerate. The wiring_checklist explicitly calls this out, so it is intentional and documented. No ambiguity; the import is harmless and the code compiles regardless.

2. **SpectralKpointPathEntry unnecessarily imported in G4-7**: The cell_document.rs import list adds `SpectralKpointPathEntry` alongside the 6 wired spectral types, but this entry type is consumed internally by `SpectralKpointPath` and never appears as a standalone field or type annotation in cell_document.rs. The import compiles but is unused. Minor.

3. **SymmetryGenerate serialization style mismatch in G3-3**: Uses `if self.symmetry_generate.is_some()` while existing code uses `if let Some(_fc) = &self.fix_com`. Both compile to the same thing, but the pattern is inconsistent with the surrounding code.

4. **No integration tests for any wiring task**: The acceptance criteria for the 4 wiring tasks (G1-3, G3-3, G4-7, G5-9) are `cargo check` + `cargo test` only. There are no integration tests that validate round-trip parsing of a cell file containing the new fields. This is a pre-existing gap acknowledged in `codebase-state.md`. The directions are honest about this limitation.

### Contradictions and Ambiguity

**None found.** The architecture_notes, known_pitfalls, and individual task details are all internally consistent and aligned with the actual codebase.

### tdd_interface Quality Summary

All 16 `lib-tdd` tasks have `tdd_interface` blocks. Every `test_code` is a meaningful, falsifiable assertion (not `assert!(true)`). Every `signature` matches the trait method being tested. Every `expected_behavior` describes the parsing/behavior constraints:

- All test inputs use realistic `CellValue` structures (Array, Float, UInt, Str)
- All tests assert concrete field values
- No test is trivial (no empty bodies, no `assert!(true)`)
- Signatures match trait method signatures (e.g., `FromCellValue::from_cell_value`, `FromBlock::from_block_rows`, `ToCell::to_cell`)

### Dependency Chain

The dependency graph is:
```
G1-fix-gaps (G1-1, G1-2, G1-3)
  |
  v
G2-lattice-abc (G2-1)
  |
  v
G3-new-simple-types (G3-1, G3-2, G3-3)
  |
  v
G4-spectral-types (G4-1 .. G4-7)
  |
  v
G5-phonon-mp-fine (G5-1 .. G5-9)
```

Within each group, type-creation tasks that edit the same mod.rs file run sequentially (e.g., G4-1 depends on G4-0 being complete). Wiring tasks depend on the last type-creation task in the group. The chain is logical and conflict-free.

---

## Overall Verdict: Ready to Implement

All 22 tasks (4 direct fix/wiring + 2 G2 changes + 16 lib-tdd type creations) are clearly specified. The files_in_scope are correct and complete. The acceptance commands are sufficient for compile-time and unit-test validation. The wiring_checklist items are accurate. The 16 tdd_interface blocks all contain meaningful, falsifiable test code with matching signatures. The 5 minor observations above are style notes, not blockers. No task has ambiguous, underspecified, or contradictory guidance.
