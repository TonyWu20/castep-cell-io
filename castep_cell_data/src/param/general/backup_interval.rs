use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies the interval, in seconds, between updates of the backup restart files.
///
/// Keyword type: Integer
///
/// Default: 0
///
/// Example:
/// BACKUP_INTERVAL : 3600
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "BACKUP_INTERVAL")]
pub struct BackupInterval(pub i32); // Using i32 as it can be negative (<=0 means no updates)

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

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_backup_interval_serde() {
        // 1. Test Deserialization
        let backup_interval_str = "BACKUP_INTERVAL : 3600";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithBackupInterval {
            backup_interval: BackupInterval,
        }

        let cell_file_result: Result<CellFileWithBackupInterval, _> = from_str(backup_interval_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.backup_interval.0, 3600);

        // 2. Test Serialization using ToCell
        let backup_interval_instance = BackupInterval(7200);
        let serialized_result = to_string(&backup_interval_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        println!("Serialized BACKUP_INTERVAL (7200):\n{serialized_string}"); // Clippy suggestion
        assert!(serialized_string.contains("BACKUP_INTERVAL"));
        assert!(serialized_string.contains("7200"));
    }
}
