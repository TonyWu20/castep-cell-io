use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the parallelization strategy used.
///
/// Keyword type: String
///
/// Default: Default
///
/// Example:
/// DATA_DISTRIBUTION : Gvector
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "DATA_DISTRIBUTION")]
pub enum DataDistribution {
    #[serde(rename = "Kpoint")]
    KPoint,
    #[serde(rename = "Gvector")]
    GVector,
    #[serde(rename = "Mixed")]
    Mixed,
    #[serde(rename = "Default")]
    Default,
}

impl ToCell for DataDistribution {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("DATA_DISTRIBUTION", self.to_cell_value())
    }
}

impl ToCellValue for DataDistribution {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                DataDistribution::KPoint => "Kpoint",
                DataDistribution::GVector => "Gvector",
                DataDistribution::Mixed => "Mixed",
                DataDistribution::Default => "Default",
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
    fn test_data_distribution_serde() {
        // 1. Test Deserialization for each variant
        let test_cases = [
            ("DATA_DISTRIBUTION : Kpoint", DataDistribution::KPoint),
            ("DATA_DISTRIBUTION : Gvector", DataDistribution::GVector),
            ("DATA_DISTRIBUTION : Mixed", DataDistribution::Mixed),
            ("DATA_DISTRIBUTION : Default", DataDistribution::Default),
        ];

        for (input_str, expected_unit) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithDataDistribution {
                data_distribution: DataDistribution,
            }

            let cell_file_result: Result<CellFileWithDataDistribution, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.data_distribution, expected_unit,
                "Failed for input: {input_str}"
            );
        }

        // 2. Test Serialization using ToCell for one example
        let data_distribution_instance = DataDistribution::GVector;
        let serialized_result = to_string(&data_distribution_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        println!("Serialized DATA_DISTRIBUTION (Gvector):\n{serialized_string}"); // Clippy suggestion
        assert!(serialized_string.contains("DATA_DISTRIBUTION"));
        assert!(serialized_string.contains("Gvector"));

        // 3. Test ToCellValue for examples
        assert_eq!(
            DataDistribution::KPoint.to_cell_value(),
            CellValue::String("Kpoint".to_string())
        );
        assert_eq!(
            DataDistribution::Default.to_cell_value(),
            CellValue::String("Default".to_string())
        );
    }
}
