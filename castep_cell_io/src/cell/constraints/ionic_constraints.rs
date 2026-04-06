use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, FromCellValue, FromBlock, CResult, Error, query::value_as_f64};

use crate::cell::species::Species;

/// Represents a single constraint entry within the IONIC_CONSTRAINTS block.
///
/// # Format:
///   Ic CCC/Ic In Rix Riy Riz
/// Where:
/// - Ic: Constraint number
/// - CCC/Ic: Species (symbol or atomic number)
/// - In: Ion number within the species
/// - Rix, Riy, Riz: Coefficients for x, y, z Cartesian coordinates
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct IonicConstraintEntry {
    /// The constraint number.
    pub constraint_number: u32,
    /// The species of the ion.
    pub species: Species,
    /// The ion number within the species (1-based index).
    pub ion_number: u32,
    /// Coefficients for the Cartesian coordinates [x, y, z].
    pub coefficients: [f64; 3],
}

impl FromCellValue for IonicConstraintEntry {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() >= 6 => {
                let constraint_number = castep_cell_fmt::query::value_as_u32(&arr[0])?;
                let species = Species::from_cell_value(&arr[1])?;
                let ion_number = castep_cell_fmt::query::value_as_u32(&arr[2])?;
                let coefficients = [
                    value_as_f64(&arr[3])?,
                    value_as_f64(&arr[4])?,
                    value_as_f64(&arr[5])?,
                ];
                Ok(IonicConstraintEntry {
                    constraint_number,
                    species,
                    ion_number,
                    coefficients,
                })
            }
            _ => Err(castep_cell_fmt::Error::Message(
                "IonicConstraintEntry must be an array of at least 6 elements".into(),
            )),
        }
    }
}

impl ToCellValue for IonicConstraintEntry {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Array(
            vec![
                CellValue::UInt(self.constraint_number),
                self.species.to_cell_value(),
                CellValue::UInt(self.ion_number),
            ]
            .into_iter()
            .chain(self.coefficients.into_iter().map(CellValue::Float))
            .collect(),
        )
    }
}

/// Represents the IONIC_CONSTRAINTS block.
///
/// Defines linear constraints on changes to Cartesian ionic positions.
/// Format:
/// %BLOCK IONIC_CONSTRAINTS
/// I1 CCC1/I1 In1 R1i R1j R1k
/// I2 CCC2/I2 In2 R2i R2j R2k
/// ...
/// %ENDBLOCK IONIC_CONSTRAINTS
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct IonicConstraints {
    /// The list of constraint entries.
    #[builder(default)]
    pub constraints: Vec<IonicConstraintEntry>,
}

impl FromBlock for IonicConstraints {
    const BLOCK_NAME: &'static str = "IONIC_CONSTRAINTS";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        let constraints = rows
            .iter()
            .map(IonicConstraintEntry::from_cell_value)
            .collect::<CResult<Vec<_>>>()?;
        Ok(IonicConstraints { constraints })
    }
}

impl ToCell for IonicConstraints {
    fn to_cell(&self) -> Cell<'_> {
        Cell::Block(
            "IONIC_CONSTRAINTS",
            self.constraints
                .iter()
                .map(|entry| entry.to_cell_value())
                .collect::<Vec<CellValue>>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_fmt::{CellValue, FromCellValue, parse::FromBlock};

    #[test]
    fn test_ionic_constraint_entry_basic() {
        let val = CellValue::Array(vec![
            CellValue::UInt(1),
            CellValue::Str("Fe"),
            CellValue::UInt(1),
            CellValue::Float(1.0),
            CellValue::Float(0.0),
            CellValue::Float(0.0),
        ]);
        let entry = IonicConstraintEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.constraint_number, 1);
        assert_eq!(entry.species, Species::Symbol("Fe".to_string()));
        assert_eq!(entry.ion_number, 1);
        assert_eq!(entry.coefficients, [1.0, 0.0, 0.0]);
    }

