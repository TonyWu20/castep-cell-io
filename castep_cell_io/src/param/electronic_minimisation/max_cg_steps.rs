use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, FromKeyValue, CResult};
use castep_cell_fmt::query::value_as_i32;

/// Determines the maximum number of conjugate gradient steps in an SCF cycle.
///
/// Keyword type: Integer
///
/// Default:
/// SD then MAX_CG_STEPS : 0
/// CG then MAX_CG_STEPS : 4
/// RMM/DIIS then MAX_CG_STEPS : 2
/// If ELECTRONIC_MINIMIZER is not defined, the default is 4.
///
/// Example:
/// MAX_CG_STEPS : 5
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaxCgSteps(pub i32);

impl Default for MaxCgSteps {
    fn default() -> Self {
        Self(4)
    }
}

impl FromKeyValue for MaxCgSteps {
    const KEY_NAME: &'static str = "MAX_CG_STEPS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_i32(value)?))
    }
}

impl ToCell for MaxCgSteps {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MAX_CG_STEPS", CellValue::Int(self.0))
    }
}

impl ToCellValue for MaxCgSteps {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::Int(5);
        let result = MaxCgSteps::from_cell_value_kv(&val).unwrap();
        assert_eq!(result.0, 5);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(MaxCgSteps::KEY_NAME, "MAX_CG_STEPS");
    }
}

