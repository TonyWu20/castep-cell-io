use super::{
    band_structure::{BSExtraBands, BSXcFunctional, BandStructureBuilder},
    BandsOptionBuilder, BasisSetBuilder, CalculatePropertiesBuilder, CastepParam,
    CastepParamBuilder, CastepParamBuilderError, CastepTask, Continuation,
    DensityMixingParamsBuilder, ElectroMinimizationBuilder, ElectronicParamBuilder, ExtraBands,
    FiniteBasisCorr, GeneralBuilder, GeomMethod, GeomOptBuilder, MetalsMethod, MixingScheme,
    OptStrategy, PopulationAnalysisBuilder, WritePropertiesBuilder, XCFunctional, XcParamBuilder,
};

impl CastepParam {
    pub fn param_template(cutoff_energy: f64, spin: f64) -> CastepParamBuilder {
        let mut builder = CastepParamBuilder::default();
        builder
            .basis_set(
                BasisSetBuilder::default()
                    .cut_off_energy(cutoff_energy)
                    .grid_scale(1.5)
                    .fine_grid_scale(1.5)
                    .finite_basis_corr(FiniteBasisCorr::NoCorrection)
                    .build()
                    .expect("Error in building Basis Set keywords"),
            )
            .electronic(
                ElectronicParamBuilder::default()
                    .bands_option(
                        BandsOptionBuilder::default()
                            .extra_bands(ExtraBands::PercExtraBands(72.0))
                            .build()
                            .expect("Error in building extra bands options"),
                    )
                    .spin_polarised(true)
                    .spin(spin)
                    .build()
                    .expect("Error in building Electronic Parameter keywords"),
            )
            .electro_min(
                ElectroMinimizationBuilder::default()
                    .elec_energy_tol(1e-5)
                    .max_scf_cycles(6000)
                    .fix_occupancy(false)
                    .smearing_width(0.10)
                    .spin_fix(6)
                    .num_dump_cycles(0)
                    .metals_method(MetalsMethod::DM)
                    .build()
                    .expect("Error in building Electronic Minimization keywords"),
            )
            .density_mixing(
                DensityMixingParamsBuilder::default()
                    .mixing_scheme(MixingScheme::Pulay)
                    .mix_charge_amp(0.5)
                    .mix_spin_amp(2.0)
                    .mix_charge_gmax(1.5)
                    .mix_spin_gmax(1.5)
                    .mix_history_length(20)
                    .build()
                    .expect("Error in building Mixing Scheme keywords"),
            )
            .population_analysis(
                PopulationAnalysisBuilder::default()
                    .popn_calculate(true)
                    .popn_bond_cutoff(3.0)
                    .pdos_calculate_weights(true)
                    .build()
                    .expect("Error in building Population Analysis keywords"),
            )
            .xc_correlation(
                XcParamBuilder::default()
                    .xc_functional(XCFunctional::PBE)
                    .spin_polarised(true)
                    .build()
                    .expect("Error in building Exchange Correlation keywords"),
            );
        builder
    }
    pub fn geom_opt_param_template(
        cutoff_energy: f64,
        spin: f64,
    ) -> Result<CastepParam, CastepParamBuilderError> {
        let mut builder = CastepParam::param_template(cutoff_energy, spin);
        builder
            .general_keywords(
                GeneralBuilder::default()
                    .task(CastepTask::GeometryOptimization)
                    .comment("CASTEP calculation from Rhino".to_string())
                    .opt_strategy(OptStrategy::Speed)
                    .page_wvfns(0)
                    .write_props(
                        WritePropertiesBuilder::default()
                            .formatted_potential(true)
                            .formatted_density(true)
                            .orbitals(true)
                            .build()
                            .expect("Error in building Write Properties"),
                    )
                    .calculate_props(
                        CalculatePropertiesBuilder::default()
                            .elf(false)
                            .stress(false)
                            .hirshfeld(true)
                            .densdiff(false)
                            .build()
                            .expect("Calculate properties build failed"),
                    )
                    .build()
                    .expect("Error in building General keywords"),
            )
            .geom_opt(
                GeomOptBuilder::default()
                    .geom_energy_tol(5e-5)
                    .geom_force_tol(0.1)
                    .geom_stress_tol(0.2)
                    .geom_disp_tol(0.005)
                    .geom_max_iter(6000)
                    .geom_method(GeomMethod::BFGS)
                    .build()
                    .expect("Error in building Geometry Optimization keywords"),
            );
        builder.build()
    }

    pub fn dos_opt_param_template(
        cutoff_energy: f64,
        spin: f64,
    ) -> Result<CastepParam, CastepParamBuilderError> {
        let mut builder = CastepParam::param_template(cutoff_energy, spin);
        builder
            .general_keywords(
                GeneralBuilder::default()
                    .task(CastepTask::BandStructure)
                    .comment("CASTEP calculation from Rhino".to_string())
                    .continuation_reuse(Continuation::Default)
                    .opt_strategy(OptStrategy::Speed)
                    .page_wvfns(0)
                    .write_props(
                        WritePropertiesBuilder::default()
                            .formatted_potential(true)
                            .formatted_density(true)
                            .orbitals(true)
                            .build()
                            .expect("Error in building Write Properties"),
                    )
                    .calculate_props(
                        CalculatePropertiesBuilder::default()
                            .elf(false)
                            .stress(false)
                            .hirshfeld(true)
                            .densdiff(false)
                            .build()
                            .expect("Calculate properties build failed"),
                    )
                    .build()
                    .expect("Error in building General keywords"),
            )
            .band_structure(
                BandStructureBuilder::default()
                    .bs_extra_bands(BSExtraBands::PercExtra(72.0))
                    .bs_xc_functional(BSXcFunctional::PBE)
                    .bs_eigenvalue_tol(1e-5)
                    .bs_write_eigenvalues(true)
                    .build()
                    .expect("Error in building band_structure keywords"),
            )
            .build()
    }
}
