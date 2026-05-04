# Plan: Add Mutual Exclusion Validation to `CellDocument` Builder

## Context

The `CellDocument` builder currently has no validation for the mutual exclusion rules specified in the [official CASTEP cell keywords documentation](supplemented_official_docs/cell_keywords.md). Five of six rules are unenforced — users can construct invalid documents with conflicting field combinations (e.g., both `kpoints_list` AND `kpoints_mp_grid` set).

The user wants validation integrated into the builder's `build()` method using **bon's fallible builder pattern** ("Custom Finishing Function" approach), not a separate `.validate()` method.

## Current State

- `CellDocument` uses `#[derive(Builder)]` → `build()` is infallible, returns `CellDocument` directly.
- `from_cell_file` parses all optional fields independently — no cross-field validation.
- `ParamDocument` does NOT use bon fallible builders; it uses a separate `fn validate(self) -> Result<Self, String>` called after `.build()`.
- Two rules are already structurally enforced via enum types:
  - ^1 `LATTICE_CART`/`LATTICE_ABC` → `Lattice` enum
  - ^2 `POSITIONS_FRAC`/`POSITIONS_ABS` → `Positions` enum

## Rules to Implement

All from `supplemented_official_docs/cell_keywords.md` footnotes, plus **D2** (spectral/BS_ backward-compat alias duplication):

| Rule | Fields | Constraint |
|------|--------|-----------|
| ^3 | `kpoints_list`, `kpoints_mp_grid`, `kpoints_mp_spacing` | At most 1 of 3 may be `Some` |
| ^4 | `spectral_kpoint_path`, `spectral_kpoints_mp_grid`, `spectral_kpoints_mp_spacing`, `spectral_kpoints_list`, **`bs_kpoint_path`** (D2), **`bs_kpoints_list`** (D2) | At most 1 of 6 may be `Some` |
| ^5 | `phonon_kpoint_path`, `phonon_kpoint_list` | At most 1 of 2 may be `Some` |
| ^6 | `symmetry_generate`, `symmetry_ops` | At most 1 of 2 may be `Some` |

Note: `kpoints_mp_offset` and `spectral_kpoints_mp_offset` are **not** mutually exclusive — they are tuning parameters.

D2 absorption: `bs_kpoint_path` and `bs_kpoints_list` are merged into the ^4 check because they are the same data parsed under different block names (backward-compat aliases). A document must not have both `spectral_kpoint_path = Some(...)` and `bs_kpoint_path = Some(...)`.

## Implementation Plan

### Step 1: Add bon fallible builder to `CellDocument`

**File**: `castep_cell_io/src/cell_document.rs` (line 238)

Change the builder derive to use a custom finishing function:

```rust
#[derive(Debug, Clone, Builder)]
#[builder(on(Lattice, into), on(Positions, into), finish_fn(vis = "", name = build_internal))]
pub struct CellDocument {
    // ... fields unchanged ...
}
```

This makes `bon` (v3.9.1) generate `build_internal()` (infallible, private) instead of the default `build()`.

### Step 2: Implement custom `build()` on `CellDocumentBuilder`

Add after the `CellDocument` struct definition:

```rust
use cell_document_builder::IsComplete;

impl<S: cell_document_builder::IsComplete> CellDocumentBuilder<S> {
    pub fn build(self) -> CResult<CellDocument> {
        let doc = self.build_internal();

        // ^3: KPOINTS_LIST / KPOINTS_MP_GRID / KPOINTS_MP_SPACING — at most one
        let kp_count = [doc.kpoints_list.is_some(),
                        doc.kpoints_mp_grid.is_some(),
                        doc.kpoints_mp_spacing.is_some()]
            .iter().filter(|&&x| x).count();
        if kp_count > 1 {
            return Err(Error::Message(
                "Only one of KPOINTS_LIST, KPOINTS_MP_GRID, and KPOINTS_MP_SPACING may be specified".into()
            ));
        }

        // ^4 + D2: SPECTRAL/BS k-points — at most one of 6
        let sp_count = [doc.spectral_kpoint_path.is_some(),
                        doc.spectral_kpoints_mp_grid.is_some(),
                        doc.spectral_kpoints_mp_spacing.is_some(),
                        doc.spectral_kpoints_list.is_some(),
                        doc.bs_kpoint_path.is_some(),
                        doc.bs_kpoints_list.is_some()]
            .iter().filter(|&&x| x).count();
        if sp_count > 1 {
            return Err(Error::Message(
                "Only one of SPECTRAL_KPOINT_PATH, SPECTRAL_KPOINTS_MP_GRID, SPECTRAL_KPOINTS_MP_SPACING, SPECTRAL_KPOINT_LIST, BS_KPOINT_PATH, and BS_KPOINTS_LIST may be specified".into()
            ));
        }

        // ^5: PHONON_KPOINT_PATH / PHONON_KPOINT_LIST — at most one
        let ph_count = [doc.phonon_kpoint_path.is_some(),
                        doc.phonon_kpoint_list.is_some()]
            .iter().filter(|&&x| x).count();
        if ph_count > 1 {
            return Err(Error::Message(
                "Only one of PHONON_KPOINT_PATH and PHONON_KPOINT_LIST may be specified".into()
            ));
        }

        // ^6: SYMMETRY_GENERATE / SYMMETRY_OPS — at most one
        let sym_count = [doc.symmetry_generate.is_some(),
                         doc.symmetry_ops.is_some()]
            .iter().filter(|&&x| x).count();
        if sym_count > 1 {
            return Err(Error::Message(
                "Only one of SYMMETRY_GENERATE and SYMMETRY_OPS may be specified".into()
            ));
        }

        Ok(doc)
    }
}
```

