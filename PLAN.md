# Refactor Plan: Drop Serde + Rename `castep_cell_serde` → `castep_cell_io`

## Context

The original motivation for serde was to let juniors declaratively add CASTEP data types with
minimal boilerplate. With Claude available for code generation, that motivation is gone. The
serde approach has accumulated real bugs (case-insensitive key lookup silently fails; double `\n`
in `KeyValue` serialization) and required complex workarounds (`#[serde(from = "XxxRepr")]`,
untagged enums, multi-level `From` chains) that fight serde's data model rather than working
with it.

The replacement is a set of plain Rust parsing traits backed by explicit case-insensitive query
helpers. The crate is also renamed from `castep_cell_serde` to `castep_cell_io` since it will
no longer depend on serde.

`castep-param-io` is deprecated and **not touched**.

---

## Current Status (2026-04-03)

### ✅ Phase 1 Complete
- `src/query.rs`, `src/parse.rs`, `src/format.rs` exist with new API
- Primitive `FromCellValue` impls for scalars work
- Tests confirm case-insensitive lookup and no double-`\n`

### 🔄 Phase 2 Partial (16/~200 types migrated)
- ✅ 14 unit enums in `units/` have `FromCellValue`
- ✅ 2 param types have `FromKeyValue` (`Task`, `EfieldEnergyTol`)
- ❌ ~118 remaining param keyword types still have serde
- ❌ All 30 `.cell` block types still use `#[serde(from = "XxxRepr")]` pattern
- ❌ 497 `#[serde(...)]` attributes across 175 files remain

### ❌ Phase 3 Not Started
- Old serde layer (`de/`, `ser.rs`) still present
- Crate still named `castep_cell_serde`
- `serde` dependency still in `Cargo.toml`

---

## Revised Execution Plan

### Phase 2A — Migrate remaining param types (118 files)

All param types follow Pattern B or C:
- Pattern B: newtype struct wrapping scalar → `FromKeyValue` only
- Pattern C: keyword enum → `FromCellValue` + `FromKeyValue`

**Scope**: `castep_cell_data/src/param/**/*.rs` (excluding already-migrated `general/task.rs` and `efield/efield_energy_tol.rs`)

**Action per file**:
1. Remove `#[derive(Serialize, Deserialize)]`
2. Remove all `#[serde(...)]` attributes
3. Add `FromKeyValue` impl (and `FromCellValue` if keyword enum)
4. Update tests to use new traits

### Phase 2B — Migrate `.cell` block types (30 files)

Block types follow Pattern D (optional unit prefix) or Pattern E (Vec of rows):

**Pattern D files** (8 types with optional unit):
- `cell/lattice_param.rs` (LatticeCart, LatticeABC)
- `cell/external_fields/external_efield.rs`
- `cell/external_fields/external_pressure.rs`
- `cell/species/species_mass.rs`
- `cell/species/hubbard_u/*.rs` (3 files)

**Pattern E files** (remaining ~22 types with Vec rows):
- `cell/positions/positions_frac.rs`
- `cell/bz_sampling_kpoints/*.rs`
- `cell/constraints/*.rs`
- `cell/species/*.rs` (remaining)
- `cell/symmetry/*.rs`

**Action per file**:
1. Delete `*Repr` enum and `From<*Repr>` impl
2. Remove `#[derive(Serialize, Deserialize)]` and `#[serde(...)]`
3. Add `FromBlock` impl (and `FromCellValue` for row types in Pattern E)
4. Update tests

### Phase 3 — Remove serde and rename crate

1. Delete `castep_cell_serde/src/de/` (3 files)
2. Delete `castep_cell_serde/src/ser.rs`
3. Update `castep_cell_serde/src/error.rs` (remove serde trait impls)
4. Update `castep_cell_serde/src/lib.rs` (remove `mod de; mod ser;`)
5. Rename `to_string_direct` → `to_string` in `format.rs`
6. Update `castep_cell_serde/Cargo.toml`: drop `serde`, `anyhow`, `derive_builder`; rename to `castep_cell_io`
7. Rename directory: `castep_cell_serde/` → `castep_cell_io/`
8. Update workspace `Cargo.toml`: member name
9. Update `castep_cell_data/Cargo.toml`: dep name/path, drop `serde`
10. Update `castep-param-io/Cargo.toml`: dep name/path only
11. Global find-replace: `use castep_cell_serde::` → `use castep_cell_io::`
12. Run `cargo test --workspace`

---

## Task Breakdown for Agents

Phase 2A and 2B are broken into minimal, independent tasks in `tasks/*.md`.
Each task file contains:
- Exact file list to modify
- Pattern to apply
- Acceptance criteria

Phase 3 is a single sequential task (cannot be parallelized due to file renames).
