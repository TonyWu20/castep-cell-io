use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the electronic minimization method used in the self-consistent calculation.
///
/// Keyword type: String
///
/// Default: MetalsMethod::Edft (or MetalsMethod::Dm if FIX_OCCUPANCY is FALSE)
///
/// Example:
/// METALS_METHOD : dm
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "METALS_METHOD")]
pub enum MetalsMethod {
    /// System treated by density mixing
    #[serde(alias = "dm", alias = "DM")]
    Dm,
    /// System treated by ensemble density functional method
    #[serde(alias = "edft", alias = "EDFT")]
    Edft,
    /// Currently not used
    #[serde(alias = "none", alias = "NONE")]
    None,
}

// Note: Default logic is context-dependent (depends on FIX_OCCUPANCY).
// The `Default` implementation here provides a base default.
impl Default for MetalsMethod {
    fn default() -> Self {
        Self::Edft // Base default is EDFT
    }
}

impl ToCell for MetalsMethod {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("METALS_METHOD", self.to_cell_value())
    }
}

impl ToCellValue for MetalsMethod {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                MetalsMethod::Dm => "DM",
                MetalsMethod::Edft => "EDFT",
                MetalsMethod::None => "NONE",
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
    fn test_metals_method_serde() {
        let test_cases = [
            ("METALS_METHOD : DM", MetalsMethod::Dm),
            ("METALS_METHOD : EDFT", MetalsMethod::Edft),
            ("METALS_METHOD : NONE", MetalsMethod::None),
            ("METALS_METHOD : dm", MetalsMethod::Dm),
            ("METALS_METHOD : edft", MetalsMethod::Edft),
            ("METALS_METHOD : none", MetalsMethod::None),
        ];

        for (input_str, expected_method) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE", serialize = "snake_case"))]
            struct CellFileWithMetalsMethod {
                metals_method: MetalsMethod,
            }

            let cell_file_result: Result<CellFileWithMetalsMethod, _> = from_str(input_str);
            println!("{input_str} deserialized as:\n{expected_method:?}");
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.metals_method, expected_method,
                "Failed for input: {input_str}"
            );
        }

        let metals_method_instance = MetalsMethod::Dm;
        let serialized_result = to_string(&metals_method_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized METALS_METHOD (DM):\n{serialized_string}");
        assert!(serialized_string.contains("METALS_METHOD"));
        assert!(serialized_string.contains("DM"));

        assert_eq!(MetalsMethod::default(), MetalsMethod::Edft);
    }
}
