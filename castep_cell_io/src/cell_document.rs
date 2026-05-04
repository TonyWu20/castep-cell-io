//! Top-level document structure for CASTEP `.cell` files.
//!
//! This module provides [`CellDocument`], the primary type for representing a complete
//! CASTEP cell file in memory. It handles both parsing from text format and serialization
//! back to the CASTEP format.
//!
//! # Structure
//!
//! A cell document consists of:
//! - **Required fields**: lattice vectors and atomic positions
//! - **Optional blocks**: k-point sampling, constraints, external fields, species properties,
//!   phonon calculations, and more
//!
//! # Usage
//!
//! ## Parsing from text
//!
//! ```no_run
//! use castep_cell_io::CellDocument;
//!
//! let input = std::fs::read_to_string("structure.cell")?;
//! let doc = castep_cell_fmt::parse::<CellDocument>(&input)?;
//!
//! // Access required fields
//! println!("Lattice: {:?}", doc.lattice);
//! println!("Positions: {:?}", doc.positions);
//!
//! // Check optional blocks
//! if let Some(kpoints) = &doc.kpoints_list {
//!     println!("K-points defined: {:?}", kpoints);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Building programmatically
//!
//! ```ignore
//! use castep_cell_io::CellDocument;
//!
//! let doc = CellDocument::builder()
//!     .lattice(todo!()) // LatticeCart instance
//!     .positions(todo!()) // PositionsFrac or PositionsAbs instance
//!     .build();
//! ```
//!
//! ## Serializing to text
//!
//! ```ignore
//! use castep_cell_io::CellDocument;
//! use castep_cell_fmt::{ToCellFile, format::to_string_many_spaced};
//!
//! // Assuming you have a CellDocument instance
//! let doc = todo!(); // Your CellDocument instance
//! let cells = doc.to_cell_file();
//! let output = to_string_many_spaced(&cells);
//! ```

use bon::Builder;
use castep_cell_fmt::{
    CResult, Cell, CellValue, Error, FromKeyValue, ToCell, ToCellFile,
    parse::{FromBlock, FromCellFile},
    query::{find_block, find_block_any, has_flag},
};

use crate::cell::{
    bz_sampling_kpoints::{
        BSKpointList, BsKpointPath, BsKpointPathSpacing, KpointsList, KpointsMpGrid,
        KpointsMpOffset, KpointsMpSpacing, MagresKpointsList, OpticsKpointsList,
        SpectralKpointPath, SpectralKpointsList, SpectralKpointPathSpacing,
        SpectralKpointsMpGrid, SpectralKpointsMpSpacing, SpectralKpointsMpOffset,
    },
    constraints::{
        CellConstraints, FixAllCell, FixAllIons, FixCOM, FixVOL, IonicConstraints,
        NonlinearConstraints,
    },
    external_fields::{ExternalEfield, ExternalPressure},
    lattice_param::{LatticeABC, LatticeCart},
    phonon::{
        PhononFineKpointList, PhononFineKpointPath, PhononFineKpointPathSpacing,
        PhononFineKpointsMpGrid, PhononFineKpointsMpOffset, PhononFineKpointsMpSpacing,
        PhononGammaDirections, PhononKpointList, PhononKpointPath,
        PhononKpointsMpGrid, PhononKpointsMpOffset, PhononKpointsMpSpacing,
        PhononSupercellMatrix, SupercellKpointListCastep,
    },
    positions::{PositionsAbs, PositionsFrac},
    species::{HubbardU, SedcCustomParams, SpeciesLcaoStates, SpeciesMass, SpeciesPot, SpeciesQ},
    symmetry::{SymmetryGenerate, SymmetryOps, SymmetryTol},
    velocities::IonicVelocities,
};
use cell_document_builder::IsComplete;

/// Lattice vector specification for the simulation cell.
///
/// Defines the periodic boundary conditions of the crystal structure.
/// Supports both Cartesian and ABC+angles representations.
///
/// # Example
///
/// ```no_run
/// use castep_cell_io::Lattice;
///
/// // Use builder to construct LatticeCart, then wrap in enum
/// let lattice = Lattice::Cart(todo!());
/// ```
#[derive(Debug, Clone)]
pub enum Lattice {
    /// Lattice vectors in Cartesian coordinates (Angstroms).
    ///
    /// Corresponds to the `%BLOCK LATTICE_CART` section in CASTEP input files.
    Cart(LatticeCart),
    /// Lattice vectors in ABC+angles format.
    ///
    /// Corresponds to the `%BLOCK LATTICE_ABC` section in CASTEP input files.
    Abc(LatticeABC),
}

impl ToCell for Lattice {
    fn to_cell(&self) -> Cell<'_> {
        match self {
            Lattice::Cart(cart) => cart.to_cell(),
            Lattice::Abc(abc) => abc.to_cell(),
        }
    }
}

impl From<LatticeCart> for Lattice {
    fn from(v: LatticeCart) -> Self {
        Lattice::Cart(v)
    }
}

impl From<LatticeABC> for Lattice {
    fn from(v: LatticeABC) -> Self {
        Lattice::Abc(v)
    }
}

/// Atomic positions within the simulation cell.
///
/// Positions can be specified in either fractional (relative to lattice vectors)
/// or absolute Cartesian coordinates. CASTEP requires exactly one position block
/// per cell file.
///
/// # Coordinate Systems
///
/// - **Fractional**: Coordinates relative to lattice vectors (0.0 to 1.0 range).
///   Most common for periodic systems. Corresponds to `%BLOCK POSITIONS_FRAC`.
/// - **Absolute**: Cartesian coordinates in Angstroms. Useful for non-periodic
///   or mixed systems. Corresponds to `%BLOCK POSITIONS_ABS`.
///
/// # Example
///
/// ```no_run
/// use castep_cell_io::Positions;
///
/// // Use builder to construct PositionsFrac, then wrap in enum
/// let positions = Positions::Frac(todo!());
/// ```
#[derive(Debug, Clone)]
pub enum Positions {
    /// Fractional coordinates relative to lattice vectors.
    Frac(PositionsFrac),
    /// Absolute Cartesian coordinates in Angstroms.
    Abs(PositionsAbs),
}

impl ToCell for Positions {
    fn to_cell(&self) -> Cell<'_> {
        match self {
            Positions::Frac(frac) => frac.to_cell(),
            Positions::Abs(abs) => abs.to_cell(),
        }
    }
}

