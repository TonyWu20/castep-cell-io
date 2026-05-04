# Review: Phase 1 — New Keyword Types + Flat CellDocument Wiring

**Index**: notes/directions/phase-1/directions-index.json
**Reviewed**: 2026-05-05
**Branch**: issue-8
**Base**: main

## Summary

**Overall: PASS with concerns**

The implementation is architecturally sound and mechanically correct. All 1071 tests pass, cargo clippy emits zero warnings, and every architecture note from the directions is followed. All 22 new types are correctly implemented across 5 task groups plus a parse-order fix (FIX-G4-parse-order). The single substantive concern is test coverage on the G5 phonon types (1 test each vs. 6-8 in reference types).

## Per-Task Results

### G1-fix-gaps: Fix gaps and wire orphan types

| Task | Status | Detail |
|------|--------|--------|
| TASK-G1-1: pub use CellConstraints | ✓ Passed | `pub use cell_constraints::CellConstraints;` added in constraints/mod.rs |
| TASK-G1-2: KEY_ALIASES on PhononKpointPathSpacing | ✓ Passed | `PHONON_KPOINTS_PATH_SPACING` alias added |
| TASK-G1-3: Wire 6 orphan types into CellDocument | ✓ Passed | All 6 types (KpointsMpGrid, KpointsMpSpacing, BsKpointPathSpacing, CellConstraints, FixVOL, SymmetryTol) fully wired |

### G2-lattice-abc: Add Lattice::Abc variant

| Task | Status | Detail |
|------|--------|--------|
| TASK-G2-1: Lattice::Abc variant + parsing | ✓ Passed | Abc variant, From impls, ToCell arm, dual-lattice mutual exclusion |

### G3-new-simple-types: KpointsMpOffset + SymmetryGenerate

| Task | Status | Detail |
|------|--------|--------|
| TASK-G3-1: KpointsMpOffset [f64;3] newtype | ✓ Passed | Full test suite (8 tests), correct alias |
| TASK-G3-2: SymmetryGenerate unit struct flag | ✓ Passed | ToCell via has_flag pattern |
| TASK-G3-3: Wire into CellDocument | ✓ Passed | Both types imported, fielded, parsed, serialized |

### G4-spectral-types: 6 spectral k-point types

| Task | Status | Detail |
|------|--------|--------|
| TASK-G4-1: SpectralKpointPath + entry block | ✓ Passed | bon::Builder, BS_ aliases, full test suite |
| TASK-G4-2: SpectralKpointsList block | ✓ Passed | bon::Builder, BS_ aliases, reuses Kpoint |
| TASK-G4-3: SpectralKpointPathSpacing | ✓ Passed | BS_ backward compat aliases |
| TASK-G4-4: SpectralKpointsMpGrid [u32;3] | ✓ Passed | KPOINTS plural alias |
| TASK-G4-5: SpectralKpointsMpSpacing | ✓ Passed | KPOINTS plural alias |
| TASK-G4-6: SpectralKpointsMpOffset [f64;3] | ✓ Passed | KPOINTS plural alias |
| TASK-G4-7: Wire all 6 into CellDocument | ✓ Passed | Spectral parsed BEFORE BS_ (P0 constraint satisfied) |
| **P0 constraint** (spectral before BS_) | ✓ Satisfied | Verified in from_cell_file and to_cell_file |

### G5-phonon-mp-fine: 8 phonon MP and fine types

| Task | Status | Detail |
|------|--------|--------|
| TASK-G5-1: PhononKpointsMpGrid [u32;3] | ⚠ Minor | Only 1 test (reference: 8); impl correct |
| TASK-G5-2: PhononKpointsMpSpacing | ⚠ Minor | Only 1 test (reference: 7); impl correct |
| TASK-G5-3: PhononKpointsMpOffset [f64;3] | ⚠ Minor | Only 1 test (reference: 8); impl correct |
| TASK-G5-4: PhononFineKpointPath block | ⚠ Minor | Only 1 test (reference: 7); impl correct |
| TASK-G5-5: PhononFineKpointPathSpacing | ⚠ Minor | Only 1 test (reference: 7); impl correct |
| TASK-G5-6: PhononFineKpointsMpGrid [u32;3] | ⚠ Minor | Only 1 test; style inconsistency (fully qualified Error) |
| TASK-G5-7: PhononFineKpointsMpSpacing | ⚠ Minor | Only 1 test (reference: 7); impl correct |
| TASK-G5-8: PhononFineKpointsMpOffset [f64;3] | ⚠ Minor | Only 1 test (reference: 8); impl correct |
| TASK-G5-9: Wire all 8 into CellDocument | ✓ Passed | All 8 types imported, fielded, parsed, serialized, Ok() bound |

### FIX-parse-order: Spectral parse ordering

| Check | Status |
|-------|--------|
| spectral_kpoint_path before bs_kpoint_path | ✓ Passed |
| spectral_kpoints_list before bs_kpoints_list | ✓ Passed |
| Key-value spectral types unmoved | ✓ Passed |
| Serialization order unchanged | ✓ Passed |
| No logic changes beyond reordering | ✓ Passed |

## Issues Found

1. **⚠ G5 test coverage (low severity)** — All 8 G5 phonon MP/fine types have exactly 1 unit test each, while structurally identical reference types have 6-8 tests. The implementation logic is copy-paste-correct from G4 types (which are thoroughly tested), so the risk of bugs is low. However, the thin coverage reduces regression resilience. ~50 tests should be backfilled.

2. **⚠ PhononFineKpointsMpGrid style inconsistency (low severity)** — Uses fully qualified `castep_cell_fmt::Error::Message(...)` instead of importing `Error` directly like sibling types. Functional but inconsistent.

3. **ℹ SpectralKpointsList PartialOrd derive** — No other k-point list type derives PartialOrd. Cosmetic inconsistency.

## Strategic Assessment

- **Architecture**: All architecture notes followed. 7-step export chain respected. Crate boundaries maintained (no castep_cell_fmt changes).
- **P0 mitigation**: Spectral block types correctly parse before BS_ types. Mutual-exclusion validation deferred to future phase.
- **Public API**: All types pub, builders where appropriate. Lattice::Abc is a breaking change (intentional per project conventions).
- **Recommendation**: Merge as-is. Backfill G5 tests and address style inconsistencies in the next phase.

## Deferred Items

See `notes/pr-reviews/phase-1/deferred.md` for 10 deferred items (D1–D10).
