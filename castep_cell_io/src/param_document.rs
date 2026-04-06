//! Top-level document structure for CASTEP `.param` files.
//!
//! This module provides [`ParamDocument`], the primary type for representing a complete
//! CASTEP parameter file in memory. It organizes the 100+ CASTEP parameters into 18
//! logical groups for better maintainability and discoverability.
//!
//! # Structure
//!
//! Parameters are organized into these groups:
//! - [`GeneralParams`] — task type, output verbosity, continuation
//! - [`ElectronicParams`] — charge, spin, band counts, smearing
//! - [`BasisSetParams`] — cutoff energy, finite basis corrections
//! - [`ExchangeCorrelationParams`] — XC functional, spin polarization, DFT+U
//! - [`ElectronicMinimisationParams`] — SCF convergence, mixing schemes
//! - [`GeometryOptimizationParams`] — optimization method, convergence criteria
//! - [`PhononParams`] — phonon calculation settings
//! - [`BandStructureParams`] — band structure calculation parameters
//! - [`MolecularDynamicsParams`] — MD ensemble, timestep, temperature
//! - [`ElectricFieldParams`] — finite field calculations
//! - [`PseudopotentialParams`] — pseudopotential generation and testing
//! - [`DensityMixingParams`] — charge density mixing parameters
//! - [`PopulationAnalysisParams`] — Mulliken, Hirshfeld analysis
//! - [`OpticsParams`] — optical property calculations
//! - [`NmrParams`] — NMR chemical shift calculations
//! - [`SolvationParams`] — implicit solvent models
//! - [`ElectronicExcitationsParams`] — excited state calculations
//! - [`TransitionStateParams`] — transition state search parameters
//!
//! # Usage
//!
//! ## Parsing from text
//!
//! ```no_run
//! use castep_cell_io::ParamDocument;
//!
//! let input = std::fs::read_to_string("calculation.param")?;
//! let doc = castep_cell_fmt::parse::<ParamDocument>(&input)?;
//!
//! // Access parameter groups
//! if let Some(task) = &doc.general.task {
//!     println!("Task: {:?}", task);
//! }
//! if let Some(cutoff) = &doc.basis_set.cutoff_energy {
//!     println!("Cutoff: {} eV", cutoff.value);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Building programmatically
//!
//! ```
//! use castep_cell_io::ParamDocument;
//!
//! let doc = ParamDocument::builder()
//!     .general(Default::default())
//!     .electronic(Default::default())
//!     .basis_set(Default::default())
//!     .exchange_correlation(Default::default())
//!     .electronic_minimisation(Default::default())
//!     .geometry_optimization(Default::default())
//!     .phonon(Default::default())
//!     .band_structure(Default::default())
//!     .molecular_dynamics(Default::default())
//!     .electric_field(Default::default())
//!     .pseudopotential(Default::default())
//!     .density_mixing(Default::default())
//!     .population_analysis(Default::default())
//!     .optics(Default::default())
//!     .nmr(Default::default())
//!     .solvation(Default::default())
//!     .electronic_excitations(Default::default())
//!     .transition_state(Default::default())
//!     .build();
//! ```
//!
//! ## Serializing to text
//!
//! ```no_run
//! use castep_cell_io::ParamDocument;
//! use castep_cell_fmt::{ToCellFile, format::to_string_many_spaced};
//!
//! let doc = ParamDocument::default();
//! let cells = doc.to_cell_file();
//! let output = to_string_many_spaced(&cells);
//! # drop(output);
//! ```

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

