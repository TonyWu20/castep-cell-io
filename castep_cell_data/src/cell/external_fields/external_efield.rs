use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, parse::{FromBlock, FromCellValue}, CResult, Error, query::row_as_f64_n};
use crate::units::EFieldUnit;

/// Represents the electric field vector in Cartesian coordinates.
///
/// Keyword type: Block
///
/// Default unit for the field vector: eV/Ang/e (if units are not specified).
///
/// Example:
/// %BLOCK EXTERNAL_EFIELD
/// HARTREE/BOHR/E
/// 0.0 0.0 0.1
/// %ENDBLOCK EXTERNAL_EFIELD
#[derive(Debug, Clone, PartialEq)]
pub struct ExternalEfield {
    /// Optional unit specification for the electric field.
    /// If None, the default unit (eV/Ang/e) is implied.
    pub unit: Option<EFieldUnit>,
    /// The electric field vector components [Ex, Ey, Ez] in Cartesian coordinates.
    pub field_vector: [f64; 3],
}

impl FromBlock for ExternalEfield {
    const BLOCK_NAME: &'static str = "EXTERNAL_EFIELD";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        if rows.is_empty() {
            return Err(Error::Message("EXTERNAL_EFIELD block is empty".into()));
        }

        let (unit, data_start) = if let CellValue::Array(arr) = &rows[0] {
            if arr.len() == 1 {
                if let Ok(u) = EFieldUnit::from_cell_value(&arr[0]) {
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

        if rows.len() < data_start + 1 {
            return Err(Error::Message(
                "EXTERNAL_EFIELD requires field vector data".into(),
            ));
        }

        let field_vector = row_as_f64_n::<3>(&rows[data_start])?;

        Ok(Self { unit, field_vector })
    }
}

impl ToCell for ExternalEfield {
    fn to_cell(&self) -> Cell {
        let mut block_content = Vec::new();
        if let Some(u) = &self.unit {
            block_content.push(CellValue::Array(vec![u.to_cell_value()]));
        }
        block_content.push(CellValue::Array(
            self.field_vector.into_iter().map(CellValue::Float).collect(),
        ));
        Cell::Block("EXTERNAL_EFIELD", block_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_io::CellValue;

    #[test]
    fn test_external_efield_empty() {
        let result = ExternalEfield::from_block_rows(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_external_efield_with_unit() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Str("eV/Ang/e")]),
            CellValue::Array(vec![
                CellValue::Float(0.0),
                CellValue::Float(0.0),
                CellValue::Float(0.1),
            ]),
        ];
        let result = ExternalEfield::from_block_rows(&rows).unwrap();
        assert!(result.unit.is_some());
        assert_eq!(result.field_vector, [0.0, 0.0, 0.1]);
    }

    #[test]
    fn test_external_efield_without_unit() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Float(0.0),
                CellValue::Float(0.0),
                CellValue::Float(0.1),
            ]),
        ];
        let result = ExternalEfield::from_block_rows(&rows).unwrap();
        assert!(result.unit.is_none());
        assert_eq!(result.field_vector, [0.0, 0.0, 0.1]);
    }

    #[test]
    fn test_external_efield_nonzero_field() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Float(0.1),
                CellValue::Float(0.2),
                CellValue::Float(0.3),
            ]),
        ];
        let result = ExternalEfield::from_block_rows(&rows).unwrap();
        assert_eq!(result.field_vector, [0.1, 0.2, 0.3]);
    }

    #[test]
    fn test_block_name() {
        assert_eq!(ExternalEfield::BLOCK_NAME, "EXTERNAL_EFIELD");
    }
}
