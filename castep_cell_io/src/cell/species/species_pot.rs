use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, FromCellValue, FromBlock, CResult, query::value_as_str};
use super::Species;

/// Represents a single entry within the SPECIES_POT block,
/// linking a species to its pseudopotential filename.
#[derive(Debug, Clone, PartialEq, Eq, bon::Builder)]
pub struct SpeciesPotEntry {
    /// The species (symbol or atomic number).
    pub species: Species,
    /// The filename of the pseudopotential.
    pub filename: String,
}

impl FromCellValue for SpeciesPotEntry {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() == 2 => {
                Ok(SpeciesPotEntry {
                    species: Species::from_cell_value(&arr[0])?,
                    filename: value_as_str(&arr[1])?.to_string(),
                })
            }
            _ => Err(castep_cell_fmt::Error::Message(
                "SpeciesPotEntry must be an array of [species, filename]".into(),
            )),
        }
    }
}

impl ToCellValue for SpeciesPotEntry {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Array(vec![
            self.species.to_cell_value(),
            CellValue::String(self.filename.clone()),
        ])
    }
}

/// Represents the SPECIES_POT block.
///
/// Defines the pseudopotential files associated with each atomic species.
/// Format:
/// %BLOCK SPECIES_POT
/// CCC1/I1 [filename]
/// CCC2/I2 [filename]
/// ...
/// %ENDBLOCK SPECIES_POT
#[derive(Debug, Clone, PartialEq, Eq, bon::Builder)]
pub struct SpeciesPot {
    /// The list of species and their corresponding pseudopotential filenames.
    pub potentials: Vec<SpeciesPotEntry>,
}

impl FromBlock for SpeciesPot {
    const BLOCK_NAME: &'static str = "SPECIES_POT";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        let potentials = rows
            .iter()
            .map(SpeciesPotEntry::from_cell_value)
            .collect::<CResult<Vec<_>>>()?;
        Ok(SpeciesPot { potentials })
    }
}

impl ToCell for SpeciesPot {
    fn to_cell(&self) -> Cell<'_> {
        Cell::Block(
            "SPECIES_POT",
            self.potentials
                .iter()
                .map(|pot_entry| pot_entry.to_cell_value())
                .collect::<Vec<CellValue>>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_fmt::CellValue;

    #[test]
    fn test_species_pot_entry_from_cell_value() {
        let val = CellValue::Array(vec![
            CellValue::Str("Fe"),
            CellValue::Str("Fe.usp"),
        ]);
        let entry = SpeciesPotEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.species, Species::Symbol("Fe".to_string()));
        assert_eq!(entry.filename, "Fe.usp");
    }

