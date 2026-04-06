use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_fmt::parse::FromKeyValue;
use castep_cell_fmt::query::value_as_bool;

/// Specifies whether or not timing information will be printed.
///
/// Keyword type: Logical
///
/// Default: TRUE
///
/// Example:
/// PRINT_CLOCK : TRUE
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintClock(pub bool);

impl Default for PrintClock {
    fn default() -> Self {
        Self(true) // Default is TRUE
    }
}

impl FromKeyValue for PrintClock {
    const KEY_NAME: &'static str = "PRINT_CLOCK";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl ToCell for PrintClock {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("PRINT_CLOCK", CellValue::Bool(self.0))
    }
}

impl ToCellValue for PrintClock {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Bool(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value_true() {
        let val = CellValue::Bool(true);
        assert_eq!(PrintClock::from_cell_value_kv(&val).unwrap().0, true);
    }

    #[test]
    fn test_from_cell_value_false() {
        let val = CellValue::Bool(false);
        assert_eq!(PrintClock::from_cell_value_kv(&val).unwrap().0, false);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(PrintClock::KEY_NAME, "PRINT_CLOCK");
    }
}

