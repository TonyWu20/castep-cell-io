use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromCellFile, ToCellFile};
use crate::param::{
    general_params::GeneralParams, electronic_params::ElectronicParams,
    basis_set_params::BasisSetParams, exchange_correlation_params::ExchangeCorrelationParams,
    electronic_minimisation_params::ElectronicMinimisationParams,
    geometry_optimization_params::GeometryOptimizationParams, phonon_params::PhononParams,
    band_structure_params::BandStructureParams, molecular_dynamics_params::MolecularDynamicsParams,
    electric_field_params::ElectricFieldParams, pseudopotential_params::PseudopotentialParams,
    density_mixing_params::DensityMixingParams, population_analysis_params::PopulationAnalysisParams,
    optics_params::OpticsParams, nmr_params::NmrParams, solvation_params::SolvationParams,
    electronic_excitations_params::ElectronicExcitationsParams,
    transition_state_params::TransitionStateParams,
};

/// Top-level aggregate struct for .param files
///
/// Organizes CASTEP .param file parameters into 18 logical groups for better
/// maintainability and organization. Each group contains related parameters
/// that control a specific aspect of the calculation.
#[derive(Debug, Clone, Default, Builder)]
pub struct ParamDocument {
    pub general: GeneralParams,
    pub electronic: ElectronicParams,
    pub basis_set: BasisSetParams,
    pub exchange_correlation: ExchangeCorrelationParams,
    pub electronic_minimisation: ElectronicMinimisationParams,
    pub geometry_optimization: GeometryOptimizationParams,
    pub phonon: PhononParams,
    pub band_structure: BandStructureParams,
    pub molecular_dynamics: MolecularDynamicsParams,
    pub electric_field: ElectricFieldParams,
    pub pseudopotential: PseudopotentialParams,
    pub density_mixing: DensityMixingParams,
    pub population_analysis: PopulationAnalysisParams,
    pub optics: OpticsParams,
    pub nmr: NmrParams,
    pub solvation: SolvationParams,
    pub electronic_excitations: ElectronicExcitationsParams,
    pub transition_state: TransitionStateParams,
}

impl ParamDocument {
    /// Validates inter-group constraints
    fn validate(mut self) -> Result<Self, String> {
        // Validate each group by consuming and reassigning
        self.general = self.general.validate()?;
        self.electronic = self.electronic.validate()?;
        self.basis_set = self.basis_set.validate()?;
        self.exchange_correlation = self.exchange_correlation.validate()?;
        self.electronic_minimisation = self.electronic_minimisation.validate()?;
        self.geometry_optimization = self.geometry_optimization.validate()?;
        self.phonon = self.phonon.validate()?;
        self.band_structure = self.band_structure.validate()?;
        self.molecular_dynamics = self.molecular_dynamics.validate()?;
        self.electric_field = self.electric_field.validate()?;
        self.pseudopotential = self.pseudopotential.validate()?;
        self.density_mixing = self.density_mixing.validate()?;
        self.population_analysis = self.population_analysis.validate()?;
        self.optics = self.optics.validate()?;
        self.nmr = self.nmr.validate()?;
        self.solvation = self.solvation.validate()?;
        self.electronic_excitations = self.electronic_excitations.validate()?;
        self.transition_state = self.transition_state.validate()?;

        // Inter-group validation: Band structure mutual exclusivity
        let bs_count = [
            self.band_structure.bs_nbands.is_some(),
            self.band_structure.bs_nextra_bands.is_some(),
            self.band_structure.bs_perc_extra_bands.is_some(),
        ]
        .iter()
        .filter(|&&x| x)
        .count();
        if bs_count > 1 {
            return Err(
                "BS_NBANDS, BS_NEXTRA_BANDS, and BS_PERC_EXTRA_BANDS are mutually exclusive. Only one may be specified."
                    .into(),
            );
        }

        // Inter-group validation: Electronic mutual exclusivity
        let elec_count = [
            self.electronic.nbands.is_some(),
            self.electronic.nextra_bands.is_some(),
            self.electronic.perc_extra_bands.is_some(),
        ]
        .iter()
        .filter(|&&x| x)
        .count();
        if elec_count > 1 {
            return Err(
                "NBANDS, NEXTRA_BANDS, and PERC_EXTRA_BANDS are mutually exclusive. Only one may be specified."
                    .into(),
            );
        }

        // Inter-group validation: Optics mutual exclusivity
        let optics_count = [
            self.optics.optics_nbands.is_some(),
            self.optics.optics_nextra_bands.is_some(),
            self.optics.optics_perc_extra_bands.is_some(),
        ]
        .iter()
        .filter(|&&x| x)
        .count();
        if optics_count > 1 {
            return Err(
                "OPTICS_NBANDS, OPTICS_NEXTRA_BANDS, and OPTICS_PERC_EXTRA_BANDS are mutually exclusive. Only one may be specified."
                    .into(),
            );
        }

        Ok(self)
    }
}

