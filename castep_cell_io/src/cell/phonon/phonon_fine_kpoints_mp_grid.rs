use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::query::value_as_u32;
use castep_cell_fmt::{CResult, Error};

/// Specifies the fine Monkhorst-Pack grid parameters for phonon calculations.
///
/// Keyword type: Key-value with 3 integers
///
/// Format: `phonon_fine_kpoints_mp_grid 3 4 6`
///
/// Example:
/// PHONON_FINE_KPOINTS_MP_GRID : 2 2 2
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhononFineKpointsMpGrid(pub [u32; 3]);

impl FromCellValue for PhononFineKpointsMpGrid {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() == 3 => {
                let grid = [
                    value_as_u32(&arr[0])?,
                    value_as_u32(&arr[1])?,
                    value_as_u32(&arr[2])?,
                ];
                Ok(PhononFineKpointsMpGrid(grid))
            }
            _ => Err(Error::Message(
                "PhononFineKpointsMpGrid must be an array of 3 integers".into(),
            )),
        }
    }
}

impl FromKeyValue for PhononFineKpointsMpGrid {
    const KEY_NAME: &'static str = "PHONON_FINE_KPOINT_MP_GRID";
    const KEY_ALIASES: &'static [&'static str] = &["PHONON_FINE_KPOINTS_MP_GRID"];

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for PhononFineKpointsMpGrid {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue(
            "PHONON_FINE_KPOINT_MP_GRID",
            CellValue::Array(
                self.0
                    .iter()
                    .map(|&v| CellValue::UInt(v))
                    .collect(),
            ),
        )
    }
}

impl ToCellValue for PhononFineKpointsMpGrid {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Array(
            self.0
                .iter()
                .map(|&v| CellValue::UInt(v))
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phonon_fine_kpoints_mp_grid_from_cell_value() {
        let val = CellValue::Array(vec![
            CellValue::UInt(2),
            CellValue::UInt(2),
            CellValue::UInt(2),
        ]);
        let grid = PhononFineKpointsMpGrid::from_cell_value(&val).unwrap();
        assert_eq!(grid.0, [2, 2, 2]);
    }
}
