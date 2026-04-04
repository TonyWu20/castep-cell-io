use crate::param::{
    band_structure::*, basis_set::*, density_mixing::*, efield::*, electronic::*,
    electronic_excitations::*, electronic_minimisation::*, exchange_correlation::*,
    general::*, geometry_optimization::*, molecular_dynamics::*, nmr::*, optics::*,
    phonon::*, population_analysis::*, pseudopotential::*, solvation::*, transition_state::*,
};
use castep_cell_io::{Cell, CResult, Error, FromCellFile, ToCellFile, ToCell, FromKeyValue, FromBlock};
use derive_builder::Builder;

/// Top-level aggregate struct for .param files
#[derive(Debug, Clone, Builder)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct ParamDocument {
    // === General (21 fields) ===
    #[builder(default, setter(strip_option))]
    pub task: Option<Task>,
    #[builder(default, setter(strip_option))]
    pub comment: Option<Comment>,
    #[builder(default, setter(strip_option))]
    pub continuation: Option<Continuation>,
    #[builder(default, setter(strip_option))]
    pub reuse: Option<Reuse>,
    #[builder(default, setter(strip_option))]
    pub backup_interval: Option<BackupInterval>,
    #[builder(default, setter(strip_option))]
    pub calculate_densdiff: Option<CalculateDensdiff>,
    #[builder(default, setter(strip_option))]
    pub calculate_elf: Option<CalculateElf>,
    #[builder(default, setter(strip_option))]
    pub calculate_hirshfeld: Option<CalculateHirshfeld>,
    #[builder(default, setter(strip_option))]
    pub calculate_stress: Option<CalculateStress>,
    #[builder(default, setter(strip_option))]
    pub charge_unit: Option<ChargeUnit>,
    #[builder(default, setter(strip_option))]
    pub checkpoint: Option<Checkpoint>,
    #[builder(default, setter(strip_option))]
    pub data_distribution: Option<DataDistribution>,
    #[builder(default, setter(strip_option))]
    pub iprint: Option<Iprint>,
    #[builder(default, setter(strip_option))]
    pub num_backup_iter: Option<NumBackupIter>,
    #[builder(default, setter(strip_option))]
    pub opt_strategy: Option<OptStrategy>,
    #[builder(default, setter(strip_option))]
    pub page_wvfns: Option<PageWvfns>,
    #[builder(default, setter(strip_option))]
    pub print_clock: Option<PrintClock>,
    #[builder(default, setter(strip_option))]
    pub print_memory_usage: Option<PrintMemoryUsage>,
    #[builder(default, setter(strip_option))]
    pub rand_seed: Option<RandSeed>,
    #[builder(default, setter(strip_option))]
    pub run_time: Option<RunTime>,
    #[builder(default, setter(strip_option))]
    pub stop: Option<Stop>,
    #[builder(default, setter(strip_option))]
    pub write_checkpoint: Option<WriteCheckpoint>,
    #[builder(default, setter(strip_option))]
    pub write_formatted_density: Option<WriteFormattedDensity>,
    #[builder(default, setter(strip_option))]
    pub write_formatted_elf: Option<WriteFormattedElf>,
    #[builder(default, setter(strip_option))]
    pub write_formatted_potential: Option<WriteFormattedPotential>,
    #[builder(default, setter(strip_option))]
    pub write_orbitals: Option<WriteOrbitals>,

    // === Electronic (19 fields) ===
    #[builder(default, setter(strip_option))]
    pub charge: Option<Charge>,
    #[builder(default, setter(strip_option))]
    pub nbands: Option<Nbands>,
    #[builder(default, setter(strip_option))]
    pub ndown: Option<Ndown>,
    #[builder(default, setter(strip_option))]
    pub nelectrons: Option<Nelectrons>,
    #[builder(default, setter(strip_option))]
    pub nextra_bands: Option<NextraBands>,
    #[builder(default, setter(strip_option))]
    pub nup: Option<Nup>,
    #[builder(default, setter(strip_option))]
    pub perc_extra_bands: Option<PercExtraBands>,
    #[builder(default, setter(strip_option))]
    pub sedc_apply: Option<SedcApply>,
    #[builder(default, setter(strip_option))]
    pub sedc_d_g06: Option<SedcDG06>,
    #[builder(default, setter(strip_option))]
    pub sedc_d_jchs: Option<SedcDJchs>,
    #[builder(default, setter(strip_option))]
    pub sedc_d_ts: Option<SedcDTs>,
    #[builder(default, setter(strip_option))]
    pub sedc_lambda_obs: Option<SedcLambdaObs>,
    #[builder(default, setter(strip_option))]
    pub sedc_n_obs: Option<SedcNObs>,
    #[builder(default, setter(strip_option))]
    pub sedc_s6_g06: Option<SedcS6G06>,
    #[builder(default, setter(strip_option))]
    pub sedc_s6_jchs: Option<SedcS6Jchs>,
    #[builder(default, setter(strip_option))]
    pub sedc_scheme: Option<SedcScheme>,
    #[builder(default, setter(strip_option))]
    pub sedc_sr_jchs: Option<SedcSrJchs>,
    #[builder(default, setter(strip_option))]
    pub sedc_sr_ts: Option<SedcSrTs>,
    #[builder(default, setter(strip_option))]
    pub spin: Option<Spin>,

    // === Basis Set (9 fields) ===
    #[builder(default, setter(strip_option))]
    pub basis_de_dloge: Option<BasisDeDloge>,
    #[builder(default, setter(strip_option))]
    pub basis_precision: Option<BasisPrecision>,
    #[builder(default, setter(strip_option))]
    pub cutoff_energy: Option<CutOffEnergy>,
    #[builder(default, setter(strip_option))]
    pub fine_gmax: Option<FineGmax>,
    #[builder(default, setter(strip_option))]
    pub fine_grid_scale: Option<FineGridScale>,
    #[builder(default, setter(strip_option))]
    pub finite_basis_corr: Option<FiniteBasisCorr>,
    #[builder(default, setter(strip_option))]
    pub finite_basis_npoints: Option<FiniteBasisNpoints>,
    #[builder(default, setter(strip_option))]
    pub finite_basis_spacing: Option<FiniteBasisSpacing>,
    #[builder(default, setter(strip_option))]
    pub fixed_npw: Option<FixedNpw>,
    #[builder(default, setter(strip_option))]
    pub grid_scale: Option<GridScale>,

    // === Exchange-Correlation (13 fields) ===
    #[builder(default, setter(strip_option))]
    pub k_scrn_averaging_scheme: Option<KScrnAveragingScheme>,
    #[builder(default, setter(strip_option))]
    pub spin_polarized: Option<SpinPolarized>,
    #[builder(default, setter(strip_option))]
    pub xc_functional: Option<XcFunctional>,
    #[builder(default, setter(strip_option))]
    pub nlxc_exchange_reflect_kpts: Option<NlxcExchangeReflectKpts>,
    #[builder(default, setter(strip_option))]
    pub nlxc_impose_trs: Option<NlxcImposeTrs>,
    #[builder(default, setter(strip_option))]
    pub nlxc_ppd_integral: Option<NlxcPpdIntegral>,
    #[builder(default, setter(strip_option))]
    pub nlxc_re_est_k_scrn: Option<NlxcReEstKScrn>,
    #[builder(default, setter(strip_option))]
    pub nlxc_page_ex_pot: Option<NlxcPageExPot>,
    #[builder(default, setter(strip_option))]
    pub nlxc_ppd_size_x: Option<NlxcPpdSizeX>,
    #[builder(default, setter(strip_option))]
    pub nlxc_ppd_size_y: Option<NlxcPpdSizeY>,
    #[builder(default, setter(strip_option))]
    pub nlxc_ppd_size_z: Option<NlxcPpdSizeZ>,
    #[builder(default, setter(strip_option))]
    pub xc_definition: Option<XcDefinition>,

    // === Electronic Minimisation (13 fields) ===
    #[builder(default, setter(strip_option))]
    pub efermi_tol: Option<EFermiTol>,
    #[builder(default, setter(strip_option))]
    pub elec_convergence_win: Option<ElecConvergenceWin>,
    #[builder(default, setter(strip_option))]
    pub elec_dump_file: Option<ElecDumpFile>,
    #[builder(default, setter(strip_option))]
    pub elec_eigenvalue_tol: Option<ElecEigenvalueTol>,
    #[builder(default, setter(strip_option))]
    pub elec_energy_tol: Option<ElecEnergyTol>,
    #[builder(default, setter(strip_option))]
    pub elec_restore_file: Option<ElecRestoreFile>,
    #[builder(default, setter(strip_option))]
    pub electronic_minimizer: Option<ElectronicMinimizer>,
    #[builder(default, setter(strip_option))]
    pub fix_occupancy: Option<FixOccupancy>,
    #[builder(default, setter(strip_option))]
    pub max_cg_steps: Option<MaxCgSteps>,
    #[builder(default, setter(strip_option))]
    pub max_scf_cycles: Option<MaxScfCycles>,
    #[builder(default, setter(strip_option))]
    pub max_sd_steps: Option<MaxSdSteps>,
    #[builder(default, setter(strip_option))]
    pub metals_method: Option<MetalsMethod>,
    #[builder(default, setter(strip_option))]
    pub num_dump_cycles: Option<NumDumpCycles>,
    #[builder(default, setter(strip_option))]
    pub smearing_scheme: Option<SmearingScheme>,
    #[builder(default, setter(strip_option))]
    pub smearing_width: Option<SmearingWidth>,
    #[builder(default, setter(strip_option))]
    pub spin_fix: Option<SpinFix>,

    // === Geometry Optimization (11 fields) ===
    #[builder(default, setter(strip_option))]
    pub geom_convergence_win: Option<GeomConvergenceWin>,
    #[builder(default, setter(strip_option))]
    pub geom_disp_tol: Option<GeomDispTol>,
    #[builder(default, setter(strip_option))]
    pub geom_energy_tol: Option<GeomEnergyTol>,
    #[builder(default, setter(strip_option))]
    pub geom_force_tol: Option<GeomForceTol>,
    #[builder(default, setter(strip_option))]
    pub geom_frequency_est: Option<GeomFrequencyEst>,
    #[builder(default, setter(strip_option))]
    pub geom_max_iter: Option<GeomMaxIter>,
    #[builder(default, setter(strip_option))]
    pub geom_method: Option<GeomMethod>,
    #[builder(default, setter(strip_option))]
    pub geom_modulus_est: Option<GeomModulusEst>,
    #[builder(default, setter(strip_option))]
    pub geom_preconditioner: Option<GeomPreconditioner>,
    #[builder(default, setter(strip_option))]
    pub geom_spin_fix: Option<GeomSpinFix>,
    #[builder(default, setter(strip_option))]
    pub geom_stress_tol: Option<GeomStressTol>,

    // === Phonon (11 fields) ===
    #[builder(default, setter(strip_option))]
    pub born_charge_sum_rule: Option<BornChargeSumRule>,
    #[builder(default, setter(strip_option))]
    pub calculate_born_charges: Option<CalculateBornCharges>,
    #[builder(default, setter(strip_option))]
    pub phonon_calc_lo_to_splitting: Option<PhononCalcLoToSplitting>,
    #[builder(default, setter(strip_option))]
    pub phonon_convergence_win: Option<PhononConvergenceWin>,
    #[builder(default, setter(strip_option))]
    pub phonon_energy_tol: Option<PhononEnergyTol>,
    #[builder(default, setter(strip_option))]
    pub phonon_fine_method: Option<PhononFineMethod>,
    #[builder(default, setter(strip_option))]
    pub phonon_finite_disp: Option<PhononFiniteDisp>,
    #[builder(default, setter(strip_option))]
    pub phonon_force_constant_cutoff: Option<PhononForceConstantCutoff>,
    #[builder(default, setter(strip_option))]
    pub phonon_max_cg_steps: Option<PhononMaxCgSteps>,
    #[builder(default, setter(strip_option))]
    pub phonon_max_cycles: Option<PhononMaxCycles>,
    #[builder(default, setter(strip_option))]
    pub phonon_method: Option<PhononMethod>,
    #[builder(default, setter(strip_option))]
    pub phonon_sum_rule: Option<PhononSumRule>,

    // === Band Structure (8 fields) ===
    #[builder(default, setter(strip_option))]
    pub bs_eigenvalue_tol: Option<BsEigenvalueTol>,
    #[builder(default, setter(strip_option))]
    pub bs_max_cg_steps: Option<BsMaxCgSteps>,
    #[builder(default, setter(strip_option))]
    pub bs_max_iter: Option<BsMaxIter>,
    #[builder(default, setter(strip_option))]
    pub bs_nbands: Option<BsNbands>,
    #[builder(default, setter(strip_option))]
    pub bs_nextra_bands: Option<BsNextraBands>,
    #[builder(default, setter(strip_option))]
    pub bs_perc_extra_bands: Option<BsPercExtraBands>,
    #[builder(default, setter(strip_option))]
    pub bs_re_est_k_scrn: Option<BsReEstKScrn>,
    #[builder(default, setter(strip_option))]
    pub bs_xc_functional: Option<BsXcFunctional>,

    // === Molecular Dynamics (22 fields) ===
    #[builder(default, setter(strip_option))]
    pub md_barostat: Option<MdBarostat>,
    #[builder(default, setter(strip_option))]
    pub md_cell_t: Option<MdCellT>,
    #[builder(default, setter(strip_option))]
    pub md_damping_reset: Option<MdDampingReset>,
    #[builder(default, setter(strip_option))]
    pub md_damping_scheme: Option<MdDampingScheme>,
    #[builder(default, setter(strip_option))]
    pub md_delta_t: Option<MdDeltaT>,
    #[builder(default, setter(strip_option))]
    pub md_elec_convergence_win: Option<MdElecConvergenceWin>,
    #[builder(default, setter(strip_option))]
    pub md_elec_eigenvalue_tol: Option<MdElecEigenvalueTol>,
    #[builder(default, setter(strip_option))]
    pub md_elec_energy_tol: Option<MdElecEnergyTol>,
    #[builder(default, setter(strip_option))]
    pub md_ensemble: Option<MdEnsemble>,
    #[builder(default, setter(strip_option))]
    pub md_eqm_cell_t: Option<MdEqmCellT>,
    #[builder(default, setter(strip_option))]
    pub md_eqm_ion_t: Option<MdEqmIonT>,
    #[builder(default, setter(strip_option))]
    pub md_eqm_method: Option<MdEqmMethod>,
    #[builder(default, setter(strip_option))]
    pub md_eqm_t: Option<MdEqmT>,
    #[builder(default, setter(strip_option))]
    pub md_extrap: Option<MdExtrap>,
    #[builder(default, setter(strip_option))]
    pub md_extrap_fit: Option<MdExtrapFit>,
    #[builder(default, setter(strip_option))]
    pub md_ion_t: Option<MdIonT>,
    #[builder(default, setter(strip_option))]
    pub md_nhc_length: Option<MdNhcLength>,
    #[builder(default, setter(strip_option))]
    pub md_num_iter: Option<MdNumIter>,
    #[builder(default, setter(strip_option))]
    pub md_opt_damped_delta_t: Option<MdOptDampedDeltaT>,
    #[builder(default, setter(strip_option))]
    pub md_temperature: Option<MdTemperature>,
    #[builder(default, setter(strip_option))]
    pub md_thermostat: Option<MdThermostat>,

    // === Electric Field (7 fields) ===
    #[builder(default, setter(strip_option))]
    pub efield_calc_ion_permittivity: Option<EfieldCalcIonPermittivity>,
    #[builder(default, setter(strip_option))]
    pub efield_calculate_nonlinear: Option<EfieldCalculateNonlinear>,
    #[builder(default, setter(strip_option))]
    pub efield_convergence_win: Option<EfieldConvergenceWin>,
    #[builder(default, setter(strip_option))]
    pub efield_energy_tol: Option<EfieldEnergyTol>,
    #[builder(default, setter(strip_option))]
    pub efield_ignore_mol_modes: Option<EfieldIgnoreMolModes>,
    #[builder(default, setter(strip_option))]
    pub efield_max_cg_steps: Option<EfieldMaxCgSteps>,
    #[builder(default, setter(strip_option))]
    pub efield_max_cycles: Option<EfieldMaxCycles>,

    // === Pseudopotential (3 fields) ===
    #[builder(default, setter(strip_option))]
    pub pspot_beta_phi_type: Option<PspotBetaPhiType>,
    #[builder(default, setter(strip_option))]
    pub pspot_nonlocal_type: Option<PspotNonlocalType>,
    #[builder(default, setter(strip_option))]
    pub relativistic_treatment: Option<RelativisticTreatment>,

    // === Density Mixing (8 fields) ===
    #[builder(default, setter(strip_option))]
    pub mix_charge_amp: Option<MixChargeAmp>,
    #[builder(default, setter(strip_option))]
    pub mix_charge_gmax: Option<MixChargeGmax>,
    #[builder(default, setter(strip_option))]
    pub mix_cut_off_energy: Option<MixCutOffEnergy>,
    #[builder(default, setter(strip_option))]
    pub mix_history_length: Option<MixHistoryLength>,
    #[builder(default, setter(strip_option))]
    pub mix_metric_q: Option<MixMetricQ>,
    #[builder(default, setter(strip_option))]
    pub mix_spin_amp: Option<MixSpinAmp>,
    #[builder(default, setter(strip_option))]
    pub mix_spin_gmax: Option<MixSpinGmax>,
    #[builder(default, setter(strip_option))]
    pub mixing_scheme: Option<MixingScheme>,

    // === Population Analysis (4 fields) ===
    #[builder(default, setter(strip_option))]
    pub pdos_calculate_weights: Option<PdosCalculateWeights>,
    #[builder(default, setter(strip_option))]
    pub popn_bond_cutoff: Option<PopnBondCutoff>,
    #[builder(default, setter(strip_option))]
    pub popn_calculate: Option<PopnCalculate>,
    #[builder(default, setter(strip_option))]
    pub popn_write: Option<PopnWrite>,

    // === Optics (4 fields) ===
    #[builder(default, setter(strip_option))]
    pub optics_xc_functional: Option<OpticXcFunctional>,
    #[builder(default, setter(strip_option))]
    pub optics_nbands: Option<OpticsNbands>,
    #[builder(default, setter(strip_option))]
    pub optics_nextra_bands: Option<OpticsNextraBands>,
    #[builder(default, setter(strip_option))]
    pub optics_perc_extra_bands: Option<OpticsPercExtraBands>,

    // === NMR (4 fields) ===
    #[builder(default, setter(strip_option))]
    pub magres_conv_tol: Option<MagresConvTol>,
    #[builder(default, setter(strip_option))]
    pub magres_max_cg_steps: Option<MagresMaxCgSteps>,
    #[builder(default, setter(strip_option))]
    pub magres_method: Option<MagresMethod>,
    #[builder(default, setter(strip_option))]
    pub magres_task: Option<MagresTask>,

    // === Solvation (7 fields) ===
    #[builder(default, setter(strip_option))]
    pub boundary_type: Option<BoundaryType>,
    #[builder(default, setter(strip_option))]
    pub dielec_emb_func_method: Option<DielecEmbFuncMethod>,
    #[builder(default, setter(strip_option))]
    pub dielec_emb_bulk_permittivity: Option<DielecEmbBulkPermittivity>,
    #[builder(default, setter(strip_option))]
    pub implicit_solvent_apolar_factor: Option<ImplicitSolventApolarFactor>,
    #[builder(default, setter(strip_option))]
    pub implicit_solvent_apolar_term: Option<ImplicitSolventApolarTerm>,
    #[builder(default, setter(strip_option))]
    pub implicit_solvent_surface_tension: Option<ImplicitSolventSurfaceTension>,
    #[builder(default, setter(strip_option))]
    pub use_smeared_ions: Option<UseSmearediIons>,

    // === Electronic Excitations (4 fields) ===
    #[builder(default, setter(strip_option))]
    pub spectral_task: Option<SpectralTask>,
    #[builder(default, setter(strip_option))]
    pub tddft_position_method: Option<TddftPositionMethod>,
    #[builder(default, setter(strip_option))]
    pub tddft_num_states: Option<TddftNumStates>,
    #[builder(default, setter(strip_option))]
    pub tddft_selected_state: Option<TddftSelectedState>,

    // === Transition State (8 fields) ===
    #[builder(default, setter(strip_option))]
    pub tssearch_method: Option<TssearchMethod>,
    #[builder(default, setter(strip_option))]
    pub tssearch_lstqst_protocol: Option<TssearchLstqstProtocol>,
    #[builder(default, setter(strip_option))]
    pub tssearch_cg_max_iter: Option<TssearchCgMaxIter>,
    #[builder(default, setter(strip_option))]
    pub tssearch_max_path_points: Option<TssearchMaxPathPoints>,
    #[builder(default, setter(strip_option))]
    pub tssearch_qst_max_iter: Option<TssearchQstMaxIter>,
    #[builder(default, setter(strip_option))]
    pub tssearch_disp_tol: Option<TssearchDispTol>,
    #[builder(default, setter(strip_option))]
    pub tssearch_energy_tol: Option<TssearchEnergyTol>,
    #[builder(default, setter(strip_option))]
    pub tssearch_force_tol: Option<TssearchForceTol>,
}

