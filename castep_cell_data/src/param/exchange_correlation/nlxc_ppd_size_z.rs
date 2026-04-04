use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult};
use castep_cell_io::query::value_as_u32;

/// Plane-wave density grid size in Z direction for non-local exchange-correlation.
///
/// Keyword type: Integer
///
/// Aliases: PPD_SIZE_Z
///
/// Example:
/// NLXC_PPD_SIZE_Z : 64
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NlxcPpdSizeZ(pub u32);

impl FromCellValue for NlxcPpdSizeZ {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for NlxcPpdSizeZ {
    const KEY_NAME: &'static str = "PPD_SIZE_Z";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for NlxcPpdSizeZ {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("NLXC_PPD_SIZE_Z", CellValue::UInt(self.0))
    }
}

impl ToCellValue for NlxcPpdSizeZ {
    fn to_cell_value(&self) -> CellValue {
        CellValue::UInt(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::UInt(64);
        assert_eq!(NlxcPpdSizeZ::from_cell_value(&val).unwrap().0, 64);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(NlxcPpdSizeZ::KEY_NAME, "PPD_SIZE_Z");
    }

    #[test]
    fn test_to_cell() {
        let size = NlxcPpdSizeZ(64);
        match size.to_cell() {
            Cell::KeyValue(key, CellValue::UInt(val)) => {
                assert_eq!(key, "NLXC_PPD_SIZE_Z");
                assert_eq!(val, 64);
            }
            _ => panic!("Expected KeyValue cell"),
        }
    }
}
