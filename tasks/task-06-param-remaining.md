# Task 06: Migrate remaining param types (electronic, efield, nmr, population_analysis, pseudopotential, exchange_correlation)

## Objective
Migrate remaining 20 param types from serde to `FromKeyValue` trait.

## Files (20)
### electronic (7)
- castep_cell_data/src/param/electronic/charge.rs
- castep_cell_data/src/param/electronic/nbands.rs
- castep_cell_data/src/param/electronic/ndown.rs
- castep_cell_data/src/param/electronic/nelectrons.rs
- castep_cell_data/src/param/electronic/nextra_bands.rs
- castep_cell_data/src/param/electronic/nup.rs
- castep_cell_data/src/param/electronic/spin.rs

### efield (5, excluding already-migrated efield_energy_tol.rs)
- castep_cell_data/src/param/efield/efield_calc_ion_permittivity.rs
- castep_cell_data/src/param/efield/efield_calculate_nonlinear.rs
- castep_cell_data/src/param/efield/efield_convergence_win.rs
- castep_cell_data/src/param/efield/efield_ignore_mol_modes.rs
- castep_cell_data/src/param/efield/efield_max_cg_steps.rs
- castep_cell_data/src/param/efield/efield_max_cycles.rs

### nmr (3)
- castep_cell_data/src/param/nmr/magres_conv_tol.rs
- castep_cell_data/src/param/nmr/magres_max_cg_steps.rs
- castep_cell_data/src/param/nmr/magres_method.rs

### population_analysis (4)
- castep_cell_data/src/param/population_analysis/pdos_calculate_weights.rs
- castep_cell_data/src/param/population_analysis/popn_bond_cutoff.rs
- castep_cell_data/src/param/population_analysis/popn_calculate.rs
- castep_cell_data/src/param/population_analysis/popn_write.rs

### pseudopotential (3)
- castep_cell_data/src/param/pseudopotential/pspot_beta_phi_type.rs
- castep_cell_data/src/param/pseudopotential/pspot_nonlocal_type.rs
- castep_cell_data/src/param/pseudopotential/relativistic_treatment.rs

### exchange_correlation (2)
- castep_cell_data/src/param/exchange_correlation/spin_polarized.rs
- castep_cell_data/src/param/exchange_correlation/xc_functional.rs

## Pattern
Same as Task 01. Follow Pattern B or C.

## Acceptance Criteria
- [ ] All 20 files migrated
- [ ] `cargo test -p castep_cell_data` passes
- [ ] `cargo clippy` passes

## Dependencies
None (can run in parallel)
