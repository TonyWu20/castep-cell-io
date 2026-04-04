use crate::cell::species::Species;
use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, FromCellValue, FromBlock, CResult, Error, query::value_as_str, query::value_as_i32};

/// Represents a specific atom site, including its species, index within that species,
/// and the periodic image (Miller indices) it occupies.
#[derive(Debug, Clone, PartialEq)]
pub struct AtomSite {
    /// The species of the atom (e.g., "H", "O", "C").
    pub species: Species,
    /// The ion number within the species (1-based index).
    pub ion_number: u32,
    /// The periodic image indices [N1, N2, N3] (Miller indices).
    pub image_indices: [i32; 3],
}

impl AtomSite {
    pub fn new(species: Species, ion_number: u32, image_indices: [i32; 3]) -> Self {
        Self {
            species,
            ion_number,
            image_indices,
        }
    }
}

impl ToCellValue for AtomSite {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(
            vec![
                self.species.to_cell_value(),
                CellValue::UInt(self.ion_number),
            ]
            .into_iter()
            .chain(self.image_indices.into_iter().map(CellValue::Int))
            .collect(),
        )
    }
}

/// Represents the type of internal coordinate constraint.
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintType {
    Distance,
    Bend,
    Torsion,
}

impl FromCellValue for ConstraintType {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        let s = value_as_str(value)?.to_ascii_lowercase();
        match s.trim() {
            "distance" => Ok(ConstraintType::Distance),
            "bend" => Ok(ConstraintType::Bend),
            "torsion" => Ok(ConstraintType::Torsion),
            other => Err(Error::Message(
                format!("unknown ConstraintType: {other}"),
            )),
        }
    }
}

impl ToCellValue for ConstraintType {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                ConstraintType::Distance => "distance",
                ConstraintType::Bend => "bend",
                ConstraintType::Torsion => "torsion",
            }
            .to_string(),
        )
    }
}

/// Represents a single nonlinear constraint entry within the NONLINEAR_CONSTRAINTS block.
#[derive(Debug, Clone, PartialEq)]
pub struct NonlinearConstraint {
    /// The type of the constraint (distance, bend, torsion).
    pub constraint_type: ConstraintType,
    /// The list of atom sites involved in the constraint.
    /// - 2 atoms for Distance
    /// - 3 atoms for Bend
    /// - 4 atoms for Torsion
    pub atom_sites: Vec<AtomSite>,
}

impl FromCellValue for NonlinearConstraint {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() >= 11 => {
                let constraint_type = ConstraintType::from_cell_value(&arr[0])?;
                let mut atom_sites = Vec::new();
                let mut idx = 1;

                let num_atoms = match constraint_type {
                    ConstraintType::Distance => 2,
                    ConstraintType::Bend => 3,
                    ConstraintType::Torsion => 4,
                };

                for _ in 0..num_atoms {
                    if idx + 4 > arr.len() {
                        return Err(Error::Message(
                            "insufficient elements for atom site".into(),
                        ));
                    }
                    let species = Species::from_cell_value(&arr[idx])?;
                    let ion_number = castep_cell_io::query::value_as_u32(&arr[idx + 1])?;
                    let image_indices = [
                        value_as_i32(&arr[idx + 2])?,
                        value_as_i32(&arr[idx + 3])?,
                        value_as_i32(&arr[idx + 4])?,
                    ];
                    atom_sites.push(AtomSite::new(species, ion_number, image_indices));
                    idx += 5;
                }

                Ok(NonlinearConstraint {
                    constraint_type,
                    atom_sites,
                })
            }
            _ => Err(Error::Message(
                "NonlinearConstraint must be an array of at least 11 elements".into(),
            )),
        }
    }
}

impl ToCellValue for NonlinearConstraint {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(
            [self.constraint_type.to_cell_value()]
                .to_vec()
                .into_iter()
                .chain(
                    self.atom_sites
                        .iter()
                        .flat_map(|atom_site| atom_site.to_cell_value().as_array().unwrap().clone())
                        .collect::<Vec<CellValue>>(),
                )
                .collect(),
        )
    }
}

/// Represents the NONLINEAR_CONSTRAINTS block.
///
/// Defines constrained internal coordinates (bonds, angles, torsions) that are
/// held fixed at the values of the initial structure.
/// Format:
/// %BLOCK NONLINEAR_CONSTRAINTS
/// CONSTRAIN_TYPE atom1 atom2 (atom3 atom4)
/// ...
/// %ENDBLOCK NONLINEAR_CONSTRAINTS
#[derive(Debug, Clone, PartialEq)]
pub struct NonlinearConstraints {
    /// The list of nonlinear constraint entries.
    pub constraints: Vec<NonlinearConstraint>,
}

impl FromBlock for NonlinearConstraints {
    const BLOCK_NAME: &'static str = "NONLINEAR_CONSTRAINTS";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        let constraints = rows
            .iter()
            .map(NonlinearConstraint::from_cell_value)
            .collect::<CResult<Vec<_>>>()?;
        Ok(NonlinearConstraints { constraints })
    }
}

impl ToCell for NonlinearConstraints {
    fn to_cell(&self) -> Cell {
        Cell::Block(
            "NONLINEAR_CONSTRAINTS",
            self.constraints
                .iter()
                .map(|constraint| constraint.to_cell_value())
                .collect::<Vec<CellValue>>(),
        )
    }
}


