use castep_cell_io::{CellValue, parse::{FromBlock, FromCellValue}, CResult, Error};
use super::{AtomHubbardU, HubbardU, HubbardUUnit};

impl FromBlock for HubbardU {
    const BLOCK_NAME: &'static str = "HUBBARD_U";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        if rows.is_empty() {
            return Ok(Self {
                unit: None,
                atom_u_values: Vec::new(),
            });
        }

        let (unit, data_start) = if let CellValue::Array(arr) = &rows[0] {
            if arr.len() == 1 {
                if let Ok(u) = HubbardUUnit::from_cell_value(&arr[0]) {
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

        let atom_u_values = rows[data_start..]
            .iter()
            .map(|row| AtomHubbardU::from_block_rows(&[row.clone()]))
            .collect::<CResult<Vec<_>>>()?;

        Ok(Self {
            unit,
            atom_u_values,
        })
    }
}