/// Complete representation of a CASTEP `.cell` file.
///
/// This is the primary type for working with CASTEP cell files. It contains all
/// structural information, calculation parameters, and optional blocks that can
/// appear in a cell file.
///
/// # Required Fields
///
/// - [`lattice`](Self::lattice): Periodic boundary conditions
/// - [`positions`](Self::positions): Atomic coordinates
///
/// All other fields are optional and correspond to specific CASTEP features.
///
/// # Construction
///
/// Use the builder pattern (via [`bon`](https://docs.rs/bon)) for ergonomic construction:
///
/// ```ignore
/// use castep_cell_io::CellDocument;
///
/// let doc = CellDocument::builder()
///     .lattice(todo!())  // LatticeCart - automatically wrapped in Lattice::Cart
///     .positions(todo!())  // PositionsFrac/PositionsAbs - automatically wrapped
///     .build();
/// ```
///
/// # Parsing and Serialization
///
/// Implements [`FromCellFile`] for parsing and [`ToCellFile`] for serialization:
///
/// ```no_run
/// use castep_cell_io::CellDocument;
/// use castep_cell_fmt::{ToCellFile, format::to_string_many_spaced};
///
/// // Parse from string
/// let input = std::fs::read_to_string("input.cell")?;
/// let doc = castep_cell_fmt::parse::<CellDocument>(&input)?;
///
/// // Serialize back to CASTEP format
/// let cells = doc.to_cell_file();
/// let output = to_string_many_spaced(&cells);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Optional Blocks
///
/// The document supports all standard CASTEP cell file blocks:
///
/// - **K-point sampling**: [`kpoints_list`](Self::kpoints_list), [`bs_kpoint_path`](Self::bs_kpoint_path),
///   [`bs_kpoints_list`](Self::bs_kpoints_list), [`optics_kpoints_list`](Self::optics_kpoints_list),
///   [`magres_kpoints_list`](Self::magres_kpoints_list)
/// - **Constraints**: [`ionic_constraints`](Self::ionic_constraints), [`nonlinear_constraints`](Self::nonlinear_constraints),
///   [`fix_all_ions`](Self::fix_all_ions), [`fix_all_cell`](Self::fix_all_cell), [`fix_com`](Self::fix_com)
/// - **External fields**: [`external_efield`](Self::external_efield), [`external_pressure`](Self::external_pressure)
/// - **Species properties**: [`species_mass`](Self::species_mass), [`species_pot`](Self::species_pot),
///   [`species_lcao_states`](Self::species_lcao_states), [`species_q`](Self::species_q),
///   [`hubbard_u`](Self::hubbard_u), [`sedc_custom_params`](Self::sedc_custom_params)
/// - **Phonon calculations**: [`phonon_kpoint_list`](Self::phonon_kpoint_list), [`phonon_kpoint_path`](Self::phonon_kpoint_path),
///   [`phonon_gamma_directions`](Self::phonon_gamma_directions), [`phonon_fine_kpoint_list`](Self::phonon_fine_kpoint_list),
///   [`phonon_supercell_matrix`](Self::phonon_supercell_matrix), [`supercell_kpoint_list`](Self::supercell_kpoint_list)
/// - **Dynamics**: [`ionic_velocities`](Self::ionic_velocities)
/// - **Symmetry**: [`symmetry_ops`](Self::symmetry_ops)
#[allow(clippy::duplicated_attributes)]
#[derive(Debug, Clone, Builder)]
#[builder(on(Lattice, into), on(Positions, into), finish_fn(vis = "", name = build_internal))]
pub struct CellDocument {
    /// Lattice vectors defining the simulation cell.
    ///
    /// Required field. Defines the periodic boundary conditions.
    pub lattice: Lattice,
    /// Atomic positions within the cell.
    ///
    /// Required field. Can be fractional or absolute coordinates.
    pub positions: Positions,
    /// K-point sampling grid for electronic structure calculations.
    ///
    /// Corresponds to `%BLOCK KPOINTS_LIST` in CASTEP.
    pub kpoints_list: Option<KpointsList>,
    /// K-point path for band structure calculations.
    ///
    /// Corresponds to `%BLOCK BS_KPOINT_PATH` in CASTEP.
    pub bs_kpoint_path: Option<BsKpointPath>,
    /// Explicit k-points for band structure calculations.
    ///
    /// Corresponds to `%BLOCK BS_KPOINTS_LIST` in CASTEP.
    pub bs_kpoints_list: Option<BSKpointList>,
    /// K-points for optical property calculations.
    ///
    /// Corresponds to `%BLOCK OPTICS_KPOINTS_LIST` in CASTEP.
    pub optics_kpoints_list: Option<OpticsKpointsList>,
    /// K-points for magnetic resonance calculations.
    ///
    /// Corresponds to `%BLOCK MAGRES_KPOINTS_LIST` in CASTEP.
    pub magres_kpoints_list: Option<MagresKpointsList>,
    /// Monkhorst-Pack grid for k-point sampling.
    ///
    /// Corresponds to `KPOINT_MP_GRID` in CASTEP.
    pub kpoints_mp_grid: Option<KpointsMpGrid>,
    /// Monkhorst-Pack grid spacing for k-point sampling.
    ///
    /// Corresponds to `KPOINT_MP_SPACING` in CASTEP.
    pub kpoints_mp_spacing: Option<KpointsMpSpacing>,
    /// Monkhorst-Pack grid offset for k-point sampling.
    ///
    /// Corresponds to `KPOINT_MP_OFFSET` in CASTEP.
    pub kpoints_mp_offset: Option<KpointsMpOffset>,
    /// Spacing for band structure k-point path.
    ///
    /// Corresponds to `BS_KPOINT_PATH_SPACING` in CASTEP.
    pub bs_kpoint_path_spacing: Option<BsKpointPathSpacing>,
    /// Spectral k-point path for band structure calculations.
    ///
    /// Corresponds to `%BLOCK SPECTRAL_KPOINT_PATH` in CASTEP.
    pub spectral_kpoint_path: Option<SpectralKpointPath>,
    /// Spectral k-points list for band structure calculations.
    ///
    /// Corresponds to `%BLOCK SPECTRAL_KPOINT_LIST` in CASTEP.
    pub spectral_kpoints_list: Option<SpectralKpointsList>,
    /// Spacing for spectral k-point path.
    ///
    /// Corresponds to `SPECTRAL_KPOINT_PATH_SPACING` in CASTEP.
    pub spectral_kpoint_path_spacing: Option<SpectralKpointPathSpacing>,
    /// Monkhorst-Pack grid for spectral k-point sampling.
    ///
    /// Corresponds to `SPECTRAL_KPOINT_MP_GRID` in CASTEP.
    pub spectral_kpoints_mp_grid: Option<SpectralKpointsMpGrid>,
    /// Monkhorst-Pack grid spacing for spectral k-point sampling.
    ///
    /// Corresponds to `SPECTRAL_KPOINT_MP_SPACING` in CASTEP.
    pub spectral_kpoints_mp_spacing: Option<SpectralKpointsMpSpacing>,
    /// Monkhorst-Pack grid offset for spectral k-point sampling.
    ///
    /// Corresponds to `SPECTRAL_KPOINT_MP_OFFSET` in CASTEP.
    pub spectral_kpoints_mp_offset: Option<SpectralKpointsMpOffset>,
    /// Explicit symmetry operations.
    ///
    /// Overrides automatic symmetry detection. Corresponds to `%BLOCK SYMMETRY_OPS`.
    pub symmetry_ops: Option<SymmetryOps>,
    /// Symmetry detection tolerance.
    ///
    /// Corresponds to `SYMMETRY_TOL` in CASTEP.
    pub symmetry_tol: Option<SymmetryTol>,
    /// Automatically generate symmetry operations.
    ///
    /// Corresponds to `SYMMETRY_GENERATE` flag in CASTEP.
    pub symmetry_generate: Option<SymmetryGenerate>,
    /// Fix center of mass during geometry optimization.
    ///
    /// Corresponds to `FIX_COM : TRUE` in CASTEP.
    pub fix_com: Option<FixCOM>,
    /// Linear constraints on ionic positions.
    ///
    /// Corresponds to `%BLOCK IONIC_CONSTRAINTS` in CASTEP.
    pub ionic_constraints: Option<IonicConstraints>,
    /// Nonlinear constraints on ionic positions.
    ///
    /// Corresponds to `%BLOCK NONLINEAR_CONSTRAINTS` in CASTEP.
    pub nonlinear_constraints: Option<NonlinearConstraints>,
    /// Prevent all ions from moving during optimization.
    ///
    /// Corresponds to `FIX_ALL_IONS : TRUE` in CASTEP.
    pub fix_all_ions: Option<FixAllIons>,
    /// Prevent cell parameters from changing during optimization.
    ///
    /// Corresponds to `FIX_ALL_CELL : TRUE` in CASTEP.
    pub fix_all_cell: Option<FixAllCell>,
    /// Fix the volume of the cell during optimization.
    ///
    /// Corresponds to `FIX_VOL : TRUE` in CASTEP.
    pub fix_vol: Option<FixVOL>,
    /// Cell constraints for geometry optimization.
    ///
    /// Corresponds to `%BLOCK CELL_CONSTRAINTS` in CASTEP.
    pub cell_constraints: Option<CellConstraints>,
    /// External electric field applied to the system.
    ///
    /// Corresponds to `%BLOCK EXTERNAL_EFIELD` in CASTEP.
    pub external_efield: Option<ExternalEfield>,
    /// External pressure applied to the cell.
    ///
    /// Corresponds to `%BLOCK EXTERNAL_PRESSURE` in CASTEP.
    pub external_pressure: Option<ExternalPressure>,
    /// Custom atomic masses for isotope calculations.
    ///
    /// Corresponds to `%BLOCK SPECIES_MASS` in CASTEP.
    pub species_mass: Option<SpeciesMass>,
    /// Pseudopotential files for each species.
    ///
    /// Corresponds to `%BLOCK SPECIES_POT` in CASTEP.
    pub species_pot: Option<SpeciesPot>,
    /// LCAO basis states for each species.
    ///
    /// Corresponds to `%BLOCK SPECIES_LCAO_STATES` in CASTEP.
    pub species_lcao_states: Option<SpeciesLcaoStates>,
    /// Ionic charges for each species.
    ///
    /// Corresponds to `%BLOCK SPECIES_Q` in CASTEP.
    pub species_q: Option<SpeciesQ>,
    /// Hubbard U parameters for DFT+U calculations.
    ///
    /// Corresponds to `%BLOCK HUBBARD_U` in CASTEP.
    pub hubbard_u: Option<HubbardU>,
    /// Custom parameters for semi-empirical dispersion correction.
    ///
    /// Corresponds to `%BLOCK SEDC_CUSTOM_PARAMS` in CASTEP.
    pub sedc_custom_params: Option<SedcCustomParams>,
    /// K-points for phonon calculations.
    ///
    /// Corresponds to `%BLOCK PHONON_KPOINT_LIST` in CASTEP.
    pub phonon_kpoint_list: Option<PhononKpointList>,
    /// K-point path for phonon dispersion calculations.
    ///
    /// Corresponds to `%BLOCK PHONON_KPOINT_PATH` in CASTEP.
    pub phonon_kpoint_path: Option<PhononKpointPath>,
    /// Monkhorst-Pack grid for phonon k-point sampling.
    ///
    /// Corresponds to `PHONON_KPOINT_MP_GRID` in CASTEP.
    pub phonon_kpoints_mp_grid: Option<PhononKpointsMpGrid>,
    /// Monkhorst-Pack grid spacing for phonon k-point sampling.
    ///
    /// Corresponds to `PHONON_KPOINT_MP_SPACING` in CASTEP.
    pub phonon_kpoints_mp_spacing: Option<PhononKpointsMpSpacing>,
    /// Monkhorst-Pack grid offset for phonon k-point sampling.
    ///
    /// Corresponds to `PHONON_KPOINT_MP_OFFSET` in CASTEP.
    pub phonon_kpoints_mp_offset: Option<PhononKpointsMpOffset>,
    /// Fine k-point path for phonon dispersion calculations.
    ///
    /// Corresponds to `%BLOCK PHONON_FINE_KPOINT_PATH` in CASTEP.
    pub phonon_fine_kpoint_path: Option<PhononFineKpointPath>,
    /// Spacing for fine k-point path.
    ///
    /// Corresponds to `PHONON_FINE_KPOINT_PATH_SPACING` in CASTEP.
    pub phonon_fine_kpoint_path_spacing: Option<PhononFineKpointPathSpacing>,
    /// Monkhorst-Pack grid for fine phonon k-point sampling.
    ///
    /// Corresponds to `PHONON_FINE_KPOINT_MP_GRID` in CASTEP.
    pub phonon_fine_kpoints_mp_grid: Option<PhononFineKpointsMpGrid>,
    /// Monkhorst-Pack grid spacing for fine phonon k-point sampling.
    ///
    /// Corresponds to `PHONON_FINE_KPOINT_MP_SPACING` in CASTEP.
    pub phonon_fine_kpoints_mp_spacing: Option<PhononFineKpointsMpSpacing>,
    /// Monkhorst-Pack grid offset for fine phonon k-point sampling.
    ///
    /// Corresponds to `PHONON_FINE_KPOINT_MP_OFFSET` in CASTEP.
    pub phonon_fine_kpoints_mp_offset: Option<PhononFineKpointsMpOffset>,
    /// Directions for Gamma-point phonon calculations.
    ///
    /// Corresponds to `%BLOCK PHONON_GAMMA_DIRECTIONS` in CASTEP.
    pub phonon_gamma_directions: Option<PhononGammaDirections>,
    /// Fine k-point grid for phonon interpolation.
    ///
    /// Corresponds to `%BLOCK PHONON_FINE_KPOINT_LIST` in CASTEP.
    pub phonon_fine_kpoint_list: Option<PhononFineKpointList>,
    /// Supercell matrix for phonon calculations.
    ///
    /// Corresponds to `%BLOCK PHONON_SUPERCELL_MATRIX` in CASTEP.
    pub phonon_supercell_matrix: Option<PhononSupercellMatrix>,
    /// K-points for supercell calculations.
    ///
    /// Corresponds to `%BLOCK SUPERCELL_KPOINT_LIST` in CASTEP.
    pub supercell_kpoint_list: Option<SupercellKpointListCastep>,
    /// Initial ionic velocities for molecular dynamics.
    ///
    /// Corresponds to `%BLOCK IONIC_VELOCITIES` in CASTEP.
    pub ionic_velocities: Option<IonicVelocities>,
}