**Note on `IsComplete` trait path**: The derive macro on `CellDocument` generates a private `cell_document_builder` module containing the `IsComplete` trait. Since the custom `build()` impl block is in the same parent module (`cell_document`), the `cell_document_builder::IsComplete` path resolves correctly in bon v3.9.1.

### Step 3: Update `from_cell_file` to use the builder

**File**: `castep_cell_io/src/cell_document.rs` (lines 483-766)

**3a: Keep the lattice mutual-exclusion check** (lines 486-491). The `Lattice` enum structurally prevents both variants via its `Cart | Abc` construction, but this parse-time check produces a specific, user-actionable error message ("Both LATTICE_CART and LATTICE_ABC are specified") rather than silently selecting one.

**3b: Prevent spectral/BS_ duplication at parse time** (lines 545-553). Currently when a `.cell` file uses BS_ block names, `from_cell_file` unconditionally populates both `spectral_kpoint_path` and `bs_kpoint_path` (same data, different fields). Guard BS_ parsing behind `spectral_*.is_none()` checks:

```rust
let bs_kpoint_path = if spectral_kpoint_path.is_some() {
    None
} else {
    find_block_any(cells, &["BS_KPOINT_PATH", "BS_KPOINTS_PATH"])
        .ok()
        .map(|rows| BsKpointPath::from_block_rows(rows))
        .transpose()?
};

let bs_kpoints_list = if spectral_kpoints_list.is_some() {
    None
} else {
    find_block_any(cells, &["BS_KPOINT_LIST", "BS_KPOINTS_LIST"])
        .ok()
        .map(|rows| BSKpointList::from_block_rows(rows))
        .transpose()?
};
```

**3c: Replace struct literal construction** (lines 714-765) with builder usage. For `Option<T>` fields, use bon's `maybe_*` setters (bon's generated naming for optional fields, e.g., `.maybe_kpoints_list(kpoints_list)`). For required fields (`lattice`, `positions`), use the direct setter which auto-wraps via `From`/`Into`. The final `.build()` returns `CResult<CellDocument>` — already `from_cell_file`'s return type, no signature change.

### Step 4: No export changes needed

`CellDocumentBuilder` is already exported from `lib.rs:23`.

### Step 5: No README changes needed

The README already shows `.build().unwrap()` usage.

## Verification

### Unit tests (in `#[cfg(test)] mod tests` within `cell_document.rs`)

- `build_rejects_multiple_kpoint_specs` — set both `kpoints_list` and `kpoints_mp_grid`, expect `Err`
- `build_rejects_multiple_spectral_specs` — set `spectral_kpoint_path` + `spectral_kpoints_mp_grid`, expect `Err`
- `build_rejects_spectral_and_bs_duplication` — set both `spectral_kpoint_path` and `bs_kpoint_path`, expect `Err`
- `build_rejects_multiple_phonon_specs` — set both `phonon_kpoint_path` and `phonon_kpoint_list`, expect `Err`
- `build_rejects_symmetry_generate_and_ops` — set both, expect `Err`
- `build_allows_mp_offset_with_grid` — `kpoints_mp_grid` + `kpoints_mp_offset` is valid (offset is a tuning parameter)
- `build_allows_single_spec` — sanity check that each category alone is valid

### Integration test (in `tests/`)

Parse a `.cell` string containing both `%BLOCK KPOINTS_LIST` and `KPOINT_MP_GRID` blocks, verify `from_cell_file` returns `Err`.

### Commands

```bash
cargo check                            # compiler check, IsComplete trait must resolve
cargo test -p castep_cell_io           # all unit + integration tests
cargo clippy                           # no new warnings
```

## Critical Files

- `castep_cell_io/src/cell_document.rs` — primary changes (builder derive, custom `build()`, `from_cell_file` to builder, tests)
- `castep_cell_io/src/lib.rs` — verify export (no change expected)
- `castep_cell_io/README.md` — verify example (already shows `.unwrap()`, no change)
- `supplemented_official_docs/cell_keywords.md` — reference for validation rules

## Deferred Items from Plan Review

### D2: Mutual-exclusion validation between spectral and BS_ types → **Absorbed**
Added `bs_kpoint_path` and `bs_kpoints_list` to the ^4 count check. Added parse-time guards in `from_cell_file` to prevent BS_ duplication at the source.

### D6: `#[non_exhaustive]` on `Lattice` enum → **Deferred again**
Precondition: Apply during the API stabilization milestone immediately preceding 1.0. Audit all public enums uniformly — doing it now while adding validation rules is scope creep.
