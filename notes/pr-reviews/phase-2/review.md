# Review: CellDocument Group Substruct Migration (Phase 2)

**Tasks**: `notes/plans/phase-2/TASKS.md`
**Reviewed**: 2026-05-13
**Branch**: `cell-document-groups`

## Summary

**Overall: PASS — minor issues found, non-blocking**

Runtime verification: 1116 tests pass (0 failed), 14 doctests pass, `cargo clippy --workspace -D warnings` clean. The previously `#[ignore]`d test is replaced with a real fixture-anchored test. All three fixture files parse correctly against the new group-based CellDocument.

3 issues found (1 required, 2 recommended), 2 deferred items.

## Per-Task Results

### Group A: Create Cell Document Group Structs (TASK-A-1 through A-10)
- **Status**: ✓ All 10 tasks passed
- **Runtime verification**: All group structs compile, validate, parse, and serialize correctly
- **Diff validation**: Each file follows the prescribed patterns (bon Builder, FromCellFile with `.maybe_` chaining, ToCellFile with `if let Some`, `validate()` with mutex rules where applicable)
- **Strategic review**: Architecture cleanly mirrors ParamDocument pattern. No scope creep.

### TASK-B-1: Restructure CellDocument
- **Status**: ⚠ Passed with minor issue
- **Runtime verification**: Full round-trip works — all existing tests pass
- **Diff validation**: `build()` calls `validate()` on 6 of 10 groups, omitting optics_magres, external_fields, species, dynamics. These are pass-through today so no functional gap, but it's a maintenance trap (Issue #1).
- **Strategic review**: 41 flat fields → 10 group fields as specified. `from_cell_file()` and `to_cell_file()` correctly delegate to group implementations.

### TASK-C-1: Update cell/mod.rs
- **Status**: ✓ Passed
- **Diff validation**: All 10 `pub mod` declarations present

### TASK-C-2: Update imports in cell_document.rs
- **Status**: ✓ Passed
- **Diff validation**: Group types imported, flat field types removed where unused

### TASK-D-1: Fixture-anchored tests
- **Status**: ⚠ Passed with minor issue
- **Runtime verification**: All three fixture tests pass with concrete value assertions
- **Issue**: `test_parse_zno_lr_cell` does not assert on `doc.positions` (missing Frac + 4 entries check as specified in TASKS.md) — Issue #2

### TASK-D-2: Mutual-exclusion validation tests
- **Status**: ⚠ Passed with minor issue
- **Runtime verification**: All validation tests pass
- **Issue**: `kpoints_params.rs` missing the `mp_grid + mp_spacing → Err` inline test prescribed by TASKS.md — Issue #2 (same fix-tasks group)

### TASK-D-3: Full suite
- **Status**: ✓ Passed — 1116 tests, workspace tests, clippy all clean

### TASK-E-1: LatticeCart row-major docs
- **Status**: ✓ Passed — struct doc clearly states row-major convention with ASCII diagram

### TASK-E-2: Verify new fixtures (pre-migration)
- **Status**: ✓ Implicitly passed — all three fixtures parse correctly in post-migration tests

## Issues Found

### Issue #1 (Required): Missing validate() calls in CellDocument::build()

**Severity**: Minor (maintenance trap)
**File**: `castep_cell_io/src/cell_document.rs`, lines 291-301

The builder's `build()` method validates 6 of 10 groups:
- Validated: kpoints, spectral, symmetry, constraints, phonon, phonon_fine
- **Not validated**: optics_magres, external_fields, species, dynamics

The 4 omitted groups have pass-through `validate()` today, so there is no functional gap. However, if any of those groups later acquires real validation logic, the `CellDocument::builder()...build()` path will silently bypass it (the `from_cell_file()` path is fine since each group's `from_cell_file()` calls its own `validate()` internally).

**Recommendation**: Add the 4 missing validate() calls for consistency and future-proofing:
```rust
doc.optics_magres = doc.optics_magres.validate().map_err(|e| Error::Message(e.to_string()))?;
doc.external_fields = doc.external_fields.validate().map_err(|e| Error::Message(e.to_string()))?;
doc.species = doc.species.validate().map_err(|e| Error::Message(e.to_string()))?;
doc.dynamics = doc.dynamics.validate().map_err(|e| Error::Message(e.to_string()))?;
```

### Issue #2 (Recommended): Test coverage gaps

**Severity**: Minor
**Files**: `castep_cell_io/src/cell_document.rs`, `castep_cell_io/src/cell/kpoints_params.rs`

Two gaps in TASKS.md success criteria coverage:

1. **ZnO test missing positions assertion** (`cell_document.rs`): `test_parse_zno_lr_cell` does not assert that `doc.positions` is `Positions::Frac` with 4 entries (2 O + 2 Zn), as specified in TASKS.md D-1.

2. **KpointsParams missing mp_grid + mp_spacing test** (`kpoints_params.rs`): TASKS.md D-2 prescribes `mp_grid + mp_spacing → Err` but the inline tests have `mp_grid + offset → Ok` instead (different coverage target). The mp_grid + mp_spacing case is only implicitly covered by `build_rejects_all_three_kpoint_specs` in cell_document.rs.

### Issue #3 (Recommended): Library should not use eprintln!

**Severity**: Cosmetic
**File**: `castep_cell_io/src/cell/constraints_params.rs`, line 59

`ConstraintsParams::validate()` calls `eprintln!()` for the superseding warning. Library code should use `log::warn!` or return the warning via the Result type. Deferred to follow-up.

## Deferred Items

See also `deferred.md`.

1. **Unit enum CellValue::String vs CellValue::Str round-trip fix** (DISCOVERED in TASKS.md): 12 of 14 unit files need `CellValue::String`→`CellValue::Str` migration. Only `length_units.rs` is fixed. Independent of group migration.
2. **eprintln! in library code**: Migrate to `log::warn!` (Issue #3 above).
3. **Serialization order change**: The group field order changes block emission order vs pre-migration. Not functionally breaking but could affect diff-stability-dependent workflows.
