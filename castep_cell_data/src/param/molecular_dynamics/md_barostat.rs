use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the barostat method used for molecular dynamics with variable cell volume.
///
/// Keyword type: String
///
/// Default: MdBarostat::AndersenHoover
///
/// Example:
/// MD_BAROSTAT : Andersen-Hoover
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MD_BAROSTAT")]
pub enum MdBarostat {
    /// Andersen-Hoover barostat
    #[serde(
        rename = "Andersen-Hoover",
        alias = "andersen-hoover",
        alias = "ANDERSEN-HOOVER"
    )]
    AndersenHoover,
    /// Parrinello-Rahman barostat
    #[serde(
        rename = "Parrinello-Rahman",
        alias = "parrinello-rahman",
        alias = "PARRINELLO-RAHMAN"
    )]
    ParrinelloRahman,
}

impl Default for MdBarostat {
    fn default() -> Self {
        Self::AndersenHoover // Default is Andersen-Hoover
    }
}

impl ToCell for MdBarostat {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_BAROSTAT", self.to_cell_value())
    }
}

impl ToCellValue for MdBarostat {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                MdBarostat::AndersenHoover => "Andersen-Hoover",
                MdBarostat::ParrinelloRahman => "Parrinello-Rahman",
            }
            .to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_md_barostat_serde() {
        // Test Deserialization for various cases
        let test_cases_deser = [
            ("MD_BAROSTAT : Andersen-Hoover", MdBarostat::AndersenHoover),
            ("MD_BAROSTAT : andersen-hoover", MdBarostat::AndersenHoover),
            ("MD_BAROSTAT : ANDERSEN-HOOVER", MdBarostat::AndersenHoover), // Uppercase alias
            (
                "MD_BAROSTAT : Parrinello-Rahman",
                MdBarostat::ParrinelloRahman,
            ),
            (
                "MD_BAROSTAT : parrinello-rahman",
                MdBarostat::ParrinelloRahman,
            ),
            (
                "MD_BAROSTAT : PARRINELLO-RAHMAN",
                MdBarostat::ParrinelloRahman,
            ), // Uppercase alias
        ];

        for (input_str, expected_barostat) in test_cases_deser {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithMdBarostat {
                md_barostat: MdBarostat,
            }

            let cell_file_result: Result<CellFileWithMdBarostat, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.md_barostat, expected_barostat,
                "Failed for input: {input_str}"
            );
        }

        // Test Serialization
        let md_barostat_instance = MdBarostat::ParrinelloRahman;
        let serialized_result = to_string(&md_barostat_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MD_BAROSTAT (Parrinello-Rahman): {serialized_string}");
        assert!(serialized_string.contains("MD_BAROSTAT"));
        assert!(serialized_string.contains("Parrinello-Rahman"));

        // Test Default
        assert_eq!(MdBarostat::default(), MdBarostat::AndersenHoover);
    }
}