impl FromCellFile for ParamDocument {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        ParamDocument::builder()
            .general(GeneralParams::from_cell_file(tokens)?)
            .electronic(ElectronicParams::from_cell_file(tokens)?)
            .basis_set(BasisSetParams::from_cell_file(tokens)?)
            .exchange_correlation(ExchangeCorrelationParams::from_cell_file(tokens)?)
            .electronic_minimisation(ElectronicMinimisationParams::from_cell_file(tokens)?)
            .geometry_optimization(GeometryOptimizationParams::from_cell_file(tokens)?)
            .phonon(PhononParams::from_cell_file(tokens)?)
            .band_structure(BandStructureParams::from_cell_file(tokens)?)
            .molecular_dynamics(MolecularDynamicsParams::from_cell_file(tokens)?)
            .electric_field(ElectricFieldParams::from_cell_file(tokens)?)
            .pseudopotential(PseudopotentialParams::from_cell_file(tokens)?)
            .density_mixing(DensityMixingParams::from_cell_file(tokens)?)
            .population_analysis(PopulationAnalysisParams::from_cell_file(tokens)?)
            .optics(OpticsParams::from_cell_file(tokens)?)
            .nmr(NmrParams::from_cell_file(tokens)?)
            .solvation(SolvationParams::from_cell_file(tokens)?)
            .electronic_excitations(ElectronicExcitationsParams::from_cell_file(tokens)?)
            .transition_state(TransitionStateParams::from_cell_file(tokens)?)
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for ParamDocument {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        cells.extend(self.general.to_cell_file());
        cells.extend(self.electronic.to_cell_file());
        cells.extend(self.basis_set.to_cell_file());
        cells.extend(self.exchange_correlation.to_cell_file());
        cells.extend(self.electronic_minimisation.to_cell_file());
        cells.extend(self.geometry_optimization.to_cell_file());
        cells.extend(self.phonon.to_cell_file());
        cells.extend(self.band_structure.to_cell_file());
        cells.extend(self.molecular_dynamics.to_cell_file());
        cells.extend(self.electric_field.to_cell_file());
        cells.extend(self.pseudopotential.to_cell_file());
        cells.extend(self.density_mixing.to_cell_file());
        cells.extend(self.population_analysis.to_cell_file());
        cells.extend(self.optics.to_cell_file());
        cells.extend(self.nmr.to_cell_file());
        cells.extend(self.solvation.to_cell_file());
        cells.extend(self.electronic_excitations.to_cell_file());
        cells.extend(self.transition_state.to_cell_file());
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_fmt::parse;

    #[test]
    #[ignore]
    fn test_parse_co3o4_2_param() {
        let input = std::fs::read_to_string("../Co3O4_2.param").unwrap();
        let doc: ParamDocument = parse(&input).unwrap();

        assert_eq!(
            doc.general.task.unwrap(),
            crate::param::general::Task::GeometryOptimization
        );
        assert_eq!(
            doc.exchange_correlation.xc_functional.unwrap(),
            crate::param::exchange_correlation::XcFunctional::Pbe
        );
        assert_eq!(doc.exchange_correlation.spin_polarized.unwrap().0, false);
        assert_eq!(doc.basis_set.cutoff_energy.unwrap().value, 900.0);
        assert_eq!(doc.electronic_minimisation.max_scf_cycles.unwrap().0, 400);
        assert_eq!(
            doc.geometry_optimization.geom_method.unwrap(),
            crate::param::geometry_optimization::GeomMethod::Bfgs
        );
    }

    #[test]
    fn test_default_construction() {
        let doc = ParamDocument::default();
        assert!(doc.general.task.is_none());
        assert!(doc.electronic.charge.is_none());
    }

    #[test]
    fn test_builder_construction() {
        let doc = ParamDocument::builder()
            .general(GeneralParams::default())
            .electronic(ElectronicParams::default())
            .basis_set(BasisSetParams::default())
            .exchange_correlation(ExchangeCorrelationParams::default())
            .electronic_minimisation(ElectronicMinimisationParams::default())
            .geometry_optimization(GeometryOptimizationParams::default())
            .phonon(PhononParams::default())
            .band_structure(BandStructureParams::default())
            .molecular_dynamics(MolecularDynamicsParams::default())
            .electric_field(ElectricFieldParams::default())
            .pseudopotential(PseudopotentialParams::default())
            .density_mixing(DensityMixingParams::default())
            .population_analysis(PopulationAnalysisParams::default())
            .optics(OpticsParams::default())
            .nmr(NmrParams::default())
            .solvation(SolvationParams::default())
            .electronic_excitations(ElectronicExcitationsParams::default())
            .transition_state(TransitionStateParams::default())
            .build();
        assert!(doc.general.task.is_none());
    }

    #[test]
    fn test_validate_empty() {
        let doc = ParamDocument::default();
        assert!(doc.validate().is_ok());
    }

    #[test]
    fn test_to_cell_file_empty() {
        let doc = ParamDocument::default();
        let cells = doc.to_cell_file();
        assert_eq!(cells.len(), 0);
    }
}
