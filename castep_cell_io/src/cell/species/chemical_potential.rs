use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, parse::{FromBlock, FromCellValue}, CResult, Error, query::value_as_f64};
use super::Species;
use crate::units::EnergyUnit;

/// Represents a single entry within the CHEMICAL_POTENTIAL block,
/// linking a species to its chemical potential value.
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct ChemicalPotentialEntry {
    /// The species (symbol or atomic number).
    pub species: Species,
    /// The chemical potential value for this species.
    pub value: f64,
}

impl FromCellValue for ChemicalPotentialEntry {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() == 2 => {
                Ok(ChemicalPotentialEntry {
                    species: Species::from_cell_value(&arr[0])?,
                    value: value_as_f64(&arr[1])?,
                })
            }
            _ => Err(Error::Message(
                "ChemicalPotentialEntry must be an array of [species, value]".into(),
            )),
        }
    }
}

impl ToCellValue for ChemicalPotentialEntry {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Array(vec![
            self.species.to_cell_value(),
            CellValue::Float(self.value),
        ])
    }
}

/// Represents the CHEMICAL_POTENTIAL block.
///
/// Defines the chemical potential of each atomic species.
/// Format:
/// %BLOCK CHEMICAL_POTENTIAL
/// [UNITS]
/// CCC1/I1 R1
/// CCC2/I2 R2
/// ...
/// %ENDBLOCK CHEMICAL_POTENTIAL
///
/// 6.11: the optional units line is an energy unit; when omitted, electron
/// volts are assumed.
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct ChemicalPotential {
    /// The unit of the chemical potential values. If `None`, the default (eV) is used.
    pub unit: Option<EnergyUnit>,
    /// The list of species and their chemical potential values.
    #[builder(default)]
    pub entries: Vec<ChemicalPotentialEntry>,
}

impl FromBlock for ChemicalPotential {
    const BLOCK_NAME: &'static str = "CHEMICAL_POTENTIAL";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        if rows.is_empty() {
            return Ok(Self {
                unit: None,
                entries: Vec::new(),
            });
        }

        let (unit, data_start) = if let CellValue::Array(arr) = &rows[0] {
            if arr.len() == 1 {
                if let Ok(u) = EnergyUnit::from_cell_value(&arr[0]) {
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

        let entries = rows[data_start..]
            .iter()
            .map(ChemicalPotentialEntry::from_cell_value)
            .collect::<CResult<Vec<_>>>()?;

        Ok(Self { unit, entries })
    }
}

impl ToCell for ChemicalPotential {
    fn to_cell(&self) -> Cell<'_> {
        let mut block_content = Vec::new();

        if let Some(ref u) = self.unit {
            block_content.push(CellValue::Array(vec![u.to_cell_value()]));
        }

        block_content.extend(
            self.entries
                .iter()
                .map(|entry| entry.to_cell_value()),
        );

        Cell::Block("CHEMICAL_POTENTIAL", block_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chemical_potential_entry_from_cell_value() {
        let val = CellValue::Array(vec![CellValue::Str("O"), CellValue::Float(-0.5)]);
        let entry = ChemicalPotentialEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.species, Species::Symbol("O".to_string()));
        assert_eq!(entry.value, -0.5);
    }

    #[test]
    fn test_chemical_potential_entry_insufficient_elements() {
        let val = CellValue::Array(vec![CellValue::Str("O")]);
        assert!(ChemicalPotentialEntry::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_chemical_potential_empty() {
        let result = ChemicalPotential::from_block_rows(&[]).unwrap();
        assert!(result.unit.is_none());
        assert_eq!(result.entries.len(), 0);
    }

    #[test]
    fn test_chemical_potential_with_unit() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Str("ha")]),
            CellValue::Array(vec![CellValue::Str("Fe"), CellValue::Float(0.1)]),
        ];
        let result = ChemicalPotential::from_block_rows(&rows).unwrap();
        assert!(result.unit.is_some());
        assert_eq!(result.entries.len(), 1);
    }

    #[test]
    fn test_chemical_potential_without_unit() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Str("O"),
                CellValue::Float(-0.5),
            ]),
        ];
        let result = ChemicalPotential::from_block_rows(&rows).unwrap();
        assert!(result.unit.is_none());
        assert_eq!(result.entries.len(), 1);
    }

    #[test]
    fn test_block_name() {
        assert_eq!(ChemicalPotential::BLOCK_NAME, "CHEMICAL_POTENTIAL");
    }

    #[test]
    fn test_chemical_potential_builder() {
        let entry = ChemicalPotentialEntry::builder()
            .species(Species::Symbol("O".to_string()))
            .value(-0.5)
            .build();

        let cp = ChemicalPotential::builder()
            .unit(EnergyUnit::Hartree)
            .entries(vec![entry])
            .build();

        assert_eq!(cp.unit, Some(EnergyUnit::Hartree));
        assert_eq!(cp.entries.len(), 1);
    }
}