impl ParamDocumentBuilder {
    fn validate(&self) -> Result<(), String> {
        // Band structure mutual exclusivity
        let bs_count = [
            self.bs_nbands.flatten().is_some(),
            self.bs_nextra_bands.flatten().is_some(),
            self.bs_perc_extra_bands.flatten().is_some(),
        ].iter().filter(|&&x| x).count();
        if bs_count > 1 {
            return Err("BS_NBANDS, BS_NEXTRA_BANDS, and BS_PERC_EXTRA_BANDS are mutually exclusive. Only one may be specified.".into());
        }

        // Electronic mutual exclusivity
        let elec_count = [
            self.nbands.flatten().is_some(),
            self.nextra_bands.flatten().is_some(),
            self.perc_extra_bands.flatten().is_some(),
        ].iter().filter(|&&x| x).count();
        if elec_count > 1 {
            return Err("NBANDS, NEXTRA_BANDS, and PERC_EXTRA_BANDS are mutually exclusive. Only one may be specified.".into());
        }

        // Optics mutual exclusivity
        let optics_count = [
            self.optics_nbands.flatten().is_some(),
            self.optics_nextra_bands.flatten().is_some(),
            self.optics_perc_extra_bands.flatten().is_some(),
        ].iter().filter(|&&x| x).count();
        if optics_count > 1 {
            return Err("OPTICS_NBANDS, OPTICS_NEXTRA_BANDS, and OPTICS_PERC_EXTRA_BANDS are mutually exclusive. Only one may be specified.".into());
        }

        Ok(())
    }
}

