---
name: castep-cell-io current codebase state (as of 2026-04-03)
description: What is already done vs what remains from the serde→trait refactor plan
type: project
---

## Phase 1 status: COMPLETE

All three new modules exist in `castep_cell_serde/src/`:

- `query.rs` — all query + scalar helpers implemented and tested (case-insensitive `find_block`, `find_keyvalue`, `has_flag`, `value_as_*`, `row_as_f64_n`)
- `parse.rs` — all four traits (`FromCellValue`, `FromBlock`, `FromKeyValue`, `FromCellFile`) + primitive impls + `parse<T>` entry point
- `format.rs` — `to_string_direct` + `to_string_many`, double-newline bug fixed, round-trip test against `Mg2SiO4_Cr_1.cell` passing

`lib.rs` exports all new symbols alongside the old serde API. Old `mod de; mod ser;` still present (not removed yet).

## Phase 2 status: NOT STARTED

All 175 `.rs` files in `castep_cell_data/src/` still use `serde::{Deserialize, Serialize}` and `castep_cell_serde::{from_str, to_string}`.

Migration needed for:
- 13 unit enums in `castep_cell_data/src/units/` (Pattern A)
- `Species` enum in `castep_cell_data/src/cell/species/mod.rs` (Pattern A variant — match on CellValue variant, not string)
- ~120 param types across `castep_cell_data/src/param/` (Patterns B + C)
- Pattern D block types: `LatticeCart`, `LatticeABC`, `ExternalEfield`, `ExternalPressure`, `SpeciesMass`, `HubbardU`
- Pattern E block types: `PositionsFrac`, `KpointsList`, `IonicConstraints`, `CellConstraints`, `NonlinearConstraints`, `SpeciesLcaoStates`, `SpeciesPot`, `QuantizationAxis`, `SymmetryOps`

Note: `CutOffEnergy` has `{ value: f64, unit: Option<EnergyUnit> }` — it is NOT a simple newtype; it's Pattern B variant with optional unit inline in a KeyValue Array.
Note: `SymmetryTol` has `{ value: f64, unit: LengthUnit }` (required unit).
Note: `Iprint` maps integers to enum variants (not a string keyword enum) — Pattern B with integer mapping.
Note: `HubbardU` rows are `AtomHubbardU` which themselves contain `OrbitalU` enum variants — need `FromCellValue` on `OrbitalU` first.

## Phase 3 status: NOT STARTED

Crate rename, deletion of `src/de/` and `src/ser.rs`, dropping serde from Cargo.tomls, all import path updates — none done.

**Why:** Phase 3 is a single atomic filesystem+manifest operation that should be done last to minimize broken-state duration.
