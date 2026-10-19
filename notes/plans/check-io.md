# Plan: `castep-check-io` — typed I/O for `.check` / `.castep_bin`

Status: in progress. ADR-0005 records the crate decision and the version design.
See `docs/adr/0005-check-io-crate.md`.

## Goal

Read, edit, and write CASTEP binary model files with typed document structs
like `CellDocument`/`ParamDocument`. Validate the writer with CASTEP-6.11:
a job with `continuation: default` must reuse our written `.check` ground
state and finish without error.

## Format facts (verified against source + 5 real files)

- Framing: Fortran sequential unformatted, 4-byte big-endian record markers
  (header + trailer). Every `obj/platforms/*.mk` sets big-endian convert;
  gfortran adds `-frecord-marker=4`.
- Physical sizes: i32 = 4B, f64 = 8B, complex128 = 16B, logical = 4B
  (0x00000000 / 0x00000001), char(N) = N bytes, space/NUL padded.
- `.check`: first record is `BEGIN_PARAMETERS_DUMP` (30B tag).
  `.castep_bin`: first record is the 10B `CASTEP_BIN` magic, then the same
  parameters section, but no wavefunction section.
- Layout order: `[magic]` parameters (23 blocks, 30B tags) -> cell
  (`BEGIN_UNIT_CELL`..`END_CELL_GLOBAL`, 256B tags) -> orig cell (same) ->
  found_gs_wvfn (4B) -> found_gs_den (4B) -> total_energy (8B) ->
  fermi_energy(1) (8B) -> nbands_max + nspins (8B: 2 i32) ->
  [wave section: 'wave' 4B tag, grid 12B, dims 16B, per-(spin,kpt) blocks] ->
  eigenvalues (per kpt: 3 f64; per spin: nbands occ + nbands eig) ->
  found_gs_den (4B) -> fine grid (12B) -> density columns
  (per (nx,ny): 2 i32 + ngz complex128, + ngz complex128 spin when nspins=2)
  -> optional sections (10B tags: FORCES, STRESS, E_FERMI, BFGS, MD,
  LDAUOCCM, ...) -> `END` (10B).
- Unknown optional tag = hard abort in CASTEP. A writer may only emit known
  tags.

## Version gate table (parameters section only)

Writer: unconditional, full 6.11 layout. Reader: gates below, from
`parameters_restore` line 9090 ff. `X` = file version number.

| Gate | Records read only when X >= gate |
| --- | --- |
| base | all base records, incl. `md_nose_t`/`md_langevin_t` legacy pair, the two `dummy_int` electron records, `dummy_logical` phonon record, three `dummy_real` wannier records |
| 3.0 | `backup_interval`, `force_constant_unit`, `volume_unit`, `opt_strategy_bias`; 8 `nlxc_*` (page_ex_pot..re_est_k_scrn); `fixed_npw`; `bs_xc_functional`, `bs_re_est_k_scrn`; `md_barostat`, `md_ion_t`, `md_cell_t`, `md_nhc_length`; 7 phonon records (dos_spacing..born_charge_sum_rule) |
| 3.1 | `calculate_densdiff`, `num_farms_requested`, `num_proc_in_smp`; `md_use_pathint`, `md_num_beads`; 5 phonon records (calculate_dos, dos_limit, force_constant_cutoff, fine_method, method) |
| 3.2 | `num_proc_in_smp_fine`, `message_size`, `requested_seed`; `dummy_int` x2 + `spin_polarized` + `electronic_minimizer`; `thermo_t_spacing` |
| 4.0 | `ir_intensity_unit`; 4 `nlxc_div_corr_*`; `fine_grid_scale`; `geom_spin_fix`, `geom_linmin_tol`; `md_pathint_staging`, `md_pathint_num_stages`, `md_sample_iter`; `efield_freq_spacing`, `efield_oscillator_q`; 5 `wannier_*` (spread_tol..ion_cmoments) |
| 4.1 | `print_memory_usage`, `write_formatted_potential`, `write_formatted_density`, `dipole_unit`, `efield_unit`, `calc_molecular_dipole`; `nelectrons`, `nup`, `ndown`, `spin`, `charge`; `elec_force_tol`; 5 `md_eqm_*`/`md_elec_force_tol` |
| 4.2 | `write_orbitals` |
| 4.3 | `calculate_elf`, `write_formatted_elf`, `cml_output`, `cml_filename`; `magres_max_sc_cycles`, `magres_jcoupling_task`, `magres_write_response` |
| 4.4 | `fft_max_prime_factor`; `bs_write_eigenvalues`; `phonon_force_constant_cut_scale`, `phonon_sum_rule_method`, `calculate_raman` |
| 5.0 | `calculate_Hirshfeld`; `sedc_apply`, `sedc_scheme`, `nlxc_exchange_screening`; `phonon_fine_cutoff_method` |
| 5.5 | `entropy_unit`, `write_cif_structure`, `write_cell_structure`; `geom_use_linmin`; `md_pathint_init` |
| 6.0 | `write_bib`; `geom_lbfgs_max_updates`; `md_use_plumed`; 6 phonon/raman records; `efield_dfpt_method` |
| 6.1 | `nlxc_exchange_fraction`; `dipole_correction`, `dipole_dir`; `geom_tpsd_init_stepsize`, `geom_tpsd_iterchange` |

