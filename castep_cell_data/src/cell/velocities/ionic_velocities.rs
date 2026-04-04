#![allow(dead_code)]
use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, FromCellValue, FromBlock, CResult, query::value_as_f64};

use crate::cell::species::Species;
use crate::units::VelocityUnit;

/// Represents a single atom entry within the IONIC_VELOCITIES block.
///
/// Consists of the element symbol/number and velocity components in Cartesian coordinates.
/// Format: <element> <vx> <vy> <vz>
#[derive(Debug, Clone, PartialEq)]
pub struct IonicVelocityEntry {
    /// The chemical element symbol (e.g., "Fe") or atomic number as a string (e.g., "26").
    pub species: Species,
    /// Velocity components [vx, vy, vz].
    pub velocity: [f64; 3],
}

impl FromCellValue for IonicVelocityEntry {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() >= 4 => {
                let species = Species::from_cell_value(&arr[0])?;
                let velocity = [
                    value_as_f64(&arr[1])?,
                    value_as_f64(&arr[2])?,
                    value_as_f64(&arr[3])?,
                ];

                Ok(IonicVelocityEntry { species, velocity })
            }
            _ => Err(castep_cell_io::Error::Message(
                "IonicVelocityEntry must be an array of at least 4 elements".into(),
            )),
        }
    }
}

impl ToCellValue for IonicVelocityEntry {
    /// Converts the entry into a `CellValue::Array` representing one line of the block.
    fn to_cell_value(&self) -> CellValue {
        let mut arr = vec![self.species.to_cell_value()];
        arr.extend(self.velocity.into_iter().map(CellValue::Float));
        CellValue::Array(arr)
    }
}

/// Represents the IONIC_VELOCITIES block.
///
/// Contains a list of ionic velocities in Cartesian coordinates.
/// Format:
/// %BLOCK IONIC_VELOCITIES
/// [units]
/// Species1/I1 V1x V1y V1z
/// Species2/I2 V2x V2y V2z
/// ...
/// %ENDBLOCK IONIC_VELOCITIES
#[derive(Debug, Clone, PartialEq)]
pub struct IonicVelocities {
    /// Optional unit specifier (default: Å/ps).
    pub unit: Option<VelocityUnit>,
    /// The list of velocity entries.
    pub velocities: Vec<IonicVelocityEntry>,
}

