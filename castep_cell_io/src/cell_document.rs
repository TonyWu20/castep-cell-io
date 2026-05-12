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
//! if let Some(kpoints) = &doc.kpoints.kpoints_list {
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
    CResult, Cell, CellValue, Error, ToCell, ToCellFile,
    parse::{FromBlock, FromCellFile},
    query::find_block,
};

use crate::cell::{
    constraints_params::ConstraintsParams,
    dynamics_params::DynamicsParams,
    external_field_params::ExternalFieldParams,
    kpoints_params::KpointsParams,
    lattice_param::{LatticeABC, LatticeCart},
    optics_magres_params::OpticsMagresParams,
    phonon_params::PhononParams,
    phonon_fine_params::PhononFineParams,
    positions::{PositionsAbs, PositionsFrac},
    species_params::SpeciesParams,
    spectral_params::SpectralParams,
    symmetry_params::SymmetryParams,
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
/// # Optional Blocks (via Cell Document Groups)
///
/// The document organizes optional blocks into logical sub-groups,
/// mirroring the `ParamDocument` pattern:
///
/// - **K-points**: [`kpoints`](Self::kpoints) — SCF k-point sampling
/// - **Spectral**: [`spectral`](Self::spectral) — BS_ and SPECTRAL_ k-point paths
/// - **Optics/Magres**: [`optics_magres`](Self::optics_magres) — optics and magnetic resonance k-points
/// - **Symmetry**: [`symmetry`](Self::symmetry) — symmetry operations and generation
/// - **Constraints**: [`constraints`](Self::constraints) — ionic and cell constraints
/// - **External fields**: [`external_fields`](Self::external_fields) — electric field and pressure
/// - **Species**: [`species`](Self::species) — masses, pseudopotentials, Hubbard U
/// - **Phonon**: [`phonon`](Self::phonon) — coarse phonon k-point settings
/// - **Phonon fine**: [`phonon_fine`](Self::phonon_fine) — fine phonon k-point settings
/// - **Dynamics**: [`dynamics`](Self::dynamics) — ionic velocities for MD
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
    /// SCF k-point sampling parameters.
    ///
    /// Contains KPOINTS_LIST, KPOINTS_MP_GRID, KPOINTS_MP_SPACING, and KPOINTS_MP_OFFSET.
    #[builder(default)]
    pub kpoints: KpointsParams,
    /// Spectral/BS k-point parameters.
    ///
    /// Contains BS_ and SPECTRAL_ prefixed k-point types for band structure calculations.
    #[builder(default)]
    pub spectral: SpectralParams,
    /// Optics and magnetic resonance k-point lists.
    ///
    /// Contains OPTICS_KPOINTS_LIST and MAGRES_KPOINTS_LIST.
    #[builder(default)]
    pub optics_magres: OpticsMagresParams,
    /// Symmetry parameters.
    ///
    /// Contains SYMMETRY_OPS, SYMMETRY_GENERATE, and SYMMETRY_TOL.
    #[builder(default)]
    pub symmetry: SymmetryParams,
    /// Movement constraints for ions and cell.
    ///
    /// Contains FIX_COM, IONIC_CONSTRAINTS, NONLINEAR_CONSTRAINTS,
    /// FIX_ALL_IONS, FIX_ALL_CELL, CELL_CONSTRAINTS, and FIX_VOL.
    #[builder(default)]
    pub constraints: ConstraintsParams,
    /// External field parameters.
    ///
    /// Contains EXTERNAL_EFIELD and EXTERNAL_PRESSURE.
    #[builder(default)]
    pub external_fields: ExternalFieldParams,
    /// Species properties.
    ///
    /// Contains SPECIES_MASS, SPECIES_POT, SPECIES_LCAO_STATES,
    /// SPECIES_Q, HUBBARD_U, and SEDC_CUSTOM_PARAMS.
    #[builder(default)]
    pub species: SpeciesParams,
    /// Phonon (coarse) k-point parameters.
    ///
    /// Contains phonon k-point lists, paths, MP grids, and related settings.
    #[builder(default)]
    pub phonon: PhononParams,
    /// Phonon fine k-point parameters.
    ///
    /// Contains fine phonon k-point paths, lists, and MP grids.
    #[builder(default)]
    pub phonon_fine: PhononFineParams,
    /// Molecular dynamics dynamics parameters.
    ///
    /// Contains IONIC_VELOCITIES for MD restart.
    #[builder(default)]
    pub dynamics: DynamicsParams,
}