MD thermostat rule: after `md_thermostat`, exactly 2 f64 legacy records in a
fixed slot. NOSE: `(md_ion_t, 10*md_ion_t)`. LANGEVIN:
`(0.1*md_ion_t, md_ion_t)`. Reader stores the raw pair. Writer emits the
stored pair. No recompute.

Layout floor: 3.0. The cell and electronic sections have no gates.

## Work breakdown (actual shape)

The implementation is table-driven, not one typed struct per block. The
specs in `check-params-spec.txt` and the two `check-cell_dump_*` files feed
three data tables; the codec replays raw payloads so a roundtrip is
byte-exact.

1. [done] Scaffold the crate in the workspace. Features per ADR-0005.
   `cargo build` green under all feature combos.
2. [done] `record.rs`: big-endian record reader/writer. Primitives: i32,
   f64, complex128, logical4, char(n). Unit tests incl. marker-mismatch
   and truncated-file errors.
3. [done] `version.rs`: version record parse/compare. Gate table as data
   (`gate::V3_0`...`gate::V6_1`). Errors `VersionTooNew`/`VersionTooOld`.
4. [done] `params.rs`: the 340-field `PARAM_FIELDS` table (generated from
   `check-params-spec.txt`, byte-verified against `Cu111_CO.castep_bin`).
   Gate-aware read into `RawParams`; full-layout write that fills absent
   records with default encodings.
5. [done] `cell.rs`: the `UNIT_CELL` and `CELL_GLOBAL` tables with
   derived `Size` expressions and count-gated pairs. `RawCell` stores
   payloads keyed by the 256-byte tag. The global section inherits
   `ns`/`mi` from the preceding unit cell section.
6. [done] `electronic.rs`: flags, energies, `WaveSection` (jagged blocks,
   i32 plane-wave grid records), `EigenKpt` records, raw density columns,
   and `OptionalSection` (typed `E_FERMI`/`FORCES`/`STRESS`/`MD`, raw
   passthrough for the rest).
7. [done] `document.rs`: `CheckDocument` with `from_bytes`,
   `from_check_file`, `from_castep_bin_file`, `to_check_bytes`,
   `to_castep_bin_bytes`, `to_check_file`, `to_castep_bin_file`. Kind
   detection from the first record.
8. [done] Roundtrip tests (`tests/roundtrip.rs`): byte-identical
   `from_bytes` -> `to_bytes` for Cu111_CO.check, Cu111_CO.castep_bin,
   Cu111_CO_F7.check, Fe2O3.castep_bin, and the 0614 spin job `.check`.
   `tests/codec.rs` covers the error paths and synthetic streams.
9. [in progress] CASTEP-6.11 oracle via slurm (see below). Job 3592
   submitted with the eigensolve binary (root cause of the 3590/3591
   failures: the Sep 30 `~/Downloads` build has a local `devel_code`
   256-byte patch, so it cannot read standard-layout model files; the
   0614 seed `.check` is standard layout, 80-byte `devel_code`).
10. [done] Docs: ADR-0005 carries the fixed layout facts. The crate
    README, CHANGELOG entry, and CONTEXT note are written.

## Oracle test design

- Job dir: `/export/public_castep_jobs/tony/castep_check_io_oracle/<seed>/`.
- Seed files from an existing good job (e.g.
  `Cu111_CO_Single_Point_0614_spin_cpu_serial`): `.cell`, `.usp`.
- `.param`: load with `ParamDocument`, set
  `general.continuation = default`, save.
- `.check`: load the existing good `.check` with `castep-check-io`, save as
  `<seed>.check`.
- Run: slurm job, serial, `castep.mpi` from
  `/export/castep-rust-eigensolve/castep/obj/linux_x86_64_gfortran/`,
  under `nix develop /export/castep-rust-eigensolve/castep` (the launch
  pattern proven by the 0604 warm-start job). The `~/Downloads` Sep 30
  build is unusable for reading standard files: its local
  `devel_code = file_maxpath` (256B) patch desyncs `parameters_restore`
  on the 80-byte `devel_code` record ("I/O past end of record").
  The eigensolve tree differs from the reference 6.11 tree only in
  diagnostics and the `OCC_WEIGHTED` keyword; the model-file layout is
  unchanged.
- Oracle asserts: exit 0. Log shows continuation ground-state reuse. No
  error. The calc is skipped because the stored model matches the live
  `.cell`/`.param`.
- Edit variant: change one non-dependency parameter in `.param` via
  `ParamDocument` and the matching record in `.check` via `CheckDocument`.
  Expect a clean recompute, no error.

## Open items (need owner decision at oracle time)

- [settled] Seed job for the spin test: `Cu111_CO_Single_Point_0614_spin_cpu_serial`.
- [settled, revised] CASTEP binary path:
  `/export/castep-rust-eigensolve/castep/obj/linux_x86_64_gfortran/castep.mpi`.
  The Sep 30 `~/Downloads` build was tried first (jobs 3590, 3591): it
  aborts in `parameters_restore` on any standard-layout file because of
  its local 256-byte `devel_code` patch. The eigensolve build keeps the
  standard 80-byte `devel_code` and its source differs from the 6.11
  reference only in diagnostics plus the `OCC_WEIGHTED` mixing keyword.
- Optional-section typing scope for v1: the four small headers are typed
  (`E_FERMI`, `FORCES`, `STRESS`, `MD`); everything else is raw passthrough.
  A wider typed set can land later without a format change.

## Coverage target

>= 85% unit coverage on all new modules, per repo convention.
