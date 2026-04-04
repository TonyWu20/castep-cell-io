# Task 02: Migrate param/electronic_minimisation types

## Objective
Migrate 15 param types in `electronic_minimisation/` module from serde to `FromKeyValue` trait.

## Files (15)
- castep_cell_data/src/param/electronic_minimisation/efermi_tol.rs
- castep_cell_data/src/param/electronic_minimisation/elec_convergence_win.rs
- castep_cell_data/src/param/electronic_minimisation/elec_dump_file.rs
- castep_cell_data/src/param/electronic_minimisation/elec_eigenvalue_tol.rs
- castep_cell_data/src/param/electronic_minimisation/elec_energy_tol.rs
- castep_cell_data/src/param/electronic_minimisation/elec_restore_file.rs
- castep_cell_data/src/param/electronic_minimisation/electronic_minimizer.rs
- castep_cell_data/src/param/electronic_minimisation/fix_occupancy.rs
- castep_cell_data/src/param/electronic_minimisation/max_cg_steps.rs
- castep_cell_data/src/param/electronic_minimisation/max_scf_cycles.rs
- castep_cell_data/src/param/electronic_minimisation/max_sd_steps.rs
- castep_cell_data/src/param/electronic_minimisation/metals_method.rs
- castep_cell_data/src/param/electronic_minimisation/num_dump_cycles.rs
- castep_cell_data/src/param/electronic_minimisation/smearing_scheme.rs
- castep_cell_data/src/param/electronic_minimisation/smearing_width.rs
- castep_cell_data/src/param/electronic_minimisation/spin_fix.rs

## Pattern
Same as Task 01. Follow Pattern B (newtype) or Pattern C (keyword enum).

## Acceptance Criteria
- [ ] All 15 files migrated
- [ ] `cargo test -p castep_cell_data` passes
- [ ] `cargo clippy` passes

## Dependencies
None (can run in parallel)
