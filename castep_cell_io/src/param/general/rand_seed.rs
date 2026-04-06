use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_fmt::parse::FromKeyValue;
use castep_cell_fmt::query::value_as_i32;

/// Controls the initialization of random number seeds.
///
/// Keyword type: Integer (expert)
///
/// Default: 0
///
/// Example:
/// RAND_SEED : -25
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RandSeed(pub i32);

impl FromKeyValue for RandSeed {
    const KEY_NAME: &'static str = "RAND_SEED";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_i32(value)?))
    }
}

impl ToCell for RandSeed {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("RAND_SEED", CellValue::Int(self.0))
    }
}

impl ToCellValue for RandSeed {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Int(self.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::Int(-25);
        let result = RandSeed::from_cell_value_kv(&val).unwrap();
        assert_eq!(result.0, -25);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(RandSeed::KEY_NAME, "RAND_SEED");
    }
}

