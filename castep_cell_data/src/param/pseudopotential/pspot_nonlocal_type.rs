use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the representation of the nonlocal part of the pseudopotential.
///
/// Keyword type: String
///
/// Default: PspotNonlocalType::Reciprocal
///
/// Example:
/// PSPOT_NONLOCAL_TYPE : REAL
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "PSPOT_NONLOCAL_TYPE")]
pub enum PspotNonlocalType {
    /// Reciprocal space nonlocal pseudopotentials
    #[serde(alias = "RECIPROCAL", alias = "reciprocal")]
    Reciprocal,
    /// Real space nonlocal pseudopotentials
    #[serde(alias = "REAL", alias = "real")]
    Real,
}

impl Default for PspotNonlocalType {
    fn default() -> Self {
        Self::Reciprocal // Default is RECIPROCAL
    }
}

impl ToCell for PspotNonlocalType {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PSPOT_NONLOCAL_TYPE", self.to_cell_value())
    }
}

impl ToCellValue for PspotNonlocalType {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                PspotNonlocalType::Reciprocal => "RECIPROCAL",
                PspotNonlocalType::Real => "REAL",
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
    fn test_pspot_nonlocal_type_serde() {
        // 1. Test Deserialization for variants
        let test_cases = [
            (
                "PSPOT_NONLOCAL_TYPE : RECIPROCAL",
                PspotNonlocalType::Reciprocal,
            ),
            ("PSPOT_NONLOCAL_TYPE : REAL", PspotNonlocalType::Real),
        ];

        for (input_str, expected_type) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithPspotNonlocalType {
                pspot_nonlocal_type: PspotNonlocalType,
            }

            let cell_file_result: Result<CellFileWithPspotNonlocalType, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.pspot_nonlocal_type, expected_type,
                "Failed for input: {input_str}"
            );
        }

        // 2. Test Serialization using ToCell
        let pspot_nonlocal_type_instance = PspotNonlocalType::Reciprocal;
        let serialized_result = to_string(&pspot_nonlocal_type_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized PSPOT_NONLOCAL_TYPE (REAL): {serialized_string}"); // Clippy suggestion
        assert!(serialized_string.contains("PSPOT_NONLOCAL_TYPE"));
        assert!(serialized_string.contains("RECIPROCAL"));

        // 3. Test Default
        assert_eq!(PspotNonlocalType::default(), PspotNonlocalType::Reciprocal);
    }
}
