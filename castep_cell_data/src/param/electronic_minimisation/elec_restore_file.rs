use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies the name of the file from which wavefunction and density data should be restored.
///
/// Keyword type: String
///
/// Default: NULL (no restore)
///
/// Example:
/// ELEC_RESTORE_FILE : test.wvfn
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "ELEC_RESTORE_FILE")]
pub struct ElecRestoreFile(pub String); // Could be an enum Option<String> or similar for NULL handling

impl ToCell for ElecRestoreFile {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("ELEC_RESTORE_FILE", CellValue::String(self.0.clone()))
    }
}

impl ToCellValue for ElecRestoreFile {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_elec_restore_file_serde() {
        let elec_restore_file_str = "ELEC_RESTORE_FILE : test.wvfn";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithElecRestoreFile {
            elec_restore_file: ElecRestoreFile,
        }

        let cell_file_result: Result<CellFileWithElecRestoreFile, _> =
            from_str(elec_restore_file_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.elec_restore_file.0, "test.wvfn");

        let elec_restore_file_instance = ElecRestoreFile("backup.wvfn".to_string());
        let serialized_result = to_string(&elec_restore_file_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized ELEC_RESTORE_FILE (backup.wvfn): {serialized_string}");
        assert!(serialized_string.contains("ELEC_RESTORE_FILE"));
        assert!(serialized_string.contains("backup.wvfn"));
    }
}
