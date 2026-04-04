use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, parse::{FromBlock, FromCellValue}, CResult, Error, query::value_as_f64};
use super::Species;
use crate::units::MassUnit;

/// Represents a single entry within the SPECIES_MASS block,
/// linking a species to its mass.
#[derive(Debug, Clone, PartialEq)]
pub struct SpeciesMassEntry {
    /// The species (symbol or atomic number).
    pub species: Species,
    /// The mass of the species.
    pub mass: f64,
}

impl FromCellValue for SpeciesMassEntry {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() == 2 => {
                Ok(SpeciesMassEntry {
                    species: Species::from_cell_value(&arr[0])?,
                    mass: value_as_f64(&arr[1])?,
                })
            }
            _ => Err(Error::Message(
                "SpeciesMassEntry must be an array of [species, mass]".into(),
            )),
        }
    }
}

impl ToCellValue for SpeciesMassEntry {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(vec![
            self.species.to_cell_value(),
            CellValue::Float(self.mass),
        ])
    }
}

/// Represents the SPECIES_MASS block.
///
/// Defines the mass of each atomic species.
/// Format:
/// %BLOCK SPECIES_MASS
/// [units]
/// CCC1/I1 R1
/// CCC2/I2 R2
/// ...
/// %ENDBLOCK SPECIES_MASS
#[derive(Debug, Clone, PartialEq)]
pub struct SpeciesMass {
    /// The unit of mass. If None, the default (amu) is used.
    pub unit: Option<MassUnit>,
    /// The list of species and their corresponding masses.
    pub masses: Vec<SpeciesMassEntry>,
}

impl FromBlock for SpeciesMass {
    const BLOCK_NAME: &'static str = "SPECIES_MASS";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        if rows.is_empty() {
            return Ok(Self {
                unit: None,
                masses: Vec::new(),
            });
        }

        let (unit, data_start) = if let CellValue::Array(arr) = &rows[0] {
            if arr.len() == 1 {
                if let Ok(u) = MassUnit::from_cell_value(&arr[0]) {
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

        let masses = rows[data_start..]
            .iter()
            .map(SpeciesMassEntry::from_cell_value)
            .collect::<CResult<Vec<_>>>()?;

        Ok(Self { unit, masses })
    }
}

impl ToCell for SpeciesMass {
    fn to_cell(&self) -> Cell {
        let mut block_content = Vec::new();

        if let Some(ref u) = self.unit {
            block_content.push(CellValue::Array(vec![u.to_cell_value()]));
        }

        block_content.extend(
            self.masses.iter().map(|entry| entry.to_cell_value()),
        );

        Cell::Block("SPECIES_MASS", block_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_io::CellValue;

    #[test]
    fn test_species_mass_entry_from_cell_value() {
        let val = CellValue::Array(vec![
            CellValue::Str("Fe"),
            CellValue::Float(55.845),
        ]);
        let entry = SpeciesMassEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.species, Species::Symbol("Fe".to_string()));
        assert_eq!(entry.mass, 55.845);
    }

    #[test]
    fn test_species_mass_entry_insufficient_elements() {
        let val = CellValue::Array(vec![CellValue::Str("Fe")]);
        assert!(SpeciesMassEntry::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_species_mass_empty() {
        let result = SpeciesMass::from_block_rows(&[]).unwrap();
        assert!(result.unit.is_none());
        assert_eq!(result.masses.len(), 0);
    }

    #[test]
    fn test_species_mass_with_unit() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Str("amu")]),
            CellValue::Array(vec![
                CellValue::Str("Fe"),
                CellValue::Float(55.845),
            ]),
        ];
        let result = SpeciesMass::from_block_rows(&rows).unwrap();
        assert!(result.unit.is_some());
        assert_eq!(result.masses.len(), 1);
    }

    #[test]
    fn test_species_mass_without_unit() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Str("Fe"),
                CellValue::Float(55.845),
            ]),
        ];
        let result = SpeciesMass::from_block_rows(&rows).unwrap();
        assert!(result.unit.is_none());
        assert_eq!(result.masses.len(), 1);
    }

    #[test]
    fn test_species_mass_multiple_entries() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Str("amu")]),
            CellValue::Array(vec![
                CellValue::Str("Fe"),
                CellValue::Float(55.845),
            ]),
            CellValue::Array(vec![
                CellValue::Str("O"),
                CellValue::Float(15.999),
            ]),
        ];
        let result = SpeciesMass::from_block_rows(&rows).unwrap();
        assert_eq!(result.masses.len(), 2);
    }

    #[test]
    fn test_block_name() {
        assert_eq!(SpeciesMass::BLOCK_NAME, "SPECIES_MASS");
    }
}