    #[test]
    fn test_ionic_constraint_entry_all_coefficients() {
        let val = CellValue::Array(vec![
            CellValue::UInt(2),
            CellValue::Str("O"),
            CellValue::UInt(3),
            CellValue::Float(0.5),
            CellValue::Float(0.5),
            CellValue::Float(1.0),
        ]);
        let entry = IonicConstraintEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.constraint_number, 2);
        assert_eq!(entry.coefficients, [0.5, 0.5, 1.0]);
    }

    #[test]
    fn test_ionic_constraint_entry_insufficient_elements() {
        let val = CellValue::Array(vec![
            CellValue::UInt(1),
            CellValue::Str("Fe"),
            CellValue::UInt(1),
        ]);
        assert!(IonicConstraintEntry::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_ionic_constraint_entry_not_array() {
        let val = CellValue::Str("invalid");
        assert!(IonicConstraintEntry::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_ionic_constraints_single_entry() {
        let rows = vec![CellValue::Array(vec![
            CellValue::UInt(1),
            CellValue::Str("Fe"),
            CellValue::UInt(1),
            CellValue::Float(1.0),
            CellValue::Float(0.0),
            CellValue::Float(0.0),
        ])];
        let result = IonicConstraints::from_block_rows(&rows).unwrap();
        assert_eq!(result.constraints.len(), 1);
        assert_eq!(result.constraints[0].constraint_number, 1);
    }

    #[test]
    fn test_ionic_constraints_multiple_entries() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::UInt(1),
                CellValue::Str("Fe"),
                CellValue::UInt(1),
                CellValue::Float(1.0),
                CellValue::Float(0.0),
                CellValue::Float(0.0),
            ]),
            CellValue::Array(vec![
                CellValue::UInt(2),
                CellValue::Str("O"),
                CellValue::UInt(2),
                CellValue::Float(0.0),
                CellValue::Float(1.0),
                CellValue::Float(0.0),
            ]),
        ];
        let result = IonicConstraints::from_block_rows(&rows).unwrap();
        assert_eq!(result.constraints.len(), 2);
        assert_eq!(result.constraints[0].constraint_number, 1);
        assert_eq!(result.constraints[1].constraint_number, 2);
    }

    #[test]
    fn test_ionic_constraints_empty() {
        let result = IonicConstraints::from_block_rows(&[]).unwrap();
        assert_eq!(result.constraints.len(), 0);
    }

    #[test]
    fn test_ionic_constraints_block_name() {
        assert_eq!(IonicConstraints::BLOCK_NAME, "IONIC_CONSTRAINTS");
    }

    // Builder pattern tests
    #[test]
    fn test_ionic_constraint_entry_builder_basic() {
        let entry = IonicConstraintEntry::builder()
            .constraint_number(1)
            .species(Species::Symbol("Fe".to_string()))
            .ion_number(1)
            .coefficients([1.0, 0.0, 0.0])
            .build();

        assert_eq!(entry.constraint_number, 1);
        assert_eq!(entry.species, Species::Symbol("Fe".to_string()));
        assert_eq!(entry.ion_number, 1);
        assert_eq!(entry.coefficients, [1.0, 0.0, 0.0]);
    }

    #[test]
    fn test_ionic_constraints_builder_empty() {
        let constraints = IonicConstraints::builder().build();
        assert_eq!(constraints.constraints.len(), 0);
    }

    #[test]
    fn test_ionic_constraints_builder_single_entry() {
        let entry = IonicConstraintEntry::builder()
            .constraint_number(1)
            .species(Species::Symbol("Fe".to_string()))
            .ion_number(1)
            .coefficients([1.0, 0.0, 0.0])
            .build();

        let constraints = IonicConstraints::builder()
            .constraints(vec![entry.clone()])
            .build();

        assert_eq!(constraints.constraints.len(), 1);
        assert_eq!(constraints.constraints[0].constraint_number, 1);
    }

    #[test]
    fn test_ionic_constraints_builder_multiple_entries() {
        let entry1 = IonicConstraintEntry::builder()
            .constraint_number(1)
            .species(Species::Symbol("Fe".to_string()))
            .ion_number(1)
            .coefficients([1.0, 0.0, 0.0])
            .build();

        let entry2 = IonicConstraintEntry::builder()
            .constraint_number(2)
            .species(Species::Symbol("O".to_string()))
            .ion_number(2)
            .coefficients([0.0, 1.0, 0.0])
            .build();

        let constraints = IonicConstraints::builder()
            .constraints(vec![entry1.clone(), entry2.clone()])
            .build();

        assert_eq!(constraints.constraints.len(), 2);
        assert_eq!(constraints.constraints[0].constraint_number, 1);
        assert_eq!(constraints.constraints[1].constraint_number, 2);
    }

    #[test]
    fn test_ionic_constraints_builder_set_entire_vec() {
        let entries = vec![
            IonicConstraintEntry::builder()
                .constraint_number(1)
                .species(Species::Symbol("Fe".to_string()))
                .ion_number(1)
                .coefficients([1.0, 0.0, 0.0])
                .build(),
            IonicConstraintEntry::builder()
                .constraint_number(2)
                .species(Species::Symbol("O".to_string()))
                .ion_number(2)
                .coefficients([0.0, 1.0, 0.0])
                .build(),
        ];

        let constraints = IonicConstraints::builder()
            .constraints(entries)
            .build();

        assert_eq!(constraints.constraints.len(), 2);
        assert_eq!(constraints.constraints[0].constraint_number, 1);
        assert_eq!(constraints.constraints[1].constraint_number, 2);
    }

    #[test]
    fn test_ionic_constraints_builder_method_chaining() {
        let entry1 = IonicConstraintEntry::builder()
            .constraint_number(1)
            .species(Species::Symbol("Fe".to_string()))
            .ion_number(1)
            .coefficients([1.0, 0.0, 0.0])
            .build();

        let entry2 = IonicConstraintEntry::builder()
            .constraint_number(2)
            .species(Species::Symbol("O".to_string()))
            .ion_number(2)
            .coefficients([0.0, 1.0, 0.0])
            .build();

        let entry3 = IonicConstraintEntry::builder()
            .constraint_number(3)
            .species(Species::Symbol("C".to_string()))
            .ion_number(3)
            .coefficients([0.0, 0.0, 1.0])
            .build();

        let constraints = IonicConstraints::builder()
            .constraints(vec![entry1, entry2, entry3])
            .build();

        assert_eq!(constraints.constraints.len(), 3);
        assert_eq!(constraints.constraints[0].constraint_number, 1);
        assert_eq!(constraints.constraints[1].constraint_number, 2);
        assert_eq!(constraints.constraints[2].constraint_number, 3);
        assert_eq!(constraints.constraints[2].species, Species::Symbol("C".to_string()));
    }
}

