use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies the number of geometry optimization or molecular dynamics
/// iterations between updates of the backup restart files.
///
/// Keyword type: Integer
///
/// Default: 5
///
/// Example:
/// NUM_BACKUP_ITER : 2
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "NUM_BACKUP_ITER")]
pub struct NumBackupIter(pub i32); // i32 to allow negative values, though spec says > 0

impl Default for NumBackupIter {
    fn default() -> Self {
        Self(5) // Default is 5
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
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_num_backup_iter_serde() {
        let num_backup_iter_str = "NUM_BACKUP_ITER : 2";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithNumBackupIter {
            num_backup_iter: NumBackupIter,
        }

        let cell_file_result: Result<CellFileWithNumBackupIter, _> = from_str(num_backup_iter_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.num_backup_iter.0, 2);

        let num_backup_iter_instance = NumBackupIter(10);
        let serialized_result = to_string(&num_backup_iter_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized NUM_BACKUP_ITER (10):\n{serialized_string}");
        assert!(serialized_string.contains("NUM_BACKUP_ITER"));
        assert!(serialized_string.contains("10"));

        assert_eq!(NumBackupIter::default(), NumBackupIter(5));
    }
}
