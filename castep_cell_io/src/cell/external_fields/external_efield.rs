use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, parse::{FromBlock, FromCellValue}, CResult, Error, query::row_as_f64_n};
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
///
/// # Builder Example
/// ```
/// use castep_cell_io::cell::external_fields::ExternalEfield;
/// use castep_cell_io::units::EFieldUnit;
///
/// let efield = ExternalEfield::builder()
///     .field_vector([0.0, 0.0, 0.1])
///     .unit(EFieldUnit::HartreePerBohrPerE)
///     .build();
/// ```
#[derive(Debug, Clone, PartialEq, bon::Builder)]
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
    fn to_cell(&self) -> Cell<'_> {
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
    use castep_cell_fmt::CellValue;

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

    // Builder pattern tests
    #[test]
    fn test_builder_basic_without_unit() {
        let efield = ExternalEfield::builder()
            .field_vector([0.0, 0.0, 0.1])
            .build();

        assert_eq!(efield.unit, None);
        assert_eq!(efield.field_vector, [0.0, 0.0, 0.1]);
    }

    #[test]
    fn test_builder_with_unit() {
        let efield = ExternalEfield::builder()
            .unit(EFieldUnit::EvPerAngPerE)
            .field_vector([0.1, 0.2, 0.3])
            .build();

        assert_eq!(efield.unit, Some(EFieldUnit::EvPerAngPerE));
        assert_eq!(efield.field_vector, [0.1, 0.2, 0.3]);
    }

    #[test]
    fn test_builder_different_field_vectors() {
        let efield1 = ExternalEfield::builder()
            .field_vector([1.0, 0.0, 0.0])
            .build();

        let efield2 = ExternalEfield::builder()
            .field_vector([0.0, 1.0, 0.0])
            .build();

        let efield3 = ExternalEfield::builder()
            .field_vector([0.0, 0.0, 1.0])
            .build();

        assert_eq!(efield1.field_vector, [1.0, 0.0, 0.0]);
        assert_eq!(efield2.field_vector, [0.0, 1.0, 0.0]);
        assert_eq!(efield3.field_vector, [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_builder_method_chaining() {
        let efield = ExternalEfield::builder()
            .field_vector([0.5, 0.5, 0.5])
            .unit(EFieldUnit::HartreePerBohrPerE)
            .build();

        assert_eq!(efield.unit, Some(EFieldUnit::HartreePerBohrPerE));
        assert_eq!(efield.field_vector, [0.5, 0.5, 0.5]);

        // Verify it can be converted to Cell
        let cell = efield.to_cell();
        if let Cell::Block(name, rows) = cell {
            assert_eq!(name, "EXTERNAL_EFIELD");
            assert_eq!(rows.len(), 2); // unit + field_vector
        } else {
            panic!("Expected Cell::Block");
        }
    }
}