    #[test]
    fn test_species_pot_entry_insufficient_elements() {
        let val = CellValue::Array(vec![CellValue::Str("Fe")]);
        assert!(SpeciesPotEntry::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_species_pot_empty() {
        let result = SpeciesPot::from_block_rows(&[]).unwrap();
        assert_eq!(result.potentials.len(), 0);
    }

    #[test]
    fn test_species_pot_single_entry() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Str("Fe"),
                CellValue::Str("Fe.usp"),
            ]),
        ];
        let result = SpeciesPot::from_block_rows(&rows).unwrap();
        assert_eq!(result.potentials.len(), 1);
        assert_eq!(result.potentials[0].filename, "Fe.usp");
    }

    #[test]
    fn test_species_pot_multiple_entries() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Str("Fe"),
                CellValue::Str("Fe.usp"),
            ]),
            CellValue::Array(vec![
                CellValue::Str("O"),
                CellValue::Str("O.usp"),
            ]),
        ];
        let result = SpeciesPot::from_block_rows(&rows).unwrap();
        assert_eq!(result.potentials.len(), 2);
    }

    #[test]
    fn test_block_name() {
        assert_eq!(SpeciesPot::BLOCK_NAME, "SPECIES_POT");
    }

    #[test]
    fn test_species_pot_builder_basic() {
        let pot = SpeciesPot::builder()
            .potentials(vec![])
            .build();
        assert_eq!(pot.potentials.len(), 0);
    }

    #[test]
    fn test_species_pot_builder_with_single_entry() {
        let entry = SpeciesPotEntry {
            species: Species::Symbol("Fe".to_string()),
            filename: "Fe.usp".to_string(),
        };
        let pot = SpeciesPot::builder()
            .potentials(vec![entry.clone()])
            .build();
        assert_eq!(pot.potentials.len(), 1);
        assert_eq!(pot.potentials[0], entry);
    }

    #[test]
    fn test_species_pot_builder_with_multiple_entries() {
        let entries = vec![
            SpeciesPotEntry {
                species: Species::Symbol("Fe".to_string()),
                filename: "Fe.usp".to_string(),
            },
            SpeciesPotEntry {
                species: Species::Symbol("O".to_string()),
                filename: "O.usp".to_string(),
            },
        ];
        let pot = SpeciesPot::builder()
            .potentials(entries.clone())
            .build();
        assert_eq!(pot.potentials.len(), 2);
        assert_eq!(pot.potentials, entries);
    }

    #[test]
    fn test_species_pot_builder_push_single_entry() {
        let entry = SpeciesPotEntry {
            species: Species::Symbol("Fe".to_string()),
            filename: "Fe.usp".to_string(),
        };
        let mut pot = SpeciesPot::builder()
            .potentials(vec![])
            .build();
        pot.potentials.push(entry.clone());
        assert_eq!(pot.potentials.len(), 1);
        assert_eq!(pot.potentials[0], entry);
    }

    #[test]
    fn test_species_pot_builder_push_multiple_entries() {
        let entry1 = SpeciesPotEntry {
            species: Species::Symbol("Fe".to_string()),
            filename: "Fe.usp".to_string(),
        };
        let entry2 = SpeciesPotEntry {
            species: Species::Symbol("O".to_string()),
            filename: "O.usp".to_string(),
        };
        let entry3 = SpeciesPotEntry {
            species: Species::Symbol("Ni".to_string()),
            filename: "Ni.usp".to_string(),
        };
        let mut pot = SpeciesPot::builder()
            .potentials(vec![])
            .build();
        pot.potentials.push(entry1.clone());
        pot.potentials.push(entry2.clone());
        pot.potentials.push(entry3.clone());
        assert_eq!(pot.potentials.len(), 3);
        assert_eq!(pot.potentials[0], entry1);
        assert_eq!(pot.potentials[1], entry2);
        assert_eq!(pot.potentials[2], entry3);
    }

    #[test]
    fn test_species_pot_builder_mixed_initialization() {
        let entry1 = SpeciesPotEntry {
            species: Species::Symbol("Fe".to_string()),
            filename: "Fe.usp".to_string(),
        };
        let entry2 = SpeciesPotEntry {
            species: Species::Symbol("O".to_string()),
            filename: "O.usp".to_string(),
        };
        let entry3 = SpeciesPotEntry {
            species: Species::Symbol("Ni".to_string()),
            filename: "Ni.usp".to_string(),
        };
        let mut pot = SpeciesPot::builder()
            .potentials(vec![entry1.clone()])
            .build();
        pot.potentials.push(entry2.clone());
        pot.potentials.push(entry3.clone());
        assert_eq!(pot.potentials.len(), 3);
        assert_eq!(pot.potentials[0], entry1);
        assert_eq!(pot.potentials[1], entry2);
        assert_eq!(pot.potentials[2], entry3);
    }

    #[test]
    fn test_species_pot_builder_method_chaining() {
        let entry1 = SpeciesPotEntry {
            species: Species::Symbol("Fe".to_string()),
            filename: "Fe.usp".to_string(),
        };
        let entry2 = SpeciesPotEntry {
            species: Species::Symbol("O".to_string()),
            filename: "O.usp".to_string(),
        };
        let pot = SpeciesPot::builder()
            .potentials(vec![entry1, entry2])
            .build();
        assert_eq!(pot.potentials.len(), 2);
    }
}

