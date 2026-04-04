---
name: castep-cell-io common task archetypes
description: Recurring task patterns seen across plans in this project, with their standard sequence and file scope
type: project
---

## Pattern A — Unit/Keyword enum migration (castep_cell_data/src/units/ and param/ keyword types)

Standard sequence:
1. Write failing test for `FromCellValue` impl (TDD: separate task)
2. Remove `Serialize, Deserialize, #[serde(...)]` from enum
3. Add `impl FromCellValue for EnumType` using `to_ascii_lowercase()` match on `value_as_str`
4. For `.param` keyword enums: also add `impl FromKeyValue` delegating to `from_cell_value`

**Why:** The case-insensitive bug lives exactly here — serde's alias matching is case-sensitive in some paths; explicit `to_ascii_lowercase()` is the fix.

Files touched per unit enum: `castep_cell_data/src/units/<unit>.rs`
Files touched per param keyword enum: `castep_cell_data/src/param/<group>/<name>.rs`

## Pattern B — KeyValue newtype struct migration (castep_cell_data/src/param/)

Standard sequence:
1. Write failing test for `FromKeyValue` impl
2. Remove `Serialize, Deserialize, #[serde(...)]`
3. Add `impl FromKeyValue` with `const KEY_NAME` and `from_cell_value_kv`
4. If struct holds a unit (e.g. `CutOffEnergy { value: f64, unit: Option<EnergyUnit> }`), parse the Array manually: index 0 = value, index 1 = unit (optional)

**Why:** ~80 param types follow this pattern; each is a newtype or simple struct wrapping a scalar + optional unit.

## Pattern D — Block with optional unit prefix (castep_cell_data/src/cell/)

Standard sequence:
1. Write failing test for `FromBlock` impl
2. Delete `*Repr` enum and `From<*Repr>` impl
3. Add private `unit_from_first_row::<U: FromCellValue>(rows)` helper in the same file
4. Add `impl FromBlock` using `unit_from_first_row` to peek and slice rows
5. Add `impl FromCellValue` on the unit enum if not already done

Applies to: `LatticeCart`, `LatticeABC`, `ExternalEfield`, `ExternalPressure`, `SpeciesMass`, `HubbardU`
Note: `ExternalPressure` has a triangular tensor — rows have different lengths (3, 2, 1); must index carefully.
Note: `HubbardU` rows after the unit line are `AtomHubbardU` entries, each with `Species + Option<u32> + Vec<OrbitalU>` — these are Pattern E rows nested inside Pattern D.

## Pattern E — Block with Vec<RowType> (castep_cell_data/src/cell/)

Standard sequence:
1. Write failing test for `FromCellValue` (row type) and `FromBlock` (container)
2. Delete `*Repr` enums and `#[serde(transparent)]`
3. Add `impl FromCellValue for RowType` using explicit array indexing and `value_as_*` helpers
4. Add `impl FromBlock for Container` using `.iter().map(RowType::from_cell_value).collect::<CResult<Vec<_>>>()`

Applies to: `PositionsFrac`/`PositionFracEntry`, `KpointsList`/`Kpoint`, `SpeciesLcaoStates`, `SpeciesPot`, `CellConstraints`, `IonicConstraints`, `SymmetryOps`, etc.

Special case: `NonlinearConstraints` — count tokens in the flat row array to determine constraint type (distance=11 tokens, bend=16 tokens, torsion=21 tokens). Collapses the 3-level `Lines → NonlinearConstraintRepr → NonlinearConstraint` serde chain.

## Dependency pattern across crates

`castep_cell_serde` query/parse/format modules must exist BEFORE any `castep_cell_data` migration can begin. The rename (`castep_cell_serde` → `castep_cell_io`) must happen AFTER all `castep_cell_data` migration is complete and tests are green.

**Why:** The crate rename is a filesystem + Cargo.toml operation that breaks all `use castep_cell_serde::` paths at once; doing it before migration would require fixing hundreds of import paths under compilation failure.
