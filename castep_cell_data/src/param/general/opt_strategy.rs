use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the optimization strategy used when there are multiple strategies
/// available for the selected algorithm.
///
/// Keyword type: String
///
/// Default: OptStrategy::Default
///
/// Example:
/// OPT_STRATEGY : Memory
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "OPT_STRATEGY")]
pub enum OptStrategy {
    /// Maximizes performance at the cost of additional memory usage.
    #[serde(rename = "Speed")]
    Speed,
    /// Balances performance and memory usage.
    #[serde(rename = "Default")]
    #[default]
    Default,
    /// Minimizes memory usage at a cost of decreased performance.
    #[serde(rename = "Memory")]
    Memory,
}

impl ToCell for OptStrategy {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("OPT_STRATEGY", self.to_cell_value())
    }
}

impl ToCellValue for OptStrategy {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                OptStrategy::Speed => "Speed",
                OptStrategy::Default => "Default",
                OptStrategy::Memory => "Memory",
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
    fn test_opt_strategy_serde() {
        let test_cases = [
            ("OPT_STRATEGY : Speed", OptStrategy::Speed),
            ("OPT_STRATEGY : Default", OptStrategy::Default),
            ("OPT_STRATEGY : Memory", OptStrategy::Memory),
        ];

        for (input_str, expected_strategy) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithOptStrategy {
                opt_strategy: OptStrategy,
            }

            let cell_file_result: Result<CellFileWithOptStrategy, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.opt_strategy, expected_strategy,
                "Failed for input: {input_str}"
            );
        }

        let opt_strategy_instance = OptStrategy::Memory;
        let serialized_result = to_string(&opt_strategy_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized OPT_STRATEGY (Memory):\n{serialized_string}");
        assert!(serialized_string.contains("OPT_STRATEGY"));
        assert!(serialized_string.contains("Memory"));

        assert_eq!(OptStrategy::default(), OptStrategy::Default);
    }
}