impl<S: cell_document_builder::IsComplete> CellDocumentBuilder<S> {
    pub fn build(self) -> CResult<CellDocument> {
        let mut doc = self.build_internal();
        doc.kpoints = doc.kpoints.validate().map_err(|e| Error::Message(e.to_string()))?;
        doc.spectral = doc.spectral.validate().map_err(|e| Error::Message(e.to_string()))?;
        doc.symmetry = doc.symmetry.validate().map_err(|e| Error::Message(e.to_string()))?;
        doc.constraints = doc.constraints.validate().map_err(|e| Error::Message(e.to_string()))?;
        doc.phonon = doc.phonon.validate().map_err(|e| Error::Message(e.to_string()))?;
        doc.phonon_fine = doc.phonon_fine.validate().map_err(|e| Error::Message(e.to_string()))?;
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
            Lattice::Cart(LatticeCart::from_block_rows(find_block(cells, "LATTICE_CART")?)?)
        } else {
            Lattice::Abc(LatticeABC::from_block_rows(find_block(cells, "LATTICE_ABC")?)?)
        };

        let positions = if find_block(cells, "POSITIONS_FRAC").is_ok() {
            Positions::Frac(PositionsFrac::from_block_rows(find_block(cells, "POSITIONS_FRAC")?)?)
        } else {
            Positions::Abs(PositionsAbs::from_block_rows(find_block(cells, "POSITIONS_ABS")?)?)
        };

        Self::builder()
            .lattice(lattice)
            .positions(positions)
            .kpoints(KpointsParams::from_cell_file(cells)?)
            .spectral(SpectralParams::from_cell_file(cells)?)
            .optics_magres(OpticsMagresParams::from_cell_file(cells)?)
            .symmetry(SymmetryParams::from_cell_file(cells)?)
            .constraints(ConstraintsParams::from_cell_file(cells)?)
            .external_fields(ExternalFieldParams::from_cell_file(cells)?)
            .species(SpeciesParams::from_cell_file(cells)?)
            .phonon(PhononParams::from_cell_file(cells)?)
            .phonon_fine(PhononFineParams::from_cell_file(cells)?)
            .dynamics(DynamicsParams::from_cell_file(cells)?)
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
        cells.extend(self.kpoints.to_cell_file());
        cells.extend(self.spectral.to_cell_file());
        cells.extend(self.optics_magres.to_cell_file());
        cells.extend(self.symmetry.to_cell_file());
        cells.extend(self.constraints.to_cell_file());
        cells.extend(self.external_fields.to_cell_file());
        cells.extend(self.species.to_cell_file());
        cells.extend(self.phonon.to_cell_file());
        cells.extend(self.phonon_fine.to_cell_file());
        cells.extend(self.dynamics.to_cell_file());
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::bz_sampling_kpoints::{
        BsKpointPath, BsKpointPathEntry, Kpoint, KpointsList, KpointsMpGrid, KpointsMpOffset,
        KpointsMpSpacing, SpectralKpointPath, SpectralKpointPathEntry, SpectralKpointsMpGrid,
        SpectralKpointsMpOffset,
    };
    use crate::cell::phonon::{PhononKpointList, PhononKpointListEntry, PhononKpointPath, PhononKpointPathEntry};
    use crate::cell::positions::PositionFracEntry;
    use crate::cell::species::Species;
    use crate::cell::symmetry::{SymmetryGenerate, SymmetryOp, SymmetryOps};

    #[test]
    fn test_parse_fe2o3_cell() {
        let input = std::fs::read_to_string("tests/fixtures/Fe2O3.cell").unwrap();
        let doc = castep_cell_fmt::parse::<CellDocument>(&input).expect("Failed to parse Fe2O3.cell");
        assert!(matches!(doc.lattice, Lattice::Cart(_)));
        assert!(doc.kpoints.kpoints_list.is_some());
        assert!(doc.constraints.fix_all_cell.is_some());
        assert!(doc.external_fields.external_pressure.is_some());
        assert!(doc.species.hubbard_u.is_some());
    }

