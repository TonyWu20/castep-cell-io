#![allow(dead_code)]
use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, FromCellValue, FromBlock, CResult, query::value_as_f64, query::value_as_str};

use crate::cell::species::Species;
use crate::units::LengthUnit;

/// Represents a single atom entry within the POSITIONS_ABS_PRODUCT block.
///
/// Consists of the element symbol/number, absolute coordinates, and optional spin/mixture qualifiers.
/// Format: <element> <x> <y> <z> [SPIN <value>] [MIXTURE <index> <weight>]
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct PositionAbsProductEntry {
    /// The chemical element symbol (e.g., "Fe") or atomic number as a string (e.g., "26").
    pub species: Species,
    /// Absolute coordinates [x, y, z].
    pub coord: [f64; 3],
    /// Optional initial spin polarization for the atom.
    pub spin: Option<f64>,
    /// Optional mixture specification: (index, weight) for disordered systems.
    pub mixture: Option<(u32, f64)>,
}

impl FromCellValue for PositionAbsProductEntry {
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
                    let keyword = value_as_str(&arr[idx])?.to_ascii_uppercase();

                    if keyword == "SPIN" || keyword == "SPIN=" {
                        if idx + 1 < arr.len() {
                            spin = Some(value_as_f64(&arr[idx + 1])?);
                            idx += 2;
                        } else {
                            return Err(castep_cell_fmt::Error::Message(
                                "SPIN qualifier requires a value".into(),
                            ));
                        }
                    } else if keyword == "MIXTURE" || keyword == "MIXTURE=" {
                        if idx + 2 < arr.len() {
                            let mix_idx = match &arr[idx + 1] {
                                CellValue::UInt(u) => *u,
                                CellValue::Int(i) if *i >= 0 => *i as u32,
                                _ => {
                                    return Err(castep_cell_fmt::Error::Message(
                                        "MIXTURE index must be a positive integer".into(),
                                    ))
                                }
                            };
                            let mix_weight = value_as_f64(&arr[idx + 2])?;
                            mixture = Some((mix_idx, mix_weight));
                            idx += 3;
                        } else {
                            return Err(castep_cell_fmt::Error::Message(
                                "MIXTURE qualifier requires index and weight".into(),
                            ));
                        }
                    } else {
                        return Err(castep_cell_fmt::Error::Message(
                            format!("unknown qualifier: {keyword}"),
                        ));
                    }
                }

                Ok(PositionAbsProductEntry {
                    species,
                    coord,
                    spin,
                    mixture,
                })
            }
            _ => Err(castep_cell_fmt::Error::Message(
                "PositionAbsProductEntry must be an array of at least 4 elements".into(),
            )),
        }
    }
}

impl ToCellValue for PositionAbsProductEntry {
    /// Converts the entry into a `CellValue::Array` representing one line of the block.
    fn to_cell_value(&self) -> CellValue<'_> {
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

/// Represents the POSITIONS_ABS_PRODUCT block.
///
/// Contains a list of atomic positions in absolute (Cartesian) coordinates for product structures.
/// Format:
/// %BLOCK POSITIONS_ABS_PRODUCT
/// [units]
/// Species1/I1 R1x R1y R1z [SPIN S1] [MIXTURE M1 W1]
/// Species2/I2 R2x R2y R2z [SPIN S2] [MIXTURE M2 W2]
/// ...
/// %ENDBLOCK POSITIONS_ABS_PRODUCT
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct PositionsAbsProduct {
    /// Optional unit specifier (default: Angstrom).
    pub unit: Option<LengthUnit>,
    /// The list of atom entries.
    #[builder(default)]
    pub positions: Vec<PositionAbsProductEntry>,
}

impl FromBlock for PositionsAbsProduct {
    const BLOCK_NAME: &'static str = "POSITIONS_ABS_PRODUCT";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        if rows.is_empty() {
            return Ok(PositionsAbsProduct {
                unit: None,
                positions: vec![],
            });
        }

        let (unit, data_start) = if let CellValue::Array(arr) = &rows[0] {
            if arr.len() == 1 {
                if let Ok(u) = LengthUnit::from_cell_value(&arr[0]) {
                    (Some(u), 1)
                } else {
                    (None, 0)
                }
            } else {
                (None, 0)
            }
        } else {
            (None, 0)
        };

        let positions = rows[data_start..]
            .iter()
            .map(PositionAbsProductEntry::from_cell_value)
            .collect::<CResult<Vec<_>>>()?;

        Ok(PositionsAbsProduct { unit, positions })
    }
}

impl ToCell for PositionsAbsProduct {
    /// Converts the block into the intermediate `Cell` representation for serialization.
    fn to_cell(&self) -> Cell<'_> {
        let mut block_content = Vec::new();
        if let Some(u) = &self.unit {
            block_content.push(CellValue::Array(vec![u.to_cell_value()]));
        }
        block_content.extend(self.positions.iter().map(|entry| entry.to_cell_value()));
        Cell::Block("POSITIONS_ABS_PRODUCT", block_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_abs_product_entry_without_qualifiers() {
        let val = CellValue::Array(vec![
            CellValue::Str("Fe"),
            CellValue::Float(0.0),
            CellValue::Float(0.0),
            CellValue::Float(0.0),
        ]);
        let entry = PositionAbsProductEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.coord, [0.0, 0.0, 0.0]);
        assert!(entry.spin.is_none());
        assert!(entry.mixture.is_none());
    }

