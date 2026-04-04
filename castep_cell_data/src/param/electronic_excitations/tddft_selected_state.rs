use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_u32;

/// Index of the excited state to calculate properties for in TDDFT.
///
/// Keyword type: Integer
///
/// Default: 0
///
/// Example:
/// TDDFT_SELECTED_STATE : 3
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TddftSelectedState(pub u32);

impl FromCellValue for TddftSelectedState {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for TddftSelectedState {
    const KEY_NAME: &'static str = "TDDFT_SELECTED_STATE";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for TddftSelectedState {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("TDDFT_SELECTED_STATE", CellValue::UInt(self.0))
    }
}

impl ToCellValue for TddftSelectedState {
    fn to_cell_value(&self) -> CellValue {
        CellValue::UInt(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_io::CellValue;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::UInt(3);
        assert_eq!(TddftSelectedState::from_cell_value(&val).unwrap(), TddftSelectedState(3));
    }

    #[test]
    fn test_default() {
        assert_eq!(TddftSelectedState::default(), TddftSelectedState(0));
    }

    #[test]
    fn test_key_name() {
        assert_eq!(TddftSelectedState::KEY_NAME, "TDDFT_SELECTED_STATE");
    }

    #[test]
    fn test_to_cell_value() {
        assert_eq!(
            TddftSelectedState(3).to_cell_value(),
            CellValue::UInt(3)
        );
    }

    #[test]
    fn test_to_cell() {
        let cell = TddftSelectedState(3).to_cell();
        match cell {
            Cell::KeyValue(key, CellValue::UInt(val)) => {
                assert_eq!(key, "TDDFT_SELECTED_STATE");
                assert_eq!(val, 3);
            }
            _ => panic!("Expected KeyValue cell with UInt"),
        }
    }

    #[test]
    fn test_round_trip() {
        let original = TddftSelectedState(7);
        let cell_value = original.to_cell_value();
        let parsed = TddftSelectedState::from_cell_value(&cell_value).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_ordering() {
        assert!(TddftSelectedState(2) < TddftSelectedState(5));
        assert!(TddftSelectedState(5) > TddftSelectedState(2));
        assert_eq!(TddftSelectedState(5), TddftSelectedState(5));
    }
}