impl FromBlock for IonicVelocities {
    const BLOCK_NAME: &'static str = "IONIC_VELOCITIES";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        if rows.is_empty() {
            return Ok(IonicVelocities {
                unit: None,
                velocities: vec![],
            });
        }

        let (unit, data_start) = if let CellValue::Array(arr) = &rows[0] {
            if arr.len() == 1 {
                if let Ok(u) = VelocityUnit::from_cell_value(&arr[0]) {
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

        let velocities = rows[data_start..]
            .iter()
            .map(IonicVelocityEntry::from_cell_value)
            .collect::<CResult<Vec<_>>>()?;

        Ok(IonicVelocities { unit, velocities })
    }
}

impl ToCell for IonicVelocities {
    /// Converts the block into the intermediate `Cell` representation for serialization.
    fn to_cell(&self) -> Cell {
        let mut block_content = Vec::new();
        if let Some(u) = &self.unit {
            block_content.push(CellValue::Array(vec![u.to_cell_value()]));
        }
        block_content.extend(self.velocities.iter().map(|entry| entry.to_cell_value()));
        Cell::Block("IONIC_VELOCITIES", block_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ionic_velocity_entry_basic() {
        let val = CellValue::Array(vec![
            CellValue::Str("Fe"),
            CellValue::Float(1.0),
            CellValue::Float(2.0),
            CellValue::Float(3.0),
        ]);
        let entry = IonicVelocityEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.velocity, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_ionic_velocity_entry_negative_velocities() {
        let val = CellValue::Array(vec![
            CellValue::Str("O"),
            CellValue::Float(-1.5),
            CellValue::Float(-2.5),
            CellValue::Float(-3.5),
        ]);
        let entry = IonicVelocityEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.velocity, [-1.5, -2.5, -3.5]);
    }

    #[test]
    fn test_ionic_velocity_entry_insufficient_elements() {
        let val = CellValue::Array(vec![
            CellValue::Str("Fe"),
            CellValue::Float(0.0),
            CellValue::Float(0.0),
        ]);
        assert!(IonicVelocityEntry::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_ionic_velocity_entry_to_cell_value() {
        let entry = IonicVelocityEntry {
            species: Species::from_cell_value(&CellValue::Str("Fe")).unwrap(),
            velocity: [0.1, 0.2, 0.3],
        };
        let val = entry.to_cell_value();
        match val {
            CellValue::Array(arr) => {
                assert_eq!(arr.len(), 4); // species + 3 velocity components
            }
            _ => panic!("Expected Array"),
        }
    }

    #[test]
    fn test_ionic_velocities_no_unit() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Str("Fe"),
                CellValue::Float(1.0),
                CellValue::Float(2.0),
                CellValue::Float(3.0),
            ]),
            CellValue::Array(vec![
                CellValue::Str("O"),
                CellValue::Float(0.5),
                CellValue::Float(0.5),
                CellValue::Float(0.5),
            ]),
        ];
        let result = IonicVelocities::from_block_rows(&rows).unwrap();
        assert_eq!(result.unit, None);
        assert_eq!(result.velocities.len(), 2);
    }

    #[test]
    fn test_ionic_velocities_with_unit_ang_ps() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Str("ang/ps")]),
            CellValue::Array(vec![
                CellValue::Str("Fe"),
                CellValue::Float(1.0),
                CellValue::Float(2.0),
                CellValue::Float(3.0),
            ]),
        ];
        let result = IonicVelocities::from_block_rows(&rows).unwrap();
        assert_eq!(result.unit, Some(VelocityUnit::AngPerPs));
        assert_eq!(result.velocities.len(), 1);
    }

    #[test]
    fn test_ionic_velocities_with_unit_bohr_ps() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Str("bohr/ps")]),
            CellValue::Array(vec![
                CellValue::Str("O"),
                CellValue::Float(0.5),
                CellValue::Float(0.5),
                CellValue::Float(0.5),
            ]),
        ];
        let result = IonicVelocities::from_block_rows(&rows).unwrap();
        assert_eq!(result.unit, Some(VelocityUnit::BohrPerPs));
        assert_eq!(result.velocities.len(), 1);
    }

    #[test]
    fn test_ionic_velocities_empty() {
        let result = IonicVelocities::from_block_rows(&[]).unwrap();
        assert_eq!(result.unit, None);
        assert_eq!(result.velocities.len(), 0);
    }

    #[test]
    fn test_ionic_velocities_block_name() {
        assert_eq!(IonicVelocities::BLOCK_NAME, "IONIC_VELOCITIES");
    }

    #[test]
    fn test_ionic_velocities_to_cell_no_unit() {
        let velocities = IonicVelocities {
            unit: None,
            velocities: vec![IonicVelocityEntry {
                species: Species::from_cell_value(&CellValue::Str("Fe")).unwrap(),
                velocity: [1.0, 2.0, 3.0],
            }],
        };
        let cell = velocities.to_cell();
        match cell {
            Cell::Block(name, values) => {
                assert_eq!(name, "IONIC_VELOCITIES");
                assert_eq!(values.len(), 1);
            }
            _ => panic!("Expected Block"),
        }
    }

    #[test]
    fn test_ionic_velocities_to_cell_with_unit() {
        let velocities = IonicVelocities {
            unit: Some(VelocityUnit::AngPerPs),
            velocities: vec![IonicVelocityEntry {
                species: Species::from_cell_value(&CellValue::Str("Fe")).unwrap(),
                velocity: [1.0, 2.0, 3.0],
            }],
        };
        let cell = velocities.to_cell();
        match cell {
            Cell::Block(name, values) => {
                assert_eq!(name, "IONIC_VELOCITIES");
                assert_eq!(values.len(), 2); // unit header + 1 velocity
            }
            _ => panic!("Expected Block"),
        }
    }

    #[test]
    fn test_ionic_velocities_multiple_entries_with_unit() {
        let velocities = IonicVelocities {
            unit: Some(VelocityUnit::BohrPerFs),
            velocities: vec![
                IonicVelocityEntry {
                    species: Species::from_cell_value(&CellValue::Str("Fe")).unwrap(),
                    velocity: [1.0, 2.0, 3.0],
                },
                IonicVelocityEntry {
                    species: Species::from_cell_value(&CellValue::Str("O")).unwrap(),
                    velocity: [-1.0, -2.0, -3.0],
                },
            ],
        };
        let cell = velocities.to_cell();
        match cell {
            Cell::Block(name, values) => {
                assert_eq!(name, "IONIC_VELOCITIES");
                assert_eq!(values.len(), 3); // unit header + 2 velocities
            }
            _ => panic!("Expected Block"),
        }
    }
}
