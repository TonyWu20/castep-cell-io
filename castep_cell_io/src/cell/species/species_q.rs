use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, parse::{FromBlock, FromCellValue}, CResult, Error, query::value_as_f64};
use super::Species;
use crate::units::QuadrupoleMomentUnit;

/// Represents a single entry within the SPECIES_Q block,
/// linking a species to its quadrupole moment.
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct SpeciesQEntry {
    /// The species (symbol or atomic number).
    pub species: Species,
    /// The quadrupole moment of the species.
    pub quadrupole_moment: f64,
}

impl FromCellValue for SpeciesQEntry {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() == 2 => {
                Ok(SpeciesQEntry {
                    species: Species::from_cell_value(&arr[0])?,
                    quadrupole_moment: value_as_f64(&arr[1])?,
                })
            }
            _ => Err(Error::Message(
                "SpeciesQEntry must be an array of [species, quadrupole_moment]".into(),
            )),
        }
    }
}

impl ToCellValue for SpeciesQEntry {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Array(vec![
            self.species.to_cell_value(),
            CellValue::Float(self.quadrupole_moment),
        ])
    }
}

/// Represents the SPECIES_Q block.
///
/// Defines the nuclear electric quadrupole moment of each atomic species.
/// Format:
/// %BLOCK SPECIES_Q
/// [units]
/// CCC1/I1 R1
/// CCC2/I2 R2
/// ...
/// %ENDBLOCK SPECIES_Q
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct SpeciesQ {
    /// The unit of quadrupole moment. If None, the default (Barn) is used.
    pub unit: Option<QuadrupoleMomentUnit>,
    /// The list of species and their corresponding quadrupole moments.
    #[builder(default)]
    pub moments: Vec<SpeciesQEntry>,
}

