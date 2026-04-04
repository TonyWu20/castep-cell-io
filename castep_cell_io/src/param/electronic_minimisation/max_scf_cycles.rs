use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, FromKeyValue, CResult};
use castep_cell_fmt::query::value_as_i32;

/// Determines the maximum number of SCF cycles performed in an electronic minimization.
///
/// Keyword type: Integer
///
/// Default: 30
///
/// Example:
/// MAX_SCF_CYCLES : 20
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaxScfCycles(pub i32);

impl Default for MaxScfCycles {
    fn default() -> Self {
        Self(30)
    }
}

impl FromKeyValue for MaxScfCycles {
    const KEY_NAME: &'static str = "MAX_SCF_CYCLES";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_i32(value)?))
    }
}

impl ToCell for MaxScfCycles {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MAX_SCF_CYCLES", CellValue::Int(self.0))
    }
}

impl ToCellValue for MaxScfCycles {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::Int(20);
        let result = MaxScfCycles::from_cell_value_kv(&val).unwrap();
        assert_eq!(result.0, 20);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(MaxScfCycles::KEY_NAME, "MAX_SCF_CYCLES");
    }
}