impl<S: cell_document_builder::IsComplete> CellDocumentBuilder<S> {
    pub fn build(self) -> CResult<CellDocument> {
        let doc = self.build_internal();

        let kpoint_count = [doc.kpoints_list.is_some(), doc.kpoints_mp_grid.is_some(), doc.kpoints_mp_spacing.is_some()]
            .iter().filter(|&&x| x).count();
        if kpoint_count > 1 {
            return Err(Error::Message("At most one of kpoints_list, kpoints_mp_grid, kpoints_mp_spacing may be specified".into()));
        }

        let spectral_count = [
            doc.spectral_kpoint_path.is_some(),
            doc.spectral_kpoints_mp_grid.is_some(),
            doc.spectral_kpoints_mp_spacing.is_some(),
            doc.spectral_kpoints_list.is_some(),
            doc.bs_kpoint_path.is_some(),
            doc.bs_kpoints_list.is_some(),
        ].iter().filter(|&&x| x).count();
        if spectral_count > 1 {
            return Err(Error::Message("At most one of spectral_kpoint_path, spectral_kpoints_mp_grid, spectral_kpoints_mp_spacing, spectral_kpoints_list, bs_kpoint_path, bs_kpoints_list may be specified".into()));
        }

        let phonon_count = [doc.phonon_kpoint_path.is_some(), doc.phonon_kpoint_list.is_some()]
            .iter().filter(|&&x| x).count();
        if phonon_count > 1 {
            return Err(Error::Message("At most one of phonon_kpoint_path, phonon_kpoint_list may be specified".into()));
        }

        let symmetry_count = [doc.symmetry_generate.is_some(), doc.symmetry_ops.is_some()]
            .iter().filter(|&&x| x).count();
        if symmetry_count > 1 {
            return Err(Error::Message("At most one of symmetry_generate, symmetry_ops may be specified".into()));
        }

        Ok(doc)
    }
}

