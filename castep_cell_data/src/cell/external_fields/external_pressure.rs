use crate::units::PressureUnit;
use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, parse::{FromBlock, FromCellValue}, CResult, Error, query::row_as_f64_n};

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
#[derive(Debug, Clone, PartialEq)]
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
    fn to_cell(&self) -> Cell {
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

