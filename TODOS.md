# CASTEP Implementation TODOs

## Summary

- Total cell: 44, Implemented: 20, Missing: 24
- Total param: 194, Implemented: 146, Missing: 48

## Missing Cell Keywords

### Group: Brillouin zone sampling k-points

#### Pattern 2: Simple Scalar Type

- **BS_KPOINT_PATH_SPACING** (Type: Real, Default: 0.1 1/ang, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_bs_kpoint_path_spacing_castep.htm)
- **KPOINTS_MP_SPACING** (Type: Real, Default: 0.1 1/Å, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_kpoints_mp_spacing_castep.htm)
#### Pattern 4: Block Type

- **BS_KPOINT_PATH** (Type: Block, Default: , URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_bs_kpoint_path_castep.htm)
- **BS_KPOINT_LIST** (Type: Block, Default: , URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_bs_kpoint_list_castep.htm)
- **KPOINTS_MP_GRID** (Type: Block, Default: Generate the Monkhorst-Pack grid parameters from the KPOINTS_MP_SPACING value., URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_kpoints_mp_grid_castep.htm)
- **MAGRES_KPOINTS_LIST** (Type: Block, Default: , URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_magres_kpoints_list_castep.htm)
- **OPTICS_KPOINTS_LIST** (Type: Block, Default: , URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_optics_kpoints_list_castep.htm)

### Group: Ionic positions

#### Pattern 2: Simple Scalar Type

- **MAGMOM** (Type: Qualifier, Default: , URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_magmom_castep.htm)
- **MIXTURE** (Type: Qualifier, Default: , URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_mixture_castep.htm)
#### Pattern 4: Block Type

- **POSITIONS_ABS** (Type: Block, Default: of Å is used., URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_positions_abs_castep.htm)
- **POSITIONS_ABS_INTERMEDIATE** (Type: Block, Default: , URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_positions_abs_intermediate_castep.htm)
- **POSITIONS_ABS_PRODUCT** (Type: Block, Default: , URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_positions_abs_product_castep.htm)
- **POSITIONS_FRAC_INTERMEDIATE** (Type: Block, Default: , URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_positions_frac_intermediate_castep.htm)
- **POSITIONS_FRAC_PRODUCT** (Type: Block, Default: , URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_positions_frac_product_castep.htm)

### Group: Ionic velocities

#### Pattern 4: Block Type

- **IONIC_VELOCITIES** (Type: Block, Default: of Å/ps is used., URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_ionic_velocities_castep.htm)

### Group: Species characteristics

#### Pattern 4: Block Type

- **SPECIES_Q** (Type: Block, Default: quadrupole moment for that species. However, if the symbol specified for a species is not a standard symbol in the periodic table, the, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_species_q_castep.htm)
- **SEDC_CUSTOM_PARAMS** (Type: Block, Default: , URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_sedc_custom_params_castep.htm)

### Group: q-vectors for phonon calculations

#### Pattern 2: Simple Scalar Type

- **PHONON_KPOINT_PATH_SPACING** (Type: Real, Default: 0.1 1/ang, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_phonon_kpoint_path_spacing_castep.htm)
#### Pattern 4: Block Type

- **PHONON_GAMMA_DIRECTIONS** (Type: Block, Default: , URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_phonon_gamma_directions_castep.htm)
- **PHONON_KPOINT_PATH** (Type: Block, Default: , URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_phonon_kpoint_path_castep.htm)
- **PHONON_KPOINT_LIST** (Type: Block, Default: , URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_phonon_kpoint_list_castep.htm)
- **PHONON_FINE_KPOINT_LIST** (Type: Block, Default: , URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_phonon_fine_kpoint_list_castep.htm)
- **PHONON_SUPERCELL_MATRIX** (Type: Block, Default: , URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_phonon_supercell_matrix_castep.htm)
- **SUPERCELL_KPOINT_LIST_CASTEP** (Type: Block, Default: , URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_supercell_kpoint_list_castep.htm)

## Missing Param Keywords

### Group: Electronic excitations parameters

#### Pattern 2: Simple Scalar Type

