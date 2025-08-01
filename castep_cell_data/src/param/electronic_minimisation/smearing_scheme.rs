use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the Fermi-surface smearing scheme.
///
/// Keyword type: String
///
/// Default: SmearingScheme::Gaussian
///
/// Example:
/// SMEARING_SCHEME : ColdSmearing
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "SMEARING_SCHEME")]
pub enum SmearingScheme {
    /// Gaussian smearing
    #[serde(rename = "Gaussian")]
    #[default]
    Gaussian,
    /// Gaussian splines smearing
    #[serde(rename = "GaussianSplines")]
    GaussianSplines,
    /// Fermi-Dirac smearing
    #[serde(rename = "FermiDirac")]
    FermiDirac,
    /// Hermite polynomials smearing
    #[serde(rename = "HermitePolynomials")]
    HermitePolynomials,
    /// Cold smearing
    #[serde(rename = "ColdSmearing")]
    ColdSmearing,
}

impl ToCell for SmearingScheme {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("SMEARING_SCHEME", self.to_cell_value())
    }
}

impl ToCellValue for SmearingScheme {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                SmearingScheme::Gaussian => "Gaussian",
                SmearingScheme::GaussianSplines => "GaussianSplines",
                SmearingScheme::FermiDirac => "FermiDirac",
                SmearingScheme::HermitePolynomials => "HermitePolynomials",
                SmearingScheme::ColdSmearing => "ColdSmearing",
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
    fn test_smearing_scheme_serde() {
        let test_cases = [
            ("SMEARING_SCHEME : Gaussian", SmearingScheme::Gaussian),
            ("SMEARING_SCHEME : FermiDirac", SmearingScheme::FermiDirac),
            (
                "SMEARING_SCHEME : ColdSmearing",
                SmearingScheme::ColdSmearing,
            ),
        ];

        for (input_str, expected_scheme) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithSmearingScheme {
                smearing_scheme: SmearingScheme,
            }

            let cell_file_result: Result<CellFileWithSmearingScheme, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.smearing_scheme, expected_scheme,
                "Failed for input: {input_str}"
            );
        }

        let smearing_scheme_instance = SmearingScheme::ColdSmearing;
        let serialized_result = to_string(&smearing_scheme_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized SMEARING_SCHEME (ColdSmearing):\n{serialized_string}");
        assert!(serialized_string.contains("SMEARING_SCHEME"));
        assert!(serialized_string.contains("ColdSmearing"));

        assert_eq!(SmearingScheme::default(), SmearingScheme::Gaussian);
    }
}
