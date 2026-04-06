use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, FromCellValue, FromBlock, CResult, query::value_as_f64};

/// Represents a single direction entry within the PHONON_GAMMA_DIRECTIONS block.
///
/// Each entry contains three components of a direction vector for approaching the gamma point.
/// Format: <x> <y> <z>
#[derive(Debug, Clone, Copy, PartialEq, bon::Builder)]
pub struct PhononGammaDirectionEntry {
    /// Direction vector components [x, y, z].
    pub direction: [f64; 3],
}

impl FromCellValue for PhononGammaDirectionEntry {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() == 3 => {
                let direction = [
                    value_as_f64(&arr[0])?,
                    value_as_f64(&arr[1])?,
                    value_as_f64(&arr[2])?,
                ];
                Ok(PhononGammaDirectionEntry { direction })
            }
            _ => Err(castep_cell_fmt::Error::Message(
                "PhononGammaDirectionEntry must be an array of 3 floats".into(),
            )),
        }
    }
}

impl ToCellValue for PhononGammaDirectionEntry {
    /// Converts the entry into a `CellValue::Array` representing one line of the block.
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Array(
            self.direction
                .into_iter()
                .map(CellValue::Float)
                .collect(),
        )
    }
}

/// Represents the PHONON_GAMMA_DIRECTIONS block.
///
/// Contains a list of directions of approach to the gamma point, used to evaluate the non-analytical
/// part of the LO/TO phonon splitting.
/// Format:
/// %BLOCK PHONON_GAMMA_DIRECTIONS
/// R1i R1j R1k
/// R2i R2j R2k
/// ...
/// %ENDBLOCK PHONON_GAMMA_DIRECTIONS
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct PhononGammaDirections {
    /// The list of direction entries.
    #[builder(default)]
    pub directions: Vec<PhononGammaDirectionEntry>,
}

impl FromBlock for PhononGammaDirections {
    const BLOCK_NAME: &'static str = "PHONON_GAMMA_DIRECTIONS";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        let directions = rows
            .iter()
            .map(PhononGammaDirectionEntry::from_cell_value)
            .collect::<CResult<Vec<_>>>()?;
        Ok(PhononGammaDirections { directions })
    }
}

impl ToCell for PhononGammaDirections {
    /// Converts the block into the intermediate `Cell` representation for serialization.
    fn to_cell(&self) -> Cell<'_> {
        Cell::Block(
            "PHONON_GAMMA_DIRECTIONS",
            self.directions
                .iter()
                .map(|entry| entry.to_cell_value())
                .collect::<Vec<CellValue>>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phonon_gamma_direction_entry_from_cell_value() {
        let val = CellValue::Array(vec![
            CellValue::Float(1.0),
            CellValue::Float(0.0),
            CellValue::Float(0.0),
        ]);
        let entry = PhononGammaDirectionEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.direction[0], 1.0);
        assert_eq!(entry.direction[1], 0.0);
        assert_eq!(entry.direction[2], 0.0);
    }

    #[test]
    fn test_phonon_gamma_direction_entry_insufficient_elements() {
        let val = CellValue::Array(vec![
            CellValue::Float(1.0),
            CellValue::Float(0.0),
        ]);
        assert!(PhononGammaDirectionEntry::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_phonon_gamma_direction_entry_too_many_elements() {
        let val = CellValue::Array(vec![
            CellValue::Float(1.0),
            CellValue::Float(0.0),
            CellValue::Float(0.0),
            CellValue::Float(1.0),
        ]);
        assert!(PhononGammaDirectionEntry::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_phonon_gamma_direction_entry_to_cell_value() {
        let entry = PhononGammaDirectionEntry {
            direction: [1.0, 0.0, 0.0],
        };
        let val = entry.to_cell_value();
        match val {
            CellValue::Array(arr) => {
                assert_eq!(arr.len(), 3);
                assert_eq!(arr[0], CellValue::Float(1.0));
            }
            _ => panic!("Expected Array"),
        }
    }

    #[test]
    fn test_phonon_gamma_directions_multiple_entries() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Float(1.0),
                CellValue::Float(0.0),
                CellValue::Float(0.0),
            ]),
            CellValue::Array(vec![
                CellValue::Float(0.0),
                CellValue::Float(0.0),
                CellValue::Float(1.0),
            ]),
        ];
        let result = PhononGammaDirections::from_block_rows(&rows).unwrap();
        assert_eq!(result.directions.len(), 2);
    }

    #[test]
    fn test_phonon_gamma_directions_empty() {
        let result = PhononGammaDirections::from_block_rows(&[]).unwrap();
        assert_eq!(result.directions.len(), 0);
    }

    #[test]
    fn test_phonon_gamma_directions_block_name() {
        assert_eq!(PhononGammaDirections::BLOCK_NAME, "PHONON_GAMMA_DIRECTIONS");
    }

    #[test]
    fn test_phonon_gamma_directions_to_cell() {
        let directions = PhononGammaDirections {
            directions: vec![PhononGammaDirectionEntry {
                direction: [1.0, 0.0, 0.0],
            }],
        };
        let cell = directions.to_cell();
        match cell {
            Cell::Block(name, values) => {
                assert_eq!(name, "PHONON_GAMMA_DIRECTIONS");
                assert_eq!(values.len(), 1);
            }
            _ => panic!("Expected Block"),
        }
    }
}
