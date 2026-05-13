# Review: CellDocument Group Substruct Migration (Phase 2)

**Tasks**: `notes/plans/phase-2/TASKS.md`
**Reviewed**: 2026-05-13
**Branch**: `cell-document-groups`

## Summary

**Overall: PASS — all issues resolved**

Runtime verification: 1117 tests pass (0 failed), 14 doctests pass, `cargo clippy --workspace -D warnings` clean. The 3 fix tasks from `fix-tasks.md` are all verified and merged.

3 original issues found during initial review:
- **Issue #1 (Required)**: Missing validate() calls in `CellDocument::build()` → **FIXED** (FIX-1)
- **Issue #2 (Recommended)**: Test coverage gaps (ZnO positions + mp_grid/mp_spacing) → **FIXED** (FIX-2, FIX-3)
- **Issue #3 (Cosmetic)**: Library `eprintln!` usage → **DEFERRED** (out of scope for fix-tasks)

3 deferred items carried forward (unchanged).

## Per-Task Results

### Group A: Create Cell Document Group Structs (TASK-A-1 through A-10)
- **Status**: ✓ All 10 tasks passed
- **Runtime verification**: All group structs compile, validate, parse, and serialize correctly
- **Diff validation**: Each file follows the prescribed patterns (bon Builder, FromCellFile with `.maybe_` chaining, ToCellFile with `if let Some`, `validate()` with mutex rules where applicable)
- **Strategic review**: Architecture cleanly mirrors ParamDocument pattern. No scope creep.

### TASK-B-1: Restructure CellDocument
- **Status**: ✓ Passed (fix applied)
- **Runtime verification**: Full round-trip works — all existing tests pass
- **Diff validation**: `build()` now calls `validate()` on all 10 groups (6 pre-existing + 4 added by FIX-1). Maintenance trap closed.
- **Strategic review**: 41 flat fields → 10 group fields as specified. `from_cell_file()` and `to_cell_file()` correctly delegate to group implementations.

### TASK-C-1: Update cell/mod.rs
- **Status**: ✓ Passed
- **Diff validation**: All 10 `pub mod` declarations present

### TASK-C-2: Update imports in cell_document.rs
- **Status**: ✓ Passed
- **Diff validation**: Group types imported, flat field types removed where unused

### TASK-D-1: Fixture-anchored tests
- **Status**: ✓ Passed (fix applied)
- **Runtime verification**: All three fixture tests pass with concrete value assertions
- **Fix applied**: `test_parse_zno_lr_cell` now asserts `Positions::Frac` with 4 entries (FIX-2)

### TASK-D-2: Mutual-exclusion validation tests
- **Status**: ✓ Passed (fix applied)
- **Runtime verification**: All validation tests pass
- **Fix applied**: `test_validate_mp_grid_and_mp_spacing_err` added to `kpoints_params.rs` (FIX-3)

### TASK-D-3: Full suite
- **Status**: ✓ Passed — 1117 tests, workspace tests, clippy all clean

### TASK-E-1: LatticeCart row-major docs
- **Status**: ✓ Passed — struct doc clearly states row-major convention with ASCII diagram

### TASK-E-2: Verify new fixtures (pre-migration)
- **Status**: ✓ Implicitly passed — all three fixtures parse correctly in post-migration tests

## Fix Tasks Validation

### FIX-1: Add missing validate() calls in CellDocument::build()
- **Status**: ✓ Fully implemented as directed
- **Runtime verification**: Build, tests, clippy all pass
- **Diff validation**: 4 validate() calls added at `cell_document.rs:299-302` matching the spec exactly

### FIX-2: Add missing positions assertion in test_parse_zno_lr_cell
- **Status**: ✓ Implemented (corrective deviation)
- **Runtime verification**: Test passes against ZnO_LR fixture
- **Diff validation**: Uses `pos.positions.len()` (correct field name) instead of spec's erroneous `pos.ions.len()`

### FIX-3: Add mp_grid + mp_spacing validation test to KpointsParams
- **Status**: ✓ Fully implemented as directed
- **Runtime verification**: Test passes, validates mutual exclusion
- **Diff validation**: Test matches spec exactly — `KpointsMpGrid([2,2,2])` + `KpointsMpSpacing({0.05})` → `is_err()`

## Issues Found

All fixable issues from the initial review have been resolved. No new issues found.

### Remaining Issue #3 (Deferred): Library should not use eprintln!

**Severity**: Cosmetic
**File**: `castep_cell_io/src/cell/constraints_params.rs`, line 59

`ConstraintsParams::validate()` calls `eprintln!()` for the superseding warning. Library code should use `log::warn!` or return the warning via the Result type. Deferred to follow-up.

## Deferred Items

See `deferred.md`.
