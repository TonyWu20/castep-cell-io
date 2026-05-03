---
name: castep-cell-io task decomposition patterns
description: Recurring task archetypes and dependency patterns for castep-cell-io phase plans
type: project
---

## Task Archetypes

This project has a highly regular structure. All keyword types follow one of four patterns:

1. **Key-value newtype** (`pub struct Foo(pub [T; 3])`) -- tuple struct wrapping a 3-element array.
   Traits: `FromCellValue`, `FromKeyValue`, `ToCell`, `ToCellValue`. No bon::Builder.
   Template: `kpoints_mp_grid.rs`. Used for MP grid, MP offset types.
   Task pattern: one `lib-tdd` task creates the file + registers in parent mod.rs.

2. **Key-value spacing** (`{ value: f64, unit: Option<InvLengthUnit> }`) -- 2-field struct.
   Same traits as newtype. No bon::Builder. Template: `kpoints_mp_spacing.rs`.
   Task pattern: one `lib-tdd` task creates the file + registers in parent mod.rs.

3. **Block type with entries** -- container with `Vec<Entry>` field + separate entry type.
   Container: `FromBlock` + `ToCell` + `bon::Builder`. Entry: `FromCellValue` + `ToCellValue` + `bon::Builder`.
   Template: `bs_kpoint_path.rs`. Used for k-point path types.
   Task pattern: one `lib-tdd` task creates the file (both container + entry in same file) + registers in parent mod.rs.

4. **Flag/tag type** -- unit struct with only `ToCell` producing `Cell::Flag("NAME")`.
   Template: pattern established by `SymmetryGenerate` in G3.
   Task pattern: one `lib-tdd` task creates the file + registers in parent mod.rs.

## Dependency Patterns

- **G1 establishes wiring patterns** that G2-G5 follow. G1 is always done first.
- **G2 is standalone** (one task, one file). Can be done in parallel with G1 tasks that don't touch cell_document.rs.
- **G3 is the smallest new-type goal** -- establishes the file-creation workflow for G4/G5.
- **G4 and G5 are mass file creation** -- many similar types. Type creation tasks run sequentially within each group because they share a parent mod.rs.
- **CellDocument wiring is ALWAYS a separate task** that depends on all type-creation tasks in the group completing.
- **All groups depend on the previous group** because CellDocument accumulates fields across all groups.

## Wiring Chain (6 steps, verified by wiring_checklist)

1. `mod xxx;` in parent mod.rs
2. `pub use xxx::TypeName;` in parent mod.rs
3. Import in cell_document.rs
4. Field on CellDocument struct
5. Parsing in from_cell_file
6. Serialization in to_cell_file
7. Field in Ok(CellDocument { ... }) constructor

## Common Pitfall: BS_ alias overlap

Spectral types with BS_ equivalent have BS_ backward compat aliases. This causes dual-population with existing BS_ types when parsing. Mitigation: parse spectral BEFORE BS_ blocks. Deferred validation handles mutual exclusion.

## Crate Boundary

All type definitions and wiring happen in `castep_cell_io`. No changes to `castep_cell_fmt`. The `FromKeyValue::from_cells` method (provided by castep_cell_fmt) is the canonical key-value parsing pattern.
