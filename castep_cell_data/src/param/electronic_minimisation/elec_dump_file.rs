use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the name of the file into which wavefunction and density data are written.
///
/// Keyword type: String
///
/// Default: seedname.wvfn
///
/// Example:
/// ELEC_DUMP_FILE : test.wvfn
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "ELEC_DUMP_FILE")]
pub struct ElecDumpFile(pub String); // Could be an enum Option<String> or similar for NULL handling

impl ToCell for ElecDumpFile {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("ELEC_DUMP_FILE", CellValue::String(self.0.clone()))
    }
}

impl ToCellValue for ElecDumpFile {
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
    fn test_elec_dump_file_serde() {
        let elec_dump_file_str = "ELEC_DUMP_FILE : test.wvfn";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithElecDumpFile {
            elec_dump_file: ElecDumpFile,
        }

        let cell_file_result: Result<CellFileWithElecDumpFile, _> = from_str(elec_dump_file_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.elec_dump_file.0, "test.wvfn");

        let elec_dump_file_instance = ElecDumpFile("backup.wvfn".to_string());
        let serialized_result = to_string(&elec_dump_file_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized ELEC_DUMP_FILE (backup.wvfn): {serialized_string}");
        assert!(serialized_string.contains("ELEC_DUMP_FILE"));
        assert!(serialized_string.contains("backup.wvfn"));
    }
}
