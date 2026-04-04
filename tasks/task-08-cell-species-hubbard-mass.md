# Task 08: Migrate cell/species/hubbard_u and cell/species/species_mass (Pattern D)

## Objective
Migrate 4 block types with optional unit prefix (Pattern D) from serde to `FromBlock` trait.

## Files (4)
- castep_cell_data/src/cell/species/hubbard_u/hubbard_u_aux.rs
- castep_cell_data/src/cell/species/hubbard_u/atom_hubbard_u_aux.rs
- castep_cell_data/src/cell/species/hubbard_u/mod.rs (if contains types)
- castep_cell_data/src/cell/species/species_mass.rs

## Pattern
Same as Task 07 (Pattern D). Follow the optional unit prefix pattern.

## Acceptance Criteria
- [ ] All 4 files migrated
- [ ] No `*Repr` enums remain
- [ ] `cargo test -p castep_cell_data` passes
- [ ] `cargo clippy` passes

## Dependencies
None (can run in parallel)
