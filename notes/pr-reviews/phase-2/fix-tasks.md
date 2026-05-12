# Fix Tasks — CellDocument Group Substruct Migration

**Source**: `notes/pr-reviews/phase-2/review.md`
**Date**: 2026-05-13

## Task FIX-1: Add missing validate() calls in CellDocument::build()

**Kind:** direct
**Goal:** Add `validate()` calls for the 4 omitted groups in `CellDocument::build()` to prevent a maintenance trap.

**File:** `castep_cell_io/src/cell_document.rs`, lines 291-301

**Changes:**
In the `build()` method, after the existing validate calls, add:
```rust
doc.optics_magres = doc.optics_magres.validate().map_err(|e| Error::Message(e.to_string()))?;
doc.external_fields = doc.external_fields.validate().map_err(|e| Error::Message(e.to_string()))?;
doc.species = doc.species.validate().map_err(|e| Error::Message(e.to_string()))?;
doc.dynamics = doc.dynamics.validate().map_err(|e| Error::Message(e.to_string()))?;
```

These are pass-through validate() today, so the call is a no-op. The fix is about future-proofing and consistency with the 6 already-validated groups.

**Success Criteria:**
- `cargo build` succeeds
- All tests still pass
- `cargo clippy -- -D warnings` passes

---

## Task FIX-2: Add missing positions assertion in test_parse_zno_lr_cell

**Kind:** lib-tdd
**Goal:** Assert `doc.positions` is `Positions::Frac` with 4 entries in the ZnO fixture test.

**File:** `castep_cell_io/src/cell_document.rs`, in `test_parse_zno_lr_cell`

**Changes:**
After the existing assertions, add:
```rust
assert!(matches!(doc.positions, Positions::Frac(_)));
if let Positions::Frac(ref pos) = doc.positions {
    assert_eq!(pos.ions.len(), 4);
}
```

**Success Criteria:**
- `cargo test -p castep-cell-io test_parse_zno_lr_cell` passes
- The new assertion checks a concrete value (4 atoms), not just `is_some()`

---

## Task FIX-3: Add mp_grid + mp_spacing validation test to KpointsParams

**Kind:** lib-tdd
**Goal:** Add the missing `mp_grid + mp_spacing → Err` inline test as prescribed by TASKS.md D-2.

**File:** `castep_cell_io/src/cell/kpoints_params.rs`, tests module

**Changes:**
Add a test:
```rust
#[test]
fn test_validate_mp_grid_and_mp_spacing_err() {
    let params = KpointsParams {
        kpoints_mp_grid: Some(KpointsMpGrid([2, 2, 2])),
        kpoints_mp_spacing: Some(KpointsMpSpacing { value: 0.05, unit: None }),
        ..Default::default()
    };
    assert!(params.validate().is_err());
}
```

**Success Criteria:**
- `cargo test -p castep-cell-io kpoints_params::tests::test_validate_mp_grid_and_mp_spacing_err` passes
- The test covers the specific pair prescribed in TASKS.md