impl FromCellFile for ParamDocument {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        let mut builder = ParamDocumentBuilder::default();

        if let Ok(Some(v)) = Task::from_cells(tokens) { builder.task(v); }
        if let Ok(Some(v)) = Comment::from_cells(tokens) { builder.comment(v); }
        if let Ok(Some(v)) = Continuation::from_cells(tokens) { builder.continuation(v); }
        if let Ok(Some(v)) = Reuse::from_cells(tokens) { builder.reuse(v); }
        if let Ok(Some(v)) = BackupInterval::from_cells(tokens) { builder.backup_interval(v); }
        if let Ok(Some(v)) = CalculateDensdiff::from_cells(tokens) { builder.calculate_densdiff(v); }
        if let Ok(Some(v)) = CalculateElf::from_cells(tokens) { builder.calculate_elf(v); }
        if let Ok(Some(v)) = CalculateHirshfeld::from_cells(tokens) { builder.calculate_hirshfeld(v); }
        if let Ok(Some(v)) = CalculateStress::from_cells(tokens) { builder.calculate_stress(v); }
        if let Ok(Some(v)) = ChargeUnit::from_cells(tokens) { builder.charge_unit(v); }
        if let Ok(Some(v)) = Checkpoint::from_cells(tokens) { builder.checkpoint(v); }
        if let Ok(Some(v)) = DataDistribution::from_cells(tokens) { builder.data_distribution(v); }
        if let Ok(Some(v)) = Iprint::from_cells(tokens) { builder.iprint(v); }
        if let Ok(Some(v)) = NumBackupIter::from_cells(tokens) { builder.num_backup_iter(v); }
        if let Ok(Some(v)) = OptStrategy::from_cells(tokens) { builder.opt_strategy(v); }
        if let Ok(Some(v)) = PageWvfns::from_cells(tokens) { builder.page_wvfns(v); }
        if let Ok(Some(v)) = PrintClock::from_cells(tokens) { builder.print_clock(v); }
        if let Ok(Some(v)) = PrintMemoryUsage::from_cells(tokens) { builder.print_memory_usage(v); }
        if let Ok(Some(v)) = RandSeed::from_cells(tokens) { builder.rand_seed(v); }
        if let Ok(Some(v)) = RunTime::from_cells(tokens) { builder.run_time(v); }
        if let Ok(Some(v)) = Stop::from_cells(tokens) { builder.stop(v); }
        if let Ok(Some(v)) = WriteCheckpoint::from_cells(tokens) { builder.write_checkpoint(v); }
        if let Ok(Some(v)) = WriteFormattedDensity::from_cells(tokens) { builder.write_formatted_density(v); }
        if let Ok(Some(v)) = WriteFormattedElf::from_cells(tokens) { builder.write_formatted_elf(v); }
        if let Ok(Some(v)) = WriteFormattedPotential::from_cells(tokens) { builder.write_formatted_potential(v); }
        if let Ok(Some(v)) = WriteOrbitals::from_cells(tokens) { builder.write_orbitals(v); }

