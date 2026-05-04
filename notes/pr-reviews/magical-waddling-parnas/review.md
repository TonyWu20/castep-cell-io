# Review: Add Mutual Exclusion Validation to CellDocument Builder

**Index**: `notes/directions/magical-waddling-parnas/directions-index.json`
**Branch**: `magical-waddling-parnas`
**Base**: `main`
**Reviewed**: 2026-05-05

## Summary

**PASSED** — All three task groups (TASK-1, TASK-2, TASK-3) are fully and correctly implemented. No blocking issues found.

Build and tests pass cleanly:
- `cargo check -p castep_cell_io` — clean
- `cargo test -p castep_cell_io` — 1081 unit tests, 1 integration test, 14 doc-tests all pass
- `cargo clippy -- -A dead_code` — clean, zero warnings

## Per-Task Results

### TASK-1: Convert CellDocument builder to fallible with 4 mutual-exclusion validation rules
- **Status**: ✓ Passed
- **Diff validation**: All 11 sub-items fully implemented: `finish_fn` pattern correctly applied, `build()` calls `build_internal()` then runs 4 validation rules using the specified counting idiom, correct fields in each rule group, offsets excluded, 4 pub use re-exports in correct mod.rs files, all 10 tests present and passing, existing `#[ignore]` test preserved, `#[allow(clippy::duplicated_attributes)]` preserved.
- **Strategic review**: No concerns. The TDD pattern is followed correctly with comprehensive test coverage.

### TASK-2: Convert from_cell_file to builder pattern with BS_ duplication guards
- **Status**: ✓ Passed
- **Diff validation**: All 9 checks pass: lattice mutual-exclusion preserved, BS_ parsing guarded behind `spectral_kpoint_path.is_some()`/`spectral_kpoints_list.is_some()` checks, spectral parsed before BS (order preserved), struct literal replaced with builder chain, required fields use plain setters, all 48 optional fields use `maybe_*` setters in correct order, `.build()` return handled as trailing expression.
- **Strategic review**: No concerns. The BS_ guard pattern correctly prevents double-population while preserving the D2 alias absorption.

### TASK-3: Add integration test for k-point mutual exclusion validation
- **Status**: ✓ Passed
- **Diff validation**: All requirements met: file at correct path, minimal .cell with both KPOINTS_LIST and KPOINT_MP_GRID, imports correct, test calls `parse::<CellDocument>(input)` and asserts `result.is_err()`, no scope creep.
- **Strategic review**: Integration test covers ^3 (kpoints) rule through parse pipeline. Other rules only tested at unit level — deferred for future phase.

## Issues Found

**No blocking issues.** Three items identified for future phases (see deferred.md):

| ID | Severity | Description | Recommendation |
|----|----------|-------------|----------------|
| D1 | Enhancement | Integration tests only cover ^3 rule; spectral ^4, phonon ^5, symmetry ^6 untested end-to-end | Add integration tests in future phase |
| D2 | Enhancement | Phonon fine-variant fields have no mutual-exclusion rules | Validate CASTEP semantics and add rules if needed |
| D3 | Nit | Builder error messages use Rust field names, lattice parse error uses CASTEP block names | Consider unifying naming convention |

## Deferred Items

See `deferred.md` for details.

## Next Steps

- Branch is ready to merge into `main`
- Deferred items can be addressed in a follow-up phase
