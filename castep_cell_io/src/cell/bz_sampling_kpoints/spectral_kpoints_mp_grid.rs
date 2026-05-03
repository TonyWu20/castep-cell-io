use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_u32;

/// Specifies the Monkhorst-Pack grid for spectral k-point sampling.
///
/// Keyword type: Key-value with 3 integers
///
/// Format: `SPECTRAL_KPOINTS_MP_GRID : n1 n2 n3`
///
/// Example:
/// SPECTRAL_KPOINTS_MP_GRID : 3 4 6
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpectralKpointsMpGrid(pub [u32; 3]);

impl FromCellValue for SpectralKpointsMpGrid {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() == 3 => {
                let grid = [
                    value_as_u32(&arr[0])?,
                    value_as_u32(&arr[1])?,
                    value_as_u32(&arr[2])?,
                ];
                Ok(SpectralKpointsMpGrid(grid))
            }
            _ => Err(Error::Message(
                "SpectralKpointsMpGrid must be an array of 3 integers".into(),
            )),
        }
    }
}

impl FromKeyValue for SpectralKpointsMpGrid {
    const KEY_NAME: &'static str = "SPECTRAL_KPOINT_MP_GRID";
    const KEY_ALIASES: &'static [&'static str] = &["SPECTRAL_KPOINTS_MP_GRID"];

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for SpectralKpointsMpGrid {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue(
            "SPECTRAL_KPOINT_MP_GRID",
            CellValue::Array(
                self.0
                    .iter()
                    .map(|&v| CellValue::UInt(v))
                    .collect(),
            ),
        )
    }
}

impl ToCellValue for SpectralKpointsMpGrid {
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
    fn test_spectral_kpoints_mp_grid_from_cell_value() {
        let val = CellValue::Array(vec![
            CellValue::UInt(3),
            CellValue::UInt(4),
            CellValue::UInt(6),
        ]);
        let grid = SpectralKpointsMpGrid::from_cell_value(&val).unwrap();
        assert_eq!(grid.0, [3, 4, 6]);
    }

    #[test]
    fn test_spectral_kpoints_mp_grid_insufficient_elements() {
        let val = CellValue::Array(vec![
            CellValue::UInt(3),
            CellValue::UInt(4),
        ]);
        assert!(SpectralKpointsMpGrid::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_spectral_kpoints_mp_grid_too_many_elements() {
        let val = CellValue::Array(vec![
            CellValue::UInt(3),
            CellValue::UInt(4),
            CellValue::UInt(6),
            CellValue::UInt(2),
        ]);
        assert!(SpectralKpointsMpGrid::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_spectral_kpoints_mp_grid_to_cell_value() {
        let grid = SpectralKpointsMpGrid([3, 4, 6]);
        let val = grid.to_cell_value();
        match val {
            CellValue::Array(arr) => {
                assert_eq!(arr.len(), 3);
                assert_eq!(arr[0], CellValue::UInt(3));
            }
            _ => panic!("Expected Array"),
        }
    }

    #[test]
    fn test_spectral_kpoints_mp_grid_to_cell() {
        let grid = SpectralKpointsMpGrid([3, 4, 6]);
        let cell = grid.to_cell();
        match cell {
            Cell::KeyValue(name, CellValue::Array(arr)) => {
                assert_eq!(name, "SPECTRAL_KPOINT_MP_GRID");
                assert_eq!(arr.len(), 3);
            }
            _ => panic!("Expected Cell::KeyValue with Array"),
        }
    }

    #[test]
    fn test_spectral_kpoints_mp_grid_round_trip() {
        let original = SpectralKpointsMpGrid([2, 3, 4]);
        let val = original.to_cell_value();
        let parsed = SpectralKpointsMpGrid::from_cell_value(&val).unwrap();
        assert_eq!(parsed, original);
    }
}