- **SPECTRAL_TASK** (Type: String, Default: BandStructure, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_spectral_task.htm)
- **TDDFT_NUM_STATES** (Type: Integer, Default: 0, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_tddft_num_states.htm)
- **TDDFT_POSITION_METHOD** (Type: String, Default: MOLECULAR, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_tddft_position_method.htm)
- **TDDFT_SELECTED_STATE** (Type: Integer, Default: 0, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_tddft_selected_state.htm)

### Group: Electronic parameters

#### Pattern 2: Simple Scalar Type

- **PERC_EXTRA_BANDS** (Type: Real, Default: 0, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_perc_extra_bands_castep.htm)
- **SEDC_APPLY** (Type: Logical, Default: FALSE, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_sedc_apply_castep.htm)
- **SEDC_D_G06** (Type: Real, Default: The default value for this parameter depends on the value of XC_FUNCTIONAL. Please use the scientific literature for additional guidance., URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_sedc_d_g06_castep.htm)
- **SEDC_D_JCHS** (Type: Real, Default: The default value for this parameter depends on the value of XC_FUNCTIONAL. Please use the scientific literature for additional guidance., URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_sedc_d_jchs_castep.htm)
- **SEDC_D_TS** (Type: Real, Default: The default value for this parameter depends on the value of XC_FUNCTIONAL. Please use the scientific literature for additional guidance., URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_sedc_d_ts_castep.htm)
- **SEDC_LAMBDA_OBS** (Type: Real, Default: The default value for this parameter depends on the value of XC_FUNCTIONAL. Please use the scientific literature for additional guidance., URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_sedc_lambda_obs_castep.htm)
- **SEDC_N_OBS** (Type: Real, Default: The default value for this parameter depends on the value of XC_FUNCTIONAL. Please use the scientific literature for additional guidance., URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_sedc_n_obs_castep.htm)
- **SEDC_S6_G06** (Type: Real, Default: The default value for this parameter depends on the value of XC_FUNCTIONAL. Please use the scientific literature for additional guidance., URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_sedc_s6_g06_castep.htm)
- **SEDC_S6_JCHS** (Type: Real, Default: The default value for this parameter depends on the value of XC_FUNCTIONAL. Please use the scientific literature for additional guidance., URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_sedc_s6_jchs_castep.htm)
- **SEDC_SR_JCHS** (Type: Real, Default: The default value for this parameter depends on the value of XC_FUNCTIONAL. Please use the scientific literature for additional guidance., URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_sedc_sr_jchs_castep.htm)
- **SEDC_SR_TS** (Type: Real, Default: The default value for this parameter depends on the value of XC_FUNCTIONAL. Please use the scientific literature for additional guidance., URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_sedc_sr_ts_castep.htm)
- **SEDC_SCHEME** (Type: String, Default: TS, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_sedc_scheme_castep.htm)

### Group: Exchange-correlation parameters

#### Pattern 2: Simple Scalar Type

- **NLXC_EXCHANGE_REFLECT_KPTS, EXCHANGE_REFLECT_KPTS** (Type: Logical, Default: TRUE, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_exchange_reflect_kpts_castep.htm)
- **NLXC_IMPOSE_TRS, IMPOSE_TRS** (Type: Logical, Default: FALSE, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_impose_trs_castep.htm)
- **NLXC_K_SCRN_AVERAGING_SCHEME, K_SCRN_AVERAGING_SCHEME** (Type: String, Default: AVE_DEN, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_k_scrn_averaging_scheme_castep.htm)
- **NLXC_PAGE_EX_POT, PAGE_EX_POT** (Type: Integer, Default: 0, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_page_ex_pot_castep.htm)
- **NLXC_PPD_SIZE_X, PPD_SIZE_X** (Type: Integer, Default: , URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_ppd_size_castep.htm)
- **NLXC_PPD_SIZE_Y, PPD_SIZE_Y** (Type: Integer, Default: , URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_ppd_size_castep.htm)
- **NLXC_PPD_SIZE_Z, PPD_SIZE_Z** (Type: Integer, Default: , URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_ppd_size_castep.htm)
- **NLXC_PPD_INTEGRAL, PPD_INTEGRAL** (Type: Logical, Default: FALSE, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_ppd_integral_castep.htm)
- **NLXC_RE_EST_K_SCRN, RE_EST_K_SCRN** (Type: Logical, Default: FALSE, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_re_est_k_scrn_castep.htm)
#### Pattern 4: Block Type

