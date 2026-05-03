# Codebase State Report: Phase 1 Plan Elaboration

**Generated from:** Exploration of castep-cell-io workspace

## 1. Current File Tree for Affected Areas

```
castep-cell-io/
  Cargo.toml                         # workspace root
  castep_cell_fmt/                   # parser/formatter backend (v0.1.0)
    src/
      lib.rs, parse.rs, parser.rs, query.rs, format.rs, error.rs
  castep_cell_io/                    # domain types (v0.4.0)
    Cargo.toml                       # deps: castep-cell-fmt, bon 3.9.1, serde
    src/
      lib.rs                         # pub mod cell; pub mod param; pub mod units; mod cell_document;
      cell_document.rs               # CellDocument + Lattice/Positions enums
      param_document.rs              # ParamDocument (not in scope)
      cell/
        mod.rs                       # pub mod declarations for 9 submodules
        lattice_param.rs             # LatticeCart, LatticeABC
        constraints/
          mod.rs                     # submod decls + pub use (CellConstraints NOT exported!)
          cell_constraints.rs        # CellConstraints (block, bon::Builder)
          fix_vol.rs, fix_all_cell.rs, fix_all_ions.rs, fix_com.rs,
          ionic_constraints.rs, nonlinear_constraints.rs
        bz_sampling_kpoints/
          mod.rs                     # submod decls + pub use
          kpoint.rs, kpoints_list.rs, bs_kpoint_path.rs (IS wired),
          bs_kpoints_list.rs, bs_kpoint_path_spacing.rs (NOT wired),
          kpoints_mp_grid.rs (NOT wired), kpoints_mp_spacing.rs (NOT wired),
          magres_kpoints_list.rs, optics_kpoints_list.rs
        phonon/
          mod.rs                     # submod decls + pub use
          phonon_kpoint_path_spacing.rs (NOT wired, MISSING KEY_ALIASES),
          phonon_kpoint_path.rs, phonon_kpoint_list.rs,
          phonon_gamma_directions.rs, phonon_fine_kpoint_list.rs,
          phonon_supercell_matrix.rs, supercell_kpoint_list_castep.rs
        symmetry/
          mod.rs                     # submod decls + pub use (SymmetryTol IS exported)
          symmetry_ops.rs, symmetry_tol.rs (NOT wired, NO tests)
        external_fields/, positions/, species/, velocities/
      param/ (not in scope)
      units/ (LengthUnit, InvLengthUnit, etc.)
```

## 2. Key Type Signatures

### Orphan types (exist but NOT wired into CellDocument)

| Type | Kind | Module | Has Builder? | Has KEY_ALIASES? | Has tests? | Exported? |
|------|------|--------|-------------|------------------|-----------|----------|
| `CellConstraints` | Block | constraints/ | yes (bon) | N/A (block) | yes | **NO** (missing `pub use`) |
| `FixVOL` | Key-value | constraints/ | no | **NO** | yes | yes |
| `KpointsMpGrid` | Key-value | bz_sampling_kpoints/ | no | yes | yes | yes |
| `KpointsMpSpacing` | Key-value | bz_sampling_kpoints/ | no | yes | yes | yes |
| `BsKpointPathSpacing` | Key-value | bz_sampling_kpoints/ | no | yes | yes | yes |
| `SymmetryTol` | Key-value | symmetry/ | yes (bon) | **NO** | **NO** | yes |
| `PhononKpointPathSpacing` | Key-value | phonon/ | no | **MISSING** | yes | yes |

### LatticeABC
```rust
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, bon::Builder)]
pub struct LatticeABC {
    pub unit: Option<LengthUnit>,
    pub abc: [f64; 3],
    pub angles: [f64; 3],
}
// BLOCK_NAME = "LATTICE_ABC" (no aliases), FromBlock, ToCell impls, 7 unit tests
```

### Current Lattice enum
```rust
pub enum Lattice {
    Cart(LatticeCart),
}
impl ToCell for Lattice {
    fn to_cell(&self) -> Cell<'_> {
        match self { Lattice::Cart(cart) => cart.to_cell() }
    }
}
```

### Current CellDocument
28-field struct with no fields for any orphan or planned new types.

## 3. Module Dependency Graph

```
castep_cell_fmt (v0.1.0)  -- standalone
castep_cell_io (v0.4.0)   -- depends on castep_cell_fmt, bon, serde
```

Import chain: `lib.rs` → `mod cell_document;` → `cell_document.rs` imports from `crate::cell::{bz_sampling_kpoints, constraints, lattice_param, phonon, symmetry, ...}`

## 4. Existing Test Structure

- No integration tests
- 43 unit test modules across `cell/` tree
- Common patterns: FromCellValue, FromBlock, ToCell tests
- Test gaps: SymmetryTol has no tests; CellDocument has one `#[ignore]` test

## 5. Key Observations

**Pattern — Three keyword type sub-patterns:**
- **Block**: FromBlock + ToCell; parsed via `find_block_any(cells, &[ALIASES])`
- **Key-value**: FromKeyValue + ToCell; parsed via `TypeName::from_cells(cells)?`
- **Flag**: FromCellValue + ToCell; parsed via raw `find_map` on `Cell::KeyValue`/`Cell::Flag`

**Export chain requirement**: Type file → `mod` + `pub use` in parent `mod.rs` → import in `cell_document.rs` → field + parsing + serialization.

**Known bugs in current code:**
- `CellConstraints` not re-exported from `constraints/mod.rs` (has `mod cell_constraints;` but no `pub use`)
- `PhononKpointPathSpacing` missing `KEY_ALIASES`
- `SymmetryTol` has no test coverage
- `FixVOL` lacks `KEY_ALIASES`

**Spectral naming convention**: BS-equivalent types get BS_ backward-compat aliases; net-new types do not.

**PhononFineKpointPath reuses `PhononKpointPathEntry`** — no separate entry type needed.
