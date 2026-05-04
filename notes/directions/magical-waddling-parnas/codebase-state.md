# Codebase State: Add Mutual Exclusion Validation to `CellDocument` Builder

Date: 2026-05-05
Plan: `plans/magical-waddling-parnas/PLAN.md`

---

## 1. Affected File Tree

```
castep-cell-io/
  Cargo.toml                    # v0.4.0, bon 3.9.1, serde
  README.md                     # Shows .build().unwrap() -- currently incorrectly referenced (build is infallible)
  src/
    lib.rs                      # Exports CellDocument, CellDocumentBuilder, Lattice, Positions
    cell_document.rs            # THE PRIMARY TARGET -- 974 lines
    param_document.rs           # Reference pattern (uses separate .validate() + String return)
    cell/
      mod.rs
      lattice_param.rs          # LatticeCart, LatticeABC
      positions/                # PositionsFrac, PositionsAbs
      bz_sampling_kpoints/
        mod.rs                  # Module wiring + re-exports
        kpoint.rs
        kpoints_list.rs
        kpoints_mp_grid.rs
        kpoints_mp_spacing.rs
        kpoints_mp_offset.rs
        bs_kpoint_path.rs       # BsKpointPath
        bs_kpoints_list.rs      # BSKpointList
        bs_kpoint_path_spacing.rs
        spectral_kpoint_path.rs # SpectralKpointPath
        spectral_kpoints_list.rs
        spectral_kpoint_path_spacing.rs
        spectral_kpoints_mp_grid.rs
        spectral_kpoints_mp_spacing.rs
        spectral_kpoints_mp_offset.rs
        magres_kpoints_list.rs
        optics_kpoints_list.rs
      phonon/
        phonon_kpoint_path.rs
        phonon_kpoint_list.rs
        phonon_kpoints_mp_grid.rs
        phonon_kpoints_mp_spacing.rs
        phonon_kpoints_mp_offset.rs
        phonon_fine_kpoint_path.rs
        phonon_fine_kpoint_path_spacing.rs
        phonon_fine_kpoints_mp_grid.rs
        phonon_fine_kpoints_mp_spacing.rs
        phonon_fine_kpoints_mp_offset.rs
        phonon_fine_kpoint_list.rs
        phonon_gamma_directions.rs
        phonon_supercell_matrix.rs
      symmetry/
        symmetry_generate.rs    # SymmetryGenerate (unit struct, flag)
        symmetry_ops.rs         # SymmetryOps (block)
      constraints/
      external_fields/
      species/
      velocities/
    param/                      # 18 parameter groups (not directly changed, pattern reference only)
    units/
  examples/
    format_demo.rs              # Only example
```

No `tests/` directory exists at crate or workspace level.

---

## 2. Key Type/Function Signatures

### `CellDocument` (cell_document.rs, line 238)

Current derive:
```rust
#[allow(clippy::duplicated_attributes)]
#[derive(Debug, Clone, Builder)]
#[builder(on(Lattice, into), on(Positions, into))]
pub struct CellDocument {
    pub lattice: Lattice,
    pub positions: Positions,
    // 48 optional fields, all Option<T>
    // ...
}
```

Current builder behavior: infallible, returns `CellDocument` directly.

### Supporting enums

```rust
// line 105
pub enum Lattice { Cart(LatticeCart), Abc(LatticeABC) }
// impl From<LatticeCart>, From<LatticeABC> -- used by builder .into()

// line 159
pub enum Positions { Frac(PositionsFrac), Abs(PositionsAbs) }
```

### Error types (castep_cell_fmt/src/error.rs)

```rust
pub type CResult<T> = Result<T, Error>;
pub enum Error {
    Message(String),
    Empty,
    UnexpectedType(String, String),
    KeyNotFound(String),
    TryFromInt(TryFromIntError),
}
```

### `from_cell_file` (lines 483-766)

```rust
impl FromCellFile for CellDocument {
    fn from_cell_file(cells: &[Cell<'_>]) -> CResult<Self> {
        // Lines 484-491: Has LATTICE_CART/LATTICE_ABC mutual exclusion error
        // Lines 493-512: Parse lattice and positions
        // Lines 514-553: Parse all kpoint fields (including BS_ duplication bug)
        // Lines 555-712: Parse all remaining fields
        // Lines 714-765: Struct literal construction
    }
}
```

**The BS_ duplication bug** (lines 529-553): The function parses `spectral_kpoint_path` with aliases that INCLUDE `BS_KPOINT_PATH`/`BS_KPOINTS_PATH`, so a file using BS_ blocks populates BOTH the `spectral_*` and `bs_*` fields with the same data.

