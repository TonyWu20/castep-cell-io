// File: xc_functional.rs (or part of your main module structure)
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls which functional is used to calculate the exchange-correlation potential.
///
/// Keyword type: String
///
/// Default: XcFunctional::Lda
///
/// Example:
/// XC_FUNCTIONAL : PW91
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "XC_FUNCTIONAL")]
pub enum XcFunctional {
    /// Local Density Approximation
    #[serde(alias = "lda", alias = "LDA")]
    #[default]
    Lda,
    /// Perdew Wang '91 GGA
    #[serde(alias = "pw91", alias = "PW91")]
    Pw91,
    /// Perdew Burke Ernzerhof
    #[serde(alias = "pbe", alias = "PBE")]
    Pbe,
    /// Revised Perdew Burke Ernzerhof
    #[serde(alias = "rpbe", alias = "RPBE")]
    Rpbe,
    /// Wu-Cohen
    #[serde(alias = "wc", alias = "WC")]
    Wc,
    /// PBEsol, PBE functional for solids
    #[serde(alias = "pbesol", alias = "PBESOL")]
    Pbesol,
    /// Becke Lee Young Parr
    #[serde(alias = "blyp", alias = "BLYP")]
    Blyp,
    /// Exact exchange, no correlation
    #[serde(alias = "hf", alias = "HF")]
    Hf,
    /// Exact exchange, LDA correlation
    #[serde(alias = "hf-lda", alias = "HF-LDA", alias = "Hf-LDA")]
    HfLda,
    /// Screened exchange, no correlation
    #[serde(alias = "sx", alias = "SX", alias = "sX")] // Note: Lowercase alias added
    SX, // Note: serde(rename) handles non-standard identifiers
    /// Screened exchange, LDA correlation
    #[serde(alias = "sx-lda", alias = "SX-LDA", alias = "sX-LDA", alias = "sX-Lda")]
    // Note: Lowercase alias added
    SXlda,
    /// PBE0 hybrid functional
    #[serde(alias = "pbe0", alias = "PBE0")]
    Pbe0,
    /// B3LYP hybrid functional
    #[serde(alias = "b3lyp", alias = "B3LYP", alias = "B3Lyp")]
    B3lyp,
    /// HSE03 hybrid functional
    #[serde(alias = "hse03", alias = "HSE03")]
    Hse03,
    /// HSE06 hybrid functional
    #[serde(alias = "hse06", alias = "HSE06")]
    Hse06,
    /// Regularized SCAN meta-GGA functional
    #[serde(alias = "rscan", alias = "RSCAN")]
    Rscan,
}

impl ToCell for XcFunctional {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("XC_FUNCTIONAL", self.to_cell_value())
    }
}

impl ToCellValue for XcFunctional {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                XcFunctional::Lda => "LDA",
                XcFunctional::Pw91 => "PW91",
                XcFunctional::Pbe => "PBE",
                XcFunctional::Rpbe => "RPBE",
                XcFunctional::Wc => "WC",
                XcFunctional::Pbesol => "PBESOL",
                XcFunctional::Blyp => "BLYP",
                XcFunctional::Hf => "HF",
                XcFunctional::HfLda => "HF-LDA",
                XcFunctional::SX => "SX", // Capitalized as per original rename
                XcFunctional::SXlda => "SX-LDA", // Capitalized as per original rename
                XcFunctional::Pbe0 => "PBE0",
                XcFunctional::B3lyp => "B3LYP",
                XcFunctional::Hse03 => "HSE03",
                XcFunctional::Hse06 => "HSE06",
                XcFunctional::Rscan => "RSCAN",
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
    fn test_xc_functional_serde() {
        // Test Deserialization for various cases including case insensitivity
        let test_cases_deser = [
            ("XC_FUNCTIONAL : Lda", XcFunctional::Lda),
            ("XC_FUNCTIONAL : lda", XcFunctional::Lda),
            ("XC_FUNCTIONAL : LDA", XcFunctional::Lda), // Uppercase alias
            ("XC_FUNCTIONAL : Pw91", XcFunctional::Pw91),
            ("XC_FUNCTIONAL : pw91", XcFunctional::Pw91),
            ("XC_FUNCTIONAL : PW91", XcFunctional::Pw91), // Uppercase alias
            ("XC_FUNCTIONAL : Pbe", XcFunctional::Pbe),
            ("XC_FUNCTIONAL : pbe", XcFunctional::Pbe),
            ("XC_FUNCTIONAL : PBE", XcFunctional::Pbe), // Uppercase alias
            ("XC_FUNCTIONAL : Hf-LDA", XcFunctional::HfLda),
            ("XC_FUNCTIONAL : hf-lda", XcFunctional::HfLda),
            ("XC_FUNCTIONAL : HF-LDA", XcFunctional::HfLda), // Uppercase alias
            ("XC_FUNCTIONAL : sX", XcFunctional::SX),
            ("XC_FUNCTIONAL : sx", XcFunctional::SX), // Lowercase alias added
            ("XC_FUNCTIONAL : SX", XcFunctional::SX), // Uppercase alias
            ("XC_FUNCTIONAL : sX-LDA", XcFunctional::SXlda),
            ("XC_FUNCTIONAL : sx-lda", XcFunctional::SXlda), // Lowercase alias added
            ("XC_FUNCTIONAL : SX-LDA", XcFunctional::SXlda), // Uppercase alias
            ("XC_FUNCTIONAL : B3Lyp", XcFunctional::B3lyp),
            ("XC_FUNCTIONAL : b3lyp", XcFunctional::B3lyp),
            ("XC_FUNCTIONAL : B3LYP", XcFunctional::B3lyp), // Uppercase alias
                                                            // Add more test cases as needed for other variants...
        ];

        for (input_str, expected_functional) in test_cases_deser {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithXcFunctional {
                xc_functional: XcFunctional,
            }

            let cell_file_result: Result<CellFileWithXcFunctional, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.xc_functional, expected_functional,
                "Failed for input: {input_str}"
            );
        }

        // Test Serialization
        let xc_functional_instance = XcFunctional::Pbe0;
        let serialized_result = to_string(&xc_functional_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized XC_FUNCTIONAL (PBE0): {serialized_string}");
        assert!(serialized_string.contains("XC_FUNCTIONAL"));
        assert!(serialized_string.contains("PBE0")); // Should serialize to the canonical uppercase form

        // Test Default
        assert_eq!(XcFunctional::default(), XcFunctional::Lda);
    }
}