impl FromCellFile for CellDocument {
    /// Parse a [`CellDocument`] from a slice of parsed [`Cell`] tokens.
    ///
    /// This method is called by [`castep_cell_fmt::parse`] after tokenizing the input text.
    /// It extracts all recognized blocks and keywords from the token stream.
    ///
    /// # Required Blocks
    ///
    /// - `%BLOCK LATTICE_CART` — must be present
    /// - Either `%BLOCK POSITIONS_FRAC` or `%BLOCK POSITIONS_ABS` — must have exactly one
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if:
    /// - Required blocks are missing
    /// - Block content is malformed
    /// - Multiple position blocks are present
    /// - Any block fails to parse according to its schema
    ///
    /// # Example
    ///
    /// ```no_run
    /// use castep_cell_io::CellDocument;
    ///
    /// let input = r#"
    /// %BLOCK LATTICE_CART
    ///   10.0  0.0  0.0
    ///    0.0 10.0  0.0
    ///    0.0  0.0 10.0
    /// %ENDBLOCK LATTICE_CART
    ///
    /// %BLOCK POSITIONS_FRAC
    /// Si  0.0  0.0  0.0
    /// Si  0.25 0.25 0.25
    /// %ENDBLOCK POSITIONS_FRAC
    /// "#;
    ///
    /// let doc = castep_cell_fmt::parse::<CellDocument>(input)?;
    /// # Ok::<(), castep_cell_fmt::Error>(())
    /// ```
    fn from_cell_file(cells: &[Cell<'_>]) -> CResult<Self> {
        let has_lattice_cart = find_block(cells, "LATTICE_CART").is_ok();
        let has_lattice_abc = find_block(cells, "LATTICE_ABC").is_ok();
        if has_lattice_cart && has_lattice_abc {
            return Err(Error::Message(
                "Both LATTICE_CART and LATTICE_ABC are specified. Only one lattice specification is allowed."
                    .into(),
            ));
        }
        let lattice = if has_lattice_cart {
            Lattice::Cart(LatticeCart::from_block_rows(find_block(
                cells,
                "LATTICE_CART",
            )?)?)
        } else {
            let rows = find_block(cells, "LATTICE_ABC")?;
            Lattice::Abc(LatticeABC::from_block_rows(rows)?)
        };

        let positions = if find_block(cells, "POSITIONS_FRAC").is_ok() {
            Positions::Frac(PositionsFrac::from_block_rows(find_block(
                cells,
                "POSITIONS_FRAC",
            )?)?)
        } else {
            Positions::Abs(PositionsAbs::from_block_rows(find_block(
                cells,
                "POSITIONS_ABS",
            )?)?)
        };

        let kpoints_list = find_block_any(cells, &["KPOINT_LIST", "KPOINTS_LIST"])
            .ok()
            .map(|rows| KpointsList::from_block_rows(rows))
            .transpose()?;

        let optics_kpoints_list = find_block_any(cells, &["OPTICS_KPOINT_LIST", "OPTICS_KPOINTS_LIST"])
            .ok()
            .map(|rows| OpticsKpointsList::from_block_rows(rows))
            .transpose()?;

        let magres_kpoints_list = find_block_any(cells, &["MAGRES_KPOINT_LIST", "MAGRES_KPOINTS_LIST"])
            .ok()
            .map(|rows| MagresKpointsList::from_block_rows(rows))
            .transpose()?;

        let spectral_kpoint_path = find_block_any(
            cells,
            &["SPECTRAL_KPOINT_PATH", "SPECTRAL_KPOINTS_PATH", "BS_KPOINT_PATH", "BS_KPOINTS_PATH"],
        )
        .ok()
        .map(|rows| SpectralKpointPath::from_block_rows(rows))
        .transpose()?;

        let spectral_kpoints_list = find_block_any(
            cells,
            &["SPECTRAL_KPOINT_LIST", "SPECTRAL_KPOINTS_LIST", "BS_KPOINT_LIST", "BS_KPOINTS_LIST"],
        )
        .ok()
        .map(|rows| SpectralKpointsList::from_block_rows(rows))
        .transpose()?;

        let bs_kpoint_path = if spectral_kpoint_path.is_some() {
            None
        } else {
            find_block_any(cells, &["BS_KPOINT_PATH", "BS_KPOINTS_PATH"])
                .ok()
                .map(|rows| BsKpointPath::from_block_rows(rows))
                .transpose()?
        };

        let bs_kpoints_list = if spectral_kpoints_list.is_some() {
            None
        } else {
            find_block_any(cells, &["BS_KPOINT_LIST", "BS_KPOINTS_LIST"])
                .ok()
                .map(|rows| BSKpointList::from_block_rows(rows))
                .transpose()?
        };

        let bs_kpoint_path_spacing = BsKpointPathSpacing::from_cells(cells)?;

        let kpoints_mp_grid = KpointsMpGrid::from_cells(cells)?;
        let kpoints_mp_spacing = KpointsMpSpacing::from_cells(cells)?;
        let kpoints_mp_offset = KpointsMpOffset::from_cells(cells)?;

        let spectral_kpoint_path_spacing = SpectralKpointPathSpacing::from_cells(cells)?;
        let spectral_kpoints_mp_grid = SpectralKpointsMpGrid::from_cells(cells)?;
        let spectral_kpoints_mp_spacing = SpectralKpointsMpSpacing::from_cells(cells)?;
        let spectral_kpoints_mp_offset = SpectralKpointsMpOffset::from_cells(cells)?;

        let symmetry_ops = find_block(cells, "SYMMETRY_OPS")
            .ok()
            .map(|rows| SymmetryOps::from_block_rows(rows))
            .transpose()?;

        let symmetry_tol = SymmetryTol::from_cells(cells)?;

        let symmetry_generate = if has_flag(cells, "SYMMETRY_GENERATE") {
            Some(SymmetryGenerate)
        } else {
            None
        };

        let fix_com = cells.iter().find_map(|c| {
            if let Cell::KeyValue(k, _v) = c
                && k.eq_ignore_ascii_case("FIX_COM")
            {
                return Some(FixCOM(true));
            }
            None
        });

        let ionic_constraints = find_block(cells, "IONIC_CONSTRAINTS")
            .ok()
            .map(|rows| IonicConstraints::from_block_rows(rows))
            .transpose()?;

        let nonlinear_constraints = find_block(cells, "NONLINEAR_CONSTRAINTS")
            .ok()
            .map(|rows| NonlinearConstraints::from_block_rows(rows))
            .transpose()?;

        let fix_all_ions = cells.iter().find_map(|c| {
            if let Cell::KeyValue(k, _v) = c
                && k.eq_ignore_ascii_case("FIX_ALL_IONS")
            {
                return Some(FixAllIons(true));
            }
            None
        });

        let fix_all_cell = cells.iter().find_map(|c| {
            if let Cell::KeyValue(k, _v) = c
                && k.eq_ignore_ascii_case("FIX_ALL_CELL")
            {
                return Some(FixAllCell(true));
            }
            None
        });

        let fix_vol = FixVOL::from_cells(cells)?;
        let cell_constraints = find_block(cells, "CELL_CONSTRAINTS")
            .ok()
            .map(|rows| CellConstraints::from_block_rows(rows))
            .transpose()?;

        let external_efield = find_block(cells, "EXTERNAL_EFIELD")
            .ok()
            .map(|rows| ExternalEfield::from_block_rows(rows))
            .transpose()?;

        let external_pressure = find_block(cells, "EXTERNAL_PRESSURE")
            .ok()
            .map(|rows| ExternalPressure::from_block_rows(rows))
            .transpose()?;

        let species_mass = find_block(cells, "SPECIES_MASS")
            .ok()
            .map(|rows| SpeciesMass::from_block_rows(rows))
            .transpose()?;

        let species_pot = find_block(cells, "SPECIES_POT")
            .ok()
            .map(|rows| SpeciesPot::from_block_rows(rows))
            .transpose()?;

        let species_lcao_states = find_block(cells, "SPECIES_LCAO_STATES")
            .ok()
            .map(|rows| SpeciesLcaoStates::from_block_rows(rows))
            .transpose()?;

        let species_q = find_block(cells, "SPECIES_Q")
            .ok()
            .map(|rows| SpeciesQ::from_block_rows(rows))
            .transpose()?;

        let hubbard_u = find_block(cells, "HUBBARD_U")
            .ok()
            .map(|rows| HubbardU::from_block_rows(rows))
            .transpose()?;

        let sedc_custom_params = find_block(cells, "SEDC_CUSTOM_PARAMS")
            .ok()
            .map(|rows| SedcCustomParams::from_block_rows(rows))
            .transpose()?;

        let phonon_kpoint_list = find_block_any(cells, &["PHONON_KPOINT_LIST", "PHONON_KPOINTS_LIST"])
            .ok()
            .map(|rows| PhononKpointList::from_block_rows(rows))
            .transpose()?;

        let phonon_kpoint_path = find_block_any(cells, &["PHONON_KPOINT_PATH", "PHONON_KPOINTS_PATH"])
            .ok()
            .map(|rows| PhononKpointPath::from_block_rows(rows))
            .transpose()?;

        let phonon_kpoints_mp_grid = PhononKpointsMpGrid::from_cells(cells)?;
        let phonon_kpoints_mp_spacing = PhononKpointsMpSpacing::from_cells(cells)?;
        let phonon_kpoints_mp_offset = PhononKpointsMpOffset::from_cells(cells)?;

        let phonon_fine_kpoint_path = find_block_any(
            cells,
            &["PHONON_FINE_KPOINT_PATH", "PHONON_FINE_KPOINTS_PATH"],
        )
        .ok()
        .map(|rows| PhononFineKpointPath::from_block_rows(rows))
        .transpose()?;

        let phonon_fine_kpoint_path_spacing = PhononFineKpointPathSpacing::from_cells(cells)?;
        let phonon_fine_kpoints_mp_grid = PhononFineKpointsMpGrid::from_cells(cells)?;
        let phonon_fine_kpoints_mp_spacing = PhononFineKpointsMpSpacing::from_cells(cells)?;
        let phonon_fine_kpoints_mp_offset = PhononFineKpointsMpOffset::from_cells(cells)?;

        let phonon_gamma_directions = find_block(cells, "PHONON_GAMMA_DIRECTIONS")
            .ok()
            .map(|rows| PhononGammaDirections::from_block_rows(rows))
            .transpose()?;

        let phonon_fine_kpoint_list = find_block_any(cells, &["PHONON_FINE_KPOINT_LIST", "PHONON_FINE_KPOINTS_LIST"])
            .ok()
            .map(|rows| PhononFineKpointList::from_block_rows(rows))
            .transpose()?;

        let phonon_supercell_matrix = find_block(cells, "PHONON_SUPERCELL_MATRIX")
            .ok()
            .map(|rows| PhononSupercellMatrix::from_block_rows(rows))
            .transpose()?;

        let supercell_kpoint_list = find_block_any(cells, &["SUPERCELL_KPOINT_LIST", "SUPERCELL_KPOINTS_LIST"])
            .ok()
            .map(|rows| SupercellKpointListCastep::from_block_rows(rows))
            .transpose()?;

        let ionic_velocities = find_block(cells, "IONIC_VELOCITIES")
            .ok()
            .map(|rows| IonicVelocities::from_block_rows(rows))
            .transpose()?;

        CellDocument::builder()
            .lattice(lattice)
            .positions(positions)
            .maybe_kpoints_list(kpoints_list)
            .maybe_bs_kpoint_path(bs_kpoint_path)
            .maybe_bs_kpoints_list(bs_kpoints_list)
            .maybe_optics_kpoints_list(optics_kpoints_list)
            .maybe_magres_kpoints_list(magres_kpoints_list)
            .maybe_bs_kpoint_path_spacing(bs_kpoint_path_spacing)
            .maybe_kpoints_mp_grid(kpoints_mp_grid)
            .maybe_kpoints_mp_spacing(kpoints_mp_spacing)
            .maybe_kpoints_mp_offset(kpoints_mp_offset)
            .maybe_spectral_kpoint_path(spectral_kpoint_path)
            .maybe_spectral_kpoints_list(spectral_kpoints_list)
            .maybe_spectral_kpoint_path_spacing(spectral_kpoint_path_spacing)
            .maybe_spectral_kpoints_mp_grid(spectral_kpoints_mp_grid)
            .maybe_spectral_kpoints_mp_spacing(spectral_kpoints_mp_spacing)
            .maybe_spectral_kpoints_mp_offset(spectral_kpoints_mp_offset)
            .maybe_symmetry_ops(symmetry_ops)
            .maybe_symmetry_tol(symmetry_tol)
            .maybe_symmetry_generate(symmetry_generate)
            .maybe_fix_com(fix_com)
            .maybe_ionic_constraints(ionic_constraints)
            .maybe_nonlinear_constraints(nonlinear_constraints)
            .maybe_fix_all_ions(fix_all_ions)
            .maybe_fix_all_cell(fix_all_cell)
            .maybe_cell_constraints(cell_constraints)
            .maybe_fix_vol(fix_vol)
            .maybe_external_efield(external_efield)
            .maybe_external_pressure(external_pressure)
            .maybe_species_mass(species_mass)
            .maybe_species_pot(species_pot)
            .maybe_species_lcao_states(species_lcao_states)
            .maybe_species_q(species_q)
            .maybe_hubbard_u(hubbard_u)
            .maybe_sedc_custom_params(sedc_custom_params)
            .maybe_phonon_kpoint_list(phonon_kpoint_list)
            .maybe_phonon_kpoint_path(phonon_kpoint_path)
            .maybe_phonon_kpoints_mp_grid(phonon_kpoints_mp_grid)
            .maybe_phonon_kpoints_mp_spacing(phonon_kpoints_mp_spacing)
            .maybe_phonon_kpoints_mp_offset(phonon_kpoints_mp_offset)
            .maybe_phonon_fine_kpoint_path(phonon_fine_kpoint_path)
            .maybe_phonon_fine_kpoint_path_spacing(phonon_fine_kpoint_path_spacing)
            .maybe_phonon_fine_kpoints_mp_grid(phonon_fine_kpoints_mp_grid)
            .maybe_phonon_fine_kpoints_mp_spacing(phonon_fine_kpoints_mp_spacing)
            .maybe_phonon_fine_kpoints_mp_offset(phonon_fine_kpoints_mp_offset)
            .maybe_phonon_gamma_directions(phonon_gamma_directions)
            .maybe_phonon_fine_kpoint_list(phonon_fine_kpoint_list)
            .maybe_phonon_supercell_matrix(phonon_supercell_matrix)
            .maybe_supercell_kpoint_list(supercell_kpoint_list)
            .maybe_ionic_velocities(ionic_velocities)
            .build()
    }
}

impl ToCellFile for CellDocument {
    /// Serialize this document to a vector of [`Cell`] tokens.
    ///
    /// Converts the structured document back to the token representation used by
    /// [`castep_cell_fmt`]. The tokens can then be formatted to text with
    /// [`castep_cell_fmt::format`].
    ///
    /// # Block Order
    ///
    /// Blocks are emitted in a standard order:
    /// 1. Lattice and positions (required)
    /// 2. K-point sampling blocks
    /// 3. Constraints and flags
    /// 4. External fields
    /// 5. Species properties
    /// 6. Phonon calculation blocks
    /// 7. Dynamics (ionic velocities)
    ///
    /// Optional blocks that are `None` are omitted from the output.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use castep_cell_io::CellDocument;
    /// use castep_cell_fmt::{ToCellFile, format::to_string_many_spaced};
    ///
    /// // Assuming you have a CellDocument instance
    /// let doc = todo!(); // Your CellDocument instance
    /// let cells = doc.to_cell_file();
    /// let output = to_string_many_spaced(&cells);
    /// ```
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = vec![self.lattice.to_cell(), self.positions.to_cell()];

