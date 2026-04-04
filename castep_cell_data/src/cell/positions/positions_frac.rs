#![allow(dead_code)]
use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, FromCellValue, FromBlock, CResult, query::value_as_f64, query::value_as_str, query::value_as_string};

use crate::cell::species::Species;

/// Represents a single atom entry within the POSITIONS_FRAC block.
///
/// Consists of the element symbol/number, fractional coordinates, and optional spin/mixture qualifiers.
/// Format: <element> <x> <y> <z> [SPIN <value>] [MIXTURE <index> <weight>]
#[derive(Debug, Clone, PartialEq)]
pub struct PositionFracEntry {
    /// The chemical element symbol (e.g., "Fe") or atomic number as a string (e.g., "26").
    pub species: Species,
    /// Fractional coordinates [x, y, z].
    pub coord: [f64; 3],
    /// Optional initial spin polarization for the atom.
    pub spin: Option<f64>,
    /// Optional mixture specification: (index, weight) for disordered systems.
    pub mixture: Option<(u32, f64)>,
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

                let mut spin = None;
                let mut mixture = None;
                let mut idx = 4;

                // Parse optional qualifiers (SPIN, MIXTURE) in any order
                while idx < arr.len() {
                    let keyword = value_as_string(&arr[idx])?.to_ascii_uppercase();

                    if keyword == "SPIN" || keyword == "SPIN=" {
                        if idx + 1 < arr.len() {
                            spin = Some(value_as_f64(&arr[idx + 1])?);
                            idx += 2;
                        } else {
                            return Err(castep_cell_io::Error::Message(
                                "SPIN qualifier requires a value".into(),
                            ));
                        }
                    } else if keyword == "MIXTURE" || keyword == "MIXTURE=" {
                        if idx + 2 < arr.len() {
                            let mix_idx = match &arr[idx + 1] {
                                CellValue::UInt(u) => *u,
                                CellValue::Int(i) if *i >= 0 => *i as u32,
                                _ => {
                                    return Err(castep_cell_io::Error::Message(
                                        "MIXTURE index must be a positive integer".into(),
                                    ))
                                }
                            };
                            let mix_weight = value_as_f64(&arr[idx + 2])?;
                            mixture = Some((mix_idx, mix_weight));
                            idx += 3;
                        } else {
                            return Err(castep_cell_io::Error::Message(
                                "MIXTURE qualifier requires index and weight".into(),
                            ));
                        }
                    } else {
                        return Err(castep_cell_io::Error::Message(
                            format!("unknown qualifier: {keyword}"),
                        ));
                    }
                }

                Ok(PositionFracEntry {
                    species,
                    coord,
                    spin,
                    mixture,
                })
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
        let mut arr = vec![self.species.to_cell_value()];
        arr.extend(self.coord.into_iter().map(CellValue::Float));

        // Append SPIN qualifier if present
        if let Some(spin) = self.spin {
            arr.push(CellValue::String("SPIN=".to_string()));
            arr.push(CellValue::Float(spin));
        }

        // Append MIXTURE qualifier if present
        if let Some((mix_idx, mix_weight)) = self.mixture {
            arr.push(CellValue::String("MIXTURE=".to_string()));
            arr.push(CellValue::UInt(mix_idx));
            arr.push(CellValue::Float(mix_weight));
        }

