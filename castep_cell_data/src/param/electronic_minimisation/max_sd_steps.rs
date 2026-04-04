use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, FromKeyValue, CResult};
use castep_cell_io::query::value_as_i32;

/// Determines the maximum number of steepest descent steps in an SCF cycle.
///
/// Keyword type: Integer
///
/// Default:
/// SD then MAX_SD_STEPS : 10
/// CG then MAX_SD_STEPS : 1
/// RMM/DIIS then MAX_SD_STEPS : 1
/// If ELECTRONIC_MINIMIZER is not defined, the default is 1.
///
/// Example:
/// MAX_SD_STEPS : 5
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaxSdSteps(pub i32);

impl Default for MaxSdSteps {
    fn default() -> Self {
        Self(1)
    }
}

impl FromKeyValue for MaxSdSteps {
    const KEY_NAME: &'static str = "MAX_SD_STEPS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_i32(value)?))
    }
}

impl ToCell for MaxSdSteps {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MAX_SD_STEPS", CellValue::Int(self.0))
    }
}

impl ToCellValue for MaxSdSteps {
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
        let result = MaxSdSteps::from_cell_value_kv(&val).unwrap();
        assert_eq!(result.0, 5);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(MaxSdSteps::KEY_NAME, "MAX_SD_STEPS");
    }
}

