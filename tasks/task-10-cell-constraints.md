# Task 10: Migrate cell/constraints (Pattern E)

## Objective
Migrate 7 constraint block types (Pattern E) from serde to `FromBlock` trait.

## Files (7)
- castep_cell_data/src/cell/constraints/cell_constraints.rs
- castep_cell_data/src/cell/constraints/fix_all_cell.rs
- castep_cell_data/src/cell/constraints/fix_all_ions.rs
- castep_cell_data/src/cell/constraints/fix_com.rs
- castep_cell_data/src/cell/constraints/fix_vol.rs
- castep_cell_data/src/cell/constraints/ionic_constraints.rs
- castep_cell_data/src/cell/constraints/nonlinear_constraints.rs

## Pattern
Same as Task 09 (Pattern E). Each block contains Vec of constraint rows.

## Acceptance Criteria
- [ ] All 7 files migrated
- [ ] Row types implement `FromCellValue` where needed
- [ ] Block types implement `FromBlock`
- [ ] No `*Repr` enums remain
- [ ] `cargo test -p castep_cell_data` passes
- [ ] `cargo clippy` passes

## Dependencies
None (can run in parallel)
