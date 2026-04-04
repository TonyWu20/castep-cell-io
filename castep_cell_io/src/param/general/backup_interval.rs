use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_fmt::parse::FromKeyValue;
use castep_cell_fmt::query::value_as_i32;

/// Specifies the interval, in seconds, between updates of the backup restart files.
///
/// Keyword type: Integer
///
/// Default: 0
///
/// Example:
/// BACKUP_INTERVAL : 3600
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackupInterval(pub i32); // Using i32 as it can be negative (<=0 means no updates)

impl FromKeyValue for BackupInterval {
    const KEY_NAME: &'static str = "BACKUP_INTERVAL";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_i32(value)?))
    }
}

impl ToCell for BackupInterval {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("BACKUP_INTERVAL", CellValue::Int(self.0))
    }
}

impl ToCellValue for BackupInterval {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}


