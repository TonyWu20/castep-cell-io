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
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "XC_FUNCTIONAL")]
pub enum XcFunctional {
    /// Local Density Approximation
    #[serde(rename = "LDA")]
    Lda,
    /// Perdew Wang '91 GGA
    #[serde(rename = "PW91")]
    Pw91,
    /// Perdew Burke Ernzerhof
    #[serde(rename = "PBE")]
    Pbe,
    /// Revised Perdew Burke Ernzerhof
    #[serde(rename = "RPBE")]
    Rpbe,
    /// Wu-Cohen
    #[serde(rename = "WC")]
    Wc,
    /// PBEsol, PBE functional for solids
    #[serde(rename = "PBESOL")]
    Pbesol,
    /// Becke Lee Young Parr
    #[serde(rename = "BLYP")]
    Blyp,
    /// Exact exchange, no correlation
    #[serde(rename = "HF")]
    Hf,
    /// Exact exchange, LDA correlation
    #[serde(rename = "HF-LDA")]
    HfLda,
    /// Screened exchange, no correlation
    #[serde(rename = "sX")]
    SX, // Note: serde(rename) handles non-standard identifiers
    /// Screened exchange, LDA correlation
    #[serde(rename = "sX-LDA")]
    SXlda,
    /// PBE0 hybrid functional
    #[serde(rename = "PBE0")]
    Pbe0,
    /// B3LYP hybrid functional
    #[serde(rename = "B3LYP")]
    B3lyp,
    /// HSE03 hybrid functional
    #[serde(rename = "HSE03")]
    Hse03,
    /// HSE06 hybrid functional
    #[serde(rename = "HSE06")]
    Hse06,
    /// Regularized SCAN meta-GGA functional
    #[serde(rename = "RSCAN")]
    Rscan,
}

impl Default for XcFunctional {
    fn default() -> Self {
        Self::Lda // Default is LDA
    }
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
                XcFunctional::SX => "sX",
                XcFunctional::SXlda => "sX-LDA",
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
        // 1. Test Deserialization for a few key variants
        let test_cases = [
            ("XC_FUNCTIONAL : LDA", XcFunctional::Lda),
            ("XC_FUNCTIONAL : PW91", XcFunctional::Pw91),
            ("XC_FUNCTIONAL : PBE0", XcFunctional::Pbe0),
            ("XC_FUNCTIONAL : sX", XcFunctional::SX),
            ("XC_FUNCTIONAL : RSCAN", XcFunctional::Rscan),
        ];

        for (input_str, expected_functional) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithXc {
                xc_functional: XcFunctional,
            }

            let cell_file_result: Result<CellFileWithXc, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.xc_functional, expected_functional,
                "Failed for input: {}",
                input_str
            );
        }

        // 2. Test Serialization using ToCell for one example
        let xc_functional_instance = XcFunctional::Pbe;
        let serialized_result = to_string(&xc_functional_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();

        println!("Serialized XC_FUNCTIONAL (PBE): {serialized_string}"); // Clippy suggestion
        // Basic check
        assert!(serialized_string.contains("XC_FUNCTIONAL"));
        assert!(serialized_string.contains("PBE"));

        // 3. Test ToCellValue for a couple of examples
        assert_eq!(
            XcFunctional::Hf.to_cell_value(),
            CellValue::String("HF".to_string())
        );
        assert_eq!(
            XcFunctional::B3lyp.to_cell_value(),
            CellValue::String("B3LYP".to_string())
        );

        // 4. Test Default
        assert_eq!(XcFunctional::default(), XcFunctional::Lda);
    }
}
