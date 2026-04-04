use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_u32;

/// Specifies the Monkhorst-Pack grid parameters for generating SCF k-points.
///
/// Keyword type: Block (but actually a key-value with 3 integers)
///
/// Format: `kpoints_mp_grid 3 4 6`
///
/// Example:
/// KPOINTS_MP_GRID : 3 4 6
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KpointsMpGrid(pub [u32; 3]);

impl FromCellValue for KpointsMpGrid {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() == 3 => {
                let grid = [
                    value_as_u32(&arr[0])?,
                    value_as_u32(&arr[1])?,
                    value_as_u32(&arr[2])?,
                ];
                Ok(KpointsMpGrid(grid))
            }
            _ => Err(Error::Message(
                "KpointsMpGrid must be an array of 3 integers".into(),
            )),
        }
    }
}

impl FromKeyValue for KpointsMpGrid {
    const KEY_NAME: &'static str = "KPOINTS_MP_GRID";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for KpointsMpGrid {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue(
            "KPOINTS_MP_GRID",
            CellValue::Array(
                self.0
                    .iter()
                    .map(|&v| CellValue::UInt(v))
                    .collect(),
            ),
        )
    }
}

impl ToCellValue for KpointsMpGrid {
    fn to_cell_value(&self) -> CellValue {
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
    fn test_kpoints_mp_grid_from_cell_value() {
        let val = CellValue::Array(vec![
            CellValue::UInt(3),
            CellValue::UInt(4),
            CellValue::UInt(6),
        ]);
        let grid = KpointsMpGrid::from_cell_value(&val).unwrap();
        assert_eq!(grid.0, [3, 4, 6]);
    }

    #[test]
    fn test_kpoints_mp_grid_insufficient_elements() {
        let val = CellValue::Array(vec![
            CellValue::UInt(3),
            CellValue::UInt(4),
        ]);
        assert!(KpointsMpGrid::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_kpoints_mp_grid_too_many_elements() {
        let val = CellValue::Array(vec![
            CellValue::UInt(3),
            CellValue::UInt(4),
            CellValue::UInt(6),
            CellValue::UInt(2),
        ]);
        assert!(KpointsMpGrid::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_kpoints_mp_grid_non_array() {
        let val = CellValue::UInt(3);
        assert!(KpointsMpGrid::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_kpoints_mp_grid_to_cell_value() {
        let grid = KpointsMpGrid([3, 4, 6]);
        let val = grid.to_cell_value();
        match val {
            CellValue::Array(arr) => {
                assert_eq!(arr.len(), 3);
                assert_eq!(arr[0], CellValue::UInt(3));
                assert_eq!(arr[1], CellValue::UInt(4));
                assert_eq!(arr[2], CellValue::UInt(6));
            }
            _ => panic!("Expected Array"),
        }
    }

    #[test]
    fn test_kpoints_mp_grid_to_cell() {
        let grid = KpointsMpGrid([3, 4, 6]);
        let cell = grid.to_cell();
        match cell {
            Cell::KeyValue(name, CellValue::Array(arr)) => {
                assert_eq!(name, "KPOINTS_MP_GRID");
                assert_eq!(arr.len(), 3);
                assert_eq!(arr[0], CellValue::UInt(3));
                assert_eq!(arr[1], CellValue::UInt(4));
                assert_eq!(arr[2], CellValue::UInt(6));
            }
            _ => panic!("Expected Cell::KeyValue with Array"),
        }
    }

    #[test]
    fn test_kpoints_mp_grid_round_trip() {
        let original = KpointsMpGrid([2, 3, 4]);
        let cell_value = CellValue::Array(vec![
            CellValue::UInt(2),
            CellValue::UInt(3),
            CellValue::UInt(4),
        ]);
        let parsed = KpointsMpGrid::from_cell_value(&cell_value).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_kpoints_mp_grid_from_key_value() {
        let val = CellValue::Array(vec![
            CellValue::UInt(5),
            CellValue::UInt(5),
            CellValue::UInt(5),
        ]);
        let grid = KpointsMpGrid::from_cell_value_kv(&val).unwrap();
        assert_eq!(grid.0, [5, 5, 5]);
    }
}
