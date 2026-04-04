#![allow(dead_code)]
use castep_cell_fmt::{Cell, CellValue, ToCell, FromBlock, CResult, FromCellValue, ToCellValue};

use super::positions_frac::PositionFracEntry;

/// Represents the POSITIONS_FRAC_PRODUCT block.
///
/// Contains a list of atomic positions in fractional coordinates for the product
/// geometry in a transition state search calculation.
/// Format:
/// %BLOCK POSITIONS_FRAC_PRODUCT
/// Species1/I1 R1i R1j R1k [SPIN S1]
/// Species2/I2 R2i R2j R2k [SPIN S2]
/// ...
/// %ENDBLOCK POSITIONS_FRAC_PRODUCT
#[derive(Debug, Clone, PartialEq)]
pub struct PositionsFracProduct {
    /// The list of atom entries.
    pub positions: Vec<PositionFracEntry>,
}

impl FromBlock for PositionsFracProduct {
    const BLOCK_NAME: &'static str = "POSITIONS_FRAC_PRODUCT";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        let positions = rows
            .iter()
            .map(PositionFracEntry::from_cell_value)
            .collect::<CResult<Vec<_>>>()?;
        Ok(PositionsFracProduct { positions })
    }
}

impl ToCell for PositionsFracProduct {
    /// Converts the block into the intermediate `Cell` representation for serialization.
    fn to_cell(&self) -> Cell {
        Cell::Block(
            "POSITIONS_FRAC_PRODUCT",
            self.positions
                .iter()
                .map(|entry| entry.to_cell_value())
                .collect::<Vec<CellValue>>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_fmt::CellValue;
    use crate::cell::species::Species;

    #[test]
    fn test_positions_frac_product_multiple_entries() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Str("Fe"),
                CellValue::Float(0.0),
                CellValue::Float(0.0),
                CellValue::Float(0.0),
            ]),
            CellValue::Array(vec![
                CellValue::Str("O"),
                CellValue::Float(0.5),
                CellValue::Float(0.5),
                CellValue::Float(0.5),
            ]),
        ];
        let result = PositionsFracProduct::from_block_rows(&rows).unwrap();
        assert_eq!(result.positions.len(), 2);
    }

    #[test]
    fn test_positions_frac_product_empty() {
        let result = PositionsFracProduct::from_block_rows(&[]).unwrap();
        assert_eq!(result.positions.len(), 0);
    }

    #[test]
    fn test_positions_frac_product_block_name() {
        assert_eq!(PositionsFracProduct::BLOCK_NAME, "POSITIONS_FRAC_PRODUCT");
    }

    #[test]
    fn test_positions_frac_product_to_cell() {
        let positions = PositionsFracProduct {
            positions: vec![PositionFracEntry {
                species: Species::from_cell_value(&CellValue::Str("Fe")).unwrap(),
                coord: [0.0, 0.0, 0.0],
                spin: None,
                mixture: None,
            }],
        };
        let cell = positions.to_cell();
        match cell {
            Cell::Block(name, values) => {
                assert_eq!(name, "POSITIONS_FRAC_PRODUCT");
                assert_eq!(values.len(), 1);
            }
            _ => panic!("Expected Block"),
        }
    }

    #[test]
    fn test_positions_frac_product_with_spin() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Str("Fe"),
                CellValue::Float(0.25),
                CellValue::Float(0.25),
                CellValue::Float(0.25),
                CellValue::Str("SPIN"),
                CellValue::Float(2.0),
            ]),
        ];
        let result = PositionsFracProduct::from_block_rows(&rows).unwrap();
        assert_eq!(result.positions.len(), 1);
        assert_eq!(result.positions[0].spin, Some(2.0));
    }

    #[test]
    fn test_positions_frac_product_with_mixture() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Str("Al"),
                CellValue::Float(0.25),
                CellValue::Float(0.5),
                CellValue::Float(0.0),
                CellValue::Str("MIXTURE"),
                CellValue::UInt(1),
                CellValue::Float(0.666667),
            ]),
        ];
        let result = PositionsFracProduct::from_block_rows(&rows).unwrap();
        assert_eq!(result.positions.len(), 1);
        assert_eq!(result.positions[0].mixture, Some((1, 0.666667)));
    }
}