        if let Ok(Some(v)) = Charge::from_cells(tokens) { builder.charge(v); }
        if let Ok(Some(v)) = Nbands::from_cells(tokens) { builder.nbands(v); }
        if let Ok(Some(v)) = Ndown::from_cells(tokens) { builder.ndown(v); }
        if let Ok(Some(v)) = Nelectrons::from_cells(tokens) { builder.nelectrons(v); }
        if let Ok(Some(v)) = NextraBands::from_cells(tokens) { builder.nextra_bands(v); }
        if let Ok(Some(v)) = Nup::from_cells(tokens) { builder.nup(v); }
        if let Ok(Some(v)) = PercExtraBands::from_cells(tokens) { builder.perc_extra_bands(v); }
        if let Ok(Some(v)) = SedcApply::from_cells(tokens) { builder.sedc_apply(v); }
        if let Ok(Some(v)) = SedcDG06::from_cells(tokens) { builder.sedc_d_g06(v); }
        if let Ok(Some(v)) = SedcDJchs::from_cells(tokens) { builder.sedc_d_jchs(v); }
        if let Ok(Some(v)) = SedcDTs::from_cells(tokens) { builder.sedc_d_ts(v); }
        if let Ok(Some(v)) = SedcLambdaObs::from_cells(tokens) { builder.sedc_lambda_obs(v); }
        if let Ok(Some(v)) = SedcNObs::from_cells(tokens) { builder.sedc_n_obs(v); }
        if let Ok(Some(v)) = SedcS6G06::from_cells(tokens) { builder.sedc_s6_g06(v); }
        if let Ok(Some(v)) = SedcS6Jchs::from_cells(tokens) { builder.sedc_s6_jchs(v); }
        if let Ok(Some(v)) = SedcScheme::from_cells(tokens) { builder.sedc_scheme(v); }
        if let Ok(Some(v)) = SedcSrJchs::from_cells(tokens) { builder.sedc_sr_jchs(v); }
        if let Ok(Some(v)) = SedcSrTs::from_cells(tokens) { builder.sedc_sr_ts(v); }
        if let Ok(Some(v)) = Spin::from_cells(tokens) { builder.spin(v); }

        if let Ok(Some(v)) = BasisDeDloge::from_cells(tokens) { builder.basis_de_dloge(v); }
        if let Ok(Some(v)) = BasisPrecision::from_cells(tokens) { builder.basis_precision(v); }
        if let Ok(Some(v)) = CutOffEnergy::from_cells(tokens) { builder.cutoff_energy(v); }
        if let Ok(Some(v)) = FineGmax::from_cells(tokens) { builder.fine_gmax(v); }
        if let Ok(Some(v)) = FineGridScale::from_cells(tokens) { builder.fine_grid_scale(v); }
        if let Ok(Some(v)) = FiniteBasisCorr::from_cells(tokens) { builder.finite_basis_corr(v); }
        if let Ok(Some(v)) = FiniteBasisNpoints::from_cells(tokens) { builder.finite_basis_npoints(v); }
        if let Ok(Some(v)) = FiniteBasisSpacing::from_cells(tokens) { builder.finite_basis_spacing(v); }
        if let Ok(Some(v)) = FixedNpw::from_cells(tokens) { builder.fixed_npw(v); }
        if let Ok(Some(v)) = GridScale::from_cells(tokens) { builder.grid_scale(v); }

