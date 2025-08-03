use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls which functional is used to determine the exchange-correlation potential.
///
/// Keyword type: String
///
/// Default: Derived from XC_FUNCTIONAL
///
/// Example:
/// BS_XC_FUNCTIONAL : PW91
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "BS_XC_FUNCTIONAL")]
pub enum BsXcFunctional {
    /// Local Density Approximation
    #[serde(alias = "lda", alias = "LDA")]
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
    /// Exact exchange, no correlation
    #[serde(alias = "hf", alias = "HF")]
    Hf,
    /// Exact exchange, LDA correlation
    #[serde(alias = "hf-lda", alias = "HF-LDA")]
    HfLda,
    /// Screened exchange, no correlation
    #[serde(alias = "shf", alias = "SHF")]
    SHF, // Note: serde(rename) handles non-standard identifiers if needed
    /// Screened exchange, LDA correlation
    #[serde(alias = "shf-lda", alias = "SHF-LDA")]
    SHFLda,
    /// PBE0 hybrid functional
    #[serde(alias = "pbe0", alias = "PBE0")]
    Pbe0,
    /// B3LYP hybrid functional
    #[serde(alias = "b3lyp", alias = "B3LYP")]
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

// Note: Default logic is context-dependent (derived from XC_FUNCTIONAL).
// The `Default` implementation here is omitted as it's not directly applicable.
// A containing struct or the application logic would need to handle this.

impl ToCell for BsXcFunctional {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("BS_XC_FUNCTIONAL", self.to_cell_value())
    }
}

impl ToCellValue for BsXcFunctional {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                BsXcFunctional::Lda => "LDA",
                BsXcFunctional::Pw91 => "PW91",
                BsXcFunctional::Pbe => "PBE",
                BsXcFunctional::Rpbe => "RPBE",
                BsXcFunctional::Wc => "WC",
                BsXcFunctional::Pbesol => "PBESOL",
                BsXcFunctional::Hf => "HF",
                BsXcFunctional::HfLda => "HF-LDA",
                BsXcFunctional::SHF => "SHF",
                BsXcFunctional::SHFLda => "SHF-LDA",
                BsXcFunctional::Pbe0 => "PBE0",
                BsXcFunctional::B3lyp => "B3LYP",
                BsXcFunctional::Hse03 => "HSE03",
                BsXcFunctional::Hse06 => "HSE06",
                BsXcFunctional::Rscan => "RSCAN",
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
    fn test_bs_xc_functional_serde() {
        // Test Deserialization for a few key cases
        let test_cases_deser = [
            ("BS_XC_FUNCTIONAL : LDA", BsXcFunctional::Lda),
            ("BS_XC_FUNCTIONAL : lda", BsXcFunctional::Lda),
            ("BS_XC_FUNCTIONAL : LDA", BsXcFunctional::Lda), // Uppercase alias
            ("BS_XC_FUNCTIONAL : PW91", BsXcFunctional::Pw91),
            ("BS_XC_FUNCTIONAL : pw91", BsXcFunctional::Pw91),
            ("BS_XC_FUNCTIONAL : PW91", BsXcFunctional::Pw91), // Uppercase alias
            ("BS_XC_FUNCTIONAL : PBE0", BsXcFunctional::Pbe0),
            ("BS_XC_FUNCTIONAL : pbe0", BsXcFunctional::Pbe0),
            ("BS_XC_FUNCTIONAL : PBE0", BsXcFunctional::Pbe0), // Uppercase alias
            ("BS_XC_FUNCTIONAL : SHF-LDA", BsXcFunctional::SHFLda),
            ("BS_XC_FUNCTIONAL : shf-lda", BsXcFunctional::SHFLda),
            ("BS_XC_FUNCTIONAL : SHF-LDA", BsXcFunctional::SHFLda), // Uppercase alias
        ];

        for (input_str, expected_functional) in test_cases_deser {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithBsXcFunctional {
                bs_xc_functional: BsXcFunctional,
            }

            let cell_file_result: Result<CellFileWithBsXcFunctional, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.bs_xc_functional, expected_functional,
                "Failed for input: {input_str}"
            );
        }

        // Test Serialization
        let bs_xc_functional_instance = BsXcFunctional::Pbe;
        let serialized_result = to_string(&bs_xc_functional_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized BS_XC_FUNCTIONAL (PBE): {serialized_string}");
        assert!(serialized_string.contains("BS_XC_FUNCTIONAL"));
        assert!(serialized_string.contains("PBE"));
    }
}