        CellValue::Array(arr)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_frac_entry_without_spin() {
        let val = CellValue::Array(vec![
            CellValue::Str("Fe"),
            CellValue::Float(0.0),
            CellValue::Float(0.0),
            CellValue::Float(0.0),
        ]);
        let entry = PositionFracEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.coord, [0.0, 0.0, 0.0]);
        assert!(entry.spin.is_none());
    }

    #[test]
    fn test_position_frac_entry_with_spin() {
        let val = CellValue::Array(vec![
            CellValue::Str("Fe"),
            CellValue::Float(0.5),
            CellValue::Float(0.5),
            CellValue::Float(0.5),
            CellValue::Str("SPIN"),
            CellValue::Float(2.0),
        ]);
        let entry = PositionFracEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.coord, [0.5, 0.5, 0.5]);
        assert_eq!(entry.spin, Some(2.0));
    }

    #[test]
    fn test_position_frac_entry_with_spin_equals() {
        let val = CellValue::Array(vec![
            CellValue::Str("O"),
            CellValue::Float(0.25),
            CellValue::Float(0.25),
            CellValue::Float(0.25),
            CellValue::Str("SPIN="),
            CellValue::Float(1.5),
        ]);
        let entry = PositionFracEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.spin, Some(1.5));
    }

    #[test]
    fn test_position_frac_entry_insufficient_elements() {
        let val = CellValue::Array(vec![
            CellValue::Str("Fe"),
            CellValue::Float(0.0),
            CellValue::Float(0.0),
        ]);
        assert!(PositionFracEntry::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_positions_frac_multiple_entries() {
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
        let result = PositionsFrac::from_block_rows(&rows).unwrap();
        assert_eq!(result.positions.len(), 2);
    }

    #[test]
    fn test_positions_frac_empty() {
        let result = PositionsFrac::from_block_rows(&[]).unwrap();
        assert_eq!(result.positions.len(), 0);
    }

    #[test]
    fn test_positions_frac_block_name() {
        assert_eq!(PositionsFrac::BLOCK_NAME, "POSITIONS_FRAC");
    }

    #[test]
    fn test_positions_frac_to_cell() {
        let positions = PositionsFrac {
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
                assert_eq!(name, "POSITIONS_FRAC");
                assert_eq!(values.len(), 1);
            }
            _ => panic!("Expected Block"),
        }
    }

    #[test]
    fn test_position_frac_entry_to_cell_value() {
        let entry = PositionFracEntry {
            species: Species::from_cell_value(&CellValue::Str("Fe")).unwrap(),
            coord: [0.1, 0.2, 0.3],
            spin: Some(1.5),
            mixture: None,
        };
        let val = entry.to_cell_value();
        match val {
            CellValue::Array(arr) => {
                assert!(arr.len() >= 6);
            }
            _ => panic!("Expected Array"),
        }
    }

    #[test]
    fn test_position_frac_entry_with_mixture_only() {
        let val = CellValue::Array(vec![
            CellValue::Str("Al"),
            CellValue::Float(0.25),
            CellValue::Float(0.5),
            CellValue::Float(0.0),
            CellValue::Str("MIXTURE"),
            CellValue::UInt(1),
            CellValue::Float(0.666667),
        ]);
        let entry = PositionFracEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.coord, [0.25, 0.5, 0.0]);
        assert_eq!(entry.mixture, Some((1, 0.666667)));
        assert!(entry.spin.is_none());
    }

    #[test]
    fn test_position_frac_entry_with_mixture_equals() {
        let val = CellValue::Array(vec![
            CellValue::Str("Si"),
            CellValue::Float(0.25),
            CellValue::Float(0.5),
            CellValue::Float(0.0),
            CellValue::Str("MIXTURE="),
            CellValue::UInt(2),
            CellValue::Float(0.333333),
        ]);
        let entry = PositionFracEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.mixture, Some((2, 0.333333)));
    }

    #[test]
    fn test_position_frac_entry_with_spin_and_mixture() {
        let val = CellValue::Array(vec![
            CellValue::Str("Fe"),
            CellValue::Float(0.5),
            CellValue::Float(0.5),
            CellValue::Float(0.5),
            CellValue::Str("SPIN"),
            CellValue::Float(2.0),
            CellValue::Str("MIXTURE"),
            CellValue::UInt(1),
            CellValue::Float(0.5),
        ]);
        let entry = PositionFracEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.spin, Some(2.0));
        assert_eq!(entry.mixture, Some((1, 0.5)));
    }

    #[test]
    fn test_position_frac_entry_with_mixture_and_spin_reversed() {
        let val = CellValue::Array(vec![
            CellValue::Str("O"),
            CellValue::Float(0.25),
            CellValue::Float(0.25),
            CellValue::Float(0.25),
            CellValue::Str("MIXTURE"),
            CellValue::UInt(3),
            CellValue::Float(0.75),
            CellValue::Str("SPIN="),
            CellValue::Float(1.5),
        ]);
        let entry = PositionFracEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.mixture, Some((3, 0.75)));
        assert_eq!(entry.spin, Some(1.5));
    }

    #[test]
    fn test_position_frac_entry_mixture_with_int_index() {
        let val = CellValue::Array(vec![
            CellValue::Str("Al"),
            CellValue::Float(0.0),
            CellValue::Float(0.0),
            CellValue::Float(0.0),
            CellValue::Str("MIXTURE"),
            CellValue::Int(1),
            CellValue::Float(0.5),
        ]);
        let entry = PositionFracEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.mixture, Some((1, 0.5)));
    }

    #[test]
    fn test_position_frac_entry_mixture_incomplete() {
        let val = CellValue::Array(vec![
            CellValue::Str("Al"),
            CellValue::Float(0.0),
            CellValue::Float(0.0),
            CellValue::Float(0.0),
            CellValue::Str("MIXTURE"),
            CellValue::UInt(1),
        ]);
        assert!(PositionFracEntry::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_position_frac_entry_to_cell_value_with_mixture() {
        let entry = PositionFracEntry {
            species: Species::from_cell_value(&CellValue::Str("Al")).unwrap(),
            coord: [0.25, 0.5, 0.0],
            spin: None,
            mixture: Some((1, 0.666667)),
        };
        let val = entry.to_cell_value();
        match val {
            CellValue::Array(arr) => {
                assert_eq!(arr.len(), 7); // species + 3 coords + MIXTURE= + index + weight
                assert_eq!(value_as_string(&arr[4]).unwrap().to_ascii_uppercase(), "MIXTURE=");
                assert_eq!(
                    match &arr[5] {
                        CellValue::UInt(u) => *u,
                        _ => panic!("Expected UInt"),
                    },
                    1
                );
            }
            _ => panic!("Expected Array"),
        }
    }

    #[test]
    fn test_position_frac_entry_to_cell_value_with_spin_and_mixture() {
        let entry = PositionFracEntry {
            species: Species::from_cell_value(&CellValue::Str("Fe")).unwrap(),
            coord: [0.5, 0.5, 0.5],
            spin: Some(2.0),
            mixture: Some((1, 0.5)),
        };
        let val = entry.to_cell_value();
        match val {
            CellValue::Array(arr) => {
                assert_eq!(arr.len(), 9); // species + 3 coords + SPIN= + value + MIXTURE= + index + weight
            }
            _ => panic!("Expected Array"),
        }
    }

    #[test]
    fn test_position_frac_roundtrip_with_mixture() {
        let original = PositionFracEntry {
            species: Species::from_cell_value(&CellValue::Str("Si")).unwrap(),
            coord: [0.25, 0.5, 0.0],
            spin: None,
            mixture: Some((2, 0.333333)),
        };
        let cell_value = original.to_cell_value();
        let parsed = PositionFracEntry::from_cell_value(&cell_value).unwrap();
        assert_eq!(parsed.species, original.species);
        assert_eq!(parsed.coord, original.coord);
        assert_eq!(parsed.mixture, original.mixture);
        assert_eq!(parsed.spin, original.spin);
    }

    #[test]
    fn test_position_frac_roundtrip_with_spin_and_mixture() {
        let original = PositionFracEntry {
            species: Species::from_cell_value(&CellValue::Str("Fe")).unwrap(),
            coord: [0.1, 0.2, 0.3],
            spin: Some(1.5),
            mixture: Some((1, 0.75)),
        };
        let cell_value = original.to_cell_value();
        let parsed = PositionFracEntry::from_cell_value(&cell_value).unwrap();
        assert_eq!(parsed.species, original.species);
        assert_eq!(parsed.coord, original.coord);
        assert_eq!(parsed.spin, original.spin);
        assert_eq!(parsed.mixture, original.mixture);
    }
}