        if let Ok(Some(v)) = KScrnAveragingScheme::from_cells(tokens) { builder.k_scrn_averaging_scheme(v); }
        if let Ok(Some(v)) = SpinPolarized::from_cells(tokens) { builder.spin_polarized(v); }
        if let Ok(Some(v)) = XcFunctional::from_cells(tokens) { builder.xc_functional(v); }
        if let Ok(Some(v)) = NlxcExchangeReflectKpts::from_cells(tokens) { builder.nlxc_exchange_reflect_kpts(v); }
        if let Ok(Some(v)) = NlxcImposeTrs::from_cells(tokens) { builder.nlxc_impose_trs(v); }
        if let Ok(Some(v)) = NlxcPpdIntegral::from_cells(tokens) { builder.nlxc_ppd_integral(v); }
        if let Ok(Some(v)) = NlxcReEstKScrn::from_cells(tokens) { builder.nlxc_re_est_k_scrn(v); }
        if let Ok(Some(v)) = NlxcPageExPot::from_cells(tokens) { builder.nlxc_page_ex_pot(v); }
        if let Ok(Some(v)) = NlxcPpdSizeX::from_cells(tokens) { builder.nlxc_ppd_size_x(v); }
        if let Ok(Some(v)) = NlxcPpdSizeY::from_cells(tokens) { builder.nlxc_ppd_size_y(v); }
        if let Ok(Some(v)) = NlxcPpdSizeZ::from_cells(tokens) { builder.nlxc_ppd_size_z(v); }
        if let Ok(v) = XcDefinition::from_cells(tokens) { builder.xc_definition(v); }

        if let Ok(Some(v)) = EFermiTol::from_cells(tokens) { builder.efermi_tol(v); }
        if let Ok(Some(v)) = ElecConvergenceWin::from_cells(tokens) { builder.elec_convergence_win(v); }
        if let Ok(Some(v)) = ElecDumpFile::from_cells(tokens) { builder.elec_dump_file(v); }
        if let Ok(Some(v)) = ElecEigenvalueTol::from_cells(tokens) { builder.elec_eigenvalue_tol(v); }
        if let Ok(Some(v)) = ElecEnergyTol::from_cells(tokens) { builder.elec_energy_tol(v); }
        if let Ok(Some(v)) = ElecRestoreFile::from_cells(tokens) { builder.elec_restore_file(v); }
        if let Ok(Some(v)) = ElectronicMinimizer::from_cells(tokens) { builder.electronic_minimizer(v); }
        if let Ok(Some(v)) = FixOccupancy::from_cells(tokens) { builder.fix_occupancy(v); }
        if let Ok(Some(v)) = MaxCgSteps::from_cells(tokens) { builder.max_cg_steps(v); }
        if let Ok(Some(v)) = MaxScfCycles::from_cells(tokens) { builder.max_scf_cycles(v); }
        if let Ok(Some(v)) = MaxSdSteps::from_cells(tokens) { builder.max_sd_steps(v); }
        if let Ok(Some(v)) = MetalsMethod::from_cells(tokens) { builder.metals_method(v); }
        if let Ok(Some(v)) = NumDumpCycles::from_cells(tokens) { builder.num_dump_cycles(v); }
        if let Ok(Some(v)) = SmearingScheme::from_cells(tokens) { builder.smearing_scheme(v); }
        if let Ok(Some(v)) = SmearingWidth::from_cells(tokens) { builder.smearing_width(v); }
        if let Ok(Some(v)) = SpinFix::from_cells(tokens) { builder.spin_fix(v); }

        if let Ok(Some(v)) = GeomConvergenceWin::from_cells(tokens) { builder.geom_convergence_win(v); }
        if let Ok(Some(v)) = GeomDispTol::from_cells(tokens) { builder.geom_disp_tol(v); }
        if let Ok(Some(v)) = GeomEnergyTol::from_cells(tokens) { builder.geom_energy_tol(v); }
        if let Ok(Some(v)) = GeomForceTol::from_cells(tokens) { builder.geom_force_tol(v); }
        if let Ok(Some(v)) = GeomFrequencyEst::from_cells(tokens) { builder.geom_frequency_est(v); }
        if let Ok(Some(v)) = GeomMaxIter::from_cells(tokens) { builder.geom_max_iter(v); }
        if let Ok(Some(v)) = GeomMethod::from_cells(tokens) { builder.geom_method(v); }
        if let Ok(Some(v)) = GeomModulusEst::from_cells(tokens) { builder.geom_modulus_est(v); }
        if let Ok(Some(v)) = GeomPreconditioner::from_cells(tokens) { builder.geom_preconditioner(v); }
        if let Ok(Some(v)) = GeomSpinFix::from_cells(tokens) { builder.geom_spin_fix(v); }
        if let Ok(Some(v)) = GeomStressTol::from_cells(tokens) { builder.geom_stress_tol(v); }

        if let Ok(Some(v)) = BornChargeSumRule::from_cells(tokens) { builder.born_charge_sum_rule(v); }
        if let Ok(Some(v)) = CalculateBornCharges::from_cells(tokens) { builder.calculate_born_charges(v); }
        if let Ok(Some(v)) = PhononCalcLoToSplitting::from_cells(tokens) { builder.phonon_calc_lo_to_splitting(v); }
        if let Ok(Some(v)) = PhononConvergenceWin::from_cells(tokens) { builder.phonon_convergence_win(v); }
        if let Ok(Some(v)) = PhononEnergyTol::from_cells(tokens) { builder.phonon_energy_tol(v); }
        if let Ok(Some(v)) = PhononFineMethod::from_cells(tokens) { builder.phonon_fine_method(v); }
        if let Ok(Some(v)) = PhononFiniteDisp::from_cells(tokens) { builder.phonon_finite_disp(v); }
        if let Ok(Some(v)) = PhononForceConstantCutoff::from_cells(tokens) { builder.phonon_force_constant_cutoff(v); }
        if let Ok(Some(v)) = PhononMaxCgSteps::from_cells(tokens) { builder.phonon_max_cg_steps(v); }
        if let Ok(Some(v)) = PhononMaxCycles::from_cells(tokens) { builder.phonon_max_cycles(v); }
        if let Ok(Some(v)) = PhononMethod::from_cells(tokens) { builder.phonon_method(v); }
        if let Ok(Some(v)) = PhononSumRule::from_cells(tokens) { builder.phonon_sum_rule(v); }

        if let Ok(Some(v)) = BsEigenvalueTol::from_cells(tokens) { builder.bs_eigenvalue_tol(v); }
        if let Ok(Some(v)) = BsMaxCgSteps::from_cells(tokens) { builder.bs_max_cg_steps(v); }
        if let Ok(Some(v)) = BsMaxIter::from_cells(tokens) { builder.bs_max_iter(v); }
        if let Ok(Some(v)) = BsNbands::from_cells(tokens) { builder.bs_nbands(v); }
        if let Ok(Some(v)) = BsNextraBands::from_cells(tokens) { builder.bs_nextra_bands(v); }
        if let Ok(Some(v)) = BsPercExtraBands::from_cells(tokens) { builder.bs_perc_extra_bands(v); }
        if let Ok(Some(v)) = BsReEstKScrn::from_cells(tokens) { builder.bs_re_est_k_scrn(v); }
        if let Ok(Some(v)) = BsXcFunctional::from_cells(tokens) { builder.bs_xc_functional(v); }