### `ParamDocument` builder (reference pattern, param_document.rs, line 168)

```rust
#[derive(Debug, Clone, Default, Builder)]
pub struct ParamDocument {
    pub general: GeneralParams,
    // ... 17 more required fields (all defaults-able) ...
}

impl ParamDocument {
    fn validate(mut self) -> Result<Self, String> {
        // Validates each group + inter-group mutual exclusion
        // Returns Err(String) on violation
    }
}

impl FromCellFile for ParamDocument {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        ParamDocument::builder()
            .general(GeneralParams::from_cell_file(tokens)?)
            // ... 17 more fields ...
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}
```

---

## 3. Module Dependency Graph

```
castep-cell-fmt (lib)
  Deps: anyhow, ariadne, chumsky, thiserror
  Provides: CResult, Error, Cell, CellValue, parsing traits, query functions
       ^
       | (path dependency)
       |
castep-cell-io (lib)
  Deps: castep-cell-fmt, bon 3.9.1, serde
  |
  |-- cell_document (private, re-exported via lib.rs)
  |     |-- cell/* (block types)
  |     |-- castep_cell_fmt (traits, query, error)
  |
  |-- param_document (private, re-exported via lib.rs)
  |     |-- param/* (18 parameter groups)
  |     |-- castep_cell_fmt (traits, error)
  |
  |-- cell/ (private, re-exported indirectly through cell_document)
  |-- param/ (private)
  |-- units/ (pub)
```

---

## 4. Existing Test Structure

### CellDocument tests (cell_document.rs, lines 952-974)

**1 test, marked `#[ignore]`, with an empty input string that cannot pass:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_parse_mg2sio4_cell() {
        let input = "";  // empty!
        let doc = castep_cell_fmt::parse::<CellDocument>(input).expect("Failed to parse");
        assert!(matches!(doc.lattice, Lattice::Cart(_))); // fails on empty
        // ... 6 more assertions that all fail
    }
}
```

### ParamDocument tests (param_document.rs, lines 488-560)

**5 tests, 1 ignored:**

| Test | Status | Purpose |
|------|--------|---------|
| `test_parse_co3o4_2_param` | `#[ignore]` | External file parsing |
| `test_default_construction` | Active | `ParamDocument::default()` |
| `test_builder_construction` | Active | Full builder chain |
| `test_validate_empty` | Active | `.validate()` on default doc |
| `test_to_cell_file_empty` | Active | Serialization of empty doc |

### Individual block-type unit tests (good coverage)

Each kpoint/symmetry type has thorough unit tests in its own module:

- **BsKpointPath**: 8 tests (entry parsing, builder, serialization)
- **SpectralKpointPath**: 8 tests (entry parsing, builder, serialization)
- **BSKpointList**: 4 tests (parsing, serialization)
- **SymmetryOps**: 8 tests (parsing, builder, serialization)
- **SymmetryGenerate**: 1 test (serialization)

All use `#[cfg(test)] mod tests { use super::*; }` pattern. These tests validate individual type correctness but do not test cross-field constraints.

---

## 5. Observations and Patterns (Qualitative Analysis)

### Pattern A: ParamDocument uses separate `.validate()`, plan diverges intentionally

`ParamDocument` uses a two-phase approach: infallible `build()` then `.validate()` returning `Result<Self, String>`. The plan deliberately diverges by using bon's fallible builder (`finish_fn` custom finishing function) which integrates validation directly into `build()`. This means:
- The custom `build()` on `CellDocumentBuilder` replaces the need for a separate validate call
- `from_cell_file` will call `.build()?` instead of `.build().validate().map_err(...)`
- This is architecturally cleaner for the one-shot builder case but less flexible for incremental validation

### Pattern B: Error type alignment

The plan correctly uses `Error::Message(String)` from `castep_cell_fmt::Error`. Note that `ParamDocument`'s `validate()` returns `Result<Self, String>` (plain String), then converts via `.map_err(|e| Error::Message(e.to_string()))`. The plan's custom `build()` returns `CResult<CellDocument>` directly, which means errors from the plan use `Error::Message(...)` natively -- no String intermediate.

### Pattern C: bon `finish_fn` / `IsComplete` path risk

The plan assumes:
```rust
#[builder(..., finish_fn(vis = "", name = build_internal))]
```
and then references `cell_document_builder::IsComplete` in the impl block. This depends on bon v3.9.1 generating a module named `cell_document_builder` (snake_case of the struct name). The plan notes this is correct for bon v3.9.1, but it should be verified with a `cargo check`. If the module naming convention differs, the impl block will fail to compile.

