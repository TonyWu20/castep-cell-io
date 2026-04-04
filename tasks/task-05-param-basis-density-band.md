# Task 05: Migrate param/basis_set, param/density_mixing, param/band_structure types

## Objective
Migrate 10 basis_set + 8 density_mixing + 8 band_structure types from serde to `FromKeyValue` trait.

## Files (26)
### basis_set (10)
- castep_cell_data/src/param/basis_set/basis_de_dloge.rs
- castep_cell_data/src/param/basis_set/basis_precision.rs
- castep_cell_data/src/param/basis_set/cutoff_energy.rs
- castep_cell_data/src/param/basis_set/fine_gmax.rs
- castep_cell_data/src/param/basis_set/fine_grid_scale.rs
- castep_cell_data/src/param/basis_set/finite_basis_corr.rs
- castep_cell_data/src/param/basis_set/finite_basis_npoints.rs
- castep_cell_data/src/param/basis_set/finite_basis_spacing.rs
- castep_cell_data/src/param/basis_set/fixed_npw.rs
- castep_cell_data/src/param/basis_set/grid_scale.rs

### density_mixing (8)
- castep_cell_data/src/param/density_mixing/mix_charge_amp.rs
- castep_cell_data/src/param/density_mixing/mix_charge_gmax.rs
- castep_cell_data/src/param/density_mixing/mix_cut_off_energy.rs
- castep_cell_data/src/param/density_mixing/mix_history_length.rs
- castep_cell_data/src/param/density_mixing/mix_metric_q.rs
- castep_cell_data/src/param/density_mixing/mix_spin_amp.rs
- castep_cell_data/src/param/density_mixing/mix_spin_gmax.rs
- castep_cell_data/src/param/density_mixing/mixing_scheme.rs

### band_structure (8)
- castep_cell_data/src/param/band_structure/bs_eigenvalue_tol.rs
- castep_cell_data/src/param/band_structure/bs_max_cg_steps.rs
- castep_cell_data/src/param/band_structure/bs_max_iter.rs
- castep_cell_data/src/param/band_structure/bs_nbands.rs
- castep_cell_data/src/param/band_structure/bs_nextra_bands.rs
- castep_cell_data/src/param/band_structure/bs_perc_extra_bands.rs
- castep_cell_data/src/param/band_structure/bs_re_est_k_scrn.rs
- castep_cell_data/src/param/band_structure/bs_xc_functional.rs

## Pattern
Same as Task 01. Follow Pattern B or C.

## Acceptance Criteria
- [ ] All 26 files migrated
- [ ] `cargo test -p castep_cell_data` passes
- [ ] `cargo clippy` passes

## Dependencies
None (can run in parallel)