        if let Ok(Some(v)) = MdBarostat::from_cells(tokens) { builder.md_barostat(v); }
        if let Ok(Some(v)) = MdCellT::from_cells(tokens) { builder.md_cell_t(v); }
        if let Ok(Some(v)) = MdDampingReset::from_cells(tokens) { builder.md_damping_reset(v); }
        if let Ok(Some(v)) = MdDampingScheme::from_cells(tokens) { builder.md_damping_scheme(v); }
        if let Ok(Some(v)) = MdDeltaT::from_cells(tokens) { builder.md_delta_t(v); }
        if let Ok(Some(v)) = MdElecConvergenceWin::from_cells(tokens) { builder.md_elec_convergence_win(v); }
        if let Ok(Some(v)) = MdElecEigenvalueTol::from_cells(tokens) { builder.md_elec_eigenvalue_tol(v); }
        if let Ok(Some(v)) = MdElecEnergyTol::from_cells(tokens) { builder.md_elec_energy_tol(v); }
        if let Ok(Some(v)) = MdEnsemble::from_cells(tokens) { builder.md_ensemble(v); }
        if let Ok(Some(v)) = MdEqmCellT::from_cells(tokens) { builder.md_eqm_cell_t(v); }
        if let Ok(Some(v)) = MdEqmIonT::from_cells(tokens) { builder.md_eqm_ion_t(v); }
        if let Ok(Some(v)) = MdEqmMethod::from_cells(tokens) { builder.md_eqm_method(v); }
        if let Ok(Some(v)) = MdEqmT::from_cells(tokens) { builder.md_eqm_t(v); }
        if let Ok(Some(v)) = MdExtrap::from_cells(tokens) { builder.md_extrap(v); }
        if let Ok(Some(v)) = MdExtrapFit::from_cells(tokens) { builder.md_extrap_fit(v); }
        if let Ok(Some(v)) = MdIonT::from_cells(tokens) { builder.md_ion_t(v); }
        if let Ok(Some(v)) = MdNhcLength::from_cells(tokens) { builder.md_nhc_length(v); }
        if let Ok(Some(v)) = MdNumIter::from_cells(tokens) { builder.md_num_iter(v); }
        if let Ok(Some(v)) = MdOptDampedDeltaT::from_cells(tokens) { builder.md_opt_damped_delta_t(v); }
        if let Ok(Some(v)) = MdTemperature::from_cells(tokens) { builder.md_temperature(v); }
        if let Ok(Some(v)) = MdThermostat::from_cells(tokens) { builder.md_thermostat(v); }

        if let Ok(Some(v)) = EfieldCalcIonPermittivity::from_cells(tokens) { builder.efield_calc_ion_permittivity(v); }
        if let Ok(Some(v)) = EfieldCalculateNonlinear::from_cells(tokens) { builder.efield_calculate_nonlinear(v); }
        if let Ok(Some(v)) = EfieldConvergenceWin::from_cells(tokens) { builder.efield_convergence_win(v); }
        if let Ok(Some(v)) = EfieldEnergyTol::from_cells(tokens) { builder.efield_energy_tol(v); }
        if let Ok(Some(v)) = EfieldIgnoreMolModes::from_cells(tokens) { builder.efield_ignore_mol_modes(v); }
        if let Ok(Some(v)) = EfieldMaxCgSteps::from_cells(tokens) { builder.efield_max_cg_steps(v); }
        if let Ok(Some(v)) = EfieldMaxCycles::from_cells(tokens) { builder.efield_max_cycles(v); }

        if let Ok(Some(v)) = PspotBetaPhiType::from_cells(tokens) { builder.pspot_beta_phi_type(v); }
        if let Ok(Some(v)) = PspotNonlocalType::from_cells(tokens) { builder.pspot_nonlocal_type(v); }
        if let Ok(Some(v)) = RelativisticTreatment::from_cells(tokens) { builder.relativistic_treatment(v); }

        if let Ok(Some(v)) = MixChargeAmp::from_cells(tokens) { builder.mix_charge_amp(v); }
        if let Ok(Some(v)) = MixChargeGmax::from_cells(tokens) { builder.mix_charge_gmax(v); }
        if let Ok(Some(v)) = MixCutOffEnergy::from_cells(tokens) { builder.mix_cut_off_energy(v); }
        if let Ok(Some(v)) = MixHistoryLength::from_cells(tokens) { builder.mix_history_length(v); }
        if let Ok(Some(v)) = MixMetricQ::from_cells(tokens) { builder.mix_metric_q(v); }
        if let Ok(Some(v)) = MixSpinAmp::from_cells(tokens) { builder.mix_spin_amp(v); }
        if let Ok(Some(v)) = MixSpinGmax::from_cells(tokens) { builder.mix_spin_gmax(v); }
        if let Ok(Some(v)) = MixingScheme::from_cells(tokens) { builder.mixing_scheme(v); }

        if let Ok(Some(v)) = PdosCalculateWeights::from_cells(tokens) { builder.pdos_calculate_weights(v); }
        if let Ok(Some(v)) = PopnBondCutoff::from_cells(tokens) { builder.popn_bond_cutoff(v); }
        if let Ok(Some(v)) = PopnCalculate::from_cells(tokens) { builder.popn_calculate(v); }
        if let Ok(Some(v)) = PopnWrite::from_cells(tokens) { builder.popn_write(v); }

        if let Ok(Some(v)) = OpticXcFunctional::from_cells(tokens) { builder.optics_xc_functional(v); }
        if let Ok(Some(v)) = OpticsNbands::from_cells(tokens) { builder.optics_nbands(v); }
        if let Ok(Some(v)) = OpticsNextraBands::from_cells(tokens) { builder.optics_nextra_bands(v); }
        if let Ok(Some(v)) = OpticsPercExtraBands::from_cells(tokens) { builder.optics_perc_extra_bands(v); }

        if let Ok(Some(v)) = MagresConvTol::from_cells(tokens) { builder.magres_conv_tol(v); }
        if let Ok(Some(v)) = MagresMaxCgSteps::from_cells(tokens) { builder.magres_max_cg_steps(v); }
        if let Ok(Some(v)) = MagresMethod::from_cells(tokens) { builder.magres_method(v); }
        if let Ok(Some(v)) = MagresTask::from_cells(tokens) { builder.magres_task(v); }

        if let Ok(Some(v)) = BoundaryType::from_cells(tokens) { builder.boundary_type(v); }
        if let Ok(Some(v)) = DielecEmbFuncMethod::from_cells(tokens) { builder.dielec_emb_func_method(v); }
        if let Ok(Some(v)) = DielecEmbBulkPermittivity::from_cells(tokens) { builder.dielec_emb_bulk_permittivity(v); }
        if let Ok(Some(v)) = ImplicitSolventApolarFactor::from_cells(tokens) { builder.implicit_solvent_apolar_factor(v); }
        if let Ok(Some(v)) = ImplicitSolventApolarTerm::from_cells(tokens) { builder.implicit_solvent_apolar_term(v); }
        if let Ok(Some(v)) = ImplicitSolventSurfaceTension::from_cells(tokens) { builder.implicit_solvent_surface_tension(v); }
        if let Ok(Some(v)) = UseSmearediIons::from_cells(tokens) { builder.use_smeared_ions(v); }

        if let Ok(Some(v)) = SpectralTask::from_cells(tokens) { builder.spectral_task(v); }
        if let Ok(Some(v)) = TddftPositionMethod::from_cells(tokens) { builder.tddft_position_method(v); }
        if let Ok(Some(v)) = TddftNumStates::from_cells(tokens) { builder.tddft_num_states(v); }
        if let Ok(Some(v)) = TddftSelectedState::from_cells(tokens) { builder.tddft_selected_state(v); }