    #[test]
    fn test_position_abs_product_entry_with_spin() {
        let val = CellValue::Array(vec![
            CellValue::Str("Fe"),
            CellValue::Float(1.5),
            CellValue::Float(2.5),
            CellValue::Float(3.5),
            CellValue::Str("SPIN"),
            CellValue::Float(2.0),
        ]);
        let entry = PositionAbsProductEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.coord, [1.5, 2.5, 3.5]);
        assert_eq!(entry.spin, Some(2.0));
        assert!(entry.mixture.is_none());
    }

    #[test]
    fn test_position_abs_product_entry_with_mixture() {
        let val = CellValue::Array(vec![
            CellValue::Str("O"),
            CellValue::Float(0.5),
            CellValue::Float(0.5),
            CellValue::Float(0.5),
            CellValue::Str("MIXTURE="),
            CellValue::UInt(1),
            CellValue::Float(0.8),
        ]);
        let entry = PositionAbsProductEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.coord, [0.5, 0.5, 0.5]);
        assert!(entry.spin.is_none());
        assert_eq!(entry.mixture, Some((1, 0.8)));
    }

    #[test]
    fn test_position_abs_product_entry_with_spin_and_mixture() {
        let val = CellValue::Array(vec![
            CellValue::Str("Fe"),
            CellValue::Float(1.0),
            CellValue::Float(2.0),
            CellValue::Float(3.0),
            CellValue::Str("SPIN="),
            CellValue::Float(1.5),
            CellValue::Str("MIXTURE"),
            CellValue::UInt(2),
            CellValue::Float(0.6),
        ]);
        let entry = PositionAbsProductEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.coord, [1.0, 2.0, 3.0]);
        assert_eq!(entry.spin, Some(1.5));
        assert_eq!(entry.mixture, Some((2, 0.6)));
    }

    #[test]
    fn test_position_abs_product_entry_insufficient_elements() {
        let val = CellValue::Array(vec![
            CellValue::Str("Fe"),
            CellValue::Float(0.0),
            CellValue::Float(0.0),
        ]);
        assert!(PositionAbsProductEntry::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_positions_abs_product_no_unit() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Str("Fe"),
                CellValue::Float(0.0),
                CellValue::Float(0.0),
                CellValue::Float(0.0),
            ]),
            CellValue::Array(vec![
                CellValue::Str("O"),
                CellValue::Float(1.5),
                CellValue::Float(1.5),
                CellValue::Float(1.5),
            ]),
        ];
        let result = PositionsAbsProduct::from_block_rows(&rows).unwrap();
        assert_eq!(result.unit, None);
        assert_eq!(result.positions.len(), 2);
    }

    #[test]
    fn test_positions_abs_product_with_unit_ang() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Str("ang")]),
            CellValue::Array(vec![
                CellValue::Str("Fe"),
                CellValue::Float(0.0),
                CellValue::Float(0.0),
                CellValue::Float(0.0),
            ]),
        ];
        let result = PositionsAbsProduct::from_block_rows(&rows).unwrap();
        assert_eq!(result.unit, Some(LengthUnit::Ang));
        assert_eq!(result.positions.len(), 1);
    }

    #[test]
    fn test_positions_abs_product_with_unit_bohr() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Str("bohr")]),
            CellValue::Array(vec![
                CellValue::Str("Fe"),
                CellValue::Float(0.0),
                CellValue::Float(0.0),
                CellValue::Float(0.0),
            ]),
        ];
        let result = PositionsAbsProduct::from_block_rows(&rows).unwrap();
        assert_eq!(result.unit, Some(LengthUnit::Bohr));
        assert_eq!(result.positions.len(), 1);
    }

    #[test]
    fn test_positions_abs_product_empty() {
        let result = PositionsAbsProduct::from_block_rows(&[]).unwrap();
        assert_eq!(result.unit, None);
        assert_eq!(result.positions.len(), 0);
    }

    #[test]
    fn test_positions_abs_product_block_name() {
        assert_eq!(PositionsAbsProduct::BLOCK_NAME, "POSITIONS_ABS_PRODUCT");
    }

    #[test]
    fn test_positions_abs_product_to_cell_no_unit() {
        let positions = PositionsAbsProduct {
            unit: None,
            positions: vec![PositionAbsProductEntry {
                species: Species::from_cell_value(&CellValue::Str("Fe")).unwrap(),
                coord: [0.0, 0.0, 0.0],
                spin: None,
                mixture: None,
            }],
        };
        let cell = positions.to_cell();
        match cell {
            Cell::Block(name, values) => {
                assert_eq!(name, "POSITIONS_ABS_PRODUCT");
                assert_eq!(values.len(), 1);
            }
            _ => panic!("Expected Block"),
        }
    }

    #[test]
    fn test_positions_abs_product_to_cell_with_unit() {
        let positions = PositionsAbsProduct {
            unit: Some(LengthUnit::Ang),
            positions: vec![PositionAbsProductEntry {
                species: Species::from_cell_value(&CellValue::Str("Fe")).unwrap(),
                coord: [1.0, 2.0, 3.0],
                spin: Some(1.5),
                mixture: None,
            }],
        };
        let cell = positions.to_cell();
        match cell {
            Cell::Block(name, values) => {
                assert_eq!(name, "POSITIONS_ABS_PRODUCT");
                assert_eq!(values.len(), 2); // unit header + 1 position
            }
            _ => panic!("Expected Block"),
        }
    }

    #[test]
    fn test_position_abs_product_entry_to_cell_value() {
        let entry = PositionAbsProductEntry {
            species: Species::from_cell_value(&CellValue::Str("Fe")).unwrap(),
            coord: [0.1, 0.2, 0.3],
            spin: Some(1.5),
            mixture: Some((1, 0.7)),
        };
        let val = entry.to_cell_value();
        match val {
            CellValue::Array(arr) => {
                assert!(arr.len() >= 8); // species + 3 coords + SPIN= + spin + MIXTURE= + idx + weight
            }
            _ => panic!("Expected Array"),
        }
    }
}