### Pattern D: BS_ backward-compat duplication is a real correctness bug

In `from_cell_file` (lines 529-553), `SpectralKpointPath::from_block_rows` includes `BS_KPOINT_PATH` and `BS_KPOINTS_PATH` in its BLOCK_ALIASES. Then `BsKpointPath::from_block_rows` also searches for `BS_KPOINT_PATH`/`BS_KPOINTS_PATH`. Both `find_block_any` calls scan the full `cells` token list independently (blocks are not consumed), so a `.cell` file using `BS_KPOINT_PATH` causes the same data to be parsed into BOTH `spectral_kpoint_path` AND `bs_kpoint_path`.

The plan's Step 3b correctly addresses this by guarding `bs_kpoint_path` parsing behind `spectral_kpoint_path.is_none()`.

### Pattern E: `maybe_` setter convention

The plan's Step 3c references `.maybe_kpoints_list(kpoints_list)`. In bon v3.x, the naming convention for optional fields is `maybe_{field_name}`. This should be verified, but it is the standard bon convention.

### Pattern F: `#[allow(clippy::duplicated_attributes)]` must be preserved

Line 237 of `cell_document.rs`:
```rust
#[allow(clippy::duplicated_attributes)]
#[derive(Debug, Clone, Builder)]
#[builder(on(Lattice, into), on(Positions, into))]
```

After the plan adds `finish_fn(...)` to the `#[builder(...)]` attribute, this lint suppression should remain. The `clippy::duplicated_attributes` lint fires when you have both `#[derive(Builder)]` and `#[builder(...)]` (the derive macro generates its own internal attribute), and this pattern is intentional with bon.

### Pattern G: README inconsistency

The README (line 36) shows:
```rust
.build()
.unwrap();
```

Currently the builder is infallible, making `.unwrap()` on a non-Result type either meaningless or a compile error. After the plan's implementation with the fallible builder, `.unwrap()` becomes correct. **No README change is needed**, as stated in the plan.

### Pattern H: Zero integration tests

The `castep_cell_io` crate has no `tests/` directory. The plan's integration test (parsing a `.cell` string with conflicting kpoint blocks and expecting `Err`) would be the first integration test for this crate. The test would verify the full pipeline: `castep_cell_fmt::parse::<CellDocument>(input)` -> returns `Err`.

### Pattern I: SymmetryGenerate is a unit struct (flag), SymmetryOps is a block

`SymmetryGenerate` is a unit struct that serializes as `Cell::Flag("SYMMETRY_GENERATE")`. `SymmetryOps` is a block struct with `Vec<SymmetryOp>` that serializes as `Cell::Block("SYMMETRY_OPS", ...)`. They are semantically incompatible -- there is no way for both to be meaningful simultaneously. The plan's mutual exclusion check (rule ^6) correctly enforces this.

### Pattern J: `ToCellFile` serialization does not need validation

The `impl ToCellFile for CellDocument` (lines 769-949) serializes all `Some` fields. After the plan, the validation only applies at `build()` time (builder creation) and `from_cell_file` time (parsing). The `ToCellFile` serialization path is naturally safe because:
- It only serializes fields that exist (are `Some`)
- If conflicting fields are `Some`, both get serialized -- CASTEP would then reject the file
- This is the same behavior as the current code

---

## 6. Compilation Risks

1. **`IsComplete` trait path**: `cell_document_builder::IsComplete` must resolve. Verify with `cargo check` after implementing Step 2. If it does not resolve, the alternative is to place the `impl` block inside a different module context.

2. **`maybe_` setter naming**: The plan uses `maybe_kpoints_list(val)` for `Option<T>` fields. bon v3.9.1 should generate these, but the exact naming should be verified with a quick compile check on a test that uses the builder.

3. **`finish_fn` attribute syntax**: The plan's `finish_fn(vis = "", name = build_internal)` is the standard bon syntax for making the generated finish function private with a custom name. This syntax is stable in bon v2.x+. The `vis = ""` makes it private (no visibility modifier).

4. **`#[allow(clippy::duplicated_attributes)]`**: After the change, the `#[builder(...)]` attribute will contain more options, but clippy may still warn about the `derive(Builder)` + `#[builder(...)]` combination. The existing suppression should handle this.

5. **Existing test breakage**: The single existing CellDocument test is `#[ignore]` and uses an empty input string, so it will not break. The plan adds new tests that would not interfere with it.

6. **bon v3.9.1 `Default` behavior for builder**: The current `CellDocument` builder is accessed via `CellDocument::builder()`. After adding `finish_fn(vis = "", name = build_internal)`, the entry point remains `CellDocument::builder()`.
