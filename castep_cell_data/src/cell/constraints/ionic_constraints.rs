use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, FromCellValue, FromBlock, CResult, Error, query::value_as_f64};

use crate::cell::species::Species;

/// Represents a single constraint entry within the IONIC_CONSTRAINTS block.
///
/// # Format:
///   Ic CCC/Ic In Rix Riy Riz
/// Where:
/// - Ic: Constraint number
/// - CCC/Ic: Species (symbol or atomic number)
/// - In: Ion number within the species
/// - Rix, Riy, Riz: Coefficients for x, y, z Cartesian coordinates
#[derive(Debug, Clone, PartialEq)]
pub struct IonicConstraintEntry {
    /// The constraint number.
    pub constraint_number: u32,
    /// The species of the ion.
    pub species: Species,
    /// The ion number within the species (1-based index).
    pub ion_number: u32,
    /// Coefficients for the Cartesian coordinates [x, y, z].
    pub coefficients: [f64; 3],
}

impl FromCellValue for IonicConstraintEntry {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() >= 6 => {
                let constraint_number = castep_cell_io::query::value_as_u32(&arr[0])?;
                let species = Species::from_cell_value(&arr[1])?;
                let ion_number = castep_cell_io::query::value_as_u32(&arr[2])?;
                let coefficients = [
                    value_as_f64(&arr[3])?,
                    value_as_f64(&arr[4])?,
                    value_as_f64(&arr[5])?,
                ];
                Ok(IonicConstraintEntry {
                    constraint_number,
                    species,
                    ion_number,
                    coefficients,
                })
            }
            _ => Err(castep_cell_io::Error::Message(
                "IonicConstraintEntry must be an array of at least 6 elements".into(),
            )),
        }
    }
}

impl ToCellValue for IonicConstraintEntry {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(
            vec![
                CellValue::UInt(self.constraint_number),
                self.species.to_cell_value(),
                CellValue::UInt(self.ion_number),
            ]
            .into_iter()
            .chain(self.coefficients.into_iter().map(CellValue::Float))
            .collect(),
        )
    }
}

/// Represents the IONIC_CONSTRAINTS block.
///
/// Defines linear constraints on changes to Cartesian ionic positions.
/// Format:
/// %BLOCK IONIC_CONSTRAINTS
/// I1 CCC1/I1 In1 R1i R1j R1k
/// I2 CCC2/I2 In2 R2i R2j R2k
/// ...
/// %ENDBLOCK IONIC_CONSTRAINTS
#[derive(Debug, Clone, PartialEq)]
pub struct IonicConstraints {
    /// The list of constraint entries.
    pub constraints: Vec<IonicConstraintEntry>,
}

impl FromBlock for IonicConstraints {
    const BLOCK_NAME: &'static str = "IONIC_CONSTRAINTS";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        let constraints = rows
            .iter()
            .map(IonicConstraintEntry::from_cell_value)
            .collect::<CResult<Vec<_>>>()?;
        Ok(IonicConstraints { constraints })
    }
}

impl ToCell for IonicConstraints {
    fn to_cell(&self) -> Cell {
        Cell::Block(
            "IONIC_CONSTRAINTS",
            self.constraints
                .iter()
                .map(|entry| entry.to_cell_value())
                .collect::<Vec<CellValue>>(),
        )
    }
}


