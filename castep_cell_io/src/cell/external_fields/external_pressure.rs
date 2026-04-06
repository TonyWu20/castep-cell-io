use crate::units::PressureUnit;
use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, parse::{FromBlock, FromCellValue}, CResult, Error, query::row_as_f64_n};

/// Represents the external pressure tensor applied to the system.
///
/// Keyword type: Block
///
/// Default unit for the tensor components: GPa (if units are not specified).
/// Default value: No external pressure (zero tensor).
///
/// Example:
/// %BLOCK EXTERNAL_PRESSURE
/// GPa
///     5.0000000000    0.0000000000    0.0000000000
///                     5.0000000000    0.0000000000
///                                     5.0000000000
/// %ENDBLOCK EXTERNAL_PRESSURE
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct ExternalPressure {
    /// Optional unit specification for the pressure tensor.
    /// If None, the default unit (GPa) is implied.
    pub unit: Option<PressureUnit>,
    /// The upper triangular part of the pressure tensor [Rxx, Rxy, Rxz, Ryy, Ryz, Rzz].
    /// Stored in row-major order for the upper triangle.
    pub tensor: [f64; 6],
}

impl FromBlock for ExternalPressure {
    const BLOCK_NAME: &'static str = "EXTERNAL_PRESSURE";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        if rows.is_empty() {
            return Err(Error::Message("EXTERNAL_PRESSURE block is empty".into()));
        }

        let (unit, data_start) = if let CellValue::Array(arr) = &rows[0] {
            if arr.len() == 1 {
                if let Ok(u) = PressureUnit::from_cell_value(&arr[0]) {
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

        if rows.len() < data_start + 3 {
            return Err(Error::Message(
                "EXTERNAL_PRESSURE requires 3 tensor rows".into(),
            ));
        }

        let row1 = row_as_f64_n::<3>(&rows[data_start])?;
        let row2 = row_as_f64_n::<2>(&rows[data_start + 1])?;
        let row3 = row_as_f64_n::<1>(&rows[data_start + 2])?;

        let tensor = [row1[0], row1[1], row1[2], row2[0], row2[1], row3[0]];

        Ok(Self { unit, tensor })
    }
}

impl ToCell for ExternalPressure {
    fn to_cell(&self) -> Cell<'_> {
        let mut block_content = Vec::with_capacity(4);

        if let Some(unit) = &self.unit {
            block_content.push(CellValue::Array(vec![unit.to_cell_value()]));
        }

        block_content.push(CellValue::Array(
            self.tensor[0..3]
                .iter()
                .map(|&val| CellValue::Float(val))
                .collect(),
        ));
        block_content.push(CellValue::Array(
            self.tensor[3..5]
                .iter()
                .map(|&val| CellValue::Float(val))
                .collect(),
        ));
        block_content.push(CellValue::Array(vec![CellValue::Float(self.tensor[5])]));

        Cell::Block("EXTERNAL_PRESSURE", block_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic_without_unit() {
        let pressure = ExternalPressure::builder()
            .tensor([5.0, 0.0, 0.0, 5.0, 0.0, 5.0])
            .build();

        assert_eq!(pressure.unit, None);
        assert_eq!(pressure.tensor, [5.0, 0.0, 0.0, 5.0, 0.0, 5.0]);
    }

    #[test]
    fn test_builder_with_unit() {
        let pressure = ExternalPressure::builder()
            .tensor([10.0, 0.0, 0.0, 10.0, 0.0, 10.0])
            .unit(PressureUnit::GigaPascal)
            .build();

        assert_eq!(pressure.unit, Some(PressureUnit::GigaPascal));
        assert_eq!(pressure.tensor, [10.0, 0.0, 0.0, 10.0, 0.0, 10.0]);
    }

    #[test]
    fn test_builder_different_tensor_values() {
        let pressure = ExternalPressure::builder()
            .tensor([1.0, 2.0, 3.0, 4.0, 5.0, 6.0])
            .unit(PressureUnit::MegaBar)
            .build();

        assert_eq!(pressure.unit, Some(PressureUnit::MegaBar));
        assert_eq!(pressure.tensor, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_builder_method_chaining() {
        let pressure = ExternalPressure::builder()
            .unit(PressureUnit::GigaPascal)
            .tensor([2.5, 0.5, 0.3, 2.5, 0.4, 2.5])
            .build();

        assert_eq!(pressure.unit, Some(PressureUnit::GigaPascal));
        assert_eq!(pressure.tensor, [2.5, 0.5, 0.3, 2.5, 0.4, 2.5]);

        // Verify it can be converted to Cell and back
        let cell = pressure.to_cell();
        assert!(matches!(cell, Cell::Block("EXTERNAL_PRESSURE", _)));
    }
}

