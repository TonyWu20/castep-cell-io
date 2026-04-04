use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_fmt::parse::FromKeyValue;
use castep_cell_fmt::query::value_as_i32;

/// Specifies the number of geometry optimization or molecular dynamics
/// iterations between updates of the backup restart files.
///
/// Keyword type: Integer
///
/// Default: 5
///
/// Example:
/// NUM_BACKUP_ITER : 2
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumBackupIter(pub i32); // i32 to allow negative values, though spec says > 0

impl Default for NumBackupIter {
    fn default() -> Self {
        Self(5) // Default is 5
    }
}

impl FromKeyValue for NumBackupIter {
    const KEY_NAME: &'static str = "NUM_BACKUP_ITER";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_i32(value)?))
    }
}

impl ToCell for NumBackupIter {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("NUM_BACKUP_ITER", CellValue::Int(self.0))
    }
}

impl ToCellValue for NumBackupIter {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::Int(2);
        let result = NumBackupIter::from_cell_value_kv(&val).unwrap();
        assert_eq!(result.0, 2);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(NumBackupIter::KEY_NAME, "NUM_BACKUP_ITER");
    }
}