/// Complete representation of a CASTEP `.param` file.
///
/// This is the primary type for working with CASTEP parameter files. It organizes
/// the 100+ CASTEP parameters into 18 logical groups, making it easier to discover
/// related parameters and maintain the codebase.
///
/// # Organization
///
/// Each field represents a group of related parameters. All groups default to empty
/// (all parameters `None`), allowing you to specify only the parameters you need.
///
/// # Validation
///
/// The document automatically validates inter-group constraints during parsing:
/// - Band count parameters (`NBANDS`, `NEXTRA_BANDS`, `PERC_EXTRA_BANDS`) are mutually exclusive
/// - Band structure parameters (`BS_NBANDS`, `BS_NEXTRA_BANDS`, `BS_PERC_EXTRA_BANDS`) are mutually exclusive
/// - Optics parameters (`OPTICS_NBANDS`, `OPTICS_NEXTRA_BANDS`, `OPTICS_PERC_EXTRA_BANDS`) are mutually exclusive
///
/// Each parameter group also validates its own internal constraints.
///
/// # Construction
///
/// Use the builder pattern (via [`bon`](https://docs.rs/bon)):
///
/// ```
/// use castep_cell_io::ParamDocument;
///
/// let doc = ParamDocument::builder()
///     .general(Default::default())
///     .electronic(Default::default())
///     .basis_set(Default::default())
///     .exchange_correlation(Default::default())
///     .electronic_minimisation(Default::default())
///     .geometry_optimization(Default::default())
///     .phonon(Default::default())
///     .band_structure(Default::default())
///     .molecular_dynamics(Default::default())
///     .electric_field(Default::default())
///     .pseudopotential(Default::default())
///     .density_mixing(Default::default())
///     .population_analysis(Default::default())
///     .optics(Default::default())
///     .nmr(Default::default())
///     .solvation(Default::default())
///     .electronic_excitations(Default::default())
///     .transition_state(Default::default())
///     .build();
/// ```
///
/// Or use [`Default`] for an empty document:
///
/// ```
/// use castep_cell_io::ParamDocument;
///
/// let doc = ParamDocument::default();
/// assert!(doc.general.task.is_none());
/// ```
///
/// # Parsing and Serialization
///
/// Implements [`FromCellFile`] for parsing and [`ToCellFile`] for serialization.
/// Parsing automatically applies validation.
///
/// See the module-level documentation for examples.
#[derive(Debug, Clone, Default, Builder)]
pub struct ParamDocument {
    /// General calculation parameters.
    ///
    /// Controls task type, output verbosity, continuation, and runtime limits.
    /// See [`GeneralParams`] for available parameters.
    pub general: GeneralParams,
    /// Electronic structure parameters.
    ///
    /// Controls charge, spin, band counts, and electronic smearing.
    /// See [`ElectronicParams`] for available parameters.
    pub electronic: ElectronicParams,
    /// Basis set parameters.
    ///
    /// Controls plane-wave cutoff energy and finite basis corrections.
    /// See [`BasisSetParams`] for available parameters.
    pub basis_set: BasisSetParams,
    /// Exchange-correlation functional parameters.
    ///
    /// Controls XC functional choice, spin polarization, and DFT+U.
    /// See [`ExchangeCorrelationParams`] for available parameters.
    pub exchange_correlation: ExchangeCorrelationParams,
    /// Electronic minimization (SCF) parameters.
    ///
    /// Controls SCF convergence criteria and mixing schemes.
    /// See [`ElectronicMinimisationParams`] for available parameters.
    pub electronic_minimisation: ElectronicMinimisationParams,
    /// Geometry optimization parameters.
    ///
    /// Controls optimization method and convergence criteria.
    /// See [`GeometryOptimizationParams`] for available parameters.
    pub geometry_optimization: GeometryOptimizationParams,
    /// Phonon calculation parameters.
    ///
    /// Controls phonon calculation settings and convergence.
    /// See [`PhononParams`] for available parameters.
    pub phonon: PhononParams,
    /// Band structure calculation parameters.
    ///
    /// Controls band structure calculation settings.
    /// See [`BandStructureParams`] for available parameters.
    pub band_structure: BandStructureParams,
    /// Molecular dynamics parameters.
    ///
    /// Controls MD ensemble, timestep, temperature, and thermostat.
    /// See [`MolecularDynamicsParams`] for available parameters.
    pub molecular_dynamics: MolecularDynamicsParams,
    /// Electric field parameters.
    ///
    /// Controls finite electric field calculations.
    /// See [`ElectricFieldParams`] for available parameters.
    pub electric_field: ElectricFieldParams,
    /// Pseudopotential parameters.
    ///
    /// Controls pseudopotential generation and testing.
    /// See [`PseudopotentialParams`] for available parameters.
    pub pseudopotential: PseudopotentialParams,
    /// Density mixing parameters.
    ///
    /// Controls charge density mixing during SCF.
    /// See [`DensityMixingParams`] for available parameters.
    pub density_mixing: DensityMixingParams,
    /// Population analysis parameters.
    ///
    /// Controls Mulliken and Hirshfeld population analysis.
    /// See [`PopulationAnalysisParams`] for available parameters.
    pub population_analysis: PopulationAnalysisParams,
    /// Optical properties parameters.
    ///
    /// Controls optical property calculations.
    /// See [`OpticsParams`] for available parameters.
    pub optics: OpticsParams,
    /// NMR parameters.
    ///
    /// Controls NMR chemical shift calculations.
    /// See [`NmrParams`] for available parameters.
    pub nmr: NmrParams,
    /// Solvation parameters.
    ///
    /// Controls implicit solvent models.
    /// See [`SolvationParams`] for available parameters.
    pub solvation: SolvationParams,
    /// Electronic excitations parameters.
    ///
    /// Controls excited state calculations.
    /// See [`ElectronicExcitationsParams`] for available parameters.
    pub electronic_excitations: ElectronicExcitationsParams,
    /// Transition state search parameters.
    ///
    /// Controls transition state search methods.
    /// See [`TransitionStateParams`] for available parameters.
    pub transition_state: TransitionStateParams,
}