        if let Some(kp) = &self.kpoints_list {
            cells.push(kp.to_cell());
        }
        if let Some(sp) = &self.spectral_kpoint_path {
            cells.push(sp.to_cell());
        }
        if let Some(sl) = &self.spectral_kpoints_list {
            cells.push(sl.to_cell());
        }
        if let Some(sps) = &self.spectral_kpoint_path_spacing {
            cells.push(sps.to_cell());
        }
        if let Some(smg) = &self.spectral_kpoints_mp_grid {
            cells.push(smg.to_cell());
        }
        if let Some(sms) = &self.spectral_kpoints_mp_spacing {
            cells.push(sms.to_cell());
        }
        if let Some(smo) = &self.spectral_kpoints_mp_offset {
            cells.push(smo.to_cell());
        }
        if let Some(bp) = &self.bs_kpoint_path {
            cells.push(bp.to_cell());
        }
        if let Some(bk) = &self.bs_kpoints_list {
            cells.push(bk.to_cell());
        }
        if let Some(ok) = &self.optics_kpoints_list {
            cells.push(ok.to_cell());
        }
        if let Some(mk) = &self.magres_kpoints_list {
            cells.push(mk.to_cell());
        }
        if let Some(kmg) = &self.kpoints_mp_grid {
            cells.push(kmg.to_cell());
        }
        if let Some(kms) = &self.kpoints_mp_spacing {
            cells.push(kms.to_cell());
        }
        if let Some(kmo) = &self.kpoints_mp_offset {
            cells.push(kmo.to_cell());
        }
        if let Some(bps) = &self.bs_kpoint_path_spacing {
            cells.push(bps.to_cell());
        }
        if let Some(sym) = &self.symmetry_ops {
            cells.push(sym.to_cell());
        }
        if let Some(st) = &self.symmetry_tol {
            cells.push(st.to_cell());
        }
        if let Some(_sg) = &self.symmetry_generate {
            cells.push(Cell::Flag("SYMMETRY_GENERATE"));
        }
        if let Some(_fc) = &self.fix_com {
            cells.push(Cell::Flag("FIX_COM"));
        }
        if let Some(ic) = &self.ionic_constraints {
            cells.push(ic.to_cell());
        }
        if let Some(nc) = &self.nonlinear_constraints {
            cells.push(nc.to_cell());
        }
        if let Some(_fi) = &self.fix_all_ions {
            cells.push(Cell::Flag("FIX_ALL_IONS"));
        }
        if let Some(_fc) = &self.fix_all_cell {
            cells.push(Cell::Flag("FIX_ALL_CELL"));
        }
        if let Some(fv) = &self.fix_vol {
            cells.push(fv.to_cell());
        }
        if let Some(cc) = &self.cell_constraints {
            cells.push(cc.to_cell());
        }
        if let Some(ef) = &self.external_efield {
            cells.push(ef.to_cell());
        }
        if let Some(ep) = &self.external_pressure {
            cells.push(ep.to_cell());
        }
        if let Some(sm) = &self.species_mass {
            cells.push(sm.to_cell());
        }
        if let Some(sp) = &self.species_pot {
            cells.push(sp.to_cell());
        }
        if let Some(sl) = &self.species_lcao_states {
            cells.push(sl.to_cell());
        }
        if let Some(sq) = &self.species_q {
            cells.push(sq.to_cell());
        }
        if let Some(hu) = &self.hubbard_u {
            cells.push(hu.to_cell());
        }
        if let Some(sc) = &self.sedc_custom_params {
            cells.push(sc.to_cell());
        }
        if let Some(pk) = &self.phonon_kpoint_list {
            cells.push(pk.to_cell());
        }
        if let Some(pp) = &self.phonon_kpoint_path {
            cells.push(pp.to_cell());
        }
        if let Some(pmg) = &self.phonon_kpoints_mp_grid {
            cells.push(pmg.to_cell());
        }
        if let Some(pms) = &self.phonon_kpoints_mp_spacing {
            cells.push(pms.to_cell());
        }
        if let Some(pmo) = &self.phonon_kpoints_mp_offset {
            cells.push(pmo.to_cell());
        }
        if let Some(pfp) = &self.phonon_fine_kpoint_path {
            cells.push(pfp.to_cell());
        }
        if let Some(pfps) = &self.phonon_fine_kpoint_path_spacing {
            cells.push(pfps.to_cell());
        }
        if let Some(pfmg) = &self.phonon_fine_kpoints_mp_grid {
            cells.push(pfmg.to_cell());
        }
        if let Some(pfms) = &self.phonon_fine_kpoints_mp_spacing {
            cells.push(pfms.to_cell());
        }
        if let Some(pfmo) = &self.phonon_fine_kpoints_mp_offset {
            cells.push(pfmo.to_cell());
        }
        if let Some(pg) = &self.phonon_gamma_directions {
            cells.push(pg.to_cell());
        }
        if let Some(pf) = &self.phonon_fine_kpoint_list {
            cells.push(pf.to_cell());
        }
        if let Some(pm) = &self.phonon_supercell_matrix {
            cells.push(pm.to_cell());
        }
        if let Some(sk) = &self.supercell_kpoint_list {
            cells.push(sk.to_cell());
        }
        if let Some(iv) = &self.ionic_velocities {
            cells.push(iv.to_cell());
        }

        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::species::Species;
    use crate::cell::positions::PositionFracEntry;
    use crate::cell::bz_sampling_kpoints::{Kpoint, SpectralKpointPathEntry, BsKpointPathEntry};
    use crate::cell::phonon::{PhononKpointPathEntry, PhononKpointListEntry};
    use crate::cell::symmetry::SymmetryOp;

