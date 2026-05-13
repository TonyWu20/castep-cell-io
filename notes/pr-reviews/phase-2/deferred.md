# Deferred Items — CellDocument Group Substruct Migration

**Source**: `notes/pr-reviews/phase-2/review.md`
**Date**: 2026-05-13

## 1. Unit Enum CellValue::String → CellValue::Str Round-Trip Fix

Every static-string unit enum emits `CellValue::String(...)` (owned) but reads via `value_as_str()` which only accepts `CellValue::Str` (borrowed). This breaks the direct IR round-trip for any block type with a unit field.

12 of 14 files need the fix (only `length_units.rs` is already fixed). Tracked in TASKS.md "Deferred: Unit Round-Trip Consistency" section.

**Recommendation**: Implement as a single focused PR after the group migration merges.

## 2. Library eprintln! → log::warn!

`ConstraintsParams::validate()` calls `eprintln!()` for the CELL_CONSTRAINTS superseding FIX_ALL_CELL warning. Library crates should not write to stderr unconditionally. Migrate to `log::warn!` or return warnings via the Result type.

**Recommendation**: Address when adding a logging framework to the crate, or switch to returning warnings as structured data.

## 3. Serialization Order Change

The group field order changes the emission order of blocks in `to_cell_file()` output vs the pre-migration ordering. Not functionally breaking (CASTEP is order-independent), but could affect workflows that rely on diff stability.

**Recommendation**: Document in the next CHANGELOG entry. If byte-identical round-trips are required, define a canonical block ordering and sort within each group.