impl ParamDocument {
    /// Validates inter-group and intra-group constraints.
    ///
    /// This method is automatically called during parsing via [`FromCellFile`].
    /// You typically don't need to call it manually unless you're constructing
    /// a document programmatically and want to verify it's valid.
    ///
    /// # Validation Rules
    ///
    /// ## Intra-group validation
    /// Each parameter group validates its own constraints (e.g., value ranges,
    /// required combinations).
    ///
    /// ## Inter-group validation
    /// - **Electronic band counts**: `NBANDS`, `NEXTRA_BANDS`, and `PERC_EXTRA_BANDS`
    ///   are mutually exclusive
    /// - **Band structure band counts**: `BS_NBANDS`, `BS_NEXTRA_BANDS`, and
    ///   `BS_PERC_EXTRA_BANDS` are mutually exclusive
    /// - **Optics band counts**: `OPTICS_NBANDS`, `OPTICS_NEXTRA_BANDS`, and
    ///   `OPTICS_PERC_EXTRA_BANDS` are mutually exclusive
    ///
    /// # Errors
    ///
    /// Returns `Err` with a descriptive message if any validation constraint is violated.
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
    /// Parse a [`ParamDocument`] from a slice of parsed [`Cell`] tokens.
    ///
    /// This method is called by [`castep_cell_fmt::parse`] after tokenizing the input text.
    /// It delegates parsing to each parameter group, then validates the complete document.
    ///
    /// # Parsing Strategy
    ///
    /// Each parameter group independently scans the token stream for its keywords.
    /// This allows parameters to appear in any order in the file. Unknown keywords
    /// are silently ignored (CASTEP's behavior).
    ///
    /// # Validation
    ///
    /// After parsing all groups, validation is automatically called to check
    /// inter-group and intra-group constraints.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if:
    /// - Any parameter value is malformed
    /// - Validation constraints are violated
    /// - Required parameter combinations are missing
    ///
    /// # Example
    ///
    /// ```no_run
    /// use castep_cell_io::ParamDocument;
    ///
    /// let input = r#"
    /// TASK : GeometryOptimization
    /// XC_FUNCTIONAL : PBE
    /// CUT_OFF_ENERGY : 500 eV
    /// "#;
    ///
    /// let doc = castep_cell_fmt::parse::<ParamDocument>(input)?;
    /// # Ok::<(), castep_cell_fmt::Error>(())
    /// ```
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
    /// Serialize this document to a vector of [`Cell`] tokens.
    ///
    /// Converts the structured document back to the token representation used by
    /// [`castep_cell_fmt`]. The tokens can then be formatted to text with
    /// [`castep_cell_fmt::format`].
    ///
    /// # Group Order
    ///
    /// Parameters are emitted in a standard order matching the field declaration order:
    /// 1. General parameters
    /// 2. Electronic structure
    /// 3. Basis set
    /// 4. Exchange-correlation
    /// 5. Electronic minimization
    /// 6. Geometry optimization
    /// 7. Phonon calculations
    /// 8. Band structure
    /// 9. Molecular dynamics
    /// 10. Electric field
    /// 11. Pseudopotentials
    /// 12. Density mixing
    /// 13. Population analysis
    /// 14. Optics
    /// 15. NMR
    /// 16. Solvation
    /// 17. Electronic excitations
    /// 18. Transition state search
    ///
    /// Parameters that are `None` are omitted from the output.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use castep_cell_io::ParamDocument;
    /// use castep_cell_fmt::{ToCellFile, format::to_string_many_spaced};
    ///
    /// let doc = ParamDocument::default();
    /// let cells = doc.to_cell_file();
    /// let output = to_string_many_spaced(&cells);
    /// # drop(output);
    /// ```
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