    #[test]
    #[ignore]
    fn test_parse_mg2sio4_cell() {
        // TODO: Add test fixture file at tests/fixtures/Mg2SiO4_Cr_1.cell
        let input = "";
        let doc = castep_cell_fmt::parse::<CellDocument>(input).expect("Failed to parse");

        assert!(matches!(doc.lattice, Lattice::Cart(_)));
        assert!(matches!(doc.positions, Positions::Frac(_)));
        assert!(doc.kpoints_list.is_some());
        assert!(doc.symmetry_ops.is_some());
        assert!(doc.fix_com.is_some());
        assert!(doc.ionic_constraints.is_some());
        assert!(doc.external_efield.is_some());
        assert!(doc.species_mass.is_some());
        assert!(doc.species_pot.is_some());
        assert!(doc.species_lcao_states.is_some());
    }

    fn minimal_lattice() -> Lattice {
        Lattice::Cart(LatticeCart {
            unit: None,
            a: [10.0, 0.0, 0.0],
            b: [0.0, 10.0, 0.0],
            c: [0.0, 0.0, 10.0],
        })
    }

    fn minimal_positions() -> Positions {
        Positions::Frac(PositionsFrac {
            positions: vec![PositionFracEntry {
                species: Species::Symbol("Si".to_string()),
                coord: [0.0, 0.0, 0.0],
                spin: None,
                mixture: None,
            }],
        })
    }

