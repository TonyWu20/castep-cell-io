use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the representation of the nonlocal part of the pseudopotential
/// used for calculation of the <β|ϕ> overlaps.
///
/// Keyword type: String (expert)
///
/// Default: The value of PSPOT_NONLOCAL_TYPE (handled by parent struct/context).
///
/// Example:
/// PSPOT_BETA_PHI_TYPE : REAL
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "PSPOT_BETA_PHI_TYPE")]
pub enum PspotBetaPhiType {
    /// Reciprocal space nonlocal pseudopotentials
    #[serde(alias = "RECIPROCAL", alias = "reciprocal")]
    Reciprocal,
    /// Real space nonlocal pseudopotentials
    #[serde(alias = "REAL", alias = "real")]
    Real,
}

// Note: The default logic ("The default is the value of PSPOT_NONLOCAL_TYPE")
// is context-dependent and cannot be easily encoded in the enum itself.
// The `Default` implementation here is omitted as it's not directly applicable.
// A containing struct or the application logic would need to handle this.

impl ToCell for PspotBetaPhiType {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PSPOT_BETA_PHI_TYPE", self.to_cell_value())
    }
}

impl ToCellValue for PspotBetaPhiType {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                PspotBetaPhiType::Reciprocal => "RECIPROCAL",
                PspotBetaPhiType::Real => "REAL",
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
    fn test_pspot_beta_phi_type_serde() {
        // 1. Test Deserialization for variants
        let test_cases = [
            (
                "PSPOT_BETA_PHI_TYPE : RECIPROCAL",
                PspotBetaPhiType::Reciprocal,
            ),
            ("PSPOT_BETA_PHI_TYPE : REAL", PspotBetaPhiType::Real),
        ];

        for (input_str, expected_type) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithPspotBetaPhiType {
                pspot_beta_phi_type: PspotBetaPhiType,
            }

            let cell_file_result: Result<CellFileWithPspotBetaPhiType, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.pspot_beta_phi_type, expected_type,
                "Failed for input: {}",
                input_str
            );
        }

        // 2. Test Serialization using ToCell
        let pspot_beta_phi_type_instance = PspotBetaPhiType::Real;
        let serialized_result = to_string(&pspot_beta_phi_type_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized PSPOT_BETA_PHI_TYPE (REAL): {serialized_string}"); // Clippy suggestion
        assert!(serialized_string.contains("PSPOT_BETA_PHI_TYPE"));
        assert!(serialized_string.contains("REAL"));
    }
}
