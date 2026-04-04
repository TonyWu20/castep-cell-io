#![allow(dead_code)]
use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, FromCellValue, FromBlock, CResult, query::value_as_f64, query::value_as_str};

use crate::cell::species::Species;

/// Represents a single atom entry within the POSITIONS_FRAC block.
///
/// Consists of the element symbol/number, fractional coordinates, and an optional spin.
/// Format: <element> <x> <y> <z> [SPIN <value>]
#[derive(Debug, Clone, PartialEq)]
pub struct PositionFracEntry {
    /// The chemical element symbol (e.g., "Fe") or atomic number as a string (e.g., "26").
    pub species: Species,
    /// Fractional coordinates [x, y, z].
    pub coord: [f64; 3],
    /// Optional initial spin polarization for the atom.
    pub spin: Option<f64>,
}

impl FromCellValue for PositionFracEntry {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() >= 4 => {
                let species = Species::from_cell_value(&arr[0])?;
                let coord = [
                    value_as_f64(&arr[1])?,
                    value_as_f64(&arr[2])?,
                    value_as_f64(&arr[3])?,
                ];
                let spin = if arr.len() >= 6 {
                    let keyword = value_as_str(&arr[4])?.to_ascii_uppercase();
                    if keyword == "SPIN" || keyword == "SPIN=" {
                        Some(value_as_f64(&arr[5])?)
                    } else {
                        None
                    }
                } else {
                    None
                };
                Ok(PositionFracEntry { species, coord, spin })
            }
            _ => Err(castep_cell_io::Error::Message(
                "PositionFracEntry must be an array of at least 4 elements".into(),
            )),
        }
    }
}

impl ToCellValue for PositionFracEntry {
    /// Converts the entry into a `CellValue::Array` representing one line of the block.
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(
            [self.species.to_cell_value()]
                .into_iter()
                .chain(self.coord.into_iter().map(CellValue::Float))
                .chain(
                    self.spin
                        .as_ref()
                        .map(|spin| {
                            [
                                CellValue::String("SPIN=".to_string()),
                                CellValue::Float(*spin),
                            ]
                            .to_vec()
                        })
                        .unwrap_or([CellValue::Null].to_vec()),
                )
                .collect(),
        )
    }
}

/// Represents the POSITIONS_FRAC block.
///
/// Contains a list of atomic positions in fractional coordinates.
/// Format:
/// %BLOCK POSITIONS_FRAC
/// Species1/I1 R1i R1j R1k [SPIN S1]
/// Species2/I2 R2i R2j R2k [SPIN S2]
/// ...
/// %ENDBLOCK POSITIONS_FRAC
#[derive(Debug, Clone, PartialEq)]
pub struct PositionsFrac {
    /// The list of atom entries.
    pub positions: Vec<PositionFracEntry>,
}

impl FromBlock for PositionsFrac {
    const BLOCK_NAME: &'static str = "POSITIONS_FRAC";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        let positions = rows
            .iter()
            .map(PositionFracEntry::from_cell_value)
            .collect::<CResult<Vec<_>>>()?;
        Ok(PositionsFrac { positions })
    }
}

impl ToCell for PositionsFrac {
    /// Converts the block into the intermediate `Cell` representation for serialization.
    fn to_cell(&self) -> Cell {
        Cell::Block(
            "POSITIONS_FRAC", // Block name
            self.positions
                .iter()
                .map(|entry| entry.to_cell_value()) // Convert each entry to CellValue::Array
                .collect::<Vec<CellValue>>(), // Collect into Vec for the block content
        )
    }
}