- **XC_DEFINITION** (Type: Block, Default: value is 1.437 Å-1 = 0.76 Bohr-1., URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_xc_definition_castep.htm)

### Group: General parameters

#### Pattern 2: Simple Scalar Type

- **CHARGE_UNIT** (Type: String, Default: e, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_charge_unit_castep.htm)

### Group: Optics

#### Pattern 1: Simple Keyword Enum

- **OPTICS_XC_FUNCTIONAL** (Type: String, Default: The default value  is derived from the value for XC_FUNCTIONAL., URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_optics_xc_functional_castep.htm)
#### Pattern 2: Simple Scalar Type

- **OPTICS_NBANDS** (Type: Integer, Default: If OPTICS_NBANDS is present in the parameter file, the value of BS_NBANDS is determined by this keyword., URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_optics_nbands_castep.htm)
- **OPTICS_NEXTRA_BANDS** (Type: Integer, Default: 0, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_optics_nextra_bands_castep.htm)
- **OPTICS_PERC_EXTRA_BANDS** (Type: Real, Default: 0, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_optics_perc_extra_bands_castep.htm)

### Group: Phonon parameters

#### Pattern 2: Simple Scalar Type

- **PHONON_METHOD** (Type: String, Default: LINEARRESPONSE, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_phonon_method_castep.htm)

### Group: Solvation energy parameters

#### Pattern 1: Simple Keyword Enum

- **BOUNDARY_TYPE** (Type: String, Default: PERIODIC, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_boundary_type.htm)
#### Pattern 2: Simple Scalar Type

- **DIELEC_EMB_BULK_PERMITTIVITY** (Type: Real, Default: 78.54 (water) if DIELEC_EMB_FUNC_METHOD is set to FG, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_dielec_emb_bulk_permittivity.htm)
- **DIELEC_EMB_FUNC_METHOD** (Type: String, Default: FG if TASK is set to AUTOSOLVATION, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_dielec_emb_func_method.htm)
- **IMPLICIT_SOLVENT_APOLAR_FACTOR** (Type: Real, Default: value gives excellent results for hydrocarbons in water. It gives good results for other solutes in water, almost universally improving on results obtained without the dispersion-repulsion term. The expected accuracy for other solvents is lower, but still it is worthwhile including., URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_implicit_solvent_apolar_factor.htm)
- **IMPLICIT_SOLVENT_APOLAR_TERM** (Type: Logical, Default: FALSE, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_implicit_solvent_apolar_term.htm)
- **IMPLICIT_SOLVENT_SURFACE_TENSION** (Type: Real, Default: 4.7624e-05 Ha/Bohr2 corresponds to water as a solvent (equivalent to 0.0728 N/m), URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_implicit_solvent_surface_tension.htm)
- **USE_SMEARED_IONS** (Type: Logical, Default: FALSE, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_use_smeared_ions.htm)

### Group: Transition state search

#### Pattern 2: Simple Scalar Type

- **TSSEARCH_CG_MAX_ITER** (Type: Integer, Default: 20, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_tssearch_cg_max_iter_castep.htm)
- **TSSEARCH_DISP_TOL** (Type: Real, Default: 0.01 Bohr, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_tssearch_disp_tol_castep.htm)
- **TSSEARCH_ENERGY_TOL** (Type: Real, Default: 0.00002 eV/atom, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_tssearch_energy_tol_castep.htm)
- **TSSEARCH_FORCE_TOL** (Type: Real, Default: 0.005 Hartree Bohr-1, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_tssearch_force_tol_castep.htm)
- **TSSEARCH_LSTQST_PROTOCOL** (Type: String, Default: CompleteLSTQST, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_tssearch_lstqst_protocol_castep.htm)
- **TSSEARCH_MAX_PATH_POINTS** (Type: Integer, Default: 10, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_tssearch_max_path_points_castep.htm)
- **TSSEARCH_METHOD** (Type: String, Default: LSTQST, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_tssearch_method_castep.htm)
- **TSSEARCH_QST_MAX_ITER** (Type: Integer, Default: 5, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_tssearch_qst_max_iter_castep.htm)

### Group: Units

#### Pattern 2: Simple Scalar Type

- **LENGTH_UNIT** (Type: String, Default: ang, URL: https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_length_unit_castep.htm)

