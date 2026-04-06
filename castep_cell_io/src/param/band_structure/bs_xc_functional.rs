use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_str;
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
    SHF,
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

impl FromCellValue for BsXcFunctional {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "lda" => Ok(Self::Lda),
            "pw91" => Ok(Self::Pw91),
            "pbe" => Ok(Self::Pbe),
            "rpbe" => Ok(Self::Rpbe),
            "wc" => Ok(Self::Wc),
            "pbesol" => Ok(Self::Pbesol),
            "hf" => Ok(Self::Hf),
            "hf-lda" => Ok(Self::HfLda),
            "shf" => Ok(Self::SHF),
            "shf-lda" => Ok(Self::SHFLda),
            "pbe0" => Ok(Self::Pbe0),
            "b3lyp" => Ok(Self::B3lyp),
            "hse03" => Ok(Self::Hse03),
            "hse06" => Ok(Self::Hse06),
            "rscan" => Ok(Self::Rscan),
            other => Err(Error::Message(format!("unknown BsXcFunctional: {other}"))),
        }
    }
}

impl FromKeyValue for BsXcFunctional {
    const KEY_NAME: &'static str = "BS_XC_FUNCTIONAL";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for BsXcFunctional {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("BS_XC_FUNCTIONAL", self.to_cell_value())
    }
}

impl ToCellValue for BsXcFunctional {
    fn to_cell_value(&self) -> CellValue<'_> {
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
    use castep_cell_fmt::CellValue;

    #[test]
    fn test_case_insensitive() {
        assert_eq!(BsXcFunctional::from_cell_value(&CellValue::Str("lda")).unwrap(), BsXcFunctional::Lda);
        assert_eq!(BsXcFunctional::from_cell_value(&CellValue::Str("LDA")).unwrap(), BsXcFunctional::Lda);
        assert_eq!(BsXcFunctional::from_cell_value(&CellValue::Str("pbe")).unwrap(), BsXcFunctional::Pbe);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(BsXcFunctional::from_cell_value(&CellValue::Str("pw91")).unwrap(), BsXcFunctional::Pw91);
        assert_eq!(BsXcFunctional::from_cell_value(&CellValue::Str("rpbe")).unwrap(), BsXcFunctional::Rpbe);
        assert_eq!(BsXcFunctional::from_cell_value(&CellValue::Str("wc")).unwrap(), BsXcFunctional::Wc);
        assert_eq!(BsXcFunctional::from_cell_value(&CellValue::Str("pbesol")).unwrap(), BsXcFunctional::Pbesol);
        assert_eq!(BsXcFunctional::from_cell_value(&CellValue::Str("hf")).unwrap(), BsXcFunctional::Hf);
        assert_eq!(BsXcFunctional::from_cell_value(&CellValue::Str("hf-lda")).unwrap(), BsXcFunctional::HfLda);
        assert_eq!(BsXcFunctional::from_cell_value(&CellValue::Str("shf")).unwrap(), BsXcFunctional::SHF);
        assert_eq!(BsXcFunctional::from_cell_value(&CellValue::Str("shf-lda")).unwrap(), BsXcFunctional::SHFLda);
        assert_eq!(BsXcFunctional::from_cell_value(&CellValue::Str("pbe0")).unwrap(), BsXcFunctional::Pbe0);
        assert_eq!(BsXcFunctional::from_cell_value(&CellValue::Str("b3lyp")).unwrap(), BsXcFunctional::B3lyp);
        assert_eq!(BsXcFunctional::from_cell_value(&CellValue::Str("hse03")).unwrap(), BsXcFunctional::Hse03);
        assert_eq!(BsXcFunctional::from_cell_value(&CellValue::Str("hse06")).unwrap(), BsXcFunctional::Hse06);
        assert_eq!(BsXcFunctional::from_cell_value(&CellValue::Str("rscan")).unwrap(), BsXcFunctional::Rscan);
    }

    #[test]
    fn test_invalid() {
        assert!(BsXcFunctional::from_cell_value(&CellValue::Str("invalid")).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(BsXcFunctional::KEY_NAME, "BS_XC_FUNCTIONAL");
    }
}
