use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_fmt::parse::FromKeyValue;
use castep_cell_fmt::query::value_as_bool;

/// Specifies whether CASTEP should print memory estimates during the run.
///
/// Keyword type: Logical
///
/// Default: TRUE
///
/// Example:
/// PRINT_MEMORY_USAGE : FALSE
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintMemoryUsage(pub bool);

impl Default for PrintMemoryUsage {
    fn default() -> Self {
        Self(true) // Default is TRUE
    }
}

impl FromKeyValue for PrintMemoryUsage {
    const KEY_NAME: &'static str = "PRINT_MEMORY_USAGE";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl ToCell for PrintMemoryUsage {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PRINT_MEMORY_USAGE", CellValue::Bool(self.0))
    }
}

impl ToCellValue for PrintMemoryUsage {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value_true() {
        let val = CellValue::Bool(true);
        assert_eq!(PrintMemoryUsage::from_cell_value_kv(&val).unwrap().0, true);
    }

    #[test]
    fn test_from_cell_value_false() {
        let val = CellValue::Bool(false);
        assert_eq!(PrintMemoryUsage::from_cell_value_kv(&val).unwrap().0, false);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(PrintMemoryUsage::KEY_NAME, "PRINT_MEMORY_USAGE");
    }
}