    #[test]
    fn test_parse_zno_lr_cell() {
        let input = std::fs::read_to_string("tests/fixtures/ZnO_LR.cell").unwrap();
        let doc = castep_cell_fmt::parse::<CellDocument>(&input).expect("Failed to parse ZnO_LR.cell");
        assert!(matches!(doc.lattice, Lattice::Cart(_)));
        assert!(doc.kpoints.kpoints_list.is_some());
        assert!(doc.symmetry.symmetry_ops.is_some());
        assert!(doc.constraints.cell_constraints.is_some());
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
            .kpoints(KpointsParams {
                kpoints_list: Some(KpointsList::builder()
                    .kpts(vec![Kpoint::builder().coord([0.0, 0.0, 0.0]).weight(1.0).build()])
                    .build()),
                kpoints_mp_grid: Some(KpointsMpGrid([2, 2, 2])),
                ..Default::default()
            })
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn build_rejects_spectral_and_bs_duplication() {
        let result = CellDocument::builder()
            .lattice(minimal_lattice())
            .positions(minimal_positions())
            .spectral(SpectralParams {
                spectral_kpoint_path: Some(SpectralKpointPath::builder()
                    .points(vec![SpectralKpointPathEntry { coord: [0.0, 0.0, 0.0] }])
                    .build()),
                bs_kpoint_path: Some(BsKpointPath::builder()
                    .points(vec![BsKpointPathEntry { coord: [0.0, 0.0, 0.0] }])
                    .build()),
                ..Default::default()
            })
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn build_rejects_multiple_phonon_specs() {
        let result = CellDocument::builder()
            .lattice(minimal_lattice())
            .positions(minimal_positions())
            .phonon(PhononParams {
                phonon_kpoint_path: Some(PhononKpointPath {
                    points: vec![PhononKpointPathEntry { coord: [0.0, 0.0, 0.0] }],
                }),
                phonon_kpoint_list: Some(PhononKpointList::builder()
                    .kpoints(vec![PhononKpointListEntry { coord: [0.0, 0.0, 0.0], weight: 1.0 }])
                    .build()),
                ..Default::default()
            })
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn build_rejects_symmetry_generate_and_ops() {
        let result = CellDocument::builder()
            .lattice(minimal_lattice())
            .positions(minimal_positions())
            .symmetry(SymmetryParams {
                symmetry_generate: Some(SymmetryGenerate),
                symmetry_ops: Some(SymmetryOps::builder()
                    .ops(vec![SymmetryOp::builder()
                        .rotation([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
                        .translation([0.0, 0.0, 0.0])
                        .build()])
                    .build()),
                ..Default::default()
            })
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn build_allows_mp_offset_with_grid() {
        let result = CellDocument::builder()
            .lattice(minimal_lattice())
            .positions(minimal_positions())
            .kpoints(KpointsParams {
                kpoints_mp_grid: Some(KpointsMpGrid([2, 2, 2])),
                kpoints_mp_offset: Some(KpointsMpOffset([0.0, 0.0, 0.0])),
                ..Default::default()
            })
            .build();
        assert!(result.is_ok());
    }

    #[test]
    fn build_allows_spectral_mp_offset_with_grid() {
        let result = CellDocument::builder()
            .lattice(minimal_lattice())
            .positions(minimal_positions())
            .spectral(SpectralParams {
                spectral_kpoints_mp_grid: Some(SpectralKpointsMpGrid([2, 2, 2])),
                spectral_kpoints_mp_offset: Some(SpectralKpointsMpOffset([0.0, 0.0, 0.0])),
                ..Default::default()
            })
            .build();
        assert!(result.is_ok());
    }

    #[test]
    fn build_allows_single_spec_each_category() {
        let r1 = CellDocument::builder()
            .lattice(minimal_lattice())
            .positions(minimal_positions())
            .kpoints(KpointsParams {
                kpoints_mp_grid: Some(KpointsMpGrid([2, 2, 2])),
                ..Default::default()
            })
            .build();
        assert!(r1.is_ok());
        let r2 = CellDocument::builder()
            .lattice(minimal_lattice())
            .positions(minimal_positions())
            .spectral(SpectralParams {
                spectral_kpoint_path: Some(SpectralKpointPath::builder()
                    .points(vec![SpectralKpointPathEntry { coord: [0.0, 0.0, 0.0] }])
                    .build()),
                ..Default::default()
            })
            .build();
        assert!(r2.is_ok());
        let r3 = CellDocument::builder()
            .lattice(minimal_lattice())
            .positions(minimal_positions())
            .phonon(PhononParams {
                phonon_kpoint_path: Some(PhononKpointPath {
                    points: vec![PhononKpointPathEntry { coord: [0.0, 0.0, 0.0] }],
                }),
                ..Default::default()
            })
            .build();
        assert!(r3.is_ok());
        let r4 = CellDocument::builder()
            .lattice(minimal_lattice())
            .positions(minimal_positions())
            .symmetry(SymmetryParams {
                symmetry_ops: Some(SymmetryOps::builder()
                    .ops(vec![SymmetryOp::builder()
                        .rotation([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
                        .translation([0.0, 0.0, 0.0])
                        .build()])
                    .build()),
                ..Default::default()
            })
            .build();
        assert!(r4.is_ok());
    }

    #[test]
    fn build_rejects_all_three_kpoint_specs() {
        let result = CellDocument::builder()
            .lattice(minimal_lattice())
            .positions(minimal_positions())
            .kpoints(KpointsParams {
                kpoints_list: Some(KpointsList::builder()
                    .kpts(vec![Kpoint::builder().coord([0.0, 0.0, 0.0]).weight(1.0).build()])
                    .build()),
                kpoints_mp_grid: Some(KpointsMpGrid([2, 2, 2])),
                kpoints_mp_spacing: Some(KpointsMpSpacing { value: 0.05, unit: None }),
                ..Default::default()
            })
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