impl FromBlock for SpeciesQ {
    const BLOCK_NAME: &'static str = "SPECIES_Q";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        if rows.is_empty() {
            return Ok(Self {
                unit: None,
                moments: Vec::new(),
            });
        }

        let (unit, data_start) = if let CellValue::Array(arr) = &rows[0] {
            if arr.len() == 1 {
                if let Ok(u) = QuadrupoleMomentUnit::from_cell_value(&arr[0]) {
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

        let moments = rows[data_start..]
            .iter()
            .map(SpeciesQEntry::from_cell_value)
            .collect::<CResult<Vec<_>>>()?;

        Ok(Self { unit, moments })
    }
}

impl ToCell for SpeciesQ {
    fn to_cell(&self) -> Cell<'_> {
        let mut block_content = Vec::new();

        if let Some(ref u) = self.unit {
            block_content.push(CellValue::Array(vec![u.to_cell_value()]));
        }

        block_content.extend(
            self.moments.iter().map(|entry| entry.to_cell_value()),
        );

        Cell::Block("SPECIES_Q", block_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_fmt::CellValue;

    #[test]
    fn test_species_q_entry_from_cell_value() {
        let val = CellValue::Array(vec![
            CellValue::Str("B"),
            CellValue::Float(0.04059),
        ]);
        let entry = SpeciesQEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.species, Species::Symbol("B".to_string()));
        assert_eq!(entry.quadrupole_moment, 0.04059);
    }

    #[test]
    fn test_species_q_entry_with_atomic_number() {
        let val = CellValue::Array(vec![
            CellValue::UInt(5),
            CellValue::Float(0.04059),
        ]);
        let entry = SpeciesQEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.species, Species::AtomicNumber(5));
        assert_eq!(entry.quadrupole_moment, 0.04059);
    }

    #[test]
    fn test_species_q_entry_insufficient_elements() {
        let val = CellValue::Array(vec![CellValue::Str("B")]);
        assert!(SpeciesQEntry::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_species_q_entry_too_many_elements() {
        let val = CellValue::Array(vec![
            CellValue::Str("B"),
            CellValue::Float(0.04059),
            CellValue::Float(1.0),
        ]);
        assert!(SpeciesQEntry::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_species_q_empty() {
        let result = SpeciesQ::from_block_rows(&[]).unwrap();
        assert!(result.unit.is_none());
        assert_eq!(result.moments.len(), 0);
    }

    #[test]
    fn test_species_q_with_barn_unit() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Str("barn")]),
            CellValue::Array(vec![
                CellValue::Str("B"),
                CellValue::Float(0.04059),
            ]),
        ];
        let result = SpeciesQ::from_block_rows(&rows).unwrap();
        assert_eq!(result.unit, Some(QuadrupoleMomentUnit::Barn));
        assert_eq!(result.moments.len(), 1);
    }

    #[test]
    fn test_species_q_with_fm2_unit() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Str("fm2")]),
            CellValue::Array(vec![
                CellValue::Str("N"),
                CellValue::Float(0.0201),
            ]),
        ];
        let result = SpeciesQ::from_block_rows(&rows).unwrap();
        assert_eq!(result.unit, Some(QuadrupoleMomentUnit::Fm2));
        assert_eq!(result.moments.len(), 1);
    }

    #[test]
    fn test_species_q_without_unit() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Str("B"),
                CellValue::Float(0.04059),
            ]),
        ];
        let result = SpeciesQ::from_block_rows(&rows).unwrap();
        assert!(result.unit.is_none());
        assert_eq!(result.moments.len(), 1);
    }

    #[test]
    fn test_species_q_multiple_entries_with_unit() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Str("barn")]),
            CellValue::Array(vec![
                CellValue::Str("B"),
                CellValue::Float(0.04059),
            ]),
            CellValue::Array(vec![
                CellValue::Str("N"),
                CellValue::Float(0.0201),
            ]),
        ];
        let result = SpeciesQ::from_block_rows(&rows).unwrap();
        assert_eq!(result.unit, Some(QuadrupoleMomentUnit::Barn));
        assert_eq!(result.moments.len(), 2);
    }

    #[test]
    fn test_species_q_multiple_entries_without_unit() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Str("B"),
                CellValue::Float(0.04059),
            ]),
            CellValue::Array(vec![
                CellValue::Str("N"),
                CellValue::Float(0.0201),
            ]),
        ];
        let result = SpeciesQ::from_block_rows(&rows).unwrap();
        assert!(result.unit.is_none());
        assert_eq!(result.moments.len(), 2);
    }

    #[test]
    fn test_block_name() {
        assert_eq!(SpeciesQ::BLOCK_NAME, "SPECIES_Q");
    }

    #[test]
    fn test_species_q_entry_to_cell_value() {
        let entry = SpeciesQEntry {
            species: Species::Symbol("B".to_string()),
            quadrupole_moment: 0.04059,
        };
        let cell_val = entry.to_cell_value();
        match cell_val {
            CellValue::Array(arr) => {
                assert_eq!(arr.len(), 2);
                assert_eq!(arr[1], CellValue::Float(0.04059));
            }
            _ => panic!("Expected CellValue::Array"),
        }
    }

    #[test]
    fn test_species_q_to_cell_with_unit() {
        let species_q = SpeciesQ {
            unit: Some(QuadrupoleMomentUnit::Barn),
            moments: vec![
                SpeciesQEntry {
                    species: Species::Symbol("B".to_string()),
                    quadrupole_moment: 0.04059,
                },
            ],
        };
        let cell = species_q.to_cell();
        match cell {
            Cell::Block(name, content) => {
                assert_eq!(name, "SPECIES_Q");
                assert_eq!(content.len(), 2); // unit header + 1 entry
            }
            _ => panic!("Expected Cell::Block"),
        }
    }

    #[test]
    fn test_species_q_to_cell_without_unit() {
        let species_q = SpeciesQ {
            unit: None,
            moments: vec![
                SpeciesQEntry {
                    species: Species::Symbol("B".to_string()),
                    quadrupole_moment: 0.04059,
                },
            ],
        };
        let cell = species_q.to_cell();
        match cell {
            Cell::Block(name, content) => {
                assert_eq!(name, "SPECIES_Q");
                assert_eq!(content.len(), 1); // only entry, no unit header
            }
            _ => panic!("Expected Cell::Block"),
        }
    }

    #[test]
    fn test_species_q_round_trip_with_unit() {
        let original = SpeciesQ {
            unit: Some(QuadrupoleMomentUnit::Fm2),
            moments: vec![
                SpeciesQEntry {
                    species: Species::Symbol("N".to_string()),
                    quadrupole_moment: 0.0201,
                },
            ],
        };
        let cell = original.to_cell();
        if let Cell::Block(_, content) = cell {
            let parsed = SpeciesQ::from_block_rows(&content).unwrap();
            assert_eq!(parsed, original);
        } else {
            panic!("Expected Cell::Block");
        }
    }

    #[test]
    fn test_species_q_round_trip_without_unit() {
        let original = SpeciesQ {
            unit: None,
            moments: vec![
                SpeciesQEntry {
                    species: Species::Symbol("B".to_string()),
                    quadrupole_moment: 0.04059,
                },
            ],
        };
        let cell = original.to_cell();
        if let Cell::Block(_, content) = cell {
            let parsed = SpeciesQ::from_block_rows(&content).unwrap();
            assert_eq!(parsed, original);
        } else {
            panic!("Expected Cell::Block");
        }
    }

    // Builder pattern tests
    #[test]
    fn test_species_q_entry_builder() {
        let entry = SpeciesQEntry::builder()
            .species(Species::Symbol("B".to_string()))
            .quadrupole_moment(0.04059)
            .build();

        assert_eq!(entry.species, Species::Symbol("B".to_string()));
        assert_eq!(entry.quadrupole_moment, 0.04059);
    }

    #[test]
    fn test_species_q_builder_empty() {
        let species_q = SpeciesQ::builder().build();

        assert!(species_q.unit.is_none());
        assert_eq!(species_q.moments.len(), 0);
    }

    #[test]
    fn test_species_q_builder_single_entry_without_unit() {
        let entry = SpeciesQEntry::builder()
            .species(Species::Symbol("B".to_string()))
            .quadrupole_moment(0.04059)
            .build();

        let species_q = SpeciesQ::builder()
            .moments(vec![entry])
            .build();

        assert!(species_q.unit.is_none());
        assert_eq!(species_q.moments.len(), 1);
        assert_eq!(species_q.moments[0].species, Species::Symbol("B".to_string()));
        assert_eq!(species_q.moments[0].quadrupole_moment, 0.04059);
    }

    #[test]
    fn test_species_q_builder_single_entry_with_unit() {
        let entry = SpeciesQEntry::builder()
            .species(Species::Symbol("N".to_string()))
            .quadrupole_moment(0.0201)
            .build();

        let species_q = SpeciesQ::builder()
            .unit(QuadrupoleMomentUnit::Fm2)
            .moments(vec![entry])
            .build();

        assert_eq!(species_q.unit, Some(QuadrupoleMomentUnit::Fm2));
        assert_eq!(species_q.moments.len(), 1);
        assert_eq!(species_q.moments[0].species, Species::Symbol("N".to_string()));
        assert_eq!(species_q.moments[0].quadrupole_moment, 0.0201);
    }

    #[test]
    fn test_species_q_builder_multiple_entries_with_vec() {
        let entry1 = SpeciesQEntry::builder()
            .species(Species::Symbol("B".to_string()))
            .quadrupole_moment(0.04059)
            .build();

        let entry2 = SpeciesQEntry::builder()
            .species(Species::Symbol("N".to_string()))
            .quadrupole_moment(0.0201)
            .build();

        let species_q = SpeciesQ::builder()
            .unit(QuadrupoleMomentUnit::Barn)
            .moments(vec![entry1, entry2])
            .build();

        assert_eq!(species_q.unit, Some(QuadrupoleMomentUnit::Barn));
        assert_eq!(species_q.moments.len(), 2);
        assert_eq!(species_q.moments[0].species, Species::Symbol("B".to_string()));
        assert_eq!(species_q.moments[1].species, Species::Symbol("N".to_string()));
    }

    #[test]
    fn test_species_q_builder_method_chaining() {
        let entry1 = SpeciesQEntry::builder()
            .species(Species::Symbol("B".to_string()))
            .quadrupole_moment(0.04059)
            .build();

        let entry2 = SpeciesQEntry::builder()
            .species(Species::AtomicNumber(7))
            .quadrupole_moment(0.0201)
            .build();

        let species_q = SpeciesQ::builder()
            .unit(QuadrupoleMomentUnit::Barn)
            .moments(vec![entry1, entry2])
            .build();

        assert_eq!(species_q.unit, Some(QuadrupoleMomentUnit::Barn));
        assert_eq!(species_q.moments.len(), 2);
        assert_eq!(species_q.moments[0].species, Species::Symbol("B".to_string()));
        assert_eq!(species_q.moments[0].quadrupole_moment, 0.04059);
        assert_eq!(species_q.moments[1].species, Species::AtomicNumber(7));
        assert_eq!(species_q.moments[1].quadrupole_moment, 0.0201);
    }
}
