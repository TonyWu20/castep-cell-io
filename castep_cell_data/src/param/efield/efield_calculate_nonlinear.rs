use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies which non-linear optical property to calculate during a TASK=EFIELD calculation.
///
/// Keyword type: String
///
/// Default: EfieldCalculateNonlinear::None
///
/// Example:
/// EFIELD_CALCULATE_NONLINEAR : CHI2
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "EFIELD_CALCULATE_NONLINEAR")]
pub enum EfieldCalculateNonlinear {
    /// Produces second harmonic generation coefficients
    #[serde(alias = "chi2", alias = "CHI2")]
    Chi2,
    /// Non-linear optical properties are not calculated
    #[serde(alias = "none", alias = "NONE")]
    None,
}

impl Default for EfieldCalculateNonlinear {
    fn default() -> Self {
        Self::None // Default is NONE
    }
}

impl ToCell for EfieldCalculateNonlinear {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("EFIELD_CALCULATE_NONLINEAR", self.to_cell_value())
    }
}

impl ToCellValue for EfieldCalculateNonlinear {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                EfieldCalculateNonlinear::Chi2 => "CHI2",
                EfieldCalculateNonlinear::None => "NONE",
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
    fn test_efield_calculate_nonlinear_serde() {
        // Test Deserialization for various cases
        let test_cases_deser = [
            (
                "EFIELD_CALCULATE_NONLINEAR : CHI2",
                EfieldCalculateNonlinear::Chi2,
            ),
            (
                "EFIELD_CALCULATE_NONLINEAR : chi2",
                EfieldCalculateNonlinear::Chi2,
            ),
            (
                "EFIELD_CALCULATE_NONLINEAR : CHI2",
                EfieldCalculateNonlinear::Chi2,
            ), // Uppercase alias
            (
                "EFIELD_CALCULATE_NONLINEAR : NONE",
                EfieldCalculateNonlinear::None,
            ),
            (
                "EFIELD_CALCULATE_NONLINEAR : none",
                EfieldCalculateNonlinear::None,
            ),
            (
                "EFIELD_CALCULATE_NONLINEAR : NONE",
                EfieldCalculateNonlinear::None,
            ), // Uppercase alias
        ];

        for (input_str, expected_option) in test_cases_deser {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithEfieldCalculateNonlinear {
                efield_calculate_nonlinear: EfieldCalculateNonlinear,
            }

            let cell_file_result: Result<CellFileWithEfieldCalculateNonlinear, _> =
                from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.efield_calculate_nonlinear, expected_option,
                "Failed for input: {input_str}"
            );
        }

        // Test Serialization
        let efield_calculate_nonlinear_instance = EfieldCalculateNonlinear::Chi2;
        let serialized_result = to_string(&efield_calculate_nonlinear_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized EFIELD_CALCULATE_NONLINEAR (CHI2): {serialized_string}");
        assert!(serialized_string.contains("EFIELD_CALCULATE_NONLINEAR"));
        assert!(serialized_string.contains("CHI2"));

        // Test Default
        assert_eq!(
            EfieldCalculateNonlinear::default(),
            EfieldCalculateNonlinear::None
        );
    }
}
