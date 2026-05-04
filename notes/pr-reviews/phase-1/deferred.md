# Deferred Items — Phase 1

Items identified during review that are worth doing but out of scope for this phase. These are candidates for the next phase plan discussion.

## D1: Group CellDocument fields into sub-structs

**Concern**: After Phase 1, CellDocument has ~50 fields. This is maintainable but unwieldy.

**Recommendation**: Group fields into sub-structs (KpointsParams, SpectralParams, PhononParams) and flatten them into CellDocument as single fields. Each sub-struct gets its own parsing/serialization block. This reduces the CellDocument surface area significantly.

**Risk**: Low — additive refactor, no behavior change.

## D2: Mutual-exclusion validation between spectral and BS_ types

**Concern**: When a .cell file uses BS_KPOINT_PATH (backward compat alias), both SpectralKpointPath and BsKpointPath blocks get populated. This is currently documented but not validated.

**Recommendation**: Add validation that detects when both spectral and BS_ equivalents are present and emits a warning or error. This prevents ambiguous or conflicting k-point specifications.

**Risk**: Low — additive validation. Requires deciding whether to warn or error.

## D3: Integration test with full round-trip fixture

**Concern**: New types have strong unit tests but no end-to-end integration test that parses a .cell file containing the new keyword types and serializes it back.

**Recommendation**: Add a .cell fixture file that exercises all new keyword types, with a round-trip test (parse → serialize → parse → compare). The existing `test_parse_mg2sio4_cell` test is `#[ignore]` and empty — this can serve as the template.

**Risk**: Low — new test file, no production code changes.

## D4: Fix doc comments on SpectralKpointsMpGrid

**Concern**: The doc comment for `SpectralKpointsMpGrid` uses `SPECTRAL_KPOINTS_MP_GRID` (plural) but the canonical `KEY_NAME` is `SPECTRAL_KPOINT_MP_GRID` (singular).

**Recommendation**: Fix the doc to use the singular canonical name.

**Risk**: None — documentation-only change.

## D5: Fix parsing order (immediate, not deferred)

**Note**: This is the defect identified in the review — spectral block types must be parsed before BS_ blocks. This is included in `fix-directions.json` and should be applied before closing the branch. It is NOT deferred.

## D6: Consider `#[non_exhaustive]` on `Lattice` enum

**Concern**: Adding `Lattice::Abc` is a breaking change for external `match` statements. In a pre-1.0 library this is acceptable, but future variants could benefit from `#[non_exhaustive]`.

**Recommendation**: Before stabilization, add `#[non_exhaustive]` to the `Lattice` enum to allow additive variants without breaking changes.

**Risk**: Low — changes match semantics only.

## D7: Backfill G5 test coverage (~50 tests)

**Concern**: All 8 G5 phonon MP/fine types have exactly 1 unit test each, while structurally identical reference types have 6-8 tests. The implementation logic is copy-paste-correct, but the tests serve as regression guards when the underlying trait infrastructure changes.

Affected types and missing test categories:

| Type | Pattern | Missing tests |
|------|---------|---------------|
| `PhononKpointsMpGrid` | `[u32;3]` newtype | insufficient, too_many, non_array, to_cell_value, to_cell, round_trip, from_key_value |
| `PhononKpointsMpOffset` | `[f64;3]` newtype | insufficient, too_many, non_array, to_cell_value, to_cell, round_trip, from_key_value |
| `PhononKpointsMpSpacing` | spacing struct | with_unit, empty_array, to_cell_scalar, to_cell_with_unit, round_trip |
| `PhononFineKpointPath` | block w/ Vec<Entry> | empty, block_name, to_cell, builder_empty/single/multi |
| `PhononFineKpointPathSpacing` | spacing struct | with_unit, empty_array, to_cell_scalar, to_cell_with_unit, round_trip |
| `PhononFineKpointsMpGrid` | `[u32;3]` newtype | insufficient, too_many, non_array, to_cell_value, to_cell, round_trip, from_key_value |
| `PhononFineKpointsMpSpacing` | spacing struct | with_unit, empty_array, to_cell_scalar, to_cell_with_unit, round_trip |
| `PhononFineKpointsMpOffset` | `[f64;3]` newtype | insufficient, too_many, non_array, to_cell_value, to_cell, round_trip, from_key_value |

**Recommendation**: Each type should replicate the test suite of its reference counterpart. Roughly 50 tests need to be added across 8 files.

**Risk**: Low — implementation logic is correct; the coverage gap is about resilience, not correctness.

## D8: SymmetryTol has zero tests

**Concern**: `castep_cell_io/src/cell/symmetry/symmetry_tol.rs` has no `#[cfg(test)]` module at all. This is pre-existing, not introduced by Phase 1.

**Recommendation**: Add standard test suite: from_cell_value (valid), from_cell_value (insufficient), from_cell_value (non-array), from_key_value, to_cell_value, to_cell, round_trip.

**Risk**: Low — pre-existing condition, but worth fixing.

## D9: PhononKpointPathEntry builder parity

**Concern**: `SpectralKpointPathEntry` derives `bon::Builder`, but `PhononKpointPathEntry` (which FineKpointPath reuses) does not. Both are `{ coord: [f64; 3] }` structs.

**Recommendation**: Add `bon::Builder` to `PhononKpointPathEntry` for API consistency.

**Risk**: Low — additive derive, no behavior change.

## D10: SpectralKpointsList PartialOrd audit

**Concern**: `SpectralKpointsList` derives `PartialOrd`, but `KpointsList`, `PhononKpointList`, and `PhononFineKpointList` do not. The derive is a stray inconsistency.

**Recommendation**: Either remove the stray `PartialOrd` or add it consistently to all k-point list types.

**Risk**: Low — cosmetic inconsistency, no functional impact.