    #[test]
    fn build_rejects_multiple_kpoint_specs() {
        let result = CellDocument::builder()
            .lattice(minimal_lattice())
            .positions(minimal_positions())
            .maybe_kpoints_list(Some(KpointsList::builder()
                .kpts(vec![Kpoint::builder().coord([0.0, 0.0, 0.0]).weight(1.0).build()])
                .build()))
            .maybe_kpoints_mp_grid(Some(KpointsMpGrid([2, 2, 2])))
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn build_rejects_multiple_spectral_specs() {
        let result = CellDocument::builder()
            .lattice(minimal_lattice())
            .positions(minimal_positions())
            .maybe_spectral_kpoint_path(Some(SpectralKpointPath::builder()
                .points(vec![SpectralKpointPathEntry { coord: [0.0, 0.0, 0.0] }])
                .build()))
            .maybe_spectral_kpoints_mp_grid(Some(SpectralKpointsMpGrid([2, 2, 2])))
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn build_rejects_spectral_and_bs_duplication() {
        let result = CellDocument::builder()
            .lattice(minimal_lattice())
            .positions(minimal_positions())
            .maybe_spectral_kpoint_path(Some(SpectralKpointPath::builder()
                .points(vec![SpectralKpointPathEntry { coord: [0.0, 0.0, 0.0] }])
                .build()))
            .maybe_bs_kpoint_path(Some(BsKpointPath::builder()
                .points(vec![BsKpointPathEntry { coord: [0.0, 0.0, 0.0] }])
                .build()))
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn build_rejects_multiple_phonon_specs() {
        let result = CellDocument::builder()
            .lattice(minimal_lattice())
            .positions(minimal_positions())
            .maybe_phonon_kpoint_path(Some(PhononKpointPath {
                points: vec![PhononKpointPathEntry { coord: [0.0, 0.0, 0.0] }],
            }))
            .maybe_phonon_kpoint_list(Some(PhononKpointList::builder()
                .kpoints(vec![PhononKpointListEntry { coord: [0.0, 0.0, 0.0], weight: 1.0 }])
                .build()))
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn build_rejects_symmetry_generate_and_ops() {
        let result = CellDocument::builder()
            .lattice(minimal_lattice())
            .positions(minimal_positions())
            .maybe_symmetry_generate(Some(SymmetryGenerate))
            .maybe_symmetry_ops(Some(SymmetryOps::builder()
                .ops(vec![SymmetryOp::builder()
                    .rotation([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
                    .translation([0.0, 0.0, 0.0])
                    .build()])
                .build()))
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn build_allows_mp_offset_with_grid() {
        let result = CellDocument::builder()
            .lattice(minimal_lattice())
            .positions(minimal_positions())
            .maybe_kpoints_mp_grid(Some(KpointsMpGrid([2, 2, 2])))
            .maybe_kpoints_mp_offset(Some(KpointsMpOffset([0.0, 0.0, 0.0])))
            .build();
        assert!(result.is_ok());
    }

    #[test]
    fn build_allows_spectral_mp_offset_with_grid() {
        let result = CellDocument::builder()
            .lattice(minimal_lattice())
            .positions(minimal_positions())
            .maybe_spectral_kpoints_mp_grid(Some(SpectralKpointsMpGrid([2, 2, 2])))
            .maybe_spectral_kpoints_mp_offset(Some(SpectralKpointsMpOffset([0.0, 0.0, 0.0])))
            .build();
        assert!(result.is_ok());
    }

    #[test]
    fn build_allows_single_spec_each_category() {
        let r1 = CellDocument::builder()
            .lattice(minimal_lattice())
            .positions(minimal_positions())
            .maybe_kpoints_mp_grid(Some(KpointsMpGrid([2, 2, 2])))
            .build();
        assert!(r1.is_ok());
        let r2 = CellDocument::builder()
            .lattice(minimal_lattice())
            .positions(minimal_positions())
            .maybe_spectral_kpoint_path(Some(SpectralKpointPath::builder()
                .points(vec![SpectralKpointPathEntry { coord: [0.0, 0.0, 0.0] }])
                .build()))
            .build();
        assert!(r2.is_ok());
        let r3 = CellDocument::builder()
            .lattice(minimal_lattice())
            .positions(minimal_positions())
            .maybe_phonon_kpoint_path(Some(PhononKpointPath {
                points: vec![PhononKpointPathEntry { coord: [0.0, 0.0, 0.0] }],
            }))
            .build();
        assert!(r3.is_ok());
        let r4 = CellDocument::builder()
            .lattice(minimal_lattice())
            .positions(minimal_positions())
            .maybe_symmetry_ops(Some(SymmetryOps::builder()
                .ops(vec![SymmetryOp::builder()
                    .rotation([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
                    .translation([0.0, 0.0, 0.0])
                    .build()])
                .build()))
            .build();
        assert!(r4.is_ok());
    }

    #[test]
    fn build_rejects_all_three_kpoint_specs() {
        let result = CellDocument::builder()
            .lattice(minimal_lattice())
            .positions(minimal_positions())
            .maybe_kpoints_list(Some(KpointsList::builder()
                .kpts(vec![Kpoint::builder().coord([0.0, 0.0, 0.0]).weight(1.0).build()])
                .build()))
            .maybe_kpoints_mp_grid(Some(KpointsMpGrid([2, 2, 2])))
            .maybe_kpoints_mp_spacing(Some(KpointsMpSpacing { value: 0.05, unit: None }))
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn build_allows_empty_document() {
        let result = CellDocument::builder()
            .lattice(minimal_lattice())
            .positions(minimal_positions())
            .build();
        assert!(result.is_ok());
    }
}
