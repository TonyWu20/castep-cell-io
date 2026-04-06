use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_u32;

/// Number of excited states to calculate in TDDFT.
///
/// Keyword type: Integer
///
/// Default: 0
///
/// Example:
/// TDDFT_NUM_STATES : 10
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TddftNumStates(pub u32);

impl FromCellValue for TddftNumStates {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_u32(value)?))
    }
}

impl FromKeyValue for TddftNumStates {
    const KEY_NAME: &'static str = "TDDFT_NUM_STATES";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for TddftNumStates {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("TDDFT_NUM_STATES", CellValue::UInt(self.0))
    }
}

impl ToCellValue for TddftNumStates {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::UInt(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_fmt::CellValue;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::UInt(10);
        assert_eq!(TddftNumStates::from_cell_value(&val).unwrap(), TddftNumStates(10));
    }

    #[test]
    fn test_default() {
        assert_eq!(TddftNumStates::default(), TddftNumStates(0));
    }

    #[test]
    fn test_key_name() {
        assert_eq!(TddftNumStates::KEY_NAME, "TDDFT_NUM_STATES");
    }

    #[test]
    fn test_to_cell_value() {
        assert_eq!(
            TddftNumStates(10).to_cell_value(),
            CellValue::UInt(10)
        );
    }

    #[test]
    fn test_to_cell() {
        let cell = TddftNumStates(10).to_cell();
        match cell {
            Cell::KeyValue(key, CellValue::UInt(val)) => {
                assert_eq!(key, "TDDFT_NUM_STATES");
                assert_eq!(val, 10);
            }
            _ => panic!("Expected KeyValue cell with UInt"),
        }
    }

    #[test]
    fn test_round_trip() {
        let original = TddftNumStates(42);
        let cell_value = original.to_cell_value();
        let parsed = TddftNumStates::from_cell_value(&cell_value).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_ordering() {
        assert!(TddftNumStates(5) < TddftNumStates(10));
        assert!(TddftNumStates(10) > TddftNumStates(5));
        assert_eq!(TddftNumStates(10), TddftNumStates(10));
    }
}
