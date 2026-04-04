# Task 03: Migrate param/molecular_dynamics types

## Objective
Migrate 22 param types in `molecular_dynamics/` module from serde to `FromKeyValue` trait.

## Files (22)
- castep_cell_data/src/param/molecular_dynamics/md_barostat.rs
- castep_cell_data/src/param/molecular_dynamics/md_cell_t.rs
- castep_cell_data/src/param/molecular_dynamics/md_damping_reset.rs
- castep_cell_data/src/param/molecular_dynamics/md_damping_scheme.rs
- castep_cell_data/src/param/molecular_dynamics/md_delta_t.rs
- castep_cell_data/src/param/molecular_dynamics/md_elec_convergence_win.rs
- castep_cell_data/src/param/molecular_dynamics/md_elec_eigenvalue_tol.rs
- castep_cell_data/src/param/molecular_dynamics/md_elec_energy_tol.rs
- castep_cell_data/src/param/molecular_dynamics/md_ensemble.rs
- castep_cell_data/src/param/molecular_dynamics/md_eqm_cell_t.rs
- castep_cell_data/src/param/molecular_dynamics/md_eqm_ion_t.rs
- castep_cell_data/src/param/molecular_dynamics/md_eqm_method.rs
- castep_cell_data/src/param/molecular_dynamics/md_eqm_t.rs
- castep_cell_data/src/param/molecular_dynamics/md_extrap.rs
- castep_cell_data/src/param/molecular_dynamics/md_extrap_fit.rs
- castep_cell_data/src/param/molecular_dynamics/md_ion_t.rs
- castep_cell_data/src/param/molecular_dynamics/md_nhc_length.rs
- castep_cell_data/src/param/molecular_dynamics/md_num_iter.rs
- castep_cell_data/src/param/molecular_dynamics/md_opt_damped_delta_t.rs
- castep_cell_data/src/param/molecular_dynamics/md_temperature.rs
- castep_cell_data/src/param/molecular_dynamics/md_thermostat.rs

## Pattern
Same as Task 01. Follow Pattern B or C.

## Acceptance Criteria
- [ ] All 21 files migrated
- [ ] `cargo test -p castep_cell_data` passes
- [ ] `cargo clippy` passes

## Dependencies
None (can run in parallel)