        if let Ok(Some(v)) = TssearchMethod::from_cells(tokens) { builder.tssearch_method(v); }
        if let Ok(Some(v)) = TssearchLstqstProtocol::from_cells(tokens) { builder.tssearch_lstqst_protocol(v); }
        if let Ok(Some(v)) = TssearchCgMaxIter::from_cells(tokens) { builder.tssearch_cg_max_iter(v); }
        if let Ok(Some(v)) = TssearchMaxPathPoints::from_cells(tokens) { builder.tssearch_max_path_points(v); }
        if let Ok(Some(v)) = TssearchQstMaxIter::from_cells(tokens) { builder.tssearch_qst_max_iter(v); }
        if let Ok(Some(v)) = TssearchDispTol::from_cells(tokens) { builder.tssearch_disp_tol(v); }
        if let Ok(Some(v)) = TssearchEnergyTol::from_cells(tokens) { builder.tssearch_energy_tol(v); }
        if let Ok(Some(v)) = TssearchForceTol::from_cells(tokens) { builder.tssearch_force_tol(v); }

        builder.build().map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for ParamDocument {
    fn to_cell_file(&self) -> Vec<Cell> {
        let mut cells = Vec::new();
        if let Some(v) = &self.task { cells.push(v.to_cell()); }
        if let Some(v) = &self.comment { cells.push(v.to_cell()); }
        if let Some(v) = &self.continuation { cells.push(v.to_cell()); }
        if let Some(v) = &self.reuse { cells.push(v.to_cell()); }
        if let Some(v) = &self.backup_interval { cells.push(v.to_cell()); }
        if let Some(v) = &self.calculate_densdiff { cells.push(v.to_cell()); }
        if let Some(v) = &self.calculate_elf { cells.push(v.to_cell()); }
        if let Some(v) = &self.calculate_hirshfeld { cells.push(v.to_cell()); }
        if let Some(v) = &self.calculate_stress { cells.push(v.to_cell()); }
        if let Some(v) = &self.charge_unit { cells.push(v.to_cell()); }
        if let Some(v) = &self.checkpoint { cells.push(v.to_cell()); }
        if let Some(v) = &self.data_distribution { cells.push(v.to_cell()); }
        if let Some(v) = &self.iprint { cells.push(v.to_cell()); }
        if let Some(v) = &self.num_backup_iter { cells.push(v.to_cell()); }
        if let Some(v) = &self.opt_strategy { cells.push(v.to_cell()); }
        if let Some(v) = &self.page_wvfns { cells.push(v.to_cell()); }
        if let Some(v) = &self.print_clock { cells.push(v.to_cell()); }
        if let Some(v) = &self.print_memory_usage { cells.push(v.to_cell()); }
        if let Some(v) = &self.rand_seed { cells.push(v.to_cell()); }
        if let Some(v) = &self.run_time { cells.push(v.to_cell()); }
        if let Some(v) = &self.stop { cells.push(v.to_cell()); }
        if let Some(v) = &self.write_checkpoint { cells.push(v.to_cell()); }
        if let Some(v) = &self.write_formatted_density { cells.push(v.to_cell()); }
        if let Some(v) = &self.write_formatted_elf { cells.push(v.to_cell()); }
        if let Some(v) = &self.write_formatted_potential { cells.push(v.to_cell()); }
        if let Some(v) = &self.write_orbitals { cells.push(v.to_cell()); }
        if let Some(v) = &self.charge { cells.push(v.to_cell()); }
        if let Some(v) = &self.nbands { cells.push(v.to_cell()); }
        if let Some(v) = &self.ndown { cells.push(v.to_cell()); }
        if let Some(v) = &self.nelectrons { cells.push(v.to_cell()); }
        if let Some(v) = &self.nextra_bands { cells.push(v.to_cell()); }
        if let Some(v) = &self.nup { cells.push(v.to_cell()); }
        if let Some(v) = &self.perc_extra_bands { cells.push(v.to_cell()); }
        if let Some(v) = &self.sedc_apply { cells.push(v.to_cell()); }
        if let Some(v) = &self.sedc_d_g06 { cells.push(v.to_cell()); }
        if let Some(v) = &self.sedc_d_jchs { cells.push(v.to_cell()); }
        if let Some(v) = &self.sedc_d_ts { cells.push(v.to_cell()); }
        if let Some(v) = &self.sedc_lambda_obs { cells.push(v.to_cell()); }
        if let Some(v) = &self.sedc_n_obs { cells.push(v.to_cell()); }
        if let Some(v) = &self.sedc_s6_g06 { cells.push(v.to_cell()); }
        if let Some(v) = &self.sedc_s6_jchs { cells.push(v.to_cell()); }
        if let Some(v) = &self.sedc_scheme { cells.push(v.to_cell()); }
        if let Some(v) = &self.sedc_sr_jchs { cells.push(v.to_cell()); }
        if let Some(v) = &self.sedc_sr_ts { cells.push(v.to_cell()); }
        if let Some(v) = &self.spin { cells.push(v.to_cell()); }
        if let Some(v) = &self.basis_de_dloge { cells.push(v.to_cell()); }
        if let Some(v) = &self.basis_precision { cells.push(v.to_cell()); }
        if let Some(v) = &self.cutoff_energy { cells.push(v.to_cell()); }
        if let Some(v) = &self.fine_gmax { cells.push(v.to_cell()); }
        if let Some(v) = &self.fine_grid_scale { cells.push(v.to_cell()); }
        if let Some(v) = &self.finite_basis_corr { cells.push(v.to_cell()); }
        if let Some(v) = &self.finite_basis_npoints { cells.push(v.to_cell()); }
        if let Some(v) = &self.finite_basis_spacing { cells.push(v.to_cell()); }
        if let Some(v) = &self.fixed_npw { cells.push(v.to_cell()); }
        if let Some(v) = &self.grid_scale { cells.push(v.to_cell()); }
        if let Some(v) = &self.k_scrn_averaging_scheme { cells.push(v.to_cell()); }
        if let Some(v) = &self.spin_polarized { cells.push(v.to_cell()); }
        if let Some(v) = &self.xc_functional { cells.push(v.to_cell()); }
        if let Some(v) = &self.nlxc_exchange_reflect_kpts { cells.push(v.to_cell()); }
        if let Some(v) = &self.nlxc_impose_trs { cells.push(v.to_cell()); }
        if let Some(v) = &self.nlxc_ppd_integral { cells.push(v.to_cell()); }
        if let Some(v) = &self.nlxc_re_est_k_scrn { cells.push(v.to_cell()); }
        if let Some(v) = &self.nlxc_page_ex_pot { cells.push(v.to_cell()); }
        if let Some(v) = &self.nlxc_ppd_size_x { cells.push(v.to_cell()); }
        if let Some(v) = &self.nlxc_ppd_size_y { cells.push(v.to_cell()); }
        if let Some(v) = &self.nlxc_ppd_size_z { cells.push(v.to_cell()); }
        if let Some(v) = &self.xc_definition { cells.push(v.to_cell()); }
        if let Some(v) = &self.efermi_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.elec_convergence_win { cells.push(v.to_cell()); }
        if let Some(v) = &self.elec_dump_file { cells.push(v.to_cell()); }
        if let Some(v) = &self.elec_eigenvalue_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.elec_energy_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.elec_restore_file { cells.push(v.to_cell()); }
        if let Some(v) = &self.electronic_minimizer { cells.push(v.to_cell()); }
        if let Some(v) = &self.fix_occupancy { cells.push(v.to_cell()); }
        if let Some(v) = &self.max_cg_steps { cells.push(v.to_cell()); }
        if let Some(v) = &self.max_scf_cycles { cells.push(v.to_cell()); }
        if let Some(v) = &self.max_sd_steps { cells.push(v.to_cell()); }
        if let Some(v) = &self.metals_method { cells.push(v.to_cell()); }
        if let Some(v) = &self.num_dump_cycles { cells.push(v.to_cell()); }
        if let Some(v) = &self.smearing_scheme { cells.push(v.to_cell()); }
        if let Some(v) = &self.smearing_width { cells.push(v.to_cell()); }
        if let Some(v) = &self.spin_fix { cells.push(v.to_cell()); }
        if let Some(v) = &self.geom_convergence_win { cells.push(v.to_cell()); }
        if let Some(v) = &self.geom_disp_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.geom_energy_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.geom_force_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.geom_frequency_est { cells.push(v.to_cell()); }
        if let Some(v) = &self.geom_max_iter { cells.push(v.to_cell()); }
        if let Some(v) = &self.geom_method { cells.push(v.to_cell()); }
        if let Some(v) = &self.geom_modulus_est { cells.push(v.to_cell()); }
        if let Some(v) = &self.geom_preconditioner { cells.push(v.to_cell()); }
        if let Some(v) = &self.geom_spin_fix { cells.push(v.to_cell()); }
        if let Some(v) = &self.geom_stress_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.born_charge_sum_rule { cells.push(v.to_cell()); }
        if let Some(v) = &self.calculate_born_charges { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_calc_lo_to_splitting { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_convergence_win { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_energy_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_fine_method { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_finite_disp { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_force_constant_cutoff { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_max_cg_steps { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_max_cycles { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_method { cells.push(v.to_cell()); }
        if let Some(v) = &self.phonon_sum_rule { cells.push(v.to_cell()); }
        if let Some(v) = &self.bs_eigenvalue_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.bs_max_cg_steps { cells.push(v.to_cell()); }
        if let Some(v) = &self.bs_max_iter { cells.push(v.to_cell()); }
        if let Some(v) = &self.bs_nbands { cells.push(v.to_cell()); }
        if let Some(v) = &self.bs_nextra_bands { cells.push(v.to_cell()); }
        if let Some(v) = &self.bs_perc_extra_bands { cells.push(v.to_cell()); }
        if let Some(v) = &self.bs_re_est_k_scrn { cells.push(v.to_cell()); }
        if let Some(v) = &self.bs_xc_functional { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_barostat { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_cell_t { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_damping_reset { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_damping_scheme { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_delta_t { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_elec_convergence_win { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_elec_eigenvalue_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_elec_energy_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_ensemble { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_eqm_cell_t { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_eqm_ion_t { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_eqm_method { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_eqm_t { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_extrap { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_extrap_fit { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_ion_t { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_nhc_length { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_num_iter { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_temperature { cells.push(v.to_cell()); }
        if let Some(v) = &self.md_thermostat { cells.push(v.to_cell()); }
        if let Some(v) = &self.efield_calc_ion_permittivity { cells.push(v.to_cell()); }
        if let Some(v) = &self.efield_calculate_nonlinear { cells.push(v.to_cell()); }
        if let Some(v) = &self.efield_convergence_win { cells.push(v.to_cell()); }
        if let Some(v) = &self.efield_energy_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.efield_ignore_mol_modes { cells.push(v.to_cell()); }
        if let Some(v) = &self.efield_max_cg_steps { cells.push(v.to_cell()); }
        if let Some(v) = &self.efield_max_cycles { cells.push(v.to_cell()); }
        if let Some(v) = &self.pspot_beta_phi_type { cells.push(v.to_cell()); }
        if let Some(v) = &self.pspot_nonlocal_type { cells.push(v.to_cell()); }
        if let Some(v) = &self.relativistic_treatment { cells.push(v.to_cell()); }
        if let Some(v) = &self.mix_charge_amp { cells.push(v.to_cell()); }
        if let Some(v) = &self.mix_charge_gmax { cells.push(v.to_cell()); }
        if let Some(v) = &self.mix_cut_off_energy { cells.push(v.to_cell()); }
        if let Some(v) = &self.mix_history_length { cells.push(v.to_cell()); }
        if let Some(v) = &self.mix_metric_q { cells.push(v.to_cell()); }
        if let Some(v) = &self.mix_spin_amp { cells.push(v.to_cell()); }
        if let Some(v) = &self.mix_spin_gmax { cells.push(v.to_cell()); }
        if let Some(v) = &self.mixing_scheme { cells.push(v.to_cell()); }
        if let Some(v) = &self.pdos_calculate_weights { cells.push(v.to_cell()); }
        if let Some(v) = &self.popn_bond_cutoff { cells.push(v.to_cell()); }
        if let Some(v) = &self.popn_calculate { cells.push(v.to_cell()); }
        if let Some(v) = &self.popn_write { cells.push(v.to_cell()); }
        if let Some(v) = &self.optics_xc_functional { cells.push(v.to_cell()); }
        if let Some(v) = &self.optics_nbands { cells.push(v.to_cell()); }
        if let Some(v) = &self.optics_nextra_bands { cells.push(v.to_cell()); }
        if let Some(v) = &self.optics_perc_extra_bands { cells.push(v.to_cell()); }
        if let Some(v) = &self.magres_conv_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.magres_max_cg_steps { cells.push(v.to_cell()); }
        if let Some(v) = &self.magres_method { cells.push(v.to_cell()); }
        if let Some(v) = &self.magres_task { cells.push(v.to_cell()); }
        if let Some(v) = &self.boundary_type { cells.push(v.to_cell()); }
        if let Some(v) = &self.dielec_emb_func_method { cells.push(v.to_cell()); }
        if let Some(v) = &self.dielec_emb_bulk_permittivity { cells.push(v.to_cell()); }
        if let Some(v) = &self.implicit_solvent_apolar_factor { cells.push(v.to_cell()); }
        if let Some(v) = &self.implicit_solvent_apolar_term { cells.push(v.to_cell()); }
        if let Some(v) = &self.implicit_solvent_surface_tension { cells.push(v.to_cell()); }
        if let Some(v) = &self.use_smeared_ions { cells.push(v.to_cell()); }
        if let Some(v) = &self.spectral_task { cells.push(v.to_cell()); }
        if let Some(v) = &self.tddft_position_method { cells.push(v.to_cell()); }
        if let Some(v) = &self.tddft_num_states { cells.push(v.to_cell()); }
        if let Some(v) = &self.tddft_selected_state { cells.push(v.to_cell()); }
        if let Some(v) = &self.tssearch_method { cells.push(v.to_cell()); }
        if let Some(v) = &self.tssearch_lstqst_protocol { cells.push(v.to_cell()); }
        if let Some(v) = &self.tssearch_cg_max_iter { cells.push(v.to_cell()); }
        if let Some(v) = &self.tssearch_max_path_points { cells.push(v.to_cell()); }
        if let Some(v) = &self.tssearch_qst_max_iter { cells.push(v.to_cell()); }
        if let Some(v) = &self.tssearch_disp_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.tssearch_energy_tol { cells.push(v.to_cell()); }
        if let Some(v) = &self.tssearch_force_tol { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_io::parse;

    #[test]
    fn test_parse_co3o4_2_param() {
        let input = std::fs::read_to_string("../Co3O4_2.param").unwrap();
        let doc: ParamDocument = parse(&input).unwrap();

        assert_eq!(doc.task.unwrap(), crate::param::general::Task::GeometryOptimization);
        assert_eq!(doc.xc_functional.unwrap(), crate::param::exchange_correlation::XcFunctional::Pbe);
        assert_eq!(doc.spin_polarized.unwrap().0, false);
        assert_eq!(doc.cutoff_energy.unwrap().value, 900.0);
        assert_eq!(doc.max_scf_cycles.unwrap().0, 400);
        assert_eq!(doc.geom_method.unwrap(), crate::param::geometry_optimization::GeomMethod::Bfgs);
    }
}
