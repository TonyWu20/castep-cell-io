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
        KpointsMpSpacing, MagresKpointsList, OpticsKpointsList,
    },
    constraints::{
        CellConstraints, FixAllCell, FixAllIons, FixCOM, FixVOL, IonicConstraints,
        NonlinearConstraints,
    },
    external_fields::{ExternalEfield, ExternalPressure},
    lattice_param::LatticeCart,
    phonon::{
        PhononFineKpointList, PhononGammaDirections, PhononKpointList, PhononKpointPath,
        PhononSupercellMatrix, SupercellKpointListCastep,
    },
    positions::{PositionsAbs, PositionsFrac},
    species::{HubbardU, SedcCustomParams, SpeciesLcaoStates, SpeciesMass, SpeciesPot, SpeciesQ},
    symmetry::{SymmetryOps, SymmetryTol},
    velocities::IonicVelocities,
};

/// Lattice vector specification for the simulation cell.
///
/// Defines the periodic boundary conditions of the crystal structure.
/// Currently supports Cartesian coordinates; future versions may add
/// fractional or ABC+angles representations.
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
}

impl ToCell for Lattice {
    fn to_cell(&self) -> Cell<'_> {
        match self {
            Lattice::Cart(cart) => cart.to_cell(),
        }
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
#[builder(on(Lattice, into), on(Positions, into))]
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
    /// Spacing for band structure k-point path.
    ///
    /// Corresponds to `BS_KPOINT_PATH_SPACING` in CASTEP.
    pub bs_kpoint_path_spacing: Option<BsKpointPathSpacing>,
    /// Explicit symmetry operations.
    ///
    /// Overrides automatic symmetry detection. Corresponds to `%BLOCK SYMMETRY_OPS`.
    pub symmetry_ops: Option<SymmetryOps>,
    /// Symmetry detection tolerance.
    ///
    /// Corresponds to `SYMMETRY_TOL` in CASTEP.
    pub symmetry_tol: Option<SymmetryTol>,
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
        let lattice_rows = find_block(cells, "LATTICE_CART")?;
        let lattice = Lattice::Cart(LatticeCart::from_block_rows(lattice_rows)?);

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

        let bs_kpoint_path = find_block_any(cells, &["BS_KPOINT_PATH", "BS_KPOINTS_PATH"])
            .ok()
            .map(|rows| BsKpointPath::from_block_rows(rows))
            .transpose()?;

        let bs_kpoints_list = find_block_any(cells, &["BS_KPOINT_LIST", "BS_KPOINTS_LIST"])
            .ok()
            .map(|rows| BSKpointList::from_block_rows(rows))
            .transpose()?;

        let bs_kpoint_path_spacing = BsKpointPathSpacing::from_cells(cells)?;

        let optics_kpoints_list = find_block_any(cells, &["OPTICS_KPOINT_LIST", "OPTICS_KPOINTS_LIST"])
            .ok()
            .map(|rows| OpticsKpointsList::from_block_rows(rows))
            .transpose()?;

        let magres_kpoints_list = find_block_any(cells, &["MAGRES_KPOINT_LIST", "MAGRES_KPOINTS_LIST"])
            .ok()
            .map(|rows| MagresKpointsList::from_block_rows(rows))
            .transpose()?;

        let kpoints_mp_grid = KpointsMpGrid::from_cells(cells)?;
        let kpoints_mp_spacing = KpointsMpSpacing::from_cells(cells)?;

        let symmetry_ops = find_block(cells, "SYMMETRY_OPS")
            .ok()
            .map(|rows| SymmetryOps::from_block_rows(rows))
            .transpose()?;

        let symmetry_tol = SymmetryTol::from_cells(cells)?;

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

        Ok(CellDocument {
            lattice,
            positions,
            kpoints_list,
            bs_kpoint_path,
            bs_kpoints_list,
            optics_kpoints_list,
            magres_kpoints_list,
            bs_kpoint_path_spacing,
            kpoints_mp_grid,
            kpoints_mp_spacing,
            symmetry_ops,
            symmetry_tol,
            fix_com,
            ionic_constraints,
            nonlinear_constraints,
            fix_all_ions,
            fix_all_cell,
            cell_constraints,
            fix_vol,
            external_efield,
            external_pressure,
            species_mass,
            species_pot,
            species_lcao_states,
            species_q,
            hubbard_u,
            sedc_custom_params,
            phonon_kpoint_list,
            phonon_kpoint_path,
            phonon_gamma_directions,
            phonon_fine_kpoint_list,
            phonon_supercell_matrix,
            supercell_kpoint_list,
            ionic_velocities,
        })
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
        if let Some(bps) = &self.bs_kpoint_path_spacing {
            cells.push(bps.to_cell());
        }
        if let Some(sym) = &self.symmetry_ops {
            cells.push(sym.to_cell());
        }
        if let Some(st) = &self.symmetry_tol {
            cells.push(st.to_cell());
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
}
