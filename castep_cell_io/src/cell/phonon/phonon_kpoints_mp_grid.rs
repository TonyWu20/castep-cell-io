use castep_cell_fmt::{Cell, CellValue, CResult, Error, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::query::value_as_u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhononKpointsMpGrid(pub [u32; 3]);

impl FromCellValue for PhononKpointsMpGrid {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() == 3 => {
                let grid = [
                    value_as_u32(&arr[0])?,
                    value_as_u32(&arr[1])?,
                    value_as_u32(&arr[2])?,
                ];
                Ok(PhononKpointsMpGrid(grid))
            }
            _ => Err(Error::Message(
                "PhononKpointsMpGrid must be an array of 3 integers".into(),
            )),
        }
    }
}

impl FromKeyValue for PhononKpointsMpGrid {
    const KEY_NAME: &'static str = "PHONON_KPOINT_MP_GRID";
    const KEY_ALIASES: &'static [&'static str] = &["PHONON_KPOINTS_MP_GRID"];

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for PhononKpointsMpGrid {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue(
            "PHONON_KPOINT_MP_GRID",
            CellValue::Array(self.0.iter().map(|&v| CellValue::UInt(v)).collect()),
        )
    }
}

impl ToCellValue for PhononKpointsMpGrid {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Array(self.0.iter().map(|&v| CellValue::UInt(v)).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phonon_kpoints_mp_grid_from_cell_value() {
        let val = CellValue::Array(vec![
            CellValue::UInt(4),
            CellValue::UInt(4),
            CellValue::UInt(4),
        ]);
        let grid = PhononKpointsMpGrid::from_cell_value(&val).unwrap();
        assert_eq!(grid.0, [4, 4, 4]);
    }
}
