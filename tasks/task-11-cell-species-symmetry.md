# Task 11: Migrate remaining cell/species and cell/symmetry (Pattern E)

## Objective
Migrate remaining 7 block types (Pattern E) from serde to `FromBlock` trait.

## Files (7)
### species (5)
- castep_cell_data/src/cell/species/quantization_axis.rs
- castep_cell_data/src/cell/species/species_lcao_states.rs
- castep_cell_data/src/cell/species/species_pot.rs

### symmetry (2)
- castep_cell_data/src/cell/symmetry/symmetry_ops.rs
- castep_cell_data/src/cell/symmetry/symmetry_tol.rs

## Pattern
Same as Task 09 (Pattern E). Each block contains Vec of data rows.

## Acceptance Criteria
- [ ] All 7 files migrated
- [ ] Row types implement `FromCellValue` where needed
- [ ] Block types implement `FromBlock`
- [ ] No `*Repr` enums remain
- [ ] `cargo test -p castep_cell_data` passes
- [ ] `cargo clippy` passes

## Dependencies
None (can run in parallel)
