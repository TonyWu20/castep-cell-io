# Task 04: Migrate param/geometry_optimization and param/phonon types

## Objective
Migrate 11 geometry_optimization + 12 phonon types from serde to `FromKeyValue` trait.

## Files (23)
### geometry_optimization (11)
- castep_cell_data/src/param/geometry_optimization/geom_convergence_win.rs
- castep_cell_data/src/param/geometry_optimization/geom_disp_tol.rs
- castep_cell_data/src/param/geometry_optimization/geom_energy_tol.rs
- castep_cell_data/src/param/geometry_optimization/geom_force_tol.rs
- castep_cell_data/src/param/geometry_optimization/geom_frequency_est.rs
- castep_cell_data/src/param/geometry_optimization/geom_max_iter.rs
- castep_cell_data/src/param/geometry_optimization/geom_method.rs
- castep_cell_data/src/param/geometry_optimization/geom_modulus_est.rs
- castep_cell_data/src/param/geometry_optimization/geom_preconditioner.rs
- castep_cell_data/src/param/geometry_optimization/geom_spin_fix.rs
- castep_cell_data/src/param/geometry_optimization/geom_stress_tol.rs

### phonon (12)
- castep_cell_data/src/param/phonon/born_charge_sum_rule.rs
- castep_cell_data/src/param/phonon/calculate_born_charges.rs
- castep_cell_data/src/param/phonon/phonon_calc_lo_to_splitting.rs
- castep_cell_data/src/param/phonon/phonon_convergence_win.rs
- castep_cell_data/src/param/phonon/phonon_energy_tol.rs
- castep_cell_data/src/param/phonon/phonon_fine_method.rs
- castep_cell_data/src/param/phonon/phonon_finite_disp.rs
- castep_cell_data/src/param/phonon/phonon_force_constant_cutoff.rs
- castep_cell_data/src/param/phonon/phonon_max_cg_steps.rs
- castep_cell_data/src/param/phonon/phonon_max_cycles.rs
- castep_cell_data/src/param/phonon/phonon_method.rs
- castep_cell_data/src/param/phonon/phonon_sum_rule.rs

## Pattern
Same as Task 01. Follow Pattern B or C.

## Acceptance Criteria
- [ ] All 23 files migrated
- [ ] `cargo test -p castep_cell_data` passes
- [ ] `cargo clippy` passes

## Dependencies
None (can run in parallel)
